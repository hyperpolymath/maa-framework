<!--
SPDX-License-Identifier: MPL-2.0
SPDX-FileCopyrightText: 2026 Jonathan D.A. Jewell (hyperpolymath)
-->

# Proof Debt — maa-framework

**Schema**: `hyperpolymath/standards`
[`docs/TRUSTED-BASE-REDUCTION-POLICY.adoc`](https://github.com/hyperpolymath/standards/blob/main/docs/TRUSTED-BASE-REDUCTION-POLICY.adoc)
— the canonical policy, landed via standards#203.
**Enforcement**: standards#211 (`scripts/check-trusted-base.sh`).

This file is the schema-conformant per-repo proof-debt index for
`hyperpolymath/maa-framework`. The broader strategic proof goals across
the repo live in [`PROOF-NEEDS.md`](../PROOF-NEEDS.md); this file
enumerates the *specific* soundness-relevant escape hatches that the
[`check-trusted-base.sh`](https://github.com/hyperpolymath/standards/blob/main/scripts/check-trusted-base.sh)
CI gate detects in source.

## Schema extension for maa-framework: §(e) VENDORED

maa-framework carries an in-tree copy of the sibling estate repo
`hyperpolymath/absolute-zero` at `absolute-zero/`. The canonical home of
those proof files is the standalone `absolute-zero` repo, which has its
own `docs/proof-debt.md` (seeded under the same standards#203 chain).

To avoid double-counting (and double-discharging) the same markers, we
extend the schema with a fifth disposition:

| Disposition | Marker | Definition |
| --- | --- | --- |
| **(e) VENDORED — estate-sibling** | `// VENDORED: <canonical-repo>` | The marker lives in an in-tree copy of another estate repository. Disposition is **delegated** to the canonical repo's `docs/proof-debt.md`. Closing the marker upstream + re-vendoring is the close-out path. |

`(e)` is **out of scope** for maa-framework's own proof-debt close-out
budget. The maintainers' obligation is just to keep the vendored copy
fresh (re-sync periodically) and to flag if a marker disappears upstream
but persists here.

The `(e)` extension is documented inline here per the standards#203
policy's guidance: "extend the schema; document the extension at the
top of the file as a repo-specific note." If the pattern recurs across
other estate repos, it will be promoted into the central policy.

## Marker count (2026-05-27)

`scripts/check-trusted-base.sh`-style grep over `*.v / *.lean / *.agda /
*.idr / *.fst / *.dfy / *.rs / *.hs`:

- **150 syntactic matches across 13 files** (the higher number vs the
  audit's "134" reflects a more inclusive regex; the audit excluded
  some comment/import false positives).
- **All 13 files** live under `absolute-zero/proofs/{coq,lean4,agda}/`.
- **All 13 files** are vendored from `hyperpolymath/absolute-zero` (the
  sibling estate repo). File-header comparison + author/license headers
  ("Author: Jonathan D. A. Jewell; Project: Absolute Zero") confirm
  these are estate-owned but with canonical home in the sibling repo.
- The two trees have drifted (e.g.
  `proofs/coq/lambda/LambdaCNO.v` differs from the sibling-repo copy);
  re-syncing is a follow-up.

Disposition split:

| Disposition | Files | Marker count | Notes |
| --- | --- | --- | --- |
| (a) DISCHARGED | 0 | 0 | — |
| (b) BUDGETED | 0 | 0 | — |
| (c) NECESSARY AXIOM | 0 | 0 | (See §(e). The axioms in those files are mostly genuine §(c)-class — physical constants, no-cloning, function-extensionality — but the canonical home is the sibling repo.) |
| (d) DEBT | 0 | 0 | (PROOF-NEEDS.md flags `y_not_cno` `Admitted` as a known gap — that's also (e) since it lives in the vendored subtree.) |
| **(e) VENDORED — estate-sibling** | **13** | **150** | All under `absolute-zero/`; canonical home `hyperpolymath/absolute-zero`. |

## (a) DISCHARGED in this repo

*(None — maa-framework itself contains no proof-bearing source outside
the vendored `absolute-zero/` subtree.)*

## (b) BUDGETED — tested with a refutation budget

*(None — same reason as §(a).)*

## (c) NECESSARY AXIOM

*(None — same reason as §(a). Note: the axioms inside `absolute-zero/`
(`kB_positive`, `temperature_positive`, `no_cloning`, the unitary-gate
postulates, Shannon-entropy axioms, function-extensionality, etc.) are
genuine §(c)-class candidates, but their canonical disposition lives in
the sibling repo. See §(e).)*

## (d) DEBT — actively to be closed

*(None — same reason as §(a). `PROOF-NEEDS.md` notes a known `Admitted`
on `y_not_cno` in `proofs/coq/lambda/LambdaCNO.v`; that file is in the
vendored subtree (§(e)), so the discharge happens upstream in
`hyperpolymath/absolute-zero`.)*

## (e) VENDORED — estate-sibling

All entries below have the form: `<vendored path> — <marker count> —
<brief disposition rationale>`. The **canonical home** for every entry
is `hyperpolymath/absolute-zero`; this file does not duplicate that
repo's proof-debt classification.

### `absolute-zero/proofs/coq/` (7 files, 91 markers)

| File | Markers | Sample disposition (in sibling repo) |
| --- | ---: | --- |
| `absolute-zero/proofs/coq/quantum/QuantumCNO.v` | 35 | Mostly §(c): physical constants (`kB_positive`, `temperature_positive`), gate-unitarity (`X_gate_unitary`, `H_gate_unitary`, `CNOT_gate_unitary`), inner-product / `Cexp` algebra, `no_cloning`, `unitary_preserves_entropy`. |
| `absolute-zero/proofs/coq/physics/StatMech.v` | 17 | §(c): Shannon-entropy non-negativity / maximum / point-zero, `landauer_principle`, `reversible_zero_dissipation`, physical constants. |
| `absolute-zero/proofs/coq/physics/LandauerDerivation.v` | 14 | §(c): same family as `StatMech.v` plus `second_law`, `isothermal_work_bound`, `entropy_change_erasure`, `cno_preserves_shannon_entropy`. |
| `absolute-zero/proofs/coq/filesystem/FilesystemCNO.v` | 13 | §(c): syscall semantics (`mkdir_rmdir_inverse`, `create_unlink_inverse`, `read_write_identity`, `chmod_identity`, `chown_identity`, `rename_inverse`, `snapshot_restore_identity`) — POSIX-spec assumptions standard in filesystem formalisations. |
| `absolute-zero/proofs/coq/common/CNO.v` | 5 | §(c)+(d) mix: `eval_deterministic` (operational-semantics assumption), `cno_decidable` (decidability postulate), `eval_respects_state_eq_left/right` (one has a "TODO: Prove this axiom by induction on eval structure" comment — that's a (d) entry upstream). |
| `absolute-zero/proofs/coq/lambda/LambdaCNO.v` | 5 | §(c): `y_not_cno` (Y-combinator non-termination — well-established but not derivable in pure Coq), `eta_equivalence`. **Note: PROOF-NEEDS.md targets `y_not_cno` for discharge; close upstream.** |
| `absolute-zero/proofs/coq/quantum/QuantumMechanicsExact.v` | 4 | §(c): `X_gate_unitary`, `unitary_preserves_entropy`, `no_cloning` (subset of QuantumCNO.v). |
| `absolute-zero/proofs/coq/category/CNOCategory.v` | 2 | §(c): `hom_functor` (Hom-functor functoriality, standard category-theory result). |

### `absolute-zero/proofs/lean4/` (4 files, 54 markers)

| File | Markers | Sample disposition (in sibling repo) |
| --- | ---: | --- |
| `absolute-zero/proofs/lean4/FilesystemCNO.lean` | 22 | §(c): Lean4 port of `FilesystemCNO.v` axioms — same family of syscall-semantics postulates plus `mkdir_test_not_identity`, `mkdir_idempotent`. |
| `absolute-zero/proofs/lean4/StatMech.lean` | 15 | §(c): Lean4 port of `StatMech.v` plus `programState_eq_eval_fixpoint`, `state_preserving_dist`. |
| `absolute-zero/proofs/lean4/QuantumCNO.lean` | 14 | §(c): Lean4 port of `QuantumCNO.v`. |
| `absolute-zero/proofs/lean4/LambdaCNO.lean` | 3 | §(c): `subst_closed_term`, `y_combinator_not_identity`, `eta_equivalence`. |

### `absolute-zero/proofs/agda/` (1 file, 1 marker)

| File | Markers | Disposition |
| --- | ---: | --- |
| `absolute-zero/proofs/agda/EchoBridgeCNO.agda` | 1 | False positive on the strict grep — the single hit is `open import Axiom.Extensionality.Propositional`, which is a **module import** (the agda-stdlib path), not a `postulate`. Treat as resolved on close inspection. |

### One outlier on close inspection

The `EchoBridgeCNO.agda` count is a grep false positive
(`Axiom.Extensionality.Propositional` is the agda-stdlib module path,
not an axiom declaration). The real "load-bearing escape hatches" count
across `absolute-zero/` is therefore **149**, not 150. Schema preserves
the grep number for CI parity with `check-trusted-base.sh`.

## How to update this file

1. Re-run `bash scripts/check-trusted-base.sh .` (from standards#211)
   from the repo root.
2. If markers in `absolute-zero/` change, the close-out path is to fix
   the canonical `hyperpolymath/absolute-zero` repo and re-sync into
   this subtree.
3. If a **new** proof-bearing file appears **outside** `absolute-zero/`
   (e.g. under `aletheia/` for the Rust verification pipeline, or under
   `contractiles/`), classify it into §(a)/(b)/(c)/(d) per the canonical
   schema. That would be load-bearing maa-framework debt.

## Open owner question

1. **Vendoring policy**: should the `absolute-zero/` subtree be a git
   submodule (so the canonical proof-debt.md is the single source of
   truth) instead of a vendored copy? Current state is "in-tree copy
   that has drifted from the sibling repo's main branch". Filed for
   maintainer decision; this PR does not resolve it.
2. **Re-sync cadence**: until the vendoring question is resolved,
   maa-framework's vendored copies should be diffed against the
   sibling repo before each minor release, and the diff annotated in
   `CHANGELOG.adoc`.

## Companion documents

- [`PROOF-NEEDS.md`](../PROOF-NEEDS.md) — the broader strategic proof
  narrative for maa-framework as a whole.
- `hyperpolymath/standards`
  [`docs/TRUSTED-BASE-REDUCTION-POLICY.adoc`](https://github.com/hyperpolymath/standards/blob/main/docs/TRUSTED-BASE-REDUCTION-POLICY.adoc)
  — the canonical schema.
- `hyperpolymath/standards`
  [`scripts/check-trusted-base.sh`](https://github.com/hyperpolymath/standards/blob/main/scripts/check-trusted-base.sh)
  — the CI gate.
- `hyperpolymath/absolute-zero` `docs/proof-debt.md` — the canonical
  per-marker disposition for everything in §(e).

---

Initial seed under standards#203 + standards#211, 2026-05-27.
