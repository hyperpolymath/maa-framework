<!-- SPDX-License-Identifier: MPL-2.0 -->
# Glossary

| Term | Definition |
|------|------------|
| **CNO** | Certified Null Operation. A program that compiles + runs and is formally proven to have zero net computational effect — no state change, no I/O, no entropy. |
| **Identity morphism** | In category theory, the morphism `id_X : X → X` that leaves `X` unchanged. CNOs are identity morphisms in the category of computational states (ADR-001). |
| **Observational reversibility (`=st=`)** | The weakened reversibility property used after ADR-008. State equality modulo program-counter bookkeeping; PC is not observable, so two states with different PC but same observable content are `=st=`. |
| **Landauer bound** | Lower thermodynamic cost of erasing one bit (`kT ln 2`). CNOs erase nothing, so any logically-reversible CNO can in principle be thermodynamically reversible too. Formalised in `StatMech.v` (axiom) + derived in `LandauerDerivation.v` (ADR-002). |
| **Postulate / Axiom** | Asserted, unproven proposition. In Coq: `Axiom`. In Lean: `axiom`. In Idris2 0.8.0: `name = believe_me ()` (the `postulate` keyword was removed). Every estate-wide axiom is tracked in [`AUDIT.adoc`](../../AUDIT.adoc) or [`META.scm`](../../.machine_readable/META.scm). |
| **HasAlignment t n** | Idris2 type asserting that type `t` has byte alignment `n`. The constructor `AlignProof` is information-free — the obligation sits on the producer to construct it only for the genuinely-correct `n`. |
| **alignedSize** | `(size, align) ↦` smallest multiple of `align` that is ≥ `size`. Core ABI primitive; correctness proved (via `believe_me`, pending discharge) in `Proofs.DivMod`. |
| **BoJ** | Brain of Jonathan — the estate's cross-repo automation cartridge layer. ECHIDNA invocations route through `echidna-llm-mcp` BoJ. |
| **ECHIDNA** | Estate-wide neurosymbolic prover gateway. 105 prover backends, 66,674-proof corpus, ML tactic suggestion. Lives in a separate repo; called via BoJ. |
| **PMPL-1.0** | Palimpsest-MPL 1.0. The project's licence. MPL-2.0 is the OSI-fallback. |
| **RSR** | Rhodium Standard Repository — the estate-wide taxonomy convention defined by [`hyperpolymath/rsr-template-repo`](https://github.com/hyperpolymath/rsr-template-repo). Conformance tracked in [`RSR_COMPLIANCE.adoc`](../../RSR_COMPLIANCE.adoc). |
| **ADR** | Architectural Decision Record. Forward-looking design choice. Recorded in [`META.scm`](../../.machine_readable/META.scm). |
| **AUDIT-ID** | Backward-looking trust event (discharged axiom, deleted unsound code, license correction). Format `AUDIT-YYYY-MM-DD-<seq-or-PR>`. Recorded in [`AUDIT.adoc`](../../AUDIT.adoc). |
| **6a2** | The `.a2ml` machine-readable format under `.machine_readable/6a2/`. Successor to the older `.scm` (Guile Scheme) format which is kept side-by-side during the migration window. |
| **Trust escape** | An axiom or `believe_me` use that lets you bypass the type system. Hunted by the estate-wide cross-trust-escape sweep. |
