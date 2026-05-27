<!--
SPDX-License-Identifier: MPL-2.0
SPDX-FileCopyrightText: 2026 Jonathan D.A. Jewell (hyperpolymath)
-->

# Changelog

All notable changes to `maa-framework` will be documented in this file.

This file is generated from conventional commits by the
[`changelog-reusable.yml`](https://github.com/hyperpolymath/standards/blob/main/.github/workflows/changelog-reusable.yml)
workflow (`hyperpolymath/standards#206`). Adopt the workflow in this repo's CI to keep this file in sync automatically — see
[`templates/cliff.toml`](https://github.com/hyperpolymath/standards/blob/main/templates/cliff.toml)
for the canonical config.

The format follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/);
this project aims to follow [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- feat(crg): add crg-grade and crg-badge justfile recipes
- feat: add comprehensive test suite to achieve CRG C compliance
- feat: add benchmarks, unit tests, E2E and aspect tests for absolute-zero
- feat: add stapeln.toml container definition
- feat: deploy UX Manifesto infrastructure
- feat: add CLADE.a2ml — clade taxonomy declaration

### Fixed

- fix(baseline): unblock 2 baseline-rot checks blocking dependabot #69 (#70)
- fix(ci): pin upload-artifact to valid SHA in hypatia-scan.yml (Refs standards#48) (#57)
- fix(ci): bump a2ml/k9-validate-action pins to canonical (#55)
- fix(ci): sync hypatia-scan.yml to canonical (#54)
- fix(ci): build Hypatia escript from repo root (estate dogfood drift)
- fix(ci): adopt canonical hypatia-scan.yml (#52)
- fix(ci): rsr-antipattern.yml duplicate heredoc (#49)
- fix(ci): move secret-scanner Cargo.toml gate from job-level if: to step-level (#50)
- fix: remove eval, quote vars, use mktemp in absolute-zero shell scripts
- fix: eliminate all 3 sorry in LambdaCNO proofs

### Documentation

- docs: substantive CRG C annotation (EXPLAINME.adoc)
- docs(test): achieve CRG C — document all test categories passing
- docs: add EXPLAINME.adoc — prove-it file backing README claims

### CI

- build(deps): bump dtolnay/rust-toolchain from efa25f7f19611383d5b0ccf2d1c8914531636bf9 to 3c5f7ea28cd621ae0bf5283f0e981fb97b8a7af9 (#69)
- build(deps): bump haskell-actions/setup from 2.7.5 to 2.11.0 (#68)
- build(deps): bump actions/upload-pages-artifact from 3.0.1 to 5.0.0 (#67)
- build(deps): bump actions/checkout from 4.1.1 to 6.0.2 (#66)
- build(deps): bump actions/configure-pages from 5.0.0 to 6.0.0 (#65)

## Pre-history

Prior commits to this file's introduction are recorded in git history but not formally classified into Keep-a-Changelog sections. To backfill, run `git cliff -o CHANGELOG.md` locally using the canonical [`cliff.toml`](https://github.com/hyperpolymath/standards/blob/main/templates/cliff.toml) — this is one-shot mechanical work.

---

<!-- This file was seeded by the 2026-05-26 estate tech-debt audit follow-up (Row-2 Phase 3); see [`hyperpolymath/standards/docs/audits/2026-05-26-estate-documentation-debt.md`](https://github.com/hyperpolymath/standards/blob/main/docs/audits/2026-05-26-estate-documentation-debt.md). -->
