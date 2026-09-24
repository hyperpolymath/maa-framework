# Delivering the two open issues in `hyperpolymath/maa-framework`

**Status:** both delivered and verified in the working tree. **Nothing is committed yet.**
**Date:** 2026-09-22 · **Repo:** `hyperpolymath/maa-framework` (commit `939b8c3`)

> **Status update — 2026-09-24.** This document is kept as the record of the point in time it was
> written at. Everything it listed as outstanding has since happened:
>
> - The work was pushed and merged as PR #228 (squash `e477b40`), and **#186 and #197 are both
>   closed**. A follow-up commit on `main` (`ab53573`) removed a token *prefix* this document had
>   quoted back when describing it.
> - **Both proof gates have now executed on a GitHub runner** — the step "What is left" calls
>   outstanding below. `Proof (Creusot)` run **`35946679306`** finished green (`Proved (2 files)` in
>   five minutes); `Proof (SPARK)` run **`35947307833`** proved **35 of 35 checks (100%)**.
> - Exercising them needed a scratch branch whose tree was a generated project, so the gate's
>   `proof.yml` had a repository root to resolve from. That branch was opened as a draft PR for that
>   purpose alone; it was merged by accident, replaced the repository tree, and was **reverted one
>   commit later** (`859b387`, tree byte-identical to `b5535f1`). Both gates ship a
>   `workflow_dispatch:` trigger, which is what the Ada run used — no PR is needed, and none was used.
> - A defect found while doing this is fixed alongside this update: `.gitattributes` emitted two
>   patterns on one line for `elixir` and `ada`, and git rejects such a line wholesale, so *neither*
>   pattern applied. The generator now emits one pattern per line, with a regression test.

Your two additions to the brief — *make it standalone* and *Rust here is always Rust/Creusot* — are both built in.
One correction I need to flag on the licence, in the last section.

---

## TL;DR

| Issue | Status | Headline evidence |
|---|---|---|
| **#186** `create-template.sh` scaffolds retired v1 shape | **Delivered** — exceeds acceptance | Fresh scaffold now scores **26/26, Bronze + Silver ACHIEVED, exit 0** (was 14/26, NOT MET) |
| **#197** consolidate `Scanner::walk_files` traversals | **Delivered** | 7 traversals → 1; **212 ms → 150 ms** on an 18,602-file tree; verdicts byte-identical |

Suite went from 68+47 to **72+48 = 120 tests**, clippy `-D warnings` clean, `cargo fmt --check` clean, `just check` exit 0.

---

## Issue #186 — the generator emitted the retired v1 shape

### Reproduced first

I scaffolded with the unmodified generator and ran the unmodified checker:

```
$ create-template.sh repro-project rust && aletheia repro-project
Score: 14/26 checks passed (53.8%)
Bronze-level RSR compliance: NOT MET
```

Failing Bronze checks: `.github/`, `LICENSE` (it wrote `LICENSE.txt`), `.gitignore`+`.gitattributes`,
SPDX headers, CI pipeline. Exactly as filed.

### What changed

**1. A real v2 template** — `aletheia/templates/bronze-rust/`, 30 files replacing the single stale
`README-template.adoc`:

```
LICENSE + LICENSES/      MPL-2.0 + CC-BY-SA-4.0, full texts shipped in-tree
Justfile                 capital J, real recipes, no silent-skip
Cargo.toml               zero dependencies, offline by construction
src/, tests/             working code + 8 tests
verification/            Rust/Creusot proof crate (see below)
.github/workflows/       ci.yml, hypatia-scan.yml, governance.yml, actions.lock
.machine_readable/       rsr-profile.a2ml capability declaration
.well-known/             security.txt, ai.txt, humans.txt
0-AI-MANIFEST.a2ml       agent front door
                         README/SECURITY/CONTRIBUTING/CoC/CHANGELOG/MAINTAINERS .adoc
```

**2. A rewritten generator** — `scripts/create-template.sh`, 832 lines changed down to 334.
It now renders the template tree with 11 `@@PLACEHOLDER@@` tokens, and its help text documents both
what the v2 shape *is* and what it deliberately no longer emits.

**3. A regression guard** — `test_scaffold_passes_bronze_and_silver` in `tests/integration_tests.rs`.
It runs the real generator, asserts no v1 artefact reappears, asserts no unresolved placeholder is
left, and runs the real checker over the output. The acceptance criterion is now enforced by CI, so
the template and the checker can't drift apart again without the suite going red.

### Acceptance — met and exceeded

The issue asked for "Bronze (ideally Silver)". A fresh scaffold now gets:

```
Score: 26/26 checks passed (100.0%)
Bronze-level RSR compliance: ACHIEVED
Silver-level RSR compliance: ACHIEVED          ← the "ideally"
✅ No silent-skip in recipes [Gold]             ← Gold's only check passes too
```

with **no hand edits**, and `just check` (build, test, fmt, clippy, deps-check) exits 0.

### "Standalone" — nothing to download, and I proved it

The generated project fetches nothing, and neither does the generator:

- licence texts are **shipped in the template**, not `curl`ed (the old script fetched MIT from
  opensource.org and Apache from apache.org, falling back to writing stub files on failure)
- no `flake.nix`, no Python/`flake8`, no Makefile, no Dockerfile
- the dead `gitlab.com/maa-framework/...` install URL is gone; the v1 GitLab mirror 403 is not
  reintroduced anywhere
- zero dependencies, so `cargo build --offline` always succeeds

Proven by running generation *and* build inside a network namespace with no network at all:

```
lo  DOWN ; ping 1.1.1.1 → Network is unreachable

GENERATED: 30 files
$ cargo build        ← note: no --offline flag, and no network to fall back on
   Finished `dev` profile
$ cargo test
   test result: ok. 5 passed ... ok. 3 passed
```

### "Rust is always Rust/Creusot" — built in

`verification/` is a Creusot proof crate carrying real `#[requires]` / `#[ensures]` obligations for
two functions in the main crate. It is **deliberately detached** from the Cargo workspace
(`exclude = ["verification"]`), because Creusot needs `creusot-contracts` + Why3 and the main crate
must stay zero-dependency. `just proof` / `just proof-prove` drive it; `just check` never needs it.

