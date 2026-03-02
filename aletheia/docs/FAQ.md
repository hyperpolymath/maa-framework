# Frequently Asked Questions (FAQ)

## General Questions

### What is Aletheia?

Aletheia (Greek: ἀλήθεια - "truth", "disclosure", "unconcealment") is a zero-dependency Rust tool for verifying Rhodium Standard Repository (RSR) compliance. It checks repositories against rigorous standards for security, documentation, and operational excellence.

### What is RSR?

RSR (Rhodium Standard Repository) is a framework defining graduated standards for software repositories. It covers type safety, memory safety, security, documentation, build systems, and testing. See [RSR-SPECIFICATION.md](RSR-SPECIFICATION.md) for details.

### Why "Aletheia"?

"Aletheia" is ancient Greek for "truth" or "unconcealment" - not merely factual correctness, but the revealing of reality. In software, this means transparent processes, verifiable standards, and honest documentation.

### Why "Rhodium"?

Rhodium is a rare, valuable, durable precious metal. Like rhodium, high-quality software repositories are rare, valuable, and built to last.

## Installation & Usage

### How do I install Aletheia?

```bash
# From source (recommended)
git clone https://gitlab.com/maa-framework/6-the-foundation/aletheia.git
cd aletheia
cargo build --release

# Or install directly
cargo install --git https://gitlab.com/maa-framework/6-the-foundation/aletheia.git
```

### How do I use Aletheia?

```bash
# Verify current directory
cargo run

# Verify specific repository
cargo run -- /path/to/repository
```

### What languages does Aletheia support?

Aletheia is language-agnostic. It checks documentation, structure, and build systems - not code itself. Works with:
- Rust
- Python
- JavaScript/TypeScript
- Go
- Any language with proper documentation

### Can I use Aletheia in CI/CD?

Yes! Aletheia exits with code 0 on success, 1 on failure:

```yaml
# GitLab CI
rsr-compliance:
  script:
    - aletheia
```

## Technical Questions

### Why zero dependencies?

Zero dependencies provides:
1. **Security**: No supply chain attacks
2. **Auditability**: Easy to review entire codebase
3. **Reliability**: No dependency breakage
4. **Simplicity**: Fewer moving parts
5. **Trust**: Users can verify everything

### Why no unsafe code?

No unsafe code means:
1. **Safety**: Rust's guarantees apply everywhere
2. **Trust**: No hidden memory bugs
3. **Simplicity**: No manual memory management
4. **Auditability**: No special cases to review

### Why offline-first?

Offline-first ensures:
1. **Privacy**: Cannot exfiltrate data
2. **Reliability**: Works without internet
3. **Speed**: No network latency
4. **Security**: No remote code execution
5. **Trust**: Users control all inputs

### How fast is Aletheia?

Very fast - typically <50ms for most repositories. Bottleneck is filesystem I/O (stat calls).

### How much memory does Aletheia use?

Minimal - typically <5MB RAM. Only stores check results in memory.

## Compliance Questions

### What compliance levels exist?

- **Bronze** (current): Foundation - docs, security, build system
- **Silver** (planned): Formal verification, advanced security
- **Gold** (planned): Multi-language, distributed systems
- **Platinum** (planned): Enterprise, CADRE integration

### What does Bronze compliance require?

Bronze requires:
- ✅ 7 documentation files (README, LICENSE, SECURITY, etc.)
- ✅ .well-known directory (security.txt, ai.txt, humans.txt)
- ✅ Build system (justfile, flake.nix, CI/CD)
- ✅ Source structure (src/, tests/)

See [RSR-SPECIFICATION.md](RSR-SPECIFICATION.md) for details.

### How do I make my repository compliant?

1. Run `aletheia` to see what's missing
2. Create missing files (see examples in Aletheia repo)
3. Re-run `aletheia` to verify
4. See [QUICK_START.md](QUICK_START.md) for step-by-step guide

### Can I skip some requirements?

No - all Bronze requirements are mandatory. However, you can:
- Use alternative build tools (alongside required ones)
- Add additional documentation
- Exceed minimum standards

### What if I disagree with a requirement?

The RSR specification is open for discussion:
- Open an issue on the specification repository
- Propose changes via merge request
- Join community discussions

## Project Questions

### Who maintains Aletheia?

Aletheia is maintained by the MAA Framework team. See [MAINTAINERS.md](../MAINTAINERS.md) for details.

### How can I contribute?

See [CONTRIBUTING.md](../CONTRIBUTING.md) for contribution guidelines. We welcome:
- Bug reports
- Feature requests
- Documentation improvements
- Code contributions

### What license is Aletheia under?

Dual-licensed under your choice of:
- MIT License
- Palimpsest License v0.8

See [LICENSE.txt](../LICENSE.txt) for details.

