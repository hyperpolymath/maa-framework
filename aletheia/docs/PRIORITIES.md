# Aletheia Project Priorities

**Snapshot Date**: 2025-12-26 (Updated)
**Current Version**: 0.1.x
**Compliance Status**: 16/16 Bronze RSR (100%)
**Codebase Size**: ~810 lines (main.rs) + ~480 lines (tests)

---

## Current State Assessment

### Strengths
| Area | Status | Notes |
|------|--------|-------|
| Zero Dependencies | Achieved | Only std library |
| Zero Unsafe Code | Achieved | 100% safe Rust |
| Self-Verification | Passing | 16/16 checks |
| Test Coverage | Excellent | 28 tests (10 unit + 18 integration) |
| Security Features | Implemented | Symlink detection, timestamps, warnings |
| Documentation | Complete | All required docs present |
| Build Systems | Complete | Cargo, Just, Nix, CI/CD |
| CLI Features | Complete | JSON, quiet, verbose, exit codes |
| Security Audit | Complete | 2025-12-26, 0 vulnerabilities |

### Technical Debt
| Item | Severity | Location |
|------|----------|----------|
| Timestamp impl is verbose | Low | `format_timestamp()` - 50 lines for date math |
| Some `#[allow(dead_code)]` | Low | Reserved for future use |

---

## MUST (Critical - Do First)

### Immediate (Before v0.1.0 Release) - COMPLETED

1. ~~**Tag and Release v0.1.0**~~ ✅ COMPLETED
   - Current state is release-ready
   - Tag: `git tag -a v0.1.0 -m "Initial release"`
   - Create release notes from CHANGELOG.md

2. **Verify CI/CD Pipeline Works** - PENDING
   - `.gitlab-ci.yml` exists but untested in production
   - Run full pipeline before announcing release
   - Fix any blocking issues

3. ~~**Update CHANGELOG.md**~~ ✅ COMPLETED
   - Document security improvements from this audit
   - Add symlink detection feature
   - Add timestamp feature

4. ~~**Security Audit Documentation**~~ ✅ COMPLETED
   - Update SECURITY.md "Last Audit" section
   - Document that audit was performed 2025-12-26
   - Note zero vulnerabilities found

### Before v0.2.0 - COMPLETED

5. ~~**Add JSON Output Format**~~ ✅ COMPLETED
   - `--format json` flag implemented
   - Machine-parseable results
   - CI/CD ready

6. ~~**Exit Codes for Categories**~~ ✅ COMPLETED
   - 0 = success (all checks passed)
   - 1 = compliance failure
   - 2 = security warning (critical)
   - 3 = invalid path
   - 4 = invalid arguments

---

## SHOULD (Important - Do Soon)

### v0.2.0 - v0.3.0 Timeframe

7. **Configuration File Support** - PENDING
   - `.aletheia.toml` or `.aletheia.yaml`
   - Allow ignoring specific checks
   - Custom severity levels
   - Per-project overrides

8. **SARIF Output** - PENDING
   - Static Analysis Results Interchange Format
   - GitHub/GitLab security dashboard integration
   - Industry standard for security tools

9. ~~**Quiet and Verbose Modes**~~ ✅ COMPLETED
   - `--quiet` - Only show pass/fail
   - `--verbose` - Show all details including explanations
   - Help text: `--help`

10. **Better Error Messages** - PENDING
    - Current: "Path does not exist"
    - Better: "Path '/foo/bar' does not exist. Did you mean '/foo/baz'?"
    - Suggest fixes for common issues

11. **README.md Generation** - PENDING
    - `aletheia init` command
    - Generate missing Bronze-level files
    - Template-based scaffolding

12. ~~**GitHub Actions Workflow**~~ ✅ COMPLETED
    - `.github/workflows/aletheia.yml` template provided
    - Easy adoption for GitHub users
    - Multi-platform support

