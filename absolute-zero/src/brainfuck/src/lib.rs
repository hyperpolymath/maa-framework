//! Brainfuck Interpreter with CNO Detection
//!
//! Brainfuck is an esoteric language with only 8 instructions.
//! This makes it ideal for CNO verification - we can easily prove
//! when a program does nothing.
//!
//! Author: Jonathan D. A. Jewell
//! Project: Absolute Zero
//! License: AGPL-3.0 / Palimpsest 0.5

use std::collections::VecDeque;

const MEMORY_SIZE: usize = 30000;
const DEFAULT_MAX_CYCLES: usize = 1_000_000;

/// State of the Brainfuck interpreter
#[derive(Clone)]
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

impl Default for BrainfuckState {
    fn default() -> Self {
        Self {
            memory: vec![0; MEMORY_SIZE],
            pointer: 0,
            program_counter: 0,
            input_buffer: VecDeque::new(),
            output_buffer: Vec::new(),
            halted: false,
            cycles: 0,
            max_cycles: DEFAULT_MAX_CYCLES,
        }
    }
}

/// Brainfuck interpreter with CNO detection
pub struct BrainfuckInterpreter {
    program: Vec<char>,
    pub state: BrainfuckState,
}

impl BrainfuckInterpreter {
    /// Create a new interpreter for the given program
    pub fn new(program: &str) -> Self {
        Self {
            program: program.chars().collect(),
            state: BrainfuckState::default(),
        }
    }

    /// Create with custom max cycles
    pub fn with_max_cycles(program: &str, max_cycles: usize) -> Self {
        let mut interpreter = Self::new(program);
        interpreter.state.max_cycles = max_cycles;
        interpreter
    }

    /// Execute a single instruction
    pub fn step(&mut self) {
        if self.state.halted || self.state.cycles >= self.state.max_cycles {
            self.state.halted = true;
            return;
        }

        if self.state.program_counter >= self.program.len() {
            self.state.halted = true;
            return;
        }

        let instruction = self.program[self.state.program_counter];
        self.state.cycles += 1;

        match instruction {
            '>' => {
                // Move pointer right
                self.state.pointer = (self.state.pointer + 1) % self.state.memory.len();
                self.state.program_counter += 1;
            }
            '<' => {
                // Move pointer left
                self.state.pointer = self.state.pointer.checked_sub(1)
                    .unwrap_or(self.state.memory.len() - 1);
                self.state.program_counter += 1;
            }
            '+' => {
                // Increment cell
                self.state.memory[self.state.pointer] =
                    self.state.memory[self.state.pointer].wrapping_add(1);
                self.state.program_counter += 1;
            }
            '-' => {
                // Decrement cell
                self.state.memory[self.state.pointer] =
                    self.state.memory[self.state.pointer].wrapping_sub(1);
                self.state.program_counter += 1;
            }
            '.' => {
                // Output cell
                self.state.output_buffer.push(self.state.memory[self.state.pointer]);
                self.state.program_counter += 1;
            }
            ',' => {
                // Input to cell
                self.state.memory[self.state.pointer] =
                    self.state.input_buffer.pop_front().unwrap_or(0);
                self.state.program_counter += 1;
            }
            '[' => {
                // Jump forward if cell is zero
                if self.state.memory[self.state.pointer] == 0 {
                    let mut depth = 1;
                    let mut pc = self.state.program_counter + 1;
                    while depth > 0 && pc < self.program.len() {
                        match self.program[pc] {
                            '[' => depth += 1,
                            ']' => depth -= 1,
                            _ => {}
                        }
                        pc += 1;
                    }
                    self.state.program_counter = pc;
                } else {
                    self.state.program_counter += 1;
                }
            }
            ']' => {
                // Jump backward if cell is non-zero
                if self.state.memory[self.state.pointer] != 0 {
                    let mut depth = 1;
                    let mut pc = self.state.program_counter.saturating_sub(1);
                    while depth > 0 && pc > 0 {
                        match self.program[pc] {
                            ']' => depth += 1,
                            '[' => depth -= 1,
                            _ => {}
                        }
                        if depth > 0 {
                            pc = pc.saturating_sub(1);
                        }
                    }
                    self.state.program_counter = pc + 1;
                } else {
                    self.state.program_counter += 1;
                }
            }
            _ => {
                // Ignore other characters (comments)
                self.state.program_counter += 1;
            }
        }
    }

    /// Run program to completion
    pub fn run(&mut self) -> String {
        while !self.state.halted {
            self.step();
        }
        String::from_utf8_lossy(&self.state.output_buffer).to_string()
    }

    /// Check if program is a Certified Null Operation
    pub fn is_cno(&mut self) -> CnoResult {
        // Save initial state
        let initial_memory = self.state.memory.clone();
        let initial_pointer = self.state.pointer;

        // Run the program
        self.run();

        // Check CNO properties
        if self.state.cycles >= self.state.max_cycles {
            return CnoResult {
                is_cno: false,
                reason: "Program did not terminate (hit max cycles)".to_string(),
            };
        }

        if !self.state.output_buffer.is_empty() {
            return CnoResult {
                is_cno: false,
                reason: "Program produced output (not pure)".to_string(),
            };
        }

        // Memory should be unchanged
        let memory_unchanged = self.state.memory.iter()
            .zip(initial_memory.iter())
            .all(|(a, b)| a == b);

        if !memory_unchanged {
            return CnoResult {
                is_cno: false,
                reason: "Program modified memory".to_string(),
            };
        }

        // Pointer should be at initial position
        if self.state.pointer != initial_pointer {
            return CnoResult {
                is_cno: false,
                reason: format!(
                    "Pointer moved from {} to {}",
                    initial_pointer, self.state.pointer
                ),
            };
        }

        CnoResult {
            is_cno: true,
            reason: "Program is a CNO ✓".to_string(),
        }
    }
}

/// Result of CNO verification
#[derive(Debug, Clone)]
pub struct CnoResult {
    pub is_cno: bool,
    pub reason: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_cno(program: &str, expected_cno: bool) {
        let mut interpreter = BrainfuckInterpreter::new(program);
        let result = interpreter.is_cno();
        assert_eq!(result.is_cno, expected_cno, "Program: '{}' - {}", program, result.reason);
    }

    #[test]
    fn test_empty_program() {
        test_cno("", true);
    }

    #[test]
    fn test_comments_only() {
        test_cno("This is a comment", true);
    }

    #[test]
    fn test_balanced_moves() {
        test_cno("><", true);
        test_cno(">><< >><<", true);
    }

    #[test]
    fn test_balanced_increments() {
        test_cno("+-", true);
        test_cno("+-+-+-", true);
    }

    #[test]
    fn test_increment_not_cno() {
        test_cno("+", false);
    }

    #[test]
    fn test_output_not_cno() {
        test_cno(".", false);
    }

    #[test]
    fn test_move_increment_not_cno() {
        test_cno(">+<", false);
    }

    #[test]
    fn test_empty_loop() {
        test_cno("[]", true);
    }
}
