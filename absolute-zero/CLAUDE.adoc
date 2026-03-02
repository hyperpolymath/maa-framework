# CLAUDE.md

## Project Overview

**Absolute Zero** is a formal verification research project exploring the mathematical proof that programs can be certified to compute nothing. This work formalizes **Certified Null Operations (CNOs)** — programs that, despite potentially complex implementations, can be mathematically proven to have zero net computational effect.

### What This Repository Contains

**IMPORTANT**: This GitHub repository is a **partial mirror** containing only the TypeScript web interface. The complete project lives on GitLab at:

🔗 **Full Repository**: https://gitlab.com/maa-framework/6-the-foundation/absolute-zero

This mirror contains:
- TypeScript Malbolge interpreter (`ts/interpreter/malbolge-interpreter.ts`)
- Web-based execution environment (`index.html`)
- Audit trail and narrative scaffolding utilities

The full GitLab repository contains:
- **Coq formal proofs** of CNO properties
- **Z3 SMT solver** verification scripts
- **ReScript interpreters** for Malbolge
- **Comprehensive documentation** on theoretical foundations
- **Multiple esoteric language examples** (Brainfuck, Whitespace, etc.)
- **Research papers and theoretical analysis**

### The Central Research Question

**Can we formally prove that a program does absolutely nothing?**

This seemingly trivial question leads to deep insights in:
- **Formal Verification**: Machine-checked proofs of program properties
- **Computational Complexity**: Understanding minimal computation
- **Reversible Computing**: Programs preserving thermodynamic reversibility
- **Esoteric Languages**: Using Malbolge as proof-of-concept

## What is a Certified Null Operation (CNO)?

A CNO is a program with the following formally proven properties:

### Definition (Informal)

```
∀ σ : ProgramState, ∀ p : Program,
  IsCNO(p) ↔ (
    Terminates(p, σ) ∧
    FinalState(p, σ) = σ ∧
    NoSideEffects(p) ∧
    ThermodynamicallyReversible(p)
  )
```

### Properties

