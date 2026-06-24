// SPDX-License-Identifier: MPL-2.0
// Copyright (c) Jonathan D.A. Jewell <j.d.a.jewell@open.ac.uk>
//! Integration tests for Aletheia RSR compliance verification
//!
//! These tests verify the complete end-to-end functionality of Aletheia.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// Helper to create a temporary test repository
fn create_test_repo(name: &str) -> PathBuf {
    let test_dir = std::env::temp_dir().join(format!("aletheia_test_{}", name));

    // Clean up if it exists
    if test_dir.exists() {
        fs::remove_dir_all(&test_dir).ok();
    }

    fs::create_dir_all(&test_dir).expect("Failed to create test directory");
    test_dir
}

/// Helper to create a file in the test repo
fn create_file(base: &Path, path: &str, content: &str) {
    let file_path = base.join(path);

    // Create parent directories if needed
    if let Some(parent) = file_path.parent() {
        fs::create_dir_all(parent).ok();
    }

    fs::write(file_path, content).expect("Failed to create file");
}

/// Create a fully compliant test repository (Bronze + Silver)
fn create_fully_compliant_repo(name: &str) -> PathBuf {
    let repo = create_test_repo(name);

    // Documentation (Bronze)
    create_file(&repo, "README.md", "# Test Project");
    create_file(&repo, "LICENSE.txt", "MIT License");
    create_file(&repo, "SECURITY.md", "# Security Policy");
    create_file(
        &repo,
        "CONTRIBUTING.md",
        "# Contributing\n\n## How to Contribute\n\nGetting started with development.\n\n## Pull Request Process\n\nPlease submit a pull request.\n\n## Setup\n\nRun the setup script.\n\nMore details on how to get involved.\n",
    );
    create_file(&repo, "CODE_OF_CONDUCT.md", "# Code of Conduct");
    create_file(&repo, "MAINTAINERS.md", "# Maintainers");
    create_file(&repo, "CHANGELOG.md", "# Changelog");

    // .well-known (Bronze)
    create_file(
        &repo,
        ".well-known/security.txt",
        "Contact: security@example.org",
    );
    create_file(&repo, ".well-known/ai.txt", "# AI Policy");
    create_file(&repo, ".well-known/humans.txt", "# Humans");

    // Build system (Bronze)
    create_file(&repo, "justfile", "build:\n\techo 'building'");
    create_file(&repo, "flake.nix", "{}");
    create_file(&repo, ".gitlab-ci.yml", "test:\n  script: echo 'test'");

    // Source structure (Bronze)
    create_file(
        &repo,
        "src/main.rs",
        "// SPDX-License-Identifier: MPL-2.0\nfn main() {}",
    );
    create_file(&repo, "tests/test.rs", "#[test] fn test() {}");

    // Silver checks
    create_file(&repo, ".editorconfig", "root = true\n");

    repo
}

/// Test verification of a fully compliant repository
#[test]
fn test_fully_compliant_repository() {
    let repo = create_fully_compliant_repo("compliant");

    // Run aletheia on the test repository
    let output = Command::new("cargo")
        .args(["run", "--", repo.to_str().unwrap()])
        .output()
        .expect("Failed to run aletheia");

    // Should exit with success (Bronze compliance)
    assert!(
        output.status.success(),
        "Fully compliant repository should pass verification"
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("Bronze-level RSR compliance: ACHIEVED"),
        "Should achieve Bronze compliance"
    );

    // Clean up
    fs::remove_dir_all(repo).ok();
}

/// Test verification of a partially compliant repository
#[test]
fn test_partially_compliant_repository() {
    let repo = create_test_repo("partial");

    // Create only some required files
    create_file(&repo, "README.md", "# Test Project");
    create_file(&repo, "LICENSE.txt", "MIT License");
    create_file(&repo, "src/main.rs", "fn main() {}");

    // Run aletheia on the test repository
    let output = Command::new("cargo")
        .args(["run", "--", repo.to_str().unwrap()])
        .output()
        .expect("Failed to run aletheia");

    // Should exit with failure
    assert!(
        !output.status.success(),
        "Partially compliant repository should fail verification"
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("Bronze-level RSR compliance: NOT MET"),
        "Should not achieve Bronze compliance"
    );

    // Clean up
    fs::remove_dir_all(repo).ok();
}

/// Test verification of empty repository
#[test]
fn test_empty_repository() {
    let repo = create_test_repo("empty");

    // Run aletheia on empty repository
    let output = Command::new("cargo")
        .args(["run", "--", repo.to_str().unwrap()])
        .output()
        .expect("Failed to run aletheia");

    // Should exit with failure
    assert!(
        !output.status.success(),
        "Empty repository should fail verification"
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("Bronze-level RSR compliance: NOT MET"),
        "Should not meet Bronze compliance"
    );

    // Clean up
    fs::remove_dir_all(repo).ok();
}

