//! Rhodium Pipeline CLI
//!
//! Generate RSR-compliant CI/CD configurations.

use rhodium_pipeline::{
    generate_pipeline, validate_pipeline, PipelineLevel, PipelineOptions, Platform, VERSION,
};
use std::fs;
use std::path::PathBuf;
use std::process;

fn print_help() {
    println!(
        r#"Rhodium Pipeline - RSR CI/CD Pipeline Generator

USAGE:
    rhodium-pipeline [COMMAND] [OPTIONS]

COMMANDS:
    generate <platform>    Generate CI/CD configuration
    validate [path]        Validate existing pipeline
    list                   List available templates

PLATFORMS:
    github     GitHub Actions (.github/workflows/)
    gitlab     GitLab CI (.gitlab-ci.yml)
    circle     CircleCI (.circleci/config.yml)
    jenkins    Jenkinsfile

OPTIONS:
    -o, --output <path>    Output path (default: stdout)
    -n, --name <name>      Project name (default: project)
    -l, --level <level>    RSR level: bronze, silver, gold (default: bronze)
    -f, --force            Overwrite existing files
    -h, --help             Print help information
    -V, --version          Print version information

EXAMPLES:
    rhodium-pipeline generate github
    rhodium-pipeline generate gitlab -o .gitlab-ci.yml
    rhodium-pipeline validate .
    rhodium-pipeline list
"#
    );
}

fn print_version() {
    println!("rhodium-pipeline {}", VERSION);
}

fn list_templates() {
    println!("Available Templates:");
    println!();
    println!("  Platforms:");
    println!("    github   - GitHub Actions workflow");
    println!("    gitlab   - GitLab CI configuration");
    println!("    circle   - CircleCI configuration");
    println!("    jenkins  - Jenkinsfile");
    println!();
    println!("  Levels:");
    println!("    bronze   - Basic RSR compliance (default)");
    println!("    silver   - Extended checks and testing");
    println!("    gold     - Multi-platform builds");
    println!("    platinum - Full enterprise pipeline");
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        print_help();
        process::exit(0);
    }

    let mut output_path: Option<PathBuf> = None;
    let mut project_name = String::from("project");
    let mut level = PipelineLevel::Bronze;
    let mut force = false;

    // Parse global options first
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "-h" | "--help" => {
                print_help();
                process::exit(0);
            },
            "-V" | "--version" => {
                print_version();
                process::exit(0);
            },
            "-o" | "--output" => {
                i += 1;
                if i < args.len() {
                    output_path = Some(PathBuf::from(&args[i]));
                }
            },
            "-n" | "--name" => {
                i += 1;
                if i < args.len() {
                    project_name = args[i].clone();
                }
            },
            "-l" | "--level" => {
                i += 1;
                if i < args.len() {
                    level = match args[i].as_str() {
                        "bronze" => PipelineLevel::Bronze,
                        "silver" => PipelineLevel::Silver,
                        "gold" => PipelineLevel::Gold,
                        "platinum" => PipelineLevel::Platinum,
                        _ => {
                            eprintln!("Unknown level: {}", args[i]);
                            process::exit(1);
                        },
                    };
                }
            },
            "-f" | "--force" => force = true,
            _ => break,
        }
        i += 1;
    }

    if i >= args.len() {
        print_help();
        process::exit(0);
    }

    match args[i].as_str() {
        "generate" => {
            i += 1;
            if i >= args.len() {
                eprintln!("Error: Platform required. Use: github, gitlab, circle, jenkins");
                process::exit(1);
            }

            let platform = match Platform::parse(&args[i]) {
                Some(p) => p,
                None => {
                    eprintln!("Unknown platform: {}", args[i]);
                    process::exit(1);
                },
            };

            let options = PipelineOptions {
                platform,
                level,
                include_deploy: false,
                project_name,
                rust_version: String::from("stable"),
            };

            let config = generate_pipeline(&options);

            if let Some(path) = output_path {
                if path.exists() && !force {
                    eprintln!(
                        "Error: {} already exists. Use --force to overwrite.",
                        path.display()
                    );
                    process::exit(1);
                }

                // Create parent directories if needed
                if let Some(parent) = path.parent() {
                    let _ = fs::create_dir_all(parent);
                }

                if let Err(e) = fs::write(&path, &config) {
                    eprintln!("Error writing {}: {}", path.display(), e);
                    process::exit(1);
                }
                println!("Generated: {}", path.display());
            } else {
                println!("{}", config);
            }
        },
        "validate" => {
            i += 1;
            let path = if i < args.len() {
                PathBuf::from(&args[i])
            } else {
                PathBuf::from(".")
            };

            let result = validate_pipeline(&path);

            if !result.errors.is_empty() {
                println!("Errors:");
                for error in &result.errors {
                    println!("  - {}", error);
                }
            }

            if !result.warnings.is_empty() {
                println!("Warnings:");
                for warning in &result.warnings {
                    println!("  - {}", warning);
                }
            }

            if result.valid {
                println!("Pipeline configuration is valid.");
                process::exit(0);
            } else {
                println!("Pipeline configuration has issues.");
                process::exit(1);
            }
        },
        "list" => {
            list_templates();
        },
        cmd => {
            eprintln!("Unknown command: {}", cmd);
            print_help();
            process::exit(1);
        },
    }
}
