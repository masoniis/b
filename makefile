# INFO: ----------------
#         config
# ----------------------

OUTPUT_EXE_NAME := final_project_mason_bott
REQUIRED_RUST_VERSION := 1.88.0

# INFO: --------------------
#         rust setup
# --------------------------

TAR_NAME := rust-1.88.0-x86_64-unknown-linux-gnu.tar.gz
EXTRACT_DIR := rust-1.88.0-x86_64-unknown-linux-gnu

LOCAL_DIR := $(shell pwd)/.local_rust
LOCAL_CARGO := $(LOCAL_DIR)/bin/cargo

# INFO: ---------------
#         logic
# ---------------------

.PHONY: all run build clean setup

all: build

run: build
	@echo "executing ./final ..."
	@./final

build: setup
	@echo "building Release..."
	@export PATH=$(LOCAL_DIR)/bin:$(PATH) && \
	$(LOCAL_CARGO) build --release
	@cp target/release/b final
	@echo ""
	@echo "build finished, binary placed at ./final"
	@echo ""

setup:
	@if [ ! -f "$(LOCAL_CARGO)" ]; then \
		echo "unpacking bundled toolchain..."; \
		if [ ! -f "deps/$(TAR_NAME)" ]; then \
			echo "error: deps/$(TAR_NAME) not found!"; \
			exit 1; \
		fi \
		\
		# extract tarbell \
		tar -xzf deps/$(TAR_NAME); \
		\
		# run install script outputting to local dir \
		echo "installing toolchain locally to $(LOCAL_DIR)..."; \
		./$(EXTRACT_DIR)/install.sh --prefix=$(LOCAL_DIR) --components=cargo,rustc,rust-std-x86_64-unknown-linux-gnu --disable-ldconfig; \
		\
		# cleanup the extracted folder \
		rm -rf $(EXTRACT_DIR); \
	else \
		echo "rust toolchain detected."; \
	fi

clean:
	@if [ -f "$(LOCAL_CARGO)" ]; then \
		$(LOCAL_CARGO) clean; \
	fi
	rm -rf .local_rust
	rm -f final
