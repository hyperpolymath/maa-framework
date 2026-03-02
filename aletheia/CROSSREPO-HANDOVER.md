# Cross-Repository Handover Document

**For**: Cross-repo Claude sessions managing the Hyperpolymath ecosystem
**Date**: 2026-02-05
**Author**: Generated during aletheia code review session

## Overview

This document provides context for AI assistants working across the Hyperpolymath repository ecosystem. The ecosystem is designed so that new projects start perfectly configured and maintain quality throughout their lifecycle.

## The Vision

> "Every project should leave the user with a perfect repo that has everything and avoids anything problematic, so that the project itself is just fantastic. All of this should happen in the background."

## Two Input Systems

Every Hyperpolymath project receives inputs from two complementary systems:

### RSR - Rhodium Standard Repositories
**What**: Repository organization and compliance standards
**Handles**: Documentation, security.txt, CI/CD, build systems, project structure
**Tools**: rhodium-standard-repositories, rsr-template-repo, rhodibot, rhodium-pipeline

### CCCP - Campaign for Cooling Coding and Programming
**What**: Technology stack and code standards
**Handles**: Language choices, containers, integrations, efficient code practices
**Tools**: cccp-portfolio, svalinn (containers), corre-terro (images), scaffoldia

**Core Infrastructure (almost always present):**
| Tool | Purpose |
|------|---------|
| **Cerro Torre** | Provenance-verified container base images (replaces Alpine/Wolfi) |
| **Svalinn** | Edge shield (ReScript/Deno) + Vörðr OCI runtime |
| **Valence Shell** | Reversible shell scripting |
| **Echidna/echidnabot** | Formal proof-based code validation |

**Application Layer (as needed):**
| Tool | Purpose |
|------|---------|
| **rescript-tea** | The Elm Architecture for ReScript |
| **cadre-router** | Typed HTTP routing with CRDTs |
| **casket-ssg** | Static site generator |
| **poly-*-mcp** | MCP tools (ssg, isc, container, etc.) |

**Bot Pattern (from oikos):**
Bots operate in multiple modes: consultant, advisor, regulator, policy developer.
They are **self-referential** - they validate their own code.
Polyglot: specialized languages for each task.

**The Connection to Thermodynamics**:
- Landauer's Principle: Computation dissipates heat (kT ln 2 per bit erased)
- Reversible computing: Zero heat dissipation (CNOs from Absolute Zero)
- CCCP "Cooling": Efficient, low-energy, well-structured code

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                              ANY PROJECT                                    │
└─────────────────────────────────────────────────────────────────────────────┘
                    ▲                               ▲
                    │                               │
        ┌───────────┴───────────┐       ┌───────────┴───────────┐
        │         RSR           │       │         CCCP          │
        │  (Repository Layer)   │       │   (Code/Tech Layer)   │
        │                       │       │                       │
        │ • How repo organized  │       │ • What code goes in   │
        │ • Docs, CI/CD, badges │       │ • Languages, patterns │
        │ • Compliance checking │       │ • Containers, images  │
        └───────────────────────┘       └───────────────────────┘
