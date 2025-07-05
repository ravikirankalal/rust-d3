# Contributing to Rust D3

Thank you for your interest in contributing to Rust D3! This document provides guidelines for contributing to the project.

## Development Setup

### Prerequisites
- Rust 1.70.0 or later
- Git

### Getting Started
1. Fork the repository on GitHub
2. Clone your fork locally:
   ```bash
   git clone https://github.com/YOUR_USERNAME/rust-d3.git
   cd rust-d3
   ```
3. Build the project:
   ```bash
   cargo build
   ```
4. Run tests:
   ```bash
   cargo test
   ```
5. Generate example charts:
   ```bash
   cargo run --bin rust-d3-examples
   ```

## Development Workflow

### Before Submitting Changes
1. **Format your code**: `cargo fmt`
2. **Run clippy**: `cargo clippy --all-targets --all-features -- -D warnings`
3. **Run all tests**: `cargo test`
4. **Test examples**: `cargo run --bin rust-d3-examples`

### Continuous Integration
Our CI pipeline runs on every push and pull request, testing:
- Code formatting (rustfmt)
- Linting (clippy with warnings as errors)
- Building on multiple platforms (Linux, Windows, macOS)
- Running all tests
- Security audit
- Minimum Rust version compatibility (MSRV: 1.70.0)

### Pull Request Process
1. Create a feature branch from `main`:
   ```bash
   git checkout -b feature/your-feature-name
   ```
2. Make your changes
3. Add tests for new functionality
4. Ensure all checks pass locally
5. Commit your changes with clear, descriptive messages
6. Push to your fork and create a pull request

### Code Style
- Follow Rust standard formatting (`cargo fmt`)
- Write clear, self-documenting code
- Add documentation for public APIs
- Include tests for new features

### Testing
- Unit tests for individual components
- Integration tests for chart generation
- Doc tests for code examples in documentation

## Areas for Contribution

### New Chart Types
- Scatter plots
- Histograms
- Box plots
- Heatmaps

### Scale Types
- Logarithmic scales
- Time scales
- Color scales

### Features
- Animation support
- Interactive features
- Performance optimizations
- Additional customization options

### Documentation
- API documentation improvements
- Usage examples
- Tutorials

## Release Process

Releases are automated through GitHub Actions:
- Version tags trigger the release workflow
- Binaries are built for multiple platforms
- Crates.io publication (when configured)
- GitHub releases with example charts

## Questions?

If you have questions about contributing, please:
1. Check existing issues and discussions
2. Create a new issue with the question label
3. Join discussions on GitHub

Thank you for contributing to Rust D3!