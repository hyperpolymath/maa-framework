# Security Policy

## 🔒 Security Principles

Aletheia is built with security as a foundational principle:

1. **Zero Dependencies**: No external crates = minimal supply chain attack surface
2. **No Unsafe Code**: Zero `unsafe` blocks in the entire codebase
3. **Offline-First**: No network access = cannot exfiltrate data
4. **Type Safety**: Rust's ownership model prevents memory safety vulnerabilities
5. **Minimal Attack Surface**: ~300 lines of auditable code

## 🛡️ Supported Versions

| Version | Supported          |
| ------- | ------------------ |
| 0.1.x   | :white_check_mark: |

## 🚨 Reporting a Vulnerability

We take security seriously. If you discover a security vulnerability, please follow these steps:

### 1. **DO NOT** create a public issue

Public disclosure of security vulnerabilities puts all users at risk. Please report privately.

### 2. Report via Secure Channels

Choose one of these methods:

- **Email**: security@maa-framework.org (PGP key available at `.well-known/security.txt`)
- **GitLab Confidential Issue**: [Create confidential issue](https://gitlab.com/maa-framework/6-the-foundation/aletheia/-/issues/new?issue[confidential]=true)

### 3. Include Detailed Information

Please provide:

- Description of the vulnerability
- Steps to reproduce
- Potential impact
- Suggested fix (if available)
- Your contact information (for follow-up)

### 4. Response Timeline

We aim to:

- **Acknowledge** receipt within 48 hours
- **Assess** severity within 7 days
- **Provide updates** every 7 days until resolution
- **Release fix** within 30 days for critical issues, 90 days for others

## 🏆 Security Hall of Fame

We recognize researchers who responsibly disclose vulnerabilities:

<!-- Add contributors here -->

*Be the first to help secure Aletheia!*

## 🔍 Security Features

### Memory Safety

Aletheia leverages Rust's ownership model to prevent:

- Buffer overflows
- Use-after-free
- Null pointer dereferences
- Data races
- Iterator invalidation

### Input Validation

All file paths and inputs are validated:

```rust
if !repo_path.exists() {
    eprintln!("Error: Path does not exist: {}", repo_path.display());
    process::exit(1);
}

if !repo_path.is_dir() {
    eprintln!("Error: Path is not a directory: {}", repo_path.display());
    process::exit(1);
}
```

### Error Handling

All operations use Rust's `Result` type for explicit error handling:

- No panics in production code
- Graceful degradation on errors
- Clear error messages

### Supply Chain Security

- **Zero dependencies**: No transitive dependency vulnerabilities
- **Cargo.lock committed**: Reproducible builds
- **Minimal build tools**: Only Rust toolchain required

## 🔐 Security Audit

### Last Audit: 2025-12-26

| Aspect | Result |
|--------|--------|
| **Auditor** | Automated security review |
| **Scope** | Full codebase (~586 lines) |
| **Vulnerabilities Found** | 0 |
| **Unsafe Code Blocks** | 0 |
| **Dependencies** | 0 |

**Audit Coverage**:
- Symlink escape vulnerabilities → Mitigated (detection + critical warnings)
- Path traversal attacks → Mitigated (canonicalization checks)
- Memory safety → Guaranteed (Rust ownership model)
- Supply chain attacks → N/A (zero dependencies)
- TOCTOU races → Mitigated (verification timestamps)

We welcome additional security audits! If you're interested in auditing Aletheia:

1. Review the source code in `src/main.rs` (~586 lines)
2. Check for:
   - Unsafe code blocks (should be zero)
   - Unvalidated inputs
   - Path traversal vulnerabilities
   - Integer overflows
   - Logic errors
3. Report findings via security@maa-framework.org

### Known Limitations

1. **Symbolic Links**: Aletheia detects symlinks and warns if they point outside the repository. Symlinks pointing outside the repository root generate CRITICAL warnings and cause verification to fail.
2. **Filesystem Races**: TOCTOU (time-of-check-time-of-use) gaps between existence checks and file reads. Mitigated by including verification timestamp in output.

These are documented for transparency. Aletheia does NOT read file contents - it only checks existence - so there is no memory exhaustion risk from large files.

## 🎯 Threat Model

### In Scope

- **Malicious repository contents**: Aletheia should safely analyze any repository
- **Path traversal attacks**: Cannot escape intended directory
- **Resource exhaustion**: Reasonable limits on memory/CPU usage
- **Information disclosure**: No sensitive data leaked

### Out of Scope

- **Physical access**: Attacker has physical access to the machine
- **Compromised OS/kernel**: Underlying system is already compromised
- **Side-channel attacks**: Timing attacks, cache attacks, etc.
- **Social engineering**: Tricking users into running malicious code

## 🔄 Security Updates

Security updates are released as soon as possible:

1. **Critical** (CVSS 9.0-10.0): Immediate patch release
2. **High** (CVSS 7.0-8.9): Patch within 7 days
3. **Medium** (CVSS 4.0-6.9): Patch within 30 days
4. **Low** (CVSS 0.1-3.9): Patch in next minor release

## 📜 Security.txt

This project complies with RFC 9116. See [.well-known/security.txt](.well-known/security.txt) for machine-readable security information.

## 🤝 Coordinated Disclosure

We follow coordinated disclosure principles:

1. Reporter notifies us privately
2. We acknowledge and investigate
3. We develop and test a fix
4. We coordinate public disclosure timing with reporter
5. We release patch and advisory simultaneously
6. We credit reporter (with permission)

## 📚 References

- [OWASP Top 10](https://owasp.org/www-project-top-ten/)
- [CWE Top 25](https://cwe.mitre.org/top25/)
- [RFC 9116 - security.txt](https://www.rfc-editor.org/rfc/rfc9116.html)
- [Rust Security Guidelines](https://anssi-fr.github.io/rust-guide/)

---

**Last Updated**: 2025-12-26
**Contact**: security@maa-framework.org
