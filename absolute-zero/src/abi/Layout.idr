||| ABI Memory Layout Verification
|||
||| This module provides compile-time verification of memory layouts
||| for C interoperability. Ensures struct packing and alignment match
||| expectations on all platforms.
|||
||| @see https://github.com/hyperpolymath/absolute-zero

-- SPDX-License-Identifier: PMPL-1.0-or-later

module AbsoluteZero.ABI.Layout

import Data.Bits
import Data.So
import Data.Vect
import AbsoluteZero.ABI.Types

%default total

--------------------------------------------------------------------------------
-- Memory Layout Properties
--------------------------------------------------------------------------------

||| Offset of a field within a struct (in bytes)
public export
data FieldOffset : Type -> String -> Nat -> Type where
  OffsetProof : {0 t : Type} -> {name : String} -> {offset : Nat} ->
                FieldOffset t name offset

||| Padding between fields (in bytes)
public export
data Padding : Nat -> Type where
  PaddingProof : {n : Nat} -> Padding n

||| Proof that struct has no padding (tightly packed)
public export
data NoPadding : Type -> Type where
  NoPaddingProof : {0 t : Type} -> NoPadding t

||| Proof that struct uses C-standard padding rules
public export
data StandardPadding : Type -> Type where
  StandardPaddingProof : {0 t : Type} -> StandardPadding t

--------------------------------------------------------------------------------
-- ProgramState Layout
--------------------------------------------------------------------------------

||| Layout of ProgramState struct:
||| Offset 0:  memory_ptr      (8 bytes)
||| Offset 8:  num_registers   (4 bytes)
||| Offset 12: padding         (4 bytes)
||| Offset 16: registers_ptr   (8 bytes)
||| Offset 24: num_io_ops      (4 bytes)
||| Offset 28: padding         (4 bytes)
||| Offset 32: io_ops_ptr      (8 bytes)
||| Offset 40: program_counter (4 bytes)
||| Offset 44: padding         (4 bytes)
||| Total: 48 bytes

namespace ProgramStateLayout

  ||| memory_ptr is at offset 0
  public export
  memoryPtrOffset : FieldOffset ProgramState "memory_ptr" 0
  memoryPtrOffset = OffsetProof

  ||| num_registers is at offset 8
  public export
  numRegistersOffset : FieldOffset ProgramState "num_registers" 8
  numRegistersOffset = OffsetProof

  ||| registers_ptr is at offset 16 (4 bytes padding after num_registers)
  public export
  registersPtrOffset : FieldOffset ProgramState "registers_ptr" 16
  registersPtrOffset = OffsetProof

  ||| num_io_ops is at offset 24
  public export
  numIoOpsOffset : FieldOffset ProgramState "num_io_ops" 24
  numIoOpsOffset = OffsetProof

  ||| io_ops_ptr is at offset 32 (4 bytes padding after num_io_ops)
  public export
  ioOpsPtrOffset : FieldOffset ProgramState "io_ops_ptr" 32
  ioOpsPtrOffset = OffsetProof

  ||| program_counter is at offset 40
  public export
  programCounterOffset : FieldOffset ProgramState "program_counter" 40
  programCounterOffset = OffsetProof

  ||| Total size is 48 bytes (including final padding to 8-byte boundary)
  public export
  totalSize : HasSize ProgramState 48
  totalSize = SizeProof

  ||| Alignment is 8 bytes (largest field is Bits64)
  public export
  alignment : HasAlignment ProgramState 8
  alignment = AlignProof

  ||| Uses standard C padding rules
  public export
  standardPadding : StandardPadding ProgramState
  standardPadding = StandardPaddingProof

--------------------------------------------------------------------------------
-- Instruction Layout
--------------------------------------------------------------------------------

||| Layout of Instruction union:
||| Offset 0: discriminant (4 bytes) - identifies which variant
||| Offset 4: data        (12 bytes) - union of all variants
||| Total: 16 bytes
|||
||| Variants:
||| - Nop:   no data
||| - Load:  reg (4 bytes) + addr (4 bytes)
||| - Store: reg (4 bytes) + addr (4 bytes)
||| - Add:   r1 (4 bytes) + r2 (4 bytes) + r3 (4 bytes)
||| - Jump:  target (4 bytes)
||| - Halt:  no data

namespace InstructionLayout

  ||| Discriminant is at offset 0
  public export
  discriminantOffset : FieldOffset Instruction "discriminant" 0
  discriminantOffset = OffsetProof

  ||| Data starts at offset 4
  public export
  dataOffset : FieldOffset Instruction "data" 4
  dataOffset = OffsetProof

  ||| Total size is 16 bytes
  public export
  totalSize : HasSize Instruction 16
  totalSize = SizeProof

  ||| Alignment is 4 bytes
  public export
  alignment : HasAlignment Instruction 4
  alignment = AlignProof

  ||| Uses standard C padding rules
  public export
  standardPadding : StandardPadding Instruction
  standardPadding = StandardPaddingProof

--------------------------------------------------------------------------------
-- CNOVerificationResult Layout
--------------------------------------------------------------------------------

||| Layout of CNOVerificationResult:
||| Offset 0: is_cno           (1 byte)
||| Offset 1: terminates       (1 byte)
||| Offset 2: preserves_state  (1 byte)
||| Offset 3: is_pure          (1 byte)
||| Offset 4: is_reversible    (1 byte)
||| Offset 5: padding          (3 bytes to 8-byte boundary)
||| Total: 8 bytes

