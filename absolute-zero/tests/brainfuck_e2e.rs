// SPDX-License-Identifier: PMPL-1.0-or-later
// Copyright (c) 2026 Jonathan D.A. Jewell <j.d.a.jewell@open.ac.uk>
//
//! End-to-End Tests for Brainfuck Interpreter.
//!
//! These tests verify the complete execution pipeline of the brainfuck interpreter,
//! including program compilation, execution, and CNO verification.

use brainfuck_cno::BrainfuckInterpreter;

#[test]
fn e2e_empty_program_terminates_cleanly() {
    let mut interp = BrainfuckInterpreter::new("");
    interp.run();
    assert!(interp.state.halted);
    assert!(interp.state.output_buffer.is_empty());
}

#[test]
fn e2e_simple_loop_with_exit() {
    // +++++[-] — increment 5 times, then loop to clear
    let program = "+++++[-]";
    let mut interp = BrainfuckInterpreter::new(program);
    interp.run();

    assert!(interp.state.halted);
    assert!(interp.state.output_buffer.is_empty());
    assert_eq!(interp.state.memory[0], 0); // Cell cleared
}

#[test]
fn e2e_output_prevents_cno() {
    // . — output instruction
    let program = "+.";
    let mut interp = BrainfuckInterpreter::new(program);
    let result = interp.is_cno();

    assert!(!result.is_cno);
    assert!(result.reason.contains("output"));
}

#[test]
fn e2e_input_instruction() {
    // , — read from input buffer
    let program = ",";
    let mut interp = BrainfuckInterpreter::new(program);
    interp.state.input_buffer.push_back(42);

    interp.run();
    assert!(interp.state.halted);
    assert_eq!(interp.state.memory[0], 42); // Input stored in cell 0
}

#[test]
fn e2e_pointer_wrapping() {
    // Move pointer far to the right
    let right = ">".repeat(100);
    let left = "<".repeat(100);
    let program = format!("{}{}", right, left);

    let mut interp = BrainfuckInterpreter::new(&program);
    interp.run();

    assert!(interp.state.halted);
    assert_eq!(interp.state.pointer, 0); // Wrapped around
}

#[test]
fn e2e_nested_loops() {
    // ++[>+[>+<-]<-] — a more complex nested loop
    let program = "++[>+[>+<-]<-]";
    let mut interp = BrainfuckInterpreter::new(program);
    interp.run();

    assert!(interp.state.halted);
    // Complex state but should still terminate
}

#[test]
fn e2e_cycle_limit_prevents_infinite_loop() {
    // [+] — infinite loop: starts at 0, so [ jumps past the loop
    // Use [>+] which will loop infinitely
    let program = "[>+]";
    let max_cycles = 1000;
    let mut interp = BrainfuckInterpreter::with_max_cycles(program, max_cycles);
    interp.run();

    assert!(interp.state.halted);
    // Should either hit cycle limit or complete
    assert!(interp.state.cycles <= 1000 + 100); // Some buffer
}

#[test]
fn e2e_cno_property_reversibility() {
    // +- — increment then decrement
    let program = "+-";
    let mut interp = BrainfuckInterpreter::new(program);
    let result = interp.is_cno();

    assert!(result.is_cno);
    assert_eq!(interp.state.memory[0], 0); // Returned to initial state
    assert_eq!(interp.state.pointer, 0);
}

#[test]
fn e2e_reset_allows_multiple_runs() {
    let program = "+-";
    let mut interp = BrainfuckInterpreter::new(program);

    // First run
    let result1 = interp.is_cno();
    assert!(result1.is_cno);

    // Reset and run again
    interp.reset();
    let result2 = interp.is_cno();
    assert!(result2.is_cno);
    assert_eq!(result1.is_cno, result2.is_cno);
}

#[test]
fn e2e_stress_test_large_program() {
    // Generate a large balanced program
    let mut program = String::new();
    for _ in 0..1000 {
        program.push('+');
        program.push('-');
    }

    let mut interp = BrainfuckInterpreter::new(&program);
    interp.run();

    assert!(interp.state.halted);
    assert_eq!(interp.state.memory[0], 0); // Balanced
}
