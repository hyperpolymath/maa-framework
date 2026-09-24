// SPDX-License-Identifier: MPL-2.0
// Copyright (c) Jonathan D.A. Jewell <j.d.a.jewell@open.ac.uk>
//! Integration tests for Aletheia RSR compliance verification.
//!
//! End-to-end coverage of the CLI surface: fixtures below encode the RSR
//! v2 file shape ( criteria `hyperpolymath/standards`
//! `0-canon/rsr/rsr-criteria-v2.a2ml`), NOT the retired 2025 v1 shape
//! (LICENSE.txt / justfile / flake.nix / .gitlab-ci.yml). Where v1 and
//! v2 conflict, a dedicated test pins the v2 behaviour.
//!
//! The binary under test is resolved via `CARGO_BIN_EXE_aletheia` (the
//! already-built binary), never via a nested `cargo run`.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// The already-built binary under test.
fn aletheia() -> Command {
    Command::new(env!("CARGO_BIN_EXE_aletheia"))
}

/// Helper to create a temporary test repository.
fn create_test_repo(name: &str) -> PathBuf {
    let pid = std::process::id();
    let test_dir = std::env::temp_dir().join(format!("aletheia_test_{pid}_{name}"));

    // Clean up if it exists
    if test_dir.exists() {
        fs::remove_dir_all(&test_dir).ok();
    }

    fs::create_dir_all(&test_dir).expect("Failed to create test directory");
    test_dir
}

/// Helper to create a file in the test repo.
fn create_file(base: &Path, path: &str, content: &str) {
    let file_path = base.join(path);

    // Create parent directories if needed
    if let Some(parent) = file_path.parent() {
        fs::create_dir_all(parent).ok();
    }

    fs::write(file_path, content).expect("Failed to create file");
}

const PINNED_WORKFLOW: &str = "name: CI\non: [push]\njobs:\n  t:\n    runs-on: ubuntu-latest\n    steps:\n      - uses: actions/checkout@3d3c42e5aac5ba805825da76410c181273ba90b1 # v7.0.1\n";

/// Create a fully compliant test repository (Bronze + Silver + Gold).
fn create_fully_compliant_repo(name: &str) -> PathBuf {
    let repo = create_test_repo(name);

    // Infrastructure / CI (Bronze + Silver)
    create_file(&repo, ".github/workflows/ci.yml", PINNED_WORKFLOW);
    create_file(&repo, ".github/workflows/hypatia-scan.yml", PINNED_WORKFLOW);
    create_file(&repo, ".github/workflows/governance.yml", PINNED_WORKFLOW);

    // Build system (Bronze + Silver)
    create_file(
        &repo,
        "Justfile",
        "build:\n\tcargo build\n\ntest:\n\tcargo test\n",
    );
    create_file(&repo, ".editorconfig", "root = true\n");
    create_file(&repo, ".tool-versions", "rust 1.80\n");

    // Documentation (Bronze)
    create_file(&repo, "README.adoc", "= Test Project\n");
    create_file(&repo, "LICENSE", "Mozilla Public License Version 2.0\n");
    create_file(
        &repo,
        "LICENSES/MPL-2.0.txt",
        "Mozilla Public License Version 2.0\n",
    );
    create_file(&repo, "SECURITY.md", "# Security Policy\n");
    create_file(&repo, ".gitignore", "target/\n");
    create_file(&repo, ".gitattributes", "*.rs text eol=lf\n");

    // Documentation (Silver)
    create_file(&repo, "CODE_OF_CONDUCT.md", "# Code of Conduct\n");
    create_file(&repo, "CONTRIBUTING.md", "# Contributing\n");
    create_file(&repo, "CHANGELOG.adoc", "== Changelog\n");

    // Well-known (Silver)
    create_file(
        &repo,
        ".well-known/security.txt",
        "Contact: security@example.org\n",
    );
    create_file(&repo, ".well-known/ai.txt", "# AI Policy\n");
    create_file(&repo, ".well-known/humans.txt", "# Humans\n");

    // Machine-readable (Silver)
    create_file(&repo, "0-AI-MANIFEST.a2ml", "# agent front door\n");
    create_file(
        &repo,
        ".machine_readable/rsr-profile.a2ml",
        "# capabilities\n",
    );

    // Source with SPDX header (Bronze)
    create_file(
        &repo,
        "src/main.rs",
        "// SPDX-License-Identifier: MPL-2.0\nfn main() {}\n",
    );

    repo
}

/// Test verification of a fully compliant repository.
#[test]
fn test_fully_compliant_repository() {
    let repo = create_fully_compliant_repo("compliant");

    let output = aletheia()
        .arg(repo.to_str().unwrap())
        .output()
        .expect("Failed to run aletheia");

    assert!(
        output.status.success(),
        "Fully compliant repository should pass verification: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("Bronze-level RSR compliance: ACHIEVED"),
        "Should achieve Bronze compliance"
    );
    assert!(
        stdout.contains("Silver-level RSR compliance: ACHIEVED"),
        "Should achieve Silver compliance"
    );
    assert!(stdout.contains("Score:"), "Should show score");

    fs::remove_dir_all(repo).ok();
}

/// Test verification of a partially compliant repository.
#[test]
fn test_partially_compliant_repository() {
    let repo = create_test_repo("partial");

    create_file(&repo, "README.md", "# Test Project");
    create_file(&repo, "LICENSE", "Mozilla Public License Version 2.0");
    create_file(&repo, "src/main.rs", "fn main() {}");

    let output = aletheia()
        .arg(repo.to_str().unwrap())
        .output()
        .expect("Failed to run aletheia");

    assert!(
        !output.status.success(),
        "Partially compliant repository should fail verification"
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("Bronze-level RSR compliance: NOT MET"),
        "Should not achieve Bronze compliance"
    );

    fs::remove_dir_all(repo).ok();
}

/// Test verification of empty repository.
#[test]
fn test_empty_repository() {
    let repo = create_test_repo("empty");

    let output = aletheia()
        .arg(repo.to_str().unwrap())
        .output()
        .expect("Failed to run aletheia");

    assert!(
        !output.status.success(),
        "Empty repository should fail verification"
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("Bronze-level RSR compliance: NOT MET"),
        "Should not meet Bronze compliance"
    );

    fs::remove_dir_all(repo).ok();
}

/// Test handling of non-existent path.
#[test]
fn test_nonexistent_path() {
    let output = aletheia()
        .arg("/nonexistent/path/that/does/not/exist")
        .output()
        .expect("Failed to run aletheia");

    assert!(!output.status.success());
    assert_eq!(output.status.code(), Some(3));

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("does not exist") || stderr.contains("Error"),
        "Should report path error"
    );
}

