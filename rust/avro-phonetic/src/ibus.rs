use std::{
    collections::HashMap,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
};

use avro_phonetic::{
    EngineSession, EngineSettings, EngineView, KeyEvent, KeyResult, LookupOrientation,
};
use gio::prelude::*;
use serde::Serialize;
use tokio::sync::Mutex;
use zbus::{
    connection::Builder,
    fdo,
    object_server::{ObjectServer, SignalEmitter},
    zvariant::{OwnedObjectPath, OwnedValue, Type, Value},
};

const BUS_NAME: &str = "org.freedesktop.IBus.Avro";
const FACTORY_PATH: &str = "/org/freedesktop/IBus/Factory";
const DEFAULT_ENGINE_NAME: &str = "ibus-avro";

#[tokio::main]
async fn main() -> zbus::Result<()> {
    let mut service_name = BUS_NAME.to_owned();
    let mut engine_name = DEFAULT_ENGINE_NAME.to_owned();
    for argument in std::env::args().skip(1) {
        if let Some(value) = argument.strip_prefix("--service-name=") {
            service_name = value.to_owned();
        } else if let Some(value) = argument.strip_prefix("--engine-name=") {
            engine_name = value.to_owned();
        }
    }

    let factory = Factory {
        engine_name,
        next_engine_id: Arc::new(AtomicUsize::new(0)),
    };
    let connection = Builder::ibus()?
        .name(service_name)?
        .serve_at(FACTORY_PATH, factory)?
        .serve_at(FACTORY_PATH, Service)?
        .build()
        .await?;

    // Keep the factory and all created engine objects alive for the IBus session.
    let _connection = connection;
    std::future::pending::<()>().await;
    Ok(())
}

struct Factory {
    engine_name: String,
    next_engine_id: Arc<AtomicUsize>,
}

#[zbus::interface(name = "org.freedesktop.IBus.Factory", spawn = false)]
impl Factory {
    async fn create_engine(
        &self,
        engine_name: String,
        #[zbus(object_server)] server: &ObjectServer,
    ) -> fdo::Result<OwnedObjectPath> {
        if engine_name != self.engine_name {
            return Err(fdo::Error::Failed(format!("Unknown engine: {engine_name}")));
        }

        let id = self.next_engine_id.fetch_add(1, Ordering::Relaxed) + 1;
        let path = format!("/org/freedesktop/IBus/Engine/{id}");
        server
            .at(path.as_str(), Engine::default())
            .await
            .map_err(|error| fdo::Error::Failed(error.to_string()))?;
        server
            .at(path.as_str(), Service)
            .await
            .map_err(|error| fdo::Error::Failed(error.to_string()))?;

        OwnedObjectPath::try_from(path)
            .map_err(|error| fdo::Error::Failed(error.to_string()))
    }
}

#[derive(Default)]
struct Engine {
    session: Mutex<EngineSession>,
}

impl Default for Engine {
    fn default() -> Self {
        let mut session = EngineSession::new();
        session.set_settings(read_engine_settings());
        Self {
            session: Mutex::new(session),
        }
    }
}

#[zbus::interface(name = "org.freedesktop.IBus.Engine", spawn = false)]
impl Engine {
    async fn process_key_event(
        &self,
        keyval: u32,
        keycode: u32,
        state: u32,
        #[zbus(signal_emitter)] emitter: SignalEmitter<'_>,
    ) -> fdo::Result<bool> {
        // Drop bits outside IBus's modifier mask, matching the legacy GJS
        // engine's ModifierType.MODIFIER_MASK handling.
        let state = state & 0x5f00_1fff;
        let (result, view, settings) = {
            let mut session = self.session.lock().await;
            let result = session.process_key_event(KeyEvent {
                keyval,
                keycode,
                state,
            });
            (
                result,
                session.view().clone(),
                session.settings().clone(),
            )
        };
        self.emit_result(&emitter, &result, &view, &settings).await?;
        Ok(result.handled)
    }

    async fn candidate_clicked(
        &self,
        index: u32,
        _button: u32,
        _state: u32,
        #[zbus(signal_emitter)] emitter: SignalEmitter<'_>,
    ) -> fdo::Result<()> {
        let (view, settings) = {
            let mut session = self.session.lock().await;
            if !session.candidate_clicked(index as usize) {
                return Ok(());
            }
            (session.view().clone(), session.settings().clone())
        };
        Self::emit_view(&emitter, &view, &settings).await
    }