```

## Ecosystem Architecture

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                         STANDARDS & TEMPLATES                               │
│                                                                             │
│  ┌─────────────────────────────┐    ┌─────────────────────────────┐        │
│  │ rhodium-standard-repositories│    │     rsr-template-repo       │        │
│  │                             │    │                             │        │
│  │  The RSR Specification      │───▶│  Template for new repos     │        │
│  │  Bronze/Silver/Gold/Platinum│    │  Scaffolding & structure    │        │
│  └─────────────────────────────┘    └──────────────┬──────────────┘        │
└────────────────────────────────────────────────────┼────────────────────────┘
                                                     │
                                                     ▼
┌─────────────────────────────────────────────────────────────────────────────┐
│                         CUSTOMIZATION LAYER                                 │
│                                                                             │
│  ┌─────────────────────────────┐    ┌─────────────────────────────┐        │
│  │     repo-customiser         │    │    slm-repo-automaton       │        │
│  │                             │    │    robot-repo-automaton     │        │
│  │  C4 Control Point           │◀──▶│                             │        │
│  │  Works with ANY .scm std    │    │  Automation agents          │        │
│  └─────────────────────────────┘    └─────────────────────────────┘        │
└────────────────────────────────────────────────────┬────────────────────────┘
                                                     │
                                                     ▼
┌─────────────────────────────────────────────────────────────────────────────┐
│                         COMPLIANCE & CI/CD                                  │
│                                                                             │
│  ┌─────────────────────────────┐    ┌─────────────────────────────┐        │
│  │        rhodibot             │    │    rhodium-pipeline         │        │
│  │                             │    │                             │        │
│  │  Compliance bot/advisor     │◀──▶│  CI/CD generator            │        │
│  │  Consultant/Regulator       │    │  Pipeline templates         │        │
│  └─────────────────────────────┘    └─────────────────────────────┘        │
└────────────────────────────────────────────────────┬────────────────────────┘
                                                     │
                                                     ▼
┌─────────────────────────────────────────────────────────────────────────────┐
│                    VERIFICATION & VALIDATION                                │
│                                                                             │
│  ┌─────────────────────────────┐    ┌─────────────────────────────┐        │
│  │          oikos              │    │       echidnabot            │        │
│  │                             │    │                             │        │
│  │  Economic/Ecological eval   │    │  Formal proof verification  │        │
│  │  Sustainability metrics     │    │  Code validation            │        │
│  └─────────────────────────────┘    └─────────────────────────────┘        │
└─────────────────────────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────────────────────────┐
│                         PRESENTATION LAYER                                  │
│                                                                             │
│  ┌─────────────────────────────┐    ┌─────────────────────────────┐        │
│  │       *-ssg repos           │    │      poly-ssg-mcp           │        │
│  │                             │    │                             │        │
│  │  Static site generators     │◀──▶│  MCP tool for SSG selection │        │
│  │  Various frameworks         │    │  Unified interface          │        │
│  └─────────────────────────────┘    └─────────────────────────────┘        │
└─────────────────────────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────────────────────────┐
│                    THEORETICAL FOUNDATIONS                                  │
│                                                                             │
│  ┌─────────────────────────────┐                                           │
│  │       absolute-zero         │                                           │
│  │                             │                                           │
│  │  Formal CNO verification    │                                           │
│  │  Coq/Lean/Z3/Agda/Isabelle  │                                           │
│  └──────────────┬──────────────┘                                           │
│                 │                                                           │
│       ┌─────────┴─────────┐                                                │
│       ▼                   ▼                                                │
│  ┌────────────┐    ┌─────────────┐    ┌─────────────────────────┐         │
│  │valence-shell│    │  aletheia   │    │       r-Minix           │         │
│  │            │    │             │    │      (FUTURE)           │         │
│  │ Reversible │    │ Precursor   │───▶│                         │         │
│  │ filesystem │    │ to r-Minix  │    │ Full reversible Minix   │         │
│  └────────────┘    └─────────────┘    └─────────────────────────┘         │
└─────────────────────────────────────────────────────────────────────────────┘
```

## Repository Descriptions

### Standards Layer

| Repository | Purpose | Status |
|------------|---------|--------|
| `rhodium-standard-repositories` | RSR specification (Bronze/Silver/Gold/Platinum) | Active |
| `rsr-template-repo` | Template for creating RSR-compliant repos | Active |

### Customization Layer

| Repository | Purpose | Status |
|------------|---------|--------|
| `repo-customiser` | C4 control point for ANY .scm standard | Active |
| `slm-repo-automaton` | Small Language Model automation | Check status |
| `robot-repo-automaton` | Robot/automation agent | Check status |

### Compliance Layer

