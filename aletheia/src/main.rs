// SPDX-License-Identifier: MPL-2.0
// Copyright (c) Jonathan D.A. Jewell <j.d.a.jewell@open.ac.uk>
//! Aletheia — Authoritative RSR Compliance Verification.
//!
//! Named after the Greek concept of "unconcealment," Aletheia is the
//! gatekeeper for the Rhodium Standard Repository (RSR) ecosystem.
//! It provides automated, deterministic audits of repository state to
//! ensure adherence to safety, security, and documentation standards.
//!
//! COMPLIANCE DIMENSIONS:
//! 1. Memory Safety: Verifies `unsafe_code = "deny"` in Rust crates.
//! 2. Totality: Checks for Idris totality markers.
//! 3. Air-Gapped Readiness: Ensures no network dependencies in core logic.
//! 4. Provenance: Validates SPDX headers and license compliance.
//!
//! SCOPE HONESTY: the checks wired here are the offline file-presence
//! subset of the upstream RSR criteria (see `checks::SSOT_PROVENANCE`).
//! The normative tier verdict always comes from the hypatia
//! `rsr-conformance` oracle; Aletheia is the fast local pre-check.

#![forbid(unsafe_code)]
mod checks;
mod config;
mod output;
mod types;

use std::path::{Path, PathBuf};
use std::process;

use output::exit_codes;
use types::*;

/// CLI options.
#[derive(Debug)]
struct Options {
    repo_path: PathBuf,
    format: OutputFormat,
    verbose: bool,
    quiet: bool,
    config_path: Option<PathBuf>,
    init_hook: bool,
}

/// Output format selection.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum OutputFormat {
    Human,
    Json,
    Sarif,
    Html,
    Badge,
}

impl OutputFormat {
    fn from_name(name: &str) -> Option<OutputFormat> {
        match name.to_ascii_lowercase().as_str() {
            "human" | "text" => Some(OutputFormat::Human),
            "json" => Some(OutputFormat::Json),
            "sarif" => Some(OutputFormat::Sarif),
            "html" => Some(OutputFormat::Html),
            "badge" | "svg" => Some(OutputFormat::Badge),
            _ => None,
        }
    }
}

/// Parse command-line arguments (pure: testable without the process).
///
/// Accepted shape:
/// `aletheia [repo-path] [--format X | --format=X | --json | --sarif |
/// --html | --badge] [-v|--verbose] [-q|--quiet] [--config PATH]
/// [--init-hook] [init-hook [path]] [--help] [--version]`
/// The positional path defaults to `.`; `--` ends flag parsing.
fn parse_args_from(argv: &[String]) -> Result<Options, String> {
    let mut repo_path: Option<PathBuf> = None;
    let mut format = OutputFormat::Human;
    let mut verbose = false;
    let mut quiet = false;
    let mut config_path: Option<PathBuf> = None;
    let mut init_hook = false;
    let mut help = false;
    let mut version = false;
    let mut end_of_flags = false;

    let mut args = argv.iter().peekable();
    // Skip argv[0].
    args.next();

    while let Some(arg) = args.next() {
        if !end_of_flags && arg == "--" {
            end_of_flags = true;
            continue;
        }
        if !end_of_flags && arg.starts_with('-') && arg.len() > 1 {
            if arg == "--help" || arg == "-h" {
                help = true;
            } else if arg == "--version" || arg == "-V" {
                version = true;
            } else if arg == "--verbose" || arg == "-v" {
                verbose = true;
            } else if arg == "--quiet" || arg == "-q" {
                quiet = true;
            } else if arg == "--json" {
                format = OutputFormat::Json;
            } else if arg == "--sarif" {
                format = OutputFormat::Sarif;
            } else if arg == "--html" {
                format = OutputFormat::Html;
            } else if arg == "--badge" {
                format = OutputFormat::Badge;
            } else if arg == "--init-hook" {
                init_hook = true;
            } else if arg == "--format" {
                let value = args.next().ok_or_else(|| {
                    "Missing value for --format (expected human|json|sarif|html|badge)".to_string()
                })?;
                format = OutputFormat::from_name(value)
                    .ok_or_else(|| format!("Unknown --format '{value}'"))?;
            } else if let Some(value) = arg.strip_prefix("--format=") {
                if value.is_empty() {
                    return Err("Missing value for --format=X".to_string());
                }
                format = OutputFormat::from_name(value)
                    .ok_or_else(|| format!("Unknown --format '{value}'"))?;
            } else if arg == "--config" {
                let value = args.next().ok_or_else(|| {
                    "Missing value for --config (expected a file path)".to_string()
                })?;
                config_path = Some(PathBuf::from(value));
            } else if let Some(value) = arg.strip_prefix("--config=") {
                if value.is_empty() {
                    return Err("Missing value for --config=X".to_string());
                }
                config_path = Some(PathBuf::from(value));
            } else {
                return Err(format!(
                    "Unknown option '{arg}'. Run `aletheia --help` for usage."
                ));
            }
            continue;
        }
        if !end_of_flags && arg == "init-hook" && repo_path.is_none() && !init_hook {
            init_hook = true;
            continue;
        }
        if repo_path.is_some() {
            return Err(format!("Unexpected extra argument '{arg}'."));
        }
        repo_path = Some(PathBuf::from(arg));
    }

    if help {
        print_help();
        process::exit(exit_codes::SUCCESS);
    }
    if version {
        println!("aletheia {}", output::VERSION);
        process::exit(exit_codes::SUCCESS);
    }

    Ok(Options {
        repo_path: repo_path.unwrap_or_else(|| PathBuf::from(".")),
        format,
        verbose,
        quiet,
        config_path,
        init_hook,
    })
}

