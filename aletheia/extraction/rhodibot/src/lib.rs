//! Rhodibot - RSR Compliance Bot Library
//!
//! This library provides RSR (Rhodium Standard Repository) compliance verification
//! capabilities for use in CI/CD pipelines, bots, and automation tools.
//!
//! Rhodibot is like Dependabot but for repository standards compliance instead
//! of dependency updates.
//!
//! # Features
//!
//! - Zero dependencies (uses only Rust std library)
//! - Zero unsafe code
//! - Offline-first (no network access required)
//! - Bronze-level RSR compliance checking
//! - JSON output for CI/CD integration
//! - Security-aware (symlink detection)
//!
//! # Example
//!
//! ```rust,no_run
//! use rhodibot::{verify_repository, OutputFormat, Verbosity};
//! use std::path::Path;
//!
//! let report = verify_repository(Path::new("/path/to/repo"));
//! println!("Bronze compliant: {}", report.bronze_compliance());
//! ```

pub mod bot;

use std::fs;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

/// Library version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Exit codes for different failure modes
pub mod exit_codes {
    pub const SUCCESS: i32 = 0;
    pub const COMPLIANCE_FAILED: i32 = 1;
    pub const SECURITY_WARNING: i32 = 2;
    pub const INVALID_PATH: i32 = 3;
    pub const INVALID_ARGS: i32 = 4;
}

/// Output format options
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputFormat {
    Human,
    Json,
    /// Static Analysis Results Interchange Format (SARIF 2.1.0)
    /// Supported by GitHub Security tab, GitLab, Azure DevOps
    Sarif,
}

/// Verbosity level
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Verbosity {
    Quiet,   // Only pass/fail
    Normal,  // Standard output
    Verbose, // Include all details
}

/// RSR Compliance levels
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComplianceLevel {
    Bronze,
    Silver,
    Gold,
    Platinum,
}

impl ComplianceLevel {
    /// Get the badge color for this compliance level
    pub fn badge_color(&self) -> &'static str {
        match self {
            ComplianceLevel::Bronze => "cd7f32",
            ComplianceLevel::Silver => "c0c0c0",
            ComplianceLevel::Gold => "ffd700",
            ComplianceLevel::Platinum => "e5e4e2",
        }
    }

    /// Get the display name for this compliance level
    pub fn display_name(&self) -> &'static str {
        match self {
            ComplianceLevel::Bronze => "Bronze",
            ComplianceLevel::Silver => "Silver",
            ComplianceLevel::Gold => "Gold",
            ComplianceLevel::Platinum => "Platinum",
        }
    }
}

/// Individual compliance check result
#[derive(Debug, Clone)]
pub struct CheckResult {
    pub category: String,
    pub item: String,
    pub passed: bool,
    pub required_for: ComplianceLevel,
    pub description: Option<String>,
}

/// Security warning levels
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WarningLevel {
    Info,
    Warning,
    Critical,
}

/// Security warning
#[derive(Debug, Clone)]
pub struct SecurityWarning {
    pub level: WarningLevel,
    pub message: String,
    pub path: Option<PathBuf>,
}

/// Overall compliance report
#[derive(Debug)]
pub struct ComplianceReport {
    pub checks: Vec<CheckResult>,
    pub warnings: Vec<SecurityWarning>,
    pub repository_path: PathBuf,
    pub verified_at: SystemTime,
}

impl ComplianceReport {
    /// Create a new empty compliance report
    pub fn new(path: PathBuf) -> Self {
        Self {
            checks: Vec::new(),
            warnings: Vec::new(),
            repository_path: path,
            verified_at: SystemTime::now(),
        }
    }

    /// Add a compliance check result
    pub fn add_check(&mut self, category: &str, item: &str, passed: bool, level: ComplianceLevel) {
        self.checks.push(CheckResult {
            category: category.to_string(),
            item: item.to_string(),
            passed,
            required_for: level,
            description: None,
        });
    }

