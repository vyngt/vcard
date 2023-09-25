$env:RUSTFLAGS="-C instrument-coverage"
cargo clean
cargo build
cargo test --tests
grcov . -s . --binary-path ./target/debug/ -t html --branch --ignore-not-existing -o ./target/debug/coverage/
rm **/*.profraw