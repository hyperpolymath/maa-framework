# Aletheia Architecture

## Overview

Aletheia is designed as a **simple, auditable, single-file tool** for RSR compliance verification. This document explains the architectural decisions and design principles.

## Design Principles

### 1. Simplicity Over Complexity

**Decision**: Single-file implementation (~300 lines)

**Rationale**:
- Easy to audit - one file to review
- Minimal cognitive overhead
- Clear code flow without jumping between modules
- Reduces attack surface

**Trade-offs**:
- Limited to ~1000 lines before needing modularization
- All code must be general-purpose (no domain-specific modules)

### 2. Zero Dependencies

**Decision**: Only use Rust standard library

**Rationale**:
- No supply chain attacks
- Easy security review
- Faster compilation
- Long-term stability
- Works offline

**Trade-offs**:
- Must implement features from scratch
- Cannot use ecosystem tools (clap, serde, etc.)
- More code to maintain

### 3. Type Safety First

**Decision**: Leverage Rust's type system maximally

**Rationale**:
- Compile-time correctness guarantees
- Self-documenting code
- Prevents entire classes of bugs
- No runtime type errors

**Implementation**:
```rust
enum ComplianceLevel {
    Bronze,
    Silver,
    Gold,
    Platinum,
}

struct CheckResult {
    category: String,
    item: String,
    passed: bool,
    required_for: ComplianceLevel,
}
```

### 4. Offline-First

**Decision**: No network access whatsoever

**Rationale**:
- Works air-gapped
- Cannot exfiltrate data
- No privacy concerns
- Faster execution
- More reliable

**Implementation**:
- All checks are filesystem-based
- No `std::net` usage
- No external API calls

### 5. Explicit Error Handling

**Decision**: No panics, all errors handled explicitly

**Rationale**:
- Predictable behavior
- Graceful degradation
- Clear error messages
- No crashes on invalid input

**Implementation**:
```rust
let repo_path = if args.len() > 1 {
    PathBuf::from(&args[1])
} else {
    std::env::current_dir().unwrap_or_else(|_| {
        eprintln!("Error: Cannot determine current directory");
        process::exit(1);
    })
};
```

## Code Structure

### Module Organization

```
src/main.rs
├── Type Definitions
│   ├── ComplianceLevel (enum)
│   ├── CheckResult (struct)
│   └── ComplianceReport (struct)
│
├── Core Verification Functions
│   ├── file_exists()
│   ├── dir_exists()
│   ├── check_documentation()
│   ├── check_well_known()
│   ├── check_build_system()
│   ├── check_source_structure()
│   └── verify_repository()
│
├── Reporting
│   └── print_report()
│
├── Entry Point
│   └── main()
│
└── Tests
    └── #[cfg(test)] mod tests
```

### Data Flow

```
Command Line Args
      ↓
  Parse Path
      ↓
Verify Repository
      ↓
  ┌─────────────┐
  │ Create      │
  │ Report      │
  └─────────────┘
      ↓
  ┌─────────────┐
  │ Check       │
  │ Documentation │
  └─────────────┘
      ↓
  ┌─────────────┐
  │ Check       │
  │ Well-Known  │
  └─────────────┘
      ↓
  ┌─────────────┐
  │ Check       │
  │ Build System│
  └─────────────┘
      ↓
  ┌─────────────┐
  │ Check       │
  │ Source      │
  └─────────────┘
      ↓
  Print Report
      ↓
  Exit Code
```

## Key Components

### ComplianceReport

**Purpose**: Accumulate all check results

**Design**:
```rust
struct ComplianceReport {
    checks: Vec<CheckResult>,
    repository_path: PathBuf,
}
```

**Methods**:
- `new()` - Create empty report
- `add_check()` - Add a check result
- `bronze_compliance()` - Check if all Bronze requirements pass
- `passed_count()` - Count passing checks
- `total_count()` - Count total checks

### Check Functions

**Pattern**: All check functions follow the same signature
```rust
fn check_category(report: &mut ComplianceReport, repo_path: &Path) {
    // Perform checks
    // Add results to report
}
```

**Benefits**:
- Consistent interface
- Easy to add new checks
- Clear separation of concerns

### Verification Flow

**Strategy**: Sequential checks with accumulation

1. Create empty report
2. Run each check function
3. Each function adds results to report
4. Return completed report

