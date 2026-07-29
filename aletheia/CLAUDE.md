<!--
SPDX-License-Identifier: CC-BY-SA-4.0
Copyright (c) Jonathan D.A. Jewell <j.d.a.jewell@open.ac.uk>
-->
# Claude Development Documentation

This document provides guidance for Claude (AI assistant) when working on the Aletheia project.

## Project Overview

**Aletheia** (ἀλήθεια - "truth", "disclosure", "unconcealment") is an RSR (Rhodium Standard Repository) compliance verification tool written in Rust with zero dependencies.

### Core Purpose
- Verify repositories against RSR Bronze-level standards
- Promote security, documentation, and operational excellence
- Serve as a reference implementation of RSR principles

## Critical Constraints

### 🚫 NEVER Add Dependencies

Aletheia maintains **zero dependencies** for RSR Bronze-level compliance. This is non-negotiable.

**DO NOT**:
- Add crates to `Cargo.toml` `[dependencies]`
- Add dev-dependencies (except for testing tools if absolutely necessary)
- Import external libraries
- Use `extern crate` for third-party crates

**DO**:
- Use Rust standard library (`std`) exclusively
- Implement functionality from scratch if needed
- Prefer simple, auditable code over complex abstractions

### 🚫 NEVER Use Unsafe Code

Aletheia maintains **zero unsafe blocks** for RSR compliance.

**DO NOT**:
- Use `unsafe` keyword
- Use `#[unsafe(...)]` attributes
- Call FFI functions
- Use raw pointers in unsafe context

**DO**:
- Use safe Rust abstractions
- Leverage Rust's type system for safety
- Use `Option` and `Result` for error handling

## Architecture Principles

### Module Layout (updated 2026-07-27 — this section was stale)

> **This file used to say "all core logic lives in `src/main.rs` (~950 lines)" and
> "don't split into modules unless >1000 lines". Both were out of date.** The crate has
> been split for some time. Measured 2026-07-27:

| File | Lines | Contents |
|---|---|---|
| `src/main.rs` | 121 | CLI entry, arg parsing, `verify_repository`, exit policy |
| `src/checks.rs` | 316 | `check_documentation`, `check_spdx_headers`, `check_workflow_pins`, `check_path_security`, glob matching |
| `src/config.rs` | 243 | `.aletheia.toml` loading, hand-rolled TOML parse |
| `src/output.rs` | 226 | human / JSON / SARIF report printing, date+time formatting |
| `src/types.rs` | 89 | `ComplianceLevel`, `CheckResult`, `ComplianceReport`, … |
| **total** | **995** | |

The single-file rationale still applies *in spirit* — prefer few, auditable files over
deep abstraction — but do not "restore" the single-file layout, and do not treat 1000
lines as a threshold that has not yet been crossed.

**When to add more files**:
- Integration tests in `tests/` directory
- Examples in `examples/` directory
- Documentation in `docs/` directory

**When NOT to add files**:
- Don't create abstractions prematurely
- Don't add utility files for one-off functions

### Known gap: the CLI is unfinished (issues #124 / #125)

`main.rs` parses only `<repo-path>` plus `--json` / `--sarif`, and wires **three**
checks. `tests/integration_tests.rs` is 806 lines / 32 tests describing a much larger
tool (16 Bronze checks plus Silver, `--help`, `--version`, `--verbose`, `--badge`,
`--init-hook`, `--format=`, HTML output). **2 pass, 27 fail.**

Two things to know before touching it:

1. **Those tests assert on stdout substrings**, e.g.
   `assert!(stdout.contains("Bronze-level RSR compliance: ACHIEVED"))`. They pin the
   *wording* of the report and say nothing about what the checks must verify — so they
   can be satisfied by checks that verify nothing. Treat them as a UI contract, not a
   specification.
2. **A definition of RSR conformance already exists** elsewhere in the estate (hypatia's
   `rsr-conformance` oracle). Writing checks to satisfy these strings risks creating a
   second, divergent definition. Resolve the source-of-truth question first.

Most of the clippy findings in #125 are dead code that exists *because* those modules
are unwired. **Do not silence them with `#![allow(dead_code)]`** — the root Rust CI
header forbids it, and that dead code is the specification of the missing feature.

### Type Safety First

Leverage Rust's type system:

```rust
// GOOD: Use strong types
enum ComplianceLevel {
    Bronze,
    Silver,
    Gold,
    Platinum,
}

// BAD: Use strings for enumerable values
let level = "bronze"; // Avoid this
```

