// SPDX-License-Identifier: MPL-2.0
// Copyright (c) Jonathan D.A. Jewell <j.d.a.jewell@open.ac.uk>
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
        return PathCheckResult {
            exists: true,
            ..Default::default()
        };
    }

    // RESOLUTION: Determine the absolute target of the symlink.
    let target = match fs::read_link(path) {
        Ok(t) => t,
        Err(_) => {
            return PathCheckResult {
                exists: true,
                is_symlink: true,
                ..Default::default()
            }
        },
    };

    // ESCAPE DETECTION: Canonicalize and verify prefix.
    let canonical_root = repo_root
        .canonicalize()
        .unwrap_or_else(|_| repo_root.to_path_buf());
    let resolved_target = if target.is_absolute() {
        target
    } else {
        path.parent().expect("TODO: handle error").join(target)
    };
    let canonical_target = resolved_target
        .canonicalize()
        .unwrap_or_else(|_| resolved_target);

    PathCheckResult {
        exists: true,
        is_symlink: true,
        escapes_repo: !canonical_target.starts_with(canonical_root),
        target: Some(canonical_target),
    }
}

/// VERIFICATION: Ensures mandatory documentation files (README, SECURITY, etc.)
/// are present in either Markdown (.md) or AsciiDoc (.adoc) format.
pub fn check_documentation(report: &mut ComplianceReport, repo_path: &Path, _ignore: &[String]) {
    let docs = vec![
        ("README.md", "Project overview"),
        ("SECURITY.md", "Vulnerability reporting"),
        ("CONTRIBUTING.md", "Contribution guidelines"),
        ("LICENSE", "Legal framework"),
    ];

    for (doc, _description) in docs {
        let md_path = repo_path.join(doc);
        let adoc_path = repo_path.join(doc.replace(".md", ".adoc"));
        let exists = md_path.is_file() || adoc_path.is_file();

        report.add_check("Documentation", doc, exists, ComplianceLevel::Bronze);
    }
}

/// VERIFICATION: Audits source files for standard SPDX license headers.
/// Requires headers to be within the first 10 lines of the file.
pub fn check_spdx_headers(report: &mut ComplianceReport, repo_path: &Path) {
    let src_path = repo_path.join("src");
    if src_path.is_dir() {
        if let Ok(entries) = fs::read_dir(&src_path) {
            let mut checked = 0;
            let mut valid = 0;

            for entry in entries {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    if path.extension().map_or(false, |ext| ext == "rs") {
                        checked += 1;
                        if let Ok(content) = fs::read_to_string(&path) {
                            let first_10_lines: String =
                                content.lines().take(10).collect::<Vec<_>>().join("\n");
                            if first_10_lines.contains("SPDX-License-Identifier") {
                                valid += 1;
                            }
                        }
                    }
                }
            }

            let all_valid = checked == 0 || checked == valid;
            report.add_check(
                "Security",
                "SPDX headers in src/",
                all_valid,
                ComplianceLevel::Bronze,
            );
        }
    }
}

/// VERIFICATION: Audits CI workflows for SHA-pinned actions.
/// SECURITY: Rejects 'uses: actions/checkout@v4', requires 'uses: actions/checkout@<sha>'.
pub fn check_workflow_pins(report: &mut ComplianceReport, repo_path: &Path) {
    let workflows_path = repo_path.join(".github/workflows");
    if workflows_path.is_dir() {
        if let Ok(entries) = fs::read_dir(&workflows_path) {
            let mut checked_files = 0;
            let mut valid_files = 0;

            for entry in entries {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    if path
                        .extension()
                        .map_or(false, |ext| ext == "yml" || ext == "yaml")
                    {
                        checked_files += 1;
                        if let Ok(content) = fs::read_to_string(&path) {
                            let has_unpinned =
                                content.lines().any(|line| !uses_line_is_pinned(line));
                            if !has_unpinned {
                                valid_files += 1;
                            }
                        }
                    }
                }
            }

            let all_pinned = checked_files == 0 || checked_files == valid_files;
            report.add_check(
                "Security",
                "GitHub Actions SHA pinning",
                all_pinned,
                ComplianceLevel::Silver,
            );
        }
    }
}

/// Helper: Check if a file exists at repo_path/filename
fn file_exists(repo_path: &Path, filename: &str) -> bool {
    repo_path.join(filename).is_file()
}

