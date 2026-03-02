//! Integration tests for Rhodibot

use std::path::PathBuf;
use std::process::Command;

/// Get path to the rhodibot binary
fn rhodibot_binary() -> PathBuf {
    let mut path = std::env::current_exe().unwrap();
    path.pop(); // Remove test binary name
    path.pop(); // Remove deps
    path.push("rhodibot");
    path
}

#[test]
fn test_help_flag() {
    // Build first
    let _ = Command::new("cargo").args(["build"]).output();

    let output = Command::new(rhodibot_binary())
        .arg("--help")
        .output()
        .expect("Failed to execute rhodibot");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Rhodibot"));
    assert!(stdout.contains("USAGE"));
}

#[test]
fn test_version_flag() {
    let _ = Command::new("cargo").args(["build"]).output();

    let output = Command::new(rhodibot_binary())
        .arg("--version")
        .output()
        .expect("Failed to execute rhodibot");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("rhodibot"));
}

#[test]
fn test_check_command() {
    let _ = Command::new("cargo").args(["build"]).output();

    let output = Command::new(rhodibot_binary())
        .args(["check", "."])
        .output()
        .expect("Failed to execute rhodibot");

    // May pass or fail depending on repo state, but should not error
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Rhodibot") || stdout.contains("PASS") || stdout.contains("FAIL"));
}

#[test]
fn test_badge_command() {
    let _ = Command::new("cargo").args(["build"]).output();

    let output = Command::new(rhodibot_binary())
        .args(["badge"])
        .output()
        .expect("Failed to execute rhodibot");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("RSR"));
    assert!(stdout.contains("img.shields.io"));
}

#[test]
fn test_conformity_command() {
    let _ = Command::new("cargo").args(["build"]).output();

    let output = Command::new(rhodibot_binary())
        .args(["conformity"])
        .output()
        .expect("Failed to execute rhodibot");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("RSR Conformity Statement"));
}

#[test]
fn test_json_output() {
    let _ = Command::new("cargo").args(["build"]).output();

    let output = Command::new(rhodibot_binary())
        .args(["check", ".", "--format", "json"])
        .output()
        .expect("Failed to execute rhodibot");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("\"tool\": \"rhodibot\""));
    assert!(stdout.contains("\"checks\""));
}

#[test]
fn test_quiet_mode() {
    let _ = Command::new("cargo").args(["build"]).output();

    let output = Command::new(rhodibot_binary())
        .args(["check", ".", "--quiet"])
        .output()
        .expect("Failed to execute rhodibot");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("PASS") || stdout.contains("FAIL"));
}

#[test]
fn test_invalid_path() {
    let _ = Command::new("cargo").args(["build"]).output();

    let output = Command::new(rhodibot_binary())
        .args(["check", "/nonexistent/path/12345"])
        .output()
        .expect("Failed to execute rhodibot");

    assert!(!output.status.success());
    assert_eq!(output.status.code(), Some(3)); // INVALID_PATH
}

#[test]
fn test_invalid_format() {
    let _ = Command::new("cargo").args(["build"]).output();

    let output = Command::new(rhodibot_binary())
        .args(["check", ".", "--format", "invalid"])
        .output()
        .expect("Failed to execute rhodibot");

    assert!(!output.status.success());
    assert_eq!(output.status.code(), Some(4)); // INVALID_ARGS
}