**Benefits**:
- All checks run (don't stop on first failure)
- Complete picture of compliance status
- Easy to parallelize in future (if needed)

## Security Architecture

### Threat Model

**In Scope**:
- Malicious repository contents
- Path traversal attacks
- Resource exhaustion
- Information disclosure

**Out of Scope**:
- Physical access attacks
- Compromised OS/kernel
- Side-channel attacks
- Social engineering

### Security Measures

1. **Input Validation**
   ```rust
   if !repo_path.exists() {
       eprintln!("Error: Path does not exist");
       process::exit(1);
   }
   ```

2. **No Unsafe Code**
   - Zero `unsafe` blocks
   - All code memory-safe by construction

3. **No Dependencies**
   - No supply chain vulnerabilities
   - No transitive dependencies

4. **Offline Operation**
   - Cannot exfiltrate data
   - No network attack surface

### Known Limitations

1. **Symbolic Links**: Follows symlinks (potential path traversal)
2. **Large Files**: Could consume excessive memory
3. **TOCTOU**: Time-of-check-time-of-use races between existence checks and reads

## Performance Characteristics

### Time Complexity

- **File Checks**: O(n) where n = number of required files
- **Directory Checks**: O(1) per directory
- **Overall**: O(n) where n = total checks (~20 checks)

### Space Complexity

- **Report Storage**: O(n) where n = number of checks
- **Stack Usage**: Minimal (no recursion)
- **Heap Usage**: Small (only storing check results)

### Typical Performance

- **Small repos**: <10ms
- **Large repos**: <50ms
- **Bottleneck**: Filesystem I/O (stat calls)

## Extensibility

### Adding New Checks

1. Create check function:
   ```rust
   fn check_new_category(report: &mut ComplianceReport, repo_path: &Path) {
       let passes = /* check logic */;
       report.add_check("Category", "Item", passes, ComplianceLevel::Bronze);
   }
   ```

2. Call from `verify_repository()`:
   ```rust
   check_new_category(&mut report, repo_path);
   ```

3. Add tests:
   ```rust
   #[test]
   fn test_new_category_check() { /* ... */ }
   ```

### Supporting New Compliance Levels

Currently: Bronze only

**To add Silver/Gold/Platinum**:

1. Add checks with appropriate `ComplianceLevel` enum value
2. Add level-specific verification methods to `ComplianceReport`
3. Update output to show multi-level status

### Future Modularization (>1000 lines)

**Suggested structure**:
```
src/
├── main.rs           # Entry point
├── types.rs          # Type definitions
├── checks/
│   ├── mod.rs        # Check functions
│   ├── docs.rs       # Documentation checks
│   ├── security.rs   # Security checks
│   └── build.rs      # Build system checks
└── report.rs         # Reporting
```

## Testing Strategy

### Unit Tests

Test individual functions:
```rust
#[test]
fn test_file_exists() {
    // Test file existence checking
}
```

### Integration Tests (Future)

Test complete workflows:
```rust
#[test]
fn test_complete_verification() {
    // Test end-to-end verification
}
```

### Manual Testing

Test on real repositories:
- RSR-compliant repos (should pass)
- Non-compliant repos (should fail)
- Edge cases (empty dirs, symlinks, etc.)

## Build System

### Justfile

**Purpose**: Common development tasks

**Key Recipes**:
- `build` - Compile project
- `test` - Run tests
- `check` - Run all quality checks
- `validate` - Self-verification

### Nix Flake

**Purpose**: Reproducible builds

**Benefits**:
- Exact dependency versions
- Cross-platform consistency
- Isolated build environment

### GitLab CI

**Purpose**: Automated testing

**Stages**:
1. Check (format, clippy, dependencies, unsafe code)
2. Test (unit, release, doc tests)
3. Build (debug, release, musl)
4. Verify (RSR compliance, docs, build system)
5. Deploy (releases, pages)

## Maintenance Philosophy

### Stability Over Features

- Keep core simple
- Avoid feature creep
- Maintain zero dependencies
- Preserve auditability

### Backwards Compatibility

- Follow semantic versioning
- Maintain API stability
- Document breaking changes

### Evolution Strategy

1. **Bronze** (current): Foundation
2. **Silver**: Add verification without breaking Bronze
3. **Gold**: Multi-language support
4. **Platinum**: Enterprise features

## References

- [RSR Specification](RSR-SPECIFICATION.md)
- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- [Clean Architecture](https://blog.cleancoder.com/uncle-bob/2012/08/13/the-clean-architecture.html)

---

**Last Updated**: 2025-11-22
**Version**: 1.0
