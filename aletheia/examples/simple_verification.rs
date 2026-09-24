// SPDX-License-Identifier: MPL-2.0
// Copyright (c) Jonathan D.A. Jewell <j.d.a.jewell@open.ac.uk>
//! Simple example of using Aletheia as a library
//!
//! This example shows how to programmatically verify RSR compliance
//! Note: Aletheia is currently a binary-only tool, but this demonstrates
//! the API design if library usage is added in the future.

fn main() {
    println!("Aletheia RSR Compliance Verification Example");
    println!("============================================\n");

    // Example 1: Verify current directory
    println!("Example 1: Verify current directory");
    let current_dir = std::env::current_dir().expect("Cannot get current directory");
    println!("Checking: {}", current_dir.display());
    println!("Run: cargo run\n");

    // Example 2: Verify specific path
    println!("Example 2: Verify specific repository");
    println!("Run: cargo run -- /path/to/repository\n");

    // Example 3: Expected output (RSR v2 shape — README.adoc primary,
    // LICENSE not LICENSE.txt, 26 checks in the offline subset)
    println!("Example 3: Expected output format");
    println!(
        "
Aletheia - RSR Compliance Verification Report
Repository: /path/to/repository
Verified: 2026-09-21T20:05:37Z

[Documentation]
  ✅ README.adoc (or README.md) [Bronze]
  ✅ LICENSE file (not LICENSE.txt) [Bronze]
  ✅ SECURITY policy (root or .github/) [Bronze]
  ❌ CONTRIBUTING (root or .github/) [Silver]
  ...

Score: 25/26 checks passed (96.2%)
Bronze-level RSR compliance: ACHIEVED
Silver-level RSR compliance: NOT MET
    "
    );

    println!("\nNote: To use Aletheia, run the binary:");
    println!("  cargo run                    # Verify current directory");
    println!("  cargo run -- /path/to/repo   # Verify specific repository");
}
