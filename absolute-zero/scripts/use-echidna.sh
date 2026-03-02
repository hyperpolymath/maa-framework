#!/bin/bash
# SPDX-License-Identifier: PMPL-1.0-or-later
# Use ECHIDNA to assist with CNO proof completion

set -e

ECHIDNA_BIN="$HOME/Documents/hyperpolymath-repos/echidna/target/release/echidna"
PROOF_DIR="$HOME/Documents/hyperpolymath-repos/absolute-zero/proofs"

if [ ! -x "$ECHIDNA_BIN" ]; then
    echo "❌ ECHIDNA not built. Run:"
    echo "   cd ~/Documents/hyperpolymath-repos/echidna && cargo build --release"
    exit 1
fi

echo "🧠 ECHIDNA Neurosymbolic Proof Assistant"
echo "========================================"
echo ""
echo "Available commands:"
echo ""
echo "  list-admitted     - Show all Admitted proofs in Coq"
echo "  suggest <file>    - Get tactic suggestions for a proof file"
echo "  verify-all        - Verify all proofs with multi-prover consensus"
echo "  complete <file>   - Attempt to auto-complete proofs"
echo "  repl              - Start interactive REPL"
echo ""

case "$1" in
    list-admitted)
        echo "📋 Finding Admitted proofs..."
        grep -rn "Admitted" "$PROOF_DIR/coq" | while read line; do
            file=$(echo "$line" | cut -d: -f1)
            linenum=$(echo "$line" | cut -d: -f2)
            # Get theorem name (previous line usually has "Theorem" or "Lemma")
            theorem=$(sed -n "$((linenum-1))p" "$file" | grep -o -E "(Theorem|Lemma) \w+" | cut -d' ' -f2)
            echo "  - $theorem ($(basename $file):$linenum)"
        done
        ;;

    suggest)
        if [ -z "$2" ]; then
            echo "Usage: $0 suggest <proof-file.v>"
            exit 1
        fi
        echo "💡 Getting tactic suggestions for $2..."
        "$ECHIDNA_BIN" suggest --proof "$2" --prover Coq
        ;;

    verify-all)
        echo "✓ Verifying all proofs with multi-prover consensus..."
        "$ECHIDNA_BIN" verify-all \
            --proof-dir "$PROOF_DIR/coq" \
            --provers "Coq,Lean4,Z3" \
            --consensus 2
        ;;

    complete)
        if [ -z "$2" ]; then
            echo "Usage: $0 complete <proof-file.v>"
            exit 1
        fi
        echo "🔧 Attempting to complete proofs in $2..."
        "$ECHIDNA_BIN" complete \
            --proof "$2" \
            --prover Coq \
            --output "${2%.v}_completed.v" \
            --max-tactics 50
        echo ""
        echo "✓ Suggestions written to: ${2%.v}_completed.v"
        echo "  Review carefully before integrating!"
        ;;

    repl)
        echo "🎮 Starting ECHIDNA REPL..."
        "$ECHIDNA_BIN" repl --prover Coq --model cno
        ;;

    *)
        echo "Unknown command: $1"
        exit 1
        ;;
esac
