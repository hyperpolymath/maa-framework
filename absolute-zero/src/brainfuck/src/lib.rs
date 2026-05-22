// SPDX-License-Identifier: MPL-2.0
// Copyright (c) 2026 Jonathan D.A. Jewell <j.d.a.jewell@open.ac.uk>
//
//! Brainfuck Interpreter with Certified Null Operation (CNO) Detection.
//!
//! Brainfuck is an esoteric programming language designed by Urban Müller in 1993.
//! It operates on an array of memory cells (a "tape"), each initialized to zero.
//!
//! ESOTERIC VERIFICATION: Because the language has a minimal instruction set (8 commands),
//! it is an ideal target for formal verification of the "Absolute Zero" property.
//!
//! CNO CRITERIA:
//! 1. TERMINATION: The program must halt within `max_cycles`.
//! 2. PURITY: The program must produce no output buffer content.
//! 3. REVERSIBILITY: The memory tape and pointer must be returned to their initial state.
//!
//! SPARK INTEGRATION NOTE:
//! The CNO verification logic (`is_cno`) maps directly to SPARK pre/post contracts:
//!   Pre  => snapshot initial state
//!   Post => final_state == initial_state AND output_buffer.is_empty AND terminated
//! Future work: Idris2 ABI proof of CNO properties, Zig FFI bridge.

#![forbid(unsafe_code)]
use std::collections::VecDeque;

const MEMORY_SIZE: usize = 30_000;
const DEFAULT_MAX_CYCLES: usize = 1_000_000;

/// Result of CNO verification.
#[derive(Debug, Clone)]
pub struct CnoResult {
    /// Whether the program is a Certified Null Operation.
    pub is_cno: bool,
    /// Human-readable explanation of the verdict.
    pub reason: String,
}

/// The VM state for the Brainfuck execution engine.
#[derive(Clone, Debug, PartialEq)]
pub struct BrainfuckState {
    pub memory: Vec<u8>,
    pub pointer: usize,
    pub program_counter: usize,
    pub input_buffer: VecDeque<u8>,
    pub output_buffer: Vec<u8>,
    pub halted: bool,
    pub cycles: usize,
    pub max_cycles: usize,
}

/// The Interpreter orchestrator.
pub struct BrainfuckInterpreter {
    program: Vec<char>,
    /// Pre-computed bracket matching: bracket_map[i] = j means '[' at i matches ']' at j (and vice versa).
    bracket_map: Vec<usize>,
    pub state: BrainfuckState,
}

impl BrainfuckInterpreter {
    /// Create a new interpreter for the given program string.
    pub fn new(source: &str) -> Self {
        let program: Vec<char> = source.chars().collect();
        let bracket_map = Self::build_bracket_map(&program);

        BrainfuckInterpreter {
            program,
            bracket_map,
            state: BrainfuckState {
                memory: vec![0u8; MEMORY_SIZE],
                pointer: 0,
                program_counter: 0,
                input_buffer: VecDeque::new(),
                output_buffer: Vec::new(),
                halted: false,
                cycles: 0,
                max_cycles: DEFAULT_MAX_CYCLES,
            },
        }
    }

    /// Create with a custom cycle limit.
    pub fn with_max_cycles(source: &str, max_cycles: usize) -> Self {
        let mut interp = Self::new(source);
        interp.state.max_cycles = max_cycles;
        interp
    }

    /// Pre-compute bracket matching for O(1) jump resolution.
    fn build_bracket_map(program: &[char]) -> Vec<usize> {
        let mut map = vec![0usize; program.len()];
        let mut stack: Vec<usize> = Vec::new();

        for (i, &ch) in program.iter().enumerate() {
            if ch == '[' {
                stack.push(i);
            } else if ch == ']' {
                if let Some(open) = stack.pop() {
                    map[open] = i;
                    map[i] = open;
                }
                // Unmatched ']' — treated as halt (invalid program)
            }
        }
        map
    }

