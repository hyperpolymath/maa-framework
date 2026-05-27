#!/usr/bin/env bash
# Local Verification Runner (runs without container if tools are installed)
#
# This script attempts to run proof verification locally.
# If tools are not installed, it will report which ones are missing.

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"

echo "========================================"
echo "Absolute Zero: Local Verification"
echo "========================================"
echo ""

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Track results
TOTAL_CHECKS=0
PASSED_CHECKS=0
FAILED_CHECKS=0
SKIPPED_CHECKS=0

# Function to check if command exists
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# Function to run a check
run_check() {
    local name="$1"
    local command="$2"
    TOTAL_CHECKS=$((TOTAL_CHECKS + 1))

    echo -n "[$TOTAL_CHECKS] $name... "
    if eval "$command" > /tmp/absolute-zero-check-$TOTAL_CHECKS.log 2>&1; then
        echo -e "${GREEN}PASS${NC}"
        PASSED_CHECKS=$((PASSED_CHECKS + 1))
        return 0
    else
        echo -e "${RED}FAIL${NC}"
        FAILED_CHECKS=$((FAILED_CHECKS + 1))
        echo "    Error log: /tmp/absolute-zero-check-$TOTAL_CHECKS.log"
        return 1
    fi
}

# Function to skip a check
skip_check() {
    local name="$1"
    local reason="$2"
    TOTAL_CHECKS=$((TOTAL_CHECKS + 1))
    SKIPPED_CHECKS=$((SKIPPED_CHECKS + 1))
    echo -e "[$TOTAL_CHECKS] $name... ${YELLOW}SKIP${NC} ($reason)"
}

echo "==== Tool Availability Check ===="
echo ""

# Check for Coq
if command_exists coqc; then
    echo -e "${GREEN}✓${NC} coqc found: $(coqc --version | head -1)"
    COQ_AVAILABLE=1
else
    echo -e "${RED}✗${NC} coqc not found"
    COQ_AVAILABLE=0
fi

# Check for Z3
if command_exists z3; then
    echo -e "${GREEN}✓${NC} z3 found: $(z3 --version)"
    Z3_AVAILABLE=1
else
    echo -e "${RED}✗${NC} z3 not found"
    Z3_AVAILABLE=0
fi

# Check for Lean 4
if command_exists lean; then
    echo -e "${GREEN}✓${NC} lean found: $(lean --version | head -1)"
    LEAN_AVAILABLE=1
else
    echo -e "${RED}✗${NC} lean not found"
    LEAN_AVAILABLE=0
fi

# Check for Agda
if command_exists agda; then
    echo -e "${GREEN}✓${NC} agda found: $(agda --version | head -1)"
    AGDA_AVAILABLE=1
else
    echo -e "${RED}✗${NC} agda not found"
    AGDA_AVAILABLE=0
fi

# Check for Isabelle
if command_exists isabelle; then
    echo -e "${GREEN}✓${NC} isabelle found"
    ISABELLE_AVAILABLE=1
else
    echo -e "${RED}✗${NC} isabelle not found"
    ISABELLE_AVAILABLE=0
fi

echo ""
echo "==== Running Verification ===="
echo ""

# Z3 SMT Verification
if [ $Z3_AVAILABLE -eq 1 ]; then
    echo "---- Z3 SMT Solver ----"
    run_check "Z3: CNO Properties" "z3 proofs/z3/cno_properties.smt2"
    echo ""
else
    skip_check "Z3: CNO Properties" "z3 not installed"
    echo ""
fi

# Coq Verification
if [ $COQ_AVAILABLE -eq 1 ]; then
    echo "---- Coq Proof Assistant ----"
    run_check "Coq: Phase 1 Core (CNO.v)" "coqc -Q proofs/coq/common CNO proofs/coq/common/CNO.v"
    run_check "Coq: Statistical Mechanics" "coqc -Q proofs/coq/common CNO -Q proofs/coq/physics Physics proofs/coq/physics/StatMech.v"
    run_check "Coq: Category Theory" "coqc -Q proofs/coq/common CNO -Q proofs/coq/category Category proofs/coq/category/CNOCategory.v"
    run_check "Coq: Lambda Calculus" "coqc -Q proofs/coq/common CNO -Q proofs/coq/lambda Lambda proofs/coq/lambda/LambdaCNO.v"
    run_check "Coq: Quantum CNO" "coqc -Q proofs/coq/common CNO -Q proofs/coq/quantum Quantum proofs/coq/quantum/QuantumCNO.v"
    run_check "Coq: Filesystem CNO" "coqc -Q proofs/coq/common CNO -Q proofs/coq/filesystem Filesystem proofs/coq/filesystem/FilesystemCNO.v"
    echo ""
else
    skip_check "Coq: All proofs" "coqc not installed"
    echo ""
fi

# Lean 4 Verification
if [ $LEAN_AVAILABLE -eq 1 ]; then
    echo "---- Lean 4 Proof Assistant ----"
    if [ -f "proofs/lean4/lakefile.lean" ]; then
        run_check "Lean 4: Build all proofs" "cd proofs/lean4 && lake build"
    else
        skip_check "Lean 4: Build all proofs" "lakefile.lean not found"
    fi
    echo ""
else
    skip_check "Lean 4: All proofs" "lean not installed"
    echo ""
fi

# Agda Verification
if [ $AGDA_AVAILABLE -eq 1 ]; then
    echo "---- Agda Proof Assistant ----"
    run_check "Agda: CNO Core" "agda proofs/agda/CNO.agda"
    echo ""
else
    skip_check "Agda: CNO Core" "agda not installed"
    echo ""
fi

# Isabelle Verification
if [ $ISABELLE_AVAILABLE -eq 1 ]; then
    echo "---- Isabelle/HOL ----"
    run_check "Isabelle: CNO Theory" "isabelle build -d proofs/isabelle -b CNO"
    echo ""
else
    skip_check "Isabelle: CNO Theory" "isabelle not installed"
    echo ""
fi

# Summary
echo "========================================"
echo "Verification Summary"
echo "========================================"
echo ""
echo "Total checks: $TOTAL_CHECKS"
echo -e "${GREEN}Passed: $PASSED_CHECKS${NC}"
echo -e "${RED}Failed: $FAILED_CHECKS${NC}"
echo -e "${YELLOW}Skipped: $SKIPPED_CHECKS${NC}"
echo ""

if [ $FAILED_CHECKS -eq 0 ] && [ $PASSED_CHECKS -gt 0 ]; then
    echo -e "${GREEN}✓ All available verifications passed!${NC}"
    exit 0
elif [ $PASSED_CHECKS -eq 0 ]; then
    echo -e "${YELLOW}⚠ No verification tools available locally${NC}"
    echo "Consider installing: coqc, z3, lean, agda, isabelle"
    echo "Or run: podman build -t absolute-zero . && podman run --rm absolute-zero ./verify-proofs.sh"
    exit 2
else
    echo -e "${RED}✗ Some verifications failed${NC}"
    echo "Check error logs in /tmp/absolute-zero-check-*.log"
    exit 1
fi
