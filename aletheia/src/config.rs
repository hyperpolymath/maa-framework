// SPDX-License-Identifier: MPL-2.0
// Copyright (c) Jonathan D.A. Jewell <j.d.a.jewell@open.ac.uk>
//! Aletheia Configuration Kernel — Zero-Dependency TOML Parser.
//!
//! This module implements a safe, minimal TOML parser to ingest
//! the `.aletheia.toml` policy file.
//!
//! DESIGN MANDATE: To satisfy RSR Bronze compliance (Air-Gapped/Self-Contained),
//! this parser avoids external crates like `serde` or `toml-rs`.
//!
//! SUPPORTED SHAPE (all sections optional; missing file means defaults):
//!
//! ```toml
//! [aletheia]
//! level = "bronze"            # bronze | silver | gold | platinum
//!
//! [checks]
//! readme = true               # per-check toggles, keyed by check id
//! changelog = false           # (see checks.rs inventory for ids)
//!
//! [ignore]
//! files = ["fixtures/**", "*.log"]   # glob patterns, matched against
//!                                     # repo-relative paths (`*`, `?`)
//! ```
//!
//! For backwards compatibility, top-level `level = "..."` and flat
//! `documentation` / `spdx_headers` keys are still honoured.

use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// Maximum `.aletheia.toml` size ingested (1 MiB). A hostile config must
/// not be able to exhaust memory.
const MAX_CONFIG_BYTES: u64 = 1024 * 1024;

/// TOML value types supported by the minimal parser.
#[derive(Debug, Clone, PartialEq)]
pub enum TomlValue {
    String(String),
    Boolean(bool),
    Integer(i64),
    Array(Vec<String>),
}

impl TomlValue {
    /// Interpret the value as a boolean toggle. Strings accept the
    /// `true`/`false` spellings; anything else falls back to `default`.
    pub fn as_bool(&self, default: bool) -> bool {
        match self {
            TomlValue::Boolean(b) => *b,
            TomlValue::String(s) if s.eq_ignore_ascii_case("true") => true,
            TomlValue::String(s) if s.eq_ignore_ascii_case("false") => false,
            TomlValue::Integer(i) => *i != 0,
            _ => default,
        }
    }

    /// Interpret the value as a string.
    pub fn as_string(&self) -> Option<&str> {
        match self {
            TomlValue::String(s) => Some(s),
            _ => None,
        }
    }

    /// Interpret the value as a string array (for `[ignore] files`).
    /// A lone string is treated as a single-element array.
    pub fn as_array(&self) -> Vec<String> {
        match self {
            TomlValue::Array(items) => items.clone(),
            TomlValue::String(s) => vec![s.clone()],
            _ => Vec::new(),
        }
    }
}

/// IGNORE PATTERNS: glob patterns for paths to exclude from the audit.
/// Matched against repo-relative paths with `/` separators (see
/// `checks::glob_match`). `*` crosses directory boundaries.
#[derive(Debug, Clone, Default)]
pub struct IgnoreConfig {
    pub patterns: Vec<String>,
}

impl IgnoreConfig {
    /// Whether a repo-relative path is ignored by any pattern.
    pub fn is_ignored(&self, rel_path: &str) -> bool {
        self.patterns
            .iter()
            .any(|pat| crate::checks::glob_match(pat, rel_path))
    }
}

/// COMPLIANCE CONFIG: toggles for specific verification checks.
///
/// The five named fields are the historical category toggles; `extra`
/// carries per-check-id toggles (`readme = false`, …). [`ChecksConfig::enabled`]
/// resolves a check id through both, defaulting to enabled.
#[derive(Debug, Clone)]
pub struct ChecksConfig {
    pub documentation: bool,
    pub well_known: bool,
    pub build_system: bool,
    pub spdx_headers: bool,
    pub workflow_pins: bool,
    pub extra: HashMap<String, bool>,
}

impl Default for ChecksConfig {
    fn default() -> Self {
        ChecksConfig {
            documentation: true,
            well_known: true,
            build_system: true,
            spdx_headers: true,
            workflow_pins: true,
            extra: HashMap::new(),
        }
    }
}

