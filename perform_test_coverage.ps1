# Setup
$env:RUSTFLAGS="-C instrument-coverage"

# Perform
cargo clean
cargo build
cargo test --tests
grcov . -s . --binary-path ./target/debug/ -t html --branch --ignore-not-existing -o ./target/debug/coverage/

# Cleanup
rm *.profraw; rm **/*.profraw
$env:RUSTFLAGS=""

# Display
echo "Output: ./target/debug/coverage/"
