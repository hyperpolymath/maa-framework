# Aletheia Quick Reference Card

One-page reference for Aletheia RSR compliance verification.

## 🚀 Installation

```bash
# Quick install
curl -sSf https://gitlab.com/.../install.sh | bash

# Or build from source
git clone https://gitlab.com/maa-framework/6-the-foundation/aletheia.git
cd aletheia && cargo build --release

# Or use Cargo
cargo install --git https://gitlab.com/maa-framework/6-the-foundation/aletheia.git
```

## 💻 Usage

```bash
# Verify current directory
aletheia
# or
cargo run

# Verify specific repository
aletheia /path/to/repo
# or
cargo run -- /path/to/repo

# In Docker
docker run -v $(pwd):/repo aletheia:0.1.0
```

## ✅ RSR Bronze Requirements Checklist

### Documentation (7 files)
- [ ] `README.md` - Project overview
- [ ] `LICENSE.txt` - Open source license
- [ ] `SECURITY.md` - Security policy
- [ ] `CONTRIBUTING.md` - Contribution guide
- [ ] `CODE_OF_CONDUCT.md` - Community standards
- [ ] `MAINTAINERS.md` - Governance
- [ ] `CHANGELOG.md` - Version history

### .well-known (3 files)
- [ ] `.well-known/security.txt` - RFC 9116 security contact
- [ ] `.well-known/ai.txt` - AI policy
- [ ] `.well-known/humans.txt` - Attribution

### Build System (3 files)
- [ ] `justfile` - Build automation
- [ ] `flake.nix` - Nix reproducible builds
- [ ] `.gitlab-ci.yml` or `.github/workflows/` - CI/CD

### Source Structure (2 directories)
- [ ] `src/` - Source code
- [ ] `tests/` or `test/` - Tests

## 📊 Understanding Output

```
🔍 Aletheia - RSR Compliance Verification Report
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Repository: /path/to/repo

📋 Documentation
  ✅ README.md [Bronze]        # Passes
  ❌ LICENSE.txt [Bronze]      # Fails

Score: 8/16 checks passed (50.0%)
⚠️  Bronze-level RSR compliance: NOT MET  # Exit code 1
```

**Exit Codes**:
- `0` - All Bronze checks pass (compliant)
- `1` - One or more Bronze checks fail (not compliant)

## 🛠️ Quick Fixes

### Missing Documentation

```bash
# Create all required docs at once
touch README.md LICENSE.txt SECURITY.md \
      CONTRIBUTING.md CODE_OF_CONDUCT.md \
      MAINTAINERS.md CHANGELOG.md
```

### Missing .well-known

```bash
mkdir -p .well-known
cat > .well-known/security.txt << 'EOF'
Contact: mailto:security@example.org
Expires: 2026-12-31T23:59:59.000Z
EOF

touch .well-known/ai.txt .well-known/humans.txt
```

### Missing Build Files

```bash
# Create justfile
cat > justfile << 'EOF'
build:
    cargo build
test:
    cargo test
EOF

# Create basic flake.nix
echo '{}' > flake.nix

# Create basic CI
touch .gitlab-ci.yml  # or .github/workflows/ci.yml
```

### Missing Source Structure

```bash
mkdir -p src tests
mv *.rs src/  # Move source files
```

## 🔧 Build Automation (justfile)

```bash
just                 # List all commands
just build          # Build project
just test           # Run tests
just check          # Run all checks
just validate       # Verify RSR compliance
just clean          # Clean build artifacts
```

## 🐳 Docker Commands

```bash
# Build image
docker build -t aletheia:0.1.0 .

# Verify current directory
docker run -v $(pwd):/repo aletheia:0.1.0

# Verify specific path
docker run -v /path/to/repo:/repo aletheia:0.1.0
```

## 🔄 CI/CD Integration

### GitLab CI
```yaml
rsr-compliance:
  script:
    - cargo install --git https://gitlab.com/.../aletheia.git
    - aletheia
```

### GitHub Actions
```yaml
- run: cargo install --git https://gitlab.com/.../aletheia.git
- run: aletheia
```

## 📝 Quick Templates

### Minimal README.md
```markdown
# Project Name

Description

## Usage
\`\`\`bash
command
\`\`\`

## License
See [LICENSE.txt](LICENSE.txt)
```

### Minimal SECURITY.md
```markdown
# Security Policy

Report vulnerabilities to: security@example.org

See [.well-known/security.txt](.well-known/security.txt)
```

### Minimal justfile
```makefile
build:
    cargo build

test:
    cargo test
```

## 🆘 Troubleshooting

| Problem | Solution |
|---------|----------|
| Path not found | Use absolute path: `aletheia /full/path` |
| Permission denied | Check file permissions: `ls -la` |
| Wrong directory | Run from repo root |
| Missing tool | Install Rust: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs \| sh` |

## 📚 More Information

- **Full Docs**: See `docs/` directory
- **Quick Start**: `docs/QUICK_START.md`
- **FAQ**: `docs/FAQ.md`
- **Migration**: `docs/MIGRATION-GUIDE.md`
- **Architecture**: `docs/ARCHITECTURE.md`

## 🔗 Links

- **Repository**: https://gitlab.com/maa-framework/6-the-foundation/aletheia
- **Issues**: https://gitlab.com/.../aletheia/-/issues
- **Security**: See `SECURITY.md`

## 📞 Contact

- **Email**: maintainers@maa-framework.org
- **Security**: security@maa-framework.org

---

**Version**: 0.1.0 | **RSR Level**: Bronze | **License**: MIT OR Palimpsest-0.8
