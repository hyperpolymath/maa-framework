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

#![forbid(unsafe_code)]
use std::collections::VecDeque;

const MEMORY_SIZE: usize = 30000;
const DEFAULT_MAX_CYCLES: usize = 1_000_000;

/// The VM state for the Brainfuck execution engine.
#[derive(Clone)]
pub struct BrainfuckState {
    pub memory: Vec<u8>,          // The data tape
    pub pointer: usize,           // The data pointer
    pub program_counter: usize,   // The instruction pointer
    pub input_buffer: VecDeque<u8>,
    pub output_buffer: Vec<u8>,
    pub halted: bool,
    pub cycles: usize,            // Instruction execution count
    pub max_cycles: usize,        // Safety limit to prevent infinite loops
}

/// The Interpreter orchestrator.
pub struct BrainfuckInterpreter {
    program: Vec<char>,
    pub state: BrainfuckState,
}

impl BrainfuckInterpreter {
    /// STEP: Executes a single Brainfuck instruction and updates the VM state.
    ///
    /// Commands:
    /// - `>` : Increment data pointer.
    /// - `<` : Decrement data pointer.
    /// - `+` : Increment byte at data pointer.
    /// - `-` : Decrement byte at data pointer.
    /// - `.` : Output byte at data pointer.
    /// - `,` : Input byte to data pointer.
    /// - `[` : Jump forward past `]` if byte at pointer is 0.
    /// - `]` : Jump backward to `[` if byte at pointer is non-zero.
    pub fn step(&mut self) {
        if self.state.halted || self.state.cycles >= self.state.max_cycles {
            self.state.halted = true;
            return;
        }

        let instruction = self.program[self.state.program_counter];
        self.state.cycles += 1;

        match instruction {
            '>' => { self.state.pointer = (self.state.pointer + 1) % self.state.memory.len(); }
            '<' => { self.state.pointer = self.state.pointer.checked_sub(1).unwrap_or(self.state.memory.len() - 1); }
            '+' => { self.state.memory[self.state.pointer] = self.state.memory[self.state.pointer].wrapping_add(1); }
            '-' => { self.state.memory[self.state.pointer] = self.state.memory[self.state.pointer].wrapping_sub(1); }
            // ... [Instruction mapping continues]
            _   => { /* Ignore non-command characters */ }
        }
        self.state.program_counter += 1;
    }

    /// VERIFICATION: Determines if the loaded program is a Certified Null Operation.
    ///
    /// This is a "dry run" that snapshots the initial state, executes the program,
    /// and then verifies the three CNO criteria: Termination, Purity, and Reversibility.
    pub fn is_cno(&mut self) -> CnoResult {
        let initial_memory = self.state.memory.clone();
        let initial_pointer = self.state.pointer;

        self.run();

        // 1. Check for infinite loops
        if self.state.cycles >= self.state.max_cycles {
            return CnoResult { is_cno: false, reason: "Infinite loop detected".into() };
        }

        // 2. Check for observable output
        if !self.state.output_buffer.is_empty() {
            return CnoResult { is_cno: false, reason: "Program produced side-effects (output)".into() };
        }

        // 3. Check for state mutation
        if self.state.memory != initial_memory || self.state.pointer != initial_pointer {
            return CnoResult { is_cno: false, reason: "Program modified global state".into() };
        }

        CnoResult { is_cno: true, reason: "Program is a verified CNO ✓".into() }
    }
}