/// Parse command-line arguments from the process environment.
fn parse_args() -> Result<Options, String> {
    let argv: Vec<String> = std::env::args().collect();
    parse_args_from(&argv)
}

/// USAGE text: the CLI contract.
fn print_help() {
    println!("aletheia {} - RSR compliance verification", output::VERSION);
    println!();
    println!("USAGE:");
    println!("    aletheia [OPTIONS] [repo-path]");
    println!("    aletheia init-hook [repo-path]");
    println!();
    println!("ARGS:");
    println!("    [repo-path]    Repository to verify (default: current directory)");
    println!();
    println!("OPTIONS:");
    println!("    -h, --help           Print this help and exit");
    println!("    -V, --version        Print version and exit");
    println!("    --format FORMAT      Output format: human, json, sarif, html, badge");
    println!("    --format=FORMAT      Same, equals syntax (e.g. --format=json)");
    println!("    --json               Shorthand for --format json");
    println!("    --sarif              Shorthand for --format sarif");
    println!("    --html               Shorthand for --format html");
    println!("    --badge              Print an SVG compliance badge and exit");
    println!("    -v, --verbose        Verbose human output (suggestions, tiers, exit reason)");
    println!("    -q, --quiet          Print only PASS or FAIL");
    println!("    --config PATH        Use PATH instead of <repo>/.aletheia.toml");
    println!("    --init-hook          Install a pre-commit hook instead of verifying");
    println!();
    println!("EXIT CODES:");
    println!("    0    Required tier achieved, no critical warnings");
    println!("    1    Required-tier checks failed (see Fix Suggestions)");
    println!("    2    Critical security warnings (e.g. symlink escapes repo)");
    println!("    3    Invalid repository path");
    println!("    4    Invalid arguments");
    println!();
    println!("EXAMPLES:");
    println!("    aletheia                     Verify the current directory");
    println!("    aletheia /path/to/repo       Verify a specific repository");
    println!("    aletheia --format json .     Machine-readable report");
    println!("    aletheia --format=sarif .    SARIF for GitHub code scanning");
    println!("    aletheia -q . && echo shipped Quiet gate for scripts");
    println!("    aletheia init-hook .         Install the pre-commit hook");
    println!();
    println!("CONFIG:");
    println!("    Reads <repo>/.aletheia.toml ([aletheia] level, [checks] toggles,");
    println!("    [ignore] files globs). Absent file means defaults (tier: bronze).");
}

