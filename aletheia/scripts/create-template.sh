#!/usr/bin/env bash
# SPDX-License-Identifier: MPL-2.0
# Copyright (c) Jonathan D.A. Jewell <j.d.a.jewell@open.ac.uk>
#
# create-template.sh — scaffold an RSR v2 project for a Tier-1 estate language.
#
# STANDALONE BY CONSTRUCTION: this generator reads only the template tree
# that ships beside it and writes only into the target directory. It makes
# no network calls, runs no package manager, and downloads nothing. A
# scaffolded project consequently passes `aletheia` Bronze AND Silver on
# day one with no hand edits, for every language it supports.
#
# Retired v1 shape this script no longer produces (see issue #186):
#   LICENSE.txt / lowercase justfile / flake.nix / .gitlab-ci.yml /
#   Python + flake8 / remote `curl` of licence texts.

set -euo pipefail

# Colours (suppressed when not a terminal, so CI logs stay clean)
if [ -t 1 ]; then
    RED='\033[0;31m'; GREEN='\033[0;32m'; YELLOW='\033[1;33m'
    BLUE='\033[0;34m'; NC='\033[0m'
else
    RED=''; GREEN=''; YELLOW=''; BLUE=''; NC=''
fi

log_info()  { printf '%b\n' "${GREEN}[INFO]${NC} $1"; }
log_warn()  { printf '%b\n' "${YELLOW}[WARN]${NC} $1"; }
log_error() { printf '%b\n' "${RED}[ERROR]${NC} $1" >&2; }
log_step()  { printf '%b\n' "${BLUE}[STEP]${NC} $1"; }

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ALETHEIA_DIR="$(dirname "$SCRIPT_DIR")"
TEMPLATE_ROOT="$ALETHEIA_DIR/templates"

# ---------------------------------------------------------------------------
# Language table
#
# Every language shares templates/common/ (docs, licence texts, .well-known,
# the estate CI gates) and adds templates/<lang>/ on top. The values below
# are the only things that differ; keeping them here means the estate's
# language policy is stated once, not six times.
#
# Tier-1 languages per hyperpolymath/standards (RSR_OUTLINE: "Tier 1 (Gold):
# Rust(+SPARK), Elixir, Zig, Ada, Haskell, AffineScript, Agda").
# ---------------------------------------------------------------------------
LANGS="rust zig elixir haskell ada agda"

lang_display() {
    case "$1" in
        rust)    echo "Rust/Creusot" ;;
        zig)     echo "Zig" ;;
        elixir)  echo "Elixir" ;;
        haskell) echo "Haskell" ;;
        ada)     echo "Ada/SPARK" ;;
        agda)    echo "Agda" ;;
    esac
}

lang_policy() {
    case "$1" in
        rust)    echo 'Rust here is always *Rust/Creusot*: Rust plus the Creusot deductive verifier. `just proof` translates `+src/impl.rs+` and discharges every obligation with Why3/Z3/CVC5, exiting non-zero on any unproved goal; the main crate stays zero-dependency because the specifications are gated behind `+cfg(creusot)+`.' ;;
        zig)     echo 'Zig is a Tier-1 estate language. The build is dependency-free: `zig build` compiles from this tree alone, and the Zig compiler ships its own libc, so there is nothing else to install.' ;;
        elixir)  echo 'Elixir is a Tier-1 estate language (BEAM/OTP). The project carries no Hex dependencies, so `mix compile` and `mix test` run with no network and nothing to fetch.' ;;
        haskell) echo 'Haskell is a Tier-1 estate language. The package depends only on `base`, which ships with GHC, so `cabal build --offline` resolves with no package index and no network.' ;;
        ada)     echo 'Ada here is *Ada/SPARK*: the core package declares `+pragma SPARK_Mode (On);+` and its contracts are statically proved by `gnatprove` at proof level 2, not merely checked at run time. The project builds with GNAT and plain GPR projects — no Alire, no crate index, nothing to fetch.' ;;
        agda)    echo 'Agda is a Tier-1 estate language, and the estate formal-methods emphasis (Coq, Agda, SPARK) makes it a first-class target. Typechecking *is* the verification: these modules import only `Agda.Builtin`, so there is no stdlib to resolve and `agda` proves them offline.' ;;
    esac
}