/// Test handling of non-existent path
#[test]
fn test_nonexistent_path() {
    let output = Command::new("cargo")
        .args(["run", "--", "/nonexistent/path/that/does/not/exist"])
        .output()
        .expect("Failed to run aletheia");

    // Should exit with error
    assert!(!output.status.success());

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("does not exist") || stderr.contains("Error"),
        "Should report path error"
    );
}

/// Test self-verification (Aletheia verifying itself)
#[test]
fn test_self_verification() {
    let output = Command::new("cargo")
        .args(["run"])
        .output()
        .expect("Failed to run aletheia self-verification");

    let stdout = String::from_utf8_lossy(&output.stdout);

    // Bronze compliance should always pass for aletheia itself
    assert!(
        stdout.contains("Bronze-level RSR compliance: ACHIEVED"),
        "Aletheia should achieve Bronze compliance on itself"
    );

    // Should have Bronze checks (16) plus Silver checks
    assert!(
        stdout.contains("Content Validation"),
        "Should include Content Validation section"
    );
}

/// Test output format consistency
#[test]
fn test_output_format() {
    let output = Command::new("cargo")
        .args(["run"])
        .output()
        .expect("Failed to run aletheia");

    let stdout = String::from_utf8_lossy(&output.stdout);

    // Check for expected output sections
    assert!(
        stdout.contains("Aletheia - RSR Compliance Verification Report"),
        "Should have report header"
    );
    assert!(
        stdout.contains("Repository:"),
        "Should show repository path"
    );
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
    assert!(
        stdout.contains("Source Structure"),
        "Should have Source Structure section"
    );
    assert!(
        stdout.contains("Content Validation"),
        "Should have Content Validation section"
    );
    assert!(stdout.contains("Score:"), "Should show score");
    assert!(
        stdout.contains("Bronze-level RSR compliance:"),
        "Should show compliance status"
    );
}

/// Test that tests directory can be named 'test' or 'tests'
#[test]
fn test_alternate_test_directory_names() {
    // Test with 'tests' directory
    let repo1 = create_test_repo("with_tests");
    create_file(&repo1, "src/main.rs", "fn main() {}");
    create_file(&repo1, "tests/test.rs", "#[test] fn test() {}");

    let output1 = Command::new("cargo")
        .args(["run", "--", repo1.to_str().unwrap()])
        .output()
        .expect("Failed to run aletheia");

    let stdout1 = String::from_utf8_lossy(&output1.stdout);
    assert!(
        stdout1.contains("✅ tests/ directory"),
        "Should accept 'tests' directory"
    );

    // Test with 'test' directory
    let repo2 = create_test_repo("with_test");
    create_file(&repo2, "src/main.rs", "fn main() {}");
    create_file(&repo2, "test/test.rs", "#[test] fn test() {}");

    let output2 = Command::new("cargo")
        .args(["run", "--", repo2.to_str().unwrap()])
        .output()
        .expect("Failed to run aletheia");

    let stdout2 = String::from_utf8_lossy(&output2.stdout);
    assert!(
        stdout2.contains("✅ tests/ directory"),
        "Should accept 'test' directory"
    );

    // Clean up
    fs::remove_dir_all(repo1).ok();
    fs::remove_dir_all(repo2).ok();
}

/// Test JSON output format
#[test]
fn test_json_output() {
    let output = Command::new("cargo")
        .args(["run", "--", "--format", "json"])
        .output()
        .expect("Failed to run aletheia with JSON format");

    assert!(output.status.success(), "Should succeed with JSON format");

    let stdout = String::from_utf8_lossy(&output.stdout);

    // Verify JSON structure
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
}

/// Test SARIF output format
#[test]
fn test_sarif_output() {
    let output = Command::new("cargo")
        .args(["run", "--", "--format", "sarif"])
        .output()
        .expect("Failed to run aletheia with SARIF format");

    assert!(output.status.success(), "Should succeed with SARIF format");

    let stdout = String::from_utf8_lossy(&output.stdout);

    // Verify SARIF structure
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
    assert!(
        stdout.contains("\"ruleId\":"),
        "Results should have ruleId"
    );
    assert!(
        stdout.contains("rsr/"),
        "Rule IDs should use rsr/ prefix"
    );
}

/// Test quiet mode output
#[test]
fn test_quiet_mode() {
    let output = Command::new("cargo")
        .args(["run", "--", "-q"])
        .output()
        .expect("Failed to run aletheia in quiet mode");

    assert!(output.status.success(), "Should succeed in quiet mode");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert_eq!(stdout.trim(), "PASS", "Quiet mode should only output PASS");
}

