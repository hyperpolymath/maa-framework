<!-- SPDX-License-Identifier: MPL-2.0 -->
# Verification

How to verify locally and what CI does.

## Local

The `verification/` directory contains the top-level scripts.

```bash
# One-shot: install deps + run full verification
./verification/setup-and-verify.sh

# After deps are installed
./verification/run-local-verification.sh

# Just the proof suite
./verification/verify-proofs.sh
```

Or via `Justfile` recipes (more granular):

```bash
just build-all        # everything
just build-coq        # Coq only
just build-lean       # Lean 4 only
just build-agda
just build-isabelle
just build-rescript
just verify           # build + check
```

## Per-prover commands

### Coq
```bash
cd proofs/coq/common && coqc CNO.v
cd ../physics && coqc -R ../common CNO StatMech.v
                 coqc -R ../common CNO LandauerDerivation.v
cd ../quantum && coqc -R ../common CNO QuantumMechanicsExact.v
cd ../malbolge && coqc -R ../common CNO MalbolgeCore.v
```

### Lean 4
```bash
cd proofs/lean4 && lake build
```

### Agda
```bash
cd proofs/agda && agda CNO.agda
```

### Idris2 ABI
```bash
idris2 --build absolute-zero-abi.ipkg
```

(Standalone DivMod check, useful for the issue #27 surface:)
```bash
idris2 --check src/abi/Proofs/DivMod.idr
```

## CI matrix

| Workflow | What it does | Status check name |
|----------|--------------|--------------------|
| `rust-ci.yml` | `cargo build --release`, `cargo audit`, coverage | `build`, `security`, `coverage` |
| `rescript-deno-ci.yml` | `deno lint`, `deno fmt --check`, `deno test`, `rescript build` | `build` |
| `codeql.yml` | CodeQL static analysis | `check` |
| `secret-scanner.yml` | trufflehog + gitleaks | `secrets` |
| `language-policy.yml` | Block new banned-language files | `check` |
| `governance.yml` | Estate-wide reusable governance bundle | `governance / *` |
| `hypatia-scan.yml` | Neurosymbolic CI/CD scan | (comment-only) |
| `cflite_pr.yml` | ClusterFuzzLite (address sanitizer) | `PR (address)` |
| `scorecard.yml` | OpenSSF Scorecard | (badge) |
| `jekyll-gh-pages.yml` | Deploy homepage | (deploy) |
| `publish-container.yml` | Build + push container image | (release) |

## Reproducible container

`Containerfile` at root pins toolchain versions. Build:

```bash
podman build -t absolute-zero:verify -f Containerfile .
podman run --rm absolute-zero:verify just verify
```

(Or `docker` in place of `podman`.)

## Status as of 2026-05-25

See [`.machine_readable/6a2/STATE.a2ml`](../../.machine_readable/6a2/STATE.a2ml)
for live status. Summary: Coq 11/11, Lean 4 1631/1632, Idris2 ABI
typechecks (DivMod standalone; Layout blocked by pre-existing
`Types.idr` errors — tracked as AUDIT-2026-05-20-A in
[AUDIT.adoc](../../AUDIT.adoc)).
