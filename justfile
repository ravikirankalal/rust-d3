# Justfile for rust-d3 development

# Default recipe
default:
    @just --list

# Run all CI checks
ci: format-check clippy test

# Check code formatting
format-check:
    cargo fmt --all -- --check

# Run clippy with warnings as errors
clippy:
    cargo clippy --all-targets -- -D warnings

# Run all tests
test:
    cargo test --all

# Format code
format:
    cargo fmt --all

# Run clippy and automatically fix issues where possible
clippy-fix:
    cargo clippy --all-targets --fix -- -D warnings

# Clean build artifacts
clean:
    cargo clean

# Build the project
build:
    cargo build

# Build the project in release mode
build-release:
    cargo build --release

# Run selection module tests only
test-selection:
    cargo test --test selection

# Run a specific test by name
test-name TEST_NAME:
    cargo test {{TEST_NAME}}

# Check for unused dependencies
check-unused:
    cargo +nightly udeps

# Generate documentation
docs:
    cargo doc --no-deps --open

# Run benchmarks (if any)
bench:
    cargo bench

# Pre-commit hook - run before committing
pre-commit: format clippy test
    @echo "✅ All checks passed! Ready to commit."
