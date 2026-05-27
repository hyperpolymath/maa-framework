<!-- SPDX-License-Identifier: MPL-2.0 -->
# Absolute Zero — Wiki

> **Formal Verification of Certified Null Operations: When Doing Nothing Is Everything.**

Welcome to the **absolute-zero** project wiki. This wiki is the human-facing entry
point; the machine-facing entry point is [`0-AI-MANIFEST.a2ml`](../../0-AI-MANIFEST.a2ml).

## What is a CNO?

A **Certified Null Operation** is a program that:
* compiles + executes without error,
* and is *formally proven* to have zero net computational effect — no
  observable side effect on state, I/O, or thermodynamic entropy.

The interesting question is not "can we write a no-op?" (trivially yes), but
"can we prove that an arbitrary candidate program is a no-op, in a
machine-checkable way, across multiple semantic models?". Absolute Zero
formalises this across six proof systems and verifies it for interpreters
of 23+ languages.

## Navigate

| Page | Purpose |
|------|---------|
| [Architecture](Architecture.md) | How the pieces fit together: proofs, interpreters, ABI, ECHIDNA |
| [Proof Systems](Proof-Systems.md) | Coq / Lean 4 / Z3 / Agda / Isabelle / Mizar — what each proves and why six |
| [Verification](Verification.md) | How to build and verify locally; CI matrix |
| [ABI](ABI.md) | Idris2 FFI surface; alignment + size invariants |
| [Roadmap](Roadmap.md) | v1.0 → v12.0 trajectory |
| [Contributing](Contributing.md) | How to send a PR, conventions, sign-off |
| [Glossary](Glossary.md) | CNO, observational reversibility, =st=, Landauer bound, etc. |
| [FAQ](FAQ.md) | Common questions |
| [Audit Trail](Audit-Trail.md) | Discharged axioms, deleted unsound code, open findings |

## Status (live)

* **Phase**: proof-completion (~65%)
* **Coq**: 11/11 files compile, 0 Admitted, 73 Axioms + 42 Parameters (model-layer)
* **Lean 4**: `lake build` 1631/1632 green
* **Idris2 ABI**: typechecks under 0.8.0; `Proofs/DivMod.idr` consolidates the trusted div/mod base
* For the authoritative live state, read [`.machine_readable/6a2/STATE.a2ml`](../../.machine_readable/6a2/STATE.a2ml)

## Project layout (RSR-aligned)

```
absolute-zero/
├── 0-AI-MANIFEST.a2ml      ← machine entry point
├── README.adoc             ← short pitch
├── ROADMAP.adoc            ← v1.0 → v12.0
├── AUDIT.adoc              ← discharged axioms + open findings
├── RSR_COMPLIANCE.adoc     ← Rhodium Standard conformance
├── ECHIDNA.adoc            ← integration with the prover gateway
├── docs/                   ← evergreen topical docs; wiki/ subdir
├── proofs/                 ← coq, lean4, z3, agda, isabelle, mizar
├── src/                    ← Idris2 ABI + Rust core
├── interpreters/           ← language interpreters (ReScript)
├── examples/               ← CNOs in 23+ languages
├── verification/           ← top-level verify scripts
├── tests/, tools/          ← RSR-conventional placeholders
├── .well-known/            ← security.txt, ai.txt, humans.txt
└── .machine_readable/      ← STATE / META / ECOSYSTEM / AGENTIC / NEUROSYM / PLAYBOOK
```

## License

PMPL-1.0-or-later (Palimpsest-MPL 1.0). MPL-2.0 fallback where platform requires OSI-approved licence.