/// Test verbose mode output
#[test]
fn test_verbose_mode() {
    let output = Command::new("cargo")
        .args(["run", "--", "-v"])
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
}

/// Test version flag
#[test]
fn test_version_flag() {
    let output = Command::new("cargo")
        .args(["run", "--", "--version"])
        .output()
        .expect("Failed to run aletheia with --version");

    assert!(output.status.success(), "Should succeed with --version");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("aletheia"), "Should show program name");
}

/// Test help flag
#[test]
fn test_help_flag() {
    let output = Command::new("cargo")
        .args(["run", "--", "--help"])
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

/// Test exit codes for non-compliant repository
#[test]
fn test_exit_code_compliance_failed() {
    let repo = create_test_repo("exit_code_fail");

    // Create minimal non-compliant repo
    create_file(&repo, "README.md", "# Test");

    let output = Command::new("cargo")
        .args(["run", "--", repo.to_str().unwrap()])
        .output()
        .expect("Failed to run aletheia");

    // Exit code 1 = compliance failed
    assert_eq!(
        output.status.code(),
        Some(1),
        "Should exit with code 1 for compliance failure"
    );

    fs::remove_dir_all(repo).ok();
}

/// Test exit code for invalid path
#[test]
fn test_exit_code_invalid_path() {
    let output = Command::new("cargo")
        .args(["run", "--", "/nonexistent/path/12345"])
        .output()
        .expect("Failed to run aletheia");

    // Exit code 3 = invalid path
    assert_eq!(
        output.status.code(),
        Some(3),
        "Should exit with code 3 for invalid path"
    );
}

/// Test exit code for invalid arguments
#[test]
fn test_exit_code_invalid_args() {
    let output = Command::new("cargo")
        .args(["run", "--", "--invalid-option"])
        .output()
        .expect("Failed to run aletheia");

    // Exit code 4 = invalid arguments
    assert_eq!(
        output.status.code(),
        Some(4),
        "Should exit with code 4 for invalid arguments"
    );
}

/// Test combined short format flag
#[test]
fn test_format_equals_syntax() {
    let output = Command::new("cargo")
        .args(["run", "--", "--format=json"])
        .output()
        .expect("Failed to run aletheia with --format=json");

    assert!(output.status.success(), "Should succeed with --format=json");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.starts_with('{'), "Should output JSON");
}

/// Test README.adoc alternative
#[test]
fn test_readme_adoc_alternative() {
    let repo = create_test_repo("readme_adoc");

    // Create with README.adoc instead of README.md
    create_file(&repo, "README.adoc", "= Test Project");
    create_file(&repo, "LICENSE.txt", "MIT");
    create_file(&repo, "src/main.rs", "fn main() {}");

    let output = Command::new("cargo")
        .args(["run", "--", repo.to_str().unwrap()])
        .output()
        .expect("Failed to run aletheia");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("✅ README.md"),
        "Should accept README.adoc as README.md alternative"
    );

    fs::remove_dir_all(repo).ok();
}

/// Test timestamp is present in output
#[test]
fn test_timestamp_in_output() {
    let output = Command::new("cargo")
        .args(["run"])
        .output()
        .expect("Failed to run aletheia");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("Verified:"),
        "Should show verification timestamp"
    );
    // Check ISO 8601 format (contains T and Z)
    assert!(
        stdout.contains('T') && stdout.contains('Z'),
        "Timestamp should be in ISO 8601 format"
    );
}

/// Test .aletheia.toml configuration file
#[test]
fn test_config_file() {
    let repo = create_fully_compliant_repo("config_test");

    // Create config that disables Silver checks
    create_file(
        &repo,
        ".aletheia.toml",
        r#"
[aletheia]
level = "bronze"

[checks]
editorconfig = false
spdx-headers = false
workflow-pins = false
contributing-content = false
"#,
    );

    let output = Command::new("cargo")
        .args(["run", "--", repo.to_str().unwrap()])
        .output()
        .expect("Failed to run aletheia with config");

    assert!(
        output.status.success(),
        "Should succeed with config disabling Silver checks"
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    // Should NOT contain Content Validation section when all Silver checks disabled
    assert!(
        !stdout.contains("Content Validation"),
        "Should not show Content Validation when Silver checks disabled"
    );

    fs::remove_dir_all(repo).ok();
}

/// Test SPDX header detection
#[test]
fn test_spdx_header_detection() {
    let repo = create_test_repo("spdx_test");

    create_file(&repo, "README.md", "# Test");
    create_file(
        &repo,
        "src/main.rs",
        "// SPDX-License-Identifier: MPL-2.0\nfn main() {}",
    );
    create_file(
        &repo,
        "src/lib.rs",
        "// SPDX-License-Identifier: MPL-2.0\npub fn hello() {}",
    );

    let output = Command::new("cargo")
        .args(["run", "--", repo.to_str().unwrap()])
        .output()
        .expect("Failed to run aletheia");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("SPDX license headers"),
        "Should check SPDX headers"
    );

    fs::remove_dir_all(repo).ok();
}

