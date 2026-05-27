<!-- SPDX-License-Identifier: MPL-2.0 -->
# Roadmap

Short summary; the authoritative version is [`ROADMAP.adoc`](../../ROADMAP.adoc) at the root.

## Now (v1.0.0-alpha → v1.0.0)

* Discharge remaining Coq Admitteds (currently 0 in core, 19→0 historic)
* Complete Idris2 ABI typecheck (unblock `Types.idr` pre-existing errors)
* Wire `idris2 --build absolute-zero-abi.ipkg` into CI matrix
* Strict language-policy compliance (banned-lang examples removed 2026-05-25)
* RSR-template-repo conformance (see [RSR_COMPLIANCE.adoc](../../RSR_COMPLIANCE.adoc))

## Next (v0.8 → v0.9 milestones)

* `v0.8.0` — Compliance Sprint: complete checkpoint files, remove npm
* `v0.9.0` — Container Verification: Containerfile with all 6 provers, cross-arch CI
* `v1.0.0` — Publication: zero Admitted, paper submitted, 3 industrial CNO examples

## Long horizon (v2 → v12)

7-year vision: language expansion, AI-assisted proving, quantum verification,
standardisation. Detailed phase decomposition in [`ROADMAP.adoc`](../../ROADMAP.adoc).

## How decisions are recorded

* Forward-looking architectural choices → ADRs in
  [`.machine_readable/META.scm`](../../.machine_readable/META.scm)
  `architecture-decisions` section
* Backward-looking audit events → [`AUDIT.adoc`](../../AUDIT.adoc)
* Live phase / progress → [`.machine_readable/6a2/STATE.a2ml`](../../.machine_readable/6a2/STATE.a2ml)
