<!--
SPDX-License-Identifier: MPL-2.0
SPDX-FileCopyrightText: 2026 Jonathan D.A. Jewell (hyperpolymath)
-->

# Proof Debt — maa-framework

**Schema**: [hyperpolymath/standards `TRUSTED-BASE-REDUCTION-POLICY.adoc`](https://github.com/hyperpolymath/standards/blob/main/docs/TRUSTED-BASE-REDUCTION-POLICY.adoc) (standards#203).

This file is the schema-conformant **index** for this repo's proof
debt. The substantive content lives in
[`PROOF-NEEDS.md`](../PROOF-NEEDS.md) — keep that file as the source of truth
and use this one as the schema bridge for the
`check-trusted-base.sh` CI gate ([standards#211](https://github.com/hyperpolymath/standards/pull/211)).

## Marker count (2026-05-26)

134 soundness-relevant escape hatches detected by
`scripts/check-trusted-base.sh`. See `PROOF-NEEDS.md` for the per-site
classification rationale.

## (a) DISCHARGED in this repo

See `PROOF-NEEDS.md` — any markers no longer present in source.

## (b) BUDGETED — tested with a refutation budget

See `PROOF-NEEDS.md` — markers at FFI / extraction boundaries with
documented property-test budgets.

## (c) NECESSARY AXIOM

See `PROOF-NEEDS.md` — markers that encode metatheoretic assumptions.

## (d) DEBT — actively to be closed

See `PROOF-NEEDS.md` — markers still owed a proof or a §(b)/§(c)
classification.

## How to update this file

When markers change in source:

1. Update `PROOF-NEEDS.md` first (source of truth).
2. If the marker count changes substantially, update the count above
   so this index doesn't drift.
3. The `check-trusted-base` CI gate (standards#211) reads BOTH
   `docs/proof-debt.md` AND `PROOF-NEEDS.md` for documentation lookups,
   so the index can stay light.

## Companion documents

- [standards#195](https://github.com/hyperpolymath/standards/pull/195) — estate proof-debt audit.
- [standards#203](https://github.com/hyperpolymath/standards/pull/203) — trusted-base reduction policy (the schema).
- [`PROOF-NEEDS.md`](../PROOF-NEEDS.md) — this repo's substantive proof-debt audit (source of truth).

---

🤖 Schema-conformant index seeded by Claude Code, 2026-05-26.
