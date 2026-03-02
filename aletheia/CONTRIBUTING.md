# Contributing to Aletheia

Thank you for your interest in contributing to Aletheia!

## Getting Started

1. Fork the repository on GitHub
2. Clone your fork locally
3. Create a feature branch from `main`
4. Set up your development environment (see below)

## Development Setup

Aletheia requires only the Rust toolchain. No external dependencies.

```bash
# Clone and build
git clone https://github.com/hyperpolymath/aletheia.git
cd aletheia
cargo build
cargo test
```

## How to Contribute

### Reporting Issues

Open an issue on GitHub with a clear description of the problem or suggestion.

### Submitting Changes

1. Create a feature branch: `git checkout -b feat/my-feature`
2. Make your changes following the code style below
3. Run `cargo test` and `cargo clippy -- -D warnings`
4. Sign off your commits: `git commit -s`
5. Submit a pull request against `main`

## Pull Request Process

- Keep PRs focused and atomic
- Include tests for new functionality
- Ensure all existing tests pass
- Follow conventional commit format: `type(scope): description`
- All PRs require review before merge

## Code Style

- Zero external dependencies (use std library only)
- Zero unsafe code
- Run `cargo fmt` before committing
- All public items must have doc comments

## License

Contributions are licensed under PMPL-1.0-or-later. See [CONTRIBUTING.adoc](CONTRIBUTING.adoc) for full details.
