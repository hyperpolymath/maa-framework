// SPDX-License-Identifier: MPL-2.0
// Copyright (c) Jonathan D.A. Jewell <j.d.a.jewell@open.ac.uk>
//! Aletheia Output and Reporting Engine.
//!
//! This module implements the presentation layer for compliance audits.
//! It provides multiple serialization formats (JSON, SARIF, HTML, badge)
//! and a high-fidelity human-readable CLI report.
//!
//! ZERO-DEPENDENCY DESIGN: To maintain RSR Bronze compliance, this module
//! implements its own timestamp formatting and string escaping rather than
//! pulling in external crates like `chrono` or `serde_json`.

use crate::checks::{LOCAL_SUBSET_NOTE, SSOT_PROVENANCE};
use crate::types::*;
use std::time::SystemTime;

/// The current version of the Aletheia tool.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// EXIT STRATEGY: Standardized exit codes for CI/CD integration.
pub mod exit_codes {
    pub const SUCCESS: i32 = 0; // Required tier achieved, no critical warnings.
    pub const COMPLIANCE_FAILED: i32 = 1; // Required checks failed.
    pub const SECURITY_WARNING: i32 = 2; // Critical security issues (e.g. symlink escape).
    pub const INVALID_PATH: i32 = 3; // Repository path missing or not a directory.
    pub const INVALID_ARGS: i32 = 4; // Unrecognised flags or malformed arguments.
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

            format!("{year:04}-{month:02}-{day:02}T{hour:02}:{minute:02}:{second:02}Z")
        },
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

/// Escape a string for JSON embedding (minimal, hand-rolled).
fn json_escape(text: &str) -> String {
    let mut out = String::with_capacity(text.len() + 2);
    for ch in text.chars() {
        match ch {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if c.is_control() => out.push_str(&format!("\\u{:04x}", c as u32)),
            c => out.push(c),
        }
    }
    out
}

/// Escape a string for HTML embedding (minimal, hand-rolled).
fn html_escape(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

/// Group checks by category, preserving first-seen category order so
/// reports are deterministic across runs and platforms.
fn group_by_category(report: &ComplianceReport) -> Vec<(String, Vec<&CheckResult>)> {
    let mut groups: Vec<(String, Vec<&CheckResult>)> = Vec::new();
    for check in &report.checks {
        match groups.iter_mut().find(|(cat, _)| *cat == check.category) {
            Some((_, items)) => items.push(check),
            None => groups.push((check.category.clone(), vec![check])),
        }
    }
    groups
}

/// REPORTING: quiet mode. A single machine-greppable word.
pub fn print_quiet(report: &ComplianceReport, level: ComplianceLevel) {
    if report.compliant_at(level) && !report.has_critical_warnings() {
        println!("PASS");
    } else {
        println!("FAIL");
    }
}

/// REPORTING: Standard CLI output.
/// Prints a formatted summary using ✅/❌ markers and per-tier verdicts.
/// Includes categorized check results, fix suggestions for failed items,
/// prominent security warnings, score and verification timestamp.
/// Verbose mode adds version, per-tier counts, inline suggestions,
/// exit-code explanation and the local-subset honesty note.
pub fn print_report(report: &ComplianceReport, level: ComplianceLevel, verbose: bool) {
    if verbose {
        println!("Aletheia - RSR Compliance Verification Report (Verbose)");
    } else {
        println!("Aletheia - RSR Compliance Verification Report");
    }
    println!("Repository: {}", report.repository_path.display());
    println!("Verified: {}", format_timestamp(report.verified_at));
    if verbose {
        println!("Version: {VERSION}");
        println!("Required tier: {}", level.name());
    }
    println!();

    for (category, checks) in group_by_category(report) {
        println!("[{category}]");
        for check in checks {
            let status = if check.passed { "✅" } else { "❌" };
            println!("  {status} {} [{}]", check.item, check.required_for.name());
            if verbose {
                if let Some(suggestion) = &check.suggestion {
                    println!("    💡 {suggestion}");
                }
            }
        }
        println!();
    }

    if !report.warnings.is_empty() {
        println!("Warnings:");
        for warning in &report.warnings {
            match &warning.path {
                Some(path) => println!(
                    "  [{}] {} ({})",
                    warning.level,
                    warning.message,
                    path.display()
                ),
                None => println!("  [{}] {}", warning.level, warning.message),
            }
        }
        println!();
    }

    let passed = report.passed_count();
    let total = report.total_count();
    println!(
        "Score: {passed}/{total} checks passed ({:.1}%)",
        report.score_pct()
    );

    let bronze = if report.compliant_at(ComplianceLevel::Bronze) {
        "ACHIEVED"
    } else {
        "NOT MET"
    };
    println!("Bronze-level RSR compliance: {bronze}");
    let silver = if report.compliant_at(ComplianceLevel::Silver) {
        "ACHIEVED"
    } else {
        "NOT MET"
    };
    println!("Silver-level RSR compliance: {silver}");

    if verbose {
        println!();
        println!("Per-tier results:");
        for (tier, tier_passed, tier_total) in report.tier_counts() {
            println!("  {}: {tier_passed}/{tier_total}", tier.name());
        }
        println!();
        println!(
            "Exit code: {} ({})",
            exit_code_for(report, level),
            exit_reason(report, level)
        );
        println!();
        println!("Aligned to: {SSOT_PROVENANCE}");
        println!("Note: {LOCAL_SUBSET_NOTE}");
    } else {
        let failing = report.failing_checks();
        if !failing.is_empty() {
            println!();
            println!("Fix Suggestions:");
            for check in failing {
                if let Some(suggestion) = &check.suggestion {
                    println!("  - {}: {suggestion}", check.item);
                }
            }
        }
    }
}

/// EXIT POLICY shared by the CLI and the verbose explainer: critical
/// security warnings outrank compliance failures (both are non-zero).
pub fn exit_code_for(report: &ComplianceReport, level: ComplianceLevel) -> i32 {
    if report.has_critical_warnings() {
        exit_codes::SECURITY_WARNING
    } else if !report.compliant_at(level) {
        exit_codes::COMPLIANCE_FAILED
    } else {
        exit_codes::SUCCESS
    }
}

/// Human-readable reason for [`exit_code_for`].
fn exit_reason(report: &ComplianceReport, level: ComplianceLevel) -> &'static str {
    if report.has_critical_warnings() {
        "critical security warnings present"
    } else if !report.compliant_at(level) {
        "required-tier checks failed"
    } else {
        "required tier achieved"
    }
}

