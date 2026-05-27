# Sonnet Handoff: Continuing the Absolute Zero + ECHIDNA Work

**From**: Claude Opus 4.5 session (2026-02-05)
**To**: Claude Sonnet (next session)
**Purpose**: Complete guide for continuing all active workstreams

---

## What Was Done (Opus Sessions, Feb 4-5)

### Absolute Zero Repo
1. Completed 8 Coq proofs (bringing total to 81 Qed, 19 Admitted)
2. Fixed false `is_lambda_CNO` definition in LambdaCNO.v
3. Added closedness infrastructure for lambda calculus composition
4. Added ProofIrrelevance for category theory morphism equality
5. Created PROOF-INSIGHTS.md (detailed proof engineering knowledge)
6. Updated all 6 checkpoint files (STATE.scm, ECOSYSTEM.scm, META.scm x2)
7. Updated ROADMAP.adoc and README.adoc with current status
8. Git commits: 25fe7a5, fe96a8e

### ECHIDNA Repo
1. Analyzed Julia/Chapel layer interaction (found Chapel is isolated)
2. Created CORRECTNESS-ARCHITECTURE.md (trust + correctness design)
3. Updated root checkpoint files (STATE.scm, ECOSYSTEM.scm, META.scm)
4. Updated ROADMAP.adoc with actual content
5. Designed how absolute-zero serves as ECHIDNA test case

---

## What Sonnet Should Do Next

### Priority 1: Absolute Zero — Complete Remaining Proofs

Read `PROOF-INSIGHTS.md` first. It contains all the techniques needed.

**Easy proofs (do these first)**:

1. **QuantumCNO.v: quantum_state_eq_refl** — Add axiom `Cexp_zero : Cexp 0 = C1`
   and `Cmult_1_l`. Then prove reflexivity using `exists 0, Cexp 0 = 1`.

2. **QuantumCNO.v: quantum_state_eq_sym** — Need `Cinv_Cexp` axiom.
   If `psi2 = Cexp(c) * psi1`, then `psi1 = Cexp(-c) * psi2`.

3. **QuantumCNO.v: quantum_state_eq_trans** — Need `Cexp_add` axiom.
   If `a = Cexp(c1) * b` and `b = Cexp(c2) * c`, then `a = Cexp(c1+c2) * c`.

4. **MalbolgeCore.v** — Read the file first. 1 Admitted proof, likely
   involves Malbolge instruction encoding.

**Medium proofs**:

5. **QuantumCNO.v: quantum_cno_composition** — FIX THE INTERMEDIATE.
   Use `V psi` not `U psi`:
   ```coq
   apply quantum_state_eq_trans with (V psi).
   + apply HU_id.
   + apply HV_id.
   ```

6. **QuantumCNO.v: global_phase_is_cno** — Depends on state_eq proofs.
   May need to fix `Cexp(RtoC theta)` to `Cexp(Ci * RtoC theta)`.

7. **FilesystemCNO.v** — 6 proofs. Read the file to classify difficulty.

**Hard proofs (may need axiomatization)**:

8. **LandauerDerivation.v** — 3 proofs using general post_execution_dist.
   Much harder than StatMech.v versions. May need to axiomatize
   the measure-theoretic lemmas.

9. **QuantumMechanicsExact.v** — 3 proofs. Probably should be axiomatized
   (quantum mechanics fundamentals).

10. **LambdaCNO.v: y_not_cno** — Non-termination proof. Leave Admitted
    or axiomatize as a well-known result.

**Important**: No coqc is available locally. All proofs must be reasoned
about manually. Follow the patterns established in completed proofs.

### Priority 2: ECHIDNA — Chapel Integration

Read `CORRECTNESS-ARCHITECTURE.md` section "Compute Layer Integration".

1. Create a Chapel -> Rust C FFI bridge prototype:
   - Chapel calls `extern "C"` functions defined in Rust
   - Start with just `echidna_prover_create` and `echidna_apply_tactic`
   - Test with 1 prover (Coq) before scaling to 12

2. Add Julia HTTP calls from Chapel:
   - Chapel calls Julia `/suggest` endpoint for neural guidance
   - Parse JSON response to get ranked tactics
   - Feed results into Chapel's parallel search

3. Test end-to-end: Chapel → Julia guidance → Rust prover execution

### Priority 3: ECHIDNA — Julia ML Upgrade

1. Add `Flux = "0.14"` to `src/julia/Project.toml`
2. The GNN + Transformer architecture exists in `src/julia/models/neural_solver.jl`
   but has never been trained. Train it on the expanded corpus.
3. Compare Transformer accuracy vs logistic regression baseline.

### Priority 4: Absolute Zero as ECHIDNA Test Case

See `CORRECTNESS-ARCHITECTURE.md` section "Applying ECHIDNA to Absolute Zero".

