// SPDX-License-Identifier: MPL-2.0
// Copyright (c) Jonathan D.A. Jewell <j.d.a.jewell@open.ac.uk>
//! RSR Compliance Verification Kernel.
//!
//! This module implements the deterministic checks used by Aletheia to
//! audit repository state. It performs physical filesystem analysis to
//! validate documentation, build system files, and security configurations.
//!
//! SOURCE OF TRUTH: every check below cites the upstream RSR criterion
//! it evaluates (see [`SSOT_PROVENANCE`]). Aletheia covers the *offline
//! file-presence subset* — criteria whose verdict can be computed from
//! the working tree with zero dependencies. Everything else (Scorecard,
//! secret-scanner depth, content carve-outs, containers, proofs, …) is
//! explicitly out of scope and deferred to the hypatia oracle; see
//! [`LOCAL_SUBSET_NOTE`]. Aletheia is non-normative by design.

use std::cell::Cell;
use std::fs;
use std::path::{Path, PathBuf};

use crate::config::{Config, IgnoreConfig};
use crate::types::*;

/// Provenance of the check inventory: the upstream single source of
/// truth this file is aligned to. When the criteria rev, update this
/// block and the inventory together — never one without the other.
pub const SSOT_PROVENANCE: &str = "RSR criteria v2.1.0-draft (2026-09-19), \
    hyperpolymath/standards 0-canon/rsr/rsr-criteria-v2.a2ml \
    (sha256 6a5aa885…1694d82a); normative oracle hypatia:rsr-conformance";

/// Honest scope statement, printed in verbose output so nobody mistakes
/// the local subset for the normative tier.
pub const LOCAL_SUBSET_NOTE: &str = "Local subset: offline file-presence checks only. \
    Scorecard, SBOM, containers, Guix, git hooks, signed commits, formal proofs, \
    dependency audit and content carve-outs (https-only, weak hashes, deep secret \
    scanning) are evaluated by the hypatia rsr-conformance oracle, not here.";

/// Bound on files examined per walk. Repos beyond this get a `warning`
/// (results partial) rather than a silent pass.
const MAX_SCAN_FILES: usize = 50_000;
/// Maximum recursion depth for filesystem walks.
const MAX_SCAN_DEPTH: u32 = 16;
/// Maximum depth for the symlink security sweep.
const MAX_SYMLINK_DEPTH: u32 = 6;
/// Maximum entries examined by the symlink security sweep.
const MAX_SYMLINK_ENTRIES: usize = 20_000;
/// Content reads are capped at 64 KiB per file.
const MAX_CONTENT_BYTES: u64 = 64 * 1024;
/// SPDX detection reads the first 8 KiB (headers live in lines 1–10).
const SPDX_HEAD_BYTES: u64 = 8 * 1024;

/// Directory names never descended into (build outputs, caches, VCS).
const SKIP_DIR_NAMES: &[&str] = &[
    ".git",
    "target",
    "node_modules",
    "_build",
    ".zig-cache",
    "zig-out",
    "zig-cache",
    "deps",
    "vendor",
    "dist",
    "build",
    "out",
    ".cache",
    ".venv",
    "__pycache__",
    ".hypatia",
    ".idea",
    ".vscode",
    ".tox",
    ".nox",
    "coverage",
    ".dart_tool",
    ".gradle",
    "DerivedData",
    "Pods",
];

/// Source-code extensions examined for SPDX headers.
const SPDX_EXTENSIONS: &[&str] = &[
    "rs", "sh", "bash", "zsh", "ksh", "py", "pyi", "js", "mjs", "cjs", "jsx", "ts", "mts", "cts",
    "tsx", "ex", "exs", "erl", "hrl", "gleam", "res", "resi", "go", "java", "kt", "kts", "swift",
    "c", "h", "cc", "hh", "cpp", "hpp", "cxx", "hxx", "cs", "fs", "fsi", "hs", "lhs", "ml", "mli",
    "idr", "adb", "ads", "pl", "pm", "rb", "php", "lua", "r", "jl", "scala", "zig", "zon", "nix",
    "scm", "rkt", "v", "el", "ps1",
];

/// Config extensions additionally scanned for committed secrets.
/// Documentation formats (md/adoc/txt) are deliberately excluded: they
/// routinely show redacted key *examples*, which a naive scan would flag.
const SECRET_SCAN_EXTENSIONS: &[&str] = &[
    "rs", "sh", "bash", "zsh", "toml", "yaml", "yml", "json", "env", "ini", "cfg", "conf", "py",
    "js", "ts", "go", "java", "kt", "ex", "exs", "c", "h", "cpp", "zig", "nix", "pem", "key",
    "p12", "pfx", "keystore", "jks",
];

/// Filenames that must never be committed (v2 4.1.2, file half).
const SECRET_FILENAMES: &[&str] = &[".env", ".env.local", "credentials.json"];

/// Fragment pairs concatenating into the committed-secret markers
/// (v2 4.1.2, content half): private-key block headers and unmistakable
/// token prefixes. Deliberately narrow — anything fuzzier belongs to the
/// oracle's secret scanner.
///
/// WHY FRAGMENTS: if a full marker appeared here as one literal, this
/// scanner would flag its own source file — the definition would contain
/// the pattern it detects (self-hosting paradox). Each half is innocuous
/// alone; only their concatenation is ever matched, and that full string
/// appears nowhere in this crate. Keep it that way: never write a whole
/// marker in any scanned source file, including comments and tests (build
/// test fixtures from halves too).
const SECRET_MARKER_FRAGMENTS: &[(&str, &str)] = &[
    ("-----BEGIN RSA ", "PRIVATE KEY-----"),
    ("-----BEGIN OPENSSH ", "PRIVATE KEY-----"),
    ("-----BEGIN EC ", "PRIVATE KEY-----"),
    ("-----BEGIN DSA ", "PRIVATE KEY-----"),
    ("-----BEGIN ENCRYPTED ", "PRIVATE KEY-----"),
    ("-----BEGIN ", "PRIVATE KEY-----"),
    ("aws_secret_access", "_key"),
    ("AK", "IA"),
    ("xo", "xb-"),
    ("xo", "xp-"),
    ("gh", "p_"),
    ("github_pa", "t_"),
];

/// Assemble the full matchable markers from their fragments.
fn secret_markers() -> Vec<String> {
    SECRET_MARKER_FRAGMENTS
        .iter()
        .map(|(head, tail)| format!("{head}{tail}"))
        .collect()
}

/// Banned build-system files (v2 1.1.3 + the Mustfile contractile,
/// which additionally bans Dockerfiles).
const BANNED_BUILD_FILES: &[&str] = &[
    "Makefile",
    "makefile",
    "GNUmakefile",
    "Dockerfile",
    "dockerfile",
];

/// Estate-banned language extensions (v2 5.1.1–5.1.5). `.v` is
/// content-sniffed (Coq vs V-lang) rather than blanket-banned — see
/// [`v_file_is_suspect`].
const BANNED_LANGUAGE_EXTENSIONS: &[&str] = &[
    "py", "pyi", "ts", "tsx", "mts", "cts", "res", "resi", "go", "v",
];

/// Markers identifying Coq sources (which share `.v` with V-lang).
const COQ_MARKERS: &[&str] = &[
    "Definition ",
    "Lemma ",
    "Theorem ",
    "Axiom ",
    "Require ",
    "Inductive ",
    "Fixpoint ",
    "Proposition ",
    "Corollary ",
    "Record ",
    "Class ",
    "Instance ",
    "Scheme ",
    "Ltac ",
    "Proof.",
    "Qed.",
    "Admitted.",
    "Section ",
    "Variable ",
    "Notation ",
];