    /// STEP: Executes a single Brainfuck instruction and updates the VM state.
    ///
    /// Commands:
    /// - `>` : Increment data pointer (wrapping).
    /// - `<` : Decrement data pointer (wrapping).
    /// - `+` : Increment byte at data pointer (wrapping).
    /// - `-` : Decrement byte at data pointer (wrapping).
    /// - `.` : Output byte at data pointer.
    /// - `,` : Input byte to data pointer (0 if buffer empty).
    /// - `[` : Jump past matching `]` if byte at pointer is 0.
    /// - `]` : Jump back to matching `[` if byte at pointer is non-zero.
    pub fn step(&mut self) {
        if self.state.halted || self.state.program_counter >= self.program.len() {
            self.state.halted = true;
            return;
        }

        if self.state.cycles >= self.state.max_cycles {
            self.state.halted = true;
            return;
        }

        let instruction = self.program[self.state.program_counter];
        self.state.cycles += 1;

        match instruction {
            '>' => {
                self.state.pointer = (self.state.pointer + 1) % self.state.memory.len();
            }
            '<' => {
                self.state.pointer = self
                    .state
                    .pointer
                    .checked_sub(1)
                    .unwrap_or(self.state.memory.len() - 1);
            }
            '+' => {
                self.state.memory[self.state.pointer] =
                    self.state.memory[self.state.pointer].wrapping_add(1);
            }
            '-' => {
                self.state.memory[self.state.pointer] =
                    self.state.memory[self.state.pointer].wrapping_sub(1);
            }
            '.' => {
                self.state
                    .output_buffer
                    .push(self.state.memory[self.state.pointer]);
            }
            ',' => {
                let byte = self.state.input_buffer.pop_front().unwrap_or(0);
                self.state.memory[self.state.pointer] = byte;
            }
            '[' => {
                if self.state.memory[self.state.pointer] == 0 {
                    // Jump to matching ']'
                    self.state.program_counter = self.bracket_map[self.state.program_counter];
                }
            }
            ']' => {
                if self.state.memory[self.state.pointer] != 0 {
                    // Jump back to matching '['
                    self.state.program_counter = self.bracket_map[self.state.program_counter];
                }
            }
            _ => { /* Non-command characters are comments — ignore */ }
        }

        self.state.program_counter += 1;
    }

    /// RUN: Execute the program to completion (or cycle limit).
    pub fn run(&mut self) {
        while !self.state.halted && self.state.program_counter < self.program.len() {
            self.step();
        }
        self.state.halted = true;
    }

    /// VERIFICATION: Determines if the loaded program is a Certified Null Operation.
    ///
    /// This is a "dry run" that snapshots the initial state, executes the program,
    /// and then verifies the three CNO criteria: Termination, Purity, and Reversibility.
    ///
    /// SPARK contract equivalent:
    ///   Pre:  state == initial_state
    ///   Post: result.is_cno => (state == initial_state AND output.is_empty AND terminated)
    pub fn is_cno(&mut self) -> CnoResult {
        let initial_memory = self.state.memory.clone();
        let initial_pointer = self.state.pointer;

        self.run();

        // 1. TERMINATION: Check for infinite loops
        if self.state.cycles >= self.state.max_cycles {
            return CnoResult {
                is_cno: false,
                reason: "Non-termination: exceeded cycle limit".into(),
            };
        }

        // 2. PURITY: Check for observable output (side effects)
        if !self.state.output_buffer.is_empty() {
            return CnoResult {
                is_cno: false,
                reason: "Impurity: program produced observable output".into(),
            };
        }

        // 3. REVERSIBILITY: Check that state returned to initial
        if self.state.memory != initial_memory {
            return CnoResult {
                is_cno: false,
                reason: "Irreversible: memory tape was modified".into(),
            };
        }

        if self.state.pointer != initial_pointer {
            return CnoResult {
                is_cno: false,
                reason: "Irreversible: data pointer was displaced".into(),
            };
        }

        CnoResult {
            is_cno: true,
            reason: "Certified Null Operation: terminates, pure, reversible".into(),
        }
    }

