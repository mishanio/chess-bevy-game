# chess-bevy-game
chess game using bevy

Run command
RUST_LOG="warn,chess-bevy-game=debug" cargo run --features bevy/dynamic

For development
Run with log debug stacktrace on error and reload on save changes:
RUST_BACKTRACE=1 RUST_LOG="warn,chess-bevy-game=debug"  cargo watch -q -c -x 'run --features bevy/dynamic'

Install Cargo Watch
cargo install cargo-watch

Run in browser
https://bevy-cheatbook.github.io/platforms/wasm.html

cargo run --target wasm32-unknown-unknown


cargo build --release --target wasm32-unknown-unknown
wasm-bindgen --out-dir ./out/ --target web ./target/