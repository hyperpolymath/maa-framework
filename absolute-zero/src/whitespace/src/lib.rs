//! Whitespace Interpreter with CNO Detection
//!
//! Whitespace is an esoteric language that uses only spaces, tabs, and linefeeds.
//! All other characters are ignored.
//!
//! Author: Jonathan D. A. Jewell
//! Project: Absolute Zero
//! License: AGPL-3.0 / Palimpsest 0.5

use std::collections::HashMap;

const DEFAULT_MAX_CYCLES: usize = 1_000_000;

/// State of the Whitespace interpreter
#[derive(Clone)]
pub struct WhitespaceState {
    pub stack: Vec<i64>,
    pub heap: HashMap<i64, i64>,
    pub call_stack: Vec<usize>,
    pub program_counter: usize,
    pub output_buffer: Vec<String>,
    pub input_buffer: Vec<char>,
    pub halted: bool,
    pub cycles: usize,
    pub max_cycles: usize,
}

impl Default for WhitespaceState {
    fn default() -> Self {
        Self {
            stack: Vec::new(),
            heap: HashMap::new(),
            call_stack: Vec::new(),
            program_counter: 0,
            output_buffer: Vec::new(),
            input_buffer: Vec::new(),
            halted: false,
            cycles: 0,
            max_cycles: DEFAULT_MAX_CYCLES,
        }
    }
}

/// Whitespace interpreter with CNO detection
pub struct WhitespaceInterpreter {
    program: Vec<char>,
    pub state: WhitespaceState,
    labels: HashMap<String, usize>,
}

impl WhitespaceInterpreter {
    /// Create a new interpreter for the given program
    pub fn new(program: &str) -> Self {
        // Filter to only whitespace characters
        let filtered: Vec<char> = program
            .chars()
            .filter(|c| *c == ' ' || *c == '\t' || *c == '\n')
            .collect();

        let mut interpreter = Self {
            program: filtered,
            state: WhitespaceState::default(),
            labels: HashMap::new(),
        };
        interpreter.parse_labels();
        interpreter
    }

    /// Pre-parse all labels for jump targets
    fn parse_labels(&mut self) {
        let mut i = 0;
        while i < self.program.len() {
            if self.match_pattern(i, &['\n', ' ', ' ']) {
                if let Some((label, offset)) = self.parse_label(i + 3) {
                    self.labels.insert(label, i);
                    i += offset;
                }
            }
            i += 1;
        }
    }

    /// Check if pattern matches at position
    fn match_pattern(&self, pos: usize, pattern: &[char]) -> bool {
        if pos + pattern.len() > self.program.len() {
            return false;
        }
        self.program[pos..pos + pattern.len()] == *pattern
    }

    /// Parse a number (space=0, tab=1, terminated by newline)
    fn parse_number(&self, pos: usize) -> (i64, usize) {
        if pos >= self.program.len() {
            return (0, 0);
        }

        // First char is sign (space=+, tab=-)
        let sign: i64 = if self.program[pos] == '\t' { -1 } else { 1 };
        let mut current_pos = pos + 1;

        // Parse binary number
        let mut num: i64 = 0;
        let mut offset = 1;
        while current_pos < self.program.len() && self.program[current_pos] != '\n' {
            num = num * 2 + if self.program[current_pos] == '\t' { 1 } else { 0 };
            current_pos += 1;
            offset += 1;
        }

        (sign * num, offset + 1)
    }