lang_invariant() {
    case "$1" in
        rust)    echo 'Rust here means Rust/Creusot — a specification that is not discharged by `just proof` is a bug, never a claim. The verified source is the shipped source: both crates `include!` `src/impl.rs`.' ;;
        zig)     echo 'Zig here means the Zig compiler toolchain: `zig fmt` and the built-in test runner are the gates, and no package manager sits in the build path.' ;;
        elixir)  echo 'Elixir here means the standard toolchain: `mix format` and ExUnit are the gates, and `deps` stays empty.' ;;
        haskell) echo 'Haskell here means GHC/Cabal, with `-Wall -Werror` as the static-analysis gate rather than extra linters to install.' ;;
        ada)     echo 'Ada here means GNAT + GPR projects, with `-gnatwa -gnatwe` (all warnings, warnings as errors) as the build gate and `gnatprove --level=2 --checks-as-errors` as the proof gate. `--checks-as-errors` matters: without it gnatprove exits 0 on an unproved check.' ;;
        agda)    echo 'Agda here means machine-checked proofs, not tests with a proof-shaped comment: a proof that does not typecheck must fail the gate.' ;;
    esac
}

lang_dep_invariant() {
    case "$1" in
        rust)    echo 'the crate depends only on `std`. Adding a dependency breaks the air-gapped guarantee and RSR Bronze.' ;;
        zig)     echo 'there is no package manager in the build path and no `build.zig.zon`. Adding a dependency means creating one with `zig fetch --save` — a deliberate decision, not a default.' ;;
        elixir)  echo '`mix.exs` declares `deps: []`. Adding a Hex dependency makes the build require the network.' ;;
        haskell) echo 'the package depends only on `base`, which ships with GHC. Any further `build-depends` makes `--offline` fail.' ;;
        ada)     echo 'the GPR projects reference no external crates and do not use Alire, so nothing is fetched at build time.' ;;
        agda)    echo 'the modules import only `Agda.Builtin`. Adding a `depend:` line to the `.agda-lib` makes the check require that library to be installed.' ;;
    esac
}

lang_gitattr() {
    case "$1" in
        rust)    echo '*.rs    text eol=lf diff=rust' ;;
        zig)     echo '*.zig   text eol=lf' ;;
        elixir)  echo '*.ex *.exs text eol=lf' ;;
        haskell) echo '*.hs    text eol=lf' ;;
        ada)     echo '*.adb *.ads text eol=lf' ;;
        agda)    echo '*.agda  text eol=lf' ;;
    esac
}

# ---------------------------------------------------------------------------
# Defaults
# ---------------------------------------------------------------------------
PROJECT_NAME=""
LANGUAGE="rust"
DESCRIPTION=""
AUTHOR_NAME="Jonathan D.A. Jewell"
AUTHOR_EMAIL="j.d.a.jewell@open.ac.uk"
REPO_URL=""
SECURITY_CONTACT=""
CONDUCT_CONTACT=""
INIT_GIT=1
FORCE=0
RUN_VERIFY=0

