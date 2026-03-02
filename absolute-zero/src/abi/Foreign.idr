||| Foreign Function Interface Declarations
|||
||| This module declares all C-compatible functions that will be
||| implemented in the Zig FFI layer for CNO verification.
|||
||| All functions are declared here with type signatures and safety proofs.
||| Implementations live in ffi/zig/

-- SPDX-License-Identifier: PMPL-1.0-or-later

module AbsoluteZero.ABI.Foreign

import AbsoluteZero.ABI.Types
import AbsoluteZero.ABI.Layout

%default total

--------------------------------------------------------------------------------
-- Library Lifecycle
--------------------------------------------------------------------------------

||| Initialize the CNO verification library
||| Returns a handle to the library instance, or 0 on failure
export
%foreign "C:absolute_zero_init, libabsolute_zero"
prim__init : PrimIO Bits64

||| Safe wrapper for library initialization
export
init : IO (Maybe StateHandle)
init = do
  ptr <- primIO prim__init
  pure (createStateHandle ptr)

||| Clean up library resources
export
%foreign "C:absolute_zero_free, libabsolute_zero"
prim__free : Bits64 -> PrimIO ()

||| Safe wrapper for cleanup
export
free : StateHandle -> IO ()
free h = primIO (prim__free (stateHandlePtr h))

--------------------------------------------------------------------------------
-- Program State Operations
--------------------------------------------------------------------------------

||| Create a new program state
||| Returns handle to state, or 0 on failure
export
%foreign "C:az_state_create, libabsolute_zero"
prim__stateCreate : PrimIO Bits64

||| Safe state creation
export
stateCreate : IO (Maybe StateHandle)
stateCreate = do
  ptr <- primIO prim__stateCreate
  pure (createStateHandle ptr)

||| Destroy a program state
export
%foreign "C:az_state_destroy, libabsolute_zero"
prim__stateDestroy : Bits64 -> PrimIO ()

||| Safe state destruction
export
stateDestroy : StateHandle -> IO ()
stateDestroy h = primIO (prim__stateDestroy (stateHandlePtr h))

||| Compare two program states for equality
||| Returns 1 if equal, 0 if not equal
export
%foreign "C:az_state_eq, libabsolute_zero"
prim__stateEq : Bits64 -> Bits64 -> PrimIO Bits32

||| Safe state comparison
export
stateEq : StateHandle -> StateHandle -> IO Bool
stateEq s1 s2 = do
  result <- primIO (prim__stateEq (stateHandlePtr s1) (stateHandlePtr s2))
  pure (result /= 0)

||| Clone a program state
||| Returns handle to new state, or 0 on failure
export
%foreign "C:az_state_clone, libabsolute_zero"
prim__stateClone : Bits64 -> PrimIO Bits64

||| Safe state cloning
export
stateClone : StateHandle -> IO (Maybe StateHandle)
stateClone h = do
  ptr <- primIO (prim__stateClone (stateHandlePtr h))
  pure (createStateHandle ptr)

||| Get memory value at address
export
%foreign "C:az_state_get_memory, libabsolute_zero"
prim__stateGetMemory : Bits64 -> Bits32 -> PrimIO Bits32

||| Safe memory read
export
stateGetMemory : StateHandle -> Bits32 -> IO Bits32
stateGetMemory h addr = primIO (prim__stateGetMemory (stateHandlePtr h) addr)

||| Set memory value at address
export
%foreign "C:az_state_set_memory, libabsolute_zero"
prim__stateSetMemory : Bits64 -> Bits32 -> Bits32 -> PrimIO ()

||| Safe memory write
export
stateSetMemory : StateHandle -> Bits32 -> Bits32 -> IO ()
stateSetMemory h addr val = primIO (prim__stateSetMemory (stateHandlePtr h) addr val)

||| Get register value
export
%foreign "C:az_state_get_register, libabsolute_zero"
prim__stateGetRegister : Bits64 -> Bits32 -> PrimIO Bits32

||| Safe register read
export
stateGetRegister : StateHandle -> Bits32 -> IO Bits32
stateGetRegister h reg = primIO (prim__stateGetRegister (stateHandlePtr h) reg)

