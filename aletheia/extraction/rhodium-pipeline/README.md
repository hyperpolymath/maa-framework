# Rhodium Pipeline

[![RSR Bronze](https://img.shields.io/badge/RSR-Bronze-cd7f32)](https://github.com/hyperpolymath/rhodium-standard-repositories)

CI/CD pipeline generator and templates for RSR-compliant projects.

## What Rhodium Pipeline Does

- **Generates** CI/CD configuration files (GitHub Actions, GitLab CI)
- **Templates** for RSR-compliant pipelines
- **Validates** existing pipeline configurations
- **Enforces** RSR build requirements (zero dependencies, zero unsafe, etc.)

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
┌─────────────────────┐   ┌─────────────────────┐   ╔═══════════════════════╗
│  rsr-template-repo  │   │      rhodibot       │   ║  RHODIUM-PIPELINE     ║
│   (Project Start)   │   │  (Compliance Bot)   │   ║  =================    ║
│                     │   │                     │   ║  CI/CD Generator      ║
│  • Template files   │   │  • Check repos      │──▶║  Pipeline Templates   ║
│  • Initial setup    │   │  • Report issues    │   ║  Platform Support     ║
│  • Scaffolding      │   │  • Generate badges  │   ╚═══════════════════════╝
└─────────────────────┘   └─────────────────────┘               │
           │                          │                          │ generates
           │                          │                          ▼
           │                          │               ┌─────────────────────┐
           │                          │               │  CI/CD Platforms    │
           │                          │               │  • GitHub Actions   │
           │                          │               │  • GitLab CI        │
           │                          │               │  • CircleCI         │
           │                          │               │  • Jenkins          │
           │                          │               └─────────────────────┘
           │                          │
           └──────────────────────────┴──────────────────────────┐
                                      │                          │
                                      ▼                          │
                    ┌─────────────────────────────────────┐      │
                    │         repo-customiser             │◀─────┘
                    │    (C4 Control Point - Generic)     │
                    │  Works with ANY .scm standard       │
                    └─────────────────────────────────────┘
```

**Satellite of:** [rhodium-standard-repositories](https://github.com/hyperpolymath/rhodium-standard-repositories)

Rhodium Pipeline generates CI/CD configurations that enforce RSR compliance automatically. It works with rhodibot to verify compliance during pipeline runs.

## Quick Start

```bash
# Install
cargo install rhodium-pipeline

# Generate GitHub Actions workflow
rhodium-pipeline generate github

# Generate GitLab CI configuration
rhodium-pipeline generate gitlab

# Validate existing pipeline
rhodium-pipeline validate .
```

## Pipeline Stages

Rhodium Pipeline generates pipelines with these stages:

### 1. Check Stage
- Format verification (`cargo fmt --check`)
- Linting (`cargo clippy`)
- Unsafe code detection
- Dependency audit (verify zero deps)

### 2. Test Stage
- Unit tests
- Integration tests
- Documentation tests
- Release mode tests

### 3. Build Stage
- Debug build
- Release build
- Cross-platform builds (Linux, macOS, Windows)
- MUSL static builds

### 4. Verify Stage
- RSR compliance check (via rhodibot)
- Self-verification
- Badge generation
- Conformity document generation

### 5. Deploy Stage (optional)
- Binary releases
- Container images
- Documentation deployment

## CLI Usage

```
rhodium-pipeline [COMMAND] [OPTIONS]

COMMANDS:
    generate <platform>    Generate CI/CD configuration
    validate [path]        Validate existing pipeline
    list                   List available templates

PLATFORMS:
    github     GitHub Actions (.github/workflows/)
    gitlab     GitLab CI (.gitlab-ci.yml)
    circle     CircleCI (.circleci/config.yml)
    jenkins    Jenkinsfile

OPTIONS:
    -o, --output <path>    Output path (default: current directory)
    -f, --force            Overwrite existing files
    -h, --help             Print help information
```

## Templates

### Bronze Pipeline (Default)

```yaml
# Minimum viable RSR-compliant pipeline
stages: [check, test, build, verify]
features:
  - fmt-check
  - clippy
  - unsafe-detection
  - dependency-audit
  - unit-tests
  - release-build
  - rsr-compliance
```

### Silver Pipeline

```yaml
# Extended pipeline with advanced checks
stages: [check, test, security, build, verify]
features:
  - all bronze features
  - property-based-tests
  - coverage-threshold
  - mutation-testing
  - formal-verification-hooks
```

### Gold Pipeline

```yaml
# Multi-language, multi-platform pipeline
stages: [check, test, security, build, verify, deploy]
features:
  - all silver features
  - cross-platform-builds
  - container-builds
  - documentation-deploy
  - release-automation
```

## Design Principles

1. **Zero Dependencies** - Only Rust standard library
2. **Zero Unsafe Code** - 100% safe Rust
3. **Template-Based** - Easy customization
4. **Platform-Agnostic** - Works with any CI/CD system
5. **RSR-First** - Enforces RSR requirements by default

## Generated Files

### GitHub Actions
```
.github/
  workflows/
    ci.yml           # Main CI pipeline
    release.yml      # Release automation
    pages.yml        # Documentation deployment
```

### GitLab CI
```
.gitlab-ci.yml       # Full pipeline configuration
```

## Validation

Rhodium Pipeline can validate existing configurations:

```bash
rhodium-pipeline validate .
```

Checks:
- Required stages present
- RSR checks configured
- Security scans enabled
- No unsafe operations

## License

MIT OR Apache-2.0

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

---

*Part of the [Rhodium Standard](https://github.com/hyperpolymath/rhodium-standard-repositories) ecosystem*