usage() {
    cat <<'EOF'
Usage: create-template.sh <project-name> [options]

Scaffold an RSR v2-compliant project for a Tier-1 estate language. The
generated tree passes `aletheia` Bronze AND Silver with no hand edits, and
needs no network: the scaffold embeds every licence text and downloads
nothing.

Arguments:
    project-name              Name of the new project (required)

Options:
    -l, --lang <language>     Template language (default: rust)
        --list-languages      List supported languages and exit
    -d, --description <text>  One-line project description
        --author <name>       Copyright holder / maintainer name
        --email <email>       Maintainer email
        --repo-url <url>      Canonical repository URL
        --security-contact <email>  Vulnerability disclosure contact
        --conduct-contact <email>   Code-of-conduct contact
        --no-git              Do not initialise a git repository
        --verify              Run `aletheia` on the result (must be on PATH)
        --force               Write into a directory that already exists
    -h, --help                Show this message

Supported languages (Tier-1 per hyperpolymath/standards):
    rust       Rust/Creusot — zero-dependency crate + Creusot proof crate
    zig        Zig 0.16 — build.zig, no package manager in the build path
    elixir     Elixir 1.18 / OTP 27 — mix, empty deps
    haskell    GHC 9.6 — cabal, `base` only
    ada        Ada — GNAT + plain GPR projects (no Alire)
    agda       Agda — machine-checked proofs, Agda.Builtin only

Every template provides (v2 shape):
    LICENSE + LICENSES/         MPL-2.0 (+ CC-BY-SA-4.0 for docs), full texts
    Justfile                    capital J, real recipes, failing loudly
    source + tests              working code and a real test gate
    .github/workflows/          CI, hypatia-scan, governance (actions.lock)
    .machine_readable/          rsr-profile.a2ml capability declaration
    .well-known/                security.txt, ai.txt, humans.txt
    0-AI-MANIFEST.a2ml          agent front door
    Documentation               README.adoc, SECURITY, CONTRIBUTING, CoC,
                                CHANGELOG, MAINTAINERS

Every template deliberately does NOT provide (retired v1 shape):
    LICENSE.txt, lowercase justfile, flake.nix, .gitlab-ci.yml,
    Makefile/Dockerfile, remote licence downloads

NOT SUPPORTED, deliberately:
    AffineScript — the estate successor to ReScript (`.res` -> `.affine`).
    It has no verifiable toolchain yet, so a scaffold could not pass its own
    `just check`; shipping one would be exactly the fake gate this estate
    bans. Add it here once its compiler is installable and pinned.

Examples:
    create-template.sh my-service
    create-template.sh my-service -l zig -d "A small service" --verify
    create-template.sh my-service --no-git --repo-url https://codeberg.org/me/my-service
EOF
    exit "${1:-1}"
}

list_languages() {
    printf 'Supported languages:\n\n'
    for l in $LANGS; do
        printf '  %-8s %s\n' "$l" "$(lang_display "$l")"
    done
    printf '\nUnsupported, deliberately: affinescript (no verifiable toolchain yet)\n'
}

# ---------------------------------------------------------------------------
# Argument parsing
# ---------------------------------------------------------------------------
parse_args() {
    while [ $# -gt 0 ]; do
        case "$1" in
            -h|--help|help) usage 0 ;;
            --list-languages|languages) list_languages; exit 0 ;;
            -l|--lang|--language)
                [ $# -ge 2 ] || { log_error "$1 requires a value"; exit 2; }
                LANGUAGE="$2"; shift 2 ;;
            -d|--description)
                [ $# -ge 2 ] || { log_error "$1 requires a value"; exit 2; }
                DESCRIPTION="$2"; shift 2 ;;
            --author)
                [ $# -ge 2 ] || { log_error "$1 requires a value"; exit 2; }
                AUTHOR_NAME="$2"; shift 2 ;;
            --email)
                [ $# -ge 2 ] || { log_error "$1 requires a value"; exit 2; }
                AUTHOR_EMAIL="$2"; shift 2 ;;
            --repo-url)
                [ $# -ge 2 ] || { log_error "$1 requires a value"; exit 2; }
                REPO_URL="$2"; shift 2 ;;
            --security-contact)
                [ $# -ge 2 ] || { log_error "$1 requires a value"; exit 2; }
                SECURITY_CONTACT="$2"; shift 2 ;;
            --conduct-contact)
                [ $# -ge 2 ] || { log_error "$1 requires a value"; exit 2; }
                CONDUCT_CONTACT="$2"; shift 2 ;;
            --no-git)   INIT_GIT=0; shift ;;
            --verify)   RUN_VERIFY=1; shift ;;
            --force)    FORCE=1; shift ;;
            -*)         log_error "unknown option: $1"; usage 2 ;;
            *)
                if [ -z "$PROJECT_NAME" ]; then
                    PROJECT_NAME="$1"
                else
                    log_error "unexpected extra argument: $1"; usage 2
                fi
                shift ;;
        esac
    done

    [ -n "$PROJECT_NAME" ] || { log_error "project name is required"; usage 2; }
}

