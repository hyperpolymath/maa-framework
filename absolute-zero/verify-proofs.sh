#!/usr/bin/env bash
# Proof Verification Script for Absolute Zero
#
# This script runs all proof checkers and reports verification status.
# Should be run inside the container or with all proof tools installed.
#
# Usage: ./verify-proofs.sh [--verbose]
#
# Author: Jonathan D. A. Jewell
# Project: Absolute Zero

set -e  # Exit on error

VERBOSE=false
if [[ "$1" == "--verbose" ]]; then
    VERBOSE=true
fi

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Counters
TOTAL_TESTS=0
PASSED_TESTS=0
FAILED_TESTS=0
SKIPPED_TESTS=0

# Helper functions
log_info() {
    echo -e "${BLUE}[INFO]${NC} ${1}"
}

log_success() {
    echo -e "${GREEN}[✓]${NC} ${1}"
    ((PASSED_TESTS++))
}

log_failure() {
    echo -e "${RED}[✗]${NC} ${1}"
    ((FAILED_TESTS++))
}

log_skip() {
    echo -e "${YELLOW}[SKIP]${NC} ${1}"
    ((SKIPPED_TESTS++))
}

check_command() {
    if command -v "$1" &> /dev/null; then
        return 0
    else
        return 1
    fi
}

# Create a secure temporary directory for test output
TMPDIR_VERIFY="$(mktemp -d)"
trap 'rm -rf "${TMPDIR_VERIFY}"' EXIT

run_test() {
    local test_name="$1"
    shift
    local test_cmd=("$@")
    ((TOTAL_TESTS++))

    local log_file="${TMPDIR_VERIFY}/test_output_${TOTAL_TESTS}.log"

    if $VERBOSE; then
        echo ""
        log_info "Running: ${test_name}"
        log_info "Command: ${test_cmd[*]}"
    fi

    if "${test_cmd[@]}" &> "${log_file}"; then
        log_success "${test_name}"
        if $VERBOSE; then
            cat "${log_file}"
        fi
        return 0
    else
        log_failure "${test_name}"
        echo "    Error output:"
        sed 's/^/    /' "${log_file}" | head -20
        return 1
    fi
}

echo "========================================"
echo "  Absolute Zero: Proof Verification"
echo "========================================"
echo ""

# Check working directory
if [[ ! -f "justfile" ]] || [[ ! -d "proofs" ]]; then
    log_failure "Must be run from project root directory"
    exit 1
fi

log_info "Checking installed proof tools..."
echo ""

# =============================================================================
# Coq Verification
# =============================================================================

echo "--- Coq Proofs ---"
if check_command coqc; then
    log_info "Coq found: $(coqc --version | head -1)"

    run_test "Coq: CNO.v (common)" \
        bash -c "cd proofs/coq/common && coqc CNO.v"

    run_test "Coq: MalbolgeCore.v" \
        bash -c "cd proofs/coq/malbolge && coqc -R ../common CNO MalbolgeCore.v"
else
    log_skip "Coq not installed (coqc not found)"
    ((TOTAL_TESTS+=2))
    ((SKIPPED_TESTS+=2))
fi
echo ""

# =============================================================================
# Z3 Verification
# =============================================================================

echo "--- Z3 SMT Proofs ---"
if check_command z3; then
    log_info "Z3 found: $(z3 --version)"

    # Check if verify.sh exists and is executable
    if [[ -x "proofs/z3/verify.sh" ]]; then
        run_test "Z3: CNO properties" \
            bash -c "cd proofs/z3 && ./verify.sh"
    else
        # Run z3 directly
        run_test "Z3: CNO properties" \
            bash -c "cd proofs/z3 && z3 cno_properties.smt2 | grep -q 'sat'"
    fi
else
    log_skip "Z3 not installed (z3 not found)"
    ((TOTAL_TESTS+=1))
    ((SKIPPED_TESTS+=1))
fi
echo ""

# =============================================================================
# Lean 4 Verification
# =============================================================================

echo "--- Lean 4 Proofs ---"
if check_command lean; then
    log_info "Lean found: $(lean --version | head -1)"

    run_test "Lean 4: CNO.lean" \
        bash -c "cd proofs/lean4 && lake build"
else
    log_skip "Lean 4 not installed (lean not found)"
    ((TOTAL_TESTS+=1))
    ((SKIPPED_TESTS+=1))
fi
echo ""

# =============================================================================
# Agda Verification
# =============================================================================

echo "--- Agda Proofs ---"
if check_command agda; then
    log_info "Agda found: $(agda --version | head -1)"

    run_test "Agda: CNO.agda" \
        bash -c "cd proofs/agda && agda CNO.agda"
else
    log_skip "Agda not installed (agda not found)"
    ((TOTAL_TESTS+=1))
    ((SKIPPED_TESTS+=1))
fi
echo ""

# =============================================================================
# Isabelle/HOL Verification
# =============================================================================

echo "--- Isabelle/HOL Proofs ---"
if check_command isabelle; then
    log_info "Isabelle found: $(isabelle version)"

    run_test "Isabelle/HOL: CNO.thy" \
        isabelle build -D proofs/isabelle
else
    log_skip "Isabelle/HOL not installed (isabelle not found)"
    ((TOTAL_TESTS+=1))
    ((SKIPPED_TESTS+=1))
fi
echo ""

# =============================================================================
# Summary
# =============================================================================

echo "========================================"
echo "  Verification Summary"
echo "========================================"
echo ""
echo "Total tests:   ${TOTAL_TESTS}"
echo -e "Passed:        ${GREEN}${PASSED_TESTS}${NC}"
echo -e "Failed:        ${RED}${FAILED_TESTS}${NC}"
echo -e "Skipped:       ${YELLOW}${SKIPPED_TESTS}${NC}"
echo ""

if [[ "${FAILED_TESTS}" -eq 0 ]]; then
    if [[ "${SKIPPED_TESTS}" -eq 0 ]]; then
        echo -e "${GREEN}✓ All proofs verified successfully!${NC}"
        exit 0
    else
        echo -e "${YELLOW}⚠ Some proof tools not installed (see VERIFICATION.md)${NC}"
        echo "  Build the container for full verification:"
        echo "    podman build -t absolute-zero ."
        echo "    podman run --rm absolute-zero ./verify-proofs.sh"
        exit 0
    fi
else
    echo -e "${RED}✗ Some proofs failed verification${NC}"
    echo "  Run with --verbose for detailed error output:"
    echo "    ./verify-proofs.sh --verbose"
    exit 1
fi