### Is Aletheia production-ready?

Yes, for Bronze-level verification. It's:
- Well-tested (100% test pass rate)
- Zero unsafe code
- Zero dependencies
- Self-verified (Aletheia verifies itself)

Silver/Gold/Platinum levels are planned for future releases.

## Security Questions

### How do I report a vulnerability?

See [SECURITY.md](../SECURITY.md) for:
- Security contact email
- Confidential issue reporting
- Response timeline
- Disclosure process

### Has Aletheia been audited?

Not yet. We welcome security audits! The codebase is:
- ~300 lines of Rust
- Zero unsafe code
- Zero dependencies
- Easy to audit

### What's the threat model?

**In Scope**:
- Malicious repository contents
- Path traversal attacks
- Resource exhaustion

**Out of Scope**:
- Physical access
- Compromised OS/kernel
- Side-channel attacks

See [SECURITY.md](../SECURITY.md) for full threat model.

### Can Aletheia be tricked?

Known limitations:
1. Follows symbolic links (potential path traversal)
2. Could consume memory on very large files
3. TOCTOU races between checks and file reads

These are documented for transparency but not considered security vulnerabilities in the current threat model.

## Development Questions

### Can I use Aletheia as a library?

Currently, Aletheia is binary-only. Library API may be added in future versions.

### How do I run tests?

```bash
cargo test              # Run all tests
cargo test -- --nocapture  # See output
cargo test --release    # Test optimized build
```

### How do I build for different platforms?

```bash
# Linux (glibc)
cargo build --release --target x86_64-unknown-linux-gnu

# Linux (musl - static binary)
cargo build --release --target x86_64-unknown-linux-musl

# macOS
cargo build --release --target x86_64-apple-darwin

# Windows
cargo build --release --target x86_64-pc-windows-gnu
```

### How do I add a new check?

See [CLAUDE.md](../CLAUDE.md) for development guide. Basic steps:

1. Add check function
2. Call from `verify_repository()`
3. Add tests
4. Update documentation

## Philosophy Questions

### What is the TPCF?

TPCF (Tri-Perimeter Contribution Framework) is a graduated trust model:
- **Perimeter 1**: Core maintainers (full access)
- **Perimeter 2**: Trusted contributors (review/merge rights)
- **Perimeter 3**: Community sandbox (open contribution)

See [CODE_OF_CONDUCT.md](../CODE_OF_CONDUCT.md) for details.

### What is the Palimpsest License?

The Palimpsest License v0.8 is a permissive license embodying principles of:
- Reversibility (Git makes everything reversible)
- Iteration (constant improvement)
- Impermanence (nothing is final)
- History preservation (layers remain visible)

See [LICENSE-PALIMPSEST.txt](../LICENSE-PALIMPSEST.txt) for full text.

### Why focus on emotional safety?

We believe healthy open source requires:
- Psychological safety to experiment
- Safe to fail (mistakes are learning)
- Safe to dissent (respectful disagreement)
- Reduced anxiety (reversibility helps)

See [CODE_OF_CONDUCT.md](../CODE_OF_CONDUCT.md) for our emotional safety framework.

## Roadmap Questions

### What's next for Aletheia?

Planned features:
- **v0.2.0**: Silver-level compliance checks
- **v0.3.0**: Gold-level compliance checks
- **v0.4.0**: JSON output format
- **v0.5.0**: Configurable checks
- **v1.0.0**: Platinum-level compliance

### When will Silver/Gold/Platinum be ready?

No fixed timeline. We prioritize quality over speed. Follow:
- [CHANGELOG.md](../CHANGELOG.md) for updates
- [Issues](https://gitlab.com/maa-framework/6-the-foundation/aletheia/-/issues) for progress

### Can I request features?

Yes! Open an issue:
- Describe the feature
- Explain the use case
- Suggest implementation (optional)

## Troubleshooting

### Aletheia won't compile

Check:
```bash
rustc --version  # Should be 1.75+
cargo --version
```

Update Rust:
```bash
rustup update
```

### Tests fail

Run with verbose output:
```bash
cargo test -- --nocapture
```

Check for:
- Filesystem permissions
- Correct working directory
- Updated Rust version

### Self-verification fails

This shouldn't happen! If it does:
1. Check which files are missing
2. Verify you're in Aletheia directory
3. Open an issue (this is a bug)

### CI/CD integration doesn't work

Check:
- Aletheia installed correctly
- Running in repository root
- Exit code checked (0 = success, 1 = failure)

## Still Have Questions?

- **Documentation**: Browse [docs/](.) directory
- **Issues**: https://gitlab.com/maa-framework/6-the-foundation/aletheia/-/issues
- **Email**: maintainers@maa-framework.org

---

*"The unexamined repository is not worth maintaining." - Adapted from Socrates*