# ---------------------------------------------------------------------------
# Validation
# ---------------------------------------------------------------------------
lang_supported() {
    for l in $LANGS; do [ "$l" = "$1" ] && return 0; done
    return 1
}

validate() {
    case "$PROJECT_NAME" in
        *[!a-zA-Z0-9._-]*|"")
            log_error "invalid project name: '$PROJECT_NAME'"
            log_error "use letters, digits, dot, underscore or hyphen"
            exit 2 ;;
        -*)
            log_error "project name may not start with a hyphen: '$PROJECT_NAME'"
            exit 2 ;;
    esac

    # Identifiers derived from the name must be valid for the target language:
    # a leading digit is not, so reject it here rather than emit code that
    # cannot compile.
    case "$PROJECT_NAME" in
        [0-9]*) log_error "project name may not start with a digit: '$PROJECT_NAME'"; exit 2 ;;
    esac

    if ! lang_supported "$LANGUAGE"; then
        log_error "unsupported language: '$LANGUAGE'"
        case "$LANGUAGE" in
            python|py|typescript|ts|go|golang|rescript|res)
                log_error "Note: .py/.ts/.go/.res sources are BANNED by the v2"
                log_error "estate language policy, so this template never passed"
                log_error "Bronze. That v1 path was removed, not ported." ;;
            affinescript|affine)
                log_error "Note: AffineScript is the estate's intended successor,"
                log_error "but it has no verifiable toolchain yet, so a scaffold"
                log_error "could not pass its own 'just check'." ;;
        esac
        printf '\n'
        list_languages
        exit 2
    fi

    if [ -e "$PROJECT_NAME" ] && [ "$FORCE" -ne 1 ]; then
        log_error "directory '$PROJECT_NAME' already exists (use --force to reuse)"
        exit 1
    fi

    for dir in "$TEMPLATE_ROOT/common" "$TEMPLATE_ROOT/$LANGUAGE"; do
        if [ ! -d "$dir" ]; then
            log_error "template not found: $dir"
            log_error "is this script running from an intact aletheia checkout?"
            exit 1
        fi
    done
}

# ---------------------------------------------------------------------------
# Rendering
# ---------------------------------------------------------------------------
# Escape the characters sed treats specially on the right-hand side, so
# values containing '&', '|' or '\' cannot corrupt the output.
sed_escape() { printf '%s' "$1" | sed -e 's/[\\&|]/\\&/g'; }

# Substitute every placeholder in stdin -> stdout.
subst() {
    sed \
        -e "s|@@PROJECT_NAME@@|$E_PROJECT_NAME|g" \
        -e "s|@@MOD_NAME@@|$E_MOD_NAME|g" \
        -e "s|@@CRATE_NAME@@|$E_MOD_NAME|g" \
        -e "s|@@MOD_CAMEL@@|$E_MOD_CAMEL|g" \
        -e "s|@@MOD_ADA@@|$E_MOD_ADA|g" \
        -e "s|@@VERSION@@|0.1.0|g" \
        -e "s|@@PROJECT_DESCRIPTION@@|$E_DESCRIPTION|g" \
        -e "s|@@REPO_URL@@|$E_REPO_URL|g" \
        -e "s|@@AUTHOR_NAME@@|$E_AUTHOR_NAME|g" \
        -e "s|@@AUTHOR_EMAIL@@|$E_AUTHOR_EMAIL|g" \
        -e "s|@@SECURITY_CONTACT@@|$E_SECURITY_CONTACT|g" \
        -e "s|@@CONDUCT_CONTACT@@|$E_CONDUCT_CONTACT|g" \
        -e "s|@@DATE@@|$E_DATE|g" \
        -e "s|@@SECURITY_EXPIRES@@|$E_SECURITY_EXPIRES|g" \
        -e "s|@@LANG_DISPLAY@@|$E_LANG_DISPLAY|g" \
        -e "s|@@LANG_POLICY@@|$E_LANG_POLICY|g" \
        -e "s|@@LANG_INVARIANT@@|$E_LANG_INVARIANT|g" \
        -e "s|@@DEP_INVARIANT@@|$E_DEP_INVARIANT|g" \
        -e "s|@@STANDALONE_INVARIANT@@|$E_STANDALONE|g" \
        -e "s|@@LANG_GITATTR_LINE@@|$E_LANG_GITATTR|g"
}