/// Markers identifying V-lang sources.
const V_MARKERS: &[&str] = &["fn main", "\nfn ", "module main"];

/// Recognised licence-identifier anchors for the structural 7.1.1 check.
/// Real classification is the oracle's job (`cicd_rules/license_finding`);
/// this only asserts the LICENSE file names *something* known. Both SPDX
/// ids and spelled-out names are accepted: real LICENSE files often carry
/// only the full name (e.g. "Mozilla Public License Version 2.0" with no
/// "MPL-2.0" string anywhere).
const KNOWN_LICENSE_IDS: &[&str] = &[
    "Mozilla Public License",
    "MPL-2.0",
    "MIT License",
    "Permission is hereby granted",
    "Apache License",
    "Apache-2.0",
    "General Public License",
    "Lesser General Public License",
    "Affero General Public License",
    "GPL-",
    "LGPL-",
    "AGPL-",
    "Palimpsest",
    "PMPL-1.0",
    "Creative Commons",
    "CC-BY-SA",
    "CC0",
    "BSD-2-Clause",
    "BSD-3-Clause",
    "Redistribution and use",
    "ISC License",
    "Unlicense",
    "European Union Public Licence",
    "EUPL-",
    "Blue Oak",
    "BlueOak",
    "0BSD",
    "zlib/libpng",
];

/// Outcome of one check: pass/fail plus an optional remediation hint.
/// `ignored` marks "missing but covered by an `[ignore]` pattern", which
/// passes and is labelled `(ignored)` in reports.
struct Outcome {
    passed: bool,
    suggestion: Option<String>,
    ignored: bool,
}

impl Outcome {
    fn pass() -> Self {
        Outcome {
            passed: true,
            suggestion: None,
            ignored: false,
        }
    }

    fn pass_ignored() -> Self {
        Outcome {
            passed: true,
            suggestion: None,
            ignored: true,
        }
    }

    fn fail(suggestion: String) -> Self {
        Outcome {
            passed: false,
            suggestion: Some(suggestion),
            ignored: false,
        }
    }
}

/// Presence of a file looked up under several candidate paths.
enum Presence {
    Found,
    MissingIgnored,
    Missing,
}

/// ALGORITHM: Implements a glob pattern matcher for path filtering.
/// Supports `*` (zero or more chars, crossing `/`) and `?` (exactly one
/// char). Used for `[ignore]` patterns against repo-relative paths.
pub fn glob_match(pattern: &str, text: &str) -> bool {
    let pat: Vec<char> = pattern.chars().collect();
    let txt: Vec<char> = text.chars().collect();
    glob_match_recursive(&pat, &txt, 0, 0)
}

/// SECURITY: Validates that a path does not contain malicious symlinks.
/// Specifically checks if a symlink "escapes" the repository root, which
/// is a critical safety invariant for air-gapped or verified builds.
pub fn check_path_security(path: &Path, repo_root: &Path) -> PathCheckResult {
    let metadata = match fs::symlink_metadata(path) {
        Ok(m) => m,
        Err(_) => return PathCheckResult::default(),
    };

    if !metadata.file_type().is_symlink() {
        return PathCheckResult {
            exists: true,
            ..Default::default()
        };
    }

    // RESOLUTION: Determine the absolute target of the symlink.
    let target = match fs::read_link(path) {
        Ok(t) => t,
        Err(_) => {
            return PathCheckResult {
                exists: true,
                is_symlink: true,
                ..Default::default()
            }
        },
    };

    // ESCAPE DETECTION: Canonicalize and verify prefix.
    let canonical_root = repo_root.canonicalize().unwrap_or(repo_root.to_path_buf());
    let joined = if target.is_absolute() {
        target
    } else {
        match path.parent() {
            Some(parent) => parent.join(target),
            None => {
                return PathCheckResult {
                    exists: true,
                    is_symlink: true,
                    ..Default::default()
                }
            },
        }
    };
    // A dangling link cannot canonicalize; judge the joined path as-is.
    // Best-effort by construction — documented, not silent.
    let canonical_target = joined.canonicalize().unwrap_or(joined);

    PathCheckResult {
        exists: true,
        is_symlink: true,
        escapes_repo: !canonical_target.starts_with(&canonical_root),
        target: Some(canonical_target),
    }
}

/// Helper: Check if a file exists at repo_path/filename.
fn file_exists(repo_path: &Path, filename: &str) -> bool {
    repo_path.join(filename).is_file()
}

/// Parse `.gitmodules` content into submodule paths. Submodule contents
/// belong to their upstream repo; scanning them here would attribute
/// foreign failures to this repository.
fn parse_gitmodules_content(content: &str) -> Vec<String> {
    content
        .lines()
        .filter_map(|line| {
            let trimmed = line.trim();
            let rest = trimmed.strip_prefix("path")?.trim();
            let value = rest.strip_prefix('=')?.trim();
            if value.is_empty() {
                None
            } else {
                Some(value.to_string())
            }
        })
        .collect()
}

/// Read submodule paths from `.gitmodules` (absent file: no submodules).
fn submodule_paths(repo_path: &Path) -> Vec<String> {
    let path = repo_path.join(".gitmodules");
    match fs::read_to_string(&path) {
        Ok(content) => parse_gitmodules_content(&content),
        Err(_) => Vec::new(),
    }
}

/// Bounded file read: the first `max_bytes` bytes, lossy-decoded.
/// Returns `None` on I/O error so callers treat unreadable files as
/// "cannot verify" rather than crashing the scan.
fn read_head(path: &Path, max_bytes: u64) -> Option<String> {
    use std::io::Read;
    let file = fs::File::open(path).ok()?;
    let mut content = String::new();
    file.take(max_bytes).read_to_string(&mut content).ok()?;
    Some(content)
}

/// Filesystem scanner: bounded recursive walks honouring skip dirs,
/// submodule boundaries and `[ignore]` globs. Never follows symlinks.
struct Scanner<'a> {
    root: &'a Path,
    ignore: &'a IgnoreConfig,
    submodules: &'a [String],
    truncated: Cell<bool>,
}

impl<'a> Scanner<'a> {
    /// Repo-relative path with `/` separators ("" for the root itself).
    fn rel(&self, path: &Path) -> String {
        path.strip_prefix(self.root)
            .map(|p| p.to_string_lossy().replace('\\', "/"))
            .unwrap_or_default()
    }

    fn under_submodule(&self, rel: &str) -> bool {
        self.submodules
            .iter()
            .any(|s| rel == s || (rel.starts_with(s) && rel.as_bytes().get(s.len()) == Some(&b'/')))
    }

    fn skipped(&self, rel: &str) -> bool {
        rel.is_empty() || self.under_submodule(rel) || self.ignore.is_ignored(rel)
    }

