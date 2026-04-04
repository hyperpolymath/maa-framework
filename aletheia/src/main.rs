// SPDX-License-Identifier: PMPL-1.0-or-later

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

#![forbid(unsafe_code)]
mod checks;
mod config;
mod output;
mod types;

use std::path::Path;
use std::process;

use output::exit_codes;
use types::*;

/// CLI options.
#[derive(Debug)]
struct Options {
    repo_path: std::path::PathBuf,
    format: OutputFormat,
}

/// Output format selection.
#[derive(Debug)]
enum OutputFormat {
    Human,
    Json,
    Sarif,
}

/// Parse command-line arguments.
fn parse_args() -> Result<Options, String> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        return Err("Usage: aletheia <repo-path> [--json|--sarif]".to_string());
    }

    let repo_path = std::path::PathBuf::from(&args[1]);
    let format = if args.len() > 2 {
        match args[2].as_str() {
            "--json" => OutputFormat::Json,
            "--sarif" => OutputFormat::Sarif,
            _ => OutputFormat::Human,
        }
    } else {
        OutputFormat::Human
    };

    Ok(Options { repo_path, format })
}

/// ENTRY POINT: Handles CLI orchestration and reporting.
fn main() {
    // 1. ARGUMENT PARSING
    let options = match parse_args() {
        Ok(opts) => opts,
        Err(e) => {
            eprintln!("Error: {}", e);
            process::exit(1);
        }
    };

    // 2. ENVIRONMENT VALIDATION
    if !options.repo_path.exists() || !options.repo_path.is_dir() {
        eprintln!("Error: Invalid repository path.");
        process::exit(1);
    }

    // 3. AUDIT EXECUTION
    // Loads policy from .aletheia.toml and runs the verification suite.
    let config = config::Config::load_config(&options.repo_path);
    let report = verify_repository(&options.repo_path, &config);

    // 4. RESULT DISPATCH
    // Outputs results in the requested format (Human, JSON, SARIF).
    match options.format {
        OutputFormat::Json => output::print_json_report(&report),
        OutputFormat::Human => output::print_report(&report),
        OutputFormat::Sarif => output::print_sarif_report(&report),
    }

    // 5. EXIT POLICY
    let exit_code = if report.checks.iter().any(|c| !c.passed && c.required_for == ComplianceLevel::Bronze) {
        exit_codes::COMPLIANCE_FAILED
    } else if report.warnings.iter().any(|w| w.level == "critical") {
        exit_codes::SECURITY_WARNING
    } else {
        exit_codes::SUCCESS
    };

    process::exit(exit_code);
}

/// Verify repository against RSR standards.
fn verify_repository(repo_path: &Path, _config: &config::Config) -> ComplianceReport {
    let mut report = ComplianceReport::new(repo_path.to_path_buf());

    // Run checks
    checks::check_documentation(&mut report, repo_path, &[]);
    checks::check_spdx_headers(&mut report, repo_path);
    checks::check_workflow_pins(&mut report, repo_path);

    report
}