||| Set register value
export
%foreign "C:az_state_set_register, libabsolute_zero"
prim__stateSetRegister : Bits64 -> Bits32 -> Bits32 -> PrimIO ()

||| Safe register write
export
stateSetRegister : StateHandle -> Bits32 -> Bits32 -> IO ()
stateSetRegister h reg val = primIO (prim__stateSetRegister (stateHandlePtr h) reg val)

--------------------------------------------------------------------------------
-- Program Operations
--------------------------------------------------------------------------------

||| Create a new program
||| Returns handle to program, or 0 on failure
export
%foreign "C:az_program_create, libabsolute_zero"
prim__programCreate : PrimIO Bits64

||| Safe program creation
export
programCreate : IO (Maybe ProgramHandle)
programCreate = do
  ptr <- primIO prim__programCreate
  pure (createProgramHandle ptr)

||| Destroy a program
export
%foreign "C:az_program_destroy, libabsolute_zero"
prim__programDestroy : Bits64 -> PrimIO ()

||| Safe program destruction
export
programDestroy : ProgramHandle -> IO ()
programDestroy h = primIO (prim__programDestroy (programHandlePtr h))

||| Add instruction to program
||| discriminant: 0=Nop, 1=Load, 2=Store, 3=Add, 4=Jump, 5=Halt
export
%foreign "C:az_program_add_instruction, libabsolute_zero"
prim__programAddInstruction : Bits64 -> Bits32 -> Bits32 -> Bits32 -> Bits32 -> PrimIO Bits32

||| Safe instruction addition
export
programAddInstruction : ProgramHandle -> Instruction -> IO (Either Result ())
programAddInstruction h instr = do
  let (disc, arg1, arg2, arg3) = instrToArgs instr
  result <- primIO (prim__programAddInstruction (programHandlePtr h) disc arg1 arg2 arg3)
  pure $ if result == 0 then Right () else Left (intToResult result)
  where
    instrToArgs : Instruction -> (Bits32, Bits32, Bits32, Bits32)
    instrToArgs Nop = (0, 0, 0, 0)
    instrToArgs (Load reg addr) = (1, reg, addr, 0)
    instrToArgs (Store reg addr) = (2, reg, addr, 0)
    instrToArgs (Add r1 r2 r3) = (3, r1, r2, r3)
    instrToArgs (Jump target) = (4, target, 0, 0)
    instrToArgs Halt = (5, 0, 0, 0)

    intToResult : Bits32 -> Result
    intToResult 1 = Error
    intToResult 2 = InvalidParam
    intToResult 3 = OutOfMemory
    intToResult _ = Error

||| Get program length (number of instructions)
export
%foreign "C:az_program_length, libabsolute_zero"
prim__programLength : Bits64 -> PrimIO Bits32

||| Safe program length query
export
programLength : ProgramHandle -> IO Bits32
programLength h = primIO (prim__programLength (programHandlePtr h))

--------------------------------------------------------------------------------
-- Program Evaluation
--------------------------------------------------------------------------------

||| Evaluate a program on a state
||| Returns handle to resulting state, or 0 on failure
export
%foreign "C:az_eval, libabsolute_zero"
prim__eval : Bits64 -> Bits64 -> PrimIO Bits64

||| Safe program evaluation
export
eval : ProgramHandle -> StateHandle -> IO (Maybe StateHandle)
eval prog state = do
  ptr <- primIO (prim__eval (programHandlePtr prog) (stateHandlePtr state))
  pure (createStateHandle ptr)

||| Check if program terminates on given state
||| Returns 1 if terminates, 0 if not
export
%foreign "C:az_terminates, libabsolute_zero"
prim__terminates : Bits64 -> Bits64 -> PrimIO Bits32

||| Safe termination check
export
terminates : ProgramHandle -> StateHandle -> IO Bool
terminates prog state = do
  result <- primIO (prim__terminates (programHandlePtr prog) (stateHandlePtr state))
  pure (result /= 0)

--------------------------------------------------------------------------------
-- CNO Verification
--------------------------------------------------------------------------------

