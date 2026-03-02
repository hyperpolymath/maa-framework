# Quick Start Guide

Get started with Aletheia in 5 minutes.

## Installation

### Option 1: Build from Source (Recommended)

```bash
# Clone the repository
git clone https://gitlab.com/maa-framework/6-the-foundation/aletheia.git
cd aletheia

# Build release binary
cargo build --release

# Run verification
cargo run
```

### Option 2: Using Nix (Reproducible Build)

```bash
# Clone the repository
git clone https://gitlab.com/maa-framework/6-the-foundation/aletheia.git
cd aletheia

# Enter development environment
nix develop

# Build and run
just build
just run
```

### Option 3: Direct Installation

```bash
# Install directly from repository
cargo install --git https://gitlab.com/maa-framework/6-the-foundation/aletheia.git
```

## First Verification

### Verify Current Directory

```bash
cargo run
```

This verifies the current directory against RSR Bronze-level standards.

### Verify Specific Repository

```bash
cargo run -- /path/to/your/repository
```

## Understanding the Output

### Passing Example

```
🔍 Aletheia - RSR Compliance Verification Report
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Repository: /home/user/my-project

📋 Documentation
  ✅ README.md [Bronze]
  ✅ LICENSE.txt [Bronze]
  ✅ SECURITY.md [Bronze]
  ✅ CONTRIBUTING.md [Bronze]
  ✅ CODE_OF_CONDUCT.md [Bronze]
  ✅ MAINTAINERS.md [Bronze]
  ✅ CHANGELOG.md [Bronze]

📋 Well-Known
  ✅ .well-known/ directory [Bronze]
  ✅ security.txt [Bronze]
  ✅ ai.txt [Bronze]
  ✅ humans.txt [Bronze]

📋 Build System
  ✅ justfile [Bronze]
  ✅ flake.nix [Bronze]
  ✅ .gitlab-ci.yml [Bronze]

📋 Source Structure
  ✅ src/ directory [Bronze]
  ✅ tests/ directory [Bronze]

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Score: 16/16 checks passed (100.0%)
🏆 Bronze-level RSR compliance: ACHIEVED
```

**Exit code**: 0 (success)

### Failing Example

```
🔍 Aletheia - RSR Compliance Verification Report
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Repository: /home/user/incomplete-project

📋 Documentation
  ✅ README.md [Bronze]
  ❌ LICENSE.txt [Bronze]
  ❌ SECURITY.md [Bronze]
  ...

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Score: 8/16 checks passed (50.0%)
⚠️  Bronze-level RSR compliance: NOT MET
```

**Exit code**: 1 (failure)

## Making Your Repository Compliant

If your repository fails verification, follow these steps:

### 1. Create Missing Documentation

```bash
# Required files
touch README.md
touch LICENSE.txt
touch SECURITY.md
touch CONTRIBUTING.md
touch CODE_OF_CONDUCT.md
touch MAINTAINERS.md
touch CHANGELOG.md
```

See [examples in Aletheia repository](../) for content templates.

### 2. Create .well-known Directory

```bash
mkdir -p .well-known
touch .well-known/security.txt
touch .well-known/ai.txt
touch .well-known/humans.txt
```

**security.txt** (RFC 9116 format):
```
Contact: mailto:security@example.org
Expires: 2026-01-01T00:00:00.000Z
Preferred-Languages: en
Canonical: https://example.org/.well-known/security.txt
```

**ai.txt** (AI training policy):
```
# AI Training Policy
License: MIT
Training: Allowed with attribution
```

**humans.txt** (attribution):
```
/* TEAM */
Project: Your Project
Maintainers: Your Team
```

### 3. Create Build System Files

```bash
# Create justfile
touch justfile

# Create flake.nix (for Nix users)
touch flake.nix

# Create CI/CD
touch .gitlab-ci.yml  # or .github/workflows/ci.yml
```

### 4. Organize Source Structure

```bash
# Create required directories
mkdir -p src
mkdir -p tests

# Move source files to src/
mv *.rs src/  # (or your language)
```

### 5. Re-verify

```bash
cargo run
```

## Using with CI/CD

### GitLab CI

```yaml
# .gitlab-ci.yml
rsr-compliance:
  stage: test
  image: rust:1.75
  script:
    - cargo install --git https://gitlab.com/maa-framework/6-the-foundation/aletheia.git
    - aletheia
  allow_failure: false
```

### GitHub Actions

```yaml
# .github/workflows/rsr.yml
name: RSR Compliance
on: [push, pull_request]
jobs:
  verify:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      - run: cargo install --git https://gitlab.com/maa-framework/6-the-foundation/aletheia.git
      - run: aletheia
```

## Development Workflow

### Using Just (Recommended)

```bash
# List all commands
just

# Build
just build

# Run tests
just test

# Run all checks
just check

# Verify RSR compliance
just validate
```

### Using Cargo Directly

```bash
# Build
cargo build --release

# Run tests
cargo test

# Check formatting
cargo fmt --check

# Run linter
cargo clippy

# Verify
cargo run
```

## Common Issues

### Issue: "Path does not exist"

**Problem**: Specified path doesn't exist

**Solution**:
```bash
# Check path
ls /path/to/repository

# Use absolute path
cargo run -- /absolute/path/to/repository
```

### Issue: "Not a directory"

**Problem**: Path is a file, not a directory

**Solution**:
```bash
# Verify directory
cargo run -- /path/to/directory/
```

### Issue: Low compliance score

**Problem**: Missing required files

**Solution**:
1. Check which files are missing (marked with ❌)
2. Create missing files
3. Re-run verification

### Issue: Tests not found

**Problem**: No `tests/` directory

**Solution**:
```bash
mkdir tests
# Add test files
```

## Next Steps

- Read [ARCHITECTURE.md](ARCHITECTURE.md) - Understand how Aletheia works
- Read [RSR-SPECIFICATION.md](RSR-SPECIFICATION.md) - Learn RSR standards
- Read [CONTRIBUTING.md](../CONTRIBUTING.md) - Contribute to Aletheia

## Support

- **Issues**: https://gitlab.com/maa-framework/6-the-foundation/aletheia/-/issues
- **Documentation**: https://gitlab.com/maa-framework/6-the-foundation/aletheia
- **Email**: maintainers@maa-framework.org

---

*Start pursuing alētheia (truth) through rigorous standards!*
