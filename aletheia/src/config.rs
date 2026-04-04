// SPDX-License-Identifier: PMPL-1.0-or-later

//! Aletheia Configuration Kernel — Zero-Dependency TOML Parser.
//!
//! This module implements a safe, minimal TOML parser to ingest
//! the `.aletheia.toml` policy file.
//!
//! DESIGN MANDATE: To satisfy RSR Bronze compliance (Air-Gapped/Self-Contained),
//! this parser avoids external crates like `serde` or `toml-rs`.

use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// TOML value types.
#[derive(Debug, Clone)]
pub enum TomlValue {
    String(String),
    Boolean(bool),
    Integer(i64),
}

/// IGNORE PATTERNS: Paths to exclude from audit.
#[derive(Debug, Clone, Default)]
pub struct IgnoreConfig {
    pub patterns: Vec<String>,
}

/// COMPLIANCE CONFIG: Toggles specific verification modules.
#[derive(Debug, Clone)]
pub struct ChecksConfig {
    pub documentation: bool,
    pub well_known: bool,
    pub build_system: bool,
    pub spdx_headers: bool,
    pub workflow_pins: bool,
}

impl Default for ChecksConfig {
    fn default() -> Self {
        ChecksConfig {
            documentation: true,
            well_known: true,
            build_system: true,
            spdx_headers: true,
            workflow_pins: true,
        }
    }
}

/// CONFIGURATION: The top-level policy record.
#[derive(Debug, Clone)]
pub struct Config {
    pub level: String, // Target RSR tier (bronze, silver, gold)
    pub checks: ChecksConfig,
    pub ignore: IgnoreConfig, // Path patterns to exclude from audit
}

impl Default for Config {
    fn default() -> Self {
        Config {
            level: "bronze".to_string(),
            checks: ChecksConfig::default(),
            ignore: IgnoreConfig::default(),
        }
    }
}

impl Config {
    /// LOADER: Reconciles the physical `.aletheia.toml` file with
    /// the hardcoded system defaults.
    pub fn load_config(repo_path: &Path) -> Config {
        let config_path = repo_path.join(".aletheia.toml");

        if config_path.is_file() {
            if let Ok(content) = fs::read_to_string(&config_path) {
                return Self::parse_from_string(&content);
            }
        }

        Config::default()
    }

    /// Parse configuration from a TOML string.
    fn parse_from_string(content: &str) -> Config {
        let mut config = Config::default();

        for line in content.lines() {
            let trimmed = line.trim();
            if trimmed.is_empty() || trimmed.starts_with('#') {
                continue;
            }

            if trimmed.starts_with("level") {
                if let Some(value) = trimmed.split('=').nth(1) {
                    config.level = value.trim().trim_matches('"').to_string();
                }
            } else if trimmed.starts_with("documentation") {
                if let Some(value) = trimmed.split('=').nth(1) {
                    config.checks.documentation = value.trim().parse().unwrap_or(true);
                }
            } else if trimmed.starts_with("spdx_headers") {
                if let Some(value) = trimmed.split('=').nth(1) {
                    config.checks.spdx_headers = value.trim().parse().unwrap_or(true);
                }
            }
        }

        config
    }
}

/// PARSER: A line-based state machine for TOML key-value pairs.
fn parse_toml(content: &str) -> HashMap<String, HashMap<String, TomlValue>> {
    let mut sections: HashMap<String, HashMap<String, TomlValue>> = HashMap::new();
    let mut current_section = String::new();

    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }

        if trimmed.starts_with('[') && trimmed.ends_with(']') {
            current_section = trimmed[1..trimmed.len() - 1].to_string();
            sections.insert(current_section.clone(), HashMap::new());
        } else if let Some(eq_idx) = trimmed.find('=') {
            let key = trimmed[..eq_idx].trim().to_string();
            let value_str = trimmed[eq_idx + 1..].trim();
            let value = if value_str.starts_with('"') && value_str.ends_with('"') {
                TomlValue::String(value_str[1..value_str.len() - 1].to_string())
            } else if let Ok(b) = value_str.parse::<bool>() {
                TomlValue::Boolean(b)
            } else if let Ok(i) = value_str.parse::<i64>() {
                TomlValue::Integer(i)
            } else {
                TomlValue::String(value_str.to_string())
            };

            if let Some(section) = sections.get_mut(&current_section) {
                section.insert(key, value);
            }
        }
    }

    sections
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_default() {
        let config = Config::default();
        assert_eq!(config.level, "bronze");
        assert!(config.checks.documentation);
        assert!(config.checks.spdx_headers);
    }

    #[test]
    fn test_config_parse_simple() {
        let toml = r#"
level = "silver"
documentation = true
spdx_headers = false
"#;
        let config = Config::parse_from_string(toml);
        assert_eq!(config.level, "silver");
        assert!(config.checks.documentation);
        assert!(!config.checks.spdx_headers);
    }

    #[test]
    fn test_config_parse_with_comments() {
        let toml = r#"
# This is a comment
level = "bronze"
# Another comment
documentation = true
"#;
        let config = Config::parse_from_string(toml);
        assert_eq!(config.level, "bronze");
        assert!(config.checks.documentation);
    }

    #[test]
    fn test_ignore_config_default() {
        let ignore = IgnoreConfig::default();
        assert!(ignore.patterns.is_empty());
    }

    #[test]
    fn test_checks_config_default() {
        let checks = ChecksConfig::default();
        assert!(checks.documentation);
        assert!(checks.well_known);
        assert!(checks.build_system);
        assert!(checks.spdx_headers);
        assert!(checks.workflow_pins);
    }

    #[test]
    fn test_toml_value_string() {
        match TomlValue::String("test".to_string()) {
            TomlValue::String(s) => assert_eq!(s, "test"),
            _ => panic!("Expected String variant"),
        }
    }

    #[test]
    fn test_toml_value_boolean() {
        match TomlValue::Boolean(true) {
            TomlValue::Boolean(b) => assert!(b),
            _ => panic!("Expected Boolean variant"),
        }
    }

    #[test]
    fn test_toml_value_integer() {
        match TomlValue::Integer(42) {
            TomlValue::Integer(i) => assert_eq!(i, 42),
            _ => panic!("Expected Integer variant"),
        }
    }

    #[test]
    fn test_parse_toml_basic() {
        let toml = r#"
[project]
name = "test"
version = "1.0"
enabled = true
"#;
        let parsed = parse_toml(toml);
        assert!(parsed.contains_key("project"));
    }
}
