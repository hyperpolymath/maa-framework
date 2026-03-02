#!/bin/bash
# Absolute Zero - Setup and Verification Script
# This script sets up your environment and verifies the proofs

set -e  # Exit on error

echo "=== Absolute Zero Setup and Verification ==="
echo ""

# Fix git boundary issue for toolbox
export GIT_DISCOVERY_ACROSS_FILESYSTEM=1

# Navigate to repo root
cd "$(dirname "$0")"
REPO_ROOT=$(pwd)

echo "Repository root: $REPO_ROOT"
echo ""

# ============================================================================
# STEP 1: Git Configuration
# ============================================================================

echo "=== Step 1: Git Configuration ==="

# Check if we're in a git repo
if [ ! -d ".git" ]; then
    echo "ERROR: Not a git repository!"
    echo "You need to clone the repository first:"
    echo "  git clone https://github.com/Hyperpolymath/absolute-zero.git"
    exit 1
fi

# Show current remote
echo "Current git remote:"
git remote -v

# Ask about SSH
echo ""
read -p "Do you want to switch to SSH? (y/n) " -n 1 -r
echo ""
if [[ $REPLY =~ ^[Yy]$ ]]; then
    echo "Switching to SSH..."
    git remote set-url origin git@github.com:Hyperpolymath/absolute-zero.git
    echo "✓ Remote updated to SSH"
    git remote -v
fi

echo ""

# ============================================================================
# STEP 2: Check Current Status
# ============================================================================

echo "=== Step 2: Current Status ==="
echo ""

git status

echo ""
echo "Current branch:"
git branch --show-current

echo ""
echo "Recent commits:"
git log --oneline -5

echo ""

# ============================================================================
# STEP 3: Tool Availability Check
# ============================================================================

echo "=== Step 3: Tool Availability ==="
echo ""

check_tool() {
    if command -v $1 &> /dev/null; then
        echo "✓ $1: $(command -v $1)"
        return 0
    else
        echo "✗ $1: NOT FOUND"
        return 1
    fi
}

MISSING_TOOLS=()
OPTIONAL_TOOLS=()

# Core proof tools (required)
check_tool coqc || MISSING_TOOLS+=("coq")
check_tool z3 || MISSING_TOOLS+=("z3")
check_tool lake || MISSING_TOOLS+=("lean4/lake")
check_tool just || MISSING_TOOLS+=("just")

# Modern runtime (required - replaces npm)
check_tool deno || MISSING_TOOLS+=("deno")

# Configuration language (recommended)
check_tool nickel || OPTIONAL_TOOLS+=("nickel")

# Echidna testing stack (optional)
check_tool echidna || OPTIONAL_TOOLS+=("echidna")
check_tool slither || OPTIONAL_TOOLS+=("slither")

# Alternative SMT solvers (optional)
check_tool cvc5 || OPTIONAL_TOOLS+=("cvc5")
check_tool bitwuzla || OPTIONAL_TOOLS+=("bitwuzla")

# Elm for GUI (optional)
check_tool elm || OPTIONAL_TOOLS+=("elm")

# Deprecated (will be removed)
if check_tool npm; then
    echo "⚠ npm found (deprecated - migrate to Deno)"
fi

echo ""