impl ChecksConfig {
    /// Resolve whether the check with this stable id is enabled.
    ///
    /// A per-id entry in `[checks]` wins; otherwise the owning category
    /// toggle applies; unknown ids default to enabled (a typo must not
    /// silently disable a check — the id simply never matches).
    pub fn enabled(&self, check_id: &str) -> bool {
        if let Some(toggle) = self.extra.get(check_id) {
            return *toggle;
        }
        match check_id {
            "readme" | "license-file" | "security-policy" | "gitignore" | "coc"
            | "contributing" | "changelog" => self.documentation,
            "wellknown-core" => self.well_known,
            "justfile" | "no-makefile" | "editorconfig" | "tool-versions" => self.build_system,
            "spdx-headers" => self.spdx_headers,
            "sha-pinned" => self.workflow_pins,
            _ => true,
        }
    }

    /// Set a toggle from a `[checks]` key. Known category aliases update
    /// the named field; anything else becomes a per-id entry.
    fn set(&mut self, key: &str, value: bool) {
        match key {
            "documentation" => self.documentation = value,
            "well_known" | "well-known" | "wellknown" => self.well_known = value,
            "build_system" | "build-system" | "buildsystem" => self.build_system = value,
            "spdx_headers" | "spdx-headers" | "spdx" => self.spdx_headers = value,
            "workflow_pins" | "workflow-pins" | "workflowpins" => self.workflow_pins = value,
            other => {
                self.extra.insert(other.to_string(), value);
            },
        }
    }
}

/// CONFIGURATION: the top-level policy record.
#[derive(Debug, Clone)]
pub struct Config {
    pub level: String, // Target RSR tier (bronze, silver, gold, platinum)
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
    /// LOADER: reconciles the physical `.aletheia.toml` file with
    /// the hardcoded system defaults. Missing or unreadable file means
    /// defaults — a policy file must never be load-bearing for a scan.
    pub fn load_config(repo_path: &Path) -> Config {
        Self::load_from_file(&repo_path.join(".aletheia.toml"))
    }

    /// Load policy from an explicit path (`--config`). Same fallback
    /// semantics as [`Config::load_config`].
    pub fn load_from_file(config_path: &Path) -> Config {
        if config_path.is_file() {
            if let Ok(file) = fs::File::open(config_path) {
                use std::io::Read;
                let mut content = String::new();
                // Cap the read so a hostile config cannot exhaust memory.
                if file
                    .take(MAX_CONFIG_BYTES)
                    .read_to_string(&mut content)
                    .is_ok()
                {
                    return Self::parse_from_string(&content);
                }
            }
        }
        Config::default()
    }

    /// Parse configuration from a TOML string.
    ///
    /// Section-aware keys (`[aletheia]`, `[checks]`, `[ignore]`) are read
    /// via [`parse_toml`]; the historical flat keys (`level`,
    /// `documentation`, `spdx_headers` at top level) are honoured too.
    fn parse_from_string(content: &str) -> Config {
        let mut config = Config::default();
        let sections = parse_toml(content);

        // Top-level (flat) keys, for backwards compatibility.
        if let Some(top) = sections.get("") {
            if let Some(level) = top.get("level").and_then(TomlValue::as_string) {
                config.level = level.to_string();
            }
            for key in ["documentation", "spdx_headers"] {
                if let Some(value) = top.get(key) {
                    config.checks.set(key, value.as_bool(true));
                }
            }
        }

        // [aletheia] section.
        if let Some(section) = sections.get("aletheia") {
            if let Some(level) = section.get("level").and_then(TomlValue::as_string) {
                config.level = level.to_string();
            }
        }

        // [checks] section: every key is a toggle.
        if let Some(section) = sections.get("checks") {
            for (key, value) in section {
                config.checks.set(key, value.as_bool(true));
            }
        }

        // [ignore] section: `files = [...]` (or a lone string).
        if let Some(section) = sections.get("ignore") {
            if let Some(files) = section.get("files") {
                config.ignore.patterns = files.as_array();
            }
        }

        config
    }
}

