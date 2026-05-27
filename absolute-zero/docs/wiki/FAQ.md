<!-- SPDX-License-Identifier: MPL-2.0 -->
# FAQ

### Why prove that a program does nothing?

Because the universe of "no-op" candidates is huge and most of them
aren't no-ops. A `for i in 1..1e9` loop that increments and decrements
a counter looks like a no-op until you notice it heats the CPU. Formal
verification draws the line: a program is a CNO iff a machine-checkable
proof says so, in a model that captures the side-effects you care about
(state, I/O, *and* entropy).

### Why six proof systems?

Each catches what the others miss. A property proved in Coq + Lean +
Agda + Isabelle + Mizar + Z3 doesn't depend on any single backend's
foundational quirks. When something is provable only in some, that
tells you something about the foundational dependency. See
[Proof Systems](Proof-Systems.md).

### Why is Idris2 here separately from `proofs/`?

`src/abi/` is the FFI surface, not a proof system in its own right —
it carries formal alignment + size proofs for the C ABI used by the
Zig shim. See [ABI](ABI.md).

### What's the difference between an Axiom and an Admitted?

* **Axiom** (Coq) / `axiom` (Lean) / `believe_me ()` (Idris2 0.8.0): an
  asserted proposition. Permanent unless discharged.
* **Admitted** (Coq) / `sorry` (Lean) / `?hole` (Idris2): a placeholder
  in a proof, marking incomplete work. Should not ship.

absolute-zero core: **0 Admitted**, 73 Axioms + 42 Parameters, all
owner-classified.

### Why is `examples/python` / `examples/go` / `examples/java` missing?

Deleted 2026-05-25 per the strict CLAUDE.md language policy. See
[Contributing](Contributing.md). The CNO concept generalises across
languages; the demonstrations now cover the 23 still-allowed ones.

### Where do I file issues?

`hyperpolymath/absolute-zero/issues`. For audit / trust-escape findings,
use the existing [`AUDIT.adoc`](../../AUDIT.adoc) issue template; for
proof discharges, the regular issue template + an ADR plan.

### How do I talk to ECHIDNA from a script?

Through the `echidna-llm-mcp` BoJ cartridge — never directly. See the
ECHIDNA tool table in [`0-AI-MANIFEST.a2ml`](../../0-AI-MANIFEST.a2ml).

### Is the Coq proof complete?

Core theory: yes (11/11 files compile, 0 Admitted). Model-layer
assumptions (Parameters about abstract physics, quantum, POSIX) are
owner-classified — these are *intended* axiomatisations of the
external world, not gaps in the verification of the computational claim.

### Where's the live state?

`.machine_readable/6a2/STATE.a2ml`. Updated on every meaningful change.

### Why "Absolute Zero"?

The thermodynamic analogy: at 0 K, a system has no entropy to release.
A CNO is a program at "computational absolute zero" — it has no
side-effect entropy to release into the world.