    /// Add a compliance check with description
    pub fn add_check_with_desc(
        &mut self,
        category: &str,
        item: &str,
        passed: bool,
        level: ComplianceLevel,
        description: &str,
    ) {
        self.checks.push(CheckResult {
            category: category.to_string(),
            item: item.to_string(),
            passed,
            required_for: level,
            description: Some(description.to_string()),
        });
    }

    /// Add a security warning
    pub fn add_warning(&mut self, level: WarningLevel, message: &str, path: Option<PathBuf>) {
        self.warnings.push(SecurityWarning {
            level,
            message: message.to_string(),
            path,
        });
    }

    /// Check if Bronze-level compliance is met
    pub fn bronze_compliance(&self) -> bool {
        self.checks
            .iter()
            .filter(|c| c.required_for == ComplianceLevel::Bronze)
            .all(|c| c.passed)
    }

    /// Check if Silver-level compliance is met
    pub fn silver_compliance(&self) -> bool {
        self.bronze_compliance()
            && self
                .checks
                .iter()
                .filter(|c| c.required_for == ComplianceLevel::Silver)
                .all(|c| c.passed)
    }

    /// Get the highest compliance level achieved
    pub fn highest_level(&self) -> Option<ComplianceLevel> {
        if !self.bronze_compliance() || self.has_critical_warnings() {
            return None;
        }
        if self.silver_compliance() {
            // Check for gold and platinum when implemented
            Some(ComplianceLevel::Silver)
        } else {
            Some(ComplianceLevel::Bronze)
        }
    }

    /// Count of passed checks
    pub fn passed_count(&self) -> usize {
        self.checks.iter().filter(|c| c.passed).count()
    }

    /// Total number of checks
    pub fn total_count(&self) -> usize {
        self.checks.len()
    }

    /// Check if there are any critical warnings
    pub fn has_critical_warnings(&self) -> bool {
        self.warnings
            .iter()
            .any(|w| w.level == WarningLevel::Critical)
    }

    /// Get pass percentage
    pub fn percentage(&self) -> f64 {
        if self.total_count() == 0 {
            0.0
        } else {
            (self.passed_count() as f64 / self.total_count() as f64) * 100.0
        }
    }

    /// Get checks by category
    pub fn checks_by_category(&self) -> std::collections::HashMap<String, Vec<&CheckResult>> {
        let mut map = std::collections::HashMap::new();
        for check in &self.checks {
            map.entry(check.category.clone())
                .or_insert_with(Vec::new)
                .push(check);
        }
        map
    }
}

/// Result of checking a path for existence and symlink status
struct PathCheckResult {
    exists: bool,
    is_symlink: bool,
    escapes_repo: bool,
    target: Option<PathBuf>,
}

/// Check if a path is a symlink and if it escapes the repository root
fn check_path_security(path: &Path, repo_root: &Path) -> PathCheckResult {
    let metadata = match fs::symlink_metadata(path) {
        Ok(m) => m,
        Err(_) => {
            return PathCheckResult {
                exists: false,
                is_symlink: false,
                escapes_repo: false,
                target: None,
            }
        },
    };

    let is_symlink = metadata.file_type().is_symlink();

    if !is_symlink {
        return PathCheckResult {
            exists: true,
            is_symlink: false,
            escapes_repo: false,
            target: None,
        };
    }

    let target = match fs::read_link(path) {
        Ok(t) => t,
        Err(_) => {
            return PathCheckResult {
                exists: true,
                is_symlink: true,
                escapes_repo: false,
                target: None,
            };
        },
    };

    let resolved_target = if target.is_absolute() {
        target.clone()
    } else {
        path.parent()
            .map(|p| p.join(&target))
            .unwrap_or(target.clone())
    };

    let canonical_root = repo_root
        .canonicalize()
        .unwrap_or_else(|_| repo_root.to_path_buf());
    let canonical_target = resolved_target
        .canonicalize()
        .unwrap_or_else(|_| resolved_target.clone());

    let escapes_repo = !canonical_target.starts_with(canonical_root);

    PathCheckResult {
        exists: true,
        is_symlink: true,
        escapes_repo,
        target: Some(resolved_target),
    }
}