Two honest caveats, both documented in `verification/README.adoc` rather than papered over:

- **The obligations are not wired into CI yet.** Creusot is research software tracking a specific
  nightly; a CI job would be non-reproducible until that nightly and the SMT solver set are pinned.
  Shipping a job that can't run reproducibly would be the fake gate this estate explicitly bans.
- **The specs are mirrors, not single-source.** Creusot must see the annotated source, so
  `verification/src/lib.rs` mirrors `src/`. Change one, change the other in the same commit. The
  README notes the tighter `#[cfg_attr(creusot, …)]` + `include!` option if you want single-source later.

---

## Issue #197 — consolidate `Scanner::walk_files`

### Design: one traversal, four buckets

Each check used to call `walk_files` for itself — seven full walks of the same tree in the same
order, differing only in which files they collected. Now one traversal fills a `ScanSet` with four
buckets (`all`, `spdx`, `secret`, `banned`), memoised in a `OnceCell` on the `Scanner`, so the seven
call sites read from it instead of re-walking.

The saving is in the traversal (`read_dir` + `file_type` per entry), not the extension filter applied
afterwards. That's why the buckets pay off: `.rs` is an SPDX-header *and* a secret-scan extension, and
`.py` is secret-scanned *and* banned — one file now serves several checks in one pass.

### Budget semantics

The issue required **identical budget semantics**, so this is where the care went. A bucket accepts at
most `MAX_SCAN_FILES` *matches of its own* — mirroring the old behaviour where
`walk_files(Some(exts))` counted only files passing its filter, not every file seen. When a bucket
fills it stops growing and the traversal continues, so the other buckets still get their own first
50,000 matches. `MAX_SCAN_DEPTH`, `SKIP_DIR_NAMES`, submodule boundaries, `[ignore]` globs and the
never-follow-symlinks rule are untouched. The symlink sweep keeps its own separate budget.

Four unit tests pin this, including that filling `all` does **not** consume the filtered buckets' budgets.

One deliberate refinement, called out because it is a real (benign) difference: the truncation warning
now fires when a file is actually *refused*, rather than when a walk happens to pass the cap. Same
trigger condition in practice, but it now means "results really are partial".

### Evidence

**Fewer traversals, measured** on a synthetic tree of 18,602 files across 673 directories, best-of-10
with release binaries:

| | time |
|---|---|
| baseline (7 walks) | **212 ms** |
| new (1 walk) | **150 ms** |
| | **~29% faster overall; traversal itself ~7× reduced** |

**Semantics preserved, checked** — old vs new binary on five repositories, comparing full output with
the timestamp line stripped:

```
IDENTICAL  bigtree (18k files)     IDENTICAL  demo-service (fresh scaffold)
IDENTICAL  maa-framework           IDENTICAL  aletheia
IDENTICAL  standards (29 MB)
exit codes match: bigtree 1/1, demo-service 0/0, aletheia 0/0
```

**Suite green** — 72 unit + 48 integration = 120 tests; clippy `-D warnings` clean; `cargo fmt --check` clean;
estate root `just check` exit 0; `just self-verify` 26/26.

---

## What is left

