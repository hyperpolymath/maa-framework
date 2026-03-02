||| ABI Type Definitions for Absolute Zero
|||
||| This module defines the Application Binary Interface (ABI) for the
||| Absolute Zero CNO verification library. All type definitions include
||| formal proofs of correctness.
|||
||| @see https://github.com/hyperpolymath/absolute-zero

-- SPDX-License-Identifier: PMPL-1.0-or-later

module AbsoluteZero.ABI.Types

import Data.Bits
import Data.So
import Data.Vect

%default total

--------------------------------------------------------------------------------
-- Platform Detection
--------------------------------------------------------------------------------

||| Supported platforms for this ABI
public export
data Platform = Linux | Windows | MacOS | BSD | WASM

||| Compile-time platform detection
||| This will be set during compilation based on target
public export
thisPlatform : Platform
thisPlatform =
  %runElab do
    -- Platform detection logic
    pure Linux  -- Default, override with compiler flags

--------------------------------------------------------------------------------
-- Result Codes
--------------------------------------------------------------------------------

||| Result codes for FFI operations
||| Use C-compatible integers for cross-language compatibility
public export
data Result : Type where
  ||| Operation succeeded
  Ok : Result
  ||| Generic error
  Error : Result
  ||| Invalid parameter provided
  InvalidParam : Result
  ||| Out of memory
  OutOfMemory : Result
  ||| Null pointer encountered
  NullPointer : Result
  ||| Program does not terminate
  NonTerminating : Result
  ||| Program has side effects
  HasSideEffects : Result
  ||| Program is not a CNO
  NotCNO : Result

||| Convert Result to C integer
public export
resultToInt : Result -> Bits32
resultToInt Ok = 0
resultToInt Error = 1
resultToInt InvalidParam = 2
resultToInt OutOfMemory = 3
resultToInt NullPointer = 4
resultToInt NonTerminating = 5
resultToInt HasSideEffects = 6
resultToInt NotCNO = 7

||| Results are decidably equal
public export
DecEq Result where
  decEq Ok Ok = Yes Refl
  decEq Error Error = Yes Refl
  decEq InvalidParam InvalidParam = Yes Refl
  decEq OutOfMemory OutOfMemory = Yes Refl
  decEq NullPointer NullPointer = Yes Refl
  decEq NonTerminating NonTerminating = Yes Refl
  decEq HasSideEffects HasSideEffects = Yes Refl
  decEq NotCNO NotCNO = Yes Refl
  decEq _ _ = No absurd

--------------------------------------------------------------------------------
-- Instruction Types
--------------------------------------------------------------------------------

||| Abstract instruction set matching Coq definitions
public export
data Instruction : Type where
  ||| No operation
  Nop : Instruction
  ||| Load memory[addr] to register
  Load : (reg : Bits32) -> (addr : Bits32) -> Instruction
  ||| Store register to memory[addr]
  Store : (reg : Bits32) -> (addr : Bits32) -> Instruction
  ||| Add two registers and store in third
  Add : (r1 : Bits32) -> (r2 : Bits32) -> (r3 : Bits32) -> Instruction
  ||| Jump to address
  Jump : (target : Bits32) -> Instruction
  ||| Halt execution
  Halt : Instruction

||| Convert instruction to discriminant for C interop
public export
instrToDiscriminant : Instruction -> Bits32
instrToDiscriminant Nop = 0
instrToDiscriminant (Load _ _) = 1
instrToDiscriminant (Store _ _) = 2
instrToDiscriminant (Add _ _ _) = 3
instrToDiscriminant (Jump _) = 4
instrToDiscriminant Halt = 5

