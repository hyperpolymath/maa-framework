#!/usr/bin/env bash
# Script to create RSR Bronze-compliant project template
# Usage: ./create-template.sh <project-name> <language>

set -euo pipefail

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

# Configuration
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
TEMPLATE_DIR="$(dirname "$SCRIPT_DIR")/templates"

log_info() { echo -e "${GREEN}[INFO]${NC} $1"; }
log_warn() { echo -e "${YELLOW}[WARN]${NC} $1"; }
log_error() { echo -e "${RED}[ERROR]${NC} $1"; }
log_step() { echo -e "${BLUE}[STEP]${NC} $1"; }

usage() {
    cat <<EOF
Usage: $0 <project-name> [language]

Create an RSR Bronze-compliant project from template.

Arguments:
    project-name    Name of the new project
    language        Programming language (rust, python, typescript, go)
                   Default: rust

Examples:
    $0 my-awesome-project rust
    $0 data-analyzer python
    $0 web-app typescript

Supported Languages:
    - rust        Rust project with Cargo
    - python      Python project with pyproject.toml
    - typescript  TypeScript project with package.json
    - go          Go project with go.mod

This will create:
    - All required RSR Bronze documentation
    - .well-known directory with security.txt
    - Build automation (justfile, flake.nix)
    - Basic CI/CD configuration
    - Source structure (src/, tests/)
    - License files (MIT + Apache-2.0)
EOF
    exit 1
}

create_directory_structure() {
    local project_name=$1

    log_step "Creating directory structure..."

    mkdir -p "$project_name"/{src,tests,.well-known,docs,scripts}

    log_info "Directory structure created"
}