/// SECURITY: Decide whether one workflow line satisfies SHA pinning.
///
/// Returns `true` for any line that is not a `uses:` line, so callers can apply
/// this with `.any(|l| !uses_line_is_pinned(l))` over a whole file.
///
/// A `uses:` value is pinned only when the ref after the final `@` is exactly
/// 40 hexadecimal characters (a full-length Git SHA-1). `@v4`, `@main`,
/// `@master` and a bare action with no `@` at all are all unpinned.
///
/// Exempt (not pinnable, so treated as pinned):
///   - local actions and local reusable workflows — `./…`
///   - `docker://` image references, which use a different digest syntax
///
/// A trailing `# v4` provenance comment is ignored, so
/// `uses: actions/checkout@3d3c42e5… # v7.0.1` is correctly seen as pinned.
fn uses_line_is_pinned(line: &str) -> bool {
    let trimmed = line.trim();
    // Accept both `uses:` and list form `- uses:`.
    let rest = match trimmed
        .strip_prefix("uses:")
        .or_else(|| trimmed.strip_prefix("- uses:"))
    {
        Some(r) => r,
        None => return true, // not a uses: line — nothing to judge
    };

    // Strip the trailing provenance comment, then surrounding quotes.
    let value = rest.split('#').next().unwrap_or("").trim();
    let value = value.trim_matches(|c| c == '"' || c == '\'');

    if value.is_empty() {
        return true;
    }
    if value.starts_with("./") || value.starts_with(".\\") || value.starts_with("docker://") {
        return true;
    }

    match value.rsplit_once('@') {
        Some((_, git_ref)) => git_ref.len() == 40 && git_ref.chars().all(|c| c.is_ascii_hexdigit()),
        None => false, // no ref at all — unpinned
    }
}

