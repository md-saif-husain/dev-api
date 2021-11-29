cargo tarpaulin --ignore-tests
cargo install cargo-expand
cargo install cargo-udeps
rustup override set nightly
// Use `cargo add actix-rt --dev --vers 2` to add `actix-rt`
// under `[dev-dependencies]` in Cargo.toml
// Use `cargo add reqwest --dev --vers 0.11` to add
// it under `[dev-dependencies]` in Cargo.toml
curl -i -X POST -d 'email=thomas3_mann@hotmail.com&name=Tom3' http://127.0.0.1:8000/subscriptions