### Explicit Error Handling

```rust
// GOOD: Return Result, handle errors explicitly
fn check_file(path: &Path) -> Result<bool, std::io::Error> {
    let exists = path.exists();
    Ok(exists)
}

// BAD: Unwrap or panic
fn check_file(path: &Path) -> bool {
    path.exists().unwrap() // Never do this
}
```

### Offline-First

**DO NOT**:
- Make network requests
- Access external APIs
- Download files
- Phone home for telemetry

**DO**:
- Work entirely from local filesystem
- Be completely air-gapped compatible
- Function without internet access

## Development Workflow

### Making Changes

1. **Read existing code first**
   ```bash
   # Understand the current implementation
   cat src/main.rs
   ```

2. **Write tests for new functionality**
   ```rust
   #[test]
   fn test_new_feature() {
       // Test code here
   }
   ```

3. **Implement the feature**
   - Keep it simple
   - Follow existing patterns
   - Maintain zero dependencies

4. **Run checks**
   ```bash
   cargo fmt          # Format code
   cargo clippy       # Lint
   cargo test         # Run tests
   cargo run          # Self-verify RSR compliance
   ```

5. **Update documentation**
   - Update README.md if user-facing
   - Update CHANGELOG.md
   - Add doc comments for public APIs

### Code Style

**Follow Rust conventions**:
- Use `snake_case` for functions and variables
- Use `CamelCase` for types and enums
- Use `SCREAMING_SNAKE_CASE` for constants
- Max line length: 100 characters
- Use `cargo fmt` for formatting

**Documentation**:
```rust
/// Check if a file exists at the given path.
///
/// # Arguments
///
/// * `base` - Base directory path
/// * `filename` - Name of file to check
///
/// # Returns
///
/// `true` if file exists, `false` otherwise
fn file_exists(base: &Path, filename: &str) -> bool {
    base.join(filename).is_file()
}
```

### Testing Strategy

