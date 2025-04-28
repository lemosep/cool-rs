# Makefile for cool-rs project

# Variables
CARGO = cargo
LALRPOP = lalrpop
SRC_DIR = src
TARGET_DIR = target
LALRPOP_SRC = $(SRC_DIR)/cool.lalrpop
LALRPOP_OUT = $(SRC_DIR)/cool.rs
TEST_DIR = tests
VALID_TEST = $(TEST_DIR)/valid_class.cl
INVALID_TEST = $(TEST_DIR)/invalid_class.cl

# Default target
all: build

# Generate cool.rs with lalrpop
generate:
	@echo "Checking lalrpop version..."
	@$(LALRPOP) --version
	@echo "Generating parser from $(LALRPOP_SRC) to $(LALRPOP_OUT)..."
	@$(LALRPOP) $(LALRPOP_SRC)
	@if [ -f "$(LALRPOP_OUT)" ]; then \
		echo "Generated $(LALRPOP_OUT)"; \
		grep -q "ProgramParser" $(LALRPOP_OUT) && echo "ProgramParser found in $(LALRPOP_OUT)" || { echo "Error: ProgramParser not found in $(LALRPOP_OUT)"; exit 1; }; \
	else \
		echo "Error: $(LALRPOP_OUT) not generated"; exit 1; \
	fi

# Build the project
build: generate
	@echo "Building project..."
	@$(CARGO) build --verbose

# Run the program with valid test
run: build
	@echo "Running with $(VALID_TEST)..."
	@$(CARGO) run -- --file $(VALID_TEST)

# Run the program with invalid test
run-invalid: build
	@echo "Running with $(INVALID_TEST)..."
	@$(CARGO) run -- --file $(INVALID_TEST)

# Run tests
test: build
	@echo "Running tests..."
	@$(CARGO) test

# Clean build artifacts
clean:
	@echo "Cleaning..."
	@rm -rf $(TARGET_DIR)
	@rm -f $(LALRPOP_OUT) lalrpop.log
	@$(CARGO) clean

# Force clean and rebuild
force-clean:
	@echo "Force cleaning..."
	@rm -rf $(TARGET_DIR) $(LALRPOP_OUT) lalrpop.log
	@$(CARGO) clean

# Install lalrpop CLI if needed
install-lalrpop:
	@echo "Installing lalrpop CLI..."
	@$(CARGO) install lalrpop

# Phony targets
.PHONY: all generate build run run-invalid test clean force-clean install-lalrpop