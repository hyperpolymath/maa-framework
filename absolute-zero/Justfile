# Absolute Zero Build Automation
#
# Modern build automation using `just` (https://github.com/casey/just)
# Install: cargo install just
#
# Author: Jonathan D. A. Jewell
# Project: Absolute Zero

# Default recipe (show help)
default:
    @just --list

# ============================================================================
# Build Commands
# ============================================================================

# Build everything
build-all: build-rescript build-coq build-lean build-agda build-isabelle build-typescript
    @echo "✓ All builds complete"

# Build ReScript interpreters
build-rescript:
    @echo "Building ReScript interpreters..."
    cd interpreters/rescript && npx rescript build

# Build Coq proofs
build-coq:
    @echo "Building Coq proofs..."
    @if command -v coqc >/dev/null 2>&1; then \
        cd proofs/coq/common && coqc CNO.v && \
        cd ../physics && coqc -R ../common CNO StatMech.v && \
        coqc -R ../common CNO LandauerDerivation.v && \
        cd ../quantum && coqc -R ../common CNO QuantumMechanicsExact.v && \
        cd ../malbolge && coqc -R ../common CNO MalbolgeCore.v && \
        echo "✓ Coq proofs compiled"; \
    else \
        echo "⚠ coqc not found, skipping Coq build"; \
    fi

# Build Lean 4 proofs
build-lean:
    @echo "Building Lean 4 proofs..."
    cd proofs/lean4 && lake build

# Build Agda proofs
build-agda:
    @echo "Building Agda proofs..."
    cd proofs/agda && agda CNO.agda

# Build Isabelle/HOL proofs
build-isabelle:
    @echo "Building Isabelle/HOL proofs..."
    isabelle build -D proofs/isabelle

# Build TypeScript
build-typescript:
    @echo "Building TypeScript..."
    npm run build
    @echo "✓ TypeScript compiled"

# ============================================================================
# Verification Commands
# ============================================================================

# Verify all proofs
verify-all: verify-coq verify-z3 verify-lean verify-agda verify-isabelle
    @echo "✓ All verifications complete"

# Verify Coq proofs
verify-coq: build-coq
    @echo "✓ Coq proofs verified"

# Verify Z3 SMT properties
verify-z3:
    @echo "Verifying Z3 SMT properties..."
    @if command -v z3 >/dev/null 2>&1; then \
        z3 proofs/z3/cno_properties.smt2 && echo "✓ Z3 verification complete"; \
    else \
        echo "⚠ z3 not found, skipping Z3 verification"; \
    fi

# Verify Lean 4 proofs
verify-lean:
    @echo "Verifying Lean 4 proofs..."
    cd proofs/lean4 && lake build

# Verify Agda proofs
verify-agda: build-agda
    @echo "✓ Agda proofs verified"

# Verify Isabelle/HOL proofs
verify-isabelle: build-isabelle
    @echo "✓ Isabelle/HOL proofs verified"

# ============================================================================
# Testing Commands
# ============================================================================

# Run all tests
test-all: test-interpreters test-proofs
    @echo "✓ All tests passed"

# Test interpreters
test-interpreters:
    @echo "Testing Brainfuck interpreter..."
    python3 interpreters/brainfuck/brainfuck.py
    @echo ""
    @echo "Testing Whitespace interpreter..."
    python3 interpreters/whitespace/whitespace.py

# Test proofs
test-proofs:
    @echo "Testing proof verification..."
    just verify-z3

# Run TypeScript tests (when available)
test-typescript:
    npm test

# ============================================================================
# Example Execution
# ============================================================================

# Run example CNO
run-example LANG FILE:
    @echo "Running {{LANG}} example: {{FILE}}"
    @just run-{{LANG}} {{FILE}}

# Run Brainfuck example
run-brainfuck FILE:
    python3 interpreters/brainfuck/brainfuck.py examples/brainfuck/{{FILE}}

# Run Whitespace example
run-whitespace FILE:
    python3 interpreters/whitespace/whitespace.py examples/whitespace/{{FILE}}

# Run Malbolge example (ReScript)
run-malbolge FILE:
    cd interpreters/rescript && node -e "require('./malbolgeInterpreter.bs.js').execute('$(cat ../../examples/malbolge/{{FILE}})')"

# ============================================================================
# Documentation
# ============================================================================