# Copy one template layer into the target, then render every file. Called
# once for templates/common and once for templates/<lang>; the language
# layer is copied second so it wins where both define a path.
copy_layer() {
    local layer="$1" target="$2"
    local src rel
    while IFS= read -r src; do
        rel="${src#"$layer"/}"
        # Recreate the directory skeleton (mkdir -p handles nesting).
        mkdir -p "$target/$(dirname "$rel")"
        cp "$src" "$target/$rel"
        # Preserve the executable bit. Written as an `if`, not
        # `[ -x ] && chmod`, because under `set -e` a failing test as the
        # last command of a loop body would abort the whole render.
        if [ -x "$src" ]; then
            chmod +x "$target/$rel"
        fi
    done < <(find "$layer" -type f)
}

render_tree() {
    local target="$1"

    # File contents.
    local f
    while IFS= read -r f; do
        subst < "$f" > "$f.tmp"
        mv "$f.tmp" "$f"
    done < <(find "$target" -type f)

    # Path names, deepest first, so renamed directories stay consistent.
    local p new
    while IFS= read -r p; do
        new="$(printf '%s' "$p" | subst)"
        if [ "$p" != "$new" ]; then
            mv "$p" "$new"
        fi
    done < <(find "$target" -depth -name '*@@*')
}

finish() {
    local target="$1"

    if [ "$INIT_GIT" -eq 1 ]; then
        log_step "Initialising git repository..."
        ( cd "$target"
          git init -q 2>/dev/null || true
          # Only set local identity when the user has none, so the initial
          # commit cannot fail on a fresh machine.
          git config user.email >/dev/null 2>&1 || git config user.email "$AUTHOR_EMAIL"
          git config user.name  >/dev/null 2>&1 || git config user.name  "$AUTHOR_NAME"
          git add -A
          git commit -q -m "chore: initial commit from aletheia v2 Bronze template" \
            || log_warn "initial commit skipped"
        )
        log_info "Git repository initialised"
    fi

    if [ "$RUN_VERIFY" -eq 1 ]; then
        log_step "Verifying RSR compliance..."
        if command -v aletheia >/dev/null 2>&1; then
            if aletheia "$target"; then
                log_info "Compliance check passed"
            else
                log_warn "Compliance check reported failures (see output above)"
            fi
        else
            log_warn "aletheia not on PATH — skipping verification"
            log_warn "build it with: just build  (from the maa-framework root)"
        fi
    fi
}