/// Parse a single scalar value (string, bool or integer).
fn parse_scalar(value_str: &str) -> TomlValue {
    let value_str = value_str.trim();
    let quoted = |quote: char| {
        value_str.len() >= 2 && value_str.starts_with(quote) && value_str.ends_with(quote)
    };
    if quoted('"') || quoted('\'') {
        TomlValue::String(value_str[1..value_str.len() - 1].to_string())
    } else if let Ok(b) = value_str.parse::<bool>() {
        TomlValue::Boolean(b)
    } else if let Ok(i) = value_str.parse::<i64>() {
        TomlValue::Integer(i)
    } else {
        TomlValue::String(value_str.to_string())
    }
}

/// Parse an inline string array: `["a", "b"]`. Elements must be quoted;
/// unquoted elements are taken literally (TOML-strictness is deliberately
/// not the goal — resilience is).
fn parse_inline_array(value_str: &str) -> Vec<String> {
    let inner = value_str.trim();
    // Only a complete single-line array is understood; an unterminated
    // (multi-line) array yields no patterns rather than a bogus one.
    let Some(inner) = inner.strip_prefix('[').and_then(|s| s.strip_suffix(']')) else {
        return Vec::new();
    };
    inner
        .split(',')
        .map(|item| {
            let item = item.trim().trim_matches('"').trim_matches('\'');
            item.to_string()
        })
        .filter(|item| !item.is_empty())
        .collect()
}

/// PARSER: a line-based state machine for TOML key-value pairs.
///
/// Understands `[section]` headers, `key = value` pairs, `#` comments,
/// quoted strings, booleans, integers and single-line string arrays.
/// Keys before any section header land in the `""` section. A `#` inside
/// a quoted string does not start a comment.
pub fn parse_toml(content: &str) -> HashMap<String, HashMap<String, TomlValue>> {
    let mut sections: HashMap<String, HashMap<String, TomlValue>> = HashMap::new();
    sections.insert(String::new(), HashMap::new());
    let mut current_section = String::new();

    for line in content.lines() {
        let trimmed = strip_comment(line).trim();
        if trimmed.is_empty() {
            continue;
        }

        if trimmed.starts_with('[') && trimmed.ends_with(']') {
            current_section = trimmed[1..trimmed.len() - 1].trim().to_string();
            sections.entry(current_section.clone()).or_default();
        } else if let Some(eq_idx) = trimmed.find('=') {
            let key = trimmed[..eq_idx].trim().to_string();
            let value_str = trimmed[eq_idx + 1..].trim();
            if key.is_empty() || value_str.is_empty() {
                continue;
            }
            let value = if value_str.starts_with('[') {
                TomlValue::Array(parse_inline_array(value_str))
            } else {
                parse_scalar(value_str)
            };
            if let Some(section) = sections.get_mut(&current_section) {
                section.insert(key, value);
            }
        }
    }

    sections
}

