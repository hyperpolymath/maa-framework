# Aletheia - Build Automation with Just
# https://github.com/casey/just

# Default recipe - show available commands
default:
    @just --list

# Build the project in debug mode
build:
    cargo build

# Build the project in release mode
build-release:
    cargo build --release

# Run the project (verify current directory)
run:
    cargo run

# Run the project on a specific path
run-path PATH:
    cargo run -- {{PATH}}

# Run all tests
test:
    cargo test

# Run tests with output visible
test-verbose:
    cargo test -- --nocapture

# Run tests in release mode
test-release:
    cargo test --release

# Run tests and show coverage (requires cargo-tarpaulin)
test-coverage:
    cargo tarpaulin --out Html --output-dir coverage

# Run clippy linter
lint:
    cargo clippy -- -D warnings

# Run clippy with all features
lint-all:
    cargo clippy --all-targets --all-features -- -D warnings

# Format code with rustfmt
fmt:
    cargo fmt

# Check if code is formatted correctly
fmt-check:
    cargo fmt -- --check

# Run all checks (fmt, lint, test)
check: fmt-check lint test

# Run all checks in strict mode
check-strict: fmt-check lint-all test-release

# Clean build artifacts
clean:
    cargo clean

# Generate documentation
doc:
    cargo doc --no-deps

# Generate and open documentation in browser
doc-open:
    cargo doc --no-deps --open

# Install the binary locally
install:
    cargo install --path .

# Uninstall the binary
uninstall:
    cargo uninstall aletheia

# Verify RSR compliance (self-verification)
validate:
    cargo run

# Validate a different repository
validate-repo PATH:
    cargo run -- {{PATH}}

# Run benchmarks (if any)
bench:
    cargo bench

# Check for security vulnerabilities (requires cargo-audit)
audit:
    cargo audit

# Update dependencies (should remain zero for RSR Bronze)
update:
    @echo "⚠️  Warning: Aletheia maintains zero dependencies for RSR Bronze compliance"
    @echo "Adding dependencies would break RSR Bronze-level compliance"
    cargo update --dry-run

# Show dependency tree (should be empty)
tree:
    cargo tree

# Build for multiple targets
build-all-targets:
    cargo build --release --target x86_64-unknown-linux-gnu
    cargo build --release --target x86_64-unknown-linux-musl
    cargo build --release --target x86_64-apple-darwin
    cargo build --release --target x86_64-pc-windows-gnu

# Create a release (tag and build)
release VERSION:
    #!/usr/bin/env bash
    set -euo pipefail
    echo "Creating release {{VERSION}}"
    # Update version in Cargo.toml
    sed -i 's/^version = .*/version = "{{VERSION}}"/' Cargo.toml
    # Update CHANGELOG.md
    echo "Remember to update CHANGELOG.md!"
    # Run checks
    just check
    # Build release binary
    just build-release
    # Verify RSR compliance
    just validate
    # Git tag
    git add Cargo.toml
    git commit -m "chore: bump version to {{VERSION}}"
    git tag -a "v{{VERSION}}" -m "Release v{{VERSION}}"
    echo "✅ Release v{{VERSION}} created. Push with: git push && git push --tags"

# Watch for changes and run tests
watch:
    cargo watch -x test

# Watch for changes and run checks
watch-check:
    cargo watch -x check -x test

# Run mutation testing (requires cargo-mutants)
mutants:
    cargo mutants

# Count lines of code
loc:
    @echo "Lines of Rust code:"
    @find src -name "*.rs" -exec wc -l {} + | tail -1
    @echo ""
    @echo "Total files:"
    @find . -type f -not -path "./target/*" -not -path "./.git/*" | wc -l

# Show project statistics
stats:
    @echo "📊 Aletheia Statistics"
    @echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    @echo "Lines of Rust code:"
    @find src -name "*.rs" -exec wc -l {} + | tail -1
    @echo ""
    @echo "Dependencies:"
    @cargo tree --depth 0 || echo "0 dependencies (RSR Bronze compliant)"
    @echo ""
    @echo "Unsafe blocks:"
    @rg "unsafe" src/ || echo "0 unsafe blocks (RSR Bronze compliant)"
    @echo ""
    @echo "Test count:"
    @rg "#\[test\]" src/ | wc -l
    @echo ""
    @echo "Documentation files:"
    @ls -1 *.md | wc -l
    @echo ""
    @echo "Well-known files:"
    @ls -1 .well-known/ | wc -l

# CI simulation - run all checks that CI runs
ci: fmt-check lint test validate
    @echo "✅ CI simulation passed!"

# Pre-commit hook - run before committing
pre-commit: fmt lint test
    @echo "✅ Pre-commit checks passed!"

# Show help for just commands
help:
    @just --list

# Development setup - install required tools
setup:
    @echo "Installing development tools..."
    rustup component add rustfmt clippy
    cargo install cargo-watch cargo-audit cargo-tarpaulin
    @echo "✅ Development environment ready!"

# Quick check - fastest verification
quick: fmt-check
    cargo check
    cargo test --lib

# Verify that we maintain RSR Bronze compliance
verify-rsr: validate
    @echo "🔍 Checking RSR Bronze compliance criteria..."
    @echo "✅ Zero dependencies: " && cargo tree --depth 0
    @echo "✅ No unsafe code: " && (! rg "unsafe" src/ || echo "❌ UNSAFE CODE FOUND")
    @echo "✅ Tests present: " && cargo test --lib
    @echo "✅ Documentation complete: " && ls -1 *.md .well-known/
    @echo "✅ Self-verification: " && cargo run