/// SERIALIZATION: JSON Output.
/// Machine-readable object representing the audit, hand-rolled to keep
/// zero-dependency builds.
pub fn print_json_report(report: &ComplianceReport, level: ComplianceLevel) {
    println!("{{");
    println!("  \"version\": \"{VERSION}\",");
    println!(
        "  \"repository\": \"{}\",",
        json_escape(&report.repository_path.display().to_string())
    );
    println!(
        "  \"verified_at\": \"{}\",",
        format_timestamp(report.verified_at)
    );
    println!("  \"required_tier\": \"{}\",", level.name().to_lowercase());
    println!(
        "  \"score\": {{\"passed\": {}, \"total\": {}, \"percent\": {:.1}}},",
        report.passed_count(),
        report.total_count(),
        report.score_pct()
    );
    println!(
        "  \"bronze_compliant\": {},",
        report.compliant_at(ComplianceLevel::Bronze)
    );
    println!(
        "  \"silver_compliant\": {},",
        report.compliant_at(ComplianceLevel::Silver)
    );
    println!("  \"tiers\": [");
    let tiers = report.tier_counts();
    for (i, (tier, passed, total)) in tiers.iter().enumerate() {
        println!(
            "    {{\"tier\": \"{}\", \"passed\": {passed}, \"total\": {total}}}{}",
            tier.name().to_lowercase(),
            if i + 1 < tiers.len() { "," } else { "" }
        );
    }
    println!("  ],");
    println!("  \"checks\": [");
    for (i, check) in report.checks.iter().enumerate() {
        println!("    {{");
        println!("      \"id\": \"{}\",", json_escape(&check.id));
        println!("      \"category\": \"{}\",", json_escape(&check.category));
        println!("      \"item\": \"{}\",", json_escape(&check.item));
        println!("      \"passed\": {},", check.passed);
        println!(
            "      \"required_for\": \"{}\"{}",
            check.required_for.name().to_lowercase(),
            match &check.suggestion {
                Some(suggestion) =>
                    format!(",\n      \"suggestion\": \"{}\"", json_escape(suggestion)),
                None => String::new(),
            }
        );
        println!(
            "    }}{}",
            if i + 1 < report.checks.len() { "," } else { "" }
        );
    }
    println!("  ],");
    println!("  \"warnings\": [");
    for (i, warning) in report.warnings.iter().enumerate() {
        println!("    {{");
        println!("      \"level\": \"{}\",", json_escape(&warning.level));
        println!(
            "      \"message\": \"{}\"{}",
            json_escape(&warning.message),
            match &warning.path {
                Some(path) => format!(
                    ",\n      \"path\": \"{}\"",
                    json_escape(&path.display().to_string())
                ),
                None => String::new(),
            }
        );
        println!(
            "    }}{}",
            if i + 1 < report.warnings.len() {
                ","
            } else {
                ""
            }
        );
    }
    println!("  ]");
    println!("}}");
}