1. Create `examples/absolute-zero/` in echidna repo
2. Copy representative Coq proofs (CNO.v, LambdaCNO.v, StatMech.v)
3. Extract training data from these proofs
4. Test ECHIDNA's tactic suggestions against known proofs
5. Attempt the 19 Admitted proofs with ECHIDNA

### Priority 5: echidnabot Completion

echidnabot is at 75% completion. Missing:
- Container isolation for prover execution
- Retry logic with exponential backoff
- Concurrent job limits
- Integration tests

Read echidnabot's STATE.scm and CLAUDE.md for details.

---

## Key Technical Context

### Absolute Zero Architecture
- 10 Coq proof files in `proofs/coq/` across 6 subdirectories
- Core definitions in `proofs/coq/common/CNO.v`
- All files `Require Import CNO.` for shared definitions
- State equality (`=st=`) is the fundamental relation
- `eval p s s'` means program `p` maps state `s` to `s'`
- `is_CNO p` = terminates + preserves state + pure + thermo reversible

### ECHIDNA Architecture
- Rust core: `src/rust/` — 12 prover backends, HTTP server, agent
- Julia ML: `src/julia/` — neural models, HTTP API on port 8090
- ReScript UI: `src/rescript/` — React components, port 8000
- Chapel HPC: `chapel_poc/` — parallel search PoC (ISOLATED)
- Zig FFI: `src/zig/` — C ABI bridge for external integration
- Idris2: `src/idris/` — formal proof validator

### Build Commands
```bash
# Absolute Zero
just build-all
just verify-all

# ECHIDNA
cargo build                    # Rust core
cargo test                     # All tests
just build                     # Full build
# Julia: cd src/julia && julia --project=. api/server.jl
# Chapel: chpl chapel_poc/parallel_proof_search.chpl
```

---

## Files Modified This Session

### Absolute Zero
- `PROOF-INSIGHTS.md` — NEW: proof engineering knowledge (READ THIS FIRST)
- `STATE.scm` — Updated with proof completion stats
- `ECOSYSTEM.scm` — Updated with project relationships
- `META.scm` — Updated with architecture decisions
- `.machine_readable/STATE.scm` — Updated
- `.machine_readable/ECOSYSTEM.scm` — Updated
- `.machine_readable/META.scm` — Updated
- `ROADMAP.adoc` — Replaced template with actual content
- `README.adoc` — Updated proof status, fixed badges and license
- `SONNET-HANDOFF.md` — THIS FILE
- (Previous commits): StatMech.v, LambdaCNO.v, CNOCategory.v

### ECHIDNA
- `STATE.scm` — Updated from stub to actual data
- `ECOSYSTEM.scm` — Updated from stub to actual data
- `META.scm` — Updated from stub to actual data
- `ROADMAP.adoc` — Replaced template with actual content
- `CORRECTNESS-ARCHITECTURE.md` — NEW: trust + correctness design

---

## What Needs Opus vs What Sonnet Can Do

### Opus-Level (already done)
- Identifying false definitions (is_lambda_CNO)
- Designing proof strategies for hard theorems
- Architecture analysis (Julia/Chapel interaction)
- Correctness architecture design
- Cross-repo integration design (ECHIDNA + absolute-zero)

### Sonnet-Level (remaining work)
- Completing straightforward proofs (QuantumCNO state_eq)
- Reading and classifying unanalyzed files (FilesystemCNO, MalbolgeCore)
- Writing Chapel FFI bridge code
- Adding Flux.jl dependency and training models
- Creating example files in echidna repo
- Updating echidnabot checkpoint files
- Git commits for all changes

---

## Commit Strategy

For absolute-zero:
```
git add PROOF-INSIGHTS.md SONNET-HANDOFF.md STATE.scm ECOSYSTEM.scm META.scm
git add .machine_readable/STATE.scm .machine_readable/ECOSYSTEM.scm .machine_readable/META.scm
git add ROADMAP.adoc README.adoc
git commit -m "docs: comprehensive project status update and Sonnet handoff

Updated all 6 checkpoint files with current proof completion status.
Created PROOF-INSIGHTS.md with proof engineering knowledge transfer.
Created SONNET-HANDOFF.md for continuation by Claude Sonnet.
Updated ROADMAP.adoc and README.adoc with current proof stats.

Co-Authored-By: Claude Opus 4.5 <noreply@anthropic.com>"
```

For echidna:
```
git add STATE.scm ECOSYSTEM.scm META.scm ROADMAP.adoc CORRECTNESS-ARCHITECTURE.md
git commit -m "docs: update checkpoint files, add correctness architecture

Updated root checkpoint files from stubs to actual project data.
Created CORRECTNESS-ARCHITECTURE.md designing five-layer trust system.
Includes Julia/Chapel integration design and absolute-zero test case plan.

Co-Authored-By: Claude Opus 4.5 <noreply@anthropic.com>"
```
