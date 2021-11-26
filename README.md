cargo tarpaulin --ignore-tests
cargo install cargo-expand
rustup override set nightly
// Use `cargo add actix-rt --dev --vers 2` to add `actix-rt`
// under `[dev-dependencies]` in Cargo.toml
// Use `cargo add reqwest --dev --vers 0.11` to add
// it under `[dev-dependencies]` in Cargo.toml
