//! Bot integration module for CI/CD platforms
//!
//! This module provides integration with various CI/CD platforms:
//! - GitHub Actions
//! - GitLab CI
//! - Generic CI environments

use crate::{ComplianceReport, WarningLevel};
use std::env;

/// Detected CI/CD platform
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CIPlatform {
    GitHubActions,
    GitLabCI,
    CircleCI,
    Travis,
    Jenkins,
    Unknown,
}

impl CIPlatform {
    /// Detect the current CI/CD platform from environment variables
    pub fn detect() -> Self {
        if env::var("GITHUB_ACTIONS").is_ok() {
            CIPlatform::GitHubActions
        } else if env::var("GITLAB_CI").is_ok() {
            CIPlatform::GitLabCI
        } else if env::var("CIRCLECI").is_ok() {
            CIPlatform::CircleCI
        } else if env::var("TRAVIS").is_ok() {
            CIPlatform::Travis
        } else if env::var("JENKINS_URL").is_ok() {
            CIPlatform::Jenkins
        } else {
            CIPlatform::Unknown
        }
    }

    /// Get a human-readable name for the platform
    pub fn name(&self) -> &'static str {
        match self {
            CIPlatform::GitHubActions => "GitHub Actions",
            CIPlatform::GitLabCI => "GitLab CI",
            CIPlatform::CircleCI => "CircleCI",
            CIPlatform::Travis => "Travis CI",
            CIPlatform::Jenkins => "Jenkins",
            CIPlatform::Unknown => "Unknown",
        }
    }
}

/// GitHub Actions specific output commands
pub mod github_actions {
    use super::*;

    /// Set an output variable for GitHub Actions
    pub fn set_output(name: &str, value: &str) {
        // GitHub Actions uses GITHUB_OUTPUT file since Oct 2022
        if let Ok(output_file) = env::var("GITHUB_OUTPUT") {
            if let Ok(mut file) = std::fs::OpenOptions::new().append(true).open(output_file) {
                use std::io::Write;
                let _ = writeln!(file, "{}={}", name, value);
            }
        } else {
            // Fallback to deprecated set-output command
            println!("::set-output name={}::{}", name, value);
        }
    }

    /// Add a warning annotation
    pub fn warning(message: &str, file: Option<&str>, line: Option<u32>) {
        let mut cmd = "::warning".to_string();
        if let Some(f) = file {
            cmd.push_str(&format!(" file={}", f));
            if let Some(l) = line {
                cmd.push_str(&format!(",line={}", l));
            }
        }
        cmd.push_str(&format!("::{}", message));
        println!("{}", cmd);
    }

    /// Add an error annotation
    pub fn error(message: &str, file: Option<&str>, line: Option<u32>) {
        let mut cmd = "::error".to_string();
        if let Some(f) = file {
            cmd.push_str(&format!(" file={}", f));
            if let Some(l) = line {
                cmd.push_str(&format!(",line={}", l));
            }
        }
        cmd.push_str(&format!("::{}", message));
        println!("{}", cmd);
    }

    /// Start a log group
    pub fn group(title: &str) {
        println!("::group::{}", title);
    }

    /// End a log group
    pub fn endgroup() {
        println!("::endgroup::");
    }

    /// Create a job summary entry
    pub fn summary(markdown: &str) {
        if let Ok(summary_file) = env::var("GITHUB_STEP_SUMMARY") {
            if let Ok(mut file) = std::fs::OpenOptions::new()
                .append(true)
                .create(true)
                .open(summary_file)
            {
                use std::io::Write;
                let _ = writeln!(file, "{}", markdown);
            }
        }
    }

    /// Output report as GitHub Actions annotations and summary
    pub fn output_report(report: &ComplianceReport) {
        // Set outputs
        set_output("passed", &report.passed_count().to_string());
        set_output("total", &report.total_count().to_string());
        set_output("percentage", &format!("{:.1}", report.percentage()));
        set_output("bronze_compliant", &report.bronze_compliance().to_string());
        set_output("has_warnings", &report.has_critical_warnings().to_string());

        // Output annotations for failed checks
        for check in &report.checks {
            if !check.passed {
                warning(
                    &format!("RSR check failed: {} - {}", check.category, check.item),
                    None,
                    None,
                );
            }
        }

        // Output annotations for security warnings
        for warning_item in &report.warnings {
            let file = warning_item.path.as_ref().map(|p| p.to_string_lossy());
            match warning_item.level {
                WarningLevel::Critical => {
                    error(&warning_item.message, file.as_deref(), None);
                },
                _ => {
                    warning(&warning_item.message, file.as_deref(), None);
                },
            }
        }

        // Generate job summary
        let mut md = String::new();
        md.push_str("## 🤖 Rhodibot RSR Compliance Report\n\n");

        if report.bronze_compliance() && !report.has_critical_warnings() {
            md.push_str("✅ **Bronze-level RSR compliance: ACHIEVED**\n\n");
        } else {
            md.push_str("❌ **Bronze-level RSR compliance: NOT MET**\n\n");
        }

        md.push_str(&format!(
            "**Score**: {}/{} checks passed ({:.1}%)\n\n",
            report.passed_count(),
            report.total_count(),
            report.percentage()
        ));

        md.push_str("### Checks\n\n");
        md.push_str("| Category | Item | Status |\n");
        md.push_str("|----------|------|--------|\n");
        for check in &report.checks {
            let status = if check.passed { "✅" } else { "❌" };
            md.push_str(&format!(
                "| {} | {} | {} |\n",
                check.category, check.item, status
            ));
        }

        if !report.warnings.is_empty() {
            md.push_str("\n### Security Warnings\n\n");
            for warning_item in &report.warnings {
                let icon = match warning_item.level {
                    WarningLevel::Info => "ℹ️",
                    WarningLevel::Warning => "⚠️",
                    WarningLevel::Critical => "🚨",
                };
                md.push_str(&format!("- {} {}\n", icon, warning_item.message));
            }
        }

        summary(&md);
    }
}