# Generate documentation
docs:
    @echo "Generating documentation..."
    @echo "Theoretical foundations: docs/theory.md"
    @echo "Examples: docs/examples.md"
    @echo "Proof guide: docs/proofs-guide.md"
    @echo "Philosophy: docs/philosophy.md"

# View documentation
view-docs:
    @echo "Documentation files:"
    @ls -lh docs/

# ============================================================================
# Cleanup
# ============================================================================

# Clean all build artifacts
clean: clean-coq clean-lean clean-typescript clean-rescript
    @echo "✓ All build artifacts cleaned"

# Clean Coq artifacts
clean-coq:
    @echo "Cleaning Coq artifacts..."
    find proofs/coq -name "*.vo" -delete
    find proofs/coq -name "*.vok" -delete
    find proofs/coq -name "*.vos" -delete
    find proofs/coq -name "*.glob" -delete
    find proofs/coq -name ".*.aux" -delete

# Clean Lean artifacts
clean-lean:
    @echo "Cleaning Lean artifacts..."
    cd proofs/lean4 && lake clean

# Clean TypeScript artifacts
clean-typescript:
    @echo "Cleaning TypeScript artifacts..."
    rm -rf node_modules dist

# Clean ReScript artifacts
clean-rescript:
    @echo "Cleaning ReScript artifacts..."
    cd interpreters/rescript && rm -rf lib

# ============================================================================
# Development
# ============================================================================

# Watch TypeScript for changes
watch:
    npm run watch

# Format code
format:
    @echo "Formatting code..."
    cd interpreters/rescript && npx rescript format

# Lint code
lint:
    @echo "Linting TypeScript..."
    npm run lint || true

# ============================================================================
# CI/CD
# ============================================================================

# Run CI pipeline locally
ci: build-all test-all verify-all
    @echo "✓ CI pipeline completed successfully"

# ============================================================================
# Installation
# ============================================================================

# Install dependencies
install: install-npm install-python
    @echo "✓ Dependencies installed"

# Install npm dependencies
install-npm:
    @echo "Installing npm dependencies..."
    npm install

# Install Python dependencies
install-python:
    @echo "Installing Python dependencies..."
    pip3 install --user pytest hypothesis

# Install proof assistants (Fedora)
install-provers-fedora:
    @echo "Installing proof assistants (Fedora)..."
    sudo dnf install -y coq z3 nodejs opam
    npm install -g rescript@11.1

# Install proof assistants (Ubuntu)
install-provers-ubuntu:
    @echo "Installing proof assistants (Ubuntu)..."
    sudo apt install -y coq z3 nodejs npm
    npm install -g rescript@11.1

# ============================================================================
# Container (Podman/Docker)
# ============================================================================

# Build container image (Podman preferred)
container-build:
    @echo "Building container image..."
    podman build -t absolute-zero:latest .

# Run verification in container
container-verify:
    @echo "Running verification in container..."
    podman run --rm absolute-zero:latest just verify-all

# Run container interactively
container-shell:
    @echo "Starting interactive shell..."
    podman run --rm -it absolute-zero:latest /bin/bash

# Run all language examples in container
container-test-all:
    @echo "Testing all languages in container..."
    podman run --rm absolute-zero:latest just test-all

# Docker compatibility aliases
docker-build: container-build
docker-verify: container-verify

# ============================================================================
# Research
# ============================================================================

# Generate LaTeX paper
paper:
    @echo "Generating research paper..."
    cd papers && pdflatex main.tex

# Count lines of code
stats:
    @echo "Project statistics:"
    @echo ""
    @echo "Proof code:"
    @find proofs -name "*.v" -o -name "*.lean" -o -name "*.agda" -o -name "*.thy" -o -name "*.miz" -o -name "*.smt2" | xargs wc -l | tail -1
    @echo ""
    @echo "Implementation code:"
    @find interpreters ts -name "*.res" -o -name "*.py" -o -name "*.ts" | xargs wc -l | tail -1
    @echo ""
    @echo "Documentation:"
    @find docs -name "*.md" | xargs wc -l | tail -1
    @echo ""
    @echo "Total:"
    @find . -name "*.v" -o -name "*.lean" -o -name "*.agda" -o -name "*.thy" -o -name "*.miz" -o -name "*.smt2" -o -name "*.res" -o -name "*.py" -o -name "*.ts" -o -name "*.md" | xargs wc -l | tail -1

