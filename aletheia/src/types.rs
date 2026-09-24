// SPDX-License-Identifier: MPL-2.0
// Copyright (c) Jonathan D.A. Jewell <j.d.a.jewell@open.ac.uk>
//! Aletheia — Core Domain Types and Models.
//!
//! This module defines the formal data structures used throughout the
//! compliance verification engine. It establishes the schema for
//! audit results, security warnings, and RSR compliance tiers.
//!
//! TIER VOCABULARY: [`ComplianceLevel`] follows the estate-wide RSR tier
//! ladder (Bronze → Silver → Gold → Platinum/Rhodium). The *normative*
//! definition of what each tier requires lives upstream in
//! `hyperpolymath/standards` (`0-canon/rsr/rsr-criteria-v2.a2ml`) and is
//! scored by the hypatia `rsr-conformance` oracle. Aletheia evaluates a
//! documented *local file-presence subset* of those criteria — see
//! `checks::SSOT_PROVENANCE`. It is a non-normative convenience checker
//! in the same class as the rhodium-pipeline rsr-certifier (criteria
//! file `[oracle]` section: "MAY consume this file but is not normative").

use std::path::PathBuf;
use std::time::SystemTime;

/// COMPLIANCE TIERS: RSR assurance levels, lowest to highest.
///
/// - **Bronze**: foundational documentation, security and build baselines.
/// - **Silver**: enhanced quality (provenance, pinning, machine-readable substrate).
/// - **Gold**: production readiness (release hygiene, hardened enforcement).
/// - **Platinum**: estate-rhodium exemplary tier (proofs, succession, signed history).
///
/// All four variants are constructed: [`ComplianceLevel::all`] feeds the
/// per-tier summary, and [`ComplianceLevel::from_v2_tier`] maps the
/// upstream tier vocabulary (including `rhodium`, the v2 name for this
/// tier) onto it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ComplianceLevel {
    Bronze,
    Silver,
    Gold,
    Platinum,
}

impl ComplianceLevel {
    /// All levels, lowest to highest. Used by the per-tier score summary
    /// so every tier is always reported, even with zero local checks.
    pub fn all() -> [ComplianceLevel; 4] {
        [
            ComplianceLevel::Bronze,
            ComplianceLevel::Silver,
            ComplianceLevel::Gold,
            ComplianceLevel::Platinum,
        ]
    }

    /// Severity rank: higher tiers build on lower ones.
    pub fn rank(self) -> u8 {
        match self {
            ComplianceLevel::Bronze => 0,
            ComplianceLevel::Silver => 1,
            ComplianceLevel::Gold => 2,
            ComplianceLevel::Platinum => 3,
        }
    }

    /// Short display name, matching the RSR badge vocabulary.
    pub fn name(self) -> &'static str {
        match self {
            ComplianceLevel::Bronze => "Bronze",
            ComplianceLevel::Silver => "Silver",
            ComplianceLevel::Gold => "Gold",
            ComplianceLevel::Platinum => "Platinum",
        }
    }

    /// Map the upstream criteria-file tier vocabulary onto this enum.
    ///
    /// Accepts `bronze`, `silver`, `gold`, `platinum` and `rhodium` (the
    /// v2 name for the exemplary tier). Case-insensitive. Returns `None`
    /// for anything else so callers can fall back honestly instead of
    /// guessing.
    pub fn from_v2_tier(tier: &str) -> Option<ComplianceLevel> {
        if tier.eq_ignore_ascii_case("bronze") {
            Some(ComplianceLevel::Bronze)
        } else if tier.eq_ignore_ascii_case("silver") {
            Some(ComplianceLevel::Silver)
        } else if tier.eq_ignore_ascii_case("gold") {
            Some(ComplianceLevel::Gold)
        } else if tier.eq_ignore_ascii_case("platinum") || tier.eq_ignore_ascii_case("rhodium") {
            Some(ComplianceLevel::Platinum)
        } else {
            None
        }
    }
}