/// Test self-verification (Aletheia verifying itself).
#[test]
fn test_self_verification() {
    let output = aletheia()
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("Failed to run aletheia self-verification");

    let stdout = String::from_utf8_lossy(&output.stdout);

    // Bronze compliance must always pass for aletheia itself.
    assert!(
        stdout.contains("Bronze-level RSR compliance: ACHIEVED"),
        "Aletheia should achieve Bronze compliance on itself:\n{stdout}"
    );
}

/// Test output format consistency.
#[test]
fn test_output_format() {
    let output = aletheia()
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("Failed to run aletheia");

    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(
        stdout.contains("Aletheia - RSR Compliance Verification Report"),
        "Should have report header"
    );
    assert!(
        stdout.contains("Repository:"),
        "Should show repository path"
    );
    assert!(stdout.contains("Verified:"), "Should show timestamp");
    assert!(
        stdout.contains("Documentation"),
        "Should have Documentation section"
    );
    assert!(
        stdout.contains("Well-Known"),
        "Should have Well-Known section"
    );
    assert!(
        stdout.contains("Build System"),
        "Should have Build System section"
    );
    assert!(stdout.contains("Security"), "Should have Security section");
    assert!(stdout.contains("Score:"), "Should show score");
    assert!(
        stdout.contains("Bronze-level RSR compliance:"),
        "Should show compliance status"
    );
}

/// Test JSON output format.
#[test]
fn test_json_output() {
    let repo = create_fully_compliant_repo("json_out");
    let output = aletheia()
        .args(["--format", "json"])
        .arg(repo.to_str().unwrap())
        .output()
        .expect("Failed to run aletheia with JSON format");

    assert!(output.status.success(), "Should succeed with JSON format");

    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(stdout.contains("\"version\":"), "Should have version field");
    assert!(
        stdout.contains("\"repository\":"),
        "Should have repository field"
    );
    assert!(
        stdout.contains("\"verified_at\":"),
        "Should have verified_at field"
    );
    assert!(stdout.contains("\"score\":"), "Should have score field");
    assert!(
        stdout.contains("\"bronze_compliant\":"),
        "Should have bronze_compliant field"
    );
    assert!(stdout.contains("\"checks\":"), "Should have checks array");
    assert!(
        stdout.contains("\"warnings\":"),
        "Should have warnings array"
    );

    fs::remove_dir_all(repo).ok();
}

/// Test SARIF output format.
#[test]
fn test_sarif_output() {
    let repo = create_fully_compliant_repo("sarif_out");
    let output = aletheia()
        .args(["--format", "sarif"])
        .arg(repo.to_str().unwrap())
        .output()
        .expect("Failed to run aletheia with SARIF format");

    assert!(output.status.success(), "Should succeed with SARIF format");

    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(
        stdout.contains("\"version\": \"2.1.0\""),
        "Should have SARIF version"
    );
    assert!(stdout.contains("\"runs\":"), "Should have runs array");
    assert!(stdout.contains("\"tool\":"), "Should have tool section");
    assert!(
        stdout.contains("\"name\": \"aletheia\""),
        "Should identify as aletheia"
    );
    assert!(stdout.contains("\"rules\":"), "Should have rules");
    assert!(stdout.contains("\"results\":"), "Should have results");
    assert!(stdout.contains("\"ruleId\":"), "Results should have ruleId");
    assert!(stdout.contains("rsr/"), "Rule IDs should use rsr/ prefix");

    fs::remove_dir_all(repo).ok();
}

/// Test quiet mode output (pass).
#[test]
fn test_quiet_mode() {
    let repo = create_fully_compliant_repo("quiet_pass");
    let output = aletheia()
        .arg("-q")
        .arg(repo.to_str().unwrap())
        .output()
        .expect("Failed to run aletheia in quiet mode");

    assert!(output.status.success(), "Should succeed in quiet mode");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert_eq!(stdout.trim(), "PASS", "Quiet mode should only output PASS");

    fs::remove_dir_all(repo).ok();
}

/// Test quiet mode output (fail).
#[test]
fn test_quiet_mode_fail() {
    let repo = create_test_repo("quiet_fail");
    let output = aletheia()
        .arg("-q")
        .arg(repo.to_str().unwrap())
        .output()
        .expect("Failed to run aletheia in quiet mode");

    assert!(!output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert_eq!(stdout.trim(), "FAIL", "Quiet mode should only output FAIL");

    fs::remove_dir_all(repo).ok();
}

/// Test verbose mode output.
#[test]
fn test_verbose_mode() {
    let repo = create_fully_compliant_repo("verbose");
    let output = aletheia()
        .arg("-v")
        .arg(repo.to_str().unwrap())
        .output()
        .expect("Failed to run aletheia in verbose mode");

    assert!(output.status.success(), "Should succeed in verbose mode");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("(Verbose)"), "Should indicate verbose mode");
    assert!(
        stdout.contains("Version:"),
        "Should show version in verbose"
    );
    assert!(
        stdout.contains("Exit code:"),
        "Should show exit code explanation"
    );
    assert!(
        stdout.contains("Per-tier results:"),
        "Should show per-tier counts"
    );

    fs::remove_dir_all(repo).ok();
}

/// Test version flag.
#[test]
fn test_version_flag() {
    let output = aletheia()
        .arg("--version")
        .output()
        .expect("Failed to run aletheia with --version");

    assert!(output.status.success(), "Should succeed with --version");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("aletheia"), "Should show program name");
}

/// Test help flag.
#[test]
fn test_help_flag() {
    let output = aletheia()
        .arg("--help")
        .output()
        .expect("Failed to run aletheia with --help");

    assert!(output.status.success(), "Should succeed with --help");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("USAGE:"), "Should show usage");
    assert!(stdout.contains("OPTIONS:"), "Should show options");
    assert!(stdout.contains("EXIT CODES:"), "Should show exit codes");
    assert!(stdout.contains("EXAMPLES:"), "Should show examples");
    assert!(stdout.contains("sarif"), "Should mention SARIF format");
}

/// Test exit codes for non-compliant repository.
#[test]
fn test_exit_code_compliance_failed() {
    let repo = create_test_repo("exit_code_fail");

    create_file(&repo, "README.md", "# Test");

    let output = aletheia()
        .arg(repo.to_str().unwrap())
        .output()
        .expect("Failed to run aletheia");

    assert_eq!(
        output.status.code(),
        Some(1),
        "Should exit with code 1 for compliance failure"
    );

    fs::remove_dir_all(repo).ok();
}

