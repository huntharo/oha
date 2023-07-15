pgo:
    cargo run --release --manifest-path pgo/server/Cargo.toml &
    trap 'kill $(jobs -p)' EXIT
    cargo pgo run -- -- -z 1m -c 900 --no-tui http://localhost:8888
    cargo pgo optimize