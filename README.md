// SPDX-License-Identifier: MPL-2.0
= MAA Framework — Mutually Assured Accountability
:toc: preamble
:toc-title: Contents
:icons: font
:doctype: article

image:https://img.shields.io/badge/OpenSSF-BestPractices-green[link="https://www.bestpractices.dev/projects/XXXX"]

Governance hub and microkernel for the Mutually Assured Accountability paradigm: building verifiably-compliant, secure, and sustainable systems where accountability is symmetric and enforceable.

== Overview

Mutually Assured Accountability (MAA) is a paradigm where accountability between system participants is symmetric—each party can verify the other's compliance, and no party is above audit. This repository is the governance hub and implementation centre for the MAA ecosystem.

The ecosystem has three components:

[cols="1,2", options="header"]
|===
| Component | Role

| MAA Framework
| Governance hub, specifications, CI, and integration layer

| Oblíbený
| Reference language designed for the framework (specification stage)

| Aletheia
| Microkernel implementation (Rust, in-tree)
|===

The Certified Null Operation (CNO) formalisation is developed upstream in `link:https://github.com/hyperpolymath/absolute-zero[absolute-zero]` and pinned here as a git submodule.

== What is built and what is planned

[cols="1,2,2", options="header"]
|===
| Artefact | Status | Evidence

| Aletheia microkernel (Rust)
| Builds, 29 unit tests passing, zero dependencies
| `aletheia/`, CI gate `rust-ci.yml`

| CRG Tier-C test suite
| Landed 2026-04-04 (unit, E2E, property, aspect, benchmarks)
| `tests/`

| CI gates at repository root
| Green across every enabled workflow; each shown capable of failing
| `.github/workflows/`

| Hub-and-spoke mirroring
| GitHub → GitLab, Codeberg, Bitbucket (automated)
| Mirror workflows

| CLI surface for Aletheia
| **Unbuilt** — `main.rs` wires 3 checks; integration suite specifies more
| Issue #124

| Oblíbený reference language
| **Specification stage**
| Not yet in tree

| Specification documents
| **Pending upload** (ROADMAP Phase 1)
| `docs/` (partial)
|===

== Known gaps

[CAUTION]
====
**Aletheia CLI is unbuilt.** `main.rs` wires 3 checks. The integration suite specifies far more. 27 of 29 integration tests fail by design (not by regression), because the code to make them pass does not exist yet. Tracked in Issue #124.
====

[CAUTION]
====
**23 clippy findings in Aletheia**, mostly dead code that exists because of the unbuilt CLI (#124). Not yet a blocking gate. Tracked in Issue #125.
====

[CAUTION]
====
**`aletheia/.github/workflows/` is inert.** GitHub Actions reads `.github/workflows/` at the repository root only, and `aletheia/` is vendored as plain tracked files (not a submodule). Those 16 workflow files have never executed. Gates must be added at the root, not inside `aletheia/`.
====

[CAUTION]
====
**Specification uploads are pending.** ROADMAP Phase 1 (specification publication) is not complete. The project scope is stated; the formal specifications backing it are not yet in-tree.
====

== What is standard and what is ours

[cols="1,2,2", options="header"]
|===
| Concept | Status | Home

| Microkernel architecture (Rust)
| Standard
| `aletheia/`

| Hub-and-spoke repo mirroring
| Standard
| `.github/workflows/` (mirror sync)

| SHA-pinned GitHub Actions
| Standard supply-chain hygiene
| `.github/workflows/`

| Mutually Assured Accountability paradigm
| **Novel governance concept**
| This repository

| Symmetric accountability enforcement
| **Novel** (paradigm definition)
| Specifications (pending)

| Oblíbený reference language
| **Novel** (specification stage)
| Not yet in tree
|===

== Project scope

The framework addresses these domains. Where an implementation exists, it is noted; where it does not, the domain is a stated intention:

[cols="1,2", options="header"]
|===
| Domain | Current status

| Security & Formal Verification
| CNO submodule pinned; Aletheia builds

| AI Ethics & Axiology
| Specification pending

| Dependability
| Specification pending

| Language Design & Compilers
| Oblíbený at specification stage

| Microkernel Architecture
| Aletheia: builds, 29 unit tests, zero dependencies

| RISC-V
| Specification pending

| Sustainability
| Specification pending

| Economics-as-Code
| Specification pending

| Distributed Computing
| Specification pending

| Reversibility
| CNO submodule (upstream: `absolute-zero`)

| Web Protocols
| Specification pending
|===

== Repository Layout

[cols="1,3", options="header"]
|===
| Path | Purpose

| `aletheia/`
| Microkernel implementation (Rust, vendored in-tree)

| `absolute-zero/`
| CNO formalisation (git submodule, developed upstream)

| `tests/`
| E2E, property, aspect tests (CRG Tier-C)

| `.github/workflows/`
| CI gates and mirror synchronization (root level only)

| `docs/`
| Specifications and state-of-play documents

| `.machine_readable/`
| 6a2 metadata, contractiles, bot directives
|===

== Build

[source,bash]
----
# Aletheia microkernel
cd aletheia/
cargo build
cargo test

# Full CI (from repository root)
just verify
----

== Documentation

* link:EXPLAINME.adoc[EXPLAINME] — claim-by-claim receipts and known gaps
* link:Glossary.adoc[Glossary] — terminology reference
* `docs/STATE-OF-PLAY-2026-07-29.adoc` — CI status, gated checks, proof state, open questions
* `ROADMAP.adoc` — development phases
* `GOVERNANCE.adoc` — project governance
* link:https://github.com/hyperpolymath/absolute-zero[absolute-zero] — CNO formalisation (upstream)

== License

SPDX-License-Identifier: MPL-2.0 — see link:LICENSE[LICENSE].

Prose documentation is licensed under CC-BY-SA-4.0; see `LICENSES/`.
[cols="1,3", options="header"]
|===
| Path | Purpose

| `aletheia/`
| Microkernel implementation (Rust, vendored in-tree)