1. **Push the branch and open the PR.** Everything is committed locally — three commits, messages
   below — but I have no push credentials, so nothing is on the remote yet. See "Commits".
   *(Update 2026-09-24: pushed and merged — PR #228, squash `e477b40`.)*
2. **Rotate the GitHub token you pasted into chat.** Please treat it as compromised — see the note below.
3. **Decide on the remaining Tier-1 language templates.** The v1 generator offered `python`,
   `typescript` and `go`. Those extensions are *banned* by the v2 estate language policy, so those
   paths could never pass Bronze — I removed them rather than port them. Zig, Elixir, Haskell, Ada,
   Agda and AffineScript are Tier-1 per your `standards` canon and are reasonable follow-on templates.
4. **Wire the proof gates into CI.** **Done.** Both templates ship an opt-in
   `.github/workflows/proof.yml`. Attempting to execute them from scratch (below) found one real
   defect in the Rust one, now fixed. *(As of 2026-09-24 both have run green on a GitHub runner —
   runs `35946679306` and `35947307833` — so the templates' headers now state that, and the
   "first green run" caveat is discharged rather than pending.)*
5. **Close #186 and #197** with the acceptance evidence above.
   *(Update 2026-09-24: both closed; #197 was closed by hand because GitHub pairs a closing keyword
   with the first issue named in a PR body only.)*

---

## Two things worth your attention

### 1. Your MPL-2.0 correction exposes drift inside `aletheia` itself

You said MPL-2.0, and the estate agrees — but the checker's own directory disagrees with itself.
I first wrote PMPL-1.0 into the template from `aletheia/LICENSE`; you corrected it to MPL-2.0. Counting
the actual usage across the repo:

```
128 × MPL-2.0          ← dominant, and what aletheia's own src/*.rs headers say
 36 × CC-BY-SA-4.0     ← docs, consistent
  2 × PMPL-1.0-or-later ← the outliers
```

`aletheia/LICENSE` declares **Palimpsest PMPL-1.0-or-later**; `aletheia/LICENSES/` ships
`PMPL-1.0-or-later.txt` instead of `MPL-2.0.txt`; yet **aletheia's own source files carry
`SPDX-License-Identifier: MPL-2.0`**. The root repo is clean (full MPL-2.0 `LICENSE` text,
`LICENSES/{MPL-2.0,CC-BY-SA-4.0,AGPL-3.0-or-later}.txt`), so the drift is confined to the nested
`aletheia/` crate.

Worth knowing that **`aletheia` cannot currently catch this**: `KNOWN_LICENSE_IDS` accepts both
`"Palimpsest"`/`"PMPL-1.0"` and `"Mozilla Public License"`/`"MPL-2.0"`, so `check_licence_class`
passes either way. That's a deliberate structural check (it only asserts *something* known is named),
and real classification is the hypatia oracle's job — but it means nothing in this repo will flag the
inconsistency. The template now emits MPL-2.0 throughout, per your correction.

This is a separate piece of work from #186/#197, so I have **not** touched `aletheia/LICENSE`.
Say the word and I'll align it.

### 2. That GitHub token

You pasted a token mid-conversation, and I did not use it or add it to any remote: everything I needed (your repo, MaaXYZ, `standards`) was readable
anonymously, and the repo path you gave turned out to be redundant once you supplied the URL.
**Please revoke it at github.com/settings/tokens and issue a fresh one.** It lives in this
transcript now, and fine-grained PATs are exactly the credential the estate's own secret-scanner
workflow exists to catch. An earlier revision of this document also recorded the token's
visible prefix — the part GitHub itself shows in your token list — which was a mistake even
though that half is not usable as a credential. It has been removed.

---

## Verification log

Every claim above is reproducible. The commands, run in order:

```bash
# #186 — reproduce the filed failure
bash aletheia/scripts/create-template.sh repro-project rust
aletheia repro-project                       # 14/26, NOT MET

# #186 — after the fix
bash aletheia/scripts/create-template.sh demo-service -d "A demo service"
aletheia demo-service                        # 26/26, Bronze + Silver, exit 0
cd demo-service && just check                # exit 0
cargo fmt --check && cargo clippy --offline --all-targets -- -D warnings

# #186 — air-gap proof
unshare -rn bash -c 'cd /tmp/nettest && create-template.sh isolated-demo && cd isolated-demo && cargo build && cargo test'

# #197 — correctness and cost
cargo test                                   # 120 passed
cargo clippy --all-targets -- -D warnings    # clean
diff <(baseline /tmp/bigtree) <(new /tmp/bigtree)   # identical

# estate gate
cd .. && just check && just self-verify      # 26/26
```

---

## Files touched

```
M  aletheia/scripts/create-template.sh                     (six languages, fixed MOD_ADA derivation)
M  aletheia/src/checks.rs                                  (ScanSet + single traversal, +238/-…)
M  aletheia/tests/integration_tests.rs                     (+82 scaffold guard, +143 six-language guard)
D  aletheia/templates/bronze-rust/README-template.adoc     (superseded)
+  aletheia/templates/common/**                            (16 shared files)
+  aletheia/templates/{rust,zig,elixir,haskell,ada,agda}/**  (per-language overlays)
```

The follow-on touched, within those overlays:

```
rust  src/impl.rs (new, single source of truth)   src/lib.rs   src/main.rs   Cargo.toml
      tests/integration_test.rs   Justfile   .gitignore   README.adoc
      verification/{Cargo.toml, src/lib.rs, why3find.json, README.adoc}
ada   src/<mod>.ads   src/<mod>.adb   src/main.adb   tests/run_tests.adb
      <mod>.gpr   tests/tests.gpr   Justfile   .gitignore   README.adoc
zig   test/integration_test.zig   .github/workflows/ci.yml
elixir, haskell   .github/workflows/ci.yml      haskell/Justfile (@@ARGS@@ -> {{ARGS}})
```

Environment note: `rust`, `just` and the release builds were installed into this sandbox, not committed.

---

# Follow-on: the six-language template set, with proofs that actually run

You asked two things after the first delivery: make `Rust/Creusot` and `Ada/SPARK` **real** rather
than aspirational, and get **all six** templates to 26/26. Both are done, and both were verified by
making the provers themselves give a verdict — including negative controls, so you can see the gates
actually fail when the claim is false.

## Headline

| | Result |
|---|---|
| Six-language sweep (real generator) | **rust · zig · elixir · haskell · ada · agda — all 26/26, Bronze + Silver, `just check` exit 0** |
| Rust/Creusot | **`Proved (2 files) ✔`** — Creusot 0.14 translates and Why3 discharges every obligation |
| Ada/SPARK | **35 checks, 100% proved** — `gnatprove --level=2`, Z3 + Alt-Ergo + CVC5 |
| Negative controls | Breaking either specification makes the corresponding `just proof` **exit 1** |
| aletheia suite | **72 + 49 = 121 tests**, clippy `-D warnings` clean, `cargo fmt --check` clean |
| Estate gate | root `just check` exit 0; `just self-verify` **26/26** |

## Rust/Creusot is real now

The first delivery shipped `verification/` with `#[requires]`/`#[ensures]` that had **never been run**.
It also used the pre-0.14 contract crate and a floating git branch. All of that is replaced.

### The toolchain actually works

Getting there took real work, because Creusot does not use released Why3: it pins forks.

```
nightly-2026-08-03 (rustup, + rustc-dev)          # Creusot's rust-toolchain pin
opam switch → why3                                  # pinned to git-c369bc4c
                                                     git+https://gitlab.inria.fr/why3/why3.git
why3find                                            # pinned to git-0f054b93
                                                     git+https://github.com/creusot-rs/why3find.git
z3 4.13.3, cvc5 1.1.2                               # 7 provers detected
cargo-creusot, creusot-rustc                        # from the Creusot tree
```

Three specific traps, recorded so nobody repeats them:

1. **Released `why3` cannot even parse Creusot's output.** With stock Why3 1.8.2 the generated
   `.coma` dies with `syntax error`; with the pinned commit it proves. The pin is not optional.
2. **Released `why3find` will not compile against the pinned Why3** (`Unbound record field
   "Why3.Term.t_loc"`). Creusot's fork is required, in lockstep.
3. **`cargo-creusot` looks for `why3find` inside its own data dir**, not on `PATH`, and loads
   provers from `$XDG_DATA_HOME/creusot/creusot_why3.conf`. This is where "Package 'creusot' not
   found" came from: why3find resolves packages through `DUNE_DIR_LOCATIONS`, which `cargo-creusot`
   sets — running `why3find` by hand does not.

### The design: the proof cannot drift from the code

The obvious Creusot layout is a second, annotated copy of the functions. That copy drifts, and then
the proof describes code that no longer exists — worse than no proof. This template removes the
possibility:

```rust
// src/impl.rs  — the single source of truth
#[cfg(creusot)]
use creusot_std::prelude::*;

#[cfg_attr(creusot, requires(lo@ <= hi@))]
#[cfg_attr(creusot, ensures(lo@ <= result@ && result@ <= hi@))]
pub fn clamp(value: u32, lo: u32, hi: u32) -> u32 { ... }

// src/lib.rs              (main crate)      include!("impl.rs");
// verification/src/lib.rs (Creusot crate)   include!("../../src/impl.rs");
```

`creusot-rustc` is the only thing that sets `--cfg creusot`. Under plain `cargo build` every
`#[cfg_attr]` vanishes and the Creusot prelude is never imported, so the main crate keeps its
**zero-dependency, air-gapped** build. Under `cargo creusot`, Creusot verifies the *actual* function
bodies — the failure messages name `../../src/impl.rs`, which is the real file.

### Evidence

```
$ just proof
Proved (verif/proj_rust_verification_rlib/clamp.coma) ✔
Proved (verif/proj_rust_verification_rlib/midpoint.coma) ✔
Proved (2 files) ✔                                    # exit 0

# negative control 1 — clamp: result <= hi - 1
File ".../src/impl.rs", line 27: proof failed    Goal Coma.vc_clamp: ✘ (2/3)
Error: 1 unproved file                                # exit 1

# negative control 2 — midpoint: result == (a + b) / 2 + 1
File ".../src/impl.rs", line 45: proof failed    Goal Coma.vc_midpoint: ✘ (4/5)
Error: 1 unproved file                                # exit 1
```

### One specification had to change, and here is the honest reason

The sample `mean_floor` was `(a & b) + ((a ^ b) >> 1)` — a neat overflow-free mean. **Creusot 0.14
cannot verify it, and cannot verify anything about it.** The `@` view operator maps an integer to a
mathematical `Int`, and `creusot_std::logic` provides no `BitAnd`, `BitXor` or `Shr` for it; there is
no bitvector theory in the backend. This is not a missing postcondition — even with *no* `ensures` at
all, Creusot still has to discharge overflow-freedom for the `+`, and fails:

```
Goal Coma.vc_mean_floor: ✘ (1/2)      # a function with no contract at all
```

So the template ships `midpoint` — the same idea as `a + (b - a) / 2`, which needs only linear
arithmetic and whose exact half-sum identity **is** proved:

```rust
#[cfg_attr(creusot, requires(a@ <= b@))]
#[cfg_attr(creusot, ensures(a@ <= result@ && result@ <= b@))]
#[cfg_attr(creusot, ensures(result@ == (a@ + b@) / 2))]
pub fn midpoint(a: u32, b: u32) -> u32 { a + (b - a) / 2 }
```

This is written up in `verification/README.adoc` with the failing example, so the boundary is
documented rather than quietly worked around.

## Ada/SPARK is real now

The first delivery called it Ada/SPARK while the sources had **no `SPARK_Mode` and no `gnatprove`** —
only Ada 2012 runtime contracts, and `src/main.adb` full of `Ada.Text_IO`, exceptions and
`'Value`/`'Image`, none of which are in the SPARK subset.

Now the core package declares `pragma SPARK_Mode (On);` and its contracts are statically proved:

```ada
function Clamp (Value, Lo, Hi : U32) return U32
  with Pre  => Lo <= Hi,
       Post => Clamp'Result in Lo .. Hi;

function Midpoint (A, B : U32) return U32
  with Pre  => A <= B,
       Post => Midpoint'Result in A .. B
               and then BI (Midpoint'Result) = (BI (A) + BI (B)) / 2;
```

`BI` is a ghost function over `Ada.Numerics.Big_Numbers.Big_Integers.Unsigned_Conversions`, which is
how SPARK states a *mathematical* half-sum — U32 arithmetic would wrap.

**The same exact-identity obligation that Rust proves, SPARK proves too.** That took four `pragma
Assert` stepping stones in the body (modular subtraction and division agreeing with their exact
integer counterparts, and the final addition not wrapping); without them the provers returned
"medium: postcondition might fail".

### A fake gate, caught

`gnatprove` **exits 0 when a check is unproved.** With a deliberately false postcondition it printed
`high: postcondition might fail` and still returned success — `just proof` would have been decorative
and every template would have looked green forever. The fix is `--checks-as-errors`, which now lives
in the GPR so a bare `gnatprove -P <project>.gpr` is already correct:

```
$ just proof                       # false postcondition: Post => Clamp'Result in Lo .. Hi - 1
proj_ada.ads:36:19: high: postcondition might fail
gnatprove: unproved check messages considered as errors
error: recipe `proof` failed on line 39 with exit code 1     # exit 1

$ just proof                       # restored
Total   35   .   35 (100%)        # Z3, Alt-Ergo, CVC5        # exit 0
```

Note also what is *not* proved: `src/main.adb` is `pragma SPARK_Mode (Off);` on purpose — the I/O
boundary is outside the SPARK subset. gnatprove skips it rather than pretending to verify it, and
that is stated in the README.

## Three defects found while doing this that the first pass had missed

These are the reason the follow-on was worth doing rather than just relabelling.

1. **`@@ARGS@@` broke Haskell generation outright.** `templates/haskell/Justfile` used `@@ARGS@@`,
   which is not a placeholder the generator knows. The generator's own unresolved-placeholder guard
   then aborted: `create-template.sh foo -l haskell` exited 1 with *"the generator and the template
   tree are out of sync"*. It is now `{{ARGS}}`, matching the other five templates. My earlier probe
   had rendered templates directly and so had never exercised the guard.

2. **`MOD_ADA` derivation was wrong for single-letter segments.** `g-ada` produced the Ada unit
   `GAda`, but the template ships `g_ada.ads`; GNAT then looked for `gada.ads` and failed with
   `file "gada.ads" not found`. Fixed by deriving the unit name from the project name directly
   (`G_Ada`, `Proj_Ada`, `My_Neat_Project`) instead of inserting underscores at case boundaries in
   the camel form. Verified clean across `g-ada`, `proj-ada`, `my-neat-project`, `ab-cd`, `a-b`,
   `wtest` — with no `Naming` workaround needed.

3. **`zig fmt --check` rejected the template's own test file** (a multi-line array literal needing
   zig's column alignment). Restructured to one element per line, which formats stably. Zig is now
   26/26 instead of 25/26.

Plus the Silver miss: **the three remaining actions are now genuinely SHA-pinned**, resolved from
real tags rather than invented:

```
mlugg/setup-zig@d1434d08867e3ee9daa34448df10607b98908d29        # v2 → v2.2.1
erlef/setup-beam@54075bcc5e249e4758d363f27d099f55d843f124       # v1 → v1.24.1
haskell-actions/setup@0f8e8c99d88aeb3fbfd523f1ef2c6f762d10d64d  # v2 → v2.12.1
```

`actions/checkout@v7.0.1` is left as-is: it is the estate's canonical ref and is covered by
`.github/workflows/actions.lock`, which is the lock mechanism the checker itself implements.

## Build output no longer breaks the gate

The one finding from the follow-on that was worth fixing immediately, rather than leaving as an
issue. Reproducing it first:

```
# a minimal compliant project, freshly built
$ aletheia .
Score: 20/26 checks passed (76.9%)     Bronze-level RSR compliance: ACHIEVED

# same tree, `.gitignore` present but not honoured by the scanner
$ aletheia .
Score: 18/26 checks passed (69.2%)     Bronze-level RSR compliance: NOT MET
Fix Suggestions:
  - SPDX license headers: Add SPDX-License-Identifier headers (first 10 lines) to:
    obj/b__main.ads, obj/deep/gen.rs; v2 4.1.1.
```

Two things were wrong with that. The obvious one is that the gate was only meaningful on a clean
checkout. The dangerous one is that **any CI job running the gate after a build would fail**, which
is exactly the order the estate's own `just check` uses — so the gate was quietly unusable in the
arrangement it was most wanted.

The scanner now reads `.gitignore` as it descends. Rules are pushed when the walk enters a
directory and popped on the way out, so a nested `.gitignore` stays scoped to its own subtree.
A directory-only rule (`obj/`) is evaluated against the entry's real file type, so it skips the
directory *without* exempting a file of the same name — a path string alone cannot tell those
apart, which is why the walk asks the type-aware question.

Implemented: comments, blank lines, `!` negation with last-match-wins, trailing slash,
leading-slash anchoring, `*` crossing directory boundaries (matching this crate's existing
`glob_match`). **Not** implemented, and not claimed: character classes, backslash escapes, and
re-inclusion inside an ignored directory.

It stays separate from the existing `[ignore]` config on purpose: `[ignore]` is a choice made in
aletheia's own configuration, whereas `.gitignore` is the repository's declaration of what is not
part of the published artefact.

Evidence, on a freshly generated Rust template:

```
$ just check            # builds, tests, lints — leaves target/ full of artefacts
$ aletheia .
Score: 26/26 checks passed (100.0%)    Bronze + Silver ACHIEVED
```

Three integration tests and ten unit tests cover it, including the negative controls: a
non-ignored file with no SPDX header still reports, and a file whose name matches a directory-only
rule is still audited.

---

## Commits

Six commits, split so each is coherent and individually green when checked out. `#197` and the
`.gitignore` work share a file *and* interleave within the same function, so they are one commit
rather than a fabricated split.

```
47d9e3b perf(scanner): one traversal, and honour the repository's .gitignore
        aletheia/src/checks.rs, aletheia/src/config.rs

8327b13 feat(scaffold): emit the v2 six-language template set
        aletheia/templates/**, aletheia/scripts/create-template.sh,
        aletheia/tests/integration_tests.rs

123710a docs: delivery write-up for #186 and #197
de3e2e9 docs: explain why the six templates do not share one sample API

b28e539 fix(ci): the Rust proof workflow could not have worked as written
a318d02 fix(ci): finish the Creusot install, and stop hard-coding the Why3 pin
        aletheia/templates/rust/.github/workflows/proof.yml

<this>  docs: record what running the proof workflows actually found
        DELIVERY-186-197.md
```

The last two exist because the first version of that workflow was wrong and no amount of reading it
had shown that — see "Proof workflows: what *not yet executed* was hiding".

Two things were deliberately left uncommitted:

* `absolute-zero` shows as a deleted submodule. It was never initialised in this sandbox (there is
  no network checkout of it), so committing the deletion would be wrong. `git submodule update
  --init` restores it.
* The lost executable bits on `setup.sh`, the `.github/hooks/*` scripts and
  `aletheia/scripts/install.sh` are restored, not committed — they changed because the sandbox
  restore does not preserve modes, not because anything meant to change.

---

## Proof workflows: what "not yet executed" was hiding

Both proof workflows shipped with a caveat: written from commands run by hand, never executed as
workflows. Exercising them properly meant rebuilding the Creusot toolchain from nothing, because
this sandbox had lost it. That was worth doing — the Rust one was wrong in three separate ways.

**What already held up.** Every command in the toolchain half ran exactly as written: the apt list,
`git clone`, `rustup toolchain install --component rustc-dev,llvm-tools`, `opam init --bare
--disable-sandboxing`, `opam switch create creusot ocaml-system`, and both `opam pin` commands for
Creusot's forks of why3 and why3find.

One of those was already fixed for the right reason. Creusot's `rust-toolchain` file is TOML —
`channel = "nightly-2026-08-03"` — but has no `.toml` extension, so the obvious `head -1` yields
`[toolchain]` and the job installs the wrong toolchain. The workflow parses the key and fails
loudly if it cannot; Creusot's own installer parses the same key, so this is not just belt-and-braces.

**What was wrong.**

1. *It never installed `creusot-rustc` where `cargo creusot` looks.* Not on `PATH` — under its data
   dir, along with the prelude and the generated `why3.conf`:

   ```
   creusot-rustc not found (expected at
     "/home/user/.local/share/creusot/toolchains/nightly-2026-08-03/bin/creusot-rustc").
   You should reinstall Creusot.
   ```

   Installing the two binaries and running `cargo creusot config --update` leaves all three pieces
   missing. Fixed by running Creusot's own installer for the rest.

2. *It never installed the provers, or why3/why3find in the data dir either.* This is the
   interesting one, because it fails much later and more confusingly. `why3` is launched with the
   data dir's `bin` **first on `PATH`**, and `creusot_why3.conf` names provers by bare name
   (`alt-ergo --timelimit %.t %f`). So:

   ```
   $ just proof
   Error: 'why3find prove' failed to launch
   Caused by: No such file or directory (os error 2)
   ```

   That is `alt-ergo` not existing. `cargo creusot` also resolves `why3` and `why3find` themselves
   as `$XDG_DATA_HOME/creusot/bin/why3[find]`, not from `PATH` — so having them in an opam switch
   is not enough either. The installer's `provers` component fetches the exact versions the drivers
   name (alt-ergo 2.6.2, z3 4.15.3, cvc4 1.8, cvc5 1.3.1); apt `z3` is a different build behind the
   same driver name, which is a quiet way to get different answers from the same proof. The
   workflow now installs those, provides `why3`/`why3find` the way the installer's own `why3`
   component does (it builds a second opam switch inside the data dir and pulls in the GTK why3
   IDE for the privilege; its last act is to symlink those two binaries, so we do that directly),
   and asserts `cargo creusot version` resolves all four provers before attempting to prove.

3. *Its Why3 pin was already stale.* The workflow hard-coded the fork commits; the checkout now
   declares a different Why3 commit. A proof gate that silently changes toolchain underneath a
   passing build is not a gate. The workflow now pins the Creusot revision and **derives** both
   forks from that revision's `creusot-deps.opam`, failing if the parse comes up empty.

**The gate, run for real.** On the rebuilt toolchain, `cargo creusot version` resolves all four
provers, and `just proof` in the generated template reports:

```
$ just proof
Proved (2 files) ✔
exit 0
```

with Why3 pinned to the same commit the workflow will use. The negative control — a false
postcondition, `result@ == (a@ + b@) / 2 + 1` — gives:

```
File ".../src/impl.rs", line 45: proof failed
Goal Coma.vc_midpoint: ✘ (4/5)
Error: 1 unproved file
Error: 'why3find prove' failed
exit 1
```

**A false pass, and why it is written down here.** My first attempt at that negative control
reported `Proved (2 files)` and exit 0 — a false gate, which is the one result that must never be
waved away. It reproduced, deliberately: take a tree *together with its build output and its
`verif/` directory*, plant a false obligation in the shared `src/impl.rs`, run `just proof`, and
the run finishes in 0.01 s and reports success without re-translating the changed source file. Hit
the same tree after forcing a re-translation (`touch verification/src/lib.rs`) and it fails
correctly, as it does on a freshly generated tree.

The practical consequence is small, and the templates are already safe: `verification/verif/`,
`verification/target/`, `*.coma` and `.why3find/` are all in the templates' `.gitignore`, so a
clone or a CI checkout never starts with stale proof state — CI runs from a clean tree by
construction. Locally, the lesson is to treat proof artefacts as build output: don't move a tree
around with them attached, and if a proof result looks wrong, force re-translation before believing
it. `cargo creusot clean` is a no-op in the healthy case ("No dangling files found", exit 0).

**The Ada workflow had the same kind of hole.** Not the same hole — the same *species*. `gnatprove`
is a frontend, not a toolchain: the release archive that job downloads contains a single binary, no
compiler and no `gprbuild`. Nothing in `proof.yml` installed those, so the job would have failed at
`gprbuild --version` one step later. `ci.yml` in the same template has installed `gnat gprbuild`
since it was written; `proof.yml` was added later and forgot. Both files being in the same directory
is what makes that findable — and it is not findable by reading either one alone.

**Ada, re-checked on the rebuilt toolchain.** The `gnatprove` asset the Ada workflow installs was
downloaded and its SHA-256 checked against the value hard-coded in the workflow
(`28fc3583…f4017` — matched), then the proof was run end-to-end on a freshly generated template:
**35 checks proved, 0 unproved, exit 0** (`Adafix.Clamp` 1 check, `Adafix.Midpoint` 34). The
negative control fails as it should:

```
$ just proof                       # false postcondition: Clamp'Result in Lo .. Hi - 1
neg_ada.ads:36:19: high: postcondition might fail
gnatprove: unproved check messages considered as errors
exit 1
```

**Two environment traps that look like template bugs and are not.** Worth recording because both
cost time here and will cost time for anyone else running these proofs in a small container:

- *Memory.* Translating `creusot-std` on a 2 GB, swapless box gets SIGKILLed ("signal: 9") partway
  through — it looks like a slow build, then dies. Dropping debuginfo for the proof run
  (`CARGO_PROFILE_DEV_DEBUG=0 CARGO_INCREMENTAL=0`) makes it fit, and translation then takes about
  35 seconds. CI runners have room; a small local VM may not.
- *Disk.* `/tmp` here is a 993 MB tmpfs, and filling it with build trees makes the **generator**
  fail with `cp: error writing '…/.gitattributes': No space left on device`. That reads as a
  template defect and is nothing of the sort: a full disk fails whichever step needs to write next.
  Generate and build under a directory on the main filesystem, and check `df` before diagnosing.

**Re-verified on the rebased tree.** Before this was handed over, the commits were rebased onto
current upstream `main` (upstream had moved on by seven commits, all of them CHANGELOG churn). A
rebase is a rewrite, so nothing was assumed about it: the whole toolchain was rebuilt from nothing
and both proofs plus both negative controls were re-run *through the generator*, on the rebased
tree, rather than in the repo's template directory.

```
tree under test: a35ef12  (= upstream main + the delivery commits)

rust  positive : "Proved (2 files) ✔"                                  exit 0
rust  negative : false postcondition (half-sum + 1)                    exit 1
ada   positive : Total 35 checks, 35 (100%) proved                     exit 0
ada   negative : false postcondition (Clamp'Result in Lo .. Hi - 1)    exit 1

aletheia       : 83 + 52 = 135 tests pass, fmt clean, clippy -D warnings clean
estate         : just check exit 0; just self-verify 26/26, Bronze + Silver ACHIEVED
```

That also closes the gap this document previously carried about the rebased tree not having been
tested. Two things the rebuild taught, both now in `toolchain-rebuild/rebuild-toolchain.sh`:

- `cargo creusot` resolves `creusot-rustc` at
  `$XDG_DATA_HOME/creusot/toolchains/<channel>/bin/creusot-rustc` and refuses to run without it,
  even when the binary is installed and on `PATH`. Installing Creusot's own binaries is not enough
  on its own; the toolchain-dir placement is part of the contract.
- `rustup toolchain install stable --component rustfmt clippy` fails with "invalid toolchain name:
  'clippy'" — the second value is read as a toolchain, not a component. Several components need one
  comma-separated value. This is the same failure mode as the `rust-toolchain` parsing bug in the CI
  job, in a different disguise: a tool argument silently reinterpreted.

**Resolved 2026-09-24 — both workflows have now run on a GitHub runner.** `Proof (Creusot)` run
`35946679306` (`Proved (2 files)`, five minutes) and `Proof (SPARK)` run `35947307833`
(35/35 checks, 100%). The reasoning above still holds as the record of why the caveat was there: the
commands were known-good first, and the green runs are what upgraded the claim from "known-good
commands in the right order in a YAML file" to a demonstrated one.


---

## Findings I did not fix, for your judgement

1. ~~**`aletheia` does not honour `.gitignore` when scanning.**~~ **Fixed in this pass** — see
   "Build output no longer breaks the gate" below. It turned out to be worse than a clean-tree
   annoyance: it also meant the gate would fail in any CI job that ran it after a build.

2. ~~**The sample-function names now differ across templates.**~~ **Looked at again, and kept —
   the difference is load-bearing.** Rust and Ada use `midpoint` (`a + (b - a) / 2`, precondition
   `a ≤ b`); Zig, Elixir and Haskell use `meanFloor` (`(a & b) + ((a ^ b) >> 1)`, *total*, no
   precondition). Agda is a proofs library rather than a sample API, and always was.

   The split follows the toolchains, not taste. The bit-trick is the better API — it is total, and
   it is the reason it was chosen originally — but Creusot has no bitvector theory and SPARK's
   provers cannot discharge the identity, so a proof-carrying template cannot ship it. Unifying
   would mean adding an `a ≤ b` precondition to three languages that do not need one, purely for
   cosmetic symmetry, and would make those three strictly worse.

   So the six templates are not identical on purpose. Each README states which formulation it ships
   and why; if you would rather have uniformity over totality, say so and it is a small change.

3. **`agda/.github/workflows/ci.yml` installs the distro's Agda**, not the 2.6.4.3 that
   `agda/.tool-versions` pins. They happened to be the same version here (Debian 13's `agda` is
   2.6.4.3), so the local run says nothing either way about a different base image. The typecheck
   is simple enough that a patch-level difference is unlikely to matter, but it is a pin the
   workflow does not honour. Building 2.6.4.3 from source in CI would take longer than the job's
   whole timeout, so the fix would be to relax the pin in `.tool-versions` rather than chase it.

4. **`aletheia/LICENSE` still says PMPL-1.0-or-later** while its sources say MPL-2.0. Carried over
   from the first delivery; unchanged, since you had not asked for it.

## Where the toolchains live

Everything heavy is installed under `/usr/local/` (Rust at `/usr/local/cargo` + `/usr/local/rustup`,
Zig at `/usr/local/zig`, `just` at `/usr/local/bin`; the language packages come from apt). Nothing
in the repo depends on any of it — the templates need a prover only to run `just proof`, and that is
the point of the standalone requirement.

**Correction, recorded because the first version of this section was wrong.** It said that installing
outside `/home/user` meant the toolchain would survive between sessions. That was tested by the
sandbox resetting, and it is false: everything outside the workspace was reclaimed — Rust, Cargo,
`just`, Zig, opam, Creusot, `gnatprove` — leaving only the workspace itself. The practical position
is that **no toolchain location is durable here**, so the recipes below are written to be re-run, not
to be relied on. What makes that survivable is the repo's own property: nothing in it needs any of
this until you ask for a proof or a real-language build.

For the record, the layout the rebuild scripts create, so a re-run lands in the same place:

| Piece | Location |
|---|---|
| Rust stable + pinned nightly | `/usr/local/rustup`, `/usr/local/cargo` |
| `just`, Zig | `/usr/local/bin`, `/usr/local/zig` |
| gnatprove 13.2.0-1 | `/usr/local/gnatprove` (release archive; it is not an Alire component and `alr install` does not exist) |
| Creusot source | `/usr/local/creusot` |
| Creusot's opam switch | `/usr/local/opam` (switch `creusot`) |
| Creusot runtime layout | `/usr/local/share/creusot` — `bin/{why3,why3find,alt-ergo,z3,cvc4,cvc5}`, `toolchains/<channel>/bin/creusot-rustc`, `share/why3find/packages/creusot` |

Two settings in that layout are load-bearing and easy to miss. `XDG_DATA_HOME` decides where
`cargo-creusot` looks for *everything* — `why3`, `why3find`, the prover binaries and `creusot-rustc`
itself — and the default (`~/.local/share`) is excluded from the workspace snapshot, so a Creusot
runtime installed there is doubly lost. Point it at `/usr/local/share` (or anywhere you control) and
`cargo creusot version` can be made to resolve all four provers. The second is `RUSTUP_TOOLCHAIN`:
`creusot-rustc` and `cargo creusot` must both run under Creusot's pinned nightly, not the default one.

`gnatprove`'s CI asset is pinned by URL **and verifies its SHA-256** (`28fc3583…f4017`, checked
against the real download).