    /// Parse a label (terminated by newline)
    fn parse_label(&self, pos: usize) -> Option<(String, usize)> {
        let mut label = String::new();
        let mut offset = 0;
        while pos + offset < self.program.len() && self.program[pos + offset] != '\n' {
            label.push(self.program[pos + offset]);
            offset += 1;
        }
        Some((label, offset + 1))
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

        let pos = self.state.program_counter;
        self.state.cycles += 1;

        // Stack manipulation (space prefix)
        if self.match_pattern(pos, &[' ', ' ']) {
            // Push number
            let (num, offset) = self.parse_number(pos + 2);
            self.state.stack.push(num);
            self.state.program_counter += 2 + offset;
        } else if self.match_pattern(pos, &[' ', '\n', ' ']) {
            // Duplicate top
            if let Some(&top) = self.state.stack.last() {
                self.state.stack.push(top);
            }
            self.state.program_counter += 3;
        } else if self.match_pattern(pos, &[' ', '\n', '\t']) {
            // Swap top two
            let len = self.state.stack.len();
            if len >= 2 {
                self.state.stack.swap(len - 1, len - 2);
            }
            self.state.program_counter += 3;
        } else if self.match_pattern(pos, &[' ', '\n', '\n']) {
            // Discard top
            self.state.stack.pop();
            self.state.program_counter += 3;
        }
        // Arithmetic (tab-space prefix)
        else if self.match_pattern(pos, &['\t', ' ', ' ', ' ']) {
            // Addition
            if self.state.stack.len() >= 2 {
                let b = self.state.stack.pop().unwrap();
                let a = self.state.stack.pop().unwrap();
                self.state.stack.push(a + b);
            }
            self.state.program_counter += 4;
        } else if self.match_pattern(pos, &['\t', ' ', ' ', '\t']) {
            // Subtraction
            if self.state.stack.len() >= 2 {
                let b = self.state.stack.pop().unwrap();
                let a = self.state.stack.pop().unwrap();
                self.state.stack.push(a - b);
            }
            self.state.program_counter += 4;
        } else if self.match_pattern(pos, &['\t', ' ', ' ', '\n']) {
            // Multiplication
            if self.state.stack.len() >= 2 {
                let b = self.state.stack.pop().unwrap();
                let a = self.state.stack.pop().unwrap();
                self.state.stack.push(a * b);
            }
            self.state.program_counter += 4;
        }
        // Heap access (tab-tab prefix)
        else if self.match_pattern(pos, &['\t', '\t', ' ']) {
            // Store to heap
            if self.state.stack.len() >= 2 {
                let value = self.state.stack.pop().unwrap();
                let address = self.state.stack.pop().unwrap();
                self.state.heap.insert(address, value);
            }
            self.state.program_counter += 3;
        } else if self.match_pattern(pos, &['\t', '\t', '\t']) {
            // Retrieve from heap
            if let Some(address) = self.state.stack.pop() {
                let value = *self.state.heap.get(&address).unwrap_or(&0);
                self.state.stack.push(value);
            }
            self.state.program_counter += 3;
        }
        // I/O (tab-newline prefix)
        else if self.match_pattern(pos, &['\t', '\n', ' ', ' ']) {
            // Output character
            if let Some(char_code) = self.state.stack.pop() {
                let c = char::from_u32((char_code % 256) as u32).unwrap_or('?');
                self.state.output_buffer.push(c.to_string());
            }
            self.state.program_counter += 4;
        } else if self.match_pattern(pos, &['\t', '\n', ' ', '\t']) {
            // Output number
            if let Some(num) = self.state.stack.pop() {
                self.state.output_buffer.push(num.to_string());
            }
            self.state.program_counter += 4;
        }
        // Flow control (newline prefix)
        else if self.match_pattern(pos, &['\n', '\n', '\n']) {
            // End program
            self.state.halted = true;
        } else {
            // Unknown instruction or comment
            self.state.program_counter += 1;
        }
    }

    /// Run program to completion
    pub fn run(&mut self) -> String {
        while !self.state.halted {
            self.step();
        }
        self.state.output_buffer.join("")
    }

    /// Check if program is a Certified Null Operation
    pub fn is_cno(&mut self) -> CnoResult {
        // Save initial state
        let initial_stack = self.state.stack.clone();
        let initial_heap = self.state.heap.clone();

        // Run the program
        self.run();

        // Check CNO properties
        if self.state.cycles >= self.state.max_cycles {
            return CnoResult {
                is_cno: false,
                reason: "Program did not terminate".to_string(),
            };
        }

        if !self.state.output_buffer.is_empty() {
            return CnoResult {
                is_cno: false,
                reason: "Program produced output".to_string(),
            };
        }

        if self.state.stack != initial_stack {
            return CnoResult {
                is_cno: false,
                reason: "Stack was modified".to_string(),
            };
        }

        if self.state.heap != initial_heap {
            return CnoResult {
                is_cno: false,
                reason: "Heap was modified".to_string(),
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

    #[test]
    fn test_empty_program() {
        let mut interp = WhitespaceInterpreter::new("");
        let result = interp.is_cno();
        assert!(result.is_cno);
    }

    #[test]
    fn test_non_whitespace_ignored() {
        let mut interp = WhitespaceInterpreter::new("This is ignored");
        let result = interp.is_cno();
        assert!(result.is_cno);
    }

    #[test]
    fn test_immediate_halt() {
        let mut interp = WhitespaceInterpreter::new("\n\n\n");
        let result = interp.is_cno();
        assert!(result.is_cno);
    }
}
