# just run
run *args:
    cargo run {{args}}

debug *args:
	#!/usr/bin/env bash
	set -euo pipefail

	set -- {{args}}

	# If no args provided, RG the codebase to find all available debug targets
	if [ "$#" -eq 0 ]; then
			echo -e "\033[1;33mNo debug targets specified. Available targets are:\033[0m"

			# Find all targets, sort them, remove duplicates, and format as a nice list.
			rg --no-heading -o --replace '$f:$1' 'target\s*:\s*"([^"]+)"' src/ \
				| sort \
				| while IFS= read -r line; do
						# Split the line into two parts at the last colon ':'
						filepath="${line%:*}"
						target="${line##*:}"

						# Print target with coloring for easy reading
						printf '  - %s:\033[1;35m%s\033[0m\n' "$filepath" "$target"
					done

			exit 0
	fi

	log_targets=""
	for target in "$@"; do
			log_targets="$log_targets$target=debug,"
	done

	# Set the RUST_LOG environment variable.
	# ${log_targets%,} removes the trailing comma from the loop.
	export RUST_LOG="${log_targets%,},b=info"

	# Print the RUST_LOG value that will be used (with color for visibility).
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
