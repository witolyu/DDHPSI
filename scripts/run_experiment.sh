echo "Starting Program"
cargo build --release

cargo run --release --bin server &
sleep 5
cargo run --release --bin client &