| Repository | Purpose | Status |
|------------|---------|--------|
| `rhodibot` | RSR compliance bot (like Dependabot for standards) | Extraction in `/aletheia/extraction/rhodibot/` |
| `rhodium-pipeline` | CI/CD pipeline generator | Extraction in `/aletheia/extraction/rhodium-pipeline/` |

### Verification Layer

| Repository | Purpose | Status |
|------------|---------|--------|
| `oikos` | Economic/ecological evaluation | Check status |
| `echidnabot` | Formal proof verification (Echidna integration) | Check status |

### Presentation Layer

| Repository | Purpose | Status |
|------------|---------|--------|
| `*-ssg` repos | Various static site generators | Multiple repos |
| `poly-ssg-mcp` | MCP tool for SSG selection | Check status |

### Theoretical Foundations

| Repository | Purpose | Status |
|------------|---------|--------|
| `absolute-zero` | Formal CNO verification (Coq/Lean/Z3/Agda/Isabelle) | Active, Phase 1 complete |
| `valence-shell` | Reversible filesystem operations | Check status |
| `aletheia` | RSR verification tool & precursor to r-Minix | v1.0.0 - Stable |

## Aletheia Specific Setup

This project (aletheia) will have the following structure when properly configured:

### Bots Present
- **echidnabot** - Formal proof-based code validation
- **oikos bot** - Economic/ecological analysis
- **rhodibot** - RSR compliance checking

### AI Support Files (.scm - Guile Scheme)
| File | Purpose |
|------|---------|
| `META.scm` | Project metadata |
| `ECOSYSTEM.scm` | Ecosystem connections and dependencies |
| `STATE.scm` | Current project state |
| `PLAYBOOK.scm` | Development playbook/workflows |
| `AGENTIC.scm` | AI agent configuration |
| `NEUROSYM.scm` | Neurosymbolic integration settings |

### Other Files
- `mustfile` - Custom build/task file format (hyperpolymath standard)
- Created via `repo-customiser`

### Licensing
- **Primary**: Palimpsest License (PMPL-1.0-or-later)

### Foundational Input
- **Absolute Zero** - CNO proofs, reversibility theory, Landauer/Bennett thermodynamics

## Workflow: New Project Creation

```
1. User starts with rsr-template-repo
                    │
                    ▼
2. repo-customiser applies customizations
   ├── slm-repo-automaton (if applicable)
   ├── robot-repo-automaton (if applicable)
   └── User's .scm configuration
                    │
                    ▼
3. rhodium-pipeline sets up CI/CD
   ├── GitHub Actions / GitLab CI
   ├── RSR compliance checks
   └── Build/test/deploy stages
                    │
                    ▼
4. rhodibot monitors ongoing compliance
   ├── Continuous checking
   ├── Badge generation
   └── Advisory reports
                    │
                    ▼
5. Verification tools validate
   ├── oikos: Economic/ecological metrics
   ├── echidnabot: Formal proofs
   └── Other validators as needed
                    │
                    ▼
6. SSG generates documentation/site
   └── poly-ssg-mcp selects appropriate generator
```

## Key Relationships

### rhodibot ↔ rhodium-pipeline
- rhodibot checks compliance
- rhodium-pipeline enforces compliance in CI/CD
- Both are satellites of `rhodium-standard-repositories`

### absolute-zero ↔ aletheia
- absolute-zero provides formal proofs (CNOs)
- aletheia is the precursor to practical reversible OS
- Valence Shell handles filesystem reversibility
- All feed toward future r-Minix project

### repo-customiser ↔ Everything
- Generic C4 control point
- Works with ANY .scm formatted standard
- RSR is one standard, but can handle others (oikos, custom)

### echidnabot ↔ absolute-zero
- Both deal with formal verification
- echidnabot focuses on practical code validation
- absolute-zero focuses on theoretical proofs
- They complement each other