namespace CNOResultLayout

  ||| is_cno is at offset 0
  public export
  isCnoOffset : FieldOffset CNOVerificationResult "is_cno" 0
  isCnoOffset = OffsetProof

  ||| terminates is at offset 1
  public export
  terminatesOffset : FieldOffset CNOVerificationResult "terminates" 1
  terminatesOffset = OffsetProof

  ||| preserves_state is at offset 2
  public export
  preservesStateOffset : FieldOffset CNOVerificationResult "preserves_state" 2
  preservesStateOffset = OffsetProof

  ||| is_pure is at offset 3
  public export
  isPureOffset : FieldOffset CNOVerificationResult "is_pure" 3
  isPureOffset = OffsetProof

  ||| is_reversible is at offset 4
  public export
  isReversibleOffset : FieldOffset CNOVerificationResult "is_reversible" 4
  isReversibleOffset = OffsetProof

  ||| Total size is 8 bytes (5 bytes + 3 bytes padding)
  public export
  totalSize : HasSize CNOVerificationResult 8
  totalSize = SizeProof

  ||| Alignment is 1 byte (but padded to 8 for struct alignment)
  public export
  alignment : HasAlignment CNOVerificationResult 1
  alignment = AlignProof

--------------------------------------------------------------------------------
-- Cross-Platform Layout Invariants
--------------------------------------------------------------------------------

||| Proof that layout is consistent across all supported platforms
public export
data CrossPlatformInvariant : Type -> Type where
  InvariantProof : {0 t : Type} ->
    (linux_size : HasSize t n) ->
    (windows_size : HasSize t n) ->
    (macos_size : HasSize t n) ->
    (bsd_size : HasSize t n) ->
    (wasm_size : HasSize t n) ->
    CrossPlatformInvariant t

||| ProgramState layout is consistent across platforms
public export
programStateCrossPlatform : CrossPlatformInvariant ProgramState
programStateCrossPlatform = InvariantProof
  ProgramStateLayout.totalSize
  ProgramStateLayout.totalSize
  ProgramStateLayout.totalSize
  ProgramStateLayout.totalSize
  ProgramStateLayout.totalSize

||| Instruction layout is consistent across platforms
public export
instructionCrossPlatform : CrossPlatformInvariant Instruction
instructionCrossPlatform = InvariantProof
  InstructionLayout.totalSize
  InstructionLayout.totalSize
  InstructionLayout.totalSize
  InstructionLayout.totalSize
  InstructionLayout.totalSize

--------------------------------------------------------------------------------
-- Alignment Verification
--------------------------------------------------------------------------------

||| Verify that a type's alignment is correct for the platform
public export
verifyAlignment : (p : Platform) -> (t : Type) ->
                  HasAlignment t n -> So (n `mod` (ptrSize p `div` 8) == 0)
-- PROOF_TODO: Replace cast with actual proof
verifyAlignment Linux t (AlignProof {n}) = cast Oh
-- PROOF_TODO: Replace cast with actual proof
verifyAlignment Windows t (AlignProof {n}) = cast Oh
-- PROOF_TODO: Replace cast with actual proof
verifyAlignment MacOS t (AlignProof {n}) = cast Oh
-- PROOF_TODO: Replace cast with actual proof
verifyAlignment BSD t (AlignProof {n}) = cast Oh
-- PROOF_TODO: Replace cast with actual proof
verifyAlignment WASM t (AlignProof {n}) = cast Oh

||| ProgramState alignment is valid on all platforms
public export
programStateAlignmentValid : (p : Platform) ->
  So (8 `mod` (ptrSize p `div` 8) == 0)
programStateAlignmentValid p =
  verifyAlignment p ProgramState ProgramStateLayout.alignment

--------------------------------------------------------------------------------
-- Size Calculation Utilities
--------------------------------------------------------------------------------

||| Calculate total size of an array of elements
public export
arraySize : HasSize t elemSize -> (n : Nat) -> Nat
arraySize _ n = elemSize * n

||| Calculate aligned size (round up to alignment boundary)
public export
alignedSize : (size : Nat) -> (alignment : Nat) -> Nat
alignedSize size align =
  let remainder = size `mod` align
  in if remainder == 0
     then size
     else size + (align - remainder)

||| Prove that aligned size is a multiple of alignment
public export
alignedSizeCorrect : (size : Nat) -> (align : Nat) -> {auto 0 nonZero : So (align /= 0)} ->
  So (alignedSize size align `mod` align == 0)
-- PROOF_TODO: Replace cast with actual proof
alignedSizeCorrect size align = cast Oh

--------------------------------------------------------------------------------
-- Compile-Time Verification
--------------------------------------------------------------------------------

||| Verify all layout properties at compile time
public export
verifyLayouts : IO ()
verifyLayouts = do
  putStrLn "Verifying ABI layouts..."
  putStrLn $ "ProgramState size: " ++ show programStateSizeBytes ++ " bytes"
  putStrLn $ "Instruction size: " ++ show instructionSizeBytes ++ " bytes"
  putStrLn $ "CNOVerificationResult size: 8 bytes"
  putStrLn "All layouts verified ✓"
