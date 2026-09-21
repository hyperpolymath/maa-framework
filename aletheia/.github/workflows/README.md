<!--
SPDX-License-Identifier: CC-BY-SA-4.0
SPDX-FileCopyrightText: 2026 Jonathan D.A. Jewell <j.d.a.jewell@open.ac.uk>
-->

# These 16 workflows do not run. They never have.

GitHub Actions only reads workflow files from `.github/workflows/` **at the root of a
repository**. This directory is nested inside `maa-framework`, so every YAML file here
is inert — including `rust-ci.yml`, `codeql.yml`, `cflite_pr.yml`, `cflite_batch.yml`,
`generator-generic-ossf-slsa3-publish.yml`, `scorecard.yml`, and `ghcr-publish.yml`.

## Why they are here

`aletheia` began as the standalone repository `hyperpolymath/aletheia` (created
2025-12-11), where these workflows *did* run. That repository was removed from GitHub —
its mirrors on GitLab, Codeberg and Bitbucket all stop by early January 2026. On
2026-02-21, commit `639f389` left behind a dangling gitlink with no `.gitmodules`
entry; on 2026-03-02, commit `25cf219` ("Fix stale submodule pointers after repo
cleanup") replaced that pointer by vendoring 361 files — these workflows among them —
into `maa-framework` as ordinary tracked files.

Nothing has executed them since.

## What that cost

Between 2026-06-17 and 2026-07-21, `aletheia` **did not compile**. Commit `b5322c2`
("security: remediate Track C and Track E findings") correctly added a 1 MiB read cap
to `Config::load_config`, but collapsed the block onto one line and dropped a closing
brace. A single `cargo build` would have caught it. Nothing ran one, so `main` stayed
broken for over a month.

## Where the real gate lives now

`/.github/workflows/rust-ci.yml`, at the repository root. (It went missing for a
while: the commit that moved the crate to `aletheia/` deleted the old root gate
without re-pointing it, so main had no Rust CI until the gate was restored on
2026-09-21.) It builds debug (`--all-targets`) and release, runs the full test
suite (66 unit + 45 integration and counting), checks formatting, runs clippy
with warnings denied (issue #125), and enforces the zero-dependency constraint
from `aletheia/CLAUDE.md`. Each job has a same-named recipe in the root Justfile
— `just check` runs the same steps locally. Read the workflow header comment for
what it deliberately does *not* gate yet.

## If you are changing CI for aletheia

Edit the root workflow. Editing anything in this directory has no effect. These files
are retained only as the source material for porting the capabilities that were lost —
ClusterFuzzLite fuzzing, SLSA3 provenance, GHCR publishing — back to root workflows.
Once a capability is ported, delete its file here so this directory shrinks toward
empty.