/// SERIALIZATION: SARIF (Static Analysis Results Interchange Format).
/// Enables native integration with GitHub Code Scanning and other tools.
/// Every check becomes a rule (`rsr/<id>`) with a pass/fail result.
pub fn print_sarif_report(report: &ComplianceReport) {
    println!("{{");
    println!("  \"$schema\": \"https://raw.githubusercontent.com/oasis-tcs/sarif-spec/master/Schemata/sarif-schema-2.1.0.json\",");
    println!("  \"version\": \"2.1.0\",");
    println!("  \"runs\": [{{");
    println!("    \"tool\": {{");
    println!("      \"driver\": {{");
    println!("        \"name\": \"aletheia\",");
    println!("        \"version\": \"{VERSION}\",");
    println!("        \"informationUri\": \"https://github.com/hyperpolymath/aletheia\",");
    println!("        \"rules\": [");
    for (i, check) in report.checks.iter().enumerate() {
        println!("          {{");
        println!("            \"id\": \"rsr/{}\",", json_escape(&check.id));
        println!(
            "            \"name\": \"{}\",",
            json_escape(
                &check
                    .item
                    .replace(|c: char| !c.is_ascii_alphanumeric(), "_")
            )
        );
        println!(
            "            \"shortDescription\": {{\"text\": \"{}\"}},",
            json_escape(&format!("{}: {}", check.category, check.item))
        );
        println!(
            "            \"properties\": {{\"requiredFor\": \"{}\"}}",
            check.required_for.name().to_lowercase()
        );
        println!(
            "          }}{}",
            if i + 1 < report.checks.len() { "," } else { "" }
        );
    }
    println!("        ]");
    println!("      }}");
    println!("    }},");
    println!("    \"results\": [");
    for (i, check) in report.checks.iter().enumerate() {
        let (kind, level) = if check.passed {
            ("pass", "none")
        } else {
            ("fail", "error")
        };
        println!("      {{");
        println!("        \"ruleId\": \"rsr/{}\",", json_escape(&check.id));
        println!("        \"kind\": \"{kind}\",");
        println!("        \"level\": \"{level}\",");
        println!(
            "        \"message\": {{\"text\": \"{}\"}}",
            json_escape(&match &check.suggestion {
                Some(suggestion) if !check.passed => suggestion.clone(),
                _ => format!("{}: {}", check.category, check.item),
            })
        );
        println!(
            "      }}{}",
            if i + 1 < report.checks.len() { "," } else { "" }
        );
    }
    println!("    ]");
    println!("  }}]");
    println!("}}");
}

/// SERIALIZATION: HTML report with embedded CSS (no external assets —
/// the report must render identically offline and air-gapped).
pub fn print_html_report(report: &ComplianceReport, level: ComplianceLevel) {
    let compliant = report.compliant_at(level);
    let verdict = if compliant { "ACHIEVED" } else { "NOT MET" };
    let verdict_class = if compliant { "pass" } else { "fail" };
    println!("<!DOCTYPE html>");
    println!("<html lang=\"en\">");
    println!("<head>");
    println!("<meta charset=\"utf-8\">");
    println!("<title>Aletheia Compliance Report</title>");
    println!("<style>");
    println!("body{{font-family:sans-serif;max-width:60em;margin:2em auto;padding:0 1em}}");
    println!(".pass{{color:#137333}}.fail{{color:#b3261e}}");
    println!("table{{border-collapse:collapse;width:100%}}");
    println!("th,td{{border:1px solid #ccc;padding:.4em .6em;text-align:left}}");
    println!("</style>");
    println!("</head>");
    println!("<body>");
    println!("<h1>Aletheia Compliance Report</h1>");
    println!(
        "<p>Repository: {}</p>",
        html_escape(&report.repository_path.display().to_string())
    );
    println!("<p>Verified: {}</p>", format_timestamp(report.verified_at));
    println!(
        "<p>Score: {}/{} checks passed ({:.1}%)</p>",
        report.passed_count(),
        report.total_count(),
        report.score_pct()
    );
    println!("<p class=\"{verdict_class}\">Bronze-level RSR compliance: {verdict}</p>");
    for (category, checks) in group_by_category(report) {
        println!("<h2>{}</h2>", html_escape(&category));
        println!("<table><tr><th>Status</th><th>Check</th><th>Tier</th></tr>");
        for check in checks {
            let (mark, class) = if check.passed {
                ("&#x2705;", "pass")
            } else {
                ("&#x274C;", "fail")
            };
            println!(
                "<tr class=\"{class}\"><td>{mark}</td><td>{}</td><td>{}</td></tr>",
                html_escape(&check.item),
                check.required_for.name()
            );
        }
        println!("</table>");
    }
    if !report.warnings.is_empty() {
        println!("<h2>Warnings</h2>");
        println!("<ul>");
        for warning in &report.warnings {
            println!(
                "<li>[{}] {}</li>",
                html_escape(&warning.level),
                html_escape(&warning.message)
            );
        }
        println!("</ul>");
    }
    println!("</body>");
    println!("</html>");
}

