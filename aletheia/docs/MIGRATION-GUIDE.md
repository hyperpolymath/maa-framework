# Migration Guide: Making Your Repository RSR-Compliant

This guide helps you migrate an existing repository to RSR Bronze-level compliance.

## Table of Contents

1. [Assessment](#assessment)
2. [Documentation Migration](#documentation-migration)
3. [Security Setup](#security-setup)
4. [Build System](#build-system)
5. [Source Organization](#source-organization)
6. [Testing](#testing)
7. [Verification](#verification)

## Assessment

### Step 1: Run Initial Assessment

```bash
# Clone and install Aletheia
git clone https://gitlab.com/maa-framework/6-the-foundation/aletheia.git
cd aletheia
cargo install --path .

# Assess your repository
cd /path/to/your/repository
aletheia .
```

You'll see output like:

```
🔍 Aletheia - RSR Compliance Verification Report
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Repository: /path/to/your/repository

📋 Documentation
  ✅ README.md [Bronze]
  ❌ LICENSE.txt [Bronze]
  ❌ SECURITY.md [Bronze]
  ❌ CONTRIBUTING.md [Bronze]
  ❌ CODE_OF_CONDUCT.md [Bronze]
  ❌ MAINTAINERS.md [Bronze]
  ❌ CHANGELOG.md [Bronze]

...

Score: 3/16 checks passed (18.8%)
⚠️  Bronze-level RSR compliance: NOT MET
```

### Step 2: Create Checklist

Based on the output, create a checklist of missing items:

```markdown
## Migration Checklist

### Documentation
- [ ] LICENSE.txt
- [ ] SECURITY.md
- [ ] CONTRIBUTING.md
- [ ] CODE_OF_CONDUCT.md
- [ ] MAINTAINERS.md
- [ ] CHANGELOG.md

### .well-known
- [ ] .well-known/security.txt
- [ ] .well-known/ai.txt
- [ ] .well-known/humans.txt

### Build System
- [ ] justfile
- [ ] flake.nix
- [ ] .gitlab-ci.yml (or GitHub Actions)

### Source Structure
- [ ] Organize into src/
- [ ] Create tests/ directory
```

## Documentation Migration

### LICENSE.txt

**If you already have a license:**

```bash
# Rename existing license
mv LICENSE LICENSE.txt

# Or create dual-license
cat > LICENSE.txt << 'EOF'
# Dual License

This project is dual-licensed under:
- MIT License (see LICENSE-MIT.txt)
- Apache License 2.0 (see LICENSE-APACHE.txt)

Choose the license that best fits your needs.
EOF
```

**If you need to choose a license:**

```bash
# MIT License (recommended for maximum compatibility)
curl -o LICENSE-MIT.txt https://opensource.org/licenses/MIT

# Or Apache 2.0
curl -o LICENSE-APACHE.txt https://www.apache.org/licenses/LICENSE-2.0.txt
```

### SECURITY.md

```bash
# Create from template
cat > SECURITY.md << 'EOF'
# Security Policy

## Supported Versions

| Version | Supported          |
| ------- | ------------------ |
| 1.x.x   | :white_check_mark: |
| < 1.0   | :x:                |

## Reporting a Vulnerability

Please report security vulnerabilities to: security@example.org

**DO NOT** create public issues for security vulnerabilities.

### Response Timeline

- Acknowledgment: Within 48 hours
- Assessment: Within 7 days
- Fix: 30 days for critical, 90 days for others

See [.well-known/security.txt](.well-known/security.txt) for more details.
EOF
```

### CONTRIBUTING.md

```bash
# Create from template
cat > CONTRIBUTING.md << 'EOF'
# Contributing

Thank you for your interest in contributing!

## Quick Start

1. Fork the repository
2. Create a feature branch: `git checkout -b feature/my-feature`
3. Make your changes
4. Test your changes
5. Commit with clear messages
6. Push and create a pull request

## Code Style

- Follow existing code style
- Write tests for new features
- Update documentation

## Pull Request Process

1. Ensure all tests pass
2. Update documentation
3. Add entry to CHANGELOG.md
4. Request review from maintainers

## Code of Conduct

See [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md)
EOF
```

### CODE_OF_CONDUCT.md

```bash
# Use Contributor Covenant
curl -o CODE_OF_CONDUCT.md https://www.contributor-covenant.org/version/2/1/code_of_conduct/code_of_conduct.md
```

### MAINTAINERS.md

```bash
cat > MAINTAINERS.md << 'EOF'
# Maintainers

## Current Maintainers

**[Your Name]**
- Role: Project Lead
- Email: you@example.org
- GitHub: @yourhandle

## Becoming a Maintainer

Maintainers are invited based on sustained contributions.

Criteria:
- 10+ merged PRs
- 3+ months of engagement
- Alignment with project values

## Decision Making

- Minor changes: Single maintainer approval
- Major changes: Consensus among maintainers
- Emergency: Any maintainer can act, notify others within 24h
EOF
```

### CHANGELOG.md

```bash
cat > CHANGELOG.md << 'EOF'
# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- RSR Bronze-level compliance

## [1.0.0] - YYYY-MM-DD

### Added
- Initial release
EOF
```

## Security Setup

### Create .well-known Directory

```bash
mkdir -p .well-known
```

### security.txt (RFC 9116)

```bash
cat > .well-known/security.txt << 'EOF'
Contact: mailto:security@example.org
Expires: 2026-12-31T23:59:59.000Z
Preferred-Languages: en
Canonical: https://example.org/.well-known/security.txt
Policy: https://example.org/security-policy
EOF
```

**Important**: Update expiration date and contact info!

### ai.txt

```bash
cat > .well-known/ai.txt << 'EOF'
# AI Training and Usage Policy

License: MIT
Training: Allowed with attribution
Usage: Commercial and non-commercial use permitted
Attribution: Please credit the project

Contact: ai-policy@example.org
EOF
```

### humans.txt

```bash
cat > .well-known/humans.txt << 'EOF'
/* TEAM */

Project: Your Project Name
Purpose: Your project purpose
Team: Your team members
Contact: contact@example.org

/* THANKS */

Contributors: See GitHub contributors page
Tools: Rust, Cargo, Just, Nix

/* SITE */

Repository: https://github.com/yourorg/yourproject
License: MIT
Last Updated: YYYY-MM-DD
EOF
```

## Build System

### justfile

```bash
cat > justfile << 'EOF'
# Build automation with Just

default:
    @just --list

build:
    cargo build

build-release:
    cargo build --release

test:
    cargo test

check:
    cargo fmt --check
    cargo clippy -- -D warnings
    cargo test

run:
    cargo run

clean:
    cargo clean
EOF
```

### flake.nix (for Nix users)

```bash
cat > flake.nix << 'EOF'
{
  description = "Your project description";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, rust-overlay }:
    let
      system = "x86_64-linux";
      pkgs = import nixpkgs {
        inherit system;
        overlays = [ (import rust-overlay) ];
      };
    in
    {
      devShells.${system}.default = pkgs.mkShell {
        buildInputs = with pkgs; [
          rust-bin.stable.latest.default
          just
        ];
      };
    };
}
EOF
```

### CI/CD (.gitlab-ci.yml or GitHub Actions)

**GitLab CI:**

```bash
cat > .gitlab-ci.yml << 'EOF'
stages:
  - check
  - test
  - build

format:check:
  stage: check
  image: rust:latest
  script:
    - cargo fmt --check

clippy:check:
  stage: check
  image: rust:latest
  script:
    - cargo clippy -- -D warnings

test:
  stage: test
  image: rust:latest
  script:
    - cargo test

build:
  stage: build
  image: rust:latest
  script:
    - cargo build --release
  artifacts:
    paths:
      - target/release/
EOF
```

**GitHub Actions:**

```bash
mkdir -p .github/workflows
cat > .github/workflows/ci.yml << 'EOF'
name: CI

on: [push, pull_request]

jobs:
  check:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      - run: cargo fmt --check
      - run: cargo clippy -- -D warnings

  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      - run: cargo test

  build:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      - run: cargo build --release
EOF
```

## Source Organization

### Reorganize Source Files

```bash
# Create directories
mkdir -p src tests

# Move source files (example for Rust)
mv *.rs src/ 2>/dev/null || true

# Create a basic main.rs if needed
cat > src/main.rs << 'EOF'
fn main() {
    println!("Hello, RSR!");
}
EOF

# Create a basic test
cat > tests/integration_test.rs << 'EOF'
#[test]
fn test_basic() {
    assert_eq!(2 + 2, 4);
}
EOF
```

## Testing

### Add Tests

```bash
# For Rust
cat >> src/lib.rs << 'EOF'
#[cfg(test)]
mod tests {
    #[test]
    fn test_something() {
        assert!(true);
    }
}
EOF

# Run tests
cargo test
```

## Verification

### Final Verification

```bash
# Run Aletheia
aletheia .

# Should see:
# 16/16 checks passed (100.0%)
# 🏆 Bronze-level RSR compliance: ACHIEVED
```

### Commit Changes

```bash
git add .
git commit -m "feat: achieve RSR Bronze-level compliance

- Add complete documentation suite
- Create .well-known directory with security.txt
- Set up build automation (justfile, flake.nix)
- Add CI/CD pipeline
- Organize source structure
- Add comprehensive tests

Verified with Aletheia - 16/16 checks passed"
```

## Common Issues

### Issue: Multiple license files

**Problem**: You have `LICENSE`, `LICENSE.md`, and `LICENSE.txt`

**Solution**:
```bash
# Choose one format, use LICENSE.txt
mv LICENSE LICENSE.txt
rm LICENSE.md  # If redundant
```

### Issue: Tests in wrong location

**Problem**: Tests are in `test/` but Aletheia expects `tests/`

**Solution**:
```bash
mv test tests
```

### Issue: Missing Cargo.toml

**Problem**: Rust project without Cargo.toml

**Solution**:
```bash
cargo init --name your-project-name
```

## Language-Specific Guides

### Python Projects

```bash
# Create src/
mkdir -p src tests

# Move Python files
mv *.py src/

# Create __init__.py
touch src/__init__.py

# Create basic test
cat > tests/test_basic.py << 'EOF'
def test_example():
    assert True
EOF

# Create justfile for Python
cat > justfile << 'EOF'
test:
    pytest

lint:
    flake8 src/
    mypy src/

format:
    black src/ tests/
EOF
```

### JavaScript/TypeScript

```bash
# Create directories
mkdir -p src tests

# Move source files
mv *.ts src/ 2>/dev/null || true
mv *.js src/ 2>/dev/null || true

# Create justfile for Node
cat > justfile << 'EOF'
build:
    npm run build

test:
    npm test

lint:
    npm run lint

format:
    npm run format
EOF
```

## Timeline

Typical migration timeline:

- **Day 1**: Assessment and planning (1-2 hours)
- **Day 2**: Documentation (2-3 hours)
- **Day 3**: Security and .well-known (1 hour)
- **Day 4**: Build system and CI/CD (2-3 hours)
- **Day 5**: Source organization and testing (2-4 hours)
- **Day 6**: Verification and cleanup (1 hour)

**Total**: 10-15 hours for most projects

## Resources

- [RSR Specification](RSR-SPECIFICATION.md)
- [Aletheia Documentation](https://gitlab.com/maa-framework/6-the-foundation/aletheia)
- [Keep a Changelog](https://keepachangelog.com/)
- [Contributor Covenant](https://www.contributor-covenant.org/)
- [RFC 9116 (security.txt)](https://www.rfc-editor.org/rfc/rfc9116.html)

## Support

Need help with migration?

- **Issues**: https://gitlab.com/maa-framework/6-the-foundation/aletheia/-/issues
- **Email**: maintainers@maa-framework.org

---

*Make your repository RSR-compliant - pursue alētheia!*
