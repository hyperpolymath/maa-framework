<!--
SPDX-License-Identifier: MPL-2.0
SPDX-FileCopyrightText: 2026 Jonathan D.A. Jewell (hyperpolymath)
-->

# Proof Debt — absolute-zero

**Schema**: [hyperpolymath/standards `TRUSTED-BASE-REDUCTION-POLICY.adoc`](https://github.com/hyperpolymath/standards/blob/main/docs/TRUSTED-BASE-REDUCTION-POLICY.adoc) (standards#203).

## Initial inventory

The 2026-05-26 estate proof-debt audit
([standards#195](https://github.com/hyperpolymath/standards/pull/195))
detected **129 soundness-relevant escape hatches** in this repo (now
**124** after intervening closures). Markers were originally seeded
in §(d) DEBT pending classification.

## Phase 1 triage — 72 Coq Axioms (2026-05-27, [#58](https://github.com/hyperpolymath/absolute-zero/pull/58))

The per-marker classification for every Coq `Axiom` lives in
[`docs/proof-debt-triage.md`](./proof-debt-triage.md). Summary:

| Disposition | Count |
|-------------|------:|
| §(c) AXIOM (TRUSTED-BASE) | 52 |
| §(a) DISCHARGE backlog    | 17 |
| §(b) PROPERTY-TEST        |  3 |
| **Total Coq Axioms**      | **72** |

Out of scope for Phase 1 (still in §(d) pending future triage):
52 Lean 4 `axiom` declarations and the 7 Idris2 postulates tracked by
[#27](https://github.com/hyperpolymath/absolute-zero/issues/27).

## Phase 2a triage — Lean Lambda cluster (2026-05-27)

Per-cluster Lean triage rolling out 2026-05-27 in cluster-sized PRs.
First cluster: `proofs/lean4/LambdaCNO.lean` (3 axioms).

| Line | Identifier | Disposition | Justification |
|-----:|------------|-------------|---------------|
| 183  | `subst_closed_term`        | §(d) DEBT   | Standard metatheoretic property of lambda calculus; provable by induction on `t` once the substitution-on-closed-terms lemma is mechanised. |
| 232  | `y_combinator_not_identity` | §(c) AXIOM | Non-termination claim about Y combinator; requires step-indexed semantics or coinduction (same justification as Coq `y_not_cno`). |
| 258  | `eta_equivalence`           | §(c) AXIOM | η-equivalence is not derivable under β-only reduction (same justification as Coq `eta_equivalence` at LambdaCNO.v:376). |

The two §(c) entries are annotated inline with `-- AXIOM:` leading
comments. The §(d) entry below has an owner + deadline.

## Phase 2c triage — Lean Filesystem cluster (2026-05-27)

Second Lean cluster: `proofs/lean4/FilesystemCNO.lean` (21 axioms).

### POSIX primitive operations (§(c) AXIOM — opaque ops, 10)

| Line | Identifier | Disposition | Justification |
|-----:|------------|-------------|---------------|
|  56  | `mkdir`     | §(c) AXIOM | Opaque POSIX primitive — no executable body in the model. |
|  60  | `rmdir`     | §(c) AXIOM | Opaque POSIX primitive. |
|  64  | `create`    | §(c) AXIOM | Opaque POSIX primitive. |
|  68  | `unlink`    | §(c) AXIOM | Opaque POSIX primitive. |
|  72  | `readFile`  | §(c) AXIOM | Opaque POSIX primitive. |
|  76  | `writeFile` | §(c) AXIOM | Opaque POSIX primitive. |
|  80  | `stat`      | §(c) AXIOM | Opaque POSIX primitive. |
|  84  | `chmod`     | §(c) AXIOM | Opaque POSIX primitive. |
|  88  | `chown`     | §(c) AXIOM | Opaque POSIX primitive. |
|  92  | `rename`    | §(c) AXIOM | Opaque POSIX primitive. |

### POSIX semantics specifications (§(c) AXIOM — mirror Coq, 6)

| Line | Identifier | Disposition |
|-----:|------------|-------------|
|  98  | `mkdir_rmdir_inverse`  | §(c) AXIOM (mirrors Coq) |
| 104  | `create_unlink_inverse`| §(c) AXIOM (mirrors Coq) |
| 109  | `read_write_identity`  | §(c) AXIOM (mirrors Coq) |
| 115  | `chmod_identity`       | §(c) AXIOM (mirrors Coq) |
| 121  | `rename_identity`      | §(c) AXIOM (mirrors Coq) |
| 126  | `rename_inverse`       | §(c) AXIOM (mirrors Coq) |

### Snapshot primitives (§(c) AXIOM — opaque ops, 2)

| Line | Identifier | Disposition |
|-----:|------------|-------------|
| 281  | `snapshot` | §(c) AXIOM (opaque snapshot primitive) |
| 285  | `restore`  | §(c) AXIOM (opaque restore primitive) |

### Discharge candidates (§(d) DEBT — 3)

These claim provable existence / equality facts that should follow
from the §(c) primitives once the model is concretely defined. They
need a discharge PR — see §(d) DEBT below.

| Line | Identifier | Disposition | Plan |
|-----:|------------|-------------|------|
| 233  | `mkdir_not_identity`         | §(d) DEBT | Existence proof; exhibit one concrete `fs` lacking the path. |
| 288  | `snapshot_restore_identity`  | §(d) DEBT | Composite theorem; derivable from `snapshot`/`restore` once a concrete snapshot model lands. |
| 309  | `mkdir_idempotent`           | §(d) DEBT | Follows from `mkdir_rmdir_inverse` family with stronger repeat-mkdir semantics. |

All 18 §(c) entries above are annotated inline with `-- AXIOM:`
leading comments.

## (a) DISCHARGE backlog (Coq, 17)

Provable propositions currently stated as `Axiom`. Enumerated in
[`docs/proof-debt-triage.md`](./proof-debt-triage.md) — each row marked
`DISCHARGE` is a candidate for a future proof PR.

## (b) BUDGETED — tested with a refutation budget (3)

Decidability claims over opaque types: `fs_eq_dec`, `state_dec`,
`state_eq_dec`. Belong to §(b) once a §(b) property-test budget is
attached; otherwise treat as §(c).

## (c) NECESSARY AXIOM (Coq, 52)

Physics constants, quantum gate primitives, POSIX semantics,
Kolmogorov + Shannon entropy core inequalities, complex exponential
algebra, and fundamental physical laws (second law, Landauer, no-cloning).
Full enumeration in [`docs/proof-debt-triage.md`](./proof-debt-triage.md).

## (d) DEBT — actively to be closed

After Phase 1, the §(d) bucket contains only the Lean axioms and 7
Idris2 postulates that have not yet been triaged. Coq markers are
no longer in §(d).

### Coq — provable, awaiting proof

- `proofs/coq/category/CNOCategory.v:323` — `hom_functor`
  - **Owner**: @hyperpolymath
  - **Plan**: replace `Axiom hom_functor : ... Functor C C` with the
    proper Yoneda construction `Functor C SetCategory`. The
    file's leading comment (L312-322) records why this is currently
    axiomatised: (1) `yoneda_cno` is already proven without it,
    (2) `SetCategory` needs universe-polymorphism machinery,
    (3) the conceptual claim stands.
  - **Triage**: classified DISCHARGE in `docs/proof-debt-triage.md`
    (Phase 1, #58).
  - **Deadline**: INDEFINITE (blocked on `SetCategory` instance —
    universe-polymorphism scaffolding precondition).

- `proofs/coq/filesystem/FilesystemCNO.v:300` — `mkdir_not_identity`
  - **Owner**: @hyperpolymath
  - **Plan**: existence proof; exhibit one concrete `fs` lacking the
    path. Triaged DISCHARGE in #58.
  - **Deadline**: INDEFINITE (small proof; awaits a discharge PR).

- `proofs/coq/filesystem/FilesystemCNO.v:316` — `write_different_not_identity`
  - **Owner**: @hyperpolymath
  - **Plan**: existence proof; exhibit one concrete content mismatch.
    Triaged DISCHARGE in #58.
  - **Deadline**: INDEFINITE.

- `proofs/coq/filesystem/FilesystemCNO.v:397` — `transaction_cno`
  - **Owner**: @hyperpolymath
  - **Plan**: composite theorem; derivable from primitive `_inverse`
    axioms once a `transaction` definition is in place. Triaged
    DISCHARGE in #58.
  - **Deadline**: INDEFINITE (blocked on `transaction` definition).

- `proofs/coq/filesystem/FilesystemCNO.v:421` — `mkdir_idempotent`
  - **Owner**: @hyperpolymath
  - **Plan**: follows from `mkdir_rmdir_inverse` family + stronger
    repeat-mkdir semantics. Triaged DISCHARGE in #58.
  - **Deadline**: INDEFINITE.

- `proofs/coq/filesystem/FilesystemCNO.v:453` — `snapshot_restore_identity`
  - **Owner**: @hyperpolymath
  - **Plan**: composite theorem; derivable from primitive `_identity`
    / `_inverse` axioms once a snapshot model lands. Triaged DISCHARGE
    in #58.
  - **Deadline**: INDEFINITE.

### Lean — provable, awaiting proof

- `proofs/lean4/LambdaCNO.lean:183` — `subst_closed_term`
  - **Owner**: @hyperpolymath
  - **Plan**: discharge by induction on `t : LambdaTerm`; closed-term
    invariant carries through `LVar`, `LAbs`, `LApp` cases. Sibling to
    Coq's `subst` lemmas in `proofs/coq/lambda/LambdaCNO.v`.
  - **Deadline**: INDEFINITE (no proof-PR scheduled yet — provable;
    awaits Lean-side discharge push).

- `proofs/lean4/FilesystemCNO.lean:233` — `mkdir_not_identity`
  - **Owner**: @hyperpolymath
  - **Plan**: existence proof; exhibit one concrete `fs` lacking the
    path. Mirrors Coq site at `FilesystemCNO.v:300`.
  - **Deadline**: INDEFINITE.

- `proofs/lean4/FilesystemCNO.lean:288` — `snapshot_restore_identity`
  - **Owner**: @hyperpolymath
  - **Plan**: composite theorem; derivable from `snapshot`/`restore`
    primitives once a concrete snapshot model is in place. Mirrors
    Coq site at `FilesystemCNO.v:453`.
  - **Deadline**: INDEFINITE.

- `proofs/lean4/FilesystemCNO.lean:309` — `mkdir_idempotent`
  - **Owner**: @hyperpolymath
  - **Plan**: follows from `mkdir_rmdir_inverse` + stronger
    repeat-mkdir semantics. Mirrors Coq site at `FilesystemCNO.v:421`.
  - **Deadline**: INDEFINITE.

### Lean — pending triage

28 Lean axioms remain to be triaged (QuantumCNO 14, StatMech 14;
Lambda and Filesystem clusters done in Phase 2a/2c). Triage planned
in cluster-sized PRs through 2026-06 — see this file's status block
at the bottom.

### Idris2 — pending triage

7 Idris2 postulates in `src/abi/Layout.idr`. Tracked by
[#27](https://github.com/hyperpolymath/absolute-zero/issues/27).

```
(Coq markers no longer in §(d) post Phase 1; see triage doc for §a/§b/§c.)
```

> If `129` > 30, the list above shows the first 30 only.
> The full list is reproducible via:
>
> ```bash
> bash /path/to/standards/scripts/check-trusted-base.sh .
> ```

## Suggested triage process

1. Run `scripts/check-trusted-base.sh` locally; it lists every marker
   with file:line.
2. For each marker, decide:
   - Can this be proven? → §(a) DISCHARGED via a PR that adds the proof.
   - Is this at an FFI / extraction / opaque-primitive boundary? →
     §(b) or §(c). Add a property test and document the refutation
     budget for §(b), or cite the metatheoretic justification for §(c).
   - Is this temporary debt? → §(d) with a deadline.
3. Update this file in the same PR that lands the disposition.
4. The `check-trusted-base` CI job (standards#211) ensures markers
   are never un-annotated AND un-enumerated simultaneously.

## Companion documents

- [standards#195](https://github.com/hyperpolymath/standards/pull/195) — estate proof-debt audit.
- [standards#203](https://github.com/hyperpolymath/standards/pull/203) — trusted-base reduction policy (the schema this file follows).
- [standards#211](https://github.com/hyperpolymath/standards/pull/211) — `check-trusted-base.sh` CI enforcement.

---

🤖 Initial seed by Claude Code, 2026-05-26.
