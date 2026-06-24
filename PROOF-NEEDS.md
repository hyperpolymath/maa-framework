<!--
SPDX-License-Identifier: CC-BY-SA-4.0
Copyright (c) Jonathan D.A. Jewell <j.d.a.jewell@open.ac.uk>
-->
# PROOF-NEEDS.md — maa-framework

## Current State

- **src/abi/*.idr**: NO
- **Dangerous patterns**: 1 `Admitted` in `proofs/coq/lambda/LambdaCNO.v` (y_not_cno theorem)
- **LOC**: ~22,700 (Rust + Coq)
- **ABI layer**: Missing
- **Existing proofs**: Coq proofs for lambda CNO (3/4 proven), quantum mechanics exactness

## What Needs Proving

| Component | What | Why |
|-----------|------|-----|
| Y combinator non-CNO (y_not_cno) | Close the remaining Admitted proof | 1 of 4 lambda CNO theorems is incomplete |
| Absolute-zero brainfuck interpreter | Interpreter preserves CNO properties | Claims of computational zero need formal backing |
| Absolute-zero whitespace interpreter | Same as brainfuck — CNO preservation | Both esoteric interpreters need same guarantee |
| Aletheia verification checks | Verification pipeline produces sound results | Rhodibot extraction depends on correct checks |
| Quantum mechanics proofs | Extend QuantumMechanicsExact.v coverage | Existing proofs are partial |

## Recommended Prover

**Coq** — Existing proof infrastructure is in Coq. The `y_not_cno` Admitted needs to be closed in Coq. Aletheia (Rust) verification would benefit from an **Idris2** ABI layer.

## Priority

**MEDIUM** — Has existing proof infrastructure with one known gap. The Admitted y_not_cno is a concrete, closable proof obligation. Aletheia verification correctness is more important long-term.

## Template ABI Cleanup (2026-03-29)

Template ABI removed -- was creating false impression of formal verification.
The removed files (Types.idr, Layout.idr, Foreign.idr) contained only RSR template
scaffolding with unresolved {{PROJECT}}/{{AUTHOR}} placeholders and no domain-specific proofs.