- ✅ **Termination**: Always halts (no infinite loops)
- ✅ **State Preservation**: Input state = Output state
- ✅ **Purity**: No I/O, no memory allocation, no observable effects
- ✅ **Reversibility**: Can be undone with zero energy cost (Landauer's principle)

## Full Project Structure (GitLab)

```
absolute-zero/
│
├── README.md                 # Comprehensive research documentation
├── LICENSE-AGPL3.md          # AGPL 3.0 license
├── LICENSE-PALIMPSEST.md     # Palimpsest 0.5 academic license
├── justfile                  # Build automation (used on GitLab)
│
├── interpreters/
│   └── rescript/
│       ├── malbolgeInterpreter.res    # Malbolge interpreter
│       └── malboldgeInterpreter.res   # Alternative implementation
│
├── proofs/
│   ├── coq/
│   │   ├── _CoqProject         # Coq project configuration
│   │   ├── common/
│   │   │   └── CNO.v           # Core CNO framework & theorems
│   │   └── malbolge/
│   │       └── MalbolgeCore.v  # Malbolge-specific proofs
│   │
│   ├── z3/
│   │   ├── cno_properties.smt2 # SMT-LIB specifications
│   │   └── verify.sh           # Automated Z3 verification
│   │
│   └── lean4/                  # Future: Lean 4 proofs
│       └── CNO.lean
│
├── docs/
│   ├── theory.md               # Theoretical foundations
│   ├── examples.md             # CNO examples across languages
│   ├── proofs-guide.md         # Guide to understanding proofs
│   └── philosophy.md           # Philosophical implications
│
├── examples/
│   ├── malbolge/
│   │   ├── nop.mal             # Simplest CNO in Malbolge
│   │   └── complex_nop.mal     # Obfuscated CNO
│   ├── brainfuck/
│   │   └── nop.bf              # CNO in Brainfuck
│   └── whitespace/
│       └── nop.ws              # CNO in Whitespace
│
└── tests/
    ├── unit/                   # Unit tests for interpreters
    └── proofs/                 # Proof test cases
```

## This GitHub Mirror Structure

```
absolute-zero/              # ⚠️ Partial mirror - web interface only
├── ts/
│   ├── interpreter/
│   │   └── malbolge-interpreter.ts    # TypeScript Malbolge interpreter
│   ├── audit-trail.ts                 # Execution tracking
│   ├── narrative-scaffold.ts          # Program analysis framework
│   └── wasm-loader.ts                 # WebAssembly utilities
├── index.html                         # Web interface
├── package.json                       # TypeScript build config
└── tsconfig.json                      # TypeScript configuration
```

## Theoretical Foundations

### Mutually Assured Accountability Framework (MAAF)

Absolute Zero is part of the larger **Mutually Assured Accountability Framework**, specifically under "6 the foundation" (6-the-foundation). This framework explores formal methods for computational accountability and verification.

### Key Theoretical Concepts

#### 1. Landauer's Principle and Reversibility

**Landauer's Principle**: Erasing information dissipates at least kT ln 2 of energy.

**Implication**: A truly reversible CNO dissipates zero energy.

```
E_dissipated = kT Σ p_i ln(p_i / q_i)

For a CNO: p_i = q_i for all i
Therefore: E_dissipated = 0
```

#### 2. Computational Complexity of Nothingness

**Question**: Is it harder to prove a program does nothing than to prove it does something?

**Intuition**: Proving absence requires ruling out all possible effects, while proving presence requires demonstrating one effect.

#### 3. Curry-Howard Isomorphism

CNOs correspond to tautologies in logic:
```
Program : Type
CNO : A → A (identity)
Proof : Construction of identity function
```

Every CNO is a proof of `A → A` for some type `A`.

## Build System

### On GitLab (Full Project)

The full project uses **`just`** (modern command runner):

```bash
# Build everything
just build-all

# Build ReScript interpreters
just build-rescript

# Compile Coq proofs
just build-coq

# Run Z3 verification
just verify-z3

# Run tests
just test

# Generate documentation
just docs
```

### On GitHub (This Mirror)

This partial mirror uses npm:

```bash
# Install dependencies
npm install

# Build TypeScript
npm run build

# Watch mode
npm run watch

# Open index.html in browser to run interpreter
```

## Development Guidelines

### Working with the Full Project (GitLab)

#### Coq Proofs (`proofs/coq/`)

- Core CNO definitions in `common/CNO.v`
- Language-specific proofs in subdirectories
- Follow [coq-community](https://github.com/coq-community/manifesto) style guide
- All theorems must be proven, no `Admitted`

Example theorem:
```coq
Theorem cno_implies_termination :
  forall p, is_CNO p -> forall σ, terminates p σ.
```

#### Z3 Verification (`proofs/z3/`)

- SMT-LIB 2.0 format
- Complementary to Coq proofs
- Use `z3 cno_properties.smt2` to verify

#### ReScript Interpreters (`interpreters/rescript/`)

- Use `rescript format` before committing
- Maintain purity (no side effects)
- Document unusual Malbolge semantics

### Working with This Mirror (GitHub)

#### TypeScript (`ts/`)

- Project uses TypeScript 5.2.2+
- Maintain separation: interpreter core vs. auxiliary features
- Keep interpreter logic in `ts/interpreter/`

#### Testing Malbolge Programs

When working with Malbolge code:
- Malbolge is deliberately obscure and difficult
- Programs self-modify during execution
- Character set is limited to specific ASCII values
- Validate programs using interpreter before committing

#### Common Tasks (This Mirror)

1. **Adding Interpreter Features**
   - Modify `ts/interpreter/malbolge-interpreter.ts`
   - Run `npm run build`
   - Test with known Malbolge programs
   - Update documentation

2. **Debugging**
   - Use `npm run watch` for continuous compilation
   - Check browser console when running `index.html`
   - Audit trail helps trace execution flow

3. **WebAssembly Integration**
   - WASM functionality in `ts/wasm-loader.ts`
   - Ensure compatibility with Malbolge's execution model
   - Test performance-critical sections

## Understanding Malbolge

Malbolge is one of the most difficult programming languages ever created:

- **Ternary logic**: Base-3 operations
- **Self-modifying code**: Instructions change after execution
- **Limited character set**: Only specific ASCII values valid
- **Encryption**: Code is "encrypted" during execution
- **Nearly impossible to write**: Programs typically generated

### Why Malbolge for CNO Research?

Malbolge's extreme complexity makes it an ideal testbed:
- If we can prove a Malbolge program does nothing, we can prove anything
- Self-modification challenges formal verification
- Demonstrates proof techniques work even for obfuscated code

## Licensing

This project is **dual-licensed** to support both open-source use and academic research:

### AGPL 3.0 (Primary License)

For general use, licensed under GNU Affero General Public License v3.0:
- ✅ Freedom to use, modify, and distribute
- ✅ Copyleft: derivatives must be open-source
- ✅ Network use triggers source disclosure

See `LICENSE-AGPL3.md` for full terms.

### Palimpsest 0.5 (Academic License)

For academic research, alternatively use Palimpsest 0.5:

**Permits**:
- ✅ Use in academic papers, theses, dissertations
- ✅ Citation without source code disclosure
- ✅ Building upon this work for pure research

**Requirements**:
- 📝 Cite this work in publications
- 📝 Acknowledge use of Absolute Zero
- 📝 Non-commercial use only

**Restrictions**:
- ❌ Commercial use requires AGPL compliance
- ❌ Patent trolling explicitly forbidden
- ❌ Cannot sublicense under different terms

See `LICENSE-PALIMPSEST.md` for full terms.

### Choosing a License

- **Open-source projects**: Use AGPL 3.0
- **Academic papers**: Use Palimpsest 0.5
- **Commercial use**: Contact author for arrangement
- **When in doubt**: Use AGPL 3.0

## Research Context

### Open Research Questions

1. **Classification**: Can we classify all CNOs up to equivalence?
2. **Complexity**: What is the computational complexity of CNO verification?
3. **Obfuscation**: What's the most complex program provable as a CNO?
4. **Language-Independence**: Do CNO properties hold across language semantics?
5. **Quantum CNOs**: What does "null operation" mean for quantum programs?

### Potential Applications

- **Secure Sandboxing**: Run untrusted code proven to be inert
- **Energy-Efficient Computing**: Design hardware optimized for CNOs
- **Formal Methods Education**: CNOs as pedagogical examples
- **Compiler Optimization**: Detect and eliminate CNOs in compiled code

## Context for AI Assistants

### What This Project Is

- **Primary Goal**: Formal verification of computational nothingness
- **Methods**: Coq proofs, Z3 verification, interpreter implementation
- **Contribution**: Pushing boundaries of what can be formally verified
- **Philosophy**: Understanding the computational structure of "nothing"

### What NOT to Do

- ❌ Don't modify Malbolge code without deep understanding
- ❌ Don't assume standard programming paradigms apply
- ❌ Don't remove seemingly "useless" code (may be critical)
- ❌ Don't optimize without profiling (behavior is non-intuitive)
- ❌ Don't modify Coq proofs without understanding proof assistant
- ❌ Don't commit code with `Admitted` (unproven theorems)

### When Working with This Repository

**Remember**: This is a **partial mirror** containing only the web interface.

For questions about:
- **Formal proofs**: Refer to GitLab repository
- **Theoretical foundations**: See GitLab `docs/theory.md`
- **CNO definitions**: Check GitLab `proofs/coq/common/CNO.v`
- **This TypeScript implementation**: Check local `ts/` directory
- **Build system**: Use `npm run build` (this mirror) or `just build-all` (GitLab)

## Citation

If you use Absolute Zero in research, please cite:

```bibtex
@misc{jewell2025absolute,
  title={Absolute Zero: Formal Verification of Certified Null Operations},
  author={Jewell, Jonathan D. A.},
  year={2025},
  publisher={GitLab},
  howpublished={\url{https://gitlab.com/maa-framework/6-the-foundation/absolute-zero}},
  note={Coq and Z3 verification of computationally inert programs}
}
```

## Resources

### Primary

- **Full GitLab Repository**: https://gitlab.com/maa-framework/6-the-foundation/absolute-zero
- **MAAF Framework**: https://gitlab.com/maa-framework/
- **Malbolge Specification**: Original esoteric language specification

### Related Work

- **CompCert**: Formally verified C compiler (Isabelle/HOL)
- **seL4**: Formally verified microkernel
- **Landauer, R. (1961)**: "Irreversibility and Heat Generation in Computing"
- **Bennett, C. (1973)**: "Logical Reversibility of Computation"

### Tools

- **Coq**: https://coq.inria.fr/
- **Z3**: https://github.com/Z3Prover/z3
- **ReScript**: https://rescript-lang.org/
- **just**: https://github.com/casey/just

## Philosophical Note

> "The universe tends toward maximum entropy. A Certified Null Operation is a pocket of perfect computational order — a program that resists the Second Law. It does nothing, but in doing nothing, it says everything about the structure of computation itself."

This is not just about silly programs. This is about understanding the computational structure of nothingness.

## Getting Started

### Using This Mirror (GitHub)

```bash
# Clone this repository
git clone https://github.com/Hyperpolymath/absolute-zero.git
cd absolute-zero

# Install dependencies
npm install

# Build TypeScript
npm run build

# Open index.html in browser to run web interface
```

### Using Full Project (GitLab)

```bash
# Clone full repository
git clone https://gitlab.com/maa-framework/6-the-foundation/absolute-zero.git
cd absolute-zero

# Install dependencies (Fedora)
sudo dnf install coq z3 nodejs opam
npm install -g rescript@11.1

# Build everything
just build-all

# Verify proofs
just verify-coq
just verify-z3

# Run examples
just run-example malbolge/nop.mal
```

## Contact

**Jonathan D. A. Jewell**
- GitLab: [@hyperpolymath](https://gitlab.com/hyperpolymath)
- GitHub: [@Hyperpolymath](https://github.com/Hyperpolymath)
- Email: jonathan@metadatastician.art

---

**Status**: Proofs verified ✓ | Theorems established ✓ | More work to do ✓

*This file helps AI assistants like Claude Code understand the project structure and research context.*
