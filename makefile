OUTPUT_EXE_NAME := final_project_mason_bott
REQUIRED_RUST_VERSION := 1.88.0

.PHONY: all build run clean check_version

all: $(OUTPUT_EXE_NAME)

build: check_version $(OUTPUT_EXE_NAME)

$(OUTPUT_EXE_NAME): check_version
	cargo build --release
	cp target/release/b $(OUTPUT_EXE_NAME)
	@echo ""
	@echo "Compiled executable placed at ./$(OUTPUT_EXE_NAME)"
	@echo "\`make run\` is available to execute it."

check_version:
	@echo -n "Checking Rust version... "
	@VER=$$(rustc --version 2>/dev/null | cut -d ' ' -f 2); \
	if [ -z "$$VER" ]; then VER="unknown"; fi; \
	if command -v sort >/dev/null 2>&1 && command -v printf >/dev/null 2>&1; then \
			if [ "$$VER" = "unknown" ] || [ "$$(printf '%s\n' "$(REQUIRED_RUST_VERSION)" "$$VER" | sort -V | head -n1)" != "$(REQUIRED_RUST_VERSION)" ]; then \
			echo ""; \
			echo "    WARNING: FOUND RUST VERSION $$VER, BUT REQUIRED >= $(REQUIRED_RUST_VERSION)"; \
			echo "    Please run 'rustup update' to fix this, or update Rust through your package manager if you don't use rustup."; \
			echo ""; \
			echo "Continuing to compile just in case the version check was inaccurate..."; \
			echo ""; \
		else \
			echo "$$VER ✓"; \
		fi \
	else \
		echo ""; \
		echo "$$VER (version check skipped due to missing tools)"; \
	fi

run:
	@if [ ! -f $(OUTPUT_EXE_NAME) ]; then \
		echo "Executable not found. Building first..."; \
		$(MAKE) build; \
	fi
	@echo "Running the compiled executable at ./$(OUTPUT_EXE_NAME)..."
	./$(OUTPUT_EXE_NAME)

clean:
	@rm -f $(OUTPUT_EXE_NAME)
	@cargo clean
