// SPDX-License-Identifier: MPL-2.0
// Copyright (c) Jonathan D.A. Jewell <j.d.a.jewell@open.ac.uk>
//! Rhodibot CLI - RSR Compliance Bot
//!
//! A command-line tool for verifying Rhodium Standard Repository compliance.
//! Like Dependabot but for repository standards instead of dependencies.

use rhodibot::{
    exit_codes, format_timestamp, generate_badge, generate_conformity_doc, generate_sarif,
    json_escape, verify_repository, BotAction, ComplianceLevel, ComplianceReport, OutputFormat,
    Verbosity, WarningLevel, VERSION,
};
use std::fs;
use std::path::{Path, PathBuf};
use std::process;

/// CLI options
struct CliOptions {
    repo_path: PathBuf,
    format: OutputFormat,
    verbosity: Verbosity,
    action: BotAction,
}

/// Print help message
fn print_help() {
    println!(
        r#"Rhodibot - RSR Compliance Bot

Like Dependabot but for Rhodium Standard Repository compliance.

USAGE:
    rhodibot [COMMAND] [OPTIONS] [PATH]

COMMANDS:
    check       Check RSR compliance (default)
    badge       Generate RSR badge markdown
    conformity  Generate RSR conformity document

ARGS:
    [PATH]    Repository path to verify (default: current directory)

OPTIONS:
    -f, --format <FORMAT>    Output format: human, json (default: human)
    -q, --quiet              Quiet mode: only show pass/fail result
    -v, --verbose            Verbose mode: show all details
    -h, --help               Print help information
    -V, --version            Print version information

EXIT CODES:
    0    Success - Bronze compliance achieved
    1    Failure - Bronze compliance not met
    2    Security - Critical security warnings detected
    3    Error - Invalid path provided
    4    Error - Invalid arguments

EXAMPLES:
    rhodibot                         # Check current directory
    rhodibot check /path/to/repo     # Check specific repository
    rhodibot badge                   # Generate badge for current directory
    rhodibot conformity              # Generate conformity document
    rhodibot --format json           # Output as JSON

CI/CD INTEGRATION:
    # GitHub Actions
    - uses: hyperpolymath/rhodibot@v1
      with:
        path: '.'
        fail-on-warning: true

    # GitLab CI
    rhodibot:
      image: hyperpolymath/rhodibot:latest
      script:
        - rhodibot check .
"#
    );
}

/// Print version information
fn print_version() {
    println!("rhodibot {}", VERSION);
}

