OUTPUT_EXE_NAME := final_project_mason_bott

.PHONY: all build run clean

all: $(OUTPUT_EXE_NAME)

build: $(OUTPUT_EXE_NAME)

$(OUTPUT_EXE_NAME):
	cargo build --release
	cp target/release/b $(OUTPUT_EXE_NAME)
	@echo ""
	@echo "Compiled executable placed at ./$(OUTPUT_EXE_NAME)"
	@echo ""

run: $(OUTPUT_EXE_NAME)
	@echo ""
	@echo "Running the compiled executable at ./$(OUTPUT_EXE_NAME)..."
	@echo ""
	@./$(OUTPUT_EXE_NAME)

clean:
	@rm -f $(OUTPUT_EXE_NAME)
	@cargo clean
