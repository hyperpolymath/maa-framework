<!--
SPDX-License-Identifier: CC-BY-SA-4.0
Copyright (c) Jonathan D.A. Jewell <j.d.a.jewell@open.ac.uk>
-->
<!-- TOPOLOGY.md — Project architecture map and completion dashboard -->
<!-- Last updated: 2026-02-19 -->

# MAA Framework — Project Topology

## System Architecture

```
                        ┌─────────────────────────────────────────┐
                        │              MAA FRAMEWORK              │
                        │        (Full-Stack System Paradigm)     │
                        └───────────────────┬─────────────────────┘
                                            │
                                            ▼
                        ┌─────────────────────────────────────────┐
                        │           GOVERNANCE & POLICY           │
                        │  ┌───────────┐  ┌───────────────────┐  │
                        │  │ AI Ethics │  │  RSR Standards    │  │
                        │  │ (Axiology)│  │  (Compliance)     │  │
                        │  └─────┬─────┘  └────────┬──────────┘  │
                        └────────│─────────────────│──────────────┘
                                 │                 │
                                 ▼                 ▼
                        ┌─────────────────────────────────────────┐
                        │           LANGUAGE & KERNEL             │
                        │  ┌───────────┐  ┌───────────────────┐  │
                        │  │ Oblíbený  │  │  Aletheia         │  │
                        │  │ (Language)│  │  (Microkernel)    │  │
                        │  └─────┬─────┘  └────────┬──────────┘  │
                        └────────│─────────────────│──────────────┘
                                 │                 │
                                 ▼                 ▼
                        ┌─────────────────────────────────────────┐
                        │          DISTRIBUTED RUNTIME            │
                        │      (BOINC, RISC-V, Sustainable)       │
                        └─────────────────────────────────────────┘

                        ┌─────────────────────────────────────────┐
                        │          REPO INFRASTRUCTURE            │
                        │  Justfile / Mustfile  .machine_readable/  │
                        │  Multi-Forge Hub      0-AI-MANIFEST.a2ml  │
                        └─────────────────────────────────────────┘
```

## Completion Dashboard

```
COMPONENT                          STATUS              NOTES
─────────────────────────────────  ──────────────────  ─────────────────────────────────
SPECIFICATIONS
  Framework Paradigm                ██████████ 100%    Core scope defined
  Oblíbený Reference Lang           ██████░░░░  60%    Grammar & semantics refining
  Aletheia Microkernel              ████░░░░░░  40%    Initial architecture stubs
  AI Ethics & Axiology              ████████░░  80%    Value-alignment policies stable

INFRASTRUCTURE
  Multi-Forge Mirroring             ██████████ 100%    GH/GL/BB/CB sync stable
  Supply Chain Security             ██████████ 100%    SHA-pinned actions verified
  .machine_readable/                ██████████ 100%    STATE tracking active

REPO INFRASTRUCTURE
  Justfile Automation               ██████████ 100%    Standard build/sync tasks
  0-AI-MANIFEST.a2ml                ██████████ 100%    AI entry point verified
  Language Policy (CCCP)            ██████████ 100%    RSR stack verified

─────────────────────────────────────────────────────────────────────────────
OVERALL:                            ██████░░░░  ~60%   Governance stable, Core pending
```

## Key Dependencies

```
Framework Spec ───► Oblíbený Lang ───► Aletheia Kernel ───► Runtime
     │                 │                   │                 │
     ▼                 ▼                   ▼                 ▼
RSR Standards ───► Infrastructure ───► Multi-Forge ──────► Compliance
```

## Update Protocol

This file is maintained by both humans and AI agents. When updating:

1. **After completing a component**: Change its bar and percentage
2. **After adding a component**: Add a new row in the appropriate section
3. **After architectural changes**: Update the ASCII diagram
4. **Date**: Update the `Last updated` comment at the top of this file

Progress bars use: `█` (filled) and `░` (empty), 10 characters wide.
Percentages: 0%, 10%, 20%, ... 100% (in 10% increments).