    /// Reset the interpreter to run the same program again.
    pub fn reset(&mut self) {
        self.state.memory = vec![0u8; MEMORY_SIZE];
        self.state.pointer = 0;
        self.state.program_counter = 0;
        self.state.input_buffer.clear();
        self.state.output_buffer.clear();
        self.state.halted = false;
        self.state.cycles = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_program_is_cno() {
        let mut interp = BrainfuckInterpreter::new("");
        let result = interp.is_cno();
        assert!(result.is_cno, "Empty program must be a CNO");
    }

    #[test]
    fn comments_only_is_cno() {
        let mut interp = BrainfuckInterpreter::new("this is just a comment");
        let result = interp.is_cno();
        assert!(result.is_cno, "Comments-only program must be a CNO");
    }

    #[test]
    fn balanced_move_is_cno() {
        let mut interp = BrainfuckInterpreter::new("><");
        let result = interp.is_cno();
        assert!(result.is_cno, ">< must be a CNO");
    }

    #[test]
    fn balanced_inc_dec_is_cno() {
        let mut interp = BrainfuckInterpreter::new("+-");
        let result = interp.is_cno();
        assert!(result.is_cno, "+- must be a CNO");
    }

    #[test]
    fn multiple_balanced_moves_is_cno() {
        let mut interp = BrainfuckInterpreter::new(">><<");
        let result = interp.is_cno();
        assert!(result.is_cno, ">><<  must be a CNO");
    }

    #[test]
    fn multiple_balanced_inc_dec_is_cno() {
        let mut interp = BrainfuckInterpreter::new("+-+-+-");
        let result = interp.is_cno();
        assert!(result.is_cno, "+-+-+- must be a CNO");
    }

    #[test]
    fn unbalanced_increment_not_cno() {
        let mut interp = BrainfuckInterpreter::new("+");
        let result = interp.is_cno();
        assert!(!result.is_cno, "Single + is NOT a CNO");
    }

    #[test]
    fn output_not_cno() {
        let mut interp = BrainfuckInterpreter::new(".");
        let result = interp.is_cno();
        assert!(!result.is_cno, "Output (.) is NOT a CNO");
    }

    #[test]
    fn cross_cell_mutation_not_cno() {
        let mut interp = BrainfuckInterpreter::new(">+<");
        let result = interp.is_cno();
        assert!(!result.is_cno, ">+< modifies cell 1, NOT a CNO");
    }

    #[test]
    fn empty_loop_is_cno() {
        // Cell 0 starts at 0, so [] never enters — it's a CNO
        let mut interp = BrainfuckInterpreter::new("[]");
        let result = interp.is_cno();
        assert!(result.is_cno, "[] on zero cell is a CNO");
    }

    #[test]
    fn loop_clear_not_cno() {
        // +[-] sets cell to 1 then clears — but final state has cell=0, same as initial
        // However the intermediate state changed, and the program terminates, so...
        // Actually: cell starts 0, +1=1, [-] loops: 1->0, exits. Final=0. IS a CNO!
        let mut interp = BrainfuckInterpreter::new("+[-]");
        let result = interp.is_cno();
        assert!(result.is_cno, "+[-] returns cell to 0, IS a CNO");
    }

    #[test]
    fn cross_cell_not_cno() {
        let mut interp = BrainfuckInterpreter::new("+>-<");
        let result = interp.is_cno();
        assert!(!result.is_cno, "+>-< modifies two cells, NOT a CNO");
    }

    #[test]
    fn reset_allows_rerun() {
        let mut interp = BrainfuckInterpreter::new("+-");
        let r1 = interp.is_cno();
        assert!(r1.is_cno);
        interp.reset();
        let r2 = interp.is_cno();
        assert!(r2.is_cno, "Reset must allow clean re-verification");
    }
}