/// Test exit code for invalid path.
#[test]
fn test_exit_code_invalid_path() {
    let output = aletheia()
        .arg("/nonexistent/path/12345")
        .output()
        .expect("Failed to run aletheia");

    assert_eq!(
        output.status.code(),
        Some(3),
        "Should exit with code 3 for invalid path"
    );
}

/// Test exit code for invalid arguments.
#[test]
fn test_exit_code_invalid_args() {
    let output = aletheia()
        .arg("--invalid-option")
        .output()
        .expect("Failed to run aletheia");

    assert_eq!(
        output.status.code(),
        Some(4),
        "Should exit with code 4 for invalid arguments"
    );
}

/// Test combined format flag.
#[test]
fn test_format_equals_syntax() {
    let repo = create_fully_compliant_repo("fmt_eq");
    let output = aletheia()
        .arg("--format=json")
        .arg(repo.to_str().unwrap())
        .output()
        .expect("Failed to run aletheia with --format=json");

    assert!(output.status.success(), "Should succeed with --format=json");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.starts_with('{'), "Should output JSON");

    fs::remove_dir_all(repo).ok();
}

/// Test README.md accepted as README.adoc alternative.
#[test]
fn test_readme_md_alternative() {
    let repo = create_fully_compliant_repo("readme_md");
    fs::remove_file(repo.join("README.adoc")).ok();
    create_file(&repo, "README.md", "# Test Project");

    let output = aletheia()
        .arg(repo.to_str().unwrap())
        .output()
        .expect("Failed to run aletheia");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("✅ README.adoc (or README.md)"),
        "Should accept README.md as README.adoc alternative"
    );

    fs::remove_dir_all(repo).ok();
}

/// Test LICENSE.txt alone is the retired v1 shape (v2 wants LICENSE).
#[test]
fn test_license_txt_is_retired_shape() {
    let repo = create_test_repo("license_txt");
    create_file(&repo, "LICENSE.txt", "MIT License");

    let output = aletheia()
        .arg(repo.to_str().unwrap())
        .output()
        .expect("Failed to run aletheia");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("❌ LICENSE file (not LICENSE.txt)"),
        "LICENSE.txt alone must fail the v2 license-file check"
    );
    assert!(
        stdout.contains("retired v1 shape"),
        "Should explain the v1 retirement"
    );

    fs::remove_dir_all(repo).ok();
}

/// Test timestamp is present in output.
#[test]
fn test_timestamp_in_output() {
    let output = aletheia()
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("Failed to run aletheia");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("Verified:"),
        "Should show verification timestamp"
    );
    assert!(
        stdout.contains('T') && stdout.contains('Z'),
        "Timestamp should be in ISO 8601 format"
    );
}

/// Test .aletheia.toml disabling checks skips them.
#[test]
fn test_config_file() {
    let repo = create_fully_compliant_repo("config_test");

    create_file(
        &repo,
        ".aletheia.toml",
        "[checks]\nchangelog = false\nwellknown-core = false\n",
    );

    let output = aletheia()
        .arg(repo.to_str().unwrap())
        .output()
        .expect("Failed to run aletheia with config");

    assert!(
        output.status.success(),
        "Should succeed with checks disabled via config"
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    // Disabled checks are skipped, not recorded.
    assert!(
        !stdout.contains("CHANGELOG.adoc (or .md)"),
        "Disabled changelog check should not be reported"
    );
    assert!(
        !stdout.contains(".well-known core files"),
        "Disabled well-known check should not be reported"
    );

    fs::remove_dir_all(repo).ok();
}

/// Test level=silver makes Silver failures fatal.
#[test]
fn test_config_level_silver() {
    let repo = create_fully_compliant_repo("level_silver");
    // Break a Silver-only check.
    fs::remove_file(repo.join("CHANGELOG.adoc")).ok();
    create_file(&repo, ".aletheia.toml", "[aletheia]\nlevel = \"silver\"\n");

    let output = aletheia()
        .arg(repo.to_str().unwrap())
        .output()
        .expect("Failed to run aletheia");

    assert_eq!(
        output.status.code(),
        Some(1),
        "Silver failure with level=silver should exit 1"
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("Bronze-level RSR compliance: ACHIEVED"),
        "Bronze should still hold"
    );
    assert!(
        stdout.contains("Silver-level RSR compliance: NOT MET"),
        "Silver should fail"
    );

    fs::remove_dir_all(repo).ok();
}

/// Test unknown level falls back to bronze with a warning.
#[test]
fn test_config_unknown_level_falls_back() {
    let repo = create_fully_compliant_repo("level_unknown");
    create_file(&repo, ".aletheia.toml", "level = \"copper\"\n");

    let output = aletheia()
        .arg(repo.to_str().unwrap())
        .output()
        .expect("Failed to run aletheia");

    assert!(output.status.success(), "Should fall back to bronze");
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("unknown level"),
        "Should warn about the unknown level"
    );

    fs::remove_dir_all(repo).ok();
}

/// Test SPDX header detection names the offending file.
#[test]
fn test_spdx_header_detection() {
    let repo = create_test_repo("spdx_test");

    create_file(&repo, "README.md", "# Test");
    create_file(
        &repo,
        "src/main.rs",
        "// SPDX-License-Identifier: MPL-2.0\nfn main() {}",
    );
    create_file(&repo, "src/lib.rs", "pub fn hello() {}");

    let output = aletheia()
        .arg(repo.to_str().unwrap())
        .output()
        .expect("Failed to run aletheia");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("❌ SPDX license headers"),
        "Should fail SPDX check for headerless file"
    );
    assert!(
        stdout.contains("src/lib.rs"),
        "Suggestion should name the offending file"
    );

    fs::remove_dir_all(repo).ok();
}

/// Test --format=sarif syntax.
#[test]
fn test_sarif_format_equals_syntax() {
    let repo = create_fully_compliant_repo("sarif_eq");
    let output = aletheia()
        .arg("--format=sarif")
        .arg(repo.to_str().unwrap())
        .output()
        .expect("Failed to run aletheia with --format=sarif");

    assert!(
        output.status.success(),
        "Should succeed with --format=sarif"
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("sarif-schema-2.1.0"),
        "Should reference SARIF schema"
    );

    fs::remove_dir_all(repo).ok();
}

