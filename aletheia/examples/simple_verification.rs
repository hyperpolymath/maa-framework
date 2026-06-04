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

    // Example 3: Expected output
    println!("Example 3: Expected output format");
    println!(
        "
🔍 Aletheia - RSR Compliance Verification Report
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Repository: /path/to/repository

📋 Documentation
  ✅ README.md [Bronze]
  ✅ LICENSE.txt [Bronze]
  ✅ SECURITY.md [Bronze]
  ❌ CONTRIBUTING.md [Bronze]
  ...

Score: 14/16 checks passed (87.5%)
⚠️  Bronze-level RSR compliance: NOT MET
    "
    );

    println!("\nNote: To use Aletheia, run the binary:");
    println!("  cargo run                    # Verify current directory");
    println!("  cargo run -- /path/to/repo   # Verify specific repository");
}
