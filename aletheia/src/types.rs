// SPDX-License-Identifier: PMPL-1.0-or-later

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
#[derive(Debug)]
pub struct CheckResult {
    pub category: String, // e.g. "Documentation", "Build System"
    pub item: String,     // e.g. "README.md"
    pub passed: bool,
    pub required_for: ComplianceLevel,
    pub suggestion: Option<String>, // Remediation hint for tutors/developers.
}

/// AUDIT REPORT: The consolidated results of a repository audit.
pub struct ComplianceReport {
    pub repository_path: PathBuf,
    pub verified_at: SystemTime,
    pub checks: Vec<CheckResult>,
    pub warnings: Vec<SecurityWarning>,
}