    /// Recursive walk collecting files. `exts`, when `Some`, restricts
    /// to those (lowercased) extensions; `None` collects every file.
    fn walk_into(&self, dir: &Path, depth: u32, exts: Option<&[&str]>, out: &mut Vec<PathBuf>) {
        if depth > MAX_SCAN_DEPTH || out.len() >= MAX_SCAN_FILES {
            if out.len() >= MAX_SCAN_FILES {
                self.truncated.set(true);
            }
            return;
        }
        let entries = match fs::read_dir(dir) {
            Ok(entries) => entries,
            Err(_) => return,
        };
        for entry in entries.flatten() {
            if out.len() >= MAX_SCAN_FILES {
                self.truncated.set(true);
                return;
            }
            let path = entry.path();
            let rel = self.rel(&path);
            if self.skipped(&rel) {
                continue;
            }
            let file_type = match entry.file_type() {
                Ok(t) => t,
                Err(_) => continue,
            };
            // Never follow symlinks; the security sweep covers them.
            if file_type.is_symlink() {
                continue;
            }
            if file_type.is_dir() {
                let skip = path
                    .file_name()
                    .and_then(|name| name.to_str())
                    .is_some_and(|name| SKIP_DIR_NAMES.contains(&name));
                if !skip {
                    self.walk_into(&path, depth + 1, exts, out);
                }
            } else if file_type.is_file() {
                if let Some(wanted) = exts {
                    let ext = path
                        .extension()
                        .and_then(|e| e.to_str())
                        .unwrap_or_default()
                        .to_ascii_lowercase();
                    if !wanted.iter().any(|w| *w == ext) {
                        continue;
                    }
                }
                out.push(path);
            }
        }
    }

    fn walk_files(&self, exts: Option<&[&str]>) -> Vec<PathBuf> {
        let mut out = Vec::new();
        self.walk_into(self.root, 0, exts, &mut out);
        out
    }

    /// Look up a file under several candidate relative paths.
    fn presence(&self, candidates: &[&str]) -> Presence {
        let mut saw_ignored = false;
        for candidate in candidates {
            if file_exists(self.root, candidate) {
                return Presence::Found;
            }
            if self.ignore.is_ignored(candidate) {
                saw_ignored = true;
            }
        }
        if saw_ignored {
            Presence::MissingIgnored
        } else {
            Presence::Missing
        }
    }

    /// Sweep for symlinks escaping the repository root. Returns
    /// `(message, path)` pairs, all critical.
    fn scan_symlinks(&self) -> Vec<(String, Option<PathBuf>)> {
        let mut findings = Vec::new();
        let mut seen = 0usize;
        self.symlink_into(self.root, 0, &mut seen, &mut findings);
        findings
    }