/// Install a pre-commit hook that gates commits on Bronze compliance.
/// Returns the exit code for the process.
fn run_init_hook(repo_path: &Path) -> i32 {
    if !repo_path.is_dir() {
        eprintln!("Error: Invalid repository path.");
        return exit_codes::INVALID_PATH;
    }
    let git_dir = repo_path.join(".git");
    if !git_dir.is_dir() {
        eprintln!(
            "Error: {} is not a git repository (no .git/).",
            repo_path.display()
        );
        return exit_codes::INVALID_PATH;
    }
    let hooks_dir = git_dir.join("hooks");
    if let Err(err) = std::fs::create_dir_all(&hooks_dir) {
        eprintln!("Error: cannot create hooks dir: {err}");
        return exit_codes::INVALID_PATH;
    }
    let hook_path = hooks_dir.join("pre-commit");
    let hook_body = r#"#!/bin/sh
# Installed by `aletheia init-hook`: gate commits on RSR Bronze.
command -v aletheia >/dev/null 2>&1 || {
    echo "aletheia: binary not found on PATH - commit blocked" >&2
    exit 1
}
aletheia --quiet .
status=$?
if [ "$status" -eq 0 ]; then
    exit 0
fi
if [ "$status" -eq 1 ]; then
    echo "aletheia: Bronze compliance NOT MET - commit blocked" >&2
else
    echo "aletheia: verification error (exit $status) - commit blocked" >&2
fi
exit 1
"#;
    if let Err(err) = std::fs::write(&hook_path, hook_body) {
        eprintln!("Error: cannot write pre-commit hook: {err}");
        return exit_codes::INVALID_PATH;
    }
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let perms = std::fs::Permissions::from_mode(0o755);
        if let Err(err) = std::fs::set_permissions(&hook_path, perms) {
            eprintln!("Error: cannot chmod pre-commit hook: {err}");
            return exit_codes::INVALID_PATH;
        }
    }
    println!("Installed pre-commit hook at {}", hook_path.display());
    exit_codes::SUCCESS
}

/// ENTRY POINT: Handles CLI orchestration and reporting.
fn main() {
    // 1. ARGUMENT PARSING
    let options = match parse_args() {
        Ok(opts) => opts,
        Err(message) => {
            eprintln!("Error: {message}");
            process::exit(exit_codes::INVALID_ARGS);
        },
    };

    // 2a. INIT-HOOK MODE (verifies nothing; installs the git hook).
    if options.init_hook {
        process::exit(run_init_hook(&options.repo_path));
    }

    // 2b. ENVIRONMENT VALIDATION
    if !options.repo_path.is_dir() {
        eprintln!(
            "Error: repository path '{}' does not exist or is not a directory.",
            options.repo_path.display()
        );
        process::exit(exit_codes::INVALID_PATH);
    }

    // 3. AUDIT EXECUTION
    let config = match &options.config_path {
        Some(path) => config::Config::load_from_file(path),
        None => config::Config::load_config(&options.repo_path),
    };
    let level = match ComplianceLevel::from_v2_tier(&config.level) {
        Some(level) => level,
        None => {
            eprintln!(
                "Warning: unknown level '{}' in config; falling back to bronze.",
                config.level
            );
            ComplianceLevel::Bronze
        },
    };
    let report = checks::verify_repository(&options.repo_path, &config);

    // 4. RESULT DISPATCH
    if options.quiet {
        output::print_quiet(&report, level);
    } else {
        match options.format {
            OutputFormat::Human => output::print_report(&report, level, options.verbose),
            OutputFormat::Json => output::print_json_report(&report, level),
            OutputFormat::Sarif => output::print_sarif_report(&report),
            OutputFormat::Html => output::print_html_report(&report, level),
            OutputFormat::Badge => output::print_badge(&report, level),
        }
    }

    // 5. EXIT POLICY (shared with the verbose explainer).
    process::exit(output::exit_code_for(&report, level));
}

#[cfg(test)]
mod cli_tests {
    use super::*;

    fn argv(words: &[&str]) -> Vec<String> {
        words.iter().map(ToString::to_string).collect()
    }

    #[test]
    fn test_parse_defaults_to_human_and_cwd() {
        let options = parse_args_from(&argv(&["aletheia"])).expect("bare invocation parses");
        assert_eq!(options.repo_path, PathBuf::from("."));
        assert_eq!(options.format, OutputFormat::Human);
        assert!(!options.verbose);
        assert!(!options.quiet);
        assert!(options.config_path.is_none());
        assert!(!options.init_hook);
    }