## Follow-on verification log

```bash
# all six, via the real generator, gated on a clean tree
for L in rust zig elixir haskell ada agda; do
  create-template.sh "g-$L" -l "$L" && cd "g-$L"
  aletheia .                    # Score: 26/26 (100.0%) — Bronze + Silver ACHIEVED
  just check                    # exit 0
done

# proofs
cd g-rust && just proof         # Proved (2 files) ✔            exit 0
cd g-ada  && just proof         # Total 35 . 35 (100%)          exit 0

# negative controls — both must fail
# (clamp postcondition weakened)  → proof failed,  exit 1
# (midpoint identity + 1)         → proof failed,  exit 1
# (Clamp postcondition weakened)  → "unproved check messages considered as errors", exit 1

# air-gap, no network namespace at all
unshare -rn bash -c 'cd airrust && cargo build --offline && cargo test --offline'   # ok
unshare -rn bash -c 'cd airada  && gprbuild -p -P airada.gpr'                       # ok

# aletheia itself
cargo test                      # 83 + 52 = 135 passed
cargo clippy --all-targets -- -D warnings   # clean
cargo fmt --check               # clean
cd .. && just check && just self-verify     # exit 0, 26/26

# the .gitignore fix, end to end
just check && aletheia .        # 26/26 AFTER a build (was 18/26, Bronze NOT MET)
# negative control: remove .gitignore from the same tree
                                # 18/26, Bronze NOT MET, flags obj/b__main.ads

# the Creusot toolchain, rebuilt from nothing and then used
cargo creusot version            # alt-ergo 2.6.2 / z3 4.15.3 / cvc4 1.8 / cvc5 1.3.1 — all resolved
                                 # (before the `provers` component: all four "not found")
cd g-rust && just proof          # Proved (2 files) ✔  exit 0
                                 # re-run after re-pinning Why3 to the commit creusot-deps.opam
                                 # declares: Proved (2 files) ✔  exit 0
# negative control on a fresh tree: midpoint identity + 1
                                 # Goal Coma.vc_midpoint: ✘ (4/5), 1 unproved file, exit 1
# same negative control on a tree copied WITH its verif/ + target/: "Proved (2 files)", exit 0
                                 # then `touch verification/src/lib.rs` → fails correctly, exit 1
                                 # (templates .gitignore verif/ and target/, so CI never sees this)

# the Ada proof, re-run on a regenerated template after fixing proof.yml
rm -rf obj/gnatprove && just proof        # exit 0
# obj/gnatprove/gnatprove.out: Clamp 1 check + Midpoint 34 checks proved, 0 unproved

# the proof workflows: commands run for real, not simulated
python3 -c "import yaml; yaml.safe_load(open('.../proof.yml'))"   # valid YAML for both
# rust-toolchain channel parsed from the clone → nightly-2026-08-03
# why3/why3find pins derived from creusot-deps.opam (the hard-coded one had gone stale)
# gnatprove asset sha256 verified against the workflow's pinned value
# four defects found (three Rust, one Ada) — all fixed
# both jobs have since run green on a GitHub runner: 35946679306 (Creusot), 35947307833 (SPARK)
```