create_documentation() {
    local project_name=$1

    log_step "Creating documentation files..."

    # README.md
    cat > "$project_name/README.md" <<EOF
# $project_name

> RSR Bronze-compliant project template

## 🚀 Quick Start

\`\`\`bash
# Build
just build

# Test
just test

# Verify RSR compliance
aletheia .
\`\`\`

## 📊 RSR Compliance

**Level**: Bronze ✅

This project maintains RSR Bronze-level compliance:
- ✅ Type Safety
- ✅ Memory Safety
- ✅ Complete Documentation
- ✅ Security-First (.well-known)
- ✅ Build System
- ✅ Testing

## 🤝 Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md)

## 📜 License

Dual-licensed under MIT OR Apache-2.0

See [LICENSE-MIT.txt](LICENSE-MIT.txt) and [LICENSE-APACHE.txt](LICENSE-APACHE.txt)
EOF

    # LICENSE files
    curl -sSf https://opensource.org/licenses/MIT -o "$project_name/LICENSE-MIT.txt" 2>/dev/null || \
        echo "MIT License" > "$project_name/LICENSE-MIT.txt"

    curl -sSf https://www.apache.org/licenses/LICENSE-2.0.txt -o "$project_name/LICENSE-APACHE.txt" 2>/dev/null || \
        echo "Apache License 2.0" > "$project_name/LICENSE-APACHE.txt"

    cat > "$project_name/LICENSE.txt" <<EOF
# Dual License

This project is dual-licensed under:
- MIT License (see LICENSE-MIT.txt)
- Apache License 2.0 (see LICENSE-APACHE.txt)

You may use this project under the terms of either license.
EOF

    # SECURITY.md
    cat > "$project_name/SECURITY.md" <<EOF
# Security Policy

## Reporting a Vulnerability

Please report security vulnerabilities to: security@example.org

**DO NOT** create public issues for security vulnerabilities.

## Response Timeline

- Acknowledgment: Within 48 hours
- Assessment: Within 7 days
- Fix: 30 days for critical, 90 days for others

See [.well-known/security.txt](.well-known/security.txt)
EOF

    # CONTRIBUTING.md
    cat > "$project_name/CONTRIBUTING.md" <<EOF
# Contributing

Thank you for your interest in contributing!

## Quick Start

1. Fork the repository
2. Create feature branch: \`git checkout -b feature/my-feature\`
3. Make changes
4. Test changes: \`just test\`
5. Commit: \`git commit -m "feat: description"\`
6. Push and create pull request

## Code Style

- Follow existing code style
- Write tests for new features
- Update documentation

## Pull Request Process

1. Ensure all tests pass (\`just check\`)
2. Update CHANGELOG.md
3. Request review

See [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md)
EOF

    # CODE_OF_CONDUCT.md
    cat > "$project_name/CODE_OF_CONDUCT.md" <<EOF
# Code of Conduct

## Our Pledge

We pledge to make participation in our community a harassment-free experience for everyone.

## Our Standards

Positive behavior:
- Being respectful and inclusive
- Accepting constructive feedback
- Focusing on what is best for the community

Unacceptable behavior:
- Harassment or trolling
- Publishing others' private information
- Other unprofessional conduct

## Enforcement

Report violations to: conduct@example.org

## Attribution

Adapted from Contributor Covenant v2.1
EOF

    # MAINTAINERS.md
    cat > "$project_name/MAINTAINERS.md" <<EOF
# Maintainers

## Current Maintainers

**[Your Name]**
- Role: Project Lead
- Email: you@example.org

## Becoming a Maintainer

Maintainers are invited based on sustained contributions.

## Decision Making

- Minor changes: Single maintainer approval
- Major changes: Consensus among maintainers
EOF

    # CHANGELOG.md
    cat > "$project_name/CHANGELOG.md" <<EOF
# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/),
and this project adheres to [Semantic Versioning](https://semver.org/).

## [Unreleased]

### Added
- Initial project structure
- RSR Bronze-level compliance

## [0.1.0] - $(date +%Y-%m-%d)

### Added
- Initial release
EOF

    log_info "Documentation created"
}

create_well_known() {
    local project_name=$1

    log_step "Creating .well-known directory..."

    # security.txt
    cat > "$project_name/.well-known/security.txt" <<EOF
Contact: mailto:security@example.org
Expires: $(date -d "+1 year" -u +%Y-%m-%dT%H:%M:%S.000Z)
Preferred-Languages: en
Canonical: https://example.org/.well-known/security.txt
Policy: https://example.org/security
EOF

    # ai.txt
    cat > "$project_name/.well-known/ai.txt" <<EOF
# AI Training and Usage Policy

License: MIT OR Apache-2.0
Training: Allowed with attribution
Usage: Commercial and non-commercial permitted
Attribution: Please credit the project

Contact: ai-policy@example.org
EOF

    # humans.txt
    cat > "$project_name/.well-known/humans.txt" <<EOF
/* TEAM */

Project: $project_name
Team: Your Team
Contact: contact@example.org

/* THANKS */

Tools: Rust, Aletheia, RSR Framework

/* SITE */

Repository: https://example.org/$project_name
License: MIT OR Apache-2.0
Last Updated: $(date +%Y-%m-%d)
EOF

    log_info ".well-known directory created"
}

create_build_system() {
    local project_name=$1
    local language=$2

    log_step "Creating build system files..."

    # justfile
    case "$language" in
        rust)
            cat > "$project_name/justfile" <<EOF
# Build automation

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

validate:
    aletheia .
EOF
            ;;
        python)
            cat > "$project_name/justfile" <<EOF
# Build automation

default:
    @just --list

test:
    pytest

lint:
    flake8 src/
    mypy src/

format:
    black src/ tests/

check: format lint test

validate:
    aletheia .
EOF
            ;;
        *)
            cat > "$project_name/justfile" <<EOF
# Build automation

default:
    @just --list

build:
    echo "Build command here"

test:
    echo "Test command here"

validate:
    aletheia .
EOF
            ;;
    esac

    # flake.nix (basic)
    cat > "$project_name/flake.nix" <<EOF
{
  description = "$project_name";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };

  outputs = { self, nixpkgs }:
    let
      system = "x86_64-linux";
      pkgs = import nixpkgs { inherit system; };
    in
    {
      devShells.\${system}.default = pkgs.mkShell {
        buildInputs = with pkgs; [
          just
        ];
      };
    };
}
EOF

    log_info "Build system created"
}

create_language_specific() {
    local project_name=$1
    local language=$2

    log_step "Creating $language-specific files..."

    case "$language" in
        rust)
            cat > "$project_name/Cargo.toml" <<EOF
[package]
name = "$project_name"
version = "0.1.0"
edition = "2021"
license = "MIT OR Apache-2.0"

[dependencies]
# Zero dependencies for RSR Bronze compliance
EOF

            cat > "$project_name/src/main.rs" <<EOF
//! $project_name

fn main() {
    println!("Hello from $project_name!");
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_basic() {
        assert_eq!(2 + 2, 4);
    }
}
EOF

            cat > "$project_name/tests/integration_test.rs" <<EOF
#[test]
fn test_integration() {
    assert!(true);
}
EOF
            ;;

        python)
            cat > "$project_name/pyproject.toml" <<EOF
[project]
name = "$project_name"
version = "0.1.0"
description = "RSR Bronze-compliant Python project"
license = {text = "MIT OR Apache-2.0"}

[tool.black]
line-length = 100

[tool.mypy]
strict = true
EOF

            cat > "$project_name/src/__init__.py" <<'EOF'
"""Package initialization."""
EOF

            cat > "$project_name/src/main.py" <<'EOF'
"""Main module."""

def main() -> None:
    """Main entry point."""
    print("Hello!")

if __name__ == "__main__":
    main()
EOF

            cat > "$project_name/tests/test_main.py" <<'EOF'
"""Tests for main module."""

def test_example() -> None:
    """Example test."""
    assert True
EOF
            ;;
    esac

    log_info "$language files created"
}

create_ci_cd() {
    local project_name=$1

    log_step "Creating CI/CD configuration..."

    cat > "$project_name/.gitlab-ci.yml" <<EOF
# GitLab CI/CD Pipeline

stages:
  - check
  - test
  - verify

rsr-compliance:
  stage: verify
  image: rust:latest
  script:
    - cargo install --git https://gitlab.com/maa-framework/6-the-foundation/aletheia.git
    - aletheia .
  allow_failure: false
EOF

    log_info "CI/CD configuration created"
}

finalize() {
    local project_name=$1

    log_step "Finalizing project..."

    cd "$project_name"
    git init
    git add .
    git commit -m "chore: initial RSR Bronze-compliant project structure"

    log_info "Git repository initialized"

    echo ""
    log_info "✅ Project created successfully!"
    echo ""
    echo "Next steps:"
    echo "  1. cd $project_name"
    echo "  2. Update contact information in:"
    echo "     - .well-known/security.txt"
    echo "     - SECURITY.md"
    echo "     - MAINTAINERS.md"
    echo "  3. Build: just build"
    echo "  4. Test: just test"
    echo "  5. Verify RSR compliance: aletheia ."
    echo ""
}

main() {
    if [ $# -lt 1 ]; then
        usage
    fi

    local project_name=$1
    local language=${2:-rust}

    if [ -d "$project_name" ]; then
        log_error "Directory '$project_name' already exists"
        exit 1
    fi

    log_info "Creating RSR Bronze-compliant project: $project_name ($language)"
    echo ""

    create_directory_structure "$project_name"
    create_documentation "$project_name"
    create_well_known "$project_name"
    create_build_system "$project_name" "$language"
    create_language_specific "$project_name" "$language"
    create_ci_cd "$project_name"
    finalize "$project_name"
}

main "$@"
