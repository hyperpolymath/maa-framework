#!/usr/bin/env bash
# SPDX-License-Identifier: MPL-2.0
# Copyright (c) 2026 Jonathan D.A. Jewell (hyperpolymath)
#
# End-to-end test: full proof verification pipeline
# Verifies: Coq proofs compile, Lean proofs compile, Z3 checks pass,
# Rust builds, benchmarks run, interpreters work

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
AZ_DIR="$(cd "${SCRIPT_DIR}/../.." && pwd)"
PASS=0
FAIL=0
SKIP=0

log_pass() { echo "[PASS] $1"; ((PASS++)); }
log_fail() { echo "[FAIL] $1"; ((FAIL++)); }
log_skip() { echo "[SKIP] $1"; ((SKIP++)); }

echo "=== Absolute Zero E2E Verification ==="
echo "Directory: ${AZ_DIR}"
echo ""

# --- Rust build ---
echo "--- Rust Build ---"
if command -v cargo >/dev/null 2>&1; then
    if (cd "${AZ_DIR}" && cargo build --release 2>/dev/null); then
        log_pass "cargo build --release"
    else
        log_fail "cargo build --release"
    fi
    if (cd "${AZ_DIR}" && cargo test -- --test-threads=1 2>/dev/null); then
        log_pass "cargo test (unit tests)"
    else
        log_fail "cargo test (unit tests)"
    fi
else
    log_skip "cargo not installed"
fi

# --- Brainfuck interpreter ---
echo ""
echo "--- Brainfuck CNO Interpreter ---"
if [ -f "${AZ_DIR}/src/brainfuck/src/lib.rs" ]; then
    if (cd "${AZ_DIR}/src/brainfuck" && cargo build 2>/dev/null); then
        log_pass "brainfuck-cno build"
    else
        log_fail "brainfuck-cno build"
    fi
    if (cd "${AZ_DIR}/src/brainfuck" && cargo test 2>/dev/null); then
        log_pass "brainfuck-cno tests"
    else
        log_fail "brainfuck-cno tests"
    fi
else
    log_skip "brainfuck interpreter not found"
fi

# --- Whitespace interpreter ---
echo ""
echo "--- Whitespace CNO Interpreter ---"
if [ -f "${AZ_DIR}/src/whitespace/src/lib.rs" ]; then
    if (cd "${AZ_DIR}/src/whitespace" && cargo build 2>/dev/null); then
        log_pass "whitespace-cno build"
    else
        log_fail "whitespace-cno build"
    fi
else
    log_skip "whitespace interpreter not found"
fi

# --- Coq proofs ---
echo ""
echo "--- Coq Proofs ---"
if command -v coqc >/dev/null 2>&1; then
    coq_pass=0
    coq_fail=0
    for vfile in "${AZ_DIR}"/proofs/coq/**/*.v; do
        if [ -f "$vfile" ]; then
            name="$(basename "$vfile")"
            if coqc "$vfile" 2>/dev/null; then
                log_pass "coqc ${name}"
                ((coq_pass++))
            else
                log_fail "coqc ${name}"
                ((coq_fail++))
            fi
        fi
    done
    echo "  Coq: ${coq_pass} passed, ${coq_fail} failed"
else
    log_skip "coqc not installed"
fi

# --- Lean 4 proofs ---
echo ""
echo "--- Lean 4 Proofs ---"
if command -v lake >/dev/null 2>&1; then
    if (cd "${AZ_DIR}/proofs/lean4" && lake build 2>/dev/null); then
        log_pass "lake build (Lean 4)"
    else
        log_fail "lake build (Lean 4)"
    fi
else
    log_skip "lake (Lean 4) not installed"
fi

# --- Z3 SMT ---
echo ""
echo "--- Z3 SMT Verification ---"
if command -v z3 >/dev/null 2>&1; then
    if [ -f "${AZ_DIR}/proofs/z3/verify.sh" ]; then
        if (cd "${AZ_DIR}/proofs/z3" && bash verify.sh 2>/dev/null); then
            log_pass "Z3 verification"
        else
            log_fail "Z3 verification"
        fi
    else
        log_skip "Z3 verify.sh not found"
    fi
else
    log_skip "z3 not installed"
fi

# --- Agda proofs ---
echo ""
echo "--- Agda Proofs ---"
if command -v agda >/dev/null 2>&1; then
    if [ -f "${AZ_DIR}/proofs/agda/CNO.agda" ]; then
        if agda --safe "${AZ_DIR}/proofs/agda/CNO.agda" 2>/dev/null; then
            log_pass "agda --safe CNO.agda"
        else
            log_fail "agda --safe CNO.agda"
        fi
    else
        log_skip "CNO.agda not found"
    fi
else
    log_skip "agda not installed"
fi

# --- Zig FFI ---
echo ""
echo "--- Zig FFI ---"
if command -v zig >/dev/null 2>&1; then
    if [ -f "${AZ_DIR}/ffi/zig/build.zig" ]; then
        if (cd "${AZ_DIR}/ffi/zig" && zig build test 2>/dev/null); then
            log_pass "zig build test (FFI)"
        else
            log_fail "zig build test (FFI)"
        fi
    else
        log_skip "Zig FFI not found"
    fi
else
    log_skip "zig not installed"
fi

# --- Panic Attack ---
echo ""
echo "--- Panic Attack Assail ---"
if command -v panic-attack >/dev/null 2>&1; then
    report_file="$(mktemp)"
    if panic-attack assail "${AZ_DIR}" --output-format json --output "${report_file}" --quiet 2>/dev/null; then
        wp_count=$(python3 -c "import json; d=json.load(open('${report_file}')); print(len(d.get('weak_points',[])))" 2>/dev/null || echo "?")
        log_pass "panic-attack assail (${wp_count} weak points)"
    else
        log_fail "panic-attack assail"
    fi
    rm -f "${report_file}"
else
    log_skip "panic-attack not installed"
fi

# --- Summary ---
echo ""
echo "==============================="
echo "  PASS: ${PASS}"
echo "  FAIL: ${FAIL}"
echo "  SKIP: ${SKIP}"
echo "==============================="

if [ "${FAIL}" -gt 0 ]; then
    exit 1
fi
