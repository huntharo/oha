pgo:
    #!/bin/bash
    trap "kill 0" EXIT
    cargo run --release --manifest-path pgo/server/Cargo.toml &
    # maybe need more longer run
    cargo pgo run -- -- -z 1m -c 900 --no-tui http://localhost:8888
    cargo pgo optimize