/// SVG BADGE: shields-style `RSR compliance | passing/failing` badge.
pub fn print_badge(report: &ComplianceReport, level: ComplianceLevel) {
    let passing = report.compliant_at(level) && !report.has_critical_warnings();
    let (status, color) = if passing {
        ("passing", "#97ca00")
    } else {
        ("failing", "#e05d44")
    };
    println!("<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"148\" height=\"20\">");
    println!("  <linearGradient id=\"s\" x2=\"0\" y2=\"100%\">");
    println!("    <stop offset=\"0\" stop-color=\"#bbb\" stop-opacity=\".1\"/>");
    println!("    <stop offset=\"1\" stop-opacity=\".1\"/>");
    println!("  </linearGradient>");
    println!("  <rect rx=\"3\" width=\"148\" height=\"20\" fill=\"#555\"/>");
    println!("  <rect rx=\"3\" x=\"94\" width=\"54\" height=\"20\" fill=\"{color}\"/>");
    println!("  <rect rx=\"3\" width=\"148\" height=\"20\" fill=\"url(#s)\"/>");
    println!(
        "  <g fill=\"#fff\" text-anchor=\"middle\" font-family=\"sans-serif\" font-size=\"11\">"
    );
    println!("    <text x=\"48\" y=\"15\">RSR compliance</text>");
    println!("    <text x=\"120\" y=\"15\">{status}</text>");
    println!("  </g>");
    println!("</svg>");
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn sample_report() -> ComplianceReport {
        let mut report = ComplianceReport::new(PathBuf::from("/tmp/sample"));
        report.add_check(
            "readme",
            "Documentation",
            "README.adoc (or README.md)",
            true,
            ComplianceLevel::Bronze,
            None,
        );
        report.add_check(
            "changelog",
            "Documentation",
            "CHANGELOG.adoc (or .md)",
            false,
            ComplianceLevel::Silver,
            Some("Add CHANGELOG.adoc; v2 2.1.6.".to_string()),
        );
        report
    }

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
    fn test_format_timestamp_shape() {
        // 2026-01-01T00:00:00Z == 1767225600.
        let time = SystemTime::UNIX_EPOCH + std::time::Duration::from_secs(1767225600);
        assert_eq!(format_timestamp(time), "2026-01-01T00:00:00Z");
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
        assert_eq!(exit_codes::INVALID_PATH, 3);
        assert_eq!(exit_codes::INVALID_ARGS, 4);
    }

    #[test]
    fn test_exit_code_for_policy() {
        let report = sample_report();
        assert_eq!(
            exit_code_for(&report, ComplianceLevel::Bronze),
            exit_codes::SUCCESS
        );
        assert_eq!(
            exit_code_for(&report, ComplianceLevel::Silver),
            exit_codes::COMPLIANCE_FAILED
        );
        let mut critical = sample_report();
        critical.add_warning("critical", "escape".to_string(), None);
        // Critical warnings outrank compliance failures.
        assert_eq!(
            exit_code_for(&critical, ComplianceLevel::Bronze),
            exit_codes::SECURITY_WARNING
        );
    }

    #[test]
    fn test_group_by_category_is_stable() {
        let report = sample_report();
        let groups = group_by_category(&report);
        assert_eq!(groups.len(), 1);
        assert_eq!(groups[0].0, "Documentation");
        assert_eq!(groups[0].1.len(), 2);
    }

    #[test]
    fn test_json_escape() {
        assert_eq!(json_escape("a\"b\\c"), "a\\\"b\\\\c");
        assert_eq!(json_escape("line\ntab\t"), "line\\ntab\\t");
    }

    #[test]
    fn test_html_escape() {
        assert_eq!(html_escape("<a>&\""), "&lt;a&gt;&amp;&quot;");
    }
}