## Critical Notes for AI Assistants

### DO NOT Confuse

1. **RSR compliance** (the standard) vs **RSR checking tools** (rhodibot/pipeline)
   - A project should BE RSR compliant
   - It should NOT become an RSR tool (unless that's its purpose)

2. **aletheia** is NOT an RSR compliance tool
   - It's a precursor to reversible Minix
   - RSR compliance is a feature, not the purpose

3. **repo-customiser** works with ANY standard
   - Not just RSR
   - oikos, custom standards, etc.

### When Working on a Project

1. Check what the project's PURPOSE is (not just its compliance)
2. Apply RSR as a badge/feature, not as the project's identity
3. Reference the theoretical foundations where appropriate
4. Maintain connections to the ecosystem without overwriting purpose

### Extraction Status

The following extractions exist in the `extraction/` directory:
- `rhodibot/` - Complete RSR compliance bot package
- `rhodium-pipeline/` - Complete CI/CD pipeline generator

These should be transferred to their proper repositories.

## File Format Standards

| File Type | Format | Notes |
|-----------|--------|-------|
| README | `.adoc` (AsciiDoc) preferred | Richer formatting |
| CLAUDE | `.md` (Markdown) | Simpler for AI parsing |
| Config | `.scm` (Guile Scheme) | STATE.scm, META.scm, ECOSYSTEM.scm |
| Licenses | `.txt` or `.md` | Dual licensing common |

## Language Standards (Hyperpolymath)

| Allowed | Use Case |
|---------|----------|
| ReScript | Primary application code |
| Deno | Runtime & package management |
| Rust | Performance-critical, systems, WASM |
| Gleam | Backend services |
| Bash/POSIX | Scripts, automation |

| Banned | Replacement |
|--------|-------------|
| TypeScript | ReScript |
| Node.js/npm/bun | Deno |
| Go | Rust |
| Python (general) | ReScript/Rust |

## Value Chain Policy

All Hyperpolymath projects follow a standardized value chain model based on Porter's Value Chain framework. See **[VALUE-CHAIN-POLICY.md](docs/VALUE-CHAIN-POLICY.md)** for the complete specification.

### Quick Reference

```
Support Activities (Cross-cutting):
├── Technology      → CCCP stack (Cerro Torre, Svalinn, Valence Shell)
├── Formalizations  → Absolute Zero, Echidna, formal proofs
├── Standards       → RSR, CCCP, .scm files, Palimpsest
└── Infrastructure  → mustfile, rhodium-pipeline, CI/CD

Primary Activities (Value Flow):
INBOUND → OPERATIONS → OUTBOUND → MARKETING → SERVICE
(theory)   (core work)  (outputs)   (RSR/maint)  (automation)
```

Each project must identify:
1. **Foundational Input** - What theoretical/practical inputs does it receive?
2. **Core Operations** - What does this project actually build?
3. **Outbound Targets** - Where do outputs go?
4. **Maintenance Strategy** - Corrective/Adaptive/Perfective
5. **Service Automation** - feedback-o-tron, cicd-hyper-a, etc.

## Next Steps for Cross-Repo Work

1. **Transfer extractions**: Move rhodibot and rhodium-pipeline to proper repos
2. **Evolve aletheia**: Continue RSR verification tool development alongside reversible Minix precursor research
3. **Align absolute-zero**: Ensure CLAUDE.md format consistency
4. **Check satellite repos**: Verify status of oikos, echidnabot, SSG repos
5. **Update rsr-template-repo**: Incorporate latest learnings
6. **Document repo-customiser**: Clarify C4 control point role

## Contact

**Jonathan D. A. Jewell**
- GitLab: [@hyperpolymath](https://gitlab.com/hyperpolymath)
- GitHub: [@Hyperpolymath](https://github.com/Hyperpolymath)

---

*This handover document enables continuity across Claude sessions working on the Hyperpolymath ecosystem.*
