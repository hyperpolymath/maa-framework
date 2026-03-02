# Hyperpolymath Project Value Chain Policy

This document defines the standard value chain model for all Hyperpolymath projects. It is based on Porter's Value Chain framework, adapted for software development.

## Overview

Every Hyperpolymath project has two types of activities:

1. **Support Activities** - Cross-cutting concerns that apply at all stages
2. **Primary Activities** - The value flow from inputs to outputs

## Support Activities (Cross-Cutting)

These apply horizontally across all primary activities:

| Activity | Description | Tools |
|----------|-------------|-------|
| **Technology** | CCCP stack (containers, runtime, shell) | Cerro Torre, Svalinn, Valence Shell |
| **Formalizations** | Proofs, validation, verification | Absolute Zero, Echidna, Coq/Lean/Z3 |
| **Standards** | Repository and code standards | RSR, CCCP, .scm files, Palimpsest |
| **Infrastructure** | Build, CI/CD, deployment | mustfile, rhodium-pipeline, GitLab CI |

## Primary Activities (Value Flow)

```
┌─────────────┐   ┌─────────────┐   ┌─────────────┐   ┌─────────────┐   ┌─────────────┐
│  INBOUND    │   │             │   │  OUTBOUND   │   │  MARKETING  │   │             │
│  LOGISTICS  │──▶│  OPERATIONS │──▶│  LOGISTICS  │──▶│  & SALES    │──▶│   SERVICE   │
│             │   │             │   │             │   │             │   │             │
│ Theoretical │   │   Core      │   │  Outputs    │   │  Standards  │   │ Maintenance │
│ foundations │   │   project   │   │  & exports  │   │  & business │   │   & ops     │
└─────────────┘   └─────────────┘   └─────────────┘   └─────────────┘   └─────────────┘
```

### 1. Inbound Logistics
**What**: Foundational inputs specific to the project's domain

Examples:
- For Aletheia: Absolute Zero (CNO proofs, Landauer, Bennett)
- For rhodibot: RSR specification
- For oikos: Economic/ecological theory

### 2. Operations
**What**: The core project work - what this repository actually builds

Examples:
- For Aletheia: Reversible Minix research (r-Minix precursor)
- For rhodibot: Compliance checking engine
- For oikos: Economic/ecological analyzer

### 3. Outbound Logistics
**What**: Where outputs go - integration with CCCP stack or other projects

Examples:
- To CCCP technology stack (containers, runtime)
- To downstream projects
- To future implementations (e.g., Aletheia → r-Minix)

### 4. Marketing & Sales
**What**: RSR ecosystem, maintenance strategies, business extensions

Components:
- **Corrective maintenance** - Bug fixes, issue resolution
- **Adaptive maintenance** - Platform/environment changes
- **Perfective maintenance** - Improvements, optimizations

### 5. Service
**What**: Ongoing operational support via automation

Tools:
- **feedback-o-tron** - Feedback collection and processing
- **cicd-hyper-a** - CI/CD automation
- **robot-repo-automaton** - Repository automation

## Two Input Systems

All projects receive inputs from two complementary systems:

### RSR - Rhodium Standard Repositories
- **Layer**: Repository organization
- **Handles**: Documentation, security.txt, CI/CD, build systems
- **Tools**: rhodium-standard-repositories, rsr-template-repo, rhodibot, rhodium-pipeline

### CCCP - Campaign for Cooling Coding and Programming
- **Layer**: Technology stack and code
- **Handles**: Languages, containers, integrations, efficiency
- **Tools**: Cerro Torre, Svalinn, Valence Shell, rescript-tea, cadre-router

## Project-Specific Foundations

Each project may have domain-specific foundational inputs:

| Project | Foundational Input |
|---------|-------------------|
| Aletheia | Absolute Zero (CNO proofs, reversibility theory) |
| Valence Shell | Absolute Zero (filesystem CNO proofs) |
| Oikos | Economic/ecological theory |
| Echidnabot | Formal methods, SAT/SMT solving |

## Standard Files

All properly configured projects include:

### AI Support Files (.scm - Guile Scheme)
- `META.scm` - Project metadata
- `ECOSYSTEM.scm` - Ecosystem connections
- `STATE.scm` - Current project state
- `PLAYBOOK.scm` - Development workflows
- `AGENTIC.scm` - AI agent configuration
- `NEUROSYM.scm` - Neurosymbolic settings

### Build & Standards
- `mustfile` - Build/task automation
- RSR compliance files (README, LICENSE, SECURITY.md, etc.)
- `.well-known/` directory

### Licensing
- **Base**: MIT or AGPL (user choice)
- **Overlay**: Palimpsest License (optional but encouraged)

## Bot Ecosystem

Projects are supported by bots operating in multiple modes:

| Bot | Role |
|-----|------|
| **rhodibot** | RSR compliance (consultant, advisor, regulator) |
| **echidnabot** | Formal proof validation |
| **oikos bot** | Economic/ecological analysis |

Bots are **self-referential** - they validate their own code.

## Applying This Policy

When setting up a new project:

1. Start with `rsr-template-repo`
2. Apply customizations via `repo-customiser`
3. Identify the project's **foundational input** (inbound logistics)
4. Define the **core operations** (what this project builds)
5. Specify **outbound targets** (where outputs go)
6. Configure **maintenance strategy** (corrective/adaptive/perfective)
7. Set up **service automation** (feedback-o-tron, cicd-hyper-a, etc.)

## Diagram

```
┌─────────────────────────────────────────────────────────────────────────────────────┐
│                           SUPPORT ACTIVITIES (Cross-cutting)                        │
├─────────────────────────────────────────────────────────────────────────────────────┤
│  TECHNOLOGY      │ CCCP: Cerro Torre, Svalinn, Valence Shell, rescript-tea...      │
├──────────────────┼──────────────────────────────────────────────────────────────────┤
│  FORMALIZATIONS  │ Absolute Zero, Echidna, Coq/Lean/Z3, formal proofs              │
├──────────────────┼──────────────────────────────────────────────────────────────────┤
│  STANDARDS       │ RSR (repo), CCCP (code), .scm files, Palimpsest licensing       │
├──────────────────┼──────────────────────────────────────────────────────────────────┤
│  INFRASTRUCTURE  │ mustfile, rhodium-pipeline, GitLab CI, GitHub Actions           │
└──────────────────┴──────────────────────────────────────────────────────────────────┘
                                            │
┌───────────────────────────────────────────┼─────────────────────────────────────────┐
│                           PRIMARY ACTIVITIES (Value Flow)                           │
│                                           │                                         │
│  ┌───────────┐   ┌───────────┐   ┌────────┴──┐   ┌───────────┐   ┌───────────┐     │
│  │ INBOUND   │   │           │   │ OUTBOUND  │   │ MARKETING │   │           │     │
│  │ LOGISTICS │──▶│ OPERATIONS│──▶│ LOGISTICS │──▶│ & SALES   │──▶│  SERVICE  │     │
│  └───────────┘   └───────────┘   └───────────┘   └───────────┘   └───────────┘     │
│                                                                                     │
│  Foundational    Core project    To CCCP/        RSR system      feedback-o-tron   │
│  theory/proofs   development     other projects  Maintenance     cicd-hyper-a      │
│                                                  strategies      robot-repo-auto   │
│                                                                                     │
│  ═══════════════════════════════════════════════════════════════════════════════▶  │
│                                   VALUE CREATION                           MARGIN  │
└─────────────────────────────────────────────────────────────────────────────────────┘
```

---

*This policy applies to all Hyperpolymath projects and their satellite repositories.*