/// Test HTML output format.
#[test]
fn test_html_output() {
    let repo = create_fully_compliant_repo("html_out");
    let output = aletheia()
        .args(["--format", "html"])
        .arg(repo.to_str().unwrap())
        .output()
        .expect("Failed to run aletheia with HTML format");

    assert!(output.status.success(), "Should succeed with HTML format");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("<!DOCTYPE html>"), "Should be valid HTML");
    assert!(
        stdout.contains("Aletheia Compliance Report"),
        "Should have report title"
    );
    assert!(stdout.contains("<style>"), "Should have embedded CSS");
    assert!(
        stdout.contains("Bronze-level RSR compliance"),
        "Should show compliance status"
    );

    fs::remove_dir_all(repo).ok();
}

/// Test the HTML verdict label follows the configured tier (review T8).
#[test]
fn test_html_output_silver_label() {
    let repo = create_fully_compliant_repo("html_silver");
    // Break a Silver-only check so the verdict is meaningfully Silver's.
    fs::remove_file(repo.join("CHANGELOG.adoc")).ok();
    create_file(&repo, ".aletheia.toml", "[aletheia]\nlevel = \"silver\"\n");
    let output = aletheia()
        .args(["--format", "html"])
        .arg(repo.to_str().unwrap())
        .output()
        .expect("Failed to run aletheia with HTML format");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("Silver-level RSR compliance"),
        "HTML label should name the configured tier"
    );
    assert!(
        !stdout.contains("Bronze-level"),
        "HTML label must not claim Bronze for a Silver verdict"
    );

    fs::remove_dir_all(repo).ok();
}

/// Test --format=html syntax.
#[test]
fn test_html_format_equals_syntax() {
    let repo = create_fully_compliant_repo("html_eq");
    let output = aletheia()
        .arg("--format=html")
        .arg(repo.to_str().unwrap())
        .output()
        .expect("Failed to run aletheia with --format=html");

    assert!(output.status.success(), "Should succeed with --format=html");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("<!DOCTYPE html>"), "Should output HTML");

    fs::remove_dir_all(repo).ok();
}

/// Test SVG badge output (passing).
#[test]
fn test_badge_output() {
    let repo = create_fully_compliant_repo("badge_pass");
    let output = aletheia()
        .arg("--badge")
        .arg(repo.to_str().unwrap())
        .output()
        .expect("Failed to run aletheia with --badge");

    assert!(output.status.success(), "Should succeed with --badge");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("<svg"), "Should output SVG");
    assert!(
        stdout.contains("RSR compliance"),
        "Should have RSR compliance label"
    );
    assert!(stdout.contains("passing"), "Should show passing status");

    fs::remove_dir_all(repo).ok();
}

/// Test SVG badge output (failing).
#[test]
fn test_badge_failing() {
    let repo = create_test_repo("badge_fail");
    let output = aletheia()
        .arg("--badge")
        .arg(repo.to_str().unwrap())
        .output()
        .expect("Failed to run aletheia with --badge");

    assert!(!output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("failing"), "Should show failing status");

    fs::remove_dir_all(repo).ok();
}

/// Test fix suggestions in verbose mode.
#[test]
fn test_fix_suggestions_verbose() {
    let repo = create_test_repo("fix_suggestions");

    create_file(&repo, "README.md", "# Test");

    let output = aletheia()
        .arg("-v")
        .arg(repo.to_str().unwrap())
        .output()
        .expect("Failed to run aletheia verbose");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("💡"),
        "Verbose mode should show fix suggestions for failing checks"
    );

    fs::remove_dir_all(repo).ok();
}

/// Test fix suggestions in normal mode.
#[test]
fn test_fix_suggestions_normal() {
    let repo = create_test_repo("fix_suggestions_normal");

    create_file(&repo, "README.md", "# Test");

    let output = aletheia()
        .arg(repo.to_str().unwrap())
        .output()
        .expect("Failed to run aletheia");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("Fix Suggestions"),
        "Normal mode should show Fix Suggestions section"
    );

    fs::remove_dir_all(repo).ok();
}

/// Test ignore patterns in config.
#[test]
fn test_ignore_patterns() {
    let repo = create_fully_compliant_repo("ignore_patterns");

    create_file(
        &repo,
        ".aletheia.toml",
        "[ignore]\nfiles = [\"CHANGELOG.adoc\", \".tool-versions\"]\n",
    );

    // Delete the files that are being ignored.
    fs::remove_file(repo.join("CHANGELOG.adoc")).ok();
    fs::remove_file(repo.join(".tool-versions")).ok();

    let output = aletheia()
        .arg(repo.to_str().unwrap())
        .output()
        .expect("Failed to run aletheia with ignore patterns");

    assert!(
        output.status.success(),
        "Should still pass with ignored files removed"
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("(ignored)"),
        "Should show ignored status for ignored files"
    );

    fs::remove_dir_all(repo).ok();
}

/// Test unpinned workflows fail Silver but keep Bronze green.
#[test]
fn test_sha_pinning_is_silver() {
    let repo = create_fully_compliant_repo("sha_pin");
    create_file(
        &repo,
        ".github/workflows/ci.yml",
        "name: CI\non: [push]\njobs:\n  t:\n    runs-on: ubuntu-latest\n    steps:\n      - uses: actions/checkout@v4\n",
    );

    let output = aletheia()
        .arg(repo.to_str().unwrap())
        .output()
        .expect("Failed to run aletheia");

    // Silver failure must not fail a Bronze gate.
    assert!(
        output.status.success(),
        "Unpinned workflow is Silver, not Bronze"
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("❌ GitHub Actions SHA pinning"),
        "Should flag the unpinned workflow"
    );

    fs::remove_dir_all(repo).ok();
}

/// Test banned languages fail Bronze.
#[test]
fn test_banned_language() {
    let repo = create_fully_compliant_repo("banned_lang");
    create_file(&repo, "scripts/helper.py", "print('hi')\n");

    let output = aletheia()
        .arg(repo.to_str().unwrap())
        .output()
        .expect("Failed to run aletheia");

    assert_eq!(
        output.status.code(),
        Some(1),
        "Banned language is a Bronze failure"
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("❌ No banned languages"),
        "Should flag the banned language"
    );

    fs::remove_dir_all(repo).ok();
}

/// Test Coq `.v` files are not mistaken for V-lang.
#[test]
fn test_coq_v_not_flagged() {
    let repo = create_fully_compliant_repo("coq_v");
    create_file(
        &repo,
        "proofs/Lemma.v",
        "(* SPDX-License-Identifier: MPL-2.0 *)\nRequire Import List.\nLemma trivial_one : 1 = 1.\nProof. reflexivity. Qed.\n",
    );

    let output = aletheia()
        .arg(repo.to_str().unwrap())
        .output()
        .expect("Failed to run aletheia");

    assert!(
        output.status.success(),
        "Coq sources must not trip the V-lang ban"
    );

    fs::remove_dir_all(repo).ok();
}