/// Parse command line arguments
fn parse_args() -> Result<CliOptions, String> {
    let args: Vec<String> = std::env::args().collect();
    let mut format = OutputFormat::Human;
    let mut verbosity = Verbosity::Normal;
    let mut repo_path: Option<PathBuf> = None;
    let mut action = BotAction::Check;

    let mut i = 1;
    while i < args.len() {
        let arg = &args[i];
        match arg.as_str() {
            "-h" | "--help" => {
                print_help();
                process::exit(exit_codes::SUCCESS);
            },
            "-V" | "--version" => {
                print_version();
                process::exit(exit_codes::SUCCESS);
            },
            "-q" | "--quiet" => {
                verbosity = Verbosity::Quiet;
            },
            "-v" | "--verbose" => {
                verbosity = Verbosity::Verbose;
            },
            "-f" | "--format" => {
                i += 1;
                if i >= args.len() {
                    return Err("--format requires an argument".to_string());
                }
                format = match args[i].as_str() {
                    "human" => OutputFormat::Human,
                    "json" => OutputFormat::Json,
                    other => {
                        return Err(format!("Unknown format: {}. Use 'human' or 'json'", other))
                    },
                };
            },
            "check" => action = BotAction::Check,
            "badge" => action = BotAction::Badge,
            "conformity" => action = BotAction::Conformity,
            "fix" => action = BotAction::Fix,
            arg if arg.starts_with('-') => {
                if let Some(value) = arg.strip_prefix("--format=") {
                    format = match value {
                        "human" => OutputFormat::Human,
                        "json" => OutputFormat::Json,
                        other => {
                            return Err(format!("Unknown format: {}. Use 'human' or 'json'", other))
                        },
                    };
                } else {
                    return Err(format!("Unknown option: {}", arg));
                }
            },
            path => {
                if repo_path.is_some() {
                    return Err("Multiple paths provided. Only one path is allowed.".to_string());
                }
                repo_path = Some(PathBuf::from(path));
            },
        }
        i += 1;
    }

    let repo_path =
        repo_path.unwrap_or_else(|| std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")));

    Ok(CliOptions {
        repo_path,
        format,
        verbosity,
        action,
    })
}

/// Print the compliance report (human format)
fn print_report(report: &ComplianceReport) {
    println!("🤖 Rhodibot - RSR Compliance Report");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("Repository: {}", report.repository_path.display());
    println!("Verified:   {}", format_timestamp(report.verified_at));
    println!();

    let mut current_category = String::new();
    for check in &report.checks {
        if check.category != current_category {
            println!("\n📋 {}", check.category);
            current_category = check.category.clone();
        }

        let icon = if check.passed { "✅" } else { "❌" };
        let level = format!("{:?}", check.required_for);
        println!("  {} {} [{}]", icon, check.item, level);
    }

    if !report.warnings.is_empty() {
        println!("\n🛡️  Security Warnings");
        for warning in &report.warnings {
            let icon = match warning.level {
                WarningLevel::Info => "ℹ️ ",
                WarningLevel::Warning => "⚠️ ",
                WarningLevel::Critical => "🚨",
            };
            println!("  {} {}", icon, warning.message);
        }
    }

    println!();
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!(
        "Score: {}/{} checks passed ({:.1}%)",
        report.passed_count(),
        report.total_count(),
        report.percentage()
    );

    if report.has_critical_warnings() {
        println!("🚨 CRITICAL: Security warnings detected - review required");
    }

    if report.bronze_compliance() && !report.has_critical_warnings() {
        println!("🏆 Bronze-level RSR compliance: ACHIEVED");
    } else if report.bronze_compliance() && report.has_critical_warnings() {
        println!("⚠️  Bronze-level RSR compliance: ACHIEVED (with warnings)");
    } else {
        println!("⚠️  Bronze-level RSR compliance: NOT MET");
    }
    println!();
}

/// Print report as JSON
fn print_json_report(report: &ComplianceReport) {
    let timestamp = format_timestamp(report.verified_at);
    let passed = report.passed_count();
    let total = report.total_count();
    let percentage = report.percentage();
    let bronze_compliant = report.bronze_compliance();
    let has_critical = report.has_critical_warnings();

    println!("{{");
    println!("  \"tool\": \"rhodibot\",");
    println!("  \"version\": \"{}\",", VERSION);
    println!(
        "  \"repository\": \"{}\",",
        json_escape(&report.repository_path.display().to_string())
    );
    println!("  \"verified_at\": \"{}\",", timestamp);
    println!("  \"score\": {{");
    println!("    \"passed\": {},", passed);
    println!("    \"total\": {},", total);
    println!("    \"percentage\": {:.1}", percentage);
    println!("  }},");
    println!("  \"bronze_compliant\": {},", bronze_compliant);
    println!("  \"has_critical_warnings\": {},", has_critical);

    println!("  \"checks\": [");
    for (i, check) in report.checks.iter().enumerate() {
        let comma = if i < report.checks.len() - 1 { "," } else { "" };
        println!("    {{");
        println!("      \"category\": \"{}\",", json_escape(&check.category));
        println!("      \"item\": \"{}\",", json_escape(&check.item));
        println!("      \"passed\": {},", check.passed);
        println!("      \"level\": \"{:?}\"", check.required_for);
        println!("    }}{}", comma);
    }
    println!("  ],");

    println!("  \"warnings\": [");
    for (i, warning) in report.warnings.iter().enumerate() {
        let comma = if i < report.warnings.len() - 1 {
            ","
        } else {
            ""
        };
        let level = match warning.level {
            WarningLevel::Info => "info",
            WarningLevel::Warning => "warning",
            WarningLevel::Critical => "critical",
        };
        println!("    {{");
        println!("      \"level\": \"{}\",", level);
        println!("      \"message\": \"{}\"", json_escape(&warning.message));
        println!("    }}{}", comma);
    }
    println!("  ]");
    println!("}}");
}

/// Print quiet mode output
fn print_quiet_report(report: &ComplianceReport) {
    let bronze_compliant = report.bronze_compliance();
    let has_critical = report.has_critical_warnings();

    if bronze_compliant && !has_critical {
        println!("PASS");
    } else if has_critical {
        println!("FAIL (security)");
    } else {
        println!("FAIL");
    }
}

/// Print verbose report
fn print_verbose_report(report: &ComplianceReport) {
    println!("🤖 Rhodibot - RSR Compliance Report (Verbose)");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("Repository: {}", report.repository_path.display());
    println!("Verified:   {}", format_timestamp(report.verified_at));
    println!("Version:    {}", VERSION);
    println!();

    let mut current_category = String::new();
    for check in &report.checks {
        if check.category != current_category {
            println!("\n📋 {}", check.category);
            current_category = check.category.clone();
        }

        let icon = if check.passed { "✅" } else { "❌" };
        let level = format!("{:?}", check.required_for);
        println!("  {} {} [{}]", icon, check.item, level);
    }

    if !report.warnings.is_empty() {
        println!("\n🛡️  Security Warnings ({} total)", report.warnings.len());
        for warning in &report.warnings {
            let icon = match warning.level {
                WarningLevel::Info => "ℹ️ ",
                WarningLevel::Warning => "⚠️ ",
                WarningLevel::Critical => "🚨",
            };
            let level_str = match warning.level {
                WarningLevel::Info => "[INFO]",
                WarningLevel::Warning => "[WARN]",
                WarningLevel::Critical => "[CRITICAL]",
            };
            println!("  {} {} {}", icon, level_str, warning.message);
            if let Some(ref path) = warning.path {
                println!("      Path: {}", path.display());
            }
        }
    }

    println!();
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!(
        "Score: {}/{} checks passed ({:.1}%)",
        report.passed_count(),
        report.total_count(),
        report.percentage()
    );

    if report.has_critical_warnings() {
        println!("🚨 CRITICAL: Security warnings detected - review required");
        println!(
            "   Exit code: {} (SECURITY_WARNING)",
            exit_codes::SECURITY_WARNING
        );
    }

    if report.bronze_compliance() && !report.has_critical_warnings() {
        println!("🏆 Bronze-level RSR compliance: ACHIEVED");
        println!("   Exit code: {} (SUCCESS)", exit_codes::SUCCESS);
    } else if report.bronze_compliance() && report.has_critical_warnings() {
        println!("⚠️  Bronze-level RSR compliance: ACHIEVED (with warnings)");
        println!(
            "   Exit code: {} (SECURITY_WARNING)",
            exit_codes::SECURITY_WARNING
        );
    } else {
        println!("⚠️  Bronze-level RSR compliance: NOT MET");
        println!(
            "   Exit code: {} (COMPLIANCE_FAILED)",
            exit_codes::COMPLIANCE_FAILED
        );
    }
    println!();
}

/// Apply fixes to create missing RSR-required files
fn apply_fixes(repo_path: &Path, report: &ComplianceReport) -> usize {
    let mut fixes_applied = 0;

    for check in &report.checks {
        if check.passed {
            continue;
        }

        // Generate appropriate content based on the missing item
        let (relative_path, content) = match (check.category.as_str(), check.item.as_str()) {
            ("Documentation", "README.md") => {
                let project_name = repo_path
                    .file_name()
                    .map(|n| n.to_string_lossy().to_string())
                    .unwrap_or_else(|| "Project".to_string());
                (
                    "README.md",
                    format!(
                        r#"# {}

## Overview

A brief description of this project.

## Installation

```bash
# Installation instructions here
```

## Usage

```bash
# Usage examples here
```

## License

See [LICENSE.txt](LICENSE.txt) for details.

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for contribution guidelines.

## Security

See [SECURITY.md](SECURITY.md) for security policy.
"#,
                        project_name
                    ),
                )
            }
            ("Documentation", "LICENSE.txt") => (
                "LICENSE.txt",
                r#"MIT License

Copyright (c) [year] [copyright holder]

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
"#
                .to_string(),
            ),
            ("Documentation", "SECURITY.md") => (
                "SECURITY.md",
                r#"# Security Policy

## Supported Versions

| Version | Supported          |
| ------- | ------------------ |
| latest  | :white_check_mark: |

## Reporting a Vulnerability

Please report security vulnerabilities by emailing security@example.org.

Do NOT open public issues for security vulnerabilities.

### What to Include

- Description of the vulnerability
- Steps to reproduce
- Potential impact
- Suggested fix (if any)

### Response Timeline

- Initial response: Within 48 hours
- Status update: Within 7 days
- Resolution target: Within 30 days (critical), 90 days (others)
"#
                .to_string(),
            ),
            ("Documentation", "CONTRIBUTING.md") => (
                "CONTRIBUTING.md",
                r#"# Contributing

Thank you for your interest in contributing!

## How to Contribute

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## Code Style

- Follow the existing code style
- Write clear commit messages
- Add tests for new features

## Reporting Issues

Use the issue tracker to report bugs or request features.

## Code of Conduct

Please read and follow our [Code of Conduct](CODE_OF_CONDUCT.md).
"#
                .to_string(),
            ),
            ("Documentation", "CODE_OF_CONDUCT.md") => (
                "CODE_OF_CONDUCT.md",
                r#"# Contributor Covenant Code of Conduct

## Our Pledge

We as members, contributors, and leaders pledge to make participation in our
community a harassment-free experience for everyone.

## Our Standards

Examples of behavior that contributes to a positive environment:

* Using welcoming and inclusive language
* Being respectful of differing viewpoints and experiences
* Gracefully accepting constructive criticism
* Focusing on what is best for the community

Examples of unacceptable behavior:

* Trolling, insulting/derogatory comments, and personal attacks
* Public or private harassment
* Publishing others' private information without permission

## Enforcement

Instances of abusive, harassing, or otherwise unacceptable behavior may be
reported to the project maintainers. All complaints will be reviewed and
investigated promptly and fairly.

## Attribution

This Code of Conduct is adapted from the [Contributor Covenant](https://www.contributor-covenant.org).
"#
                .to_string(),
            ),
            ("Documentation", "MAINTAINERS.md") => (
                "MAINTAINERS.md",
                r#"# Maintainers

## Current Maintainers

| Name | Role | Contact |
|------|------|---------|
| Your Name | Lead Maintainer | email@example.org |

## Becoming a Maintainer

Contributors who have made significant contributions may be invited to
become maintainers.

## Responsibilities

- Review and merge pull requests
- Triage issues
- Release management
- Community support
"#
                .to_string(),
            ),
            ("Documentation", "CHANGELOG.md") => (
                "CHANGELOG.md",
                r#"# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Initial project setup

### Changed
- None

### Deprecated
- None

### Removed
- None

### Fixed
- None

### Security
- None
"#
                .to_string(),
            ),
            ("Well-Known", ".well-known/ directory") => {
                // Create directory
                let well_known_path = repo_path.join(".well-known");
                if fs::create_dir_all(&well_known_path).is_ok() {
                    fixes_applied += 1;
                    println!("  Created: .well-known/");
                }
                continue;
            }
            ("Well-Known", "security.txt") => (
                ".well-known/security.txt",
                r#"# Security contact information
# See https://securitytxt.org/ for format specification

Contact: mailto:security@example.org
Expires: 2025-12-31T23:59:59.000Z
Preferred-Languages: en
Canonical: https://example.org/.well-known/security.txt
"#
                .to_string(),
            ),
            ("Well-Known", "ai.txt") => (
                ".well-known/ai.txt",
                r#"# AI Training Policy
# See proposed standard at https://example.org/ai-txt

User-agent: *
Disallow: Training

# This project does not consent to use for AI/ML training without explicit permission.
# Contact: ai-policy@example.org
"#
                .to_string(),
            ),
            ("Well-Known", "humans.txt") => (
                ".well-known/humans.txt",
                r#"/* TEAM */
Developer: Your Name
Contact: email@example.org
Location: Your Location

/* THANKS */
Name: Contributors

/* SITE */
Last update: 2024/01/01
Language: English
Standards: RSR Bronze
"#
                .to_string(),
            ),
            ("Build System", "justfile") => (
                "justfile",
                r#"# Justfile for project automation
# See https://github.com/casey/just

# Default recipe
default:
    @just --list

# Build the project
build:
    cargo build

# Build release version
release:
    cargo build --release

# Run tests
test:
    cargo test

# Run clippy lints
lint:
    cargo clippy -- -D warnings

# Format code
fmt:
    cargo fmt

# Check formatting
fmt-check:
    cargo fmt --check

# Run all checks
check: fmt-check lint test
    @echo "All checks passed!"

# Clean build artifacts
clean:
    cargo clean
"#
                .to_string(),
            ),
            ("Build System", "flake.nix") => (
                "flake.nix",
                r#"{
  description = "Project flake";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
      in {
        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            rustc
            cargo
            rust-analyzer
            clippy
            rustfmt
          ];
        };
      }
    );
}
"#
                .to_string(),
            ),
            ("Build System", ".gitlab-ci.yml") => (
                ".gitlab-ci.yml",
                r#"# GitLab CI/CD Configuration

stages:
  - check
  - test
  - build

variables:
  CARGO_HOME: ${CI_PROJECT_DIR}/.cargo

cache:
  key: ${CI_COMMIT_REF_SLUG}
  paths:
    - .cargo/
    - target/

check:
  stage: check
  image: rust:latest
  script:
    - cargo fmt --check
    - cargo clippy -- -D warnings

test:
  stage: test
  image: rust:latest
  script:
    - cargo test --verbose

build:
  stage: build
  image: rust:latest
  script:
    - cargo build --release
  artifacts:
    paths:
      - target/release/
    expire_in: 1 week
"#
                .to_string(),
            ),
            ("Source Structure", "src/ directory") => {
                let src_path = repo_path.join("src");
                if fs::create_dir_all(&src_path).is_ok() {
                    // Create a minimal main.rs
                    let main_path = src_path.join("main.rs");
                    if !main_path.exists() {
                        let content = r#"fn main() {
    println!("Hello, world!");
}
"#;
                        if fs::write(&main_path, content).is_ok() {
                            fixes_applied += 1;
                            println!("  Created: src/main.rs");
                        }
                    } else {
                        fixes_applied += 1;
                        println!("  Created: src/");
                    }
                }
                continue;
            }
            ("Source Structure", "tests/ directory") => {
                let tests_path = repo_path.join("tests");
                if fs::create_dir_all(&tests_path).is_ok() {
                    // Create a minimal test file
                    let test_path = tests_path.join("integration_tests.rs");
                    if !test_path.exists() {
                        let content = r#"#[test]
fn test_example() {
    assert!(true);
}
"#;
                        if fs::write(&test_path, content).is_ok() {
                            fixes_applied += 1;
                            println!("  Created: tests/integration_tests.rs");
                        }
                    } else {
                        fixes_applied += 1;
                        println!("  Created: tests/");
                    }
                }
                continue;
            }
            _ => continue,
        };

        // Create the file
        let file_path = repo_path.join(relative_path);

        // Create parent directories if needed
        if let Some(parent) = file_path.parent() {
            let _ = fs::create_dir_all(parent);
        }

        if !file_path.exists() && fs::write(&file_path, content).is_ok() {
            fixes_applied += 1;
            println!("  Created: {}", relative_path);
        }
    }

    fixes_applied
}