/// GitLab CI specific output
pub mod gitlab_ci {
    use super::*;

    /// Output report in GitLab CI compatible format
    pub fn output_report(report: &ComplianceReport) {
        // GitLab uses dotenv artifacts for passing variables
        // Output to console in a parseable format
        println!("RHODIBOT_PASSED={}", report.passed_count());
        println!("RHODIBOT_TOTAL={}", report.total_count());
        println!("RHODIBOT_PERCENTAGE={:.1}", report.percentage());
        println!("RHODIBOT_BRONZE_COMPLIANT={}", report.bronze_compliance());
        println!("RHODIBOT_HAS_WARNINGS={}", report.has_critical_warnings());

        // Output sections
        println!("\n\\e[0Ksection_start:{}:rhodibot_report[collapsed=false]\\r\\e[0K\x1b[36mRhodibot Report\x1b[0m",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs()
        );

        for check in &report.checks {
            let status = if check.passed { "✓" } else { "✗" };
            let color = if check.passed { "32" } else { "31" };
            println!(
                "\x1b[{}m[{}]\x1b[0m {} - {}",
                color, status, check.category, check.item
            );
        }

        println!(
            "section_end:{}:rhodibot_report\\r\\e[0K",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs()
        );
    }
}

/// Generate GitHub Actions workflow file
pub fn generate_github_actions_workflow() -> String {
    r#"# Rhodibot RSR Compliance Check
# This workflow checks your repository for Rhodium Standard Repository compliance

name: RSR Compliance

on:
  push:
    branches: [main, master]
  pull_request:
    branches: [main, master]
  schedule:
    # Run weekly on Mondays at 00:00 UTC
    - cron: '0 0 * * 1'

jobs:
  rhodibot:
    name: RSR Compliance Check
    runs-on: ubuntu-latest
    steps:
      - name: Checkout repository
        uses: actions/checkout@v4

      - name: Install Rust
        uses: dtolnay/rust-action@stable

      - name: Install Rhodibot
        run: cargo install rhodibot

      - name: Run RSR compliance check
        id: check
        run: |
          rhodibot check . --format json > rhodibot-report.json
          rhodibot check .
        continue-on-error: true

      - name: Generate badge
        run: rhodibot badge > RSR_BADGE.md

      - name: Upload report
        uses: actions/upload-artifact@v4
        with:
          name: rhodibot-report
          path: rhodibot-report.json

      - name: Check result
        if: steps.check.outcome == 'failure'
        run: |
          echo "RSR compliance check failed!"
          exit 1
"#
    .to_string()
}

/// Generate GitLab CI configuration
pub fn generate_gitlab_ci_config() -> String {
    r#"# Rhodibot RSR Compliance Check
# Add this to your .gitlab-ci.yml

rhodibot:
  stage: test
  image: rust:latest
  before_script:
    - cargo install rhodibot
  script:
    - rhodibot check . --format json > rhodibot-report.json
    - rhodibot check .
  artifacts:
    reports:
      dotenv: rhodibot.env
    paths:
      - rhodibot-report.json
    when: always
  allow_failure: false
  rules:
    - if: $CI_PIPELINE_SOURCE == "merge_request_event"
    - if: $CI_COMMIT_BRANCH == $CI_DEFAULT_BRANCH
    - if: $CI_PIPELINE_SOURCE == "schedule"
"#
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ci_platform_detection() {
        // In test environment, should be Unknown unless in CI
        let platform = CIPlatform::detect();
        // Can't guarantee which platform we're on, just test it doesn't panic
        let _ = platform.name();
    }

    #[test]
    fn test_generate_github_workflow() {
        let workflow = generate_github_actions_workflow();
        assert!(workflow.contains("rhodibot"));
        assert!(workflow.contains("actions/checkout"));
    }

    #[test]
    fn test_generate_gitlab_config() {
        let config = generate_gitlab_ci_config();
        assert!(config.contains("rhodibot"));
        assert!(config.contains("stage: test"));
    }
}