13. ~~**Performance Benchmarks**~~ ✅ COMPLETED
    - `benches/verification_benchmark.rs` fully implemented
    - Measures verification speed (~12ms avg)
    - Tracks min/max/avg with warmup

14. ~~**More Integration Tests**~~ ✅ COMPLETED
    - 18 integration tests total
    - Tests: JSON output, quiet/verbose modes, exit codes
    - Tests: timestamp format, README.adoc alternative
    - Tests: invalid args, invalid paths, format syntax

---

## COULD (Nice to Have - Do Later)

### v0.4.0 - v0.6.0 Timeframe

15. **Silver-Level RSR Checks**
    - Formal verification hooks
    - Property-based testing detection
    - Coverage thresholds
    - Mutation testing validation

16. **Multi-Language Detection**
    - Detect project language (Rust, Python, Go, etc.)
    - Language-specific checks
    - Build system detection

17. **HTML Report Generation**
    - Visual compliance report
    - Shareable dashboard
    - Badge generation

18. **Watch Mode**
    - `aletheia watch`
    - Re-verify on file changes
    - Developer feedback loop

19. **Library API**
    - Currently binary-only
    - Expose as library crate
    - Programmatic verification

20. **Incremental Verification**
    - Cache previous results
    - Only re-check changed files
    - Faster CI runs

21. **Batch Verification**
    - `aletheia scan /path/to/repos/*`
    - Verify multiple repos at once
    - Summary report

### v0.7.0+ (Long-term)

22. **Gold-Level RSR**
    - Multi-language support
    - Polyglot build systems
    - Cross-language dependencies

23. **Platinum-Level RSR**
    - CADRE integration
    - Formal proofs
    - Enterprise features

24. **Plugin Architecture**
    - Custom check modules
    - Third-party extensions
    - Language-specific plugins

25. **LSP Integration**
    - IDE feedback
    - Real-time compliance hints
    - Editor extensions

---

## Anti-Priorities (MUST NOT)

These violate RSR Bronze principles and must NEVER be done:

| Anti-Priority | Reason |
|---------------|--------|
| Add external dependencies | Violates zero-dependency principle |
| Use `unsafe` code | Violates memory safety guarantee |
| Make network requests | Violates offline-first principle |
| Phone home / telemetry | Privacy violation |
| Break single-binary | Deployment complexity |
| Remove existing tests | Quality regression |

---

## Recommended Next Actions

### This Week
1. ~~Update CHANGELOG.md with security audit results~~ ✅ DONE
2. Tag v0.1.0 release
3. Verify GitLab CI/CD pipeline

### This Month
4. ~~Implement `--format json`~~ ✅ DONE
5. ~~Add distinct exit codes~~ ✅ DONE
6. ~~Write more integration tests~~ ✅ DONE (18 total)
7. Configuration file support (`.aletheia.toml`)
8. SARIF output format

### This Quarter
9. Silver-level RSR planning
10. `aletheia init` scaffolding command
11. Better error messages with suggestions
12. Library API (programmatic usage)

---

## Metrics to Track

| Metric | Current | Target (v1.0) |
|--------|---------|---------------|
| Lines of Code | ~810 | <1000 |
| Unit Tests | 10 | 25+ |
| Integration Tests | 18 | 20+ |
| Dependencies | 0 | 0 |
| Unsafe Blocks | 0 | 0 |
| Bronze Compliance | 100% | 100% |
| Clippy Warnings | 0 | 0 |
| Security Audit | Complete | Annual |

---

## Decision Log

| Date | Decision | Rationale |
|------|----------|-----------|
| 2025-12-26 | Add symlink detection | Security: prevent repo escape |
| 2025-12-26 | Add timestamps | TOCTOU mitigation |
| 2025-12-26 | Accept README.adoc | Flexibility for AsciiDoc users |
| 2025-12-26 | Critical warnings fail verification | Security-first approach |

---

**Last Updated**: 2025-12-26
**Next Review**: After v0.1.0 release
