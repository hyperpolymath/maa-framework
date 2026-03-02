#!/bin/bash
# Z3 CNO Property Verification Script
#
# This script runs Z3 verification on CNO properties
# Author: Jonathan D. A. Jewell
# Project: Absolute Zero

set -e

echo "=========================================="
echo "Absolute Zero: Z3 CNO Verification"
echo "=========================================="
echo ""

# Check if Z3 is installed
if ! command -v z3 &> /dev/null; then
    echo "Error: Z3 is not installed"
    echo "Install with: sudo dnf install z3  (Fedora)"
    echo "           or sudo apt install z3  (Ubuntu)"
    exit 1
fi

echo "Z3 version:"
z3 --version
echo ""

# Run verification
echo "Running CNO property verification..."
echo ""

z3 cno_properties.smt2

echo ""
echo "=========================================="
echo "Verification complete!"
echo "=========================================="
echo ""
echo "All SAT results indicate theorems hold."
echo "UNSAT results indicate counterexamples were not found."
echo ""