||| Verify if a program is a CNO
||| Returns CNOVerificationResult struct (by value)
export
%foreign "C:az_verify_cno, libabsolute_zero"
prim__verifyCNO : Bits64 -> Bits64 -> PrimIO Bits64

||| Safe CNO verification
||| Returns verification result packed in Bits64
export
verifyCNO : ProgramHandle -> StateHandle -> IO CNOVerificationResult
verifyCNO prog state = do
  result <- primIO (prim__verifyCNO (programHandlePtr prog) (stateHandlePtr state))
  pure (unpackResult result)
  where
    -- Unpack 5 booleans from Bits64 (uses first 5 bytes)
    unpackResult : Bits64 -> CNOVerificationResult
    unpackResult packed = MkCNOResult
      { is_cno = ((packed `and` 0x01) /= 0)
      , terminates = ((packed `and` 0x02) /= 0)
      , preserves_state = ((packed `and` 0x04) /= 0)
      , is_pure = ((packed `and` 0x08) /= 0)
      , is_reversible = ((packed `and` 0x10) /= 0)
      }

||| Check if program is a CNO (simplified interface)
||| Returns 1 if CNO, 0 if not
export
%foreign "C:az_is_cno, libabsolute_zero"
prim__isCNO : Bits64 -> Bits64 -> PrimIO Bits32

||| Safe CNO check
export
isCNO : ProgramHandle -> StateHandle -> IO Bool
isCNO prog state = do
  result <- primIO (prim__isCNO (programHandlePtr prog) (stateHandlePtr state))
  pure (result /= 0)

--------------------------------------------------------------------------------
-- Error Handling
--------------------------------------------------------------------------------

||| Get last error message (C string pointer)
export
%foreign "C:az_last_error, libabsolute_zero"
prim__lastError : PrimIO Bits64

||| Get C string content
export
%foreign "support:idris2_getString, libidris2_support"
prim__getString : Bits64 -> String

||| Retrieve last error as string
export
lastError : IO (Maybe String)
lastError = do
  ptr <- primIO prim__lastError
  if ptr == 0
    then pure Nothing
    else pure (Just (prim__getString ptr))

||| Get error description for result code
export
errorDescription : Result -> String
errorDescription Ok = "Success"
errorDescription Error = "Generic error"
errorDescription InvalidParam = "Invalid parameter"
errorDescription OutOfMemory = "Out of memory"
errorDescription NullPointer = "Null pointer"
errorDescription NonTerminating = "Program does not terminate"
errorDescription HasSideEffects = "Program has side effects"
errorDescription NotCNO = "Program is not a CNO"

--------------------------------------------------------------------------------
-- Version Information
--------------------------------------------------------------------------------

||| Get library version string
export
%foreign "C:az_version, libabsolute_zero"
prim__version : PrimIO Bits64

||| Get version as string
export
version : IO String
version = do
  ptr <- primIO prim__version
  pure (prim__getString ptr)

||| Get ABI version numbers
export
%foreign "C:az_abi_version, libabsolute_zero"
prim__abiVersion : PrimIO Bits64

||| Get ABI version (major.minor.patch packed in Bits64)
export
abiVersion : IO (Bits32, Bits32, Bits32)
abiVersion = do
  packed <- primIO prim__abiVersion
  let major = cast (packed `shiftR` 32)
  let minor = cast ((packed `shiftR` 16) `and` 0xFFFF)
  let patch = cast (packed `and` 0xFFFF)
  pure (major, minor, patch)

--------------------------------------------------------------------------------
-- Utility Functions
--------------------------------------------------------------------------------

||| Check if library is initialized
export
%foreign "C:az_is_initialized, libabsolute_zero"
prim__isInitialized : Bits64 -> PrimIO Bits32

||| Check initialization status
export
isInitialized : StateHandle -> IO Bool
isInitialized h = do
  result <- primIO (prim__isInitialized (stateHandlePtr h))
  pure (result /= 0)

||| Get library build information
export
%foreign "C:az_build_info, libabsolute_zero"
prim__buildInfo : PrimIO Bits64

||| Get build information string
export
buildInfo : IO String
buildInfo = do
  ptr <- primIO prim__buildInfo
  pure (prim__getString ptr)
