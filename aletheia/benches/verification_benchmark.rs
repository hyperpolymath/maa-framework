//! Benchmarks for Aletheia RSR compliance verification
//!
//! These benchmarks measure the performance of verification operations.
//!
//! Run with: cargo build --release && cargo run --release --bin verification_benchmark
//!
//! Note: This is a simple benchmark implementation using std::time.
//! For more sophisticated benchmarking, consider using criterion.rs
//! (but that would add a dependency, breaking RSR Bronze compliance).

use std::path::PathBuf;
use std::process::Command;
use std::time::{Duration, Instant};

/// Number of iterations for each benchmark
const ITERATIONS: u32 = 100;

/// Number of warmup iterations
const WARMUP: u32 = 5;

/// Benchmark a function and return statistics
struct BenchmarkResult {
    name: String,
    min: Duration,
    max: Duration,
    avg: Duration,
    iterations: u32,
}

impl BenchmarkResult {
    fn print(&self) {
        println!(
            "{}: min={}μs, max={}μs, avg={}μs ({} iterations)",
            self.name,
            self.min.as_micros(),
            self.max.as_micros(),
            self.avg.as_micros(),
            self.iterations
        );
    }
}

/// Run a benchmark with warmup and statistics
fn benchmark<F: FnMut()>(name: &str, mut f: F) -> BenchmarkResult {
    // Warmup phase
    for _ in 0..WARMUP {
        f();
    }

    let mut times = Vec::with_capacity(ITERATIONS as usize);

    // Measurement phase
    for _ in 0..ITERATIONS {
        let start = Instant::now();
        f();
        times.push(start.elapsed());
    }

    let min = *times.iter().min().unwrap();
    let max = *times.iter().max().unwrap();
    let total: Duration = times.iter().sum();
    let avg = total / ITERATIONS;

    BenchmarkResult {
        name: name.to_string(),
        min,
        max,
        avg,
        iterations: ITERATIONS,
    }
}

/// Run a benchmark for command execution (fewer iterations)
fn benchmark_command(name: &str, binary_path: &str, args: &[&str]) -> BenchmarkResult {
    const CMD_ITERATIONS: u32 = 10;

    // Warmup
    for _ in 0..2 {
        let _ = Command::new(binary_path).args(args).output();
    }

    let mut times = Vec::with_capacity(CMD_ITERATIONS as usize);

    for _ in 0..CMD_ITERATIONS {
        let start = Instant::now();
        let _ = Command::new(binary_path).args(args).output();
        times.push(start.elapsed());
    }

    let min = *times.iter().min().unwrap();
    let max = *times.iter().max().unwrap();
    let total: Duration = times.iter().sum();
    let avg = total / CMD_ITERATIONS;

    BenchmarkResult {
        name: name.to_string(),
        min,
        max,
        avg,
        iterations: CMD_ITERATIONS,
    }
}

