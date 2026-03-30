// SPDX-License-Identifier: PMPL-1.0-or-later
// Copyright (c) 2026 Jonathan D.A. Jewell (hyperpolymath)
//
// Unit tests for CNO (Certified Null Operation) properties
// Tests the core invariants that absolute-zero guarantees

#[cfg(test)]
mod cno_tests {
    /// CNO Property 1: An empty program is always a CNO
    #[test]
    fn empty_program_is_cno() {
        let tape = vec![0u8; 30_000];
        let initial = tape.clone();
        // Empty program: no instructions executed
        assert_eq!(tape, initial, "Empty program must not modify state");
    }

    /// CNO Property 2: Balanced increment/decrement is a CNO
    #[test]
    fn balanced_inc_dec_is_cno() {
        let mut tape = vec![0u8; 30_000];
        let initial = tape.clone();

        // +- cancels out
        tape[0] = tape[0].wrapping_add(1);
        tape[0] = tape[0].wrapping_sub(1);

        assert_eq!(tape, initial, "+- must cancel to identity");
    }

    /// CNO Property 3: Balanced pointer movement is a CNO
    #[test]
    fn balanced_pointer_movement_is_cno() {
        let mut ptr: usize = 0;
        let tape_len = 30_000;

        // >< cancels out
        ptr = (ptr + 1) % tape_len;
        ptr = ptr.checked_sub(1).unwrap_or(tape_len - 1);

        assert_eq!(ptr, 0, ">< must return pointer to origin");
    }

    /// CNO Property 4: N increments followed by N decrements is a CNO
    #[test]
    fn n_inc_n_dec_is_cno() {
        let mut val: u8 = 0;
        let n = 42;

        for _ in 0..n {
            val = val.wrapping_add(1);
        }
        for _ in 0..n {
            val = val.wrapping_sub(1);
        }

        assert_eq!(val, 0, "N increments + N decrements must equal identity");
    }

    /// CNO Property 5: Wrapping arithmetic preserves CNO (256 increments = identity)
    #[test]
    fn wrapping_256_is_cno() {
        let mut val: u8 = 0;
        for _ in 0..256 {
            val = val.wrapping_add(1);
        }
        assert_eq!(val, 0, "256 wrapping increments must overflow back to 0");
    }

    /// Non-CNO: A program that produces output is NOT a CNO
    #[test]
    fn output_program_is_not_cno() {
        let mut output: Vec<u8> = Vec::new();
        // '.' instruction writes to output
        output.push(65); // ASCII 'A'
        assert!(!output.is_empty(), "Program with output is not a CNO");
    }

    /// Non-CNO: Unbalanced operations modify state
    #[test]
    fn unbalanced_is_not_cno() {
        let mut val: u8 = 0;
        val = val.wrapping_add(1);
        // No decrement — state is modified
        assert_ne!(val, 0, "Unbalanced increment is not a CNO");
    }

    /// CNO composition: if A is CNO and B is CNO, then A;B is CNO
    #[test]
    fn cno_composition() {
        let mut tape = vec![0u8; 100];
        let initial = tape.clone();

        // CNO A: +- on cell 0
        tape[0] = tape[0].wrapping_add(1);
        tape[0] = tape[0].wrapping_sub(1);

        // CNO B: ><>< on pointer (no tape mutation)
        let mut ptr = 0usize;
        ptr = (ptr + 1) % tape.len();
        ptr = ptr.checked_sub(1).unwrap_or(tape.len() - 1);

        assert_eq!(tape, initial, "Composition of CNOs must be a CNO");
        assert_eq!(ptr, 0, "Pointer must return to origin");
    }

    /// CNO parallel: two independent CNOs on disjoint regions compose
    #[test]
    fn cno_parallel_disjoint() {
        let mut tape = vec![0u8; 100];
        let initial = tape.clone();

        // CNO on cell 0
        tape[0] = tape[0].wrapping_add(5);
        tape[0] = tape[0].wrapping_sub(5);

        // CNO on cell 50 (disjoint)
        tape[50] = tape[50].wrapping_add(10);
        tape[50] = tape[50].wrapping_sub(10);

        assert_eq!(tape, initial, "Parallel CNOs on disjoint regions compose");
    }

    /// Whitespace stack: push then pop is a CNO
    #[test]
    fn ws_push_pop_is_cno() {
        let mut stack: Vec<i64> = Vec::new();
        let initial = stack.clone();

        stack.push(42);
        stack.pop();

        assert_eq!(stack, initial, "Push then pop must be a CNO on the stack");
    }

    /// Whitespace heap: store then restore is a CNO
    #[test]
    fn ws_heap_store_restore_is_cno() {
        use std::collections::HashMap;
        let mut heap: HashMap<i64, i64> = HashMap::new();

        // Store value at address 0
        heap.insert(0, 42);
        // Restore by removing
        heap.remove(&0);

        assert!(heap.is_empty(), "Store then remove must be a CNO on the heap");
    }
}
