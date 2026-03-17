# Detect proto-shim location for tests (needed when proto is installed via Homebrew)
export PROTO_LOOKUP_DIR := `dirname "$(which proto-shim 2>/dev/null || echo /dev/null)"`

# Initial project setup: install dependencies, build plugins, and configure git hooks
setup:
    rustup show
    cargo build --target wasm32-wasip1
    proto use
    lefthook install

# Build all WASM plugins
build:
    cargo build --target wasm32-wasip1

# Build WASM plugins in release mode
build-release:
    cargo build --target wasm32-wasip1 --release

# Run all tests (builds WASM first)
test: build
    cargo test

# Run tests for a specific tool
test-tool tool: build
    cargo test -p {{tool}}_tool

# Check code compiles without building
check:
    cargo check --target wasm32-wasip1

# Format code
fmt:
    cargo fmt

# Lint code
lint:
    cargo clippy --target wasm32-wasip1