fn main() {
    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║         Aletheia Performance Benchmarks                      ║");
    println!("╚══════════════════════════════════════════════════════════════╝\n");

    let current_dir = std::env::current_dir().expect("Cannot get current directory");

    // Build release binary first
    println!("Building release binary...");
    let build_result = Command::new("cargo")
        .args(["build", "--release"])
        .current_dir(&current_dir)
        .output()
        .expect("Failed to build release binary");

    if !build_result.status.success() {
        eprintln!("Failed to build release binary");
        std::process::exit(1);
    }
    println!("Build complete.\n");

    let binary_path = current_dir.join("target/release/aletheia");
    let binary_str = binary_path.to_str().unwrap();

    println!("┌─────────────────────────────────────────────────────────────┐");
    println!("│ Micro-benchmarks (filesystem operations)                    │");
    println!("└─────────────────────────────────────────────────────────────┘\n");

    // Benchmark 1: Path validation
    let path_validation = benchmark("Path validation", || {
        let path = PathBuf::from(&current_dir);
        let _ = path.exists();
        let _ = path.is_dir();
    });
    path_validation.print();

    // Benchmark 2: File existence checks
    let file_checks = benchmark("File existence check (single)", || {
        let _ = current_dir.join("README.md").is_file();
    });
    file_checks.print();

    // Benchmark 3: Multiple file checks
    let multi_file = benchmark("File existence checks (16 files)", || {
        let files = [
            "README.md",
            "LICENSE.txt",
            "SECURITY.md",
            "CONTRIBUTING.md",
            "CODE_OF_CONDUCT.md",
            "MAINTAINERS.md",
            "CHANGELOG.md",
            "Cargo.toml",
            "justfile",
            "flake.nix",
            ".gitlab-ci.yml",
            ".well-known/security.txt",
            ".well-known/ai.txt",
            ".well-known/humans.txt",
            "src/main.rs",
            "tests/integration_tests.rs",
        ];
        for file in &files {
            let _ = current_dir.join(file).exists();
        }
    });
    multi_file.print();

    // Benchmark 4: Directory checks
    let dir_checks = benchmark("Directory existence checks", || {
        let _ = current_dir.join("src").is_dir();
        let _ = current_dir.join("tests").is_dir();
        let _ = current_dir.join(".well-known").is_dir();
    });
    dir_checks.print();

    // Benchmark 5: Symlink checks
    let symlink_checks = benchmark("Symlink detection", || {
        let _ = current_dir.join("README.md").symlink_metadata();
    });
    symlink_checks.print();

    // Benchmark 6: Canonicalization
    let canon = benchmark("Path canonicalization", || {
        let _ = current_dir.canonicalize();
    });
    canon.print();

    println!("\n┌─────────────────────────────────────────────────────────────┐");
    println!("│ End-to-end benchmarks (full verification)                   │");
    println!("└─────────────────────────────────────────────────────────────┘\n");

    // Benchmark: Full verification (human format)
    let human_format = benchmark_command("Full verification (human)", binary_str, &[]);
    human_format.print();

    // Benchmark: Full verification (JSON format)
    let json_format = benchmark_command(
        "Full verification (JSON)",
        binary_str,
        &["--format", "json"],
    );
    json_format.print();

    // Benchmark: Quiet mode
    let quiet_mode = benchmark_command("Full verification (quiet)", binary_str, &["-q"]);
    quiet_mode.print();

    // Benchmark: Verbose mode
    let verbose_mode = benchmark_command("Full verification (verbose)", binary_str, &["-v"]);
    verbose_mode.print();

    println!("\n┌─────────────────────────────────────────────────────────────┐");
    println!("│ Summary                                                     │");
    println!("└─────────────────────────────────────────────────────────────┘\n");

    println!(
        "Average full verification time: {}μs ({:.2}ms)",
        human_format.avg.as_micros(),
        human_format.avg.as_secs_f64() * 1000.0
    );

    println!("\nPerformance Targets:");
    println!("  Target:      <10ms per verification");
    println!("  Excellent:   <5ms per verification");
    println!("  Outstanding: <2ms per verification");

    let avg_ms = human_format.avg.as_secs_f64() * 1000.0;
    if avg_ms < 2.0 {
        println!("\n  Status: ⭐ OUTSTANDING ({:.2}ms < 2ms)", avg_ms);
    } else if avg_ms < 5.0 {
        println!("\n  Status: ✨ EXCELLENT ({:.2}ms < 5ms)", avg_ms);
    } else if avg_ms < 10.0 {
        println!("\n  Status: ✅ TARGET MET ({:.2}ms < 10ms)", avg_ms);
    } else {
        println!("\n  Status: ⚠️  NEEDS IMPROVEMENT ({:.2}ms >= 10ms)", avg_ms);
    }

    // Memory info (if available on Linux)
    #[cfg(target_os = "linux")]
    {
        if let Ok(output) = Command::new("ls").args(["-lh", binary_str]).output() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            if let Some(line) = stdout.lines().next() {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 5 {
                    println!("\nBinary size: {}", parts[4]);
                }
            }
        }
    }
}