/// Test silent-skip detection is Gold (non-fatal at Bronze).
#[test]
fn test_no_silent_skip_is_gold() {
    let repo = create_fully_compliant_repo("silent_skip");
    create_file(
        &repo,
        "Justfile",
        "test:\n\tcargo test || echo SKIP tests unavailable\n",
    );

    let output = aletheia()
        .arg(repo.to_str().unwrap())
        .output()
        .expect("Failed to run aletheia");

    assert!(output.status.success(), "Silent-skip is Gold, not Bronze");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("❌ No silent-skip in recipes"),
        "Should flag the silent-skip pattern"
    );

    fs::remove_dir_all(repo).ok();
}

/// Test submodule contents are attributed to upstream, not this repo.
#[test]
fn test_submodule_skipped() {
    let repo = create_fully_compliant_repo("submodule");
    create_file(
        &repo,
        ".gitmodules",
        "[submodule \"third_party\"]\n\tpath = third_party\n\turl = https://example.test/vendor.git\n",
    );
    // Banned content inside the submodule must not fail the outer scan.
    create_file(&repo, "third_party/evil.py", "print('hi')\n");
    create_file(&repo, "third_party/Makefile", "all:\n\techo hi\n");

    let output = aletheia()
        .arg(repo.to_str().unwrap())
        .output()
        .expect("Failed to run aletheia");

    assert!(
        output.status.success(),
        "Submodule contents must be skipped: {}",
        String::from_utf8_lossy(&output.stdout)
    );

    fs::remove_dir_all(repo).ok();
}

/// Test committed secret filenames fail Bronze.
#[test]
fn test_secret_filename() {
    let repo = create_fully_compliant_repo("secret_name");
    create_file(&repo, ".env", "SMTP_PASSWORD=hunter2\n");

    let output = aletheia()
        .arg(repo.to_str().unwrap())
        .output()
        .expect("Failed to run aletheia");

    assert_eq!(
        output.status.code(),
        Some(1),
        "Committed .env is a Bronze failure"
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("❌ No secrets committed"),
        "Should flag the committed secret file"
    );

    fs::remove_dir_all(repo).ok();
}

/// Test committed key material fails Bronze.
#[test]
fn test_secret_content() {
    let repo = create_fully_compliant_repo("secret_content");
    // Built from halves: a whole marker literal here would trip the
    // scanner on its own test source (see checks.rs fragment comment).
    let key_block = format!(
        "{}{}\nZmFrZXlhdHl6eQ==\n",
        "-----BEGIN RSA ", "PRIVATE KEY-----"
    );
    create_file(&repo, "deploy/key.pem", &key_block);

    let output = aletheia()
        .arg(repo.to_str().unwrap())
        .output()
        .expect("Failed to run aletheia");

    assert_eq!(
        output.status.code(),
        Some(1),
        "Committed key material is a Bronze failure"
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("❌ No secrets committed"),
        "Should flag the committed key material"
    );

    fs::remove_dir_all(repo).ok();
}

/// Test symlink escapes are critical (exit 2).
#[test]
#[cfg(unix)]
fn test_symlink_escape() {
    let outside = create_test_repo("symlink_outside");
    create_file(&outside, "secret.txt", "outside\n");
    let repo = create_fully_compliant_repo("symlink_repo");

    std::os::unix::fs::symlink(outside.join("secret.txt"), repo.join("leak"))
        .expect("Failed to create symlink");

    let output = aletheia()
        .arg(repo.to_str().unwrap())
        .output()
        .expect("Failed to run aletheia");

    assert_eq!(
        output.status.code(),
        Some(2),
        "Escaping symlink should exit 2"
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("escapes repository"),
        "Should report the escaping symlink"
    );

    fs::remove_dir_all(repo).ok();
    fs::remove_dir_all(outside).ok();
}

/// Test a symlink sweep cut short by depth warns instead of passing silently (review T4).
#[test]
#[cfg(unix)]
fn test_symlink_depth_truncation_warns() {
    let outside = create_test_repo("symlink_deep_outside");
    create_file(&outside, "secret.txt", "outside\n");
    let repo = create_fully_compliant_repo("symlink_deep_repo");

    // Nest deeper than the sweep reaches; the escaping link at the bottom
    // is never observed, so the run must say the results are partial.
    let mut deep = repo.clone();
    for i in 0..18 {
        deep = deep.join(format!("d{i}"));
    }
    fs::create_dir_all(&deep).expect("Failed to create deep fixture");
    std::os::unix::fs::symlink(outside.join("secret.txt"), deep.join("leak"))
        .expect("Failed to create symlink");

    let output = aletheia()
        .arg(repo.to_str().unwrap())
        .output()
        .expect("Failed to run aletheia");

    assert!(
        output.status.success(),
        "Compliant repo with unobserved depth should still exit 0"
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("results are partial"),
        "Depth-truncated sweep must warn"
    );
    assert!(
        !stdout.contains("escapes repository"),
        "The unobserved link must not be reported as seen"
    );

    fs::remove_dir_all(repo).ok();
    fs::remove_dir_all(outside).ok();
}

/// Test init-hook subcommand.
#[test]
fn test_init_hook() {
    let repo = create_test_repo("init_hook");

    // Initialize a git repo (skip if git is unavailable).
    let init = Command::new("git")
        .args(["init"])
        .current_dir(&repo)
        .output();
    if init.is_err() || !init.map(|o| o.status.success()).unwrap_or(false) {
        eprintln!("skipping test_init_hook: git unavailable");
        fs::remove_dir_all(repo).ok();
        return;
    }

    let output = aletheia()
        .args(["init-hook"])
        .arg(repo.to_str().unwrap())
        .output()
        .expect("Failed to run aletheia init-hook");

    assert!(output.status.success(), "init-hook should succeed");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("Installed pre-commit hook"),
        "Should confirm hook installation"
    );

    let hook_path = repo.join(".git").join("hooks").join("pre-commit");
    assert!(hook_path.exists(), "Pre-commit hook file should exist");

    let hook_content = fs::read_to_string(&hook_path).expect("Failed to read hook");
    assert!(
        hook_content.contains("aletheia"),
        "Hook should reference aletheia"
    );
    // Review T7: exit codes distinguished, missing binary caught.
    assert!(
        hook_content.contains("command -v aletheia"),
        "Hook should check the binary exists"
    );
    assert!(
        hook_content.contains("Bronze compliance NOT MET"),
        "Status 1 keeps the compliance message"
    );
    assert!(
        hook_content.contains("verification error"),
        "Other statuses report a verification error"
    );

    #[cfg(unix)]
    {
        // The hook must be valid shell; skip where sh is unavailable.
        match Command::new("sh")
            .args(["-n", hook_path.to_str().unwrap()])
            .output()
        {
            Ok(output) => assert!(
                output.status.success(),
                "Hook should pass sh -n: {}",
                String::from_utf8_lossy(&output.stderr)
            ),
            Err(_) => eprintln!("skipping sh -n check: sh unavailable"),
        }
    }

    fs::remove_dir_all(repo).ok();
}

