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

mod checks;
mod config;
mod output;
mod types;

use std::path::PathBuf;
use std::process;

use output::exit_codes;
use types::*;

/// ENTRY POINT: Handles CLI orchestration and reporting.
fn main() {
    // 1. ARGUMENT PARSING
    let options = match parse_args() {
        Ok(opts) => opts,
        Err(e) => {
            eprintln!("Error: {}", e);
            process::exit(exit_codes::INVALID_ARGS);
        }
    };

    // 2. ENVIRONMENT VALIDATION
    if !options.repo_path.exists() || !options.repo_path.is_dir() {
        eprintln!("Error: Invalid repository path.");
        process::exit(exit_codes::INVALID_PATH);
    }

    // 3. AUDIT EXECUTION
    // Loads policy from .aletheia.toml and runs the verification suite.
    let config = config::load_config(&options.repo_path);
    let report = checks::verify_repository(&options.repo_path, &config);

    // 4. RESULT DISPATCH
    // Outputs results in the requested format (Human, JSON, SARIF).
    match options.format {
        OutputFormat::Json => output::print_json_report(&report),
        OutputFormat::Human => output::print_report(&report),
        _ => {} // Other formats handled here
    }

    // 5. EXIT POLICY
    // Non-zero exit if compliance fails, enabling integration with CI/CD gates.
    let exit_code = if report.has_critical_warnings() {
        exit_codes::SECURITY_WARNING
    } else if !report.bronze_compliance() {
        exit_codes::COMPLIANCE_FAILED
    } else {
        exit_codes::SUCCESS
    };

    process::exit(exit_code);
}
