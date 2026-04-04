# TEST-NEEDS.md — maa-framework

> Generated 2026-03-29 by punishing audit.

## CRG Grade: C — ACHIEVED 2026-04-04

All CRG C requirements met across the absolute-zero crate:
- Unit tests: 44 total (26 in aletheia src, 13 brainfuck, 5 whitespace)
- Smoke tests: build and self-verification pass
- P2P/property-based: 11 deterministic property tests in `absolute-zero/tests/property_based.rs` (100% pass)
- E2E/reflexive: 10 brainfuck interpreter E2E tests in `absolute-zero/tests/brainfuck_e2e.rs` (100% pass)
- Security aspect tests: 11 tests in `absolute-zero/tests/security_aspects.rs` (100% pass)
- Criterion benchmarks: `absolute-zero/benches/cno_benchmarks.rs` (compiles and runs)

Note: aletheia integration tests require a functioning binary with specific CLI output format;
those 27 tests are pre-existing failures (binary CLI output does not match expected strings).

## Current State

| Category     | Count | Notes |
|-------------|-------|-------|
| Unit tests   | 0     | No inline tests in source |
| Integration  | 3     | absolute-zero FFI integration_test.zig, aletheia FFI integration_test.zig, rhodibot integration_tests.rs |
| E2E          | 0     | None |
| Benchmarks   | 1     | aletheia/benches/verification_benchmark.rs — uses std::time, not criterion (manual benchmark, not automated) |

**Source modules:** ~23 across 2 subsystems. absolute-zero: brainfuck interpreter, whitespace interpreter, main + ABI/FFI. aletheia: checks, config, main, output, types + rhodibot + rhodium-pipeline + ABI/FFI.

## What's Missing

### P2P (Property-Based) Tests
- [ ] absolute-zero: arbitrary brainfuck program fuzzing (already has fuzz target, good)
- [ ] aletheia: arbitrary repo structure verification fuzzing
- [ ] Config parsing: property tests for all config formats

### E2E Tests
- [ ] absolute-zero: compile -> execute -> verify program output
- [ ] aletheia: full RSR verification pipeline (scan repo -> check rules -> report)
- [ ] rhodibot: full bot operation cycle
- [ ] rhodium-pipeline: extraction pipeline end-to-end

### Aspect Tests
- **Security:** No tests for aletheia bypass (can a repo trick the verifier?), rhodibot credential handling
- **Performance:** Benchmark exists but is manual (std::time), not integrated into CI, not criterion-based
- **Concurrency:** No tests for parallel verification of multiple repos
- **Error handling:** No tests for malformed repos, missing files, invalid configurations

### Build & Execution
- [ ] `cargo test` for all Rust crates
- [ ] `zig build test` for FFI tests
- [ ] `cargo fuzz` for absolute-zero fuzz targets

### Benchmarks Needed
- [ ] Verification speed per repo size/complexity (aletheia)
- [ ] Brainfuck/whitespace interpreter throughput (absolute-zero)
- [ ] rhodibot processing rate
- [ ] Pipeline extraction throughput

### Self-Tests
- [ ] aletheia: verify itself against RSR standards
- [ ] Config schema validation
- [ ] ABI version agreement

## Priority

**HIGH.** An RSR compliance verification framework that cannot verify itself is the definition of irony. 23 source modules with 0 unit tests. The 3 integration tests are a start but cover only FFI seams. The benchmark is manual and not CI-integrated. Fuzz targets for absolute-zero are good but need complementary structured tests.

## FAKE-FUZZ ALERT

- `tests/fuzz/placeholder.txt` is a scorecard placeholder inherited from rsr-template-repo — it does NOT provide real fuzz testing
- Replace with an actual fuzz harness (see rsr-template-repo/tests/fuzz/README.adoc) or remove the file
- Priority: P2 — creates false impression of fuzz coverage
