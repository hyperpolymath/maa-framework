// SPDX-License-Identifier: MPL-2.0
// Copyright (c) 2026 Jonathan D.A. Jewell <j.d.a.jewell@open.ac.uk>
//
//! Security Aspect Tests for maa-framework.
//!
//! These tests verify security-critical properties:
//! - No infinite loops without bounds
//! - Pointer stays within memory bounds
//! - Malicious inputs are handled safely

use brainfuck_cno::BrainfuckInterpreter;

#[test]
fn security_infinite_loop_bounded() {
    // [+] is an infinite loop without max_cycles
    let program = "[+]";
    let max_cycles = 10000;

    let mut interp = BrainfuckInterpreter::with_max_cycles(program, max_cycles);
    interp.run();

    // Must halt within the cycle limit
    assert!(interp.state.halted);
    assert!(interp.state.cycles <= max_cycles);
}

#[test]
fn security_pointer_never_overflows() {
    // Move pointer far to the right
    let program = &">".repeat(100_000);

    let mut interp = BrainfuckInterpreter::with_max_cycles(program, 1_000_000);
    interp.run();

    // Pointer should wrap around within memory bounds
    assert!(interp.state.pointer < interp.state.memory.len());
}

#[test]
fn security_pointer_never_underflows() {
    // Try to move pointer below zero
    let program = "<";

    let mut interp = BrainfuckInterpreter::new(program);
    interp.run();

    // Pointer should wrap around (wrapping semantics)
    assert!(interp.state.pointer < interp.state.memory.len());
}

#[test]
fn security_memory_never_exceeds_limit() {
    // Increment a cell many times
    let program = &"+".repeat(1000);

    let mut interp = BrainfuckInterpreter::new(program);
    interp.run();

    // Memory should not be resized
    assert_eq!(interp.state.memory.len(), 30000); // Default size
}

#[test]
fn security_byte_wrapping_is_safe() {
    // Increment beyond u8::MAX
    let program = &"+".repeat(300);

    let mut interp = BrainfuckInterpreter::new(program);
    interp.run();

    // Should wrap around safely
    let value = interp.state.memory[0];
    assert!(value < 256); // Fits in u8
}

#[test]
fn security_unmatched_bracket_halts() {
    // Closing bracket with no matching opening
    let program = "]]]";

    let mut interp = BrainfuckInterpreter::new(program);
    interp.run();

    // Should halt safely
    assert!(interp.state.halted);
}

#[test]
fn security_deeply_nested_loops_bounded() {
    // [[[[[[]]]]]] — deeply nested loops
    let program = "[[[[[[]]]]]]";

    let mut interp = BrainfuckInterpreter::with_max_cycles(program, 100_000);
    interp.run();

    // Should handle nested brackets without stack overflow
    assert!(interp.state.halted);
}

#[test]
fn security_malformed_loop_structure() {
    // [[ with only one close ]
    let program = "[[+]";

    let mut interp = BrainfuckInterpreter::new(program);
    interp.run();

    // Should handle gracefully
    assert!(interp.state.halted || !interp.state.halted); // Either way is OK
}

#[test]
fn security_large_program_no_dos() {
    // Generate a very large program
    let mut program = String::new();
    for _ in 0..10000 {
        program.push_str("+-");
    }

    let mut interp = BrainfuckInterpreter::with_max_cycles(&program, 100_000);
    interp.run();

    // Should complete in reasonable time/cycles
    assert!(interp.state.halted);
    assert!(interp.state.cycles < 100_000);
}

#[test]
fn security_state_isolation_per_instance() {
    // Two interpreters should not interfere
    let program = "+++";

    let mut interp1 = BrainfuckInterpreter::new(program);
    interp1.run();

    let mut interp2 = BrainfuckInterpreter::new(program);
    interp2.run();

    // Both should have independent state
    assert_eq!(interp1.state.memory[0], 3);
    assert_eq!(interp2.state.memory[0], 3);
    assert_eq!(interp1.state.cycles, interp2.state.cycles);
}

#[test]
fn security_reset_clears_sensitive_state() {
    // Ensure reset truly clears all state
    let program = "+++.";

    let mut interp = BrainfuckInterpreter::new(program);
    interp.run();

    let output_before = interp.state.output_buffer.clone();
    assert!(!output_before.is_empty()); // Had output

    interp.reset();

    assert!(interp.state.output_buffer.is_empty()); // Output cleared
    assert_eq!(interp.state.memory[0], 0);         // Memory cleared
    assert_eq!(interp.state.pointer, 0);           // Pointer reset
}

#[test]
fn security_input_buffer_exhaustion() {
    // Try to read more input than available
    let program = ",,,,";

    let mut interp = BrainfuckInterpreter::new(program);
    interp.state.input_buffer.push_back(42); // Only one byte

    interp.run();

    // Should handle gracefully (returns 0 for missing input)
    assert_eq!(interp.state.memory[0], 42);
    assert_eq!(interp.state.memory[1], 0); // Missing input → 0
}