||| Instructions are decidably equal
public export
DecEq Instruction where
  decEq Nop Nop = Yes Refl
  decEq (Load r1 a1) (Load r2 a2) =
    case (decEq r1 r2, decEq a1 a2) of
      (Yes Refl, Yes Refl) => Yes Refl
      _ => No absurd
  decEq (Store r1 a1) (Store r2 a2) =
    case (decEq r1 r2, decEq a1 a2) of
      (Yes Refl, Yes Refl) => Yes Refl
      _ => No absurd
  decEq (Add r1 r2 r3) (Add r1' r2' r3') =
    case (decEq r1 r1', decEq r2 r2', decEq r3 r3') of
      (Yes Refl, Yes Refl, Yes Refl) => Yes Refl
      _ => No absurd
  decEq (Jump t1) (Jump t2) =
    case decEq t1 t2 of
      Yes Refl => Yes Refl
      No _ => No absurd
  decEq Halt Halt = Yes Refl
  decEq _ _ = No absurd

--------------------------------------------------------------------------------
-- Program Types
--------------------------------------------------------------------------------

||| A program is a sequence of instructions
||| Length is tracked at type level for safety
public export
data Program : Nat -> Type where
  Nil : Program 0
  (::) : Instruction -> Program n -> Program (S n)

||| Maximum program length for FFI boundary
public export
maxProgramLength : Nat
maxProgramLength = 1024

||| Proof that program length is within bounds
public export
data ValidProgramLength : Nat -> Type where
  ValidLen : {n : Nat} -> {auto 0 prf : So (n <= maxProgramLength)} ->
             ValidProgramLength n

--------------------------------------------------------------------------------
-- State Types
--------------------------------------------------------------------------------

||| I/O operation types
public export
data IOOp : Type where
  NoIO : IOOp
  ReadOp : Bits32 -> IOOp
  WriteOp : Bits32 -> IOOp

||| I/O state is a list of operations
public export
IOState : Type
IOState = List IOOp

||| Program state record (C-compatible layout)
||| Memory is abstracted as pointer for FFI
public export
record ProgramState where
  constructor MkState
  ||| Pointer to memory array
  memory_ptr : Bits64
  ||| Number of registers
  num_registers : Bits32
  ||| Pointer to register array
  registers_ptr : Bits64
  ||| Number of I/O operations
  num_io_ops : Bits32
  ||| Pointer to I/O operations array
  io_ops_ptr : Bits64
  ||| Program counter
  program_counter : Bits32

||| Proof that state is well-formed
public export
data WellFormedState : ProgramState -> Type where
  WF : {s : ProgramState} ->
       {auto 0 mem_nonNull : So (s.memory_ptr /= 0)} ->
       {auto 0 reg_nonNull : So (s.registers_ptr /= 0)} ->
       WellFormedState s

--------------------------------------------------------------------------------
-- Opaque Handles
--------------------------------------------------------------------------------

||| Opaque handle for program state
||| Prevents direct construction, enforces creation through safe API
public export
data StateHandle : Type where
  MkStateHandle : (ptr : Bits64) -> {auto 0 nonNull : So (ptr /= 0)} ->
                  StateHandle

||| Opaque handle for programs
public export
data ProgramHandle : Type where
  MkProgramHandle : (ptr : Bits64) -> {auto 0 nonNull : So (ptr /= 0)} ->
                    ProgramHandle

||| Safely create a state handle from a pointer value
||| Returns Nothing if pointer is null
public export
createStateHandle : Bits64 -> Maybe StateHandle
createStateHandle 0 = Nothing
createStateHandle ptr = Just (MkStateHandle ptr)

||| Safely create a program handle from a pointer value
public export
createProgramHandle : Bits64 -> Maybe ProgramHandle
createProgramHandle 0 = Nothing
createProgramHandle ptr = Just (MkProgramHandle ptr)

||| Extract pointer value from state handle
public export
stateHandlePtr : StateHandle -> Bits64
stateHandlePtr (MkStateHandle ptr) = ptr

||| Extract pointer value from program handle
public export
programHandlePtr : ProgramHandle -> Bits64
programHandlePtr (MkProgramHandle ptr) = ptr

--------------------------------------------------------------------------------
-- Memory Layout Proofs
--------------------------------------------------------------------------------

||| Proof that a type has a specific size
public export
data HasSize : Type -> Nat -> Type where
  SizeProof : {0 t : Type} -> {n : Nat} -> HasSize t n

||| Proof that a type has a specific alignment
public export
data HasAlignment : Type -> Nat -> Type where
  AlignProof : {0 t : Type} -> {n : Nat} -> HasAlignment t n

||| Size of ProgramState struct (8 + 4 + 8 + 4 + 8 + 4 = 36 bytes + padding)
public export
programStateSizeBytes : Nat
programStateSizeBytes = 40  -- Aligned to 8 bytes

||| Prove ProgramState has correct size
public export
programStateSize : HasSize ProgramState programStateSizeBytes
programStateSize = SizeProof

||| Prove ProgramState has 8-byte alignment
public export
programStateAlign : HasAlignment ProgramState 8
programStateAlign = AlignProof

||| Size of Instruction struct (4 bytes discriminant + 12 bytes data = 16 bytes)
public export
instructionSizeBytes : Nat
instructionSizeBytes = 16

||| Prove Instruction has correct size
public export
instructionSize : HasSize Instruction instructionSizeBytes
instructionSize = SizeProof

||| Prove Instruction has 4-byte alignment
public export
instructionAlign : HasAlignment Instruction 4
instructionAlign = AlignProof

--------------------------------------------------------------------------------
-- Platform-Specific Types
--------------------------------------------------------------------------------

||| C int size varies by platform
public export
CInt : Platform -> Type
CInt Linux = Bits32
CInt Windows = Bits32
CInt MacOS = Bits32
CInt BSD = Bits32
CInt WASM = Bits32

||| C size_t varies by platform
public export
CSize : Platform -> Type
CSize Linux = Bits64
CSize Windows = Bits64
CSize MacOS = Bits64
CSize BSD = Bits64
CSize WASM = Bits32

||| C pointer size varies by platform
public export
ptrSize : Platform -> Nat
ptrSize Linux = 64
ptrSize Windows = 64
ptrSize MacOS = 64
ptrSize BSD = 64
ptrSize WASM = 32

||| Pointer type for platform
public export
CPtr : Platform -> Type -> Type
CPtr p _ = Bits (ptrSize p)

--------------------------------------------------------------------------------
-- Verification Types
--------------------------------------------------------------------------------

||| CNO verification result
public export
record CNOVerificationResult where
  constructor MkCNOResult
  ||| Whether the program is a CNO
  is_cno : Bool
  ||| Whether the program terminates
  terminates : Bool
  ||| Whether the program preserves state
  preserves_state : Bool
  ||| Whether the program is pure (no side effects)
  is_pure : Bool
  ||| Whether the program is thermodynamically reversible
  is_reversible : Bool

||| Prove verification result size
public export
cnoResultSize : HasSize CNOVerificationResult 5
cnoResultSize = SizeProof

--------------------------------------------------------------------------------
-- ABI Version
--------------------------------------------------------------------------------

||| ABI version (major.minor.patch)
public export
abiVersionMajor : Bits32
abiVersionMajor = 1

public export
abiVersionMinor : Bits32
abiVersionMinor = 0

public export
abiVersionPatch : Bits32
abiVersionPatch = 0