/// Strip a trailing `#` comment, honouring single/double quotes.
fn strip_comment(line: &str) -> &str {
    let mut in_single = false;
    let mut in_double = false;
    for (idx, ch) in line.char_indices() {
        match ch {
            '\'' if !in_double => in_single = !in_single,
            '"' if !in_single => in_double = !in_double,
            '#' if !in_single && !in_double => return &line[..idx],
            _ => {},
        }
    }
    line
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
    fn test_config_sections() {
        let toml = r#"
[aletheia]
level = "silver"

[checks]
readme = false
spdx_headers = false

[ignore]
files = ["fixtures/**", "*.log"]
"#;
        let config = Config::parse_from_string(toml);
        assert_eq!(config.level, "silver");
        assert!(!config.checks.enabled("readme"));
        assert!(config.checks.enabled("license-file"));
        assert!(!config.checks.enabled("spdx-headers"));
        assert_eq!(config.ignore.patterns, vec!["fixtures/**", "*.log"]);
    }

    #[test]
    fn test_config_category_toggle_disables_members() {
        let toml = "[checks]\ndocumentation = false\n";
        let config = Config::parse_from_string(toml);
        assert!(!config.checks.enabled("readme"));
        assert!(!config.checks.enabled("changelog"));
        assert!(config.checks.enabled("spdx-headers"));
    }

    #[test]
    fn test_config_per_id_overrides_category() {
        let toml = "[checks]\ndocumentation = false\nreadme = true\n";
        let config = Config::parse_from_string(toml);
        assert!(config.checks.enabled("readme"));
        assert!(!config.checks.enabled("changelog"));
    }

    #[test]
    fn test_config_unknown_check_id_defaults_enabled() {
        let config = Config::default();
        assert!(config.checks.enabled("no-such-check"));
    }

    #[test]
    fn test_config_hash_inside_string_is_not_comment() {
        let toml = "[ignore]\nfiles = [\"releases/#42/**\"]\n";
        let config = Config::parse_from_string(toml);
        assert_eq!(config.ignore.patterns, vec!["releases/#42/**"]);
    }

    #[test]
    fn test_ignore_config_default() {
        let ignore = IgnoreConfig::default();
        assert!(ignore.patterns.is_empty());
    }

    #[test]
    fn test_ignore_is_ignored() {
        let ignore = IgnoreConfig {
            patterns: vec!["*.log".to_string(), "fixtures/**".to_string()],
        };
        assert!(ignore.is_ignored("debug.log"));
        assert!(ignore.is_ignored("nested/debug.log"));
        assert!(ignore.is_ignored("fixtures/input/data.bin"));
        assert!(!ignore.is_ignored("src/main.rs"));
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
    fn test_toml_value_array() {
        let value = TomlValue::Array(vec!["a".to_string(), "b".to_string()]);
        assert_eq!(value.as_array(), vec!["a".to_string(), "b".to_string()]);
        assert_eq!(
            TomlValue::String("solo".to_string()).as_array(),
            vec!["solo"]
        );
        assert!(TomlValue::Boolean(true).as_array().is_empty());
    }

    #[test]
    fn test_toml_value_as_bool() {
        assert!(TomlValue::Boolean(true).as_bool(false));
        assert!(!TomlValue::Boolean(false).as_bool(true));
        assert!(TomlValue::String("true".to_string()).as_bool(false));
        assert!(!TomlValue::String("FALSE".to_string()).as_bool(true));
        assert!(TomlValue::Integer(1).as_bool(false));
        assert!(!TomlValue::Integer(0).as_bool(true));
        assert!(TomlValue::Array(vec![]).as_bool(true));
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

    #[test]
    fn test_parse_toml_types_and_arrays() {
        let toml = "[s]\ncount = 7\nname = \"x\"\nflag = false\nfiles = [\"a\", \"b\"]\n";
        let parsed = parse_toml(toml);
        let section = &parsed["s"];
        assert_eq!(section["count"], TomlValue::Integer(7));
        assert_eq!(section["name"], TomlValue::String("x".to_string()));
        assert_eq!(section["flag"], TomlValue::Boolean(false));
        assert_eq!(
            section["files"],
            TomlValue::Array(vec!["a".to_string(), "b".to_string()])
        );
    }

    #[test]
    fn test_parse_toml_empty_array_and_skipped_lines() {
        let toml = "[s]\nfiles = []\n= oops\nlonely\nkey =\n";
        let parsed = parse_toml(toml);
        assert_eq!(parsed["s"]["files"], TomlValue::Array(vec![]));
        assert_eq!(parsed["s"].len(), 1);
    }

    #[test]
    fn test_parse_inline_array_unterminated_yields_empty() {
        // Review T5: `files = [` must not become a literal "[" pattern.
        assert!(parse_inline_array("[").is_empty());
        assert!(parse_inline_array("").is_empty());
        assert_eq!(
            parse_inline_array("[\"a\", \"b\"]"),
            vec!["a".to_string(), "b".to_string()]
        );
    }
}
