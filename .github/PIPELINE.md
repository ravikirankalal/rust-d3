# GitHub Actions CI/CD Pipeline Documentation

This document describes the CI/CD pipeline implemented for the rust-d3 project.

## Overview

The pipeline consists of two main workflows:

1. **Continuous Integration (CI)** - Runs on every push and pull request
2. **Release** - Runs on version tags to create releases

## Continuous Integration Workflow

**File**: `.github/workflows/ci.yml`

### Triggers
- Push to `main` or `master` branches
- Pull requests to `main` or `master` branches

### Jobs

#### Test Suite
- **Platforms**: Ubuntu, Windows, macOS
- **Rust Versions**: stable, beta (beta only on Ubuntu to reduce CI load)
- **Steps**:
  - Checkout code
  - Install Rust toolchain with rustfmt and clippy
  - Cache dependencies for faster builds
  - Format check (`cargo fmt --check`)
  - Lint check (`cargo clippy -- -D warnings`)
  - Build (`cargo build --verbose`)
  - Run tests (`cargo test --verbose`)
  - Run doc tests (`cargo test --doc`)
  - Generate example charts
  - Upload artifacts (example SVGs and integration test charts)

#### Security Audit
- Runs `cargo-audit` to check for security vulnerabilities
- Uses caching for performance

#### Minimum Rust Version (MSRV)
- Tests compatibility with Rust 1.70.0
- Ensures the library works with the minimum supported version

### Artifacts
- Example chart SVG files
- Integration test chart SVG files
- Retained for 30 days

## Release Workflow

**File**: `.github/workflows/release.yml`

### Triggers
- Version tags matching `v*.*.*` pattern (e.g., `v0.1.0`, `v1.2.3`)

### Jobs

#### Create Release
- Extracts version from tag
- Creates GitHub release with detailed release notes
- Provides upload URL for assets

#### Build Assets
- **Platforms**: Linux (x86_64), Windows (x86_64), macOS (x86_64)
- **Steps**:
  - Build release binaries for each platform
  - Generate example charts
  - Create platform-specific asset names
  - Upload binaries to the release
  - Create and upload example charts archive (Linux only)

#### Publish Crate
- Verifies the package can be built
- Publishes to crates.io (if `CARGO_REGISTRY_TOKEN` secret is set)
- Conditional execution based on repository ownership

### Release Assets
- Cross-platform binaries: `rust-d3-examples-{platform}-{arch}`
- Example charts archive: `rust-d3-examples-charts.tar.gz`

## Setup Requirements

### For Contributors
No special setup required. The CI will run automatically on pull requests.

### For Maintainers
To enable crates.io publishing, add the `CARGO_REGISTRY_TOKEN` secret to the repository:

1. Go to repository Settings → Secrets and variables → Actions
2. Add a new repository secret named `CARGO_REGISTRY_TOKEN`
3. Set the value to your crates.io API token

## Caching Strategy

Both workflows use GitHub Actions caching to speed up builds:
- Cargo registry cache
- Cargo git cache
- Target directory cache

Cache keys are based on:
- Operating system
- Rust version (for CI)
- Cargo.lock hash

## Security Considerations

- Uses official actions from trusted sources
- Pins action versions for reproducibility
- Runs security audits on dependencies
- No secrets are exposed in logs
- Conditional crates.io publishing based on repository ownership

## Monitoring and Maintenance

### CI Status
- Badge available in README: `[![CI](https://github.com/ravikirankalal/rust-d3/workflows/CI/badge.svg)](https://github.com/ravikirankalal/rust-d3/actions/workflows/ci.yml)`
- Failed builds will be visible in the Actions tab
- Email notifications for failed builds (configured per user)

### Updating Workflows
- Update Rust versions as new stable releases come out
- Review and update action versions periodically
- Monitor for new security audit findings

### Release Process
1. Update version in `Cargo.toml`
2. Update `CHANGELOG.md` (if exists)
3. Commit changes
4. Create and push a version tag: `git tag v0.1.1 && git push origin v0.1.1`
5. The release workflow will automatically create the GitHub release and assets

## Troubleshooting

### Common Issues
- **Build failures**: Check Rust version compatibility and dependency updates
- **Test failures**: Ensure tests pass locally before pushing
- **Clippy warnings**: Fix all warnings as they are treated as errors
- **Format check failures**: Run `cargo fmt` before committing

### Debugging
- Check the Actions tab for detailed logs
- Failed jobs will show specific error messages
- Use `act` tool for local workflow testing (optional)

### Performance
- First runs may be slower due to cache warming
- Subsequent runs should be faster with cached dependencies
- Consider reducing test matrix if CI becomes too slow