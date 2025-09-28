# just run
run *args:
    cargo run {{args}}

debug *args:
		#!/usr/bin/env bash
		set -euo pipefail
		set -- {{args}}
		if [ "$#" -eq 0 ]; then
			echo -e "\033[1;33mNo debug targets specified. Available targets are:\033[0m"
			rg --no-heading -o --replace '$f:$1' 'target\s*:\s*"([^"]+)"' src/ \
				| awk -F: '{print $NF}' \
				| sort \
				| uniq -c \
				| sort -rn \
				| while read -r count target; do
						printf '  - \033[1;35m%s\033[0m (%sx)\n' "$target" "$count"
					done
			exit 0
		fi

		# Add targets to the rust log env variable
		log_targets=""
		for target in "$@"; do
			log_targets="$log_targets$target=debug,"
		done
		export RUST_LOG="${log_targets%,},b=info"

		echo -e "\033[1;32mRunning with RUST_LOG=\033[0m$RUST_LOG"

		cargo run

check *args:
    cargo check {{args}}

# Builds the c library
buildc:
	cargo build --release
	mv target/release/libbcraft.a cbridge/

runc *args:
    make

build-web:
    wasm-pack build --target web --out-name wasm --out-dir ./web

web:
    just build-web
    #!/usr/bin/env bash
    kill -9 $(lsof -ti:8000) || true
    cd web && python3 -m http.server &
    PID=$!
    trap 'kill $PID' EXIT
    open http://localhost:8000
    wait $PID