/// Check if a file exists at the given path (with symlink detection)
fn check_file(base: &Path, filename: &str, report: &mut ComplianceReport) -> bool {
    let path = base.join(filename);
    let security = check_path_security(&path, &report.repository_path);

    if security.is_symlink {
        if security.escapes_repo {
            report.add_warning(
                WarningLevel::Critical,
                &format!(
                    "Symlink '{}' points outside repository to '{}'",
                    filename,
                    security
                        .target
                        .as_ref()
                        .map(|p| p.display().to_string())
                        .unwrap_or_default()
                ),
                Some(path.clone()),
            );
        } else {
            report.add_warning(
                WarningLevel::Info,
                &format!("'{}' is a symlink (within repository bounds)", filename),
                Some(path.clone()),
            );
        }
    }

    security.exists && path.is_file()
}

/// Check if a directory exists at the given path (with symlink detection)
fn check_dir(base: &Path, dirname: &str, report: &mut ComplianceReport) -> bool {
    let path = base.join(dirname);
    let security = check_path_security(&path, &report.repository_path);

    if security.is_symlink {
        if security.escapes_repo {
            report.add_warning(
                WarningLevel::Critical,
                &format!(
                    "Symlink directory '{}' points outside repository to '{}'",
                    dirname,
                    security
                        .target
                        .as_ref()
                        .map(|p| p.display().to_string())
                        .unwrap_or_default()
                ),
                Some(path.clone()),
            );
        } else {
            report.add_warning(
                WarningLevel::Info,
                &format!(
                    "'{}' is a symlink directory (within repository bounds)",
                    dirname
                ),
                Some(path.clone()),
            );
        }
    }

    security.exists && path.is_dir()
}

/// Verify documentation files exist
fn check_documentation(report: &mut ComplianceReport, repo_path: &Path) {
    // README can be either .md or .adoc (AsciiDoc is acceptable alternative)
    let readme_md = check_file(repo_path, "README.md", report);
    let readme_adoc = if !readme_md {
        check_file(repo_path, "README.adoc", report)
    } else {
        false
    };
    report.add_check(
        "Documentation",
        "README.md",
        readme_md || readme_adoc,
        ComplianceLevel::Bronze,
    );

    // Documents that can be either .md or .adoc format
    let docs_with_adoc_alternative = vec![
        "SECURITY.md",
        "CONTRIBUTING.md",
        "CODE_OF_CONDUCT.md",
        "MAINTAINERS.md",
        "CHANGELOG.md",
    ];

    for doc in docs_with_adoc_alternative {
        let exists_md = check_file(repo_path, doc, report);
        let adoc_name = doc.replace(".md", ".adoc");
        let exists_adoc = if !exists_md {
            check_file(repo_path, &adoc_name, report)
        } else {
            false
        };
        report.add_check(
            "Documentation",
            doc,
            exists_md || exists_adoc,
            ComplianceLevel::Bronze,
        );
    }

    // LICENSE.txt has no alternative format
    let license_exists = check_file(repo_path, "LICENSE.txt", report);
    report.add_check(
        "Documentation",
        "LICENSE.txt",
        license_exists,
        ComplianceLevel::Bronze,
    );
}

/// Verify .well-known directory and required files
fn check_well_known(report: &mut ComplianceReport, repo_path: &Path) {
    let has_dir = check_dir(repo_path, ".well-known", report);

    report.add_check(
        "Well-Known",
        ".well-known/ directory",
        has_dir,
        ComplianceLevel::Bronze,
    );

    let well_known_path = repo_path.join(".well-known");
    let required_files = vec!["security.txt", "ai.txt", "humans.txt"];
    for file in required_files {
        let exists = if has_dir {
            check_file(&well_known_path, file, report)
        } else {
            false
        };
        report.add_check("Well-Known", file, exists, ComplianceLevel::Bronze);
    }
}