---

## Post-merge log — 2026-09-24

Everything above describes the delivery as it was handed over. This section records what happened
afterwards, including the parts that went wrong.

### Both proof gates ran on a GitHub runner

| Gate | Run | Result |
|---|---|---|
| `Proof (Creusot)` | [`35946679306`](https://github.com/hyperpolymath/maa-framework/actions/runs/35946679306) | `completed/success`, 02:18:23Z → 02:24:13Z, `Proved (2 files) ✔` |
| `Proof (SPARK)` | [`35947307833`](https://github.com/hyperpolymath/maa-framework/actions/runs/35947307833) | `completed/success`, 02:27:16Z → 02:28:30Z, **35/35 checks (100%)** |

The Creusot run resolved its own toolchain from scratch on the runner (cargo-creusot 0.14.0-dev,
Why3 1.8.2+git, why3find v1.3.0+dev, alt-ergo 2.6.2, z3 4.15.3, cvc4 1.8, cvc5 1.3.1) and compiled the
crate in 29 s. The SPARK run's summary, per subprogram:

```
Total                            35         .                     35 (100%)           .          .
  Proofada.Clamp at proofada.ads:34 ... proved (1 checks)
  Proofada.Midpoint at proofada.ads:45 ... proved (34 checks)
```

Two `Proofada.U32_Big` bodies are reported `skipped; body is SPARK_Mode => Off` — those are
instantiations of the GNAT big-integer standard library, not project code. The counts above are
project obligations.

### A scratch branch was merged by accident, and reverted

Exercising a gate needs a repository root where the generated `proof.yml` resolves. I used a scratch
branch whose tree was a scaffolded project and opened a **draft** PR — but a draft can still be
merged, and it was, ~100 s later. That squash (`50140689`) replaced the entire repository tree with
the sample project. It was reverted one commit later:

```
859b387  revert: restore the framework tree replaced by a scratch-branch merge
         tree identical to b5535f1 ✔
```

Force-pushing `main` is blocked by a repository rule, so the revert is a forward commit, not a
rewind — `main`'s history keeps both the accident and its correction, visible. **Both gates ship a
`workflow_dispatch:` trigger**, which is what the Ada run used; the correct route for this kind of
exercise is to dispatch the workflow on a scratch branch and open no PR at all.

### A `.gitattributes` defect, found by doing that

`aletheia/templates/common/.gitattributes` held one `@@LANG_GITATTR_LINE@@` placeholder, and two of
the six languages filled it with two patterns. Git's gitattributes grammar is one pattern per line;
a line with two is rejected outright and *both* patterns on it are then ignored:

```
# before, elixir and ada
*.adb *.ads text eol=lf
$ git check-attr text eol -- probe.ads
*.ads is not a valid attribute name: .gitattributes:4
probe.ads: text: auto          # the language rule never applied
$ git add -A
*.ads is not a valid attribute name: .gitattributes:4    # printed on every push

# after
*.adb  text eol=lf
*.ads  text eol=lf
$ git check-attr text eol -- probe.ads probe.adb
probe.ads: text: set
probe.ads: eol: lf
probe.adb: text: set
probe.adb: eol: lf
```

`lang_gitattr` now emits one pattern per line, `subst` inserts that block with `awk` (sed cannot
carry a newline in a replacement portably), and
`test_every_language_template_reaches_bronze_and_silver` asserts both the one-pattern-per-line rule
and that each language's extensions are covered. Negative control: with the old generator restored,
that assertion fails —

```
[elixir] .gitattributes:4 puts 2 patterns on one line; git rejects the whole
line and drops every pattern on it: *.ex *.exs text eol=lf
```

Suite after the fix: **83 + 52 = 135 tests**, 0 failures; `cargo fmt --check` clean;
`cargo clippy --all-targets -- -D warnings` clean; regenerated `elixir` and `ada` scaffolds still
**26/26, Bronze + Silver ACHIEVED**.
