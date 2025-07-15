# CI Integration & Pre-Merge Checklist

This document outlines the CI integration and pre-merge checklist implemented for the rust-d3 project.

## Requirements

The CI system runs the following commands and requires them to pass before merging:

1. **`cargo fmt --all -- --check`** - Ensures code formatting is consistent
2. **`cargo clippy --all-targets -- -D warnings`** - Runs linting with warnings treated as errors
3. **`cargo test --all`** - Runs all tests including unit tests, integration tests, and doc tests

## GitHub Actions Integration

The CI checks are implemented in `.github/workflows/ci.yml` and will run:

- On every push to `main` or `master` branches
- On every pull request to `main` or `master` branches
- Across multiple operating systems (Ubuntu, Windows, macOS)
- With both stable and beta Rust versions

### Key CI Steps

1. **Code Formatting Check**: Ensures consistent code style using `rustfmt`
2. **Linting**: Runs `clippy` to catch common mistakes and enforce best practices
3. **Testing**: Runs all tests to ensure functionality is preserved
4. **Security Audit**: Runs `cargo audit` to check for security vulnerabilities
5. **Minimum Rust Version**: Ensures code works with the minimum supported Rust version

## Local Development

### Using justfile (recommended)

If you have `just` installed, you can use the provided `justfile`:

```bash
# Run all CI checks locally
just ci

# Run individual checks
just format-check
just clippy
just test

# Format code
just format

# Run pre-commit checks
just pre-commit
```

### Manual Commands

```bash
# Check formatting
cargo fmt --all -- --check

# Run clippy
cargo clippy --all-targets -- -D warnings

# Run tests
cargo test --all

# Format code (fix formatting issues)
cargo fmt --all
```

## Pre-Merge Requirements

Before merging any pull request, ensure:

1. ✅ All CI checks pass
2. ✅ Code is formatted (`cargo fmt --all -- --check`)
3. ✅ No clippy warnings (`cargo clippy --all-targets -- -D warnings`)
4. ✅ All tests pass (`cargo test --all`)
5. ✅ Security audit passes (`cargo audit`)

## D3.js Parity Features

As part of the CI integration, we've implemented and tested the Selection `clone` method to match D3.js behavior:

- **Deep cloning**: `selection.clone()` creates new nodes and inserts them after originals
- **Shallow cloning**: `selection.clone_shallow()` clones only the selection keys
- **Event handlers**: Event handlers are not cloned (matching D3.js behavior)
- **Node insertion**: Cloned nodes are inserted immediately after their originals in the DOM tree

All clone functionality is thoroughly tested and must pass CI before merging.

## Troubleshooting

### Common Issues

1. **Formatting failures**: Run `cargo fmt --all` to fix formatting issues
2. **Clippy warnings**: Review and fix the specific warnings shown by clippy
3. **Test failures**: Review test output and fix failing tests
4. **Security vulnerabilities**: Update dependencies with `cargo update`

### CI Debugging

If CI fails:

1. Check the specific step that failed in the GitHub Actions logs
2. Run the same command locally to reproduce the issue
3. Fix the issue and push again
4. Ensure all local checks pass before pushing

## Development Workflow

1. Make changes to your code
2. Run `just pre-commit` (or manual commands) to check locally
3. Fix any issues found
4. Commit and push changes
5. Wait for CI to pass
6. Merge when all checks are green ✅
