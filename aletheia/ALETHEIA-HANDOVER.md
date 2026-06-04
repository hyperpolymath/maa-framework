<!--
SPDX-License-Identifier: MPL-2.0
Copyright (c) Jonathan D.A. Jewell <j.d.a.jewell@open.ac.uk>
-->
# Aletheia Project Handover

**For**: AI assistants starting work on the Aletheia project
**Updated**: 2026-02-05

## Conformance Badges

[![RSR Bronze](https://img.shields.io/badge/RSR-Bronze-cd7f32)](conformance/RSR-CONFORMANCE.md)
[![Palimpsest License](https://img.shields.io/badge/License-Palimpsest%20v0.8-blue)](conformance/PALIMPSEST-CONFORMANCE.md)
[![Oikos Pending](https://img.shields.io/badge/Oikos-Pending-lightgrey)](conformance/OIKOS-CONFORMANCE.md)
[![Echidna Pending](https://img.shields.io/badge/Echidna-Pending-lightgrey)](conformance/ECHIDNA-CONFORMANCE.md)
[![Value Chain](https://img.shields.io/badge/Value%20Chain-Defined-green)](docs/VALUE-CHAIN-POLICY.md)

## What is Aletheia?

**Aletheia** (ἀλήθεια - "truth", "disclosure", "unconcealment") is the **precursor research project** for building a reversible operating system based on Minix.

### Core Purpose

Aletheia explores **reversible system operations** - the foundational research required before undertaking a full rewrite of Minix into a modern, reversible operating system (**r-Minix**).

### What Aletheia is NOT

- ❌ NOT an RSR compliance checking tool (that's [rhodibot](extraction/rhodibot/))
- ❌ NOT a CI/CD pipeline generator (that's [rhodium-pipeline](extraction/rhodium-pipeline/))
- ❌ NOT just about repository standards

### What Aletheia IS

- ✅ Research into reversible computing at the OS level
- ✅ Practical application of CNO (Certified Null Operation) theory
- ✅ Precursor work for r-Minix
- ✅ Connected to [Absolute Zero](https://gitlab.com/hyperpolymath/absolute-zero) formal proofs

## Value Chain Position

```
┌────────────────────────────────────────────────────────────────┐
│ ALETHEIA VALUE CHAIN                                           │
├────────────────────────────────────────────────────────────────┤
│                                                                │
│  INBOUND          OPERATIONS        OUTBOUND                   │
│  ════════         ══════════        ════════                   │
│  Absolute Zero    Reversible        CCCP Stack                 │
│  • CNO proofs     Minix research    • Valence Shell           │
│  • Landauer       • OS primitives   • Future r-Minix          │
│  • Bennett        • Undo/redo       • Svalinn integration     │
│                   • State capture                              │
│                                                                │
└────────────────────────────────────────────────────────────────┘
```

For the complete value chain model, see [VALUE-CHAIN-POLICY.md](docs/VALUE-CHAIN-POLICY.md).

## Theoretical Foundation

Aletheia builds on **Absolute Zero** - formal verification of Certified Null Operations:

| Concept | Description |
|---------|-------------|
| **CNO** | Certified Null Operation: `op ;; reverse(op) ≡ identity` |
| **Landauer's Principle** | Computation dissipates heat (kT ln 2 per bit erased) |
| **Bennett's Insight** | Reversible computation = zero heat dissipation |
| **Reversibility** | Every operation has an inverse |

## Ecosystem Connections

```
                     ┌─────────────────────┐
                     │   absolute-zero     │
                     │  (Formal Proofs)    │
                     └──────────┬──────────┘
                                │ provides CNOs
                                ▼
┌───────────────┐    ┌─────────────────────┐    ┌───────────────┐
│ valence-shell │◀───│      ALETHEIA       │───▶│   r-Minix     │
│  (Reversible  │    │                     │    │   (FUTURE)    │
│  Filesystem)  │    │ Precursor research  │    │               │
└───────────────┘    └─────────────────────┘    └───────────────┘
```

## Project Setup

### Bots Present
- **echidnabot** - Formal proof-based code validation
- **oikos bot** - Economic/ecological analysis
- **rhodibot** - RSR compliance checking

### AI Support Files (.scm - Guile Scheme)
| File | Purpose |
|------|---------|
| `META.scm` | Project metadata |
| `ECOSYSTEM.scm` | Ecosystem connections |
| `STATE.scm` | Current project state |
| `PLAYBOOK.scm` | Development workflows |
| `AGENTIC.scm` | AI agent configuration |
| `NEUROSYM.scm` | Neurosymbolic settings |

### Build System
- `mustfile` - Hyperpolymath build/task automation

### Licensing
- **Primary**: [Palimpsest License (MPL-2.0)](LICENSE)

## Quick Reference

### Language Policy

| Allowed | Use Case |
|---------|----------|
| Rust | Systems, performance-critical |
| ReScript | Application code |
| Deno | Runtime |
| Gleam | Backend services |

| Banned | Use Instead |
|--------|-------------|
| TypeScript | ReScript |
| Node.js/npm | Deno |
| Go | Rust |
| Python (general) | Rust/AffineScript |

### Key Files

| File | Purpose |
|------|---------|
| `ALETHEIA-HANDOVER.md` | This file - project handover |
| `CROSSREPO-HANDOVER.md` | Full ecosystem documentation |
| `CLAUDE.md` | AI assistant instructions |
| `docs/VALUE-CHAIN-POLICY.md` | Value chain model |
| `conformance/` | Conformance reports |

## Conformance Reports

All conformance reports are in the [`conformance/`](conformance/) folder:

| Report | Status | Description |
|--------|--------|-------------|
| [RSR-CONFORMANCE.md](conformance/RSR-CONFORMANCE.md) | Bronze | Repository standards |
| [PALIMPSEST-CONFORMANCE.md](conformance/PALIMPSEST-CONFORMANCE.md) | Active | License compliance |
| [OIKOS-CONFORMANCE.md](conformance/OIKOS-CONFORMANCE.md) | Pending | Economic/ecological |
| [ECHIDNA-CONFORMANCE.md](conformance/ECHIDNA-CONFORMANCE.md) | Pending | Formal verification |

## Getting Started

1. **Read this document** to understand Aletheia's purpose
2. **Check CLAUDE.md** for development constraints
3. **Review conformance/** for current compliance status
4. **Start with the research** - focus on reversible OS primitives

## Contact

**Jonathan D. A. Jewell**
- GitLab: [@hyperpolymath](https://gitlab.com/hyperpolymath)
- GitHub: [@Hyperpolymath](https://github.com/Hyperpolymath)

---

*Aletheia: Unconcealing the path to reversible operating systems.*
