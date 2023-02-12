cd yew
trunk build --release

cd ..
cargo build --release

#cargo build --release --target x86_64-pc-windows-gnu

#cp target/release/automation.exe .

cargo run --release