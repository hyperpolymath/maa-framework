// SPDX-License-Identifier: PMPL-1.0-or-later
// Copyright (c) 2026 Jonathan D.A. Jewell <j.d.a.jewell@open.ac.uk>
//
//! Whitespace Interpreter with Certified Null Operation (CNO) Detection.
//!
//! Whitespace is an esoteric language that ignores all non-whitespace characters.
//! Programs are composed entirely of Spaces (S), Tabs (T), and Linefeeds (L).
//!
//! ESOTERIC VERIFICATION: Similar to Brainfuck, Whitespace's limited instruction set
//! makes it a high-assurance target for the "Absolute Zero" property.
//!
//! VM COMPONENTS:
//! 1. STACK: LIFO container for temporary data.
//! 2. HEAP: Addressable key-value store for persistent variables.
//! 3. CALL STACK: Tracks subroutine returns and labels.
//!
//! CNO CRITERIA:
//! 1. TERMINATION: Must reach the End instruction or exhaust cycles.
//! 2. PURITY: No data written to the output buffer.
//! 3. STACK REVERSIBILITY: Stack returns to initial state.
//! 4. HEAP REVERSIBILITY: All heap addresses return to initial values.
//!
//! SPARK INTEGRATION NOTE:
//! The CNO verification maps to SPARK contracts:
//!   Pre  => snapshot(stack, heap)
//!   Post => stack == initial_stack AND heap == initial_heap AND output.is_empty

#![forbid(unsafe_code)]
use std::collections::HashMap;

const DEFAULT_MAX_CYCLES: usize = 1_000_000;

/// Result of CNO verification.
#[derive(Debug, Clone)]
pub struct CnoResult {
    /// Whether the program is a Certified Null Operation.
    pub is_cno: bool,
    /// Human-readable explanation of the verdict.
    pub reason: String,
}

/// Whitespace instruction set.
#[derive(Debug, Clone, PartialEq)]
pub enum WsInstruction {
    // Stack manipulation [Space]
    Push(i64),     // SS number LF
    Dup,           // SLS
    Swap,          // SLT
    Pop,           // SLL

    // Arithmetic [Tab Space]
    Add,           // TSSS
    Sub,           // TSST
    Mul,           // TSSL
    Div,           // TSTS
    Mod,           // TSTT

    // Heap access [Tab Tab]
    Store,         // TTS
    Retrieve,      // TTT

    // Flow control [Linefeed]
    Label(String), // LSS label LF
    Call(String),  // LST label LF
    Jump(String),  // LSL label LF
    JumpZero(String),  // LTS label LF
    JumpNeg(String),   // LTT label LF
    Return,        // LTL
    End,           // LLL

    // I/O [Tab Linefeed]
    OutputChar,    // TLSS
    OutputNum,     // TLST
    InputChar,     // TLTS
    InputNum,      // TLTT
}

/// The VM state for the Whitespace execution engine.
#[derive(Clone, Debug, PartialEq)]
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

/// The Interpreter orchestrator.
pub struct WhitespaceInterpreter {
    instructions: Vec<WsInstruction>,
    labels: HashMap<String, usize>,
    pub state: WhitespaceState,
}