    async fn focus_out(
        &self,
        #[zbus(signal_emitter)] emitter: SignalEmitter<'_>,
    ) -> fdo::Result<()> {
        let (committed, view, settings) = {
            let mut session = self.session.lock().await;
            let committed = session.focus_out();
            (
                committed,
                session.view().clone(),
                session.settings().clone(),
            )
        };
        if let Some(text) = committed {
            emitter
                .commit_text(ibus_text(&text))
                .await
                .map_err(|error| fdo::Error::Failed(error.to_string()))?;
        }
        Self::emit_view(&emitter, &view, &settings).await
    }

    async fn reset(
        &self,
        #[zbus(signal_emitter)] emitter: SignalEmitter<'_>,
    ) -> fdo::Result<()> {
        let (view, settings) = {
            let mut session = self.session.lock().await;
            session.reset();
            (session.view().clone(), session.settings().clone())
        };
        Self::emit_view(&emitter, &view, &settings).await
    }

    async fn focus_in(&self) {
        self.session
            .lock()
            .await
            .set_settings(read_engine_settings());
    }
    async fn enable(&self) {}
    async fn disable(&self) {}
    async fn set_cursor_location(&self, _x: i32, _y: i32, _width: i32, _height: i32) {}
    async fn set_capabilities(&self, _capabilities: u32) {}
    async fn property_activate(&self, _name: String, _state: u32) {}
    async fn property_show(&self, _name: String) {}
    async fn property_hide(&self, _name: String) {}
    async fn cursor_up(
        &self,
        #[zbus(signal_emitter)] emitter: SignalEmitter<'_>,
    ) -> fdo::Result<()> {
        self.move_candidate_selection(-1, &emitter).await
    }

    async fn cursor_down(
        &self,
        #[zbus(signal_emitter)] emitter: SignalEmitter<'_>,
    ) -> fdo::Result<()> {
        self.move_candidate_selection(1, &emitter).await
    }
    async fn page_up(&self) {}
    async fn page_down(&self) {}

    async fn set_surrounding_text(
        &self,
        _text: OwnedValue,
        _cursor_pos: u32,
        _anchor_pos: u32,
    ) {
    }

    #[zbus(signal)]
    async fn commit_text(emitter: &SignalEmitter<'_>, text: OwnedValue) -> zbus::Result<()>;

    #[zbus(signal)]
    async fn update_preedit_text(
        emitter: &SignalEmitter<'_>,
        text: OwnedValue,
        cursor_pos: u32,
        visible: bool,
        mode: u32,
    ) -> zbus::Result<()>;

    #[zbus(signal)]
    async fn update_auxiliary_text(
        emitter: &SignalEmitter<'_>,
        text: OwnedValue,
        visible: bool,
    ) -> zbus::Result<()>;

    #[zbus(signal)]
    async fn update_lookup_table(
        emitter: &SignalEmitter<'_>,
        table: OwnedValue,
        visible: bool,
    ) -> zbus::Result<()>;
}

impl Engine {
    async fn move_candidate_selection(
        &self,
        direction: isize,
        emitter: &SignalEmitter<'_>,
    ) -> fdo::Result<()> {
        let (changed, view, settings) = {
            let mut session = self.session.lock().await;
            let changed = session.move_candidate_selection(direction);
            (
                changed,
                session.view().clone(),
                session.settings().clone(),
            )
        };
        if changed {
            Self::emit_view(emitter, &view, &settings).await?;
        }
        Ok(())
    }

    async fn emit_result(
        &self,
        emitter: &SignalEmitter<'_>,
        result: &KeyResult,
        view: &EngineView,
        settings: &EngineSettings,
    ) -> fdo::Result<()> {
        if let Some(text) = &result.committed {
            emitter
                .commit_text(ibus_text(text))
                .await
                .map_err(|error| fdo::Error::Failed(error.to_string()))?;
        }
        if result.updated {
            Self::emit_view(emitter, view, settings).await?;
        }
        Ok(())
    }

