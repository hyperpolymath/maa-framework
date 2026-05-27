<!-- SPDX-License-Identifier: MPL-2.0 -->
# Audit Trail

Short summary; the authoritative ledger is [`AUDIT.adoc`](../../AUDIT.adoc).

## Resolved

| ID | Date | What | Where |
|----|------|------|-------|
| AUDIT-2026-05-20-#27 | 2026-05-25 | Unsound `alignmentMatchesPlatformWord` Idris2 postulate deleted; `alignedSizeCorrect` isolated into shared `Proofs/DivMod.idr` | PR #41, ADR-009 |
| AUDIT-2026-05-20-#32 | 2026-05-20 | Deleted unsound `eval_respects_state_eq_{left,right}` axioms; observational reversibility (`=st=`) | PR #32, ADR-008 |
| AUDIT-2026-05-20-#24 | 2026-05-20 | `eval_deterministic`: Axiom → Theorem via `step_deterministic_strong` | PR #24, ADR-007 |
| AUDIT-2026-02-05 | 2026-02-05 | License canonicalisation to PMPL-1.0-or-later (79 files) | [archived](../archive/LICENSE-AUDIT-2026-02-05.adoc) |

## Open

| ID | Severity | What | Status |
|----|---------|------|--------|
| AUDIT-2026-05-20-A | Medium | `src/abi/Types.idr` has 5 pre-existing errors blocking Idris2 0.8.0 typecheck | Filed, needs separate PR |
| AUDIT-2026-05-20-B | Low | `cflite_pr.yml` missing `actions/checkout` before fuzzer | Filed, needs separate PR |

## Axiom classification

The 73 Coq Axioms + 42 Parameters in `proofs/coq/` are model-layer
assumptions:

* **Physics**: thermodynamic laws, Landauer bound, statistical mechanics
* **Quantum**: Hilbert-space axioms, unitarity
* **POSIX**: filesystem semantics
* **Computational complexity**: standard model assumptions

These are *intentional* axiomatisations of the external world, not
verification gaps for the CNO claim itself.

Tracked under [`hyperpolymath/standards`#133](https://github.com/hyperpolymath/standards/issues/133).

## Discharge mechanism

To turn an axiom into a theorem:

1. Identify the axiom in `proofs/<prover>/<file>.<ext>`
2. Find or write a helper that lets you derive it from less-trusted
   axioms (ideally Prelude-only)
3. Replace `Axiom name : ...` / `axiom name : ...` / `name = believe_me ()`
   with the proven theorem
4. Add an ADR-NNN entry to `META.scm`
5. Add an AUDIT-YYYY-MM-DD-#PR row to `AUDIT.adoc`

See ADR-007 (#24) for a worked example.

## Trust-escape inventory

Estate-wide cross-trust-escape sweeps are run periodically. Most recent:
2026-05-20 (the sweep that surfaced #27). All findings either:
* land in `AUDIT.adoc` if absolute-zero-local, or
* land in [`hyperpolymath/standards`#133](https://github.com/hyperpolymath/standards/issues/133) for model-layer.
