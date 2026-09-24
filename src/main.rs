// SPDX-License-Identifier: MPL-2.0
//! proofgateway — command-line entry point.
//!
//! Replace this with your own CLI. It is dependency-free by design: no
//! argument-parsing crate, no network access.

use std::process::ExitCode;

use proofgateway::{clamp, midpoint};

const USAGE: &str = "\
proofgateway 0.1.0

usage:
  proofgateway clamp <value> <lo> <hi>   clamp a value into a range
  proofgateway mid   <a> <b>             midpoint, rounded down (a <= b)
  proofgateway --help                    show this message

All arguments are unsigned 32-bit integers.";

fn parse_u32(raw: &str, what: &str) -> Result<u32, String> {
    raw.parse::<u32>()
        .map_err(|_| format!("{what} must be an unsigned integer, got {raw:?}"))
}

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();

    let result = match args.as_slice() {
        [] => {
            println!("{USAGE}");
            return ExitCode::SUCCESS;
        }
        [flag] if flag == "--help" || flag == "-h" => {
            println!("{USAGE}");
            return ExitCode::SUCCESS;
        }
        [cmd, value, lo, hi] if cmd == "clamp" => (|| {
            let value = parse_u32(value, "value")?;
            let lo = parse_u32(lo, "lo")?;
            let hi = parse_u32(hi, "hi")?;
            if lo > hi {
                return Err(format!("lo ({lo}) must not exceed hi ({hi})"));
            }
            Ok(clamp(value, lo, hi).to_string())
        })(),
        [cmd, a, b] if cmd == "mid" => (|| {
            let a = parse_u32(a, "a")?;
            let b = parse_u32(b, "b")?;
            if a > b {
                return Err(format!("a ({a}) must not exceed b ({b})"));
            }
            Ok(midpoint(a, b).to_string())
        })(),
        _ => Err(format!("unrecognised arguments\n\n{USAGE}")),
    };

    match result {
        Ok(output) => {
            println!("{output}");
            ExitCode::SUCCESS
        }
        Err(message) => {
            eprintln!("error: {message}");
            ExitCode::FAILURE
        }
    }
}
