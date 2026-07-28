# SPDX-License-Identifier: MPL-2.0
# justfile - Just recipes for this project
# See: https://github.com/hyperpolymath/mustfile

# Default recipe
import? "contractile.just"

default:
    @just --list

# These recipes mirror the root `.github/workflows/rust-ci.yml` gate exactly, so
# `just check` locally means the same thing as CI. They MUST fail loudly — a recipe
# that echoes and exits 0 is a fake gate (issue #99 was exactly that).

# Build aletheia (debug + release, locked)
build:
    cd aletheia && cargo build --locked --all-targets
    cd aletheia && cargo build --locked --release

# Run aletheia's unit tests (--bins only; see #124 note in recipe)
test:
    # `--bins` is deliberate. The integration suite is a specification for a CLI
    # that has not been built yet — 27 of 29 fail by design (issue #124).
    # Do NOT add `--tests` here to make the bar look green.
    cd aletheia && cargo test --locked --bins

# Check formatting (does not modify files)
fmt:
    cd aletheia && cargo fmt --check

# Apply formatting
fmt-fix:
    cd aletheia && cargo fmt

# Lint aletheia (not yet -D warnings — issue #125)
lint:
    # 23 findings remain, mostly dead code that exists because the CLI is
    # unwired (#124). When #125 closes, add `-- -D warnings` here AND to
    # rust-ci.yml in the same change, so local and CI never disagree about
    # what "lint passes" means.
    cd aletheia && cargo clippy --locked --all-targets

# Enforce the zero-dependency RSR Bronze constraint (mirrors rust-ci.yml)
deps-check:
    #!/usr/bin/env bash
    set -euo pipefail
    cd aletheia
    if cargo tree --depth 1 | tail -n +2 | grep -q '[a-z]'; then
      echo "ERROR: Aletheia must have zero dependencies (see aletheia/CLAUDE.md)"
      cargo tree --depth 1
      exit 1
    fi
    echo "OK: zero dependencies"

# Everything the root CI gate runs, in the same order
check: build test fmt deps-check

# Self-verify: run aletheia against this repository
self-verify:
    cd aletheia && cargo run --locked --quiet -- ..

# Clean build artifacts
clean:
    cd aletheia && cargo clean

# Run panic-attacker pre-commit scan
assail:
    @command -v panic-attack >/dev/null 2>&1 && panic-attack assail . || echo "panic-attack not found — install from https://github.com/hyperpolymath/panic-attacker"

# Self-diagnostic — checks dependencies, permissions, paths
doctor:
    @echo "Running diagnostics for maa-framework..."
    @echo "Checking required tools..."
    @command -v just >/dev/null 2>&1 && echo "  [OK] just" || echo "  [FAIL] just not found"
    @command -v git >/dev/null 2>&1 && echo "  [OK] git" || echo "  [FAIL] git not found"
    @echo "Checking for hardcoded paths..."
    @grep -rn '$HOME\|$ECLIPSE_DIR' --include='*.rs' --include='*.ex' --include='*.res' --include='*.gleam' --include='*.sh' . 2>/dev/null | head -5 || echo "  [OK] No hardcoded paths"
    @echo "Diagnostics complete."

# Auto-repair common issues
heal:
    @echo "Attempting auto-repair for maa-framework..."
    @echo "Fixing permissions..."
    @find . -name "*.sh" -exec chmod +x {} \; 2>/dev/null || true
    @echo "Cleaning stale caches..."
    @rm -rf .cache/stale 2>/dev/null || true
    @echo "Repair complete."

# Guided tour of key features
tour:
    @echo "=== maa-framework Tour ==="
    @echo ""
    @echo "1. Project structure:"
    @ls -la
    @echo ""
    @echo "2. Available commands: just --list"
    @echo ""
    @echo "3. Read README.adoc for full overview"
    @echo "4. Read EXPLAINME.adoc for architecture decisions"
    @echo "5. Run 'just doctor' to check your setup"
    @echo ""
    @echo "Tour complete! Try 'just --list' to see all available commands."

# Open feedback channel with diagnostic context
help-me:
    @echo "=== maa-framework Help ==="
    @echo "Platform: $(uname -s) $(uname -m)"
    @echo "Shell: $SHELL"
    @echo ""
    @echo "To report an issue:"
    @echo "  https://github.com/hyperpolymath/maa-framework/issues/new"
    @echo ""
    @echo "Include the output of 'just doctor' in your report."


# Print the current CRG grade (reads from READINESS.md '**Current Grade:** X' line)
crg-grade:
    @grade=$$(grep -oP '(?<=\*\*Current Grade:\*\* )[A-FX]' READINESS.md 2>/dev/null | head -1); \
    [ -z "$$grade" ] && grade="X"; \
    echo "$$grade"

# Generate a shields.io badge markdown for the current CRG grade
# Looks for '**Current Grade:** X' in READINESS.md; falls back to X
crg-badge:
    @grade=$$(grep -oP '(?<=\*\*Current Grade:\*\* )[A-FX]' READINESS.md 2>/dev/null | head -1); \
    [ -z "$$grade" ] && grade="X"; \
    case "$$grade" in \
      A) color="brightgreen" ;; B) color="green" ;; C) color="yellow" ;; \
      D) color="orange" ;; E) color="red" ;; F) color="critical" ;; \
      *) color="lightgrey" ;; esac; \
    echo "[![CRG $$grade](https://img.shields.io/badge/CRG-$$grade-$$color?style=flat-square)](https://github.com/hyperpolymath/standards/tree/main/component-readiness-grades)"
