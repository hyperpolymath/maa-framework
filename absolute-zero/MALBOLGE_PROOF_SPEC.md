# Universal Proof Specification: Absolute Zero (Malbolge)
**Target**: `malbolge_test.mb` (A Malbolge Program)
**Goal**: Prove whether this program is a "Certified Null Operation" (CNO) — meaning it halts without performing any side-effecting state mutations, or if it produces output/infinite loops.

## Prover Mapping (All 49 Tiers vs Malbolge)

### Tier 1: Interactive & SMT (Foundational Truth)
1.  **Coq**: Map the `malbolge_test.mb` ASCII string to `MalbolgeProgram` and evaluate `is_malbolge_CNO`.
2.  **Lean4**: Translate the Coq semantics to Lean4 and prove termination under base-3 arithmetic.
3.  **Agda**: Verify the trinary memory bounds (59,049 addresses).
4.  **Z3**: SMT check on the unrolled Malbolge encryption step. (EXPECTED: Path explosion).
5.  **CVC5**: Check equivalence between the Malbolge program and a NOP slice.
6.  **Isabelle/HOL**: Inductive proof over the `malbolge_step` relations.

### Tier 2-4: The "Big Six" & Legacy
7.  **Metamath**: Prove the properties of base-3 "Crazy Operation".
8.  **HOL Light**: Verify the memory array bounds.
9.  **Mizar**: Set theory properties of the trinary state space.
10. **PVS**: Prove the `encrypt` transition matrix is a bijection.
11. **ACL2**: Bounded evaluation up to 100,000 steps.
12. **HOL4**: Prove the `MOut` op is not triggered.

### Tier 5: First-Order ATPs (The Speed Racers)
13. **Vampire**: (EXPECTED: Timeout). Non-linear trinary operations defy standard heuristics.
14. **EProver**: Search for an equational proof of `state == state'`.
15. **SPASS**: Try to find a contradiction where `MOut` happens.
16. **AltErgo**: SMT/FOL check on the program counter bounds.

### Tier 6-7: Specialized & Advanced
17. **F***: Prove memory effect safety (no out-of-bounds trinary access).
18. **Dafny**: Use invariants to bound the `c` (code) and `d` (data) pointers.
19. **Why3**: Orchestrate proof obligations for the crazy op.
20. **TLAPS**: Model the Malbolge VM as a state machine.
21. **Twelf**: Type the AST of Malbolge instructions (if one even exists).
22. **Nuprl**: Constructive witness of termination.
23. **Minlog**: Minimal logic proof of instruction decoding.
24. **Imandra**: (EXPECTED: Timeout). Cryptographic-level obfuscation blocks symbolic execution.

### Tier 8: Constraint & Optimization
25. **GLPK**: Not applicable (highly non-linear).
26. **SCIP**: Not applicable.
27. **MiniZinc**: Attempt to constraint-solve the path to `MHlt`.
28. **Chuffed**: Model the `encrypt` table as a CP constraint.
29. **ORTools**: Optimize for the shortest path to a side effect.

### Tier 9: Model Checkers & Security
30. **SPIN**: Exhaustive state space exploration (59049 * 3^10 states). (EXPECTED: Out of Memory).
31. **CBMC**: Unroll the C-based interpreter 10,000 times.
32. **SeaHorn**: Abstract interpretation of the Malbolge data pointer.
33. **CaDiCaL**: SAT solving the crazy op truth table.
34. **Kissat**: Bit-blasted (or trit-blasted) SAT check.
35. **MiniSat**: (EXPECTED: Timeout).
36. **NuSMV**: Symbolic model check of the VM transition relation.
37. **TLC**: TLA+ bounded model checking.
38. **Alloy**: Find a model where Malbolge makes sense (Good luck).
39. **Prism**: Probabilistic check of termination.
40. **UPPAAL**: Model the VM cycle times.
41. **Frama-C**: Deductive verification of a C-interpreter running this program.
42. **Viper**: Permission checks on the 59049-trit array.
43. **Tamarin**: (EXPECTED: Irrelevant). Not a crypto protocol, just looks like one.
44. **ProVerif**: (EXPECTED: Irrelevant).
45. **KeY**: JavaDL verification of a Java Malbolge interpreter.
46. **DReal**: Delta-complete checking (Irrelevant for discrete state).
47. **ABC**: AIGER synthesis of the Malbolge CPU.

### Tier 10 & Oracles
48. **Idris2**: Dependent types for the 59049 bounded memory accesses.
49. **TypedWasm**: Verify the WASM version of the Malbolge runtime.
