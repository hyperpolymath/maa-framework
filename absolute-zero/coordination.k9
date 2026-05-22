# Absolute Zero: Proof Coordination Protocol

## Project

Formal verification of Computational Non-reversible Operation (CNO) theory across multiple computational models.

**Languages:** Coq, Rust, Deno, Just
**License:** MPL-2.0
**Build system:** just
**Runtime:** deno

## Build Commands

| Command | Description |
|---------|-------------|
| `just build` | Compile all Coq proof scripts |
| `just test` | Run all verification tests |
| `just doctor` | Run system diagnostics |
| `just audit` | Run self-audit with panic-attack |

## INVARIANTS — Do Not Violate

### [CRITICAL] no-admitted-proofs

**Rule:** No `Admitted` proofs are permitted in the main branch.

**Why:** Theoretical boundaries must be explicitly axiomatized as `Axiom` or `Parameter` with detailed mathematical justification, rather than skipped with `Admitted`.

### [CRITICAL] rsr-compliance

**Rule:** Deno is the standard runtime; no Node.js or npm artifacts.

**Why:** Project-wide standard for reproducibility and security.

### [MODERATE] proof-parity

**Rule:** PROOF-COMPLETION-*.md must accurately reflect the state of .v files.

**Why:** Prevents "Proof Drift" and ensures publication claims are verifiable.

## Protected Files and Directories

| Path | Reason |
|------|--------|
| `proofs/coq/` | Canonical Coq formalization |
| `6a2/` | A2ML metadata for agentic coordination |
| `Justfile` | Unified entry point for all operations |

## Terminology

Use the correct terms for this project:

- Say **"CNO"**, NOT "reversible operation" or "reversible logic"
- Say **"Axiomatization"** for theoretical boundaries, NOT "placeholders"