/// Test init-hook without a git repository fails honestly.
#[test]
fn test_init_hook_no_git() {
    let repo = create_test_repo("init_hook_no_git");

    let output = aletheia()
        .args(["init-hook"])
        .arg(repo.to_str().unwrap())
        .output()
        .expect("Failed to run aletheia init-hook");

    assert_eq!(
        output.status.code(),
        Some(3),
        "init-hook without .git should exit 3"
    );

    fs::remove_dir_all(repo).ok();
}

/// Test tags covered by actions.lock pass (estate mechanism: tags in YAML
/// plus a verified lockfile; governance forbids inline SHAs with a lock).
#[test]
fn test_tag_with_lockfile_passes() {
    let repo = create_fully_compliant_repo("tag_locked");
    create_file(
        &repo,
        ".github/workflows/ci.yml",
        "name: CI\non: [push]\njobs:\n  t:\n    runs-on: ubuntu-latest\n    steps:\n      - uses: actions/checkout@v7.0.1\n",
    );
    create_file(
        &repo,
        ".github/workflows/actions.lock",
        "version: 'v0.0.2'\nworkflows:\n    '.github/workflows/ci.yml':\n        - 'actions/checkout@v7.0.1'\n",
    );
    let output = aletheia()
        .arg(repo.to_str().unwrap())
        .output()
        .expect("Failed to run aletheia");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("\u{2705} GitHub Actions SHA pinning"),
        "Locked tag should pass: {stdout}"
    );
    fs::remove_dir_all(repo).ok();
}

/// Test tags without lock coverage fail (strict SHAs, no-lock branch).
#[test]
fn test_tag_without_lockfile_fails() {
    let repo = create_fully_compliant_repo("tag_unlocked");
    create_file(
        &repo,
        ".github/workflows/ci.yml",
        "name: CI\non: [push]\njobs:\n  t:\n    runs-on: ubuntu-latest\n    steps:\n      - uses: actions/checkout@v7.0.1\n",
    );
    let output = aletheia()
        .arg(repo.to_str().unwrap())
        .output()
        .expect("Failed to run aletheia");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("\u{274c} GitHub Actions SHA pinning"),
        "Unlocked tag should fail: {stdout}"
    );
    assert!(
        stdout.contains("actions.lock"),
        "Hint should name the lock: {stdout}"
    );
    fs::remove_dir_all(repo).ok();
}

/// Test tier-1 Bun carve-out: runtime deps with a Bun lockfile pass.
#[test]
fn test_bun_lockfile_carveout() {
    let repo = create_fully_compliant_repo("bun_ok");
    create_file(&repo, "package.json", "{\"dependencies\": {\"x\": \"1\"}}");
    create_file(&repo, "bun.lock", "# bun lockfile");
    let output = aletheia()
        .arg(repo.to_str().unwrap())
        .output()
        .expect("Failed to run aletheia");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("\u{2705} No Node/npm runtime deps"),
        "Bun lockfile should carve out: {stdout}"
    );
    fs::remove_dir_all(repo).ok();

    let bare = create_fully_compliant_repo("bun_missing");
    create_file(&bare, "package.json", "{\"dependencies\": {\"x\": \"1\"}}");
    let output = aletheia()
        .arg(bare.to_str().unwrap())
        .output()
        .expect("Failed to run aletheia");
    assert_eq!(
        output.status.code(),
        Some(1),
        "Runtime deps without a Bun lockfile are a Bronze failure"
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("\u{274c} No Node/npm runtime deps"),
        "Should flag the unlocked runtime deps: {stdout}"
    );
    fs::remove_dir_all(bare).ok();
}

/// REGRESSION GUARD (issue #186): a freshly scaffolded project must pass
/// aletheia Bronze (and Silver) with no hand edits.
///
/// The v1-era generator emitted the retired shape — `LICENSE.txt`,
/// lowercase `justfile`, `flake.nix`, `.gitlab-ci.yml` — which failed
/// Bronze on day one. This test runs the real generator and audits its
/// real output, so the template and the checker cannot drift apart again
/// without the suite going red.
#[test]
fn test_scaffold_passes_bronze_and_silver() {
    let pid = std::process::id();
    let work = std::env::temp_dir().join(format!("aletheia_scaffold_{pid}"));
    if work.exists() {
        fs::remove_dir_all(&work).ok();
    }
    fs::create_dir_all(&work).expect("Failed to create scaffold workspace");

    let generator = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("scripts")
        .join("create-template.sh");

    // `--no-git` keeps the probe hermetic: it does not depend on the
    // machine having a configured git identity.
    let generated = Command::new("bash")
        .arg(&generator)
        .arg("scaffold-probe")
        .arg("--no-git")
        .arg("-d")
        .arg("Regression probe for the v2 scaffold")
        .current_dir(&work)
        .output()
        .expect("Failed to run create-template.sh");

    assert!(
        generated.status.success(),
        "generator failed (exit {:?}): {}",
        generated.status.code(),
        String::from_utf8_lossy(&generated.stderr)
    );

    let project = work.join("scaffold-probe");
    assert!(project.is_dir(), "generator produced no project directory");

    // The retired v1 shape must never come back.
    for stale in ["LICENSE.txt", "justfile", "flake.nix", ".gitlab-ci.yml"] {
        assert!(
            !project.join(stale).exists(),
            "retired v1 artefact regenerated: {stale} (issue #186)"
        );
    }

    // The generator must leave no unresolved template placeholder behind.
    let readme = fs::read_to_string(project.join("README.adoc"))
        .expect("scaffold should contain README.adoc");
    assert!(
        !readme.contains("@@"),
        "unresolved template placeholder left in README.adoc"
    );

    let output = aletheia()
        .arg(project.to_str().unwrap())
        .output()
        .expect("Failed to run aletheia");
    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(
        stdout.contains("Bronze-level RSR compliance: ACHIEVED"),
        "fresh scaffold must pass Bronze with no hand edits (issue #186): {stdout}"
    );
    assert!(
        stdout.contains("Silver-level RSR compliance: ACHIEVED"),
        "fresh scaffold should also reach Silver: {stdout}"
    );
    assert_eq!(
        output.status.code(),
        Some(0),
        "a compliant scaffold must exit 0: {stdout}"
    );

    fs::remove_dir_all(&work).ok();
}

