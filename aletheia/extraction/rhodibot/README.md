# Rhodibot

[![RSR Bronze](https://img.shields.io/badge/RSR-Bronze-cd7f32)](https://github.com/hyperpolymath/rhodium-standard-repositories)

> **Rho**dium Stan**d**ard Repos**i**tory **Bot**

Rhodibot is like Dependabot, but for repository standards compliance instead of dependency updates. It acts as a consultant, regulator, advisor, and policy manager for RSR (Rhodium Standard Repository) compliance.

## What Rhodibot Does

- **Checks** repositories for RSR compliance (Bronze, Silver, Gold, Platinum levels)
- **Reports** issues via CI/CD annotations (GitHub Actions, GitLab CI)
- **Generates** badges and conformity documents
- **Advises** on compliance gaps and how to fix them

## Ecosystem Architecture

```
                    ┌─────────────────────────────────────┐
                    │   rhodium-standard-repositories     │
                    │      (The RSR Specification)        │
                    └─────────────────┬───────────────────┘
                                      │ defines
           ┌──────────────────────────┼──────────────────────────┐
           │                          │                          │
           ▼                          ▼                          ▼
┌─────────────────────┐   ╔═══════════════════════╗   ┌─────────────────────┐
│  rsr-template-repo  │   ║      RHODIBOT         ║   │  rhodium-pipeline   │
│   (Project Start)   │   ║  ==================   ║   │   (CI/CD Tools)     │
│                     │   ║  Consultant/Advisor   ║   │                     │
│  • Template files   │◀──║  Regulator/Policy     ║──▶│  • Pipeline gen     │
│  • Initial setup    │   ║  Compliance checker   ║   │  • Stage configs    │
│  • Scaffolding      │   ╚═══════════════════════╝   │  • Platform support │
└─────────────────────┘               │               └─────────────────────┘
                                      │ reports to
                                      ▼
                    ┌─────────────────────────────────────┐
                    │         repo-customiser             │
                    │    (C4 Control Point - Generic)     │
                    │                                     │
                    │  Works with ANY .scm standard:      │
                    │  • RSR (Rhodium Standard)           │
                    │  • oikos (Economic/Ecological)      │
                    │  • Custom organization standards    │
                    └─────────────────────────────────────┘
```

**Satellite of:** [rhodium-standard-repositories](https://github.com/hyperpolymath/rhodium-standard-repositories)

Rhodibot sits at the center of RSR compliance enforcement, connecting templates, pipelines, and the standard. It can be invoked by CI/CD systems, used interactively, or integrated via the repo-customiser C4 control point.

## Quick Start

```bash
# Install
cargo install rhodibot

# Check current directory
rhodibot check .

# Generate badge markdown
rhodibot badge

# Generate conformity document
rhodibot conformity
```

## CI/CD Integration

### GitHub Actions

```yaml
name: RSR Compliance
on: [push, pull_request]

jobs:
  rhodibot:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: hyperpolymath/rhodibot@v1
        with:
          path: '.'
          fail-on-warning: true
```

### GitLab CI

```yaml
rhodibot:
  stage: test
  image: hyperpolymath/rhodibot:latest
  script:
    - rhodibot check .
```

## CLI Usage

```
rhodibot [COMMAND] [OPTIONS] [PATH]

COMMANDS:
    check       Check RSR compliance (default)
    badge       Generate RSR badge markdown
    conformity  Generate RSR conformity document

OPTIONS:
    -f, --format <FORMAT>    Output format: human, json
    -q, --quiet              Quiet mode: only show pass/fail
    -v, --verbose            Verbose mode: show all details
    -h, --help               Print help information

EXIT CODES:
    0    Success - Bronze compliance achieved
    1    Failure - Bronze compliance not met
    2    Security - Critical security warnings detected
    3    Error - Invalid path provided
    4    Error - Invalid arguments
```

## Design Principles

1. **Zero Dependencies** - Only Rust standard library (RSR Bronze compliant)
2. **Zero Unsafe Code** - 100% safe Rust
3. **Offline-First** - Works without network access
4. **Security-Aware** - Detects symlink attacks, validates paths
5. **CI/CD Native** - First-class GitHub Actions and GitLab CI support

## Bronze Compliance Checks

Rhodibot checks for these Bronze-level requirements:

### Documentation
- README.md (or README.adoc)
- LICENSE.txt
- SECURITY.md
- CONTRIBUTING.md
- CODE_OF_CONDUCT.md
- MAINTAINERS.md
- CHANGELOG.md

### .well-known Directory
- .well-known/security.txt (RFC 9116)
- .well-known/ai.txt
- .well-known/humans.txt

### Build System
- justfile
- flake.nix
- .gitlab-ci.yml

### Source Structure
- src/ directory
- tests/ directory

## Security

Rhodibot includes security features:

- **Symlink Detection**: Warns about symlinks that point outside the repository
- **Path Validation**: Prevents directory traversal attacks
- **No Network**: Cannot exfiltrate data (offline-first design)
- **Zero Dependencies**: No supply chain attack surface

## License

MIT OR Apache-2.0

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

---

*Part of the [Rhodium Standard](https://github.com/hyperpolymath/rhodium-standard-repositories) ecosystem*
