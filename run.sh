#! /bin/bash
RUST_BACKTRACE=1 RUST_LOG="warn,chess-bevy-game=debug"  cargo watch -q -c -x 'run --features bevy/dynamic'