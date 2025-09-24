# just run
run *args:
    cargo run {{args}}

debug +args:
    #!/usr/bin/env bash
    set -euo pipefail
    log_targets=""
    for target in {{args}}; do
        log_targets="$log_targets$target=debug,"
    done
    # Remove trailing comma and set the log level
    RUST_LOG="${log_targets%}b=info,"
    echo "Running with RUST_LOG=$RUST_LOG"
    RUST_LOG=$RUST_LOG cargo run

check *args:
    cargo check {{args}}

# Builds the c library
buildc:
	cargo build --release
	mv target/release/libbcraft.a cbridge/

runc *args:
    make