/// CHECK RESULT: the outcome of a single deterministic verification item.
///
/// `id` is a stable machine key (e.g. `readme`, `sha-pinned`) used by
/// `[checks]` config toggles, JSON output and SARIF `rsr/<id>` rule ids.
/// `suggestion` carries the remediation hint shown in verbose output and
/// the "Fix Suggestions" section; it is `Some` exactly when `passed` is
/// false (a passing check has nothing to remediate).
#[derive(Debug, Clone)]
pub struct CheckResult {
    pub id: String,
    pub category: String, // e.g. "Documentation", "Security"
    pub item: String,     // e.g. "README.adoc (or README.md)"
    pub passed: bool,
    pub required_for: ComplianceLevel,
    pub suggestion: Option<String>,
}

/// PATH SECURITY CHECK RESULT: validates symlink safety.
#[derive(Debug, Clone, Default)]
pub struct PathCheckResult {
    pub exists: bool,
    pub is_symlink: bool,
    pub escapes_repo: bool,
    pub target: Option<PathBuf>,
}

/// SECURITY WARNING: detailed information about a security concern.
///
/// `path` is `Some` whenever the warning concerns a specific filesystem
/// entry (e.g. an escaping symlink) so reports can point at it.
#[derive(Debug, Clone)]
pub struct SecurityWarning {
    pub level: String,
    pub message: String,
    pub path: Option<PathBuf>,
}

/// AUDIT REPORT: the consolidated results of a repository audit.
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
    ///
    /// `suggestion` should be `Some` when (and only when) `passed` is
    /// false; see [`CheckResult::suggestion`].
    pub fn add_check(
        &mut self,
        id: &str,
        category: &str,
        item: &str,
        passed: bool,
        level: ComplianceLevel,
        suggestion: Option<String>,
    ) {
        self.checks.push(CheckResult {
            id: id.to_string(),
            category: category.to_string(),
            item: item.to_string(),
            passed,
            required_for: level,
            suggestion,
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

    /// Number of passing checks.
    pub fn passed_count(&self) -> usize {
        self.checks.iter().filter(|c| c.passed).count()
    }

    /// Total number of checks evaluated.
    pub fn total_count(&self) -> usize {
        self.checks.len()
    }

    /// Score as a percentage of evaluated checks passed.
    ///
    /// Informational only: the exit policy is strict per-tier (see
    /// [`ComplianceReport::compliant_at`]), not score-based, because the
    /// local subset cannot reproduce the oracle's capability-gated
    /// weighting. An empty report scores 100.0 (nothing failed).
    pub fn score_pct(&self) -> f64 {
        if self.checks.is_empty() {
            100.0
        } else {
            self.passed_count() as f64 / self.checks.len() as f64 * 100.0
        }
    }

    /// Per-tier `(level, passed, total)` triples, lowest tier first.
    pub fn tier_counts(&self) -> Vec<(ComplianceLevel, usize, usize)> {
        ComplianceLevel::all()
            .iter()
            .map(|level| {
                let total = self
                    .checks
                    .iter()
                    .filter(|c| c.required_for == *level)
                    .count();
                let passed = self
                    .checks
                    .iter()
                    .filter(|c| c.required_for == *level && c.passed)
                    .count();
                (*level, passed, total)
            })
            .collect()
    }

    /// Strict tier compliance: every check required at or below `level`
    /// must pass. Tiers build on each other, so Silver compliance implies
    /// Bronze compliance.
    pub fn compliant_at(&self, level: ComplianceLevel) -> bool {
        let max_rank = level.rank();
        !self
            .checks
            .iter()
            .any(|c| c.required_for.rank() <= max_rank && !c.passed)
    }

    /// Whether any `critical`-level security warning was recorded.
    pub fn has_critical_warnings(&self) -> bool {
        self.warnings.iter().any(|w| w.level == "critical")
    }

    /// Failing checks, for suggestions and SARIF results.
    pub fn failing_checks(&self) -> Vec<&CheckResult> {
        self.checks.iter().filter(|c| !c.passed).collect()
    }
}