fn main() {
    let options = match parse_args() {
        Ok(opts) => opts,
        Err(e) => {
            eprintln!("Error: {}", e);
            eprintln!("Use --help for usage information.");
            process::exit(exit_codes::INVALID_ARGS);
        },
    };

    if !options.repo_path.exists() {
        eprintln!(
            "Error: Path does not exist: {}",
            options.repo_path.display()
        );
        process::exit(exit_codes::INVALID_PATH);
    }

    if !options.repo_path.is_dir() {
        eprintln!(
            "Error: Path is not a directory: {}",
            options.repo_path.display()
        );
        process::exit(exit_codes::INVALID_PATH);
    }

    let report = verify_repository(&options.repo_path);

    // Handle different actions
    match options.action {
        BotAction::Badge => {
            let level = report.highest_level().unwrap_or(ComplianceLevel::Bronze);
            println!("{}", generate_badge(level));
            process::exit(exit_codes::SUCCESS);
        },
        BotAction::Conformity => {
            println!("{}", generate_conformity_doc(&report));
            process::exit(exit_codes::SUCCESS);
        },
        BotAction::Fix => {
            let fixes_applied = apply_fixes(&options.repo_path, &report);
            if fixes_applied > 0 {
                println!("Applied {} fixes to the repository.", fixes_applied);
                println!("Run 'rhodibot check' to verify compliance.");
            } else {
                println!("No fixes needed - repository is already compliant.");
            }
            process::exit(exit_codes::SUCCESS);
        },
        BotAction::Check => {
            // Continue with normal output
        },
    }

    // Output based on format and verbosity
    match options.format {
        OutputFormat::Json => print_json_report(&report),
        OutputFormat::Human => match options.verbosity {
            Verbosity::Quiet => print_quiet_report(&report),
            Verbosity::Normal => print_report(&report),
            Verbosity::Verbose => print_verbose_report(&report),
        },
        OutputFormat::Sarif => {
            println!("{}", generate_sarif(&report));
        },
    }

    // Exit with appropriate code
    let exit_code = if report.has_critical_warnings() {
        exit_codes::SECURITY_WARNING
    } else if !report.bronze_compliance() {
        exit_codes::COMPLIANCE_FAILED
    } else {
        exit_codes::SUCCESS
    };

    process::exit(exit_code);
}
