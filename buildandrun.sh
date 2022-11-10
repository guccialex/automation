cd yew
trunk build --release

cd ..
cargo build --release

cp target/release/automation.exe .

cargo run --release