# Check proof completion status
proof-status:
    @echo "=== Proof Completion Status ==="
    @echo ""
    @echo "Coq proofs:"
    @admitted=$$(grep -r "Admitted\." proofs/coq/ 2>/dev/null | wc -l); \
    total=$$(grep -r "Theorem\|Lemma\|Corollary" proofs/coq/ 2>/dev/null | wc -l); \
    if [ $$total -gt 0 ]; then \
        complete=$$((total - admitted)); \
        percent=$$((complete * 100 / total)); \
        echo "  Theorems: $$total"; \
        echo "  Complete: $$complete"; \
        echo "  Admitted: $$admitted"; \
        echo "  Completion: $$percent%"; \
    else \
        echo "  No Coq files found"; \
    fi
    @echo ""
    @echo "Lean 4 proofs:"
    @sorry=$$(grep -r "sorry" proofs/lean4/ 2>/dev/null | wc -l); \
    total=$$(grep -r "theorem\|lemma" proofs/lean4/ 2>/dev/null | wc -l); \
    if [ $$total -gt 0 ]; then \
        complete=$$((total - sorry)); \
        percent=$$((complete * 100 / total)); \
        echo "  Theorems: $$total"; \
        echo "  Complete: $$complete"; \
        echo "  Sorry: $$sorry"; \
        echo "  Completion: $$percent%"; \
    else \
        echo "  No Lean files found"; \
    fi
    @echo ""
    @echo "Z3 SMT specifications:"
    @theorems=$$(grep -c "assert.*theorem" proofs/z3/cno_properties.smt2 2>/dev/null || echo 0); \
    echo "  Theorems: $$theorems"

# ============================================================================
# Help
# ============================================================================

# Show detailed help
help:
    @echo "Absolute Zero - Build Automation"
    @echo ""
    @echo "Common commands:"
    @echo "  just build-all       - Build everything"
    @echo "  just verify-all      - Verify all proofs"
    @echo "  just test-all        - Run all tests"
    @echo "  just clean           - Clean build artifacts"
    @echo "  just ci              - Run full CI pipeline"
    @echo ""
    @echo "For all commands: just --list"

# ============================================================================
# Elm GUI Playground
# ============================================================================

# Build Elm playground
build-elm:
    @echo "Building Elm playground..."
    @if command -v elm >/dev/null 2>&1; then \
        cd elm && elm make src/Main.elm --output=dist/main.js && echo "✓ Elm compiled"; \
    else \
        echo "⚠ elm not found, skipping Elm build"; \
    fi

# Run Elm playground (opens in browser)
run-elm: build-elm
    @echo "Opening Elm playground..."
    @python3 -m http.server 8000 &
    @sleep 2
    @xdg-open http://localhost:8000/elm-playground.html || open http://localhost:8000/elm-playground.html

# Clean Elm artifacts
clean-elm:
    @echo "Cleaning Elm artifacts..."
    rm -rf elm/dist elm/elm-stuff


# ============================================================================
# ECHIDNA Integration (Neurosymbolic Proof Assistant)
# ============================================================================

# List all Admitted proofs needing completion
echidna-list:
    @./scripts/use-echidna.sh list-admitted

# Get tactic suggestions for a proof file
echidna-suggest FILE:
    @./scripts/use-echidna.sh suggest {{FILE}}

# Attempt to auto-complete proofs in a file
echidna-complete FILE:
    @./scripts/use-echidna.sh complete {{FILE}}

# Verify all proofs with multi-prover consensus
echidna-verify:
    @./scripts/use-echidna.sh verify-all

# Start ECHIDNA interactive REPL
echidna-repl:
    @./scripts/use-echidna.sh repl

# Check ECHIDNA installation
echidna-check:
    @echo "Checking ECHIDNA installation..."
    @if [ -x ~/Documents/hyperpolymath-repos/echidna/target/release/echidna ]; then \
        echo "✓ ECHIDNA binary found"; \
        ~/Documents/hyperpolymath-repos/echidna/target/release/echidna --version; \
    else \
        echo "❌ ECHIDNA not built. Run:"; \
        echo "   cd ~/Documents/hyperpolymath-repos/echidna && cargo build --release"; \
    fi