/// Helper: Recursive implementation of glob_match.
fn glob_match_recursive(pattern: &[char], text: &[char], pi: usize, ti: usize) -> bool {
    if pi >= pattern.len() && ti >= text.len() {
        true
    } else if pi >= pattern.len() || ti >= text.len() {
        false
    } else if pattern[pi] == '*' {
        // '*' can match zero or more characters
        glob_match_recursive(pattern, text, pi + 1, ti)
            || glob_match_recursive(pattern, text, pi, ti + 1)
    } else if pattern[pi] == '?' || pattern[pi] == text[ti] {
        // '?' matches any single character, or literal match
        glob_match_recursive(pattern, text, pi + 1, ti + 1)
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_glob_match_literal() {
        assert!(glob_match("foo.rs", "foo.rs"));
        assert!(!glob_match("foo.rs", "bar.rs"));
    }

    #[test]
    fn test_glob_match_star() {
        assert!(glob_match("*.rs", "main.rs"));
        assert!(glob_match("*.rs", "lib.rs"));
        assert!(!glob_match("*.rs", "main.toml"));
    }

    #[test]
    fn test_glob_match_question() {
        assert!(glob_match("foo?.rs", "foo1.rs"));
        assert!(glob_match("foo?.rs", "foox.rs"));
        assert!(!glob_match("foo?.rs", "foo.rs"));
    }

    #[test]
    fn test_glob_match_complex() {
        assert!(glob_match("*.rs", "test.rs"));
        assert!(glob_match("src/*.rs", "src/main.rs"));
        // Note: simple glob_match doesn't handle nested paths like we expect
        // This is OK for basic matching. More complex patterns would need enhancement.
    }

    #[test]
    fn test_path_check_result_default() {
        let result: PathCheckResult = Default::default();
        assert!(!result.exists);
        assert!(!result.is_symlink);
        assert!(!result.escapes_repo);
        assert!(result.target.is_none());
    }

    #[test]
    fn test_check_result_creation() {
        let check = CheckResult {
            category: "Documentation".to_string(),
            item: "README.md".to_string(),
            passed: true,
            required_for: ComplianceLevel::Bronze,
            suggestion: None,
        };

        assert_eq!(check.category, "Documentation");
        assert_eq!(check.item, "README.md");
        assert!(check.passed);
        assert_eq!(check.required_for, ComplianceLevel::Bronze);
    }

    #[test]
    fn test_security_warning_creation() {
        let warning = SecurityWarning {
            level: "critical".to_string(),
            message: "Symlink escapes repository".to_string(),
            path: None,
        };

        assert_eq!(warning.level, "critical");
        assert!(!warning.message.is_empty());
    }

    #[test]
    fn test_compliance_report_new() {
        let path = PathBuf::from("/tmp/test-repo");
        let report = ComplianceReport::new(path.clone());

        assert_eq!(report.repository_path, path);
        assert!(report.checks.is_empty());
        assert!(report.warnings.is_empty());
    }

    #[test]
    fn test_compliance_report_add_check() {
        let path = PathBuf::from("/tmp/test-repo");
        let mut report = ComplianceReport::new(path);

        report.add_check("Documentation", "README.md", true, ComplianceLevel::Bronze);

        assert_eq!(report.checks.len(), 1);
        assert_eq!(report.checks[0].category, "Documentation");
        assert_eq!(report.checks[0].item, "README.md");
        assert!(report.checks[0].passed);
    }

    #[test]
    fn test_compliance_report_add_warning() {
        let path = PathBuf::from("/tmp/test-repo");
        let mut report = ComplianceReport::new(path);

        report.add_warning("warning", "Test warning".to_string(), None);

        assert_eq!(report.warnings.len(), 1);
        assert_eq!(report.warnings[0].level, "warning");
    }

    #[test]
    fn test_file_exists() {
        // Anchor on CARGO_MANIFEST_DIR rather than the process working directory:
        // the crate root is fixed at compile time, so this is deterministic no
        // matter where the test binary is invoked from.
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));

        assert!(
            file_exists(root, "Cargo.toml"),
            "Cargo.toml exists at the crate root"
        );
        assert!(!file_exists(root, "no-such-file.does-not-exist"));
        // `is_file()`, not `exists()` — a directory must not count as a file.
        assert!(!file_exists(root, "src"), "a directory is not a file");
    }

    #[test]
    fn test_uses_line_is_pinned_accepts_full_sha() {
        // Real pins taken from this repository's own workflows.
        assert!(uses_line_is_pinned(
            "      - uses: actions/checkout@3d3c42e5aac5ba805825da76410c181273ba90b1"
        ));
        // A trailing provenance comment must not defeat the check.
        assert!(uses_line_is_pinned(
            "        uses: actions/deploy-pages@cd2ce8fcbc39b97be8ca5fce6e763baed58fa128 # v5.0.0"
        ));
    }

    #[test]
    fn test_uses_line_is_pinned_rejects_tags_and_branches() {
        // REGRESSION: the previous implementation was `line.contains("@v")
        // && !line.contains("@")`, which is unsatisfiable — every one of these
        // was silently reported as pinned.
        assert!(!uses_line_is_pinned("        uses: actions/checkout@v4"));
        assert!(!uses_line_is_pinned(
            "      - uses: actions/checkout@v7.0.1"
        ));
        assert!(!uses_line_is_pinned("        uses: some/action@main"));
        // The exact line that broke Governance and CodeQL on this repo.
        assert!(!uses_line_is_pinned(
            "        uses: SonarSource/sonarqube-scan-action@master"
        ));
        // A short/abbreviated SHA is not a full-length pin.
        assert!(!uses_line_is_pinned("        uses: foo/bar@3d3c42e"));
        // No ref at all.
        assert!(!uses_line_is_pinned("        uses: foo/bar"));
    }

    #[test]
    fn test_uses_line_is_pinned_ignores_non_uses_and_exempt_forms() {
        assert!(uses_line_is_pinned("      - name: Checkout"));
        assert!(uses_line_is_pinned("        run: cargo test"));
        assert!(uses_line_is_pinned(""));
        // Local actions and local reusable workflows cannot be SHA-pinned.
        assert!(uses_line_is_pinned("      - uses: ./.github/actions/setup"));
        assert!(uses_line_is_pinned(
            "    uses: ./.github/workflows/reusable.yml"
        ));
        // Docker refs use a different digest syntax; out of scope.
        assert!(uses_line_is_pinned("        uses: docker://alpine:3.20"));
    }

    #[test]
    fn test_compliance_level_equality() {
        assert_eq!(ComplianceLevel::Bronze, ComplianceLevel::Bronze);
        assert_ne!(ComplianceLevel::Bronze, ComplianceLevel::Silver);
    }
}