/// Test --format=sarif syntax
#[test]
fn test_sarif_format_equals_syntax() {
    let output = Command::new("cargo")
        .args(["run", "--", "--format=sarif"])
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
}

/// Test HTML output format
#[test]
fn test_html_output() {
    let output = Command::new("cargo")
        .args(["run", "--", "--format", "html"])
        .output()
        .expect("Failed to run aletheia with HTML format");

    assert!(output.status.success(), "Should succeed with HTML format");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("<!DOCTYPE html>"),
        "Should be valid HTML"
    );
    assert!(
        stdout.contains("Aletheia Compliance Report"),
        "Should have report title"
    );
    assert!(
        stdout.contains("<style>"),
        "Should have embedded CSS"
    );
    assert!(
        stdout.contains("Bronze-level RSR compliance"),
        "Should show compliance status"
    );
}

/// Test --format=html syntax
#[test]
fn test_html_format_equals_syntax() {
    let output = Command::new("cargo")
        .args(["run", "--", "--format=html"])
        .output()
        .expect("Failed to run aletheia with --format=html");

    assert!(
        output.status.success(),
        "Should succeed with --format=html"
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("<!DOCTYPE html>"),
        "Should output HTML"
    );
}

/// Test SVG badge output
#[test]
fn test_badge_output() {
    let output = Command::new("cargo")
        .args(["run", "--", "--badge"])
        .output()
        .expect("Failed to run aletheia with --badge");

    assert!(output.status.success(), "Should succeed with --badge");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("<svg"), "Should output SVG");
    assert!(
        stdout.contains("RSR compliance"),
        "Should have RSR compliance label"
    );
    assert!(
        stdout.contains("passing"),
        "Should show passing status (aletheia passes Bronze)"
    );
}

/// Test fix suggestions in verbose mode
#[test]
fn test_fix_suggestions_verbose() {
    let repo = create_test_repo("fix_suggestions");

    // Create minimal repo (will fail many checks)
    create_file(&repo, "README.md", "# Test");

    let output = Command::new("cargo")
        .args(["run", "--", "-v", repo.to_str().unwrap()])
        .output()
        .expect("Failed to run aletheia verbose");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("💡"),
        "Verbose mode should show fix suggestions for failing checks"
    );

    fs::remove_dir_all(repo).ok();
}

/// Test fix suggestions in normal mode
#[test]
fn test_fix_suggestions_normal() {
    let repo = create_test_repo("fix_suggestions_normal");

    create_file(&repo, "README.md", "# Test");

    let output = Command::new("cargo")
        .args(["run", "--", repo.to_str().unwrap()])
        .output()
        .expect("Failed to run aletheia");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("Fix Suggestions"),
        "Normal mode should show Fix Suggestions section"
    );

    fs::remove_dir_all(repo).ok();
}

/// Test ignore patterns in config
#[test]
fn test_ignore_patterns() {
    let repo = create_fully_compliant_repo("ignore_patterns");

    // Create config that ignores flake.nix
    create_file(
        &repo,
        ".aletheia.toml",
        r#"
[ignore]
files = ["flake.nix", ".gitlab-ci.yml"]
"#,
    );

    // Delete the files that are being ignored
    fs::remove_file(repo.join("flake.nix")).ok();
    fs::remove_file(repo.join(".gitlab-ci.yml")).ok();

    let output = Command::new("cargo")
        .args(["run", "--", repo.to_str().unwrap()])
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

/// Test init-hook subcommand
#[test]
fn test_init_hook() {
    let repo = create_test_repo("init_hook");

    // Initialize a git repo
    std::process::Command::new("git")
        .args(["init"])
        .current_dir(&repo)
        .output()
        .expect("Failed to init git repo");

    let output = Command::new("cargo")
        .args(["run", "--", "init-hook", repo.to_str().unwrap()])
        .output()
        .expect("Failed to run aletheia init-hook");

    assert!(output.status.success(), "init-hook should succeed");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("Installed pre-commit hook"),
        "Should confirm hook installation"
    );

    // Verify the hook file exists
    let hook_path = repo.join(".git").join("hooks").join("pre-commit");
    assert!(hook_path.exists(), "Pre-commit hook file should exist");

    let hook_content =
        fs::read_to_string(&hook_path).expect("Failed to read hook");
    assert!(
        hook_content.contains("aletheia"),
        "Hook should reference aletheia"
    );

    fs::remove_dir_all(repo).ok();
}
