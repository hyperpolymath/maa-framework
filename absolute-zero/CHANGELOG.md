<!--
SPDX-License-Identifier: MPL-2.0
SPDX-FileCopyrightText: 2026 Jonathan D.A. Jewell (hyperpolymath)
-->

# Changelog

All notable changes to `absolute-zero` will be documented in this file.

This file is generated from conventional commits by the
[`changelog-reusable.yml`](https://github.com/hyperpolymath/standards/blob/main/.github/workflows/changelog-reusable.yml)
workflow (`hyperpolymath/standards#206`). Adopt the workflow in this repo's CI to keep this file in sync automatically — see
[`templates/cliff.toml`](https://github.com/hyperpolymath/standards/blob/main/templates/cliff.toml)
for the canonical config.

The format follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/);
this project aims to follow [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- feat(absolute-zero): complete loadStore_preserves_memory proof — no sorry

### Fixed

- fix(baseline): repair main + estate-policy sweep (unblocks #41) (#42)
- fix(governance): enumerate banned-language demos in .hypatia-ignore (#44)
- fix(coq/cno): drop cno_decidable axiom (Rice's theorem territory) (#36)
- fix(licence): canonicalise to PMPL-1.0-or-later per authorship check (#133) (#34)
- fix(lean4/cno): finish loadStore_preserves_memory cons-case build (#28)
- fix(lean4/cno): finish loadStore_preserves_memory cons-case build (#23)
- fix(licence): canonicalise to PMPL-1.0-or-later per authorship check (#133) (#22)
- fix(licence): clear scaffold-placeholder leak (isolated; dirty repo) (#20)
- fix(ci): sync hypatia-scan.yml to canonical (413: env.HOME+Phase-2+SARIF) (#18)
- fix(ci): adopt canonical hypatia-scan.yml (env.HOME/scanner-layout + Comment-step gate) (#16)

### Documentation

- docs: Phase 1 per-axiom triage of 72 Coq Axioms (#58)
- docs: seed docs/proof-debt.md per trusted-base policy (#52)
- docs: record tech-debt audit findings (2026-05-26) (#47)

### CI

- ci(rust): convert rust-ci.yml to thin wrapper (standards#174 refile) (#53)
- ci: bump actions/upload-artifact SHA to current v4 (#12)
- ci(secret-scanner): drop duplicate --fail from trufflehog extra_args (#11)
- ci: fix workflow-linter YAML parse error + self-flag bug
- ci(antipattern): fix top-level dir matching + benchmarks/lsp/bench filename allowlists (#9)

## Pre-history

Prior commits to this file's introduction are recorded in git history but not formally classified into Keep-a-Changelog sections. To backfill, run `git cliff -o CHANGELOG.md` locally using the canonical [`cliff.toml`](https://github.com/hyperpolymath/standards/blob/main/templates/cliff.toml) — this is one-shot mechanical work.

---

<!-- This file was seeded by the 2026-05-26 estate tech-debt audit follow-up (Row-2 Phase 3); see [`hyperpolymath/standards/docs/audits/2026-05-26-estate-documentation-debt.md`](https://github.com/hyperpolymath/standards/blob/main/docs/audits/2026-05-26-estate-documentation-debt.md). -->
