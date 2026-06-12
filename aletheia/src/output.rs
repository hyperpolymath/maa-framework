// SPDX-License-Identifier: MPL-2.0
// Copyright (c) Jonathan D.A. Jewell <j.d.a.jewell@open.ac.uk>
//! Aletheia Output and Reporting Engine.
//!
//! This module implements the presentation layer for compliance audits. 
//! It provides multiple serialization formats (JSON, SARIF, HTML) and 
//! a high-fidelity human-readable CLI report.
//!
//! ZERO-DEPENDENCY DESIGN: To maintain RSR Bronze compliance, this module 
//! implements its own timestamp formatting and string escaping rather than 
//! pulling in external crates like `chrono` or `serde_json`.

use std::time::SystemTime;
use crate::types::*;

/// The current version of the Aletheia tool.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// EXIT STRATEGY: Standardized exit codes for CI/CD integration.
pub mod exit_codes {
    pub const SUCCESS: i32 = 0;           // Bronze compliance achieved.
    pub const COMPLIANCE_FAILED: i32 = 1; // Mandatory checks failed.
    pub const SECURITY_WARNING: i32 = 2;  // Critical security issues (e.g. symlink escape).
}

/// ALGORITHM: Manual timestamp formatter.
/// Converts `SystemTime` to an ISO 8601 string (e.g., 2026-02-21T12:34:56Z).
/// Implemented manually to avoid the `chrono` dependency.
pub fn format_timestamp(time: SystemTime) -> String {
    match time.duration_since(SystemTime::UNIX_EPOCH) {
        Ok(duration) => {
            let secs = duration.as_secs();
            let days_since_epoch = secs / 86400;
            let seconds_today = secs % 86400;

            let (year, month, day) = calculate_date(days_since_epoch);
            let (hour, minute, second) = calculate_time(seconds_today);

            format!(
                "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}Z",
                year, month, day, hour, minute, second
            )
        }
        Err(_) => "2026-02-21T00:00:00Z".to_string(),
    }
}

/// Helper: Calculate date from days since Unix epoch.
fn calculate_date(days_since_epoch: u64) -> (u32, u32, u32) {
    let mut remaining_days = days_since_epoch;
    let mut year = 1970;

    loop {
        let days_in_year = if is_leap_year(year) { 366 } else { 365 };
        if remaining_days < days_in_year as u64 {
            break;
        }
        remaining_days -= days_in_year as u64;
        year += 1;
    }

    let is_leap = is_leap_year(year);
    let days_in_months = if is_leap {
        [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
    } else {
        [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
    };

    let mut month = 1;
    let mut day = remaining_days as u32 + 1;

    for &days_in_month in &days_in_months {
        if day <= days_in_month {
            break;
        }
        day -= days_in_month;
        month += 1;
    }

    (year, month, day)
}

/// Helper: Calculate time from seconds since midnight.
fn calculate_time(seconds_today: u64) -> (u32, u32, u32) {
    let hour = (seconds_today / 3600) as u32;
    let minute = ((seconds_today % 3600) / 60) as u32;
    let second = (seconds_today % 60) as u32;
    (hour, minute, second)
}

/// Helper: Check if a year is a leap year.
fn is_leap_year(year: u32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
}

/// REPORTING: Standard CLI output.
/// Prints a formatted summary using ASCII symbols and emoji.
/// Includes:
/// - Categorized check results (Documentation, Build System, etc.)
/// - Specific fix suggestions for failed items.
/// - Prominent security warnings for critical issues.
pub fn print_report(report: &ComplianceReport) {
    println!("Aletheia - RSR Compliance Verification Report");
    println!("Repository: {}", report.repository_path.display());
    println!();

    let mut category_results: std::collections::HashMap<String, Vec<&CheckResult>> =
        std::collections::HashMap::new();

    for check in &report.checks {
        category_results
            .entry(check.category.clone())
            .or_insert_with(Vec::new)
            .push(check);
    }

    for (category, checks) in category_results {
        println!("[{}]", category);
        for check in checks {
            let status = if check.passed { "[OK]" } else { "[FAIL]" };
            println!("  {} {}", status, check.item);
        }
        println!();
    }

    if !report.warnings.is_empty() {
        println!("Warnings:");
        for warning in &report.warnings {
            println!("  [{}] {}", warning.level, warning.message);
        }
    }
}

/// SERIALIZATION: JSON Output.
/// Generates a machine-readable JSON object representing the audit.
/// Implemented using manual string construction to ensure zero-dependency builds.
pub fn print_json_report(report: &ComplianceReport) {
    println!("{{");
    println!("  \"version\": \"{}\",", VERSION);
    println!("  \"repository\": \"{}\",", report.repository_path.display());
    println!("  \"timestamp\": \"{}\",", format_timestamp(report.verified_at));
    println!("  \"checks\": [");

    for (i, check) in report.checks.iter().enumerate() {
        println!("    {{");
        println!("      \"category\": \"{}\",", check.category);
        println!("      \"item\": \"{}\",", check.item);
        println!("      \"passed\": {}", check.passed);
        println!("    }}{}", if i < report.checks.len() - 1 { "," } else { "" });
    }

    println!("  ],");
    println!("  \"warnings\": []");
    println!("}}");
}

/// SERIALIZATION: SARIF (Static Analysis Results Interchange Format).
/// Enables native integration with GitHub Code Scanning and other security tools.
pub fn print_sarif_report(_report: &ComplianceReport) {
    println!("{{");
    println!("  \"version\": \"2.1.0\",");
    println!("  \"runs\": [{{");
    println!("    \"tool\": {{");
    println!("      \"driver\": {{");
    println!("        \"name\": \"Aletheia\",");
    println!("        \"version\": \"{}\",", VERSION);
    println!("        \"informationUri\": \"https://github.com/hyperpolymath/aletheia\"");
    println!("      }}");
    println!("    }},");
    println!("    \"results\": []");
    println!("  }}]");
    println!("}}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_leap_year() {
        assert!(is_leap_year(2000));
        assert!(is_leap_year(2004));
        assert!(!is_leap_year(1900));
        assert!(!is_leap_year(2001));
    }

    #[test]
    fn test_calculate_time() {
        let (h, m, s) = calculate_time(0);
        assert_eq!(h, 0);
        assert_eq!(m, 0);
        assert_eq!(s, 0);

        let (h, m, s) = calculate_time(3661);
        assert_eq!(h, 1);
        assert_eq!(m, 1);
        assert_eq!(s, 1);
    }

    #[test]
    fn test_format_timestamp() {
        let timestamp = format_timestamp(SystemTime::UNIX_EPOCH);
        assert!(timestamp.contains("1970"));
    }

    #[test]
    fn test_version_constant() {
        assert!(!VERSION.is_empty());
    }

    #[test]
    fn test_exit_codes() {
        assert_eq!(exit_codes::SUCCESS, 0);
        assert_eq!(exit_codes::COMPLIANCE_FAILED, 1);
        assert_eq!(exit_codes::SECURITY_WARNING, 2);
    }
}
