# INFO: -----------------------------
#         basic cargo aliases
# -----------------------------------

run *args:
	cargo run {{args}}

bench *args:
	cargo bench {{args}}
	echo "Opening Criterion report..."
	open target/criterion/report/index.html

check *args:
	cargo check {{args}}

clean *args:
	cargo clean {{args}}

zip:
	git archive --format=zip -o b.zip HEAD

fix *args:
	cargo fix --allow-dirty

# INFO: ---------------------------
#         advanced commands
# ---------------------------------

# requires https://crates.io/crates/cargo-show-asm
asm path:
    cargo asm --bin b --color {{path}}

trace *args:
	#!/usr/bin/env bash

	# launch tracy if it isn't already running
	if pgrep -x "tracy" > /dev/null; then
			echo -e "\033[1;32mTracy profiler is already running.\033[0m"
	else
			echo -e "\033[1;36mStarting Tracy profiler...\033[0m"
			TRACY_ENABLE_MEMORY=1
			tracy &
	fi

	cargo run --features tracy {{args}}

debug_bevy *args:
	cargo run --features bevy_ecs/trace --features bevy_ecs/track_location

debug_wgpu *args:
	RUST_LOG=wgpu=trace cargo run {{args}}

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
		log_targets="$log_targets$target=trace,"
	done
	export RUST_LOG="${log_targets%,},b=info"

	echo -e "\033[1;32mRunning with RUST_LOG=\033[0m$RUST_LOG"

	cargo run
