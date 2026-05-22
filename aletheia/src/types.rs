// SPDX-License-Identifier: MPL-2.0

//! Aletheia — Core Domain Types and Models.
//!
//! This module defines the formal data structures used throughout the 
//! compliance verification engine. It establishes the schema for 
//! audit results, security warnings, and RSR compliance tiers.

use std::path::PathBuf;
use std::time::SystemTime;

/// COMPLIANCE TIERS: Authoritative RSR assurance levels.
/// - **Bronze**: Basic documentation and security baselines.
/// - **Silver**: Advanced content validation and workflow pinning.
/// - **Gold/Platinum**: Reserved for deep formal verification.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComplianceLevel {
    Bronze, Silver, Gold, Platinum,
}

/// CHECK RESULT: The outcome of a single deterministic verification item.
#[derive(Debug, Clone)]
pub struct CheckResult {
    pub category: String, // e.g. "Documentation", "Build System"
    pub item: String,     // e.g. "README.md"
    pub passed: bool,
    pub required_for: ComplianceLevel,
    pub suggestion: Option<String>, // Remediation hint for tutors/developers.
}

/// PATH SECURITY CHECK RESULT: Validates symlink safety.
#[derive(Debug, Clone, Default)]
pub struct PathCheckResult {
    pub exists: bool,
    pub is_symlink: bool,
    pub escapes_repo: bool,
    pub target: Option<PathBuf>,
}

/// SECURITY WARNING: Detailed information about a security concern.
#[derive(Debug, Clone)]
pub struct SecurityWarning {
    pub level: String,
    pub message: String,
    pub path: Option<PathBuf>,
}

/// AUDIT REPORT: The consolidated results of a repository audit.
pub struct ComplianceReport {
    pub repository_path: PathBuf,
    pub verified_at: SystemTime,
    pub checks: Vec<CheckResult>,
    pub warnings: Vec<SecurityWarning>,
}

impl ComplianceReport {
    /// Create a new compliance report for a given repository path.
    pub fn new(repo_path: PathBuf) -> Self {
        ComplianceReport {
            repository_path: repo_path,
            verified_at: SystemTime::now(),
            checks: Vec::new(),
            warnings: Vec::new(),
        }
    }

    /// Add a check result to the report.
    pub fn add_check(&mut self, category: &str, item: &str, passed: bool, level: ComplianceLevel) {
        self.checks.push(CheckResult {
            category: category.to_string(),
            item: item.to_string(),
            passed,
            required_for: level,
            suggestion: None,
        });
    }

    /// Add a security warning to the report.
    pub fn add_warning(&mut self, level: &str, message: String, path: Option<PathBuf>) {
        self.warnings.push(SecurityWarning {
            level: level.to_string(),
            message,
            path,
        });
    }
}