/// Verify build system files
fn check_build_system(report: &mut ComplianceReport, repo_path: &Path) {
    // justfile can be either lowercase or Justfile (capitalized)
    let justfile_lower = check_file(repo_path, "justfile", report);
    let justfile_cap = if !justfile_lower {
        check_file(repo_path, "Justfile", report)
    } else {
        false
    };
    report.add_check(
        "Build System",
        "justfile",
        justfile_lower || justfile_cap,
        ComplianceLevel::Bronze,
    );

    let other_build_files = vec![
        ("flake.nix", ComplianceLevel::Bronze),
        (".gitlab-ci.yml", ComplianceLevel::Bronze),
    ];

    for (file, level) in other_build_files {
        let exists = check_file(repo_path, file, report);
        report.add_check("Build System", file, exists, level);
    }
}

/// Verify source code structure
fn check_source_structure(report: &mut ComplianceReport, repo_path: &Path) {
    let has_src = check_dir(repo_path, "src", report);
    let has_tests = check_dir(repo_path, "tests", report) || check_dir(repo_path, "test", report);

    report.add_check(
        "Source Structure",
        "src/ directory",
        has_src,
        ComplianceLevel::Bronze,
    );

    report.add_check(
        "Source Structure",
        "tests/ directory",
        has_tests,
        ComplianceLevel::Bronze,
    );
}

/// Run all compliance checks on a repository
pub fn verify_repository(repo_path: &Path) -> ComplianceReport {
    let mut report = ComplianceReport::new(repo_path.to_path_buf());

    check_documentation(&mut report, repo_path);
    check_well_known(&mut report, repo_path);
    check_build_system(&mut report, repo_path);
    check_source_structure(&mut report, repo_path);

    report
}

/// Format a SystemTime as a human-readable timestamp (ISO 8601)
pub fn format_timestamp(time: SystemTime) -> String {
    match time.duration_since(SystemTime::UNIX_EPOCH) {
        Ok(duration) => {
            let secs = duration.as_secs();
            let days = secs / 86400;
            let time_secs = secs % 86400;
            let hours = time_secs / 3600;
            let minutes = (time_secs % 3600) / 60;
            let seconds = time_secs % 60;

            let mut year = 1970;
            let mut remaining_days = days;

            loop {
                let days_in_year = if year % 4 == 0 && (year % 100 != 0 || year % 400 == 0) {
                    366
                } else {
                    365
                };
                if remaining_days < days_in_year {
                    break;
                }
                remaining_days -= days_in_year;
                year += 1;
            }

            let is_leap = year % 4 == 0 && (year % 100 != 0 || year % 400 == 0);
            let days_in_months: [u64; 12] = if is_leap {
                [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
            } else {
                [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
            };

            let mut month = 1;
            for days_in_month in days_in_months.iter() {
                if remaining_days < *days_in_month {
                    break;
                }
                remaining_days -= days_in_month;
                month += 1;
            }
            let day = remaining_days + 1;

            format!(
                "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}Z",
                year, month, day, hours, minutes, seconds
            )
        },
        Err(_) => "unknown".to_string(),
    }
}

/// Escape a string for JSON output
pub fn json_escape(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '"' => result.push_str("\\\""),
            '\\' => result.push_str("\\\\"),
            '\n' => result.push_str("\\n"),
            '\r' => result.push_str("\\r"),
            '\t' => result.push_str("\\t"),
            c if c.is_control() => {
                result.push_str(&format!("\\u{:04x}", c as u32));
            },
            c => result.push(c),
        }
    }
    result
}

