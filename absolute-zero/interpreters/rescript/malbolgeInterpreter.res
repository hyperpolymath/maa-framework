// Malbolge Interpreter in ReScript
//
// Malbolge is one of the hardest programming languages to use.
// This interpreter implements the Malbolge specification for CNO verification.
//
// Author: Jonathan D. A. Jewell
// Project: Absolute Zero
// License: AGPL-3.0 / Palimpsest 0.5

type ternary = int // Base-3 numbers

// Malbolge constants
let memorySize = 59049 // 3^10
let maxCycles = 1000000

// Malbolge operations
type operation =
  | Jmp
  | Out
  | In
  | Rot
  | Mov
  | Crz // Crazy operation
  | Nop
  | Hlt

// Malbolge state
type state = {
  memory: array<int>,
  a: int, // Accumulator
  c: int, // Code pointer
  d: int, // Data pointer
  input: list<int>,
  output: list<int>,
  halted: bool,
  cycles: int,
}

// Create initial state
let makeState = (~program: string): state => {
  let memory = Array.make(memorySize, 0)

  // Load program into memory
  String.toArray(program)->Array.forEachWithIndex((i, char) => {
    memory[i] = Char.code(char)
  })

  {
    memory: memory,
    a: 0,
    c: 0,
    d: 0,
    input: list{},
    output: list{},
    halted: false,
    cycles: 0,
  }
}

// Crazy operation lookup table (simplified)
// In real Malbolge, this is a complex ternary operation
let crazyOp = (a: int, b: int): int => {
  // Simplified implementation
  // Real Malbolge uses: [i]'th element of crz table where i = (a - b) mod 3^10
  mod(a + b, 3)
}

// Rotation in ternary
let rotateRight = (n: int): int => {
  // Simplified: rotate 10-trit ternary number right
  let lower = mod(n, 3)
  let upper = n / 3
  lower * 19683 + upper // 19683 = 3^9
}

// Decode operation at position c
let decodeOp = (mem: array<int>, c: int): option<operation> => {
  let opCode = mod(mem[c] - c, 94)

  switch opCode {
  | 4 => Some(Jmp)
  | 5 => Some(Out)
  | 23 => Some(In)
  | 39 => Some(Rot)
  | 40 => Some(Mov)
  | 62 => Some(Crz)
  | 68 => Some(Nop)
  | 81 => Some(Hlt)
  | _ => None // Invalid instruction
  }
}

// Encrypt (modify) instruction after execution
let encrypt = (mem: array<int>, c: int): unit => {
  // Malbolge self-modifies by encrypting executed instructions
  // Simplified encryption: rotate the value
  mem[c] = mod(mem[c] + 1, 256)
}

// Execute single step
let step = (state: state): state => {
  if state.halted || state.cycles >= maxCycles {
    {...state, halted: true}
  } else {
    let {memory, a, c, d, input, output, cycles} = state

    switch decodeOp(memory, c) {
    | None => {...state, halted: true} // Invalid instruction

    | Some(Hlt) => {...state, halted: true}

    | Some(Nop) => {
        encrypt(memory, c)
        {...state, c: mod(c + 1, memorySize), cycles: cycles + 1}
      }

    | Some(Jmp) => {
        encrypt(memory, c)
        {...state, c: memory[d], cycles: cycles + 1}
      }

    | Some(Out) => {
        encrypt(memory, c)
        let char = mod(a, 256)
        {
          ...state,
          c: mod(c + 1, memorySize),
          output: list{char, ...output},
          cycles: cycles + 1,
        }
      }

    | Some(In) => {
        encrypt(memory, c)
        switch input {
        | list{} => {...state, a: 0, c: mod(c + 1, memorySize), cycles: cycles + 1}
        | list{val, ...rest} => {
            {...state, a: val, input: rest, c: mod(c + 1, memorySize), cycles: cycles + 1}
          }
        }
      }

    | Some(Rot) => {
        encrypt(memory, c)
        {
          ...state,
          a: rotateRight(a),
          c: mod(c + 1, memorySize),
          d: mod(d + 1, memorySize),
          cycles: cycles + 1,
        }
      }

    | Some(Mov) => {
        encrypt(memory, c)
        {
          ...state,
          a: memory[d],
          c: mod(c + 1, memorySize),
          d: mod(d + 1, memorySize),
          cycles: cycles + 1,
        }
      }

    | Some(Crz) => {
        encrypt(memory, c)
        let result = crazyOp(a, memory[d])
        memory[d] = result
        {
          ...state,
          a: result,
          c: mod(c + 1, memorySize),
          d: mod(d + 1, memorySize),
          cycles: cycles + 1,
        }
      }
    }
  }
}

// Run program until halt or max cycles
let rec run = (state: state): state => {
  if state.halted || state.cycles >= maxCycles {
    state
  } else {
    run(step(state))
  }
}

// Execute program and return output
let execute = (program: string): string => {
  let initialState = makeState(~program)
  let finalState = run(initialState)

  // Convert output list to string
  finalState.output
  ->List.reverse
  ->List.map(Char.chr)
  ->List.toArray
  ->String.fromCharArray
}

// Check if program is a CNO
let isCNO = (program: string): bool => {
  let initialState = makeState(~program)
  let finalState = run(initialState)

  // A program is a CNO if:
  // 1. It terminates (not at max cycles)
  // 2. No output produced
  // 3. Memory unchanged (difficult to check due to encryption)

  finalState.cycles < maxCycles &&
  List.length(finalState.output) == 0
}

// CNO Examples

// Empty program - the simplest CNO
let emptyProgram = ""

// Single Nop (if we could encode it properly)
// Note: Actual Malbolge encoding is complex
let nopProgram = " " // Simplified

// Test function
let testCNO = (program: string, name: string): unit => {
  let result = if isCNO(program) {
    "is a CNO ✓"
  } else {
    "is NOT a CNO ✗"
  }

  Console.log(`${name}: ${result}`)
}

// Run tests
testCNO(emptyProgram, "Empty program")
testCNO(nopProgram, "Nop program")

// Export for external use
type exports = {
  execute: string => string,
  isCNO: string => bool,
  makeState: (~program: string) => state,
  run: state => state,
  step: state => state,
}

let exports: exports = {
  execute: execute,
  isCNO: isCNO,
  makeState: makeState,
  run: run,
  step: step,
}