/// Build output must not change the verdict.
///
/// Reported after the #186/#197 work: running the gate on a tree that had just
/// been built made it fail, because the scanner counted generated files
/// (`obj/b__main.ads`, `target/...`, `_build/...`) as sources lacking SPDX
/// headers. The scanner now honours the repository's own `.gitignore`, so
/// auditing a freshly built tree gives the same answer as auditing a clean
/// checkout.
#[test]
fn test_gitignore_keeps_build_output_out_of_the_audit() {
    let repo = create_test_repo("gitignore_build_output");

    // A minimal compliant project.
    create_file(&repo, "README.md", "# Probe\n");
    create_file(&repo, "LICENSE", "MPL-2.0\n");
    create_file(&repo, ".gitattributes", "* text=auto\n");
    create_file(&repo, ".editorconfig", "root = true\n");
    create_file(&repo, "justfile", "default:\n    @true\n");
    create_file(&repo, "SECURITY.md", "# Security\n");
    create_file(&repo, "CONTRIBUTING.md", "# Contributing\n");
    create_file(&repo, "CHANGELOG.md", "# Changelog\n");
    create_file(&repo, "CODE_OF_CONDUCT.md", "# CoC\n");
    create_file(
        &repo,
        ".well-known/security.txt",
        "Contact: x@example.org\n",
    );
    create_file(&repo, ".well-known/ai.txt", "ai\n");
    create_file(&repo, ".well-known/humans.txt", "humans\n");
    create_file(&repo, ".github/workflows/ci.yml", PINNED_WORKFLOW);
    create_file(
        &repo,
        "src/main.rs",
        "// SPDX-License-Identifier: MPL-2.0\nfn main() {}\n",
    );
    create_file(&repo, ".gitignore", "target/\nobj/\ndist-newstyle/\n");

    let clean = aletheia()
        .arg(repo.to_str().unwrap())
        .output()
        .expect("run");
    let clean_out = String::from_utf8_lossy(&clean.stdout);
    assert!(
        clean_out.contains("Bronze-level RSR compliance: ACHIEVED"),
        "the clean tree should be compliant: {clean_out}"
    );

    // Now simulate a build: generated files, no SPDX headers, inside ignored
    // directories. This is exactly what broke the gate in practice.
    create_file(&repo, "target/debug/build.rs", "fn generated() {}\n");
    create_file(
        &repo,
        "obj/b__main.ads",
        "package B__Main is end B__Main;\n",
    );
    create_file(&repo, "dist-newstyle/build/x.rs", "fn y() {}\n");

    let built = aletheia()
        .arg(repo.to_str().unwrap())
        .output()
        .expect("run");
    let built_out = String::from_utf8_lossy(&built.stdout);

    // The verdict must be identical to the clean run — same score, and the
    // generated files must not appear in any suggestion.
    assert!(
        !built_out.contains("b__main.ads")
            && !built_out.contains("dist-newstyle")
            && !built_out.contains("target/debug/build.rs"),
        "ignored build output leaked back into the audit: {built_out}"
    );
    assert!(
        built_out.contains("Bronze-level RSR compliance: ACHIEVED"),
        "a built tree must give the clean-tree verdict: {built_out}"
    );
    assert_eq!(
        built.status.code(),
        clean.status.code(),
        "exit code changed after a build"
    );

    // A file that is *not* ignored must still be audited: the mechanism must
    // not become a blanket exemption.
    create_file(&repo, "src/unheadered.rs", "fn no_header() {}\n");
    let leaky = aletheia()
        .arg(repo.to_str().unwrap())
        .output()
        .expect("run");
    let leaky_out = String::from_utf8_lossy(&leaky.stdout);
    assert!(
        leaky_out.contains("unheadered.rs"),
        "a non-ignored file must still be scanned: {leaky_out}"
    );

    fs::remove_dir_all(&repo).ok();
}

/// A directory-only rule must not exempt a *file* of the same name.
///
/// `Makefile/` in git matches a directory called `Makefile`; a regular file
/// called `Makefile` is untouched. The scanner asks the type-aware question,
/// so it inherits that behaviour instead of blanket-skipping the name. Uses
/// the banned-build-files check as the probe, because it reports file names
/// and therefore makes the difference observable.
#[test]
fn test_gitignore_directory_rule_does_not_exempt_same_named_file() {
    let repo = create_test_repo("gitignore_dir_only");

    create_file(&repo, ".gitignore", "Makefile/\n");
    // A regular file whose name matches a directory-only rule.
    create_file(&repo, "Makefile", "all:\n\t@true\n");

    let output = aletheia()
        .arg(repo.to_str().unwrap())
        .output()
        .expect("run");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("Makefile"),
        "a file named `Makefile` must still be audited despite the `Makefile/` rule: {stdout}"
    );

    fs::remove_dir_all(&repo).ok();
}

/// The same name as a real directory *is* skipped, contents and all.
#[test]
fn test_gitignore_directory_rule_skips_the_directory_and_its_contents() {
    let repo = create_test_repo("gitignore_dir_real");

    create_file(&repo, ".gitignore", "vendored/\n");
    // Unmistakably generated, unmistakably headerless — if the scanner reads
    // this, the score drops and the path shows up in a suggestion.
    create_file(
        &repo,
        "vendored/generated_helper.adb",
        "package Generated_Helper is end;\n",
    );
    create_file(
        &repo,
        "vendored/deep/nested/deep_generated.adb",
        "package Deep is end;\n",
    );

    let output = aletheia()
        .arg(repo.to_str().unwrap())
        .output()
        .expect("run");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        !stdout.contains("generated_helper.adb") && !stdout.contains("deep_generated.adb"),
        "contents of an ignored directory must be skipped: {stdout}"
    );

    // And the same file in a directory that is NOT ignored still reports, so
    // the rule is scoped and not a blanket exemption.
    create_file(
        &repo,
        "src/not_ignored_helper.adb",
        "package Not_Ignored is end;\n",
    );
    let output = aletheia()
        .arg(repo.to_str().unwrap())
        .output()
        .expect("run");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("not_ignored_helper.adb"),
        "a file outside the ignored directory must still be audited: {stdout}"
    );

    fs::remove_dir_all(&repo).ok();
}