**Unit tests**: Test individual functions
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_exists() {
        // Test implementation
    }
}
```

**Integration tests**: Test complete workflows
```rust
// tests/integration_test.rs
#[test]
fn test_rsr_verification_workflow() {
    // Test complete verification process
}
```

**Manual testing**: Run on real repositories
```bash
cargo run -- /path/to/test/repository
```

## Common Tasks

### Adding a New Compliance Check

1. **Add check function**:
   ```rust
   fn check_new_requirement(report: &mut ComplianceReport, repo_path: &Path) {
       let passes = /* check logic */;
       report.add_check(
           "Category",
           "Requirement Name",
           passes,
           ComplianceLevel::Bronze,
       );
   }
   ```

2. **Call from `verify_repository()`**:
   ```rust
   fn verify_repository(repo_path: &Path) -> ComplianceReport {
       let mut report = ComplianceReport::new(repo_path.to_path_buf());
       check_documentation(&mut report, repo_path);
       check_new_requirement(&mut report, repo_path);  // Add this
       report
   }
   ```

3. **Add tests**:
   ```rust
   #[test]
   fn test_new_requirement_check() {
       // Test the new check
   }
   ```

### Updating RSR Standards

When RSR standards change:

1. Update checks in `src/main.rs`
2. Update documentation in `README.md`
3. Update `CHANGELOG.md`
4. Run self-verification: `cargo run`

### Releasing New Version

1. Update version in `Cargo.toml`
2. Update `CHANGELOG.md`
3. Run full checks: `just check`
4. Tag release: `git tag -a v0.x.0 -m "Release v0.x.0"`
5. Push: `git push && git push --tags`

## RSR Compliance Checklist

When making changes, ensure these remain true:

- [ ] ✅ Zero dependencies (`cargo tree --depth 0`)
- [ ] ✅ No unsafe code (`! rg "unsafe" src/`)
- [ ] ✅ Tests pass (`cargo test`)
- [ ] ✅ Clippy clean (`cargo clippy -- -D warnings`)
- [ ] ✅ Formatted (`cargo fmt --check`)
- [ ] ✅ Self-verification passes (`cargo run`)
- [ ] ✅ Documentation updated
- [ ] ✅ CHANGELOG.md updated

## File Structure Reference

```
aletheia/
├── src/                     # 5 modules, 995 lines total — see Module Layout above
│   ├── main.rs              # CLI entry (121)
│   ├── checks.rs            # compliance checks (316)
│   ├── config.rs            # .aletheia.toml (243)
│   ├── output.rs            # human/JSON/SARIF (226)
│   └── types.rs             # core types (89)
├── tests/
│   └── integration_tests.rs # 32 tests — 2 pass, 27 fail (issue #124)
├── benches/                 # Performance benchmarks
├── examples/                # Usage examples
├── fuzz/                    # Fuzzing infrastructure
├── .well-known/
│   ├── security.txt         # RFC 9116 security contact
│   ├── ai.txt               # AI training policies
│   └── humans.txt           # Human attribution
├── .github/workflows/       # 16 files — ⚠ ALL INERT, see below
├── Cargo.toml               # Zero dependencies, MSRV 1.80
├── Cargo.lock               # Lock file (commit this)
├── Justfile                 # Build automation
├── .gitlab-ci.yml           # CI/CD pipeline (GitLab mirror)
├── .gitignore               # Git ignore patterns
├── README.adoc              # User documentation (AsciiDoc)
├── SECURITY.md              # Security policy
├── CONTRIBUTING.md          # Contribution guidelines
├── CODE_OF_CONDUCT.md       # Community standards
├── MAINTAINERS.md           # Governance
├── CHANGELOG.md             # Version history
├── CLAUDE.md                # This file
├── LICENSE                  # Palimpsest License (MPL-2.0)
├── STATE.scm                # Current project state
├── ECOSYSTEM.scm            # Ecosystem connections
└── META.scm                 # Project metadata and ADRs
```

### ⚠ The 16 workflows in `aletheia/.github/workflows/` have NEVER run

`aletheia/` is **vendored into `maa-framework` as plain tracked files** (mode 100644),
not a submodule, and GitHub Actions reads `.github/workflows/` **at the repository root
only**. There is no standalone `hyperpolymath/aletheia` repo running them either — it was
deleted around January 2026 and its content vendored here.

Consequence: editing anything under `aletheia/.github/workflows/` has **no effect on
CI whatsoever**. This is not hypothetical — it is how commit `b5322c2` (2026-06-17) left
this crate **failing to compile for over a month** with nobody noticing.

**The real gate is `/.github/workflows/rust-ci.yml` at the repository root.** It runs
`cargo build` (debug + release), `cargo test`, `cargo fmt --check` and a zero-dependency
assertion, with `working-directory: aletheia`. Add gates there, not here. See
`aletheia/.github/workflows/README.md`.

## Troubleshooting

### "Dependency detected" error
- Check `Cargo.toml` - should have no `[dependencies]` section with crates
- Run `cargo tree --depth 0` to verify

### "Unsafe code detected" error
- Search for `unsafe` keyword: `rg "unsafe" src/`
- Remove all unsafe blocks

### Tests failing
- Run with verbose output: `cargo test -- --nocapture`
- Check test logic and assertions
- Ensure tests are deterministic

### RSR self-verification fails
- Run `cargo run` to see which checks fail
- Fix missing files or requirements
- Ensure all documentation is present

## Best Practices for AI Assistants

1. **Read before writing**: Always read existing code before making changes
2. **Maintain constraints**: Never add dependencies or unsafe code
3. **Test everything**: Write tests for new functionality
4. **Document changes**: Update docs and CHANGELOG.md
5. **Follow conventions**: Use existing patterns and style
6. **Keep it simple**: Prefer simple, clear code over clever abstractions
7. **Self-verify**: Run `cargo run` to verify RSR compliance

## Philosophical Notes

### Why Zero Dependencies?

1. **Security**: No supply chain attacks
2. **Audibility**: Easy to review entire codebase
3. **Reliability**: No dependency breakage
4. **Simplicity**: Fewer moving parts
5. **Trust**: Users can verify everything

### Why No Unsafe Code?

1. **Safety**: Rust's guarantees apply everywhere
2. **Trust**: No hidden memory bugs
3. **Simplicity**: No manual memory management
4. **Audibility**: No special cases to review

### Why Offline-First?

1. **Privacy**: Cannot exfiltrate data
2. **Reliability**: Works without internet
3. **Speed**: No network latency
4. **Trust**: Users control all inputs

## Contact

For questions about this document or Aletheia development:

- **GitHub**: https://github.com/hyperpolymath/aletheia
- **Author**: Jonathan D.A. Jewell <j.d.a.jewell@open.ac.uk>

---

**Last Updated**: 2026-07-27
**Version**: 1.2

*"Alētheia is not just absence of falsehood, but active unconcealment of truth."*
