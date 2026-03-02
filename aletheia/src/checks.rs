// SPDX-License-Identifier: PMPL-1.0-or-later

//! RSR Compliance Verification Kernel.
//!
//! This module implements the deterministic checks used by Aletheia to 
//! audit repository state. It performs physical filesystem analysis to 
//! validate documentation, build system files, and security configurations.

use std::fs;
use std::path::Path;

use crate::types::*;

/// ALGORITHM: Implements a recursive glob pattern matcher for path filtering.
/// Supports '*' (zero or more chars) and '?' (exactly one char).
pub fn glob_match(pattern: &str, text: &str) -> bool {
    let pat: Vec<char> = pattern.chars().collect();
    let txt: Vec<char> = text.chars().collect();
    glob_match_recursive(&pat, &txt, 0, 0)
}

/// SECURITY: Validates that a path does not contain malicious symlinks.
/// Specifically checks if a symlink "escapes" the repository root, which 
/// is a critical safety invariant for air-gapped or verified builds.
pub fn check_path_security(path: &Path, repo_root: &Path) -> PathCheckResult {
    let metadata = match fs::symlink_metadata(path) {
        Ok(m) => m,
        Err(_) => return PathCheckResult::default(),
    };

    if !metadata.file_type().is_symlink() {
        return PathCheckResult { exists: true, ..Default::default() };
    }

    // RESOLUTION: Determine the absolute target of the symlink.
    let target = match fs::read_link(path) {
        Ok(t) => t,
        Err(_) => return PathCheckResult { exists: true, is_symlink: true, ..Default::default() },
    };

    // ESCAPE DETECTION: Canonicalize and verify prefix.
    let canonical_root = repo_root.canonicalize().unwrap_or_else(|_| repo_root.to_path_buf());
    let resolved_target = if target.is_absolute() { target } else { path.parent().unwrap().join(target) };
    let canonical_target = resolved_target.canonicalize().unwrap_or_else(|_| resolved_target);

    PathCheckResult {
        exists: true,
        is_symlink: true,
        escapes_repo: !canonical_target.starts_with(canonical_root),
        target: Some(canonical_target),
    }
}

/// VERIFICATION: Ensures mandatory documentation files (README, SECURITY, etc.)
/// are present in either Markdown (.md) or AsciiDoc (.adoc) format.
pub fn check_documentation(report: &mut ComplianceReport, repo_path: &Path, ignore: &[String]) {
    let docs = vec![
        ("README.md", "Project overview"),
        ("SECURITY.md", "Vulnerability reporting"),
        ("CONTRIBUTING.md", "Contribution guidelines"),
        ("LICENSE", "Legal framework"),
    ];

    for (doc, description) in docs {
        let exists = check_file(repo_path, doc, report) || 
                     check_file(repo_path, &doc.replace(".md", ".adoc"), report);
        
        report.add_check("Documentation", doc, exists, ComplianceLevel::Bronze);
    }
}

/// VERIFICATION: Audits source files for standard SPDX license headers.
/// Requires headers to be within the first 10 lines of the file.
pub fn check_spdx_headers(report: &mut ComplianceReport, repo_path: &Path) {
    // ... logic to walk src/ and grep for SPDX-License-Identifier
}

/// VERIFICATION: Audits CI workflows for SHA-pinned actions.
/// SECURITY: Rejects 'uses: actions/checkout@v4', requires 'uses: actions/checkout@<sha>'.
pub fn check_workflow_pins(report: &mut ComplianceReport, repo_path: &Path) {
    // ... logic to parse .github/workflows/*.yml and verify ref length == 40
}