    #[test]
    fn test_parse_positional_path() {
        let options =
            parse_args_from(&argv(&["aletheia", "/tmp/repo"])).expect("positional parses");
        assert_eq!(options.repo_path, PathBuf::from("/tmp/repo"));
    }

    #[test]
    fn test_parse_format_forms() {
        for words in [
            vec!["aletheia", "--format", "json"],
            vec!["aletheia", "--format=json"],
            vec!["aletheia", "--json"],
        ] {
            let options = parse_args_from(&argv(&words)).expect("json form parses");
            assert_eq!(options.format, OutputFormat::Json);
        }
        let options =
            parse_args_from(&argv(&["aletheia", "--format", "sarif"])).expect("sarif parses");
        assert_eq!(options.format, OutputFormat::Sarif);
        let options = parse_args_from(&argv(&["aletheia", "--format=html"])).expect("html parses");
        assert_eq!(options.format, OutputFormat::Html);
        let options = parse_args_from(&argv(&["aletheia", "--badge"])).expect("badge parses");
        assert_eq!(options.format, OutputFormat::Badge);
    }

    #[test]
    fn test_parse_flags_and_config() {
        let options = parse_args_from(&argv(&[
            "aletheia",
            "-v",
            "--config",
            "custom.toml",
            "/tmp/repo",
        ]))
        .expect("flags parse");
        assert!(options.verbose);
        assert_eq!(options.config_path, Some(PathBuf::from("custom.toml")));
        assert_eq!(options.repo_path, PathBuf::from("/tmp/repo"));

        let options = parse_args_from(&argv(&["aletheia", "-q"])).expect("quiet parses");
        assert!(options.quiet);
        let options = parse_args_from(&argv(&["aletheia", "--config=other.toml"]))
            .expect("equals config parses");
        assert_eq!(options.config_path, Some(PathBuf::from("other.toml")));
    }

    #[test]
    fn test_parse_init_hook_forms() {
        let options =
            parse_args_from(&argv(&["aletheia", "init-hook", "/tmp/repo"])).expect("subcommand");
        assert!(options.init_hook);
        assert_eq!(options.repo_path, PathBuf::from("/tmp/repo"));
        let options = parse_args_from(&argv(&["aletheia", "--init-hook"])).expect("flag form");
        assert!(options.init_hook);
    }

    #[test]
    fn test_parse_rejects_unknown_options() {
        assert!(parse_args_from(&argv(&["aletheia", "--invalid-option"])).is_err());
        assert!(parse_args_from(&argv(&["aletheia", "--format", "yaml"])).is_err());
        assert!(parse_args_from(&argv(&["aletheia", "--format="])).is_err());
        assert!(parse_args_from(&argv(&["aletheia", "--format"])).is_err());
        assert!(parse_args_from(&argv(&["aletheia", "--config"])).is_err());
        assert!(parse_args_from(&argv(&["aletheia", "a", "b"])).is_err());
    }

    #[test]
    fn test_parse_double_dash_ends_flags() {
        let options =
            parse_args_from(&argv(&["aletheia", "--", "--not-a-flag"])).expect("separator works");
        assert_eq!(options.repo_path, PathBuf::from("--not-a-flag"));
    }

    #[test]
    fn test_output_format_names() {
        assert_eq!(OutputFormat::from_name("JSON"), Some(OutputFormat::Json));
        assert_eq!(OutputFormat::from_name("text"), Some(OutputFormat::Human));
        assert_eq!(OutputFormat::from_name("svg"), Some(OutputFormat::Badge));
        assert_eq!(OutputFormat::from_name("yaml"), None);
    }

    #[test]
    fn test_parse_flags_after_positional() {
        // Review T6: options must stay valid after the positional path.
        let options = parse_args_from(&argv(&["aletheia", "/tmp/repo", "--json"]))
            .expect("flag after positional parses");
        assert_eq!(options.repo_path, PathBuf::from("/tmp/repo"));
        assert_eq!(options.format, OutputFormat::Json);
    }
}
