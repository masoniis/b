# INFO: -----------------------------
#         Basic Cargo aliases
# -----------------------------------

run *args:
	cargo run {{args}}

check *args:
	cargo check {{args}}

clean *args:
	cargo clean {{args}}

fix *args:
	cargo fix --allow-dirty

# INFO: ---------------------------
#         Advanced commands
# ---------------------------------

bevy_debug *args:
	cargo run --features bevy_ecs/trace --features bevy_ecs/track_location

trace *args:
	#!/usr/bin/env bash
	trap 'echo -e "\n\033[1;36mStopping Tracy profiler (PID: $tracy_pid)...\033[0m"; kill $tracy_pid' EXIT
	tracy &
	tracy_pid=$!
	cargo run

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