/// Generate SARIF (Static Analysis Results Interchange Format) output
///
/// SARIF is a standard JSON format for static analysis tools, supported by
/// GitHub Security tab, GitLab, Azure DevOps, and other platforms.
pub fn generate_sarif(report: &ComplianceReport) -> String {
    let timestamp = format_timestamp(report.verified_at);
    let repo_path = json_escape(&report.repository_path.display().to_string());

    let mut sarif = String::new();
    sarif.push_str("{\n");
    sarif.push_str("  \"$schema\": \"https://raw.githubusercontent.com/oasis-tcs/sarif-spec/master/Schemata/sarif-schema-2.1.0.json\",\n");
    sarif.push_str("  \"version\": \"2.1.0\",\n");
    sarif.push_str("  \"runs\": [\n");
    sarif.push_str("    {\n");
    sarif.push_str("      \"tool\": {\n");
    sarif.push_str("        \"driver\": {\n");
    sarif.push_str("          \"name\": \"rhodibot\",\n");
    sarif.push_str(&format!("          \"version\": \"{}\",\n", VERSION));
    sarif.push_str("          \"informationUri\": \"https://github.com/hyperpolymath/rhodium-standard-repositories\",\n");
    sarif.push_str("          \"rules\": [\n");

    // Generate rules from unique check categories and items
    let mut rules: Vec<(&str, &str, &ComplianceLevel)> = Vec::new();
    for check in &report.checks {
        let rule_id = format!("{}/{}", check.category, check.item);
        if !rules
            .iter()
            .any(|(cat, item, _)| format!("{}/{}", cat, item) == rule_id)
        {
            rules.push((&check.category, &check.item, &check.required_for));
        }
    }

    for (i, (category, item, level)) in rules.iter().enumerate() {
        let rule_id = format!("{}/{}", category, item);
        let comma = if i < rules.len() - 1 { "," } else { "" };
        sarif.push_str("            {\n");
        sarif.push_str(&format!(
            "              \"id\": \"{}\",\n",
            json_escape(&rule_id)
        ));
        sarif.push_str(&format!(
            "              \"name\": \"{}\",\n",
            json_escape(item)
        ));
        sarif.push_str("              \"shortDescription\": {\n");
        sarif.push_str(&format!(
            "                \"text\": \"RSR {:?} requirement: {}\"\n",
            level,
            json_escape(item)
        ));
        sarif.push_str("              },\n");
        sarif.push_str(&format!(
            "              \"defaultConfiguration\": {{\n                \"level\": \"{}\"\n              }}\n",
            match level {
                ComplianceLevel::Bronze => "error",
                ComplianceLevel::Silver => "warning",
                ComplianceLevel::Gold => "note",
                ComplianceLevel::Platinum => "note",
            }
        ));
        sarif.push_str(&format!("            }}{}\n", comma));
    }

    sarif.push_str("          ]\n");
    sarif.push_str("        }\n");
    sarif.push_str("      },\n");

    // Invocation info
    sarif.push_str("      \"invocations\": [\n");
    sarif.push_str("        {\n");
    sarif.push_str(&format!(
        "          \"executionSuccessful\": {},\n",
        !report.has_critical_warnings() && report.bronze_compliance()
    ));
    sarif.push_str(&format!("          \"endTimeUtc\": \"{}\"\n", timestamp));
    sarif.push_str("        }\n");
    sarif.push_str("      ],\n");

    // Results (failed checks and warnings)
    sarif.push_str("      \"results\": [\n");

    let mut result_count = 0;
    let total_results = report.checks.iter().filter(|c| !c.passed).count() + report.warnings.len();

    // Add failed checks
    for check in &report.checks {
        if !check.passed {
            let rule_id = format!("{}/{}", check.category, check.item);
            let comma = if result_count < total_results - 1 {
                ","
            } else {
                ""
            };
            sarif.push_str("        {\n");
            sarif.push_str(&format!(
                "          \"ruleId\": \"{}\",\n",
                json_escape(&rule_id)
            ));
            sarif.push_str(&format!(
                "          \"level\": \"{}\",\n",
                match check.required_for {
                    ComplianceLevel::Bronze => "error",
                    ComplianceLevel::Silver => "warning",
                    ComplianceLevel::Gold => "note",
                    ComplianceLevel::Platinum => "note",
                }
            ));
            sarif.push_str("          \"message\": {\n");
            sarif.push_str(&format!(
                "            \"text\": \"Missing required file or directory: {}\"\n",
                json_escape(&check.item)
            ));
            sarif.push_str("          },\n");
            sarif.push_str("          \"locations\": [\n");
            sarif.push_str("            {\n");
            sarif.push_str("              \"physicalLocation\": {\n");
            sarif.push_str("                \"artifactLocation\": {\n");
            sarif.push_str(&format!(
                "                  \"uri\": \"{}\"\n",
                json_escape(&check.item)
            ));
            sarif.push_str("                }\n");
            sarif.push_str("              }\n");
            sarif.push_str("            }\n");
            sarif.push_str("          ]\n");
            sarif.push_str(&format!("        }}{}\n", comma));
            result_count += 1;
        }
    }

    // Add warnings
    for warning in &report.warnings {
        let comma = if result_count < total_results - 1 {
            ","
        } else {
            ""
        };
        sarif.push_str("        {\n");
        sarif.push_str("          \"ruleId\": \"security/warning\",\n");
        sarif.push_str(&format!(
            "          \"level\": \"{}\",\n",
            match warning.level {
                WarningLevel::Info => "note",
                WarningLevel::Warning => "warning",
                WarningLevel::Critical => "error",
            }
        ));
        sarif.push_str("          \"message\": {\n");
        sarif.push_str(&format!(
            "            \"text\": \"{}\"\n",
            json_escape(&warning.message)
        ));
        sarif.push_str("          }");

        if let Some(ref path) = warning.path {
            sarif.push_str(",\n");
            sarif.push_str("          \"locations\": [\n");
            sarif.push_str("            {\n");
            sarif.push_str("              \"physicalLocation\": {\n");
            sarif.push_str("                \"artifactLocation\": {\n");
            sarif.push_str(&format!(
                "                  \"uri\": \"{}\"\n",
                json_escape(&path.display().to_string())
            ));
            sarif.push_str("                }\n");
            sarif.push_str("              }\n");
            sarif.push_str("            }\n");
            sarif.push_str("          ]\n");
        } else {
            sarif.push('\n');
        }

        sarif.push_str(&format!("        }}{}\n", comma));
        result_count += 1;
    }

    sarif.push_str("      ],\n");

    // Artifact (repository being analyzed)
    sarif.push_str("      \"artifacts\": [\n");
    sarif.push_str("        {\n");
    sarif.push_str("          \"location\": {\n");
    sarif.push_str(&format!("            \"uri\": \"{}\"\n", repo_path));
    sarif.push_str("          },\n");
    sarif.push_str("          \"roles\": [\"analysisTarget\"]\n");
    sarif.push_str("        }\n");
    sarif.push_str("      ]\n");
    sarif.push_str("    }\n");
    sarif.push_str("  ]\n");
    sarif.push_str("}\n");

    sarif
}