if [ ${#MISSING_TOOLS[@]} -gt 0 ]; then
    echo "❌ Missing REQUIRED tools: ${MISSING_TOOLS[*]}"
    echo ""
    echo "Install with:"
    echo "  Fedora: sudo dnf install coq z3 && cargo install just"
    echo "  Ubuntu: sudo apt install coq z3 && cargo install just"
# WARNING: Pipe-to-shell is unsafe — download and verify first
# WARNING: Pipe-to-shell is unsafe — download and verify first
    echo "  Deno: curl -fsSL https://deno.land/install.sh | sh"
# WARNING: Pipe-to-shell is unsafe — download and verify first
# WARNING: Pipe-to-shell is unsafe — download and verify first
    echo "  Lean 4: curl https://raw.githubusercontent.com/leanprover/elan/master/elan-init.sh -sSf | sh"
    echo ""
fi

if [ ${#OPTIONAL_TOOLS[@]} -gt 0 ]; then
    echo "ℹ️  Missing OPTIONAL tools: ${OPTIONAL_TOOLS[*]}"
    echo ""
    echo "Install with:"
    echo "  Nickel: cargo install nickel-lang-cli"
    echo "  Echidna: wget https://github.com/crytic/echidna/releases/latest/download/echidna-*-Linux.tar.gz"
    echo "  Slither: pip3 install --user slither-analyzer"
    echo "  CVC5: wget https://github.com/cvc5/cvc5/releases/latest/download/cvc5-Linux"
    echo "  Bitwuzla: See https://bitwuzla.github.io/"
    echo "  Elm: npm install -g elm (or use Deno: deno install --allow-all npm:elm)"
    echo ""
    echo "See ECHIDNA_INTEGRATION.adoc for detailed setup"
    echo ""
fi

# ============================================================================
# STEP 4: Proof Status
# ============================================================================

echo "=== Step 4: Proof Completion Status ==="
echo ""

# Count Coq theorems and admitted
echo "Coq proofs:"
COQC_TOTAL=$(find proofs/coq -name "*.v" -exec grep -h "^Theorem\|^Lemma\|^Corollary" {} \; 2>/dev/null | wc -l)
COQC_ADMITTED=$(grep -r "Admitted\." proofs/coq/ 2>/dev/null | wc -l)
COQC_PROVEN=$((COQC_TOTAL - COQC_ADMITTED))
if [ $COQC_TOTAL -gt 0 ]; then
    COQC_PERCENT=$((COQC_PROVEN * 100 / COQC_TOTAL))
else
    COQC_PERCENT=0
fi

echo "  Total theorems: $COQC_TOTAL"
echo "  Proven: $COQC_PROVEN"
echo "  Admitted: $COQC_ADMITTED"
echo "  Completion: $COQC_PERCENT%"
echo ""

# Count Lean theorems and sorry
echo "Lean 4 proofs:"
LEAN_TOTAL=$(find proofs/lean4 -name "*.lean" -exec grep -h "^theorem\|^lemma" {} \; 2>/dev/null | wc -l)
LEAN_SORRY=$(grep -r "sorry" proofs/lean4/ 2>/dev/null | wc -l)
LEAN_PROVEN=$((LEAN_TOTAL - LEAN_SORRY))
if [ $LEAN_TOTAL -gt 0 ]; then
    LEAN_PERCENT=$((LEAN_PROVEN * 100 / LEAN_TOTAL))
else
    LEAN_PERCENT=0
fi

echo "  Total theorems: $LEAN_TOTAL"
echo "  Proven: $LEAN_PROVEN"
echo "  Sorry: $LEAN_SORRY"
echo "  Completion: $LEAN_PERCENT%"
echo ""

# List files with Admitted/sorry
if [ $COQC_ADMITTED -gt 0 ]; then
    echo "Files with Admitted:"
    grep -r "Admitted\." proofs/coq/ 2>/dev/null | cut -d: -f1 | sort -u | sed 's/^/  - /'
    echo ""
fi

if [ $LEAN_SORRY -gt 0 ]; then
    echo "Files with sorry:"
    grep -r "sorry" proofs/lean4/ 2>/dev/null | cut -d: -f1 | sort -u | sed 's/^/  - /'
    echo ""
fi

# ============================================================================
# STEP 5: Build and Verify (if tools available)
# ============================================================================

echo "=== Step 5: Build and Verify ==="
echo ""

read -p "Run verification now? (y/n) " -n 1 -r
echo ""
if [[ $REPLY =~ ^[Yy]$ ]]; then
    if command -v just &> /dev/null; then
        echo "Using justfile..."
        just verify-all
    else
        echo "Running manual verification..."

        # Try Coq
        if command -v coqc &> /dev/null; then
            echo "Building Coq proofs..."
            cd proofs/coq/common && coqc CNO.v || echo "⚠ CNO.v failed"
            cd "$REPO_ROOT"
        fi

        # Try Z3
        if command -v z3 &> /dev/null; then
            echo "Running Z3..."
            z3 proofs/z3/cno_properties.smt2 || echo "⚠ Z3 verification had issues"
        fi

        # Try Lean
        if command -v lake &> /dev/null; then
            echo "Building Lean 4..."
            cd proofs/lean4 && lake build || echo "⚠ Lean build failed"
            cd "$REPO_ROOT"
        fi
    fi
fi

echo ""

# ============================================================================
# STEP 6: Next Steps
# ============================================================================

echo "=== Next Steps ==="
echo ""
echo "Choose what you want to do:"
echo ""
echo "1. FIX ADMITTED PROOFS:"
echo "   - $COQC_ADMITTED Coq proofs need completion"
echo "   - $LEAN_SORRY Lean proofs need completion"
echo "   See files listed above"
echo ""
echo "2. VERIFY EXISTING PROOFS:"
echo "   just verify-all              # Verify all systems"
echo "   just verify-coq              # Just Coq"
echo "   just verify-lean             # Just Lean"
echo "   just verify-z3               # Just Z3"
echo ""
echo "3. BUILD EVERYTHING:"
echo "   just build-all               # Build all proof systems"
echo ""
echo "4. CHECK DETAILED STATUS:"
echo "   just proof-status            # Detailed completion stats"
echo "   just stats                   # Lines of code stats"
echo ""
echo "5. WORK ON SPECIFIC AREAS:"
echo "   - Physics proofs: proofs/coq/physics/LandauerDerivation.v (3 Admitted)"
echo "   - Quantum proofs: proofs/coq/quantum/QuantumMechanicsExact.v (3 Admitted)"
echo "   - Category theory: proofs/coq/category/CNOCategory.v (3 Admitted)"
echo ""
echo "6. COMMIT AND PUSH:"
echo "   git add ."
echo "   git commit -m 'Your message'"
echo "   git push -u origin \$(git branch --show-current)"
echo ""

# Create a quick reference file
cat > QUICKSTART.txt << 'EOF'
ABSOLUTE ZERO - QUICK REFERENCE

Current Status:
- Coq: {COQC_PROVEN}/{COQC_TOTAL} proven ({COQC_PERCENT}%)
- Lean: {LEAN_PROVEN}/{LEAN_TOTAL} proven ({LEAN_PERCENT}%)

Common Commands:
  just verify-all       # Verify all proofs
  just build-all        # Build everything
  just proof-status     # Check completion
  just clean            # Clean artifacts

Files Needing Work:
  - See output above for Admitted/sorry locations

Documentation:
  - COOKBOOK.adoc       # Getting started guide
  - FORMAL_ANALYSIS_PHASE1.md  # Phase 1 analysis
  - FORMAL_ANALYSIS_PHASE2.md  # Phase 2 analysis

Next Priority:
  Complete the {COQC_ADMITTED} Admitted proofs in Coq
EOF

# Substitute actual values
sed -i "s/{COQC_PROVEN}/$COQC_PROVEN/g" QUICKSTART.txt
sed -i "s/{COQC_TOTAL}/$COQC_TOTAL/g" QUICKSTART.txt
sed -i "s/{COQC_PERCENT}/$COQC_PERCENT/g" QUICKSTART.txt
sed -i "s/{LEAN_PROVEN}/$LEAN_PROVEN/g" QUICKSTART.txt
sed -i "s/{LEAN_TOTAL}/$LEAN_TOTAL/g" QUICKSTART.txt
sed -i "s/{LEAN_PERCENT}/$LEAN_PERCENT/g" QUICKSTART.txt
sed -i "s/{COQC_ADMITTED}/$COQC_ADMITTED/g" QUICKSTART.txt

echo "✓ Created QUICKSTART.txt with current status"
echo ""
echo "=== Setup Complete ==="
