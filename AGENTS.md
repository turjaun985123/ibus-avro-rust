# Rust build policy

- Do not run `cargo` commands locally, including `cargo build`, `cargo check`, `cargo test`, `cargo fmt`, `cargo clippy`, or `cargo metadata`.
- Do not perform local Rust builds or compilation by other means.
- CI is responsible for all Rust builds and validation. Use CI results to verify Rust changes.