/// Bot action types for CI/CD integration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BotAction {
    /// Check compliance and report
    Check,
    /// Check compliance and create PR with fixes
    Fix,
    /// Generate badge for README
    Badge,
    /// Generate conformity document
    Conformity,
}

/// Bot configuration
#[derive(Debug, Clone)]
pub struct BotConfig {
    pub action: BotAction,
    pub create_pr: bool,
    pub fail_on_warning: bool,
    pub target_level: ComplianceLevel,
}

impl Default for BotConfig {
    fn default() -> Self {
        Self {
            action: BotAction::Check,
            create_pr: false,
            fail_on_warning: false,
            target_level: ComplianceLevel::Bronze,
        }
    }
}

/// Generate RSR badge markdown
pub fn generate_badge(level: ComplianceLevel) -> String {
    format!(
        "[![Rhodium Standard {}](https://img.shields.io/badge/RSR-{}-{})](https://github.com/hyperpolymath/rhodium-standard-repositories)",
        level.display_name(),
        level.display_name(),
        level.badge_color()
    )
}

/// Generate RSR conformity document
pub fn generate_conformity_doc(report: &ComplianceReport) -> String {
    let level = report.highest_level();
    let level_str = level.map(|l| l.display_name()).unwrap_or("Not Met");
    let timestamp = format_timestamp(report.verified_at);

    let mut doc = String::new();
    doc.push_str("# RSR Conformity Statement\n\n");
    doc.push_str(&format!(
        "**Project**: {}\n",
        report
            .repository_path
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_else(|| "Unknown".to_string())
    ));
    doc.push_str(&format!("**RSR Level**: {}\n", level_str));
    doc.push_str("**Standard**: [Rhodium Standard Repository](https://github.com/hyperpolymath/rhodium-standard-repositories)\n");
    doc.push_str(&format!(
        "**Last Verified**: {}\n\n",
        timestamp.split('T').next().unwrap_or(&timestamp)
    ));

    if let Some(l) = level {
        doc.push_str(&format!("## {} Requirements Met\n\n", l.display_name()));
        doc.push_str("| Requirement | Status |\n");
        doc.push_str("|-------------|--------|\n");
        for check in &report.checks {
            if check.required_for == l {
                let status = if check.passed { "Yes" } else { "No" };
                doc.push_str(&format!("| {} | {} |\n", check.item, status));
            }
        }
    }

    doc.push_str("\n## Verification\n\n");
    doc.push_str("Run self-verification:\n");
    doc.push_str("```bash\n");
    doc.push_str("rhodibot check .\n");
    doc.push_str("```\n\n");
    doc.push_str(&format!(
        "Expected output: `{}/{} checks passed ({:.1}%)`\n",
        report.passed_count(),
        report.total_count(),
        report.percentage()
    ));

    doc
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compliance_report_creation() {
        let path = PathBuf::from("/tmp/test");
        let report = ComplianceReport::new(path.clone());
        assert_eq!(report.repository_path, path);
        assert_eq!(report.checks.len(), 0);
    }

    #[test]
    fn test_add_check() {
        let mut report = ComplianceReport::new(PathBuf::from("/tmp/test"));
        report.add_check("Test", "Item", true, ComplianceLevel::Bronze);
        assert_eq!(report.checks.len(), 1);
        assert!(report.checks[0].passed);
    }

    #[test]
    fn test_bronze_compliance_all_passing() {
        let mut report = ComplianceReport::new(PathBuf::from("/tmp/test"));
        report.add_check("Test", "Item1", true, ComplianceLevel::Bronze);
        report.add_check("Test", "Item2", true, ComplianceLevel::Bronze);
        assert!(report.bronze_compliance());
    }

    #[test]
    fn test_bronze_compliance_one_failing() {
        let mut report = ComplianceReport::new(PathBuf::from("/tmp/test"));
        report.add_check("Test", "Item1", true, ComplianceLevel::Bronze);
        report.add_check("Test", "Item2", false, ComplianceLevel::Bronze);
        assert!(!report.bronze_compliance());
    }

    #[test]
    fn test_compliance_level_badge_colors() {
        assert_eq!(ComplianceLevel::Bronze.badge_color(), "cd7f32");
        assert_eq!(ComplianceLevel::Silver.badge_color(), "c0c0c0");
        assert_eq!(ComplianceLevel::Gold.badge_color(), "ffd700");
    }

    #[test]
    fn test_add_warning() {
        let mut report = ComplianceReport::new(PathBuf::from("/tmp/test"));
        report.add_warning(WarningLevel::Info, "Test warning", None);
        assert_eq!(report.warnings.len(), 1);
        assert_eq!(report.warnings[0].level, WarningLevel::Info);
    }

    #[test]
    fn test_critical_warnings_detection() {
        let mut report = ComplianceReport::new(PathBuf::from("/tmp/test"));
        report.add_warning(WarningLevel::Info, "Info warning", None);
        assert!(!report.has_critical_warnings());

        report.add_warning(WarningLevel::Critical, "Critical warning", None);
        assert!(report.has_critical_warnings());
    }

    #[test]
    fn test_generate_badge() {
        let badge = generate_badge(ComplianceLevel::Bronze);
        assert!(badge.contains("RSR-Bronze"));
        assert!(badge.contains("cd7f32"));
    }

    #[test]
    fn test_format_timestamp() {
        use std::time::Duration;
        let time = SystemTime::UNIX_EPOCH + Duration::from_secs(1705322445);
        let formatted = format_timestamp(time);
        assert!(formatted.contains("2024"));
        assert!(formatted.ends_with("Z"));
    }

    #[test]
    fn test_json_escape() {
        assert_eq!(json_escape("hello"), "hello");
        assert_eq!(json_escape("he\"llo"), "he\\\"llo");
        assert_eq!(json_escape("he\\llo"), "he\\\\llo");
        assert_eq!(json_escape("he\nllo"), "he\\nllo");
    }

    #[test]
    fn test_generate_sarif_basic_structure() {
        let mut report = ComplianceReport::new(PathBuf::from("/tmp/test"));
        report.add_check("Documentation", "README.md", true, ComplianceLevel::Bronze);
        report.add_check(
            "Documentation",
            "LICENSE.txt",
            false,
            ComplianceLevel::Bronze,
        );

        let sarif = generate_sarif(&report);

        // Verify SARIF structure
        assert!(sarif.contains("\"$schema\": \"https://raw.githubusercontent.com/oasis-tcs/sarif-spec/master/Schemata/sarif-schema-2.1.0.json\""));
        assert!(sarif.contains("\"version\": \"2.1.0\""));
        assert!(sarif.contains("\"name\": \"rhodibot\""));
        assert!(sarif.contains("\"runs\":"));
        assert!(sarif.contains("\"tool\":"));
        assert!(sarif.contains("\"results\":"));
    }

    #[test]
    fn test_generate_sarif_with_failed_checks() {
        let mut report = ComplianceReport::new(PathBuf::from("/tmp/test"));
        report.add_check("Documentation", "README.md", false, ComplianceLevel::Bronze);

        let sarif = generate_sarif(&report);

        // Should contain a result for the failed check
        assert!(sarif.contains("\"ruleId\": \"Documentation/README.md\""));
        assert!(sarif.contains("Missing required file or directory: README.md"));
    }

    #[test]
    fn test_generate_sarif_with_warnings() {
        let mut report = ComplianceReport::new(PathBuf::from("/tmp/test"));
        report.add_check("Documentation", "README.md", true, ComplianceLevel::Bronze);
        report.add_warning(
            WarningLevel::Critical,
            "Symlink points outside repository",
            Some(PathBuf::from("/tmp/test/link")),
        );

        let sarif = generate_sarif(&report);

        // Should contain the warning
        assert!(sarif.contains("security/warning"));
        assert!(sarif.contains("Symlink points outside repository"));
    }

    #[test]
    fn test_generate_sarif_compliant_repo() {
        let mut report = ComplianceReport::new(PathBuf::from("/tmp/test"));
        report.add_check("Documentation", "README.md", true, ComplianceLevel::Bronze);
        report.add_check(
            "Documentation",
            "LICENSE.txt",
            true,
            ComplianceLevel::Bronze,
        );

        let sarif = generate_sarif(&report);

        // executionSuccessful should be true for compliant repos
        assert!(sarif.contains("\"executionSuccessful\": true"));
    }

    #[test]
    fn test_generate_sarif_non_compliant_repo() {
        let mut report = ComplianceReport::new(PathBuf::from("/tmp/test"));
        report.add_check("Documentation", "README.md", false, ComplianceLevel::Bronze);

        let sarif = generate_sarif(&report);

        // executionSuccessful should be false for non-compliant repos
        assert!(sarif.contains("\"executionSuccessful\": false"));
    }

    #[test]
    fn test_generate_conformity_doc() {
        let mut report = ComplianceReport::new(PathBuf::from("/tmp/test-project"));
        report.add_check("Documentation", "README.md", true, ComplianceLevel::Bronze);

        let doc = generate_conformity_doc(&report);

        assert!(doc.contains("# RSR Conformity Statement"));
        assert!(doc.contains("**RSR Level**:"));
        assert!(doc.contains("rhodibot check ."));
    }
}
