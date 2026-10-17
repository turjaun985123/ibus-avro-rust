use crate::{CandidateList, Preferences, SuggestionEngine};

const KEY_RETURN: u32 = 0xff0d;
const KEY_TAB: u32 = 0xff09;
const KEY_BACKSPACE: u32 = 0xff08;
const KEY_SPACE: u32 = 0x0020;
const KEY_LEFT: u32 = 0xff51;
const KEY_UP: u32 = 0xff52;
const KEY_RIGHT: u32 = 0xff53;
const KEY_DOWN: u32 = 0xff54;
const KEY_KP_ENTER: u32 = 0xff8d;
const KEY_KP_LEFT: u32 = 0xff96;
const KEY_KP_UP: u32 = 0xff97;
const KEY_KP_RIGHT: u32 = 0xff98;
const KEY_KP_DOWN: u32 = 0xff99;
const KEY_KP_ADD: u32 = 0xffab;
const KEY_KP_MULTIPLY: u32 = 0xffaa;
const KEY_KP_SUBTRACT: u32 = 0xffad;
const KEY_KP_DECIMAL: u32 = 0xffae;
const KEY_KP_DIVIDE: u32 = 0xffaf;
const KEY_KP_0: u32 = 0xffb0;
const KEY_KP_9: u32 = 0xffb9;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum LookupOrientation {
    #[default]
    Horizontal,
    Vertical,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EngineSettings {
    pub preview_enabled: bool,
    pub dictionary_enabled: bool,
    pub newline_on_enter: bool,
    pub lookup_table_size: usize,
    pub lookup_orientation: LookupOrientation,
}

impl Default for EngineSettings {
    fn default() -> Self {
        Self {
            preview_enabled: true,
            dictionary_enabled: true,
            newline_on_enter: false,
            lookup_table_size: 15,
            lookup_orientation: LookupOrientation::Horizontal,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct KeyEvent {
    pub keyval: u32,
    pub keycode: u32,
    pub state: u32,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct KeyResult {
    pub handled: bool,
    pub updated: bool,
    pub committed: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct EngineView {
    pub buffer: String,
    pub candidates: Vec<String>,
    pub selection: usize,
    pub show_auxiliary: bool,
    pub show_lookup_table: bool,
}

/// Stateful key handling and candidate presentation for one IBus engine instance.
///
/// The D-Bus adapter supplies IBus key events and translates the resulting view
/// into IBus preedit, auxiliary, lookup-table, and commit signals.
pub struct EngineSession {
    settings: EngineSettings,
    suggestions: SuggestionEngine,
    view: EngineView,
}

impl Default for EngineSession {
    fn default() -> Self {
        Self::new()
    }
}

impl EngineSession {
    pub fn new() -> Self {
        Self {
            settings: EngineSettings::default(),
            suggestions: SuggestionEngine::new(),
            view: EngineView::default(),
        }
    }

    pub fn settings(&self) -> &EngineSettings {
        &self.settings
    }

    pub fn view(&self) -> &EngineView {
        &self.view
    }

    pub fn set_settings(&mut self, settings: EngineSettings) {
        self.settings = EngineSettings {
            lookup_table_size: settings.lookup_table_size.max(1),
            ..settings
        };
        self.suggestions.set_preferences(Preferences {
            dictionary_enabled: self.settings.preview_enabled
                && self.settings.dictionary_enabled,
            candidate_limit: self.settings.lookup_table_size,
        });
        if !self.view.buffer.is_empty() {
            self.refresh_candidates();
        }
    }

    /// Process an IBus key event. The `state` value is the modifier mask after
    /// the adapter removes backend-specific X11 flags, as the original engine did.
    pub fn process_key_event(&mut self, event: KeyEvent) -> KeyResult {
        let mut result = KeyResult::default();

        if !matches!(event.state, 0 | 1 | 16 | 17) {
            return result;
        }

        // Preserve the legacy behavior that consumes the physical Shift key.
        if event.keycode == 42 {
            result.handled = true;
            return result;
        }

        if let Some(character) = input_character(event.keyval) {
            self.view.buffer.push(character);
            self.refresh_candidates();
            result.handled = true;
            result.updated = true;
            return result;
        }

        if event.keyval == KEY_BACKSPACE {
            if self.view.buffer.pop().is_some() {
                if self.view.buffer.is_empty() {
                    self.reset();
                } else {
                    self.refresh_candidates();
                }
                result.handled = true;
                result.updated = true;
            }
            return result;
        }

        if is_horizontal_arrow(event.keyval) {
            if self.view.candidates.is_empty()
                || self.settings.lookup_orientation == LookupOrientation::Vertical
            {
                result.committed = self.commit_candidate();
                result.updated = true;
            } else {
                self.move_selection(if matches!(event.keyval, KEY_LEFT | KEY_KP_LEFT) {
                    -1
                } else {
                    1
                });
                result.handled = true;
                result.updated = true;
            }
            return result;
        }

        if is_vertical_arrow(event.keyval) {
            if self.view.candidates.is_empty()
                || self.settings.lookup_orientation == LookupOrientation::Horizontal
            {
                result.committed = self.commit_candidate();
                result.updated = true;
            } else {
                self.move_selection(if matches!(event.keyval, KEY_UP | KEY_KP_UP) {
                    -1
                } else {
                    1
                });
                result.handled = true;
                result.updated = true;
            }
            return result;
        }

        if matches!(event.keyval, KEY_RETURN | KEY_SPACE | KEY_TAB | KEY_KP_ENTER) {
            result.committed = self.commit_candidate();
            result.updated = true;
            result.handled = event.keyval == KEY_RETURN
                && self.settings.newline_on_enter
                && self.settings.preview_enabled;
            return result;
        }

        if commits_before_passthrough(event.keyval) {
            result.committed = self.commit_candidate();
            result.updated = true;
        }

        result
    }

    pub fn candidate_clicked(&mut self, index: usize) -> bool {
        let Some(candidate) = self.view.candidates.get(index).cloned() else {
            return false;
        };
        self.view.selection = index;
        self.suggestions
            .update_candidate_selection(&self.view.buffer, &candidate);
        true
    }

    /// Move the highlighted candidate and remember the choice for this input.
    pub fn move_candidate_selection(&mut self, direction: isize) -> bool {
        if self.view.candidates.is_empty() {
            return false;
        }
        self.move_selection(direction);
        true
    }

    pub fn focus_out(&mut self) -> Option<String> {
        self.commit_candidate()
    }

    pub fn reset(&mut self) {
        self.view = EngineView::default();
    }

    fn refresh_candidates(&mut self) {
        let CandidateList {
            words,
            previous_selection,
        } = self.suggestions.suggest(&self.view.buffer);
        self.view.candidates = words;
        self.view.selection = previous_selection.min(self.view.candidates.len().saturating_sub(1));
        self.view.show_auxiliary = self.settings.preview_enabled;
        self.view.show_lookup_table = self.settings.preview_enabled
            && self.settings.dictionary_enabled
            && !self.view.candidates.is_empty();
    }

    fn move_selection(&mut self, direction: isize) {
        if self.view.candidates.is_empty() {
            return;
        }
        let count = self.view.candidates.len() as isize;
        self.view.selection = (self.view.selection as isize + direction).rem_euclid(count) as usize;
        if let Some(candidate) = self.view.candidates.get(self.view.selection) {
            self.suggestions
                .update_candidate_selection(&self.view.buffer, candidate);
        }
    }

    fn commit_candidate(&mut self) -> Option<String> {
        let candidate = self.view.candidates.get(self.view.selection).cloned();
        if let Some(candidate) = &candidate {
            self.suggestions
                .string_committed(&self.view.buffer, candidate);
        }
        self.reset();
        candidate
    }
}

fn input_character(keyval: u32) -> Option<char> {
    if (33..=126).contains(&keyval) {
        return char::from_u32(keyval);
    }
    match keyval {
        KEY_KP_0..=KEY_KP_9 => char::from_u32(b'0' as u32 + keyval - KEY_KP_0),
        KEY_KP_ADD => Some('+'),
        KEY_KP_MULTIPLY => Some('*'),
        KEY_KP_SUBTRACT => Some('-'),
        KEY_KP_DECIMAL => Some('.'),
        KEY_KP_DIVIDE => Some('/'),
        _ => None,
    }
}

fn is_horizontal_arrow(keyval: u32) -> bool {
    matches!(keyval, KEY_LEFT | KEY_RIGHT | KEY_KP_LEFT | KEY_KP_RIGHT)
}

fn is_vertical_arrow(keyval: u32) -> bool {
    matches!(keyval, KEY_UP | KEY_DOWN | KEY_KP_UP | KEY_KP_DOWN)
}

fn commits_before_passthrough(keyval: u32) -> bool {
    matches!(
        keyval,
        0xffe3 // Control_L
            | 0xffe4 // Control_R
            | 0xff63 // Insert
            | 0xff9e // KP_Insert
            | 0xffff // Delete
            | 0xff9f // KP_Delete
            | 0xff50 // Home
            | 0xff95 // KP_Home
            | 0xff55 // Page_Up
            | 0xff9a // KP_Page_Up
            | 0xff56 // Page_Down
            | 0xff9b // KP_Page_Down
            | 0xff57 // End
            | 0xff9c // KP_End
            | 0xffe9 // Alt_L
            | 0xffea // Alt_R
            | 0xffeb // Super_L
            | 0xffec // Super_R
    )
}