    async fn emit_view(
        emitter: &SignalEmitter<'_>,
        view: &EngineView,
        settings: &EngineSettings,
    ) -> fdo::Result<()> {
        let has_preedit = !view.buffer.is_empty() && !view.candidates.is_empty();
        let preedit = view
            .candidates
            .get(view.selection)
            .map(String::as_str)
            .unwrap_or("");

        emitter
            .update_preedit_text(
            ibus_text(preedit),
            preedit.chars().count() as u32,
            has_preedit,
            0,
            )
            .await
            .map_err(|error| fdo::Error::Failed(error.to_string()))?;
        emitter
            .update_auxiliary_text(ibus_text(&view.buffer), view.show_auxiliary)
            .await
            .map_err(|error| fdo::Error::Failed(error.to_string()))?;
        emitter
            .update_lookup_table(ibus_lookup_table(view, settings), view.show_lookup_table)
            .await
            .map_err(|error| fdo::Error::Failed(error.to_string()))?;
        Ok(())
    }
}

struct Service;

#[zbus::interface(name = "org.freedesktop.IBus.Service", spawn = false)]
impl Service {
    async fn destroy(
        &self,
        #[zbus(header)] header: zbus::message::Header<'_>,
        #[zbus(object_server)] server: &ObjectServer,
    ) -> fdo::Result<()> {
        let Some(path) = header.path().map(|path| path.as_str().to_owned()) else {
            return Ok(());
        };
        if path == FACTORY_PATH {
            let _ = server
                .remove::<Factory, _>(path.as_str())
                .await
                .map_err(|error| fdo::Error::Failed(error.to_string()))?;
        } else {
            let _ = server
                .remove::<Engine, _>(path.as_str())
                .await
                .map_err(|error| fdo::Error::Failed(error.to_string()))?;
        }
        let _ = server
            .remove::<Service, _>(path.as_str())
            .await
            .map_err(|error| fdo::Error::Failed(error.to_string()))?;
        Ok(())
    }
}

#[derive(Clone, Serialize, Type, zbus::zvariant::Value)]
struct IBusAttrList {
    type_name: String,
    attachments: HashMap<String, OwnedValue>,
    attributes: Vec<OwnedValue>,
}

#[derive(Clone, Serialize, Type, zbus::zvariant::Value)]
struct IBusText {
    type_name: String,
    attachments: HashMap<String, OwnedValue>,
    text: String,
    attributes: OwnedValue,
}

#[derive(Clone, Serialize, Type, zbus::zvariant::Value)]
struct IBusLookupTable {
    type_name: String,
    attachments: HashMap<String, OwnedValue>,
    page_size: u32,
    cursor_pos: u32,
    cursor_visible: bool,
    round: bool,
    orientation: i32,
    candidates: Vec<OwnedValue>,
    labels: Vec<OwnedValue>,
}

fn ibus_text(text: &str) -> OwnedValue {
    let attributes = OwnedValue::try_from(Value::from(IBusAttrList {
        type_name: "IBusAttrList".into(),
        attachments: HashMap::new(),
        attributes: Vec::new(),
    }))
    .expect("IBus attribute list is a valid D-Bus value");
    OwnedValue::try_from(Value::from(IBusText {
        type_name: "IBusText".into(),
        attachments: HashMap::new(),
        text: text.to_owned(),
        attributes,
    }))
    .expect("IBus text is a valid D-Bus value")
}

fn ibus_lookup_table(view: &EngineView, settings: &EngineSettings) -> OwnedValue {
    let candidates = view.candidates.iter().map(|word| ibus_text(word)).collect();
    // IBus displays numbered labels by default; blank labels preserve the
    // legacy Avro lookup-table presentation.
    let labels = view.candidates.iter().map(|_| ibus_text("")).collect();
    let table = IBusLookupTable {
        type_name: "IBusLookupTable".into(),
        attachments: HashMap::new(),
        page_size: settings.lookup_table_size as u32,
        cursor_pos: view.selection as u32,
        cursor_visible: true,
        round: true,
        orientation: match settings.lookup_orientation {
            LookupOrientation::Horizontal => 0,
            LookupOrientation::Vertical => 1,
        },
        candidates,
        labels,
    };
    OwnedValue::try_from(Value::from(table)).expect("IBus lookup table is a valid D-Bus value")
}

fn read_engine_settings() -> EngineSettings {
    let Some(schema_source) = gio::SettingsSchemaSource::default() else {
        return EngineSettings::default();
    };
    if schema_source
        .lookup("com.omicronlab.avro", true)
        .is_none()
    {
        return EngineSettings::default();
    }

    let settings = gio::Settings::new("com.omicronlab.avro");
    EngineSettings {
        preview_enabled: settings.boolean("switch-preview"),
        dictionary_enabled: settings.boolean("switch-dict"),
        newline_on_enter: settings.boolean("switch-newline"),
        lookup_table_size: settings.int("lutable-size").clamp(1, 15) as usize,
        lookup_orientation: if settings.int("cboxorient") == 1 {
            LookupOrientation::Vertical
        } else {
            LookupOrientation::Horizontal
        },
    }
}
