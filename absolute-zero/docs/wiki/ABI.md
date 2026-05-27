<!-- SPDX-License-Identifier: MPL-2.0 -->
# ABI

The Idris2 ABI surface at `src/abi/` declares the FFI boundary that the
Zig shim (`ffi/zig/`) implements.

## Modules

| Module | Purpose |
|--------|---------|
| `AbsoluteZero.ABI.Types` | Core types: `Platform`, `Result`, `Instruction`, `ProgramState`, opaque handles |
| `AbsoluteZero.ABI.Layout` | Memory layout proofs: field offsets, sizes, alignments per platform |
| `AbsoluteZero.ABI.Foreign` | `%foreign` declarations + safe wrappers |
| `AbsoluteZero.ABI.Proofs.DivMod` | Trusted div/mod lemma surface (estate-wide) |

## Building

```bash
idris2 --build absolute-zero-abi.ipkg
```

The package file at the repo root is `absolute-zero-abi.ipkg`.

## DivMod — the estate-wide div/mod lemma surface

`src/abi/Proofs/DivMod.idr` consolidates the trusted base of
number-theoretic lemmas used by ABI alignment proofs across the
hyperpolymath estate. Each axiom is individually named so discharge
can be incremental:

```idris
alignedSizeCorrect :
  (size : Nat) -> (align : Nat) ->
  {auto 0 nonZero : So (align /= 0)} ->
  So (alignedSize size align `mod` align == 0)

divModIdentity :
  (n : Nat) -> (d : Nat) ->
  {auto 0 nonZero : So (d /= 0)} ->
  n = (n `div` d) * d + (n `mod` d)

multModZero :
  (k : Nat) -> (d : Nat) ->
  {auto 0 nonZero : So (d /= 0)} ->
  So ((k * d) `mod` d == 0)

addModDistrib :
  (a : Nat) -> (b : Nat) -> (d : Nat) ->
  {auto 0 nonZero : So (d /= 0)} ->
  (a + b) `mod` d = ((a `mod` d) + (b `mod` d)) `mod` d
```

All four currently route through `believe_me ()` — the Idris2 0.8.0
canonical axiom idiom. Discharge path:
* `divModIdentity` is provable from `Data.Nat.Division.DivisionTheorem` in idris2-contrib
* `multModZero` follows by induction on `k`
* `addModDistrib` is in `Data.Nat.Equational` territory
* `alignedSizeCorrect` then chains them

Cross-estate: civic-connect's `src/Abi/Layout.idr` defers the same
family (`alignUpDivides`, `mkFieldsAligned`, `offsetInBoundsPrf`).
The intent is for those to migrate to import from
`AbsoluteZero.ABI.Proofs.DivMod` rather than re-postulate per repo.

## History: the unsound `alignmentMatchesPlatformWord`

A previous postulate, `alignmentMatchesPlatformWord : HasAlignment t n -> So (n `mod` word == 0)`,
was deleted as unsound on 2026-05-25 (ADR-009, issue #27). `HasAlignment t n`
has only an information-free constructor `AlignProof`, so the universal
claim could derive `So (1 mod 8 == 0)` ≡ `So False` from the file's own
`CNOResultLayout.alignment : HasAlignment CNOVerificationResult 1`. The
single consumer (`programStateAlignmentValid`) is now discharged
per-instance.

See [`AUDIT.adoc`](../../AUDIT.adoc) AUDIT-2026-05-20-#27 for the full
write-up.

## Open AUDIT items

* **AUDIT-2026-05-20-A** — `src/abi/Types.idr` has 5 pre-existing
  errors blocking standalone typecheck under Idris2 0.8.0 (missing
  `Decidable.Equality` import; `%runElab` without `ElabReflection`;
  `MkStateHandle ptr` not supplying `nonNull` proof; `No absurd`
  lacks `Uninhabited` instances). Independent of #27.
