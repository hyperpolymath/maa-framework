OPENQASM 2.0;
include "qelib1.inc";

// Identity Gate: The Quantum Change-Nothing Operation
// ====================================================
//
// The identity gate (I) is the fundamental quantum CNO. It leaves the quantum
// state completely unchanged: I|ψ⟩ = |ψ⟩ for any quantum state |ψ⟩.
//
// MATHEMATICAL REPRESENTATION:
// The identity gate is represented by the 2×2 identity matrix:
//     I = [1  0]
//         [0  1]
//
// QUANTUM CNO PROPERTIES:
// 1. Preserves quantum superposition
// 2. Preserves quantum entanglement
// 3. Preserves quantum phase
// 4. Is its own inverse: I² = I
// 5. Commutes with all gates: IG = GI for any gate G
//
// WHY IT MATTERS:
// - Identity gates are used for timing synchronization in quantum circuits
// - They introduce delays without changing quantum information
// - Useful for studying decoherence effects over time
// - Foundation for understanding what "change" means in quantum mechanics
//
// DECOHERENCE AND NOISE DESPITE CNO:
// Even though the identity gate theoretically changes nothing, in practice:
// - Environmental interactions cause decoherence (T1, T2 relaxation)
// - Gate errors can occur even for "do nothing" operations
// - Thermal noise affects qubit state
// - Cosmic rays and other radiation can flip qubits
// - Physical implementation time allows noise to accumulate
//
// The gap between theoretical CNO and physical reality reveals the fragility
// of quantum information and the challenge of quantum error correction.
//
// REVERSIBILITY:
// Identity is trivially reversible (self-inverse). All quantum gates must be
// unitary (reversible) to preserve quantum information. This reversibility
// connects to Landauer's principle: reversible computation can be performed
// without energy dissipation (thermodynamic CNO).
//
// NO-CLONING THEOREM IMPLICATIONS:
// The identity gate doesn't clone quantum states. Even the simplest quantum
// operation respects the no-cloning theorem: you cannot create an independent
// copy of an unknown quantum state. Identity preserves the single copy.

qreg q[3];
creg c[3];

// Example 1: Single qubit identity
// State before: |0⟩ or |1⟩ or α|0⟩ + β|1⟩
id q[0];
// State after: unchanged

// Example 2: Identity on entangled pair
// Create Bell state: (|00⟩ + |11⟩)/√2
h q[1];
cx q[1], q[2];
// Apply identity to both qubits - entanglement preserved
id q[1];
id q[2];
// Bell state remains: (|00⟩ + |11⟩)/√2

// Example 3: Identity as timing spacer
// In real quantum circuits, identity gates create delays
reset q[0];
h q[0];
id q[0];  // Wait one gate time
id q[0];  // Wait another gate time
id q[0];  // Wait another gate time
// During these "do nothing" operations, decoherence accumulates

// Example 4: Identity chain equivalence
// Multiple identities = single identity
id q[0];
id q[0];
id q[0];
// Mathematically equivalent to: id q[0];
// But physically: more time = more decoherence!

// Example 5: Identity commutes with all gates
h q[0];
id q[0];  // Could be before or after H gate
// Same as: id q[0]; h q[0];

// Measure to verify (measurement is NOT a CNO - it collapses superposition)
measure q[0] -> c[0];
measure q[1] -> c[1];
measure q[2] -> c[2];

// PHILOSOPHICAL NOTE:
// In quantum mechanics, "changing nothing" is profound:
// - It requires maintaining quantum coherence
// - It's information-theoretically free but physically costly
// - It connects to the arrow of time and thermodynamics
// - It demonstrates that existence in time ≠ change
//
// The identity gate is the quantum embodiment of "Absolute Zero" -
// an operation that executes while producing no transformation.