    fn symlink_into(
        &self,
        dir: &Path,
        depth: u32,
        seen: &mut usize,
        findings: &mut Vec<(String, Option<PathBuf>)>,
    ) {
        if depth > MAX_SYMLINK_DEPTH || *seen >= MAX_SYMLINK_ENTRIES {
            return;
        }
        let entries = match fs::read_dir(dir) {
            Ok(entries) => entries,
            Err(_) => return,
        };
        for entry in entries.flatten() {
            if *seen >= MAX_SYMLINK_ENTRIES {
                return;
            }
            *seen += 1;
            let path = entry.path();
            let rel = self.rel(&path);
            if self.skipped(&rel) {
                continue;
            }
            let result = check_path_security(&path, self.root);
            if !result.exists {
                // TOCTOU: vanished between read_dir and stat. Skip it.
                continue;
            }
            if result.is_symlink && result.escapes_repo {
                findings.push((
                    format!(
                        "Symlink escapes repository: {} -> {}",
                        rel,
                        result
                            .target
                            .as_ref()
                            .map(|t| t.display().to_string())
                            .unwrap_or_else(|| "?".to_string())
                    ),
                    Some(path.clone()),
                ));
                continue;
            }
            // Recurse into real dirs only — never through a symlink.
            if !result.is_symlink && path.is_dir() {
                let skip = path
                    .file_name()
                    .and_then(|name| name.to_str())
                    .is_some_and(|name| SKIP_DIR_NAMES.contains(&name));
                if !skip {
                    self.symlink_into(&path, depth + 1, seen, findings);
                }
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Pure content predicates (unit-tested below).
// ---------------------------------------------------------------------------

/// A Justfile is "real" when it defines at least one recipe and is not
/// the estate stub. Recipe lines start at column 0 with a target name
/// and contain a `:`. (v2 1.1.2: "Justfile task runner present with
/// real recipes".)
fn justfile_is_real(content: &str) -> bool {
    if content.contains("not configured yet") {
        return false;
    }
    content.lines().any(|line| {
        let mut chars = line.chars();
        matches!(chars.next(), Some(c) if c.is_ascii_alphanumeric() || c == '_' || c == '-')
            && line.contains(':')
    })
}

/// Suspect V-lang iff V markers are present and no Coq markers are.
/// `.v` is shared by Coq and V; a blanket ban would false-positive every
/// proof file, so this heuristic stays narrow by design.
fn v_file_is_suspect(content: &str) -> bool {
    let has_v = V_MARKERS.iter().any(|m| content.contains(m));
    let has_coq = COQ_MARKERS.iter().any(|m| content.contains(m));
    has_v && !has_coq
}

/// `package.json` carries runtime deps iff it has a non-empty
/// `"dependencies"` object. (v2 5.1.6. `devDependencies` alone is fine.)
fn package_json_has_runtime_deps(content: &str) -> bool {
    let Some(start) = content.find("\"dependencies\"") else {
        return false;
    };
    let rest = &content[start + "\"dependencies\"".len()..];
    let mut chars = rest.chars();
    // Expect `:`, then `{`, then something other than `}`.
    for ch in chars.by_ref() {
        if ch == ':' {
            break;
        }
        if !ch.is_whitespace() {
            return false;
        }
    }
    for ch in chars.by_ref() {
        if ch == '{' {
            break;
        }
        if !ch.is_whitespace() {
            return false;
        }
    }
    for ch in chars {
        if ch == '}' {
            return false;
        }
        if !ch.is_whitespace() {
            return true;
        }
    }
    false
}

/// A recipe line is silent-skip iff it chains `|| echo` with a skip
/// message — the literal `|| echo SKIP` shape v2 6.1.5 names. Plain
/// `|| true` (common in cleanup recipes) is out of scope.
fn silent_skip_line(line: &str) -> bool {
    line.contains("|| echo") && (line.contains("SKIP") || line.contains("skip"))
}

/// Structural licence check: the text names a known identifier.
fn license_text_ok(content: &str) -> bool {
    KNOWN_LICENSE_IDS.iter().any(|id| content.contains(id))
}

// ---------------------------------------------------------------------------
// Checks. Each returns an Outcome; the inventory table in
// `verify_repository` binds ids, categories, items and tiers.
// Criterion refs point at rsr-criteria-v2.a2ml.
// ---------------------------------------------------------------------------

/// 1.1.1 scm-github: repository canonical on GitHub (`.github/` present).
fn check_scm_github(scanner: &Scanner) -> Outcome {
    if scanner.root.join(".github").is_dir() {
        Outcome::pass()
    } else {
        Outcome::fail("Create .github/ (workflows live there; v2 1.1.1).".to_string())
    }
}

/// 1.1.2 justfile: Justfile with real recipes.
fn check_justfile(scanner: &Scanner) -> Outcome {
    match scanner.presence(&["Justfile", "justfile"]) {
        Presence::Found => {
            let content = ["Justfile", "justfile"]
                .iter()
                .find_map(|name| read_head(&scanner.root.join(name), MAX_CONTENT_BYTES))
                .unwrap_or_default();
            if justfile_is_real(&content) {
                Outcome::pass()
            } else {
                Outcome::fail(
                    "Justfile has no real recipes (stub text or no targets); v2 1.1.2.".to_string(),
                )
            }
        },
        Presence::MissingIgnored => Outcome::pass_ignored(),
        Presence::Missing => Outcome::fail(
            "Add a Justfile with real recipes (build/test/lint at minimum); v2 1.1.2.".to_string(),
        ),
    }
}

/// 1.1.3 no-makefile (+ Mustfile: no Dockerfiles).
fn check_no_makefile(scanner: &Scanner) -> Outcome {
    let mut offenders = Vec::new();
    for path in scanner.walk_files(None) {
        if path
            .file_name()
            .and_then(|name| name.to_str())
            .is_some_and(|name| BANNED_BUILD_FILES.contains(&name))
        {
            offenders.push(scanner.rel(&path));
        }
    }
    if offenders.is_empty() {
        Outcome::pass()
    } else {
        offenders.sort();
        offenders.truncate(5);
        Outcome::fail(format!(
            "Remove banned build files ({}); use Justfile/Mustfile + Containerfile (v2 1.1.3).",
            offenders.join(", ")
        ))
    }
}

/// 2.1.1 readme-adoc: README.adoc (`.md` accepted; `.adoc` primary).
fn check_readme(scanner: &Scanner) -> Outcome {
    match scanner.presence(&["README.adoc", "README.md"]) {
        Presence::Found => Outcome::pass(),
        Presence::MissingIgnored => Outcome::pass_ignored(),
        Presence::Missing => Outcome::fail(
            "Add README.adoc (README.md accepted; .adoc is primary per v2 2.1.1).".to_string(),
        ),
    }
}

/// 2.1.2 license-file: LICENSE present (NOT LICENSE.txt — retired v1 shape).
fn check_license_file(scanner: &Scanner) -> Outcome {
    match scanner.presence(&["LICENSE", "LICENSE.md", "LICENSE.adoc"]) {
        Presence::Found => Outcome::pass(),
        Presence::MissingIgnored => Outcome::pass_ignored(),
        Presence::Missing => {
            if file_exists(scanner.root, "LICENSE.txt") {
                Outcome::fail(
                    "LICENSE.txt is the retired v1 shape: rename to LICENSE (+ LICENSES/ full texts); v2 2.1.2."
                        .to_string(),
                )
            } else {
                Outcome::fail("Add a LICENSE file (+ LICENSES/ full texts); v2 2.1.2.".to_string())
            }
        },
    }
}

/// 2.1.3 security-md: vulnerability-disclosure policy, root or .github/.
fn check_security_policy(scanner: &Scanner) -> Outcome {
    match scanner.presence(&[
        "SECURITY.md",
        "SECURITY.adoc",
        ".github/SECURITY.md",
        ".github/SECURITY.adoc",
    ]) {
        Presence::Found => Outcome::pass(),
        Presence::MissingIgnored => Outcome::pass_ignored(),
        Presence::Missing => Outcome::fail(
            "Add SECURITY.md with disclosure contact + response timeline; v2 2.1.3.".to_string(),
        ),
    }
}

/// 2.1.10 gitignore: .gitignore and .gitattributes present.
fn check_gitignore(scanner: &Scanner) -> Outcome {
    let has_ignore = file_exists(scanner.root, ".gitignore");
    let has_attributes = file_exists(scanner.root, ".gitattributes");
    if has_ignore && has_attributes {
        Outcome::pass()
    } else if scanner.ignore.is_ignored(".gitignore") && scanner.ignore.is_ignored(".gitattributes")
    {
        Outcome::pass_ignored()
    } else {
        let mut missing = Vec::new();
        if !has_ignore {
            missing.push(".gitignore");
        }
        if !has_attributes {
            missing.push(".gitattributes");
        }
        Outcome::fail(format!("Add {}; v2 2.1.10.", missing.join(" and ")))
    }
}

/// 4.1.1 spdx-headers: SPDX headers on all scanned source files.
fn check_spdx_headers(scanner: &Scanner) -> Outcome {
    let mut missing = Vec::new();
    for path in scanner.walk_files(Some(SPDX_EXTENSIONS)) {
        let headed = read_head(&path, SPDX_HEAD_BYTES)
            .map(|content| {
                content
                    .lines()
                    .take(10)
                    .any(|line| line.contains("SPDX-License-Identifier"))
            })
            .unwrap_or(false);
        if !headed {
            missing.push(scanner.rel(&path));
        }
    }
    if missing.is_empty() {
        Outcome::pass()
    } else {
        missing.sort();
        let shown: Vec<&str> = missing.iter().take(5).map(String::as_str).collect();
        let more = if missing.len() > 5 {
            format!(" (+{} more)", missing.len() - 5)
        } else {
            String::new()
        };
        Outcome::fail(format!(
            "Add SPDX-License-Identifier headers (first 10 lines) to: {}{}; v2 4.1.1.",
            shown.join(", "),
            more
        ))
    }
}

/// 4.1.2 no-secrets: no committed secret files or key material.
fn check_no_secrets(scanner: &Scanner) -> Outcome {
    let mut offenders = Vec::new();
    for path in scanner.walk_files(None) {
        if path
            .file_name()
            .and_then(|name| name.to_str())
            .is_some_and(|name| SECRET_FILENAMES.contains(&name))
        {
            offenders.push(scanner.rel(&path));
        }
    }
    let markers = secret_markers();
    for path in scanner.walk_files(Some(SECRET_SCAN_EXTENSIONS)) {
        if let Some(content) = read_head(&path, MAX_CONTENT_BYTES) {
            if markers.iter().any(|m| content.contains(m)) {
                offenders.push(scanner.rel(&path));
            }
        }
    }
    if offenders.is_empty() {
        Outcome::pass()
    } else {
        offenders.sort();
        offenders.dedup();
        offenders.truncate(5);
        Outcome::fail(format!(
            "Possible committed secrets in: {}. Remove + rotate; v2 4.1.2.",
            offenders.join(", ")
        ))
    }
}

/// 5.1.1–5.1.5 language bans: no Python/TS/ReScript/Go/V sources.
fn check_language_policy(scanner: &Scanner) -> Outcome {
    let mut offenders = Vec::new();
    for path in scanner.walk_files(Some(BANNED_LANGUAGE_EXTENSIONS)) {
        let ext = path
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or_default()
            .to_ascii_lowercase();
        if ext == "v" {
            // `.v` is shared with Coq: only flag likely V-lang.
            let suspect = read_head(&path, MAX_CONTENT_BYTES)
                .is_some_and(|content| v_file_is_suspect(&content));
            if suspect {
                offenders.push(scanner.rel(&path));
            }
            continue;
        }
        offenders.push(scanner.rel(&path));
    }
    if offenders.is_empty() {
        Outcome::pass()
    } else {
        offenders.sort();
        offenders.truncate(5);
        Outcome::fail(format!(
            "Banned-language sources present: {}. See estate language policy (v2 5.1.x); carve-outs are granted via hypatia, not here.",
            offenders.join(", ")
        ))
    }
}

/// 5.1.6 no-node-runtime: no npm-style runtime dependencies.
fn check_no_node_runtime(scanner: &Scanner) -> Outcome {
    let mut offenders = Vec::new();
    for path in scanner.walk_files(None) {
        if path
            .file_name()
            .and_then(|name| name.to_str())
            .is_some_and(|name| name == "package.json")
        {
            if let Some(content) = read_head(&path, MAX_CONTENT_BYTES) {
                if package_json_has_runtime_deps(&content) {
                    offenders.push(scanner.rel(&path));
                }
            }
        }
    }
    if offenders.is_empty() {
        Outcome::pass()
    } else {
        Outcome::fail(format!(
            "package.json with runtime dependencies: {}. Use the estate runtime policy (v2 5.1.6).",
            offenders.join(", ")
        ))
    }
}

/// 6.1.1 ci-present: CI pipeline with at least one workflow.
fn check_ci_present(scanner: &Scanner) -> Outcome {
    let workflows = scanner.root.join(".github/workflows");
    if workflows.is_dir() {
        let has_workflow = fs::read_dir(&workflows).is_ok_and(|entries| {
            entries.flatten().any(|entry| {
                entry
                    .path()
                    .extension()
                    .is_some_and(|ext| ext == "yml" || ext == "yaml")
            })
        });
        if has_workflow {
            return Outcome::pass();
        }
    }
    if scanner.ignore.is_ignored(".github/workflows") {
        Outcome::pass_ignored()
    } else {
        Outcome::fail("Add at least one workflow under .github/workflows/; v2 6.1.1.".to_string())
    }
}

/// 7.1.1 licence-classified (structural half): LICENSE names a known id.
fn check_licence_class(scanner: &Scanner) -> Outcome {
    let content = ["LICENSE", "LICENSE.md", "LICENSE.adoc", "LICENSE.txt"]
        .iter()
        .find_map(|name| read_head(&scanner.root.join(name), MAX_CONTENT_BYTES));
    match content {
        None => Outcome::fail("No LICENSE file to classify; v2 7.1.1.".to_string()),
        Some(text) if license_text_ok(&text) => Outcome::pass(),
        Some(_) => Outcome::fail(
            "LICENSE names no recognised identifier; use an estate-classified licence (v2 7.1.1)."
                .to_string(),
        ),
    }
}

/// 1.1.4 editorconfig.
fn check_editorconfig(scanner: &Scanner) -> Outcome {
    match scanner.presence(&[".editorconfig"]) {
        Presence::Found => Outcome::pass(),
        Presence::MissingIgnored => Outcome::pass_ignored(),
        Presence::Missing => Outcome::fail("Add .editorconfig; v2 1.1.4.".to_string()),
    }
}

/// 1.2.4 tool-versions.
fn check_tool_versions(scanner: &Scanner) -> Outcome {
    match scanner.presence(&[".tool-versions"]) {
        Presence::Found => Outcome::pass(),
        Presence::MissingIgnored => Outcome::pass_ignored(),
        Presence::Missing => {
            Outcome::fail("Add .tool-versions pinning toolchain versions; v2 1.2.4.".to_string())
        },
    }
}

/// 2.1.4 coc-md, root or .github/.
fn check_coc(scanner: &Scanner) -> Outcome {
    match scanner.presence(&[
        "CODE_OF_CONDUCT.md",
        "CODE_OF_CONDUCT.adoc",
        ".github/CODE_OF_CONDUCT.md",
        ".github/CODE_OF_CONDUCT.adoc",
    ]) {
        Presence::Found => Outcome::pass(),
        Presence::MissingIgnored => Outcome::pass_ignored(),
        Presence::Missing => Outcome::fail("Add CODE_OF_CONDUCT.md; v2 2.1.4.".to_string()),
    }
}

/// 2.1.5 contributing-md, root or .github/.
fn check_contributing(scanner: &Scanner) -> Outcome {
    match scanner.presence(&[
        "CONTRIBUTING.md",
        "CONTRIBUTING.adoc",
        ".github/CONTRIBUTING.md",
        ".github/CONTRIBUTING.adoc",
    ]) {
        Presence::Found => Outcome::pass(),
        Presence::MissingIgnored => Outcome::pass_ignored(),
        Presence::Missing => Outcome::fail("Add CONTRIBUTING.md; v2 2.1.5.".to_string()),
    }
}

/// 2.1.6 changelog: CHANGELOG.adoc or .md.
fn check_changelog(scanner: &Scanner) -> Outcome {
    match scanner.presence(&["CHANGELOG.adoc", "CHANGELOG.md"]) {
        Presence::Found => Outcome::pass(),
        Presence::MissingIgnored => Outcome::pass_ignored(),
        Presence::Missing => {
            Outcome::fail("Add CHANGELOG.adoc (Keep a Changelog); v2 2.1.6.".to_string())
        },
    }
}

/// 2.2.1 wellknown-core: security.txt + ai.txt + humans.txt.
fn check_wellknown_core(scanner: &Scanner) -> Outcome {
    let files = ["security.txt", "ai.txt", "humans.txt"];
    let missing: Vec<&str> = files
        .iter()
        .filter(|name| !file_exists(scanner.root, &format!(".well-known/{name}")))
        .copied()
        .collect();
    if missing.is_empty() {
        Outcome::pass()
    } else if scanner.ignore.is_ignored(".well-known") {
        Outcome::pass_ignored()
    } else {
        Outcome::fail(format!(
            "Add .well-known/{}; v2 2.2.1.",
            missing.join(", .well-known/")
        ))
    }
}

/// 2.3.1 ai-manifest: agent front door (`.deed` accepted mid-migration).
fn check_ai_manifest(scanner: &Scanner) -> Outcome {
    match scanner.presence(&["0-AI-MANIFEST.a2ml", "0-AI-MANIFEST.deed"]) {
        Presence::Found => Outcome::pass(),
        Presence::MissingIgnored => Outcome::pass_ignored(),
        Presence::Missing => {
            Outcome::fail("Add 0-AI-MANIFEST.a2ml agent front door; v2 2.3.1.".to_string())
        },
    }
}

/// 3.2.2 rsr-profile: capability declaration driving the applicable set.
fn check_rsr_profile(scanner: &Scanner) -> Outcome {
    match scanner.presence(&[
        ".machine_readable/rsr-profile.a2ml",
        ".machine_readable/rsr-profile.deed",
    ]) {
        Presence::Found => Outcome::pass(),
        Presence::MissingIgnored => Outcome::pass_ignored(),
        Presence::Missing => Outcome::fail(
            "Add .machine_readable/rsr-profile.a2ml declaring capabilities; v2 3.2.2.".to_string(),
        ),
    }
}

/// SECURITY: Decide whether one workflow line satisfies SHA pinning.
///
/// Returns `true` for any line that is not a `uses:` line, so callers can apply
/// this with `.any(|l| !uses_line_is_pinned(l))` over a whole file.
///
/// A `uses:` value is pinned only when the ref after the final `@` is exactly
/// 40 hexadecimal characters (a full-length Git SHA-1). `@v4`, `@main`,
/// `@master` and a bare action with no `@` at all are all unpinned.
///
/// Exempt (not pinnable, so treated as pinned):
///   - local actions and local reusable workflows — `./…`
///   - `docker://` image references, which use a different digest syntax
///
/// A trailing `# v4` provenance comment is ignored, so
/// `uses: actions/checkout@3d3c42e5… # v7.0.1` is correctly seen as pinned.
fn uses_line_is_pinned(line: &str) -> bool {
    let trimmed = line.trim();
    // Accept both `uses:` and list form `- uses:`.
    let rest = match trimmed
        .strip_prefix("uses:")
        .or_else(|| trimmed.strip_prefix("- uses:"))
    {
        Some(r) => r,
        None => return true, // not a uses: line — nothing to judge
    };

    // Strip the trailing provenance comment, then surrounding quotes.
    let value = rest.split('#').next().unwrap_or("").trim();
    let value = value.trim_matches(|c| c == '"' || c == '\'');

    if value.is_empty() {
        return true;
    }
    if value.starts_with("./") || value.starts_with(".\\") || value.starts_with("docker://") {
        return true;
    }

    match value.rsplit_once('@') {
        Some((_, git_ref)) => git_ref.len() == 40 && git_ref.chars().all(|c| c.is_ascii_hexdigit()),
        None => false, // no ref at all — unpinned
    }
}

/// 4.1.3 sha-pinned: GitHub Actions SHA pinning.
/// SECURITY: Rejects `uses: actions/checkout@v4`, requires
/// `uses: actions/checkout@<full-sha>`.
fn check_workflow_pins(scanner: &Scanner) -> Outcome {
    let workflows_path = scanner.root.join(".github/workflows");
    if !workflows_path.is_dir() {
        // No workflows: ci-present already fails at Bronze; stay silent
        // here rather than double-reporting the same gap.
        return Outcome::pass();
    }
    let entries = match fs::read_dir(&workflows_path) {
        Ok(entries) => entries,
        Err(_) => return Outcome::pass(),
    };
    let mut unpinned_files = Vec::new();
    for entry in entries.flatten() {
        let path = entry.path();
        if !path
            .extension()
            .is_some_and(|ext| ext == "yml" || ext == "yaml")
        {
            continue;
        }
        if let Some(content) = read_head(&path, MAX_CONTENT_BYTES) {
            if content.lines().any(|line| !uses_line_is_pinned(line)) {
                unpinned_files.push(scanner.rel(&path));
            }
        }
    }
    if unpinned_files.is_empty() {
        Outcome::pass()
    } else {
        unpinned_files.sort();
        unpinned_files.truncate(5);
        Outcome::fail(format!(
            "Pin actions to full 40-char SHAs in: {}; v2 4.1.3.",
            unpinned_files.join(", ")
        ))
    }
}

/// 6.1.2 hypatia-scan: self-scan workflow wired.
fn check_hypatia_scan(scanner: &Scanner) -> Outcome {
    match scanner.presence(&[".github/workflows/hypatia-scan.yml"]) {
        Presence::Found => Outcome::pass(),
        Presence::MissingIgnored => Outcome::pass_ignored(),
        Presence::Missing => Outcome::fail(
            "Wire .github/workflows/hypatia-scan.yml self-scan; v2 6.1.2.".to_string(),
        ),
    }
}

/// 6.1.3 governance-wf: estate-policy enforcement workflow.
fn check_governance_wf(scanner: &Scanner) -> Outcome {
    match scanner.presence(&[".github/workflows/governance.yml"]) {
        Presence::Found => Outcome::pass(),
        Presence::MissingIgnored => Outcome::pass_ignored(),
        Presence::Missing => Outcome::fail(
            "Wire .github/workflows/governance.yml estate-policy enforcement; v2 6.1.3."
                .to_string(),
        ),
    }
}

/// 7.1.2 reuse-compliant: LICENSES/ dir with full texts.
fn check_reuse(scanner: &Scanner) -> Outcome {
    let dir = scanner.root.join("LICENSES");
    let has_texts =
        dir.is_dir() && fs::read_dir(&dir).is_ok_and(|entries| entries.flatten().next().is_some());
    if has_texts {
        Outcome::pass()
    } else if scanner.ignore.is_ignored("LICENSES") {
        Outcome::pass_ignored()
    } else {
        Outcome::fail("Add LICENSES/ with full licence texts (REUSE style); v2 7.1.2.".to_string())
    }
}

/// 6.1.5 no-silent-skip (Gold): no `|| echo SKIP` silent-green recipes.
fn check_no_silent_skip(scanner: &Scanner) -> Outcome {
    let mut offenders = Vec::new();
    for path in scanner.walk_files(None) {
        let is_recipe = path
            .file_name()
            .and_then(|name| name.to_str())
            .is_some_and(|name| name == "Justfile" || name == "justfile");
        let is_script = path.extension().is_some_and(|ext| {
            ext == "just" || ext == "sh" || ext == "bash" || ext == "yml" || ext == "yaml"
        });
        if !(is_recipe || is_script) {
            continue;
        }
        if let Some(content) = read_head(&path, MAX_CONTENT_BYTES) {
            if content.lines().any(silent_skip_line) {
                offenders.push(scanner.rel(&path));
            }
        }
    }
    if offenders.is_empty() {
        Outcome::pass()
    } else {
        offenders.sort();
        Outcome::fail(format!(
            "Silent-skip pattern (|| echo SKIP) in: {}. Fail loudly instead; v2 6.1.5.",
            offenders.join(", ")
        ))
    }
}

/// Helper: Recursive implementation of glob_match.
fn glob_match_recursive(pattern: &[char], text: &[char], pi: usize, ti: usize) -> bool {
    if pi >= pattern.len() && ti >= text.len() {
        true
    } else if pi >= pattern.len() || ti >= text.len() {
        // A trailing `*` also matches the empty remainder ("a*" ~ "a").
        if pi < pattern.len() && ti >= text.len() && pattern[pi..].iter().all(|c| *c == '*') {
            return true;
        }
        false
    } else if pattern[pi] == '*' {
        // '*' matches zero or more characters (crossing `/`).
        glob_match_recursive(pattern, text, pi + 1, ti)
            || glob_match_recursive(pattern, text, pi, ti + 1)
    } else if pattern[pi] == '?' || pattern[pi] == text[ti] {
        // '?' matches any single character, or literal match
        glob_match_recursive(pattern, text, pi + 1, ti + 1)
    } else {
        false
    }
}

/// One inventory row: stable id, category, item label, required tier, check.
type InventoryRow = (
    &'static str,
    &'static str,
    &'static str,
    ComplianceLevel,
    fn(&Scanner) -> Outcome,
);

/// Record one evaluated check on the report, labelling `(ignored)` when
/// the pass comes from an `[ignore]` pattern rather than presence.
fn emit(
    report: &mut ComplianceReport,
    id: &str,
    category: &str,
    item: &str,
    level: ComplianceLevel,
    outcome: Outcome,
) {
    let item = if outcome.ignored {
        format!("{item} (ignored)")
    } else {
        item.to_string()
    };
    report.add_check(
        id,
        category,
        &item,
        outcome.passed,
        level,
        outcome.suggestion,
    );
}

/// VERIFY: run the local file-presence subset against a repository.
///
/// Checks disabled via `[checks]` are *skipped* (not recorded), so the
/// score denominator is always "checks actually evaluated". Symlink
/// escapes become `critical` warnings; scan truncation becomes a
/// `warning` — partial results are labelled, never silent.
pub fn verify_repository(repo_path: &Path, config: &Config) -> ComplianceReport {
    let mut report = ComplianceReport::new(repo_path.to_path_buf());
    let submodules = submodule_paths(repo_path);
    let scanner = Scanner {
        root: repo_path,
        ignore: &config.ignore,
        submodules: &submodules,
        truncated: Cell::new(false),
    };

    // (id, category, item, tier, check). Display order follows this table.
    let inventory: &[InventoryRow] = &[
        (
            "scm-github",
            "Infrastructure",
            ".github/ directory",
            ComplianceLevel::Bronze,
            check_scm_github,
        ),
        (
            "justfile",
            "Build System",
            "Justfile with real recipes",
            ComplianceLevel::Bronze,
            check_justfile,
        ),
        (
            "no-makefile",
            "Build System",
            "No Makefile / Dockerfile",
            ComplianceLevel::Bronze,
            check_no_makefile,
        ),
        (
            "readme",
            "Documentation",
            "README.adoc (or README.md)",
            ComplianceLevel::Bronze,
            check_readme,
        ),
        (
            "license-file",
            "Documentation",
            "LICENSE file (not LICENSE.txt)",
            ComplianceLevel::Bronze,
            check_license_file,
        ),
        (
            "security-policy",
            "Documentation",
            "SECURITY policy (root or .github/)",
            ComplianceLevel::Bronze,
            check_security_policy,
        ),
        (
            "gitignore",
            "Documentation",
            ".gitignore + .gitattributes",
            ComplianceLevel::Bronze,
            check_gitignore,
        ),
        (
            "spdx-headers",
            "Security",
            "SPDX license headers",
            ComplianceLevel::Bronze,
            check_spdx_headers,
        ),
        (
            "no-secrets",
            "Security",
            "No secrets committed",
            ComplianceLevel::Bronze,
            check_no_secrets,
        ),
        (
            "language-policy",
            "Language Policy",
            "No banned languages",
            ComplianceLevel::Bronze,
            check_language_policy,
        ),
        (
            "no-node-runtime",
            "Language Policy",
            "No Node/npm runtime deps",
            ComplianceLevel::Bronze,
            check_no_node_runtime,
        ),
        (
            "ci-present",
            "CI/CD",
            "CI pipeline (.github/workflows/)",
            ComplianceLevel::Bronze,
            check_ci_present,
        ),
        (
            "licence-class",
            "Licensing",
            "LICENSE carries recognized identifier",
            ComplianceLevel::Bronze,
            check_licence_class,
        ),
        (
            "editorconfig",
            "Build System",
            ".editorconfig",
            ComplianceLevel::Silver,
            check_editorconfig,
        ),
        (
            "tool-versions",
            "Build System",
            ".tool-versions",
            ComplianceLevel::Silver,
            check_tool_versions,
        ),
        (
            "coc",
            "Documentation",
            "CODE_OF_CONDUCT (root or .github/)",
            ComplianceLevel::Silver,
            check_coc,
        ),
        (
            "contributing",
            "Documentation",
            "CONTRIBUTING (root or .github/)",
            ComplianceLevel::Silver,
            check_contributing,
        ),
        (
            "changelog",
            "Documentation",
            "CHANGELOG.adoc (or .md)",
            ComplianceLevel::Silver,
            check_changelog,
        ),
        (
            "wellknown-core",
            "Well-Known",
            ".well-known core files",
            ComplianceLevel::Silver,
            check_wellknown_core,
        ),
        (
            "ai-manifest",
            "Machine-Readable",
            "0-AI-MANIFEST.a2ml",
            ComplianceLevel::Silver,
            check_ai_manifest,
        ),
        (
            "rsr-profile",
            "Machine-Readable",
            "rsr-profile.a2ml",
            ComplianceLevel::Silver,
            check_rsr_profile,
        ),
        (
            "sha-pinned",
            "Security",
            "GitHub Actions SHA pinning",
            ComplianceLevel::Silver,
            check_workflow_pins,
        ),
        (
            "hypatia-scan",
            "CI/CD",
            "hypatia-scan.yml workflow",
            ComplianceLevel::Silver,
            check_hypatia_scan,
        ),
        (
            "governance-wf",
            "CI/CD",
            "governance.yml workflow",
            ComplianceLevel::Silver,
            check_governance_wf,
        ),
        (
            "reuse",
            "Licensing",
            "LICENSES/ with full texts",
            ComplianceLevel::Silver,
            check_reuse,
        ),
        (
            "no-silent-skip",
            "CI/CD",
            "No silent-skip in recipes",
            ComplianceLevel::Gold,
            check_no_silent_skip,
        ),
    ];

    for (id, category, item, level, run) in inventory {
        if config.checks.enabled(id) {
            let outcome = run(&scanner);
            emit(&mut report, id, category, item, *level, outcome);
        }
    }

    for (message, path) in scanner.scan_symlinks() {
        report.add_warning("critical", message, path);
    }

    if scanner.truncated.get() {
        report.add_warning(
            "warning",
            format!("Scan truncated at {MAX_SCAN_FILES} files; results are partial."),
            None,
        );
    }

    report
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_glob_match_literal() {
        assert!(glob_match("main.rs", "main.rs"));
        assert!(!glob_match("main.rs", "lib.rs"));
    }

    #[test]
    fn test_glob_match_star() {
        assert!(glob_match("*.rs", "main.rs"));
        assert!(glob_match("*.rs", "lib.rs"));
        assert!(!glob_match("*.rs", "main.toml"));
    }

    #[test]
    fn test_glob_match_star_crosses_dirs_and_trailing_star() {
        assert!(glob_match("*.log", "nested/debug.log"));
        assert!(glob_match("fixtures/**", "fixtures/input/data.bin"));
        assert!(glob_match("prefix*", "prefix"));
        assert!(glob_match("src/*", "src/main.rs"));
    }

    #[test]
    fn test_glob_match_question() {
        assert!(glob_match("file?.rs", "file1.rs"));
        assert!(glob_match("file?.rs", "filex.rs"));
        assert!(!glob_match("file?.rs", "file.rs"));
    }

    #[test]
    fn test_glob_match_complex() {
        assert!(glob_match("*.rs", "test.rs"));
        assert!(glob_match("src/*.rs", "src/main.rs"));
    }

    #[test]
    fn test_path_check_result_default() {
        let result: PathCheckResult = Default::default();
        assert!(!result.exists);
        assert!(!result.is_symlink);
        assert!(!result.escapes_repo);
        assert!(result.target.is_none());
    }

    #[test]
    fn test_check_result_creation() {
        let check = CheckResult {
            id: "readme".to_string(),
            category: "Documentation".to_string(),
            item: "README.md".to_string(),
            passed: true,
            required_for: ComplianceLevel::Bronze,
            suggestion: None,
        };

        assert_eq!(check.id, "readme");
        assert_eq!(check.category, "Documentation");
        assert_eq!(check.item, "README.md");
        assert!(check.passed);
        assert_eq!(check.required_for, ComplianceLevel::Bronze);
    }

    #[test]
    fn test_security_warning_creation() {
        let warning = SecurityWarning {
            level: "critical".to_string(),
            message: "Symlink escapes repository".to_string(),
            path: None,
        };

        assert_eq!(warning.level, "critical");
        assert!(!warning.message.is_empty());
    }

    #[test]
    fn test_compliance_report_new() {
        let path = PathBuf::from("/tmp/test-repo");
        let report = ComplianceReport::new(path.clone());

        assert_eq!(report.repository_path, path);
        assert!(report.checks.is_empty());
        assert!(report.warnings.is_empty());
    }

    #[test]
    fn test_compliance_report_add_check() {
        let path = PathBuf::from("/tmp/test-repo");
        let mut report = ComplianceReport::new(path);

        report.add_check(
            "readme",
            "Documentation",
            "README.md",
            true,
            ComplianceLevel::Bronze,
            None,
        );

        assert_eq!(report.checks.len(), 1);
        assert_eq!(report.checks[0].id, "readme");
        assert_eq!(report.checks[0].category, "Documentation");
        assert_eq!(report.checks[0].item, "README.md");
        assert!(report.checks[0].passed);
    }

    #[test]
    fn test_compliance_report_add_warning() {
        let path = PathBuf::from("/tmp/test-repo");
        let mut report = ComplianceReport::new(path);

        report.add_warning("warning", "Test warning".to_string(), None);

        assert_eq!(report.warnings.len(), 1);
        assert_eq!(report.warnings[0].level, "warning");
    }

    #[test]
    fn test_compliant_at_builds_upward() {
        let mut report = ComplianceReport::new(PathBuf::from("/tmp/x"));
        report.add_check("a", "C", "a", true, ComplianceLevel::Bronze, None);
        report.add_check("b", "C", "b", false, ComplianceLevel::Silver, None);
        assert!(report.compliant_at(ComplianceLevel::Bronze));
        assert!(!report.compliant_at(ComplianceLevel::Silver));
        assert_eq!(report.score_pct(), 50.0);
    }

    #[test]
    fn test_tier_counts_covers_all_levels() {
        let report = ComplianceReport::new(PathBuf::from("/tmp/x"));
        let counts = report.tier_counts();
        assert_eq!(counts.len(), 4);
        assert_eq!(counts[3].0, ComplianceLevel::Platinum);
        assert_eq!(report.score_pct(), 100.0);
    }

    #[test]
    fn test_compliance_level_vocabulary() {
        assert_eq!(
            ComplianceLevel::from_v2_tier("bronze"),
            Some(ComplianceLevel::Bronze)
        );
        assert_eq!(
            ComplianceLevel::from_v2_tier("SILVER"),
            Some(ComplianceLevel::Silver)
        );
        assert_eq!(
            ComplianceLevel::from_v2_tier("gold"),
            Some(ComplianceLevel::Gold)
        );
        assert_eq!(
            ComplianceLevel::from_v2_tier("rhodium"),
            Some(ComplianceLevel::Platinum)
        );
        assert_eq!(
            ComplianceLevel::from_v2_tier("platinum"),
            Some(ComplianceLevel::Platinum)
        );
        assert_eq!(ComplianceLevel::from_v2_tier("copper"), None);
        assert_eq!(ComplianceLevel::Bronze.name(), "Bronze");
        assert!(ComplianceLevel::Bronze.rank() < ComplianceLevel::Platinum.rank());
    }

    #[test]
    fn test_file_exists() {
        // Anchor on CARGO_MANIFEST_DIR rather than the process working directory:
        // the crate root is fixed at compile time, so this is deterministic no
        // matter where the test binary is invoked from.
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));

        assert!(
            file_exists(root, "Cargo.toml"),
            "Cargo.toml exists at the crate root"
        );
        assert!(!file_exists(root, "no-such-file.does-not-exist"));
        // `is_file()`, not `exists()` — a directory must not count as a file.
        assert!(!file_exists(root, "src"), "a directory is not a file");
    }

    #[test]
    fn test_uses_line_is_pinned_accepts_full_sha() {
        // Real pins taken from this repository's own workflows.
        assert!(uses_line_is_pinned(
            "      - uses: actions/checkout@3d3c42e5aac5ba805825da76410c181273ba90b1"
        ));
        // A trailing provenance comment must not defeat the check.
        assert!(uses_line_is_pinned(
            "        uses: actions/deploy-pages@cd2ce8fcbc39b97be8ca5fce6e763baed58fa128 # v5.0.0"
        ));
    }

    #[test]
    fn test_uses_line_is_pinned_rejects_tags_and_branches() {
        // REGRESSION: the previous implementation was `line.contains("@v")
        // && !line.contains("@")`, which is unsatisfiable — every one of these
        // was silently reported as pinned.
        assert!(!uses_line_is_pinned("        uses: actions/checkout@v4"));
        assert!(!uses_line_is_pinned(
            "      - uses: actions/checkout@v7.0.1"
        ));
        assert!(!uses_line_is_pinned("        uses: some/action@main"));
        // The exact line that broke Governance and CodeQL on this repo.
        assert!(!uses_line_is_pinned(
            "        uses: SonarSource/sonarqube-scan-action@master"
        ));
        // A short/abbreviated SHA is not a full-length pin.
        assert!(!uses_line_is_pinned("        uses: foo/bar@3d3c42e"));
        // No ref at all.
        assert!(!uses_line_is_pinned("        uses: foo/bar"));
    }

    #[test]
    fn test_uses_line_is_pinned_ignores_non_uses_and_exempt_forms() {
        assert!(uses_line_is_pinned("      - name: Checkout"));
        assert!(uses_line_is_pinned("        run: cargo test"));
        assert!(uses_line_is_pinned(""));
        // Local actions and local reusable workflows cannot be SHA-pinned.
        assert!(uses_line_is_pinned("      - uses: ./.github/actions/setup"));
        assert!(uses_line_is_pinned(
            "    uses: ./.github/workflows/reusable.yml"
        ));
        // Docker refs use a different digest syntax; out of scope.
        assert!(uses_line_is_pinned("        uses: docker://alpine:3.20"));
    }

    #[test]
    fn test_justfile_is_real() {
        assert!(justfile_is_real("build:\n    cargo build\n"));
        assert!(justfile_is_real("# comment\n\ntest target:\n    echo ok\n"));
        assert!(!justfile_is_real(""));
        assert!(!justfile_is_real("# only a comment\n"));
        assert!(!justfile_is_real(
            "build:\n    @echo \"Build not configured yet\"\n"
        ));
        // Indented lines alone are not recipes.
        assert!(!justfile_is_real("    cargo build --release\n"));
    }

    #[test]
    fn test_v_file_is_suspect() {
        let coq = "Require Import List.\nDefinition ident (x : nat) := x.\n";
        assert!(!v_file_is_suspect(coq));
        let vlang = "module main\n\nfn main() {\n\tprintln('hi')\n}\n";
        assert!(v_file_is_suspect(vlang));
        // No markers either way: not evidence of V.
        assert!(!v_file_is_suspect("// empty\n"));
    }

    #[test]
    fn test_package_json_has_runtime_deps() {
        assert!(package_json_has_runtime_deps(
            r#"{"dependencies": {"left-pad": "^1.0.0"}}"#
        ));
        assert!(package_json_has_runtime_deps(
            "{\"dependencies\" :\n{\n\"x\": \"1\"\n}\n}"
        ));
        assert!(!package_json_has_runtime_deps(
            r#"{"devDependencies": {"vitest": "^1.0.0"}}"#
        ));
        assert!(!package_json_has_runtime_deps(r#"{"dependencies": {}}"#));
        assert!(!package_json_has_runtime_deps(r#"{"name": "x"}"#));
    }

    #[test]
    fn test_silent_skip_line() {
        assert!("test -f x || echo SKIP".lines().any(silent_skip_line));
        assert!("run || echo skipping optional step"
            .lines()
            .any(silent_skip_line));
        assert!(!"rm -rf tmp || true".lines().any(silent_skip_line));
        assert!(!"echo done".lines().any(silent_skip_line));
    }

    #[test]
    fn test_license_text_ok() {
        assert!(license_text_ok("Mozilla Public License Version 2.0"));
        assert!(license_text_ok(
            "SPDX-License-Identifier: PMPL-1.0-or-later"
        ));
        assert!(!license_text_ok("All rights reserved. No licence given."));
    }

    #[test]
    fn test_parse_gitmodules_content() {
        let content = "[submodule \"absolute-zero\"]\n\tpath = absolute-zero\n\turl = https://example.test/x.git\n";
        assert_eq!(parse_gitmodules_content(content), vec!["absolute-zero"]);
        assert!(parse_gitmodules_content("").is_empty());
        assert!(parse_gitmodules_content("path =\n").is_empty());
    }

    #[test]
    fn test_compliance_level_equality() {
        assert_eq!(ComplianceLevel::Bronze, ComplianceLevel::Bronze);
        assert_ne!(ComplianceLevel::Bronze, ComplianceLevel::Silver);
    }
}
