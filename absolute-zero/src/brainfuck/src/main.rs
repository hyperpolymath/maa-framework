//! Brainfuck CNO Detection CLI
//!
//! Tests various Brainfuck programs for CNO properties.

use brainfuck_cno::BrainfuckInterpreter;

fn test_cno(program: &str, name: &str) {
    let mut interpreter = BrainfuckInterpreter::new(program);
    let result = interpreter.is_cno();

    let status = if result.is_cno { "✓" } else { "✗" };
    println!("{:30} [{}] {}", name, status, result.reason);
}

fn main() {
    println!("{}", "=".repeat(70));
    println!("Brainfuck CNO Detection Tests");
    println!("{}", "=".repeat(70));
    println!();

    // Test cases
    test_cno("", "Empty program");
    test_cno("This is a comment", "Comments only");
    test_cno("><", "Move right then left");
    test_cno("+-", "Increment then decrement");
    test_cno("+", "Increment (not CNO)");
    test_cno(".", "Output (not CNO)");
    test_cno(">+<", "Move, increment, move back (not CNO)");
    test_cno(">><< >><<", "Multiple balanced moves");
    test_cno("+-+-+-", "Multiple balanced increments");
    test_cno("[+]", "Loop (should be CNO if cell is 0)");
    test_cno("[]", "Empty loop");
    test_cno("+[-]", "Increment then loop to zero (not CNO)");
    test_cno("+>-<", "Increment one cell, decrement another (not CNO)");

    println!();
    println!("{}", "=".repeat(70));
}