main() {
    parse_args "$@"
    validate

    # Derived values
    MOD_NAME="$(printf '%s' "$PROJECT_NAME" | tr '[:upper:]' '[:lower:]' | tr '-' '_')"
    MOD_CAMEL="$(printf '%s' "$PROJECT_NAME" | sed -E 's/[^a-zA-Z0-9]+/ /g' \
        | awk '{ for (i = 1; i <= NF; i++) printf toupper(substr($i,1,1)) substr($i,2) }')"
    # Ada unit names are underscore-joined capitalised segments. Deriving this
    # from MOD_CAMEL by inserting underscores at case boundaries is wrong for
    # single-letter segments ("g-ada" would give "GAda", not "G_Ada"), which
    # then makes GNAT look for gada.ads while the template ships g_ada.ads.
    MOD_ADA="$(printf '%s' "$PROJECT_NAME" | sed -E 's/[^a-zA-Z0-9]+/ /g' \
        | awk '{ for (i = 1; i <= NF; i++) printf "%s%s", (i > 1 ? "_" : ""), toupper(substr($i,1,1)) substr($i,2) }')"

    if [ -z "$DESCRIPTION" ]; then
        DESCRIPTION="An RSR v2-compliant $(lang_display "$LANGUAGE") project."
    fi
    if [ -z "$REPO_URL" ]; then
        REPO_URL="https://github.com/hyperpolymath/$PROJECT_NAME"
    fi
    if [ -z "$SECURITY_CONTACT" ]; then
        SECURITY_CONTACT="$AUTHOR_EMAIL"
    fi
    if [ -z "$CONDUCT_CONTACT" ]; then
        CONDUCT_CONTACT="$AUTHOR_EMAIL"
    fi
    DATE="$(date -u +%Y-%m-%d)"
    SECURITY_EXPIRES="$(date -u -d '+1 year' +%Y-%m-%dT%H:%M:%S.000Z 2>/dev/null \
        || date -u -v+1y +%Y-%m-%dT%H:%M:%S.000Z)"

    E_PROJECT_NAME="$(sed_escape "$PROJECT_NAME")"
    E_MOD_NAME="$(sed_escape "$MOD_NAME")"
    E_MOD_CAMEL="$(sed_escape "$MOD_CAMEL")"
    E_MOD_ADA="$(sed_escape "$MOD_ADA")"
    E_DESCRIPTION="$(sed_escape "$DESCRIPTION")"
    E_REPO_URL="$(sed_escape "$REPO_URL")"
    E_AUTHOR_NAME="$(sed_escape "$AUTHOR_NAME")"
    E_AUTHOR_EMAIL="$(sed_escape "$AUTHOR_EMAIL")"
    E_SECURITY_CONTACT="$(sed_escape "$SECURITY_CONTACT")"
    E_CONDUCT_CONTACT="$(sed_escape "$CONDUCT_CONTACT")"
    E_DATE="$(sed_escape "$DATE")"
    E_SECURITY_EXPIRES="$(sed_escape "$SECURITY_EXPIRES")"
    E_LANG_DISPLAY="$(sed_escape "$(lang_display "$LANGUAGE")")"
    E_LANG_POLICY="$(sed_escape "$(lang_policy "$LANGUAGE")")"
    E_LANG_INVARIANT="$(sed_escape "$(lang_invariant "$LANGUAGE")")"
    E_DEP_INVARIANT="$(sed_escape "$(lang_dep_invariant "$LANGUAGE")")"
    E_STANDALONE="$(sed_escape 'the build fetches nothing. Do not add network calls to the build, test, or scaffolding path.')"
    E_LANG_GITATTR="$(sed_escape "$(lang_gitattr "$LANGUAGE")")"

    log_info "Creating RSR v2 project: $PROJECT_NAME ($LANGUAGE — $(lang_display "$LANGUAGE"))"
    log_info "Template: v2 Bronze (standalone — no downloads)"
    printf '\n'

    if [ -e "$PROJECT_NAME" ]; then
        rm -rf "$PROJECT_NAME"
    fi
    mkdir -p "$PROJECT_NAME"

    log_step "Rendering template tree..."
    copy_layer "$TEMPLATE_ROOT/common" "$PROJECT_NAME"
    copy_layer "$TEMPLATE_ROOT/$LANGUAGE" "$PROJECT_NAME"
    render_tree "$PROJECT_NAME"

    # A leftover placeholder means a template file references a token the
    # generator does not know about. Fail loudly rather than ship it.
    if grep -rl '@@' "$PROJECT_NAME" >/dev/null 2>&1; then
        log_error "unresolved template placeholder(s) in:"
        grep -rl '@@' "$PROJECT_NAME" | sed 's/^/    /' >&2
        log_error "the generator and the template tree are out of sync"
        exit 1
    fi
    log_info "Rendered $(find "$PROJECT_NAME" -type f | wc -l | tr -d ' ') files"

    finish "$PROJECT_NAME"

    printf '\n'
    log_info "Project created: $PROJECT_NAME"
    printf '\n'
    echo "Next steps:"
    echo "  cd $PROJECT_NAME"
    echo "  just check          # build, test, lint (offline)"
    echo "  just verify         # RSR compliance with aletheia"
    echo ""
    echo "Language: $(lang_display "$LANGUAGE") — see README.adoc for the"
    echo "toolchain and the offline guarantees."
    echo ""
    echo "Before publishing, review:"
    echo "  .well-known/security.txt   disclosure contact and expiry"
    echo "  .machine_readable/rsr-profile.a2ml   declared capabilities"
    echo "  MAINTAINERS.adoc           ownership"
    echo ""
}

main "$@"