impl WhitespaceInterpreter {
    /// Create a new interpreter from raw whitespace source.
    pub fn new(source: &str) -> Self {
        let filtered: Vec<char> = source
            .chars()
            .filter(|c| *c == ' ' || *c == '\t' || *c == '\n')
            .collect();

        let (instructions, labels) = Self::parse_instructions(&filtered);

        WhitespaceInterpreter {
            instructions,
            labels,
            state: WhitespaceState {
                stack: Vec::new(),
                heap: HashMap::new(),
                call_stack: Vec::new(),
                program_counter: 0,
                output_buffer: Vec::new(),
                input_buffer: Vec::new(),
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

    /// Parse the filtered whitespace characters into instructions.
    fn parse_instructions(chars: &[char]) -> (Vec<WsInstruction>, HashMap<String, usize>) {
        let mut instructions = Vec::new();
        let mut labels = HashMap::new();
        let mut pos = 0;

        while pos < chars.len() {
            match chars[pos] {
                ' ' => {
                    // Stack manipulation IMP
                    pos += 1;
                    if pos >= chars.len() { break; }
                    match chars[pos] {
                        ' ' => {
                            // Push: SS number LF
                            pos += 1;
                            let (num, new_pos) = Self::parse_number(chars, pos);
                            pos = new_pos;
                            instructions.push(WsInstruction::Push(num));
                        }
                        '\n' => {
                            pos += 1;
                            if pos >= chars.len() { break; }
                            match chars[pos] {
                                ' ' => { instructions.push(WsInstruction::Dup); pos += 1; }
                                '\t' => { instructions.push(WsInstruction::Swap); pos += 1; }
                                '\n' => { instructions.push(WsInstruction::Pop); pos += 1; }
                                _ => { pos += 1; }
                            }
                        }
                        _ => { pos += 1; }
                    }
                }
                '\t' => {
                    pos += 1;
                    if pos >= chars.len() { break; }
                    match chars[pos] {
                        ' ' => {
                            // Arithmetic IMP
                            pos += 1;
                            if pos >= chars.len() { break; }
                            let imp2 = if pos + 1 < chars.len() { chars[pos + 1] } else { ' ' };
                            match (chars[pos], imp2) {
                                (' ', ' ') => { instructions.push(WsInstruction::Add); pos += 2; }
                                (' ', '\t') => { instructions.push(WsInstruction::Sub); pos += 2; }
                                (' ', '\n') => { instructions.push(WsInstruction::Mul); pos += 2; }
                                ('\t', ' ') => { instructions.push(WsInstruction::Div); pos += 2; }
                                ('\t', '\t') => { instructions.push(WsInstruction::Mod); pos += 2; }
                                _ => { pos += 1; }
                            }
                        }
                        '\t' => {
                            // Heap access IMP
                            pos += 1;
                            if pos >= chars.len() { break; }
                            match chars[pos] {
                                ' ' => { instructions.push(WsInstruction::Store); pos += 1; }
                                '\t' => { instructions.push(WsInstruction::Retrieve); pos += 1; }
                                _ => { pos += 1; }
                            }
                        }
                        '\n' => {
                            // I/O IMP
                            pos += 1;
                            if pos >= chars.len() { break; }
                            let imp2 = if pos + 1 < chars.len() { chars[pos + 1] } else { ' ' };
                            match (chars[pos], imp2) {
                                (' ', ' ') => { instructions.push(WsInstruction::OutputChar); pos += 2; }
                                (' ', '\t') => { instructions.push(WsInstruction::OutputNum); pos += 2; }
                                ('\t', ' ') => { instructions.push(WsInstruction::InputChar); pos += 2; }
                                ('\t', '\t') => { instructions.push(WsInstruction::InputNum); pos += 2; }
                                _ => { pos += 1; }
                            }
                        }
                        _ => { pos += 1; }
                    }
                }
                '\n' => {
                    // Flow control IMP
                    pos += 1;
                    if pos >= chars.len() { break; }
                    match chars[pos] {
                        ' ' => {
                            pos += 1;
                            if pos >= chars.len() { break; }
                            match chars[pos] {
                                ' ' => {
                                    // Label
                                    pos += 1;
                                    let (label, new_pos) = Self::parse_label(chars, pos);
                                    pos = new_pos;
                                    labels.insert(label.clone(), instructions.len());
                                    instructions.push(WsInstruction::Label(label));
                                }
                                '\t' => {
                                    // Call
                                    pos += 1;
                                    let (label, new_pos) = Self::parse_label(chars, pos);
                                    pos = new_pos;
                                    instructions.push(WsInstruction::Call(label));
                                }
                                '\n' => {
                                    // Jump
                                    pos += 1;
                                    let (label, new_pos) = Self::parse_label(chars, pos);
                                    pos = new_pos;
                                    instructions.push(WsInstruction::Jump(label));
                                }
                                _ => { pos += 1; }
                            }
                        }
                        '\t' => {
                            pos += 1;
                            if pos >= chars.len() { break; }
                            match chars[pos] {
                                ' ' => {
                                    // Jump if zero
                                    pos += 1;
                                    let (label, new_pos) = Self::parse_label(chars, pos);
                                    pos = new_pos;
                                    instructions.push(WsInstruction::JumpZero(label));
                                }
                                '\t' => {
                                    // Jump if negative
                                    pos += 1;
                                    let (label, new_pos) = Self::parse_label(chars, pos);
                                    pos = new_pos;
                                    instructions.push(WsInstruction::JumpNeg(label));
                                }
                                '\n' => {
                                    // Return
                                    instructions.push(WsInstruction::Return);
                                    pos += 1;
                                }
                                _ => { pos += 1; }
                            }
                        }
                        '\n' => {
                            pos += 1;
                            if pos >= chars.len() { break; }
                            match chars[pos] {
                                '\n' => {
                                    // End
                                    instructions.push(WsInstruction::End);
                                    pos += 1;
                                }
                                _ => { pos += 1; }
                            }
                        }
                        _ => { pos += 1; }
                    }
                }
                _ => { pos += 1; }
            }
        }

        (instructions, labels)
    }

    /// Parse a number from the whitespace stream.
    /// Encoding: Space=0, Tab=1, Linefeed=terminator.
    /// First char is sign (Space=+, Tab=-).
    fn parse_number(chars: &[char], start: usize) -> (i64, usize) {
        let mut pos = start;
        if pos >= chars.len() {
            return (0, pos);
        }

        let sign: i64 = if chars[pos] == '\t' { -1 } else { 1 };
        pos += 1;

        let mut value: i64 = 0;
        while pos < chars.len() && chars[pos] != '\n' {
            value = value * 2 + if chars[pos] == '\t' { 1 } else { 0 };
            pos += 1;
        }

        // Skip the terminating LF
        if pos < chars.len() && chars[pos] == '\n' {
            pos += 1;
        }

        (sign * value, pos)
    }

    /// Parse a label from the whitespace stream (terminated by LF).
    fn parse_label(chars: &[char], start: usize) -> (String, usize) {
        let mut pos = start;
        let mut label = String::new();
        while pos < chars.len() && chars[pos] != '\n' {
            label.push(if chars[pos] == '\t' { '1' } else { '0' });
            pos += 1;
        }
        if pos < chars.len() && chars[pos] == '\n' {
            pos += 1;
        }
        (label, pos)
    }

    /// STEP: Executes a single Whitespace instruction.
    pub fn step(&mut self) {
        if self.state.halted || self.state.program_counter >= self.instructions.len() {
            self.state.halted = true;
            return;
        }

        if self.state.cycles >= self.state.max_cycles {
            self.state.halted = true;
            return;
        }

        self.state.cycles += 1;
        let instruction = self.instructions[self.state.program_counter].clone();

        match instruction {
            WsInstruction::Push(n) => {
                self.state.stack.push(n);
            }
            WsInstruction::Dup => {
                if let Some(&top) = self.state.stack.last() {
                    self.state.stack.push(top);
                }
            }
            WsInstruction::Swap => {
                let len = self.state.stack.len();
                if len >= 2 {
                    self.state.stack.swap(len - 1, len - 2);
                }
            }
            WsInstruction::Pop => {
                self.state.stack.pop();
            }
            WsInstruction::Add => {
                if self.state.stack.len() >= 2 {
                    let b = self.state.stack.pop().expect("checked len");
                    let a = self.state.stack.pop().expect("checked len");
                    self.state.stack.push(a.wrapping_add(b));
                }
            }
            WsInstruction::Sub => {
                if self.state.stack.len() >= 2 {
                    let b = self.state.stack.pop().expect("checked len");
                    let a = self.state.stack.pop().expect("checked len");
                    self.state.stack.push(a.wrapping_sub(b));
                }
            }
            WsInstruction::Mul => {
                if self.state.stack.len() >= 2 {
                    let b = self.state.stack.pop().expect("checked len");
                    let a = self.state.stack.pop().expect("checked len");
                    self.state.stack.push(a.wrapping_mul(b));
                }
            }
            WsInstruction::Div => {
                if self.state.stack.len() >= 2 {
                    let b = self.state.stack.pop().expect("checked len");
                    let a = self.state.stack.pop().expect("checked len");
                    if b != 0 {
                        self.state.stack.push(a / b);
                    } else {
                        self.state.halted = true;
                    }
                }
            }
            WsInstruction::Mod => {
                if self.state.stack.len() >= 2 {
                    let b = self.state.stack.pop().expect("checked len");
                    let a = self.state.stack.pop().expect("checked len");
                    if b != 0 {
                        self.state.stack.push(a % b);
                    } else {
                        self.state.halted = true;
                    }
                }
            }
            WsInstruction::Store => {
                if self.state.stack.len() >= 2 {
                    let value = self.state.stack.pop().expect("checked len");
                    let addr = self.state.stack.pop().expect("checked len");
                    self.state.heap.insert(addr, value);
                }
            }
            WsInstruction::Retrieve => {
                if let Some(&addr) = self.state.stack.last() {
                    self.state.stack.pop();
                    let value = self.state.heap.get(&addr).copied().unwrap_or(0);
                    self.state.stack.push(value);
                }
            }
            WsInstruction::Label(_) => {
                // Labels are no-ops at runtime (pre-resolved)
            }
            WsInstruction::Call(ref label) => {
                if let Some(&target) = self.labels.get(label) {
                    self.state.call_stack.push(self.state.program_counter + 1);
                    self.state.program_counter = target;
                    return; // Don't increment PC
                }
            }
            WsInstruction::Jump(ref label) => {
                if let Some(&target) = self.labels.get(label) {
                    self.state.program_counter = target;
                    return;
                }
            }
            WsInstruction::JumpZero(ref label) => {
                if let Some(top) = self.state.stack.pop() {
                    if top == 0 {
                        if let Some(&target) = self.labels.get(label) {
                            self.state.program_counter = target;
                            return;
                        }
                    }
                }
            }
            WsInstruction::JumpNeg(ref label) => {
                if let Some(top) = self.state.stack.pop() {
                    if top < 0 {
                        if let Some(&target) = self.labels.get(label) {
                            self.state.program_counter = target;
                            return;
                        }
                    }
                }
            }
            WsInstruction::Return => {
                if let Some(ret_addr) = self.state.call_stack.pop() {
                    self.state.program_counter = ret_addr;
                    return;
                }
            }
            WsInstruction::End => {
                self.state.halted = true;
                return;
            }
            WsInstruction::OutputChar => {
                if let Some(&top) = self.state.stack.last() {
                    self.state.stack.pop();
                    self.state
                        .output_buffer
                        .push(format!("{}", top as u8 as char));
                }
            }
            WsInstruction::OutputNum => {
                if let Some(&top) = self.state.stack.last() {
                    self.state.stack.pop();
                    self.state.output_buffer.push(format!("{}", top));
                }
            }
            WsInstruction::InputChar => {
                let ch = self.state.input_buffer.pop().unwrap_or('\0');
                if let Some(&addr) = self.state.stack.last() {
                    self.state.stack.pop();
                    self.state.heap.insert(addr, ch as i64);
                }
            }
            WsInstruction::InputNum => {
                // Stub: no interactive input in CNO verification context
                if let Some(&addr) = self.state.stack.last() {
                    self.state.stack.pop();
                    self.state.heap.insert(addr, 0);
                }
            }
        }

        self.state.program_counter += 1;
    }

    /// RUN: Execute the program to completion (or cycle limit).
    pub fn run(&mut self) {
        while !self.state.halted && self.state.program_counter < self.instructions.len() {
            self.step();
        }
        self.state.halted = true;
    }

    /// VERIFICATION: Determines if the loaded program is a Certified Null Operation.
    ///
    /// Criteria for CNO in Whitespace:
    /// 1. TERMINATION: Must halt within cycle limit.
    /// 2. PURITY: No data written to the output buffer.
    /// 3. STACK REVERSIBILITY: The stack must be returned to its initial depth and content.
    /// 4. HEAP REVERSIBILITY: All heap addresses must contain their initial values.
    ///
    /// SPARK contract equivalent:
    ///   Pre:  state == initial_state
    ///   Post: result.is_cno => (stack == init_stack AND heap == init_heap AND output.is_empty)
    pub fn is_cno(&mut self) -> CnoResult {
        let initial_stack = self.state.stack.clone();
        let initial_heap = self.state.heap.clone();

        self.run();

        if self.state.cycles >= self.state.max_cycles {
            return CnoResult {
                is_cno: false,
                reason: "Non-termination: exceeded cycle limit".into(),
            };
        }

        if !self.state.output_buffer.is_empty() {
            return CnoResult {
                is_cno: false,
                reason: "Impurity: program produced output".into(),
            };
        }

        if self.state.stack != initial_stack {
            return CnoResult {
                is_cno: false,
                reason: "Irreversible: stack was modified".into(),
            };
        }

        if self.state.heap != initial_heap {
            return CnoResult {
                is_cno: false,
                reason: "Irreversible: heap was modified".into(),
            };
        }

        CnoResult {
            is_cno: true,
            reason: "Certified Null Operation: terminates, pure, reversible".into(),
        }
    }

    /// Reset the interpreter for re-verification.
    pub fn reset(&mut self) {
        self.state.stack.clear();
        self.state.heap.clear();
        self.state.call_stack.clear();
        self.state.program_counter = 0;
        self.state.output_buffer.clear();
        self.state.input_buffer.clear();
        self.state.halted = false;
        self.state.cycles = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_program_is_cno() {
        let mut interp = WhitespaceInterpreter::new("");
        let result = interp.is_cno();
        assert!(result.is_cno, "Empty program must be a CNO");
    }

    #[test]
    fn push_pop_is_cno() {
        // Push 0 then pop: SS S LF SLL → stack returns to empty
        // Space Space (push IMP) + Space (sign +) + LF (terminate: value=0)
        // Space LF LF (pop)
        let program = "   \n \n\n";
        let mut interp = WhitespaceInterpreter::new(program);
        let result = interp.is_cno();
        assert!(result.is_cno, "Push then pop must be a CNO: {}", result.reason);
    }

    #[test]
    fn unbalanced_push_not_cno() {
        // Push 5 without popping
        // SS T S T LF = push 5 (sign=+, binary 101 = 5)
        let program = "  \t \t\n";
        let mut interp = WhitespaceInterpreter::new(program);
        let result = interp.is_cno();
        assert!(!result.is_cno, "Unbalanced push must NOT be a CNO");
    }

    #[test]
    fn end_instruction_halts() {
        // LLL = End program
        let program = "\n\n\n";
        let mut interp = WhitespaceInterpreter::new(program);
        let result = interp.is_cno();
        assert!(result.is_cno, "End instruction on empty state is a CNO");
    }

    #[test]
    fn reset_allows_rerun() {
        let mut interp = WhitespaceInterpreter::new("");
        let r1 = interp.is_cno();
        assert!(r1.is_cno);
        interp.reset();
        let r2 = interp.is_cno();
        assert!(r2.is_cno, "Reset must allow clean re-verification");
    }
}
