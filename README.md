<!--
SPDX-License-Identifier: CC-BY-SA-4.0
SPDX-FileCopyrightText: 2025-2026 Jonathan D.A. Jewell <j.d.a.jewell@open.ac.uk>
-->

[![License: MPL-2.0](https://img.shields.io/badge/License-MPL--2.0-brightgreen.svg)](https://www.mozilla.org/en-US/MPL/2.0/) :toc: macro :toclevels: 3 :icons: font

A full-stack, open-source paradigm for building verifiably-compliant,
secure, and sustainable systems.

<div id="toc">

</div>

# Status

> [!IMPORTANT]
> **Early implementation** — Specification uploads still pending
> (ROADMAP Phase 1).
>
> This repository serves as the official hub and governance center for
> the MAA Framework project. Implementation work lives in-tree under
> `aletheia/` (the microkernel, Rust); the Certified Null Operation is
> developed in the standalone `hyperpolymath/absolute-zero` repository and
> pinned here as a git submodule under `absolute-zero/` (PR #89) — CNO
> development happens upstream, and this pointer is bumped deliberately. A
> CRG Tier-C test suite (unit, E2E, property, aspect, benchmarks) landed
> 2026-04-04.
> Detailed specification documents will be uploaded as development
> progresses.

**Current state (2026-07-29):** CI is green across every enabled workflow, and
each gate has been shown capable of failing. `aletheia` builds, passes 29 unit
tests, is formatted, and has zero dependencies — enforced by
`.github/workflows/rust-ci.yml`, the repository's first real Rust gate (added
2026-07-21).

Two known gaps, both in `aletheia` and both blocked on one design ruling:

| Issue | Gap |
|----|----|
| [#124](https://github.com/hyperpolymath/maa-framework/issues/124) | The CLI surface is unbuilt — `main.rs` wires 3 checks; the integration suite specifies far more. 27 of 29 integration tests fail **by design**, not by regression. |
| [#125](https://github.com/hyperpolymath/maa-framework/issues/125) | 23 clippy findings, mostly dead code that exists *because* of #124. Not yet a blocking gate. |

> [!WARNING]
> **`aletheia/.github/workflows/` is inert.** GitHub Actions reads
> `.github/workflows/` at the *repository root only*, and `aletheia/` is
> vendored as plain tracked files rather than a submodule. Those 16 workflow
> files have never executed. Add gates at the root, not there.

📄 **Full measured snapshot: [`docs/STATE-OF-PLAY-2026-07-29.adoc`](docs/STATE-OF-PLAY-2026-07-29.adoc)**
— CI status, what is actually gated, proof state, open questions, and the
landmines to read before changing CI or history here. Machine-readable
counterpart: [`.machine_readable/6a2/STATE.a2ml`](.machine_readable/6a2/STATE.a2ml).

# Overview

The MAA Framework is an integrated ecosystem comprising:

| Component | Description |
|----|----|
| **MAA Framework** | Full-stack paradigm for verifiably-compliant, secure, sustainable systems |
| **Oblíbený** | Reference language designed for the framework |
| **Aletheia** | Microkernel implementation |

# Project Scope

The framework addresses the following domains:

- **Security & Formal Verification** — Provable correctness guarantees

- **AI Ethics & Axiology** — Value-aligned system design

- **Dependability** — Fault-tolerant, reliable computing

- **Language Design & Compilers** — Purpose-built toolchains

- **Microkernel Architecture** — Minimal trusted computing base

- **RISC-V** — Open hardware architecture support

- **Sustainability** — Resource-efficient computing

- **Economics-as-Code** — Programmable economic primitives

- **Distributed Computing** — BOINC-compatible workloads

- **Reversibility** — Turing-complete/incomplete computation models

- **Web Protocols** — Modern network standards

# Repository Contents

## Currently Available

    .
    ├── .claude/CLAUDE.md      # Language policy (Hyperpolymath Standard)
    ├── .github/
    │   ├── CODEOWNERS         # Maintainer assignments
    │   └── workflows/         # CI wrappers + multi-forge synchronization
    ├── .machine_readable/     # 6a2 metadata, contractiles, bot directives
    ├── aletheia/              # Microkernel implementation (Rust)
    ├── tests/                 # E2E, property, aspect tests
    ├── GOVERNANCE.adoc        # Project governance
    ├── README.adoc            # This file
    ├── EXPLAINME.adoc         # Receipts backing README claims
    ├── ROADMAP.adoc           # Development phases
    └── SECURITY.md            # Vulnerability reporting policy

## Infrastructure

- **Hub-and-spoke mirroring** to GitLab, Codeberg, Bitbucket

- **SHA-pinned GitHub Actions** for supply chain security

- **SSH host key verification** for MITM protection

- **Minimal permission model** (`contents:` `read`)

# Language Policy

This project follows the **Hyperpolymath Standard** for technology
choices.

| Technology          | Use Case                            |
|---------------------|-------------------------------------|
| AffineScript        | Primary application code            |
| Deno                | Runtime & package management        |
| Rust                | Performance-critical, systems, WASM |
| Tauri 2.0+ / Dioxus | Mobile applications                 |
| Gleam               | Backend services (BEAM/JS)          |
| Guile Scheme        | State/meta files                    |

Allowed

See <a href=".claude/CLAUDE.md" class="md">CLAUDE</a> for complete
policy.

# Mirrors

GitHub  
[hyperpolymath/maa-framework](https://github.com/hyperpolymath/maa-framework)
**(canonical)**

GitLab  
Synchronized automatically

Codeberg  
Synchronized automatically

Bitbucket  
Synchronized automatically

# Security

See <a href="SECURITY.md" class="md">SECURITY</a> for:

- Vulnerability reporting procedures

- Security measures implemented

- Secret management practices

# License

This project is licensed under the Mozilla Public License, v. 2.0. See
the `LICENSE` file for details.

SPDX-License-Identifier: CC-BY-SA-4.0

# Contributing

Contribution guidelines will be established as the project develops.
Currently maintained by
[@hyperpolymath](https://github.com/hyperpolymath).
