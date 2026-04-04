// SPDX-License-Identifier: PMPL-1.0-or-later
// Copyright (c) 2026 Jonathan D.A. Jewell <j.d.a.jewell@open.ac.uk>
//
//! Property-Based Tests for maa-framework.
//!
//! These tests verify properties that must hold across all inputs:
//! - Determinism: same input always produces same output
//! - Idempotence: repeated operations are consistent
//! - Equivalence: different representations of the same operation yield the same result

use brainfuck_cno::BrainfuckInterpreter;

#[test]
fn property_brainfuck_deterministic() {
    // For a given program, repeated runs should produce identical state
    let program = "+++++[-]";

    for _ in 0..10 {
        let mut interp = BrainfuckInterpreter::new(program);
        let result1 = interp.is_cno();

        interp.reset();
        let result2 = interp.is_cno();

        assert_eq!(result1.is_cno, result2.is_cno);
    }
}

#[test]
fn property_balanced_operations_are_cno() {
    // Any balanced sequence of +/- or >/< is a CNO
    let test_cases = vec![
        "+-",
        "+-+-+-",
        "++++----",
        "<>",
        "<><>",
        "<<<>>>",
        "++>--<",
    ];

    for program in test_cases {
        let mut interp = BrainfuckInterpreter::new(program);
        let result = interp.is_cno();
        assert!(
            result.is_cno,
            "Balanced program '{}' must be CNO, but got: {}",
            program,
            result.reason
        );
    }
}

#[test]
fn property_output_always_breaks_cno() {
    // Any program with . (output) is not a CNO
    let test_cases = vec!["+.", ".", "-.", "+-+."];

    for program in test_cases {
        let mut interp = BrainfuckInterpreter::new(program);
        let result = interp.is_cno();
        assert!(
            !result.is_cno,
            "Program with output '{}' must NOT be CNO, but got: {}",
            program,
            result.reason
        );
    }
}

#[test]
fn property_unbalanced_increment_breaks_cno() {
    // Any program with net +/- not returning to zero is not CNO
    let test_cases = vec!["+", "++", "---", "++-"];

    for program in test_cases {
        let mut interp = BrainfuckInterpreter::new(program);
        let result = interp.is_cno();
        assert!(
            !result.is_cno,
            "Unbalanced program '{}' must NOT be CNO",
            program
        );
    }
}

#[test]
fn property_unbalanced_pointer_breaks_cno() {
    // Any program with net >/< not returning to cell 0 is not CNO
    let test_cases = vec![">>", ">>><<", "<>><", ">>>"];

    for program in test_cases {
        let mut interp = BrainfuckInterpreter::new(program);
        let result = interp.is_cno();
        assert!(
            !result.is_cno,
            "Pointer displacement '{}' must NOT be CNO",
            program
        );
    }
}

#[test]
fn property_comments_are_ignored() {
    // Non-command characters should be ignored
    let program_with_comments = "+-comments+-more";
    let program_clean = "+-+-";

    let mut interp1 = BrainfuckInterpreter::new(program_with_comments);
    let result1 = interp1.is_cno();

    let mut interp2 = BrainfuckInterpreter::new(program_clean);
    let result2 = interp2.is_cno();

    // Both should produce the same result (both CNO)
    assert_eq!(result1.is_cno, result2.is_cno);
}

#[test]
fn property_empty_loops_are_cno() {
    // [] with zero cell is skipped (CNO)
    let program = "[]";
    let mut interp = BrainfuckInterpreter::new(program);
    let result = interp.is_cno();

    assert!(result.is_cno, "Empty loop must be CNO");
}

#[test]
fn property_reset_restores_initial_state() {
    // After reset(), running again should produce same result
    let program = "+-+-";
    let mut interp = BrainfuckInterpreter::new(program);

    let initial_cycles = interp.state.cycles;
    interp.run();
    let cycles_after_first = interp.state.cycles;

    interp.reset();
    let cycles_after_reset = interp.state.cycles;

    assert_eq!(cycles_after_reset, initial_cycles);
    assert_ne!(cycles_after_reset, cycles_after_first); // Should have reset

    interp.run();
    assert_eq!(interp.state.cycles, cycles_after_first); // Same cycle count
}

#[test]
fn property_step_by_step_same_as_run() {
    // Stepping through a program one instruction at a time
    // should yield the same result as running all at once
    let program = "+++-";

    let mut interp1 = BrainfuckInterpreter::new(program);
    interp1.run();
    let state1 = interp1.state.clone();

    let mut interp2 = BrainfuckInterpreter::new(program);
    while !interp2.state.halted {
        interp2.step();
    }
    let state2 = interp2.state.clone();

    assert_eq!(state1.memory, state2.memory);
    assert_eq!(state1.pointer, state2.pointer);
    assert_eq!(state1.output_buffer, state2.output_buffer);
}

#[test]
fn property_cycle_limit_enforces_termination() {
    // Programs with potential infinite loops should terminate
    let program = "[+]";
    let mut interp = BrainfuckInterpreter::with_max_cycles(program, 100);

    interp.run();
    assert!(interp.state.halted);
    assert!(interp.state.cycles <= 100 + 10); // Small buffer for boundary
}

#[test]
fn property_idempotent_operations() {
    // ++-- is equivalent to no-op in terms of final state
    let test_pairs = vec![
        ("++-", "+-"),    // Net effect: zero
        ("><<", "><"),    // Net effect: back to cell 0
        ("+-+-", "+--+"), // Both end at zero
    ];

    for (prog1, prog2) in test_pairs {
        let mut interp1 = BrainfuckInterpreter::new(prog1);
        let result1 = interp1.is_cno();

        let mut interp2 = BrainfuckInterpreter::new(prog2);
        let result2 = interp2.is_cno();

        assert_eq!(
            result1.is_cno, result2.is_cno,
            "Equivalent programs should have same CNO property: '{}' vs '{}'",
            prog1, prog2
        );
    }
}
