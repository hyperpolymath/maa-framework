#!/usr/bin/env bash
# SPDX-License-Identifier: MPL-2.0
# Copyright (c) 2026 Jonathan D.A. Jewell (hyperpolymath)
#
# Aspect tests: cross-cutting concerns for absolute-zero
# Tests: SPDX headers, documentation, proof counts, forbidden patterns

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
AZ_DIR="$(cd "${SCRIPT_DIR}/../.." && pwd)"
PASS=0
FAIL=0

check() {
    if eval "$2"; then
        echo "[PASS] $1"
        ((PASS++))
    else
        echo "[FAIL] $1"
        ((FAIL++))
    fi
}

echo "=== Absolute Zero Aspect Tests ==="

# --- SPDX Headers ---
echo ""
echo "--- SPDX License Headers ---"
rs_count=$(find "${AZ_DIR}/src" -name '*.rs' 2>/dev/null | wc -l)
rs_spdx=$(grep -rl 'SPDX-License-Identifier' "${AZ_DIR}/src" --include='*.rs' 2>/dev/null | wc -l)
check "Rust files have SPDX headers (${rs_spdx}/${rs_count})" "[ '${rs_spdx}' -ge 1 ]"

# --- Forbidden Patterns ---
echo ""
echo "--- Forbidden Patterns ---"
check "No believe_me in proofs" "! grep -rq 'believe_me' '${AZ_DIR}/proofs/' 2>/dev/null"
check "No sorry in Lean proofs" "! grep -rq 'sorry' '${AZ_DIR}/proofs/lean4/' 2>/dev/null"
check "No Admitted in Coq proofs" "! grep -rq 'Admitted' '${AZ_DIR}/proofs/coq/' 2>/dev/null"
check "No unsafe in Rust src" "! grep -rq 'unsafe' '${AZ_DIR}/src/brainfuck/src/' '${AZ_DIR}/src/whitespace/src/' 2>/dev/null"
check "No unwrap in main src" "[ $(grep -rc '\.unwrap()' '${AZ_DIR}/src/main.rs' 2>/dev/null || echo 0) -eq 0 ]"
check "No eval in shell scripts" "! grep -rq '^[^#]*eval ' '${AZ_DIR}/verify-proofs.sh' '${AZ_DIR}/run-local-verification.sh' 2>/dev/null"

# --- Documentation ---
echo ""
echo "--- Documentation Completeness ---"
check "README.adoc exists" "[ -f '${AZ_DIR}/README.adoc' ]"
check "CONTRIBUTING exists" "[ -f '${AZ_DIR}/CONTRIBUTING.adoc' ] || [ -f '${AZ_DIR}/CONTRIBUTING.md' ]"
check "SECURITY.md exists" "[ -f '${AZ_DIR}/SECURITY.md' ]"
check "LICENSE exists" "[ -f '${AZ_DIR}/LICENSE' ] || [ -f '${AZ_DIR}/license/PMPL-1.0.txt' ]"
check "PROOF-NEEDS.md exists" "[ -f '${AZ_DIR}/PROOF-NEEDS.md' ]"
check "TOPOLOGY.md exists" "[ -f '${AZ_DIR}/TOPOLOGY.md' ]"

# --- Proof Inventory ---
echo ""
echo "--- Proof Inventory ---"
coq_count=$(find "${AZ_DIR}/proofs/coq" -name '*.v' 2>/dev/null | wc -l)
lean_count=$(find "${AZ_DIR}/proofs/lean4" -name '*.lean' 2>/dev/null | wc -l)
check "Coq proofs exist (${coq_count} files)" "[ '${coq_count}' -ge 5 ]"
check "Lean proofs exist (${lean_count} files)" "[ '${lean_count}' -ge 5 ]"
check "Z3 verification exists" "[ -f '${AZ_DIR}/proofs/z3/verify.sh' ]"
check "Agda proof exists" "[ -f '${AZ_DIR}/proofs/agda/CNO.agda' ]"
check "Isabelle proof exists" "[ -f '${AZ_DIR}/proofs/isabelle/CNO.thy' ]"

# --- Build Files ---
echo ""
echo "--- Build Infrastructure ---"
check "Cargo.toml exists" "[ -f '${AZ_DIR}/Cargo.toml' ]"
check "Justfile exists" "[ -f '${AZ_DIR}/Justfile' ]"
check "Containerfile exists" "[ -f '${AZ_DIR}/Containerfile' ]"
check "Benchmarks exist" "[ -f '${AZ_DIR}/benches/cno_benchmarks.rs' ]"
check "flake.nix exists" "[ -f '${AZ_DIR}/flake.nix' ]"

# --- CI/CD ---
echo ""
echo "--- CI/CD Workflows ---"
wf_count=$(find "${AZ_DIR}/.github/workflows" -name '*.yml' 2>/dev/null | wc -l)
check "CI workflows present (${wf_count})" "[ '${wf_count}' -ge 10 ]"
check "hypatia-scan.yml exists" "[ -f '${AZ_DIR}/.github/workflows/hypatia-scan.yml' ]"
check "codeql.yml exists" "[ -f '${AZ_DIR}/.github/workflows/codeql.yml' ]"
check "quality.yml exists" "[ -f '${AZ_DIR}/.github/workflows/quality.yml' ]"

echo ""
echo "==============================="
echo "  PASS: ${PASS}"
echo "  FAIL: ${FAIL}"
echo "==============================="

[ "${FAIL}" -eq 0 ]