/// Regression probe for the six-language template set (follow-on to #186).
///
/// Every language `create-template.sh` advertises must scaffold cleanly and
/// reach Bronze *and* Silver with no hand edits. The single-language probe
/// above covers the generator's default; this one covers the whole set, so a
/// template that regresses under a language overlay cannot slip through.
///
/// Note on scope: this asserts the *build-time* contract only — file shape,
/// placeholder resolution and the RSR gate. It does not run Creusot or
/// gnatprove; those need nightly Rust plus pinned Why3 forks, and a pinned
/// GNAT, which a unit test must not require. The proof gates are exercised by
/// `just proof` in each template, and their negative controls live with them.
#[test]
fn test_every_language_template_reaches_bronze_and_silver() {
    /// Recursively collect files containing an unresolved `@@` placeholder.
    fn files_with_placeholders(dir: &Path, found: &mut Vec<PathBuf>) {
        let Ok(entries) = fs::read_dir(dir) else {
            return;
        };
        for entry in entries.flatten() {
            let path = entry.path();
            if path.file_name().is_some_and(|n| n == ".git") {
                continue;
            }
            if path.is_dir() {
                files_with_placeholders(&path, found);
            } else if fs::read_to_string(&path).is_ok_and(|body| body.contains("@@")) {
                found.push(path);
            }
        }
    }

    const LANGS: &[&str] = &["rust", "zig", "elixir", "haskell", "ada", "agda"];

    let pid = std::process::id();
    let work = std::env::temp_dir().join(format!("aletheia_langs_{pid}"));
    if work.exists() {
        fs::remove_dir_all(&work).ok();
    }
    fs::create_dir_all(&work).expect("Failed to create scaffold workspace");

    let generator = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("scripts")
        .join("create-template.sh");

    for lang in LANGS {
        let name = format!("probe-{lang}");

        // `--no-git` keeps the probe hermetic: no git identity is needed, and
        // a working tree without .git exercises the same file set.
        let generated = Command::new("bash")
            .arg(&generator)
            .arg(&name)
            .arg("-l")
            .arg(lang)
            .arg("--no-git")
            .current_dir(&work)
            .output()
            .unwrap_or_else(|e| panic!("Failed to run create-template.sh for {lang}: {e}"));

        assert!(
            generated.status.success(),
            "[{lang}] generator failed (exit {:?}): {}",
            generated.status.code(),
            String::from_utf8_lossy(&generated.stderr)
        );

        let project = work.join(&name);
        assert!(project.is_dir(), "[{lang}] no project directory produced");

        let mut stragglers = Vec::new();
        files_with_placeholders(&project, &mut stragglers);
        assert!(
            stragglers.is_empty(),
            "[{lang}] unresolved template placeholders in: {stragglers:?}"
        );

        // `.gitattributes` carries exactly one pattern per line. Git rejects a
        // line holding two (`*.ads is not a valid attribute name`) and then
        // ignores *both* patterns on it, so the language silently loses
        // `text`/`eol=lf` on precisely the files the template exists for. Two of
        // the six languages have two extensions each, which is where this broke.
        let attrs = fs::read_to_string(project.join(".gitattributes"))
            .unwrap_or_else(|e| panic!("[{lang}] no .gitattributes in the scaffold: {e}"));
        for (n, line) in attrs.lines().enumerate() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            let patterns = line.split_whitespace().filter(|t| t.contains('*')).count();
            assert!(
                patterns <= 1,
                "[{lang}] .gitattributes:{} puts {patterns} patterns on one line; git rejects the \
                 whole line and drops every pattern on it: {line}",
                n + 1
            );
        }

        let expected_attrs: &[&str] = match *lang {
            "rust" => &["*.rs"],
            "zig" => &["*.zig"],
            "elixir" => &["*.ex", "*.exs"],
            "haskell" => &["*.hs"],
            "ada" => &["*.adb", "*.ads"],
            "agda" => &["*.agda"],
            other => panic!("language {other} has no expected attribute list in this test"),
        };
        for ext in expected_attrs {
            let covered = attrs.lines().any(|l| {
                let toks: Vec<&str> = l.split_whitespace().collect();
                toks.first() == Some(ext) && toks.contains(&"text") && toks.contains(&"eol=lf")
            });
            assert!(
                covered,
                "[{lang}] .gitattributes must give {ext} `text eol=lf` on a line of its own:\n{attrs}"
            );
        }

        let output = aletheia()
            .arg(project.to_str().unwrap())
            .output()
            .unwrap_or_else(|e| panic!("Failed to run aletheia for {lang}: {e}"));
        let stdout = String::from_utf8_lossy(&output.stdout);

        assert!(
            stdout.contains("Score: 26/26 checks passed (100.0%)"),
            "[{lang}] template must reach 26/26 with no hand edits: {stdout}"
        );
        assert!(
            stdout.contains("Bronze-level RSR compliance: ACHIEVED"),
            "[{lang}] Bronze not achieved: {stdout}"
        );
        assert!(
            stdout.contains("Silver-level RSR compliance: ACHIEVED"),
            "[{lang}] Silver not achieved: {stdout}"
        );
        assert_eq!(
            output.status.code(),
            Some(0),
            "[{lang}] a compliant scaffold must exit 0: {stdout}"
        );
    }

    // The estate's two proof-carrying templates must actually carry the
    // markers that make their language label true.
    let rust_manifest = fs::read_to_string(work.join("probe-rust/verification/Cargo.toml"))
        .expect("rust template should ship verification/Cargo.toml");
    assert!(
        rust_manifest.contains("creusot-std"),
        "Rust means Rust/Creusot: the verification crate must depend on creusot-std"
    );

    let rust_impl = fs::read_to_string(work.join("probe-rust/src/impl.rs"))
        .expect("rust template should ship src/impl.rs");
    assert!(
        rust_impl.contains("cfg(creusot)") && rust_impl.contains("requires("),
        "rust template must carry Creusot contracts gated behind cfg(creusot)"
    );

    let ada_spec = fs::read_to_string(work.join("probe-ada/src/probe_ada.ads"))
        .expect("ada template should ship its package spec");
    assert!(
        ada_spec.contains("pragma SPARK_Mode (On)"),
        "Ada means Ada/SPARK: the core package must declare SPARK_Mode (On)"
    );

    fs::remove_dir_all(&work).ok();
}
