; CPU-Level Certified Null Operation
; x86-64 Assembly (Intel syntax)
;
; The NOP instruction at the CPU level - the hardware implementation
; of "doing nothing"
;
; Author: Jonathan D. A. Jewell
; Project: Absolute Zero

section .text
global _start

_start:
    ; Method 1: Single-byte NOP
    nop                     ; 0x90 - canonical one-byte NOP

    ; Method 2: Multi-byte NOPs (for alignment)
    ; Modern CPUs support NOPs of varying lengths
    nop                     ; 1-byte:  90
    xchg ax, ax             ; 2-byte:  66 90
    nop dword [eax]         ; 3-byte:  0F 1F 00
    nop dword [eax + 0]     ; 4-byte:  0F 1F 40 00
    nop dword [eax + eax]   ; 5-byte:  0F 1F 44 00 00

    ; Method 3: Functional NOPs (operations with no net effect)
    mov rax, rax            ; Move register to itself
    add rax, 0              ; Add zero
    sub rax, 0              ; Subtract zero
    xor rcx, rcx            ; XOR register with itself (sets to 0)
    xor rcx, rcx            ; Do it again - still 0

    ; Method 4: Stack-balanced NOPs
    push rax                ; Save
    pop rax                 ; Restore

    ; Method 5: Flag-preserving NOPs
    lahf                    ; Load flags into AH
    sahf                    ; Store AH back to flags

    ; Exit cleanly (not a NOP, but necessary)
    mov rax, 60             ; sys_exit
    xor rdi, rdi            ; exit code 0
    syscall

; ============================================================================
; CPU Microarchitecture Perspective
; ============================================================================
;
; What happens when NOP executes?
;
; 1. FETCH: Instruction fetched from L1I cache
; 2. DECODE: Decoded to micro-ops (μops)
; 3. RENAME: Register renaming (if applicable)
; 4. SCHEDULE: Added to reservation station
; 5. EXECUTE: "Executed" (but does nothing)
; 6. RETIRE: Retired from reorder buffer
;
; Performance Impact:
; - Takes 1 cycle on most modern CPUs
; - Can be macro-fused with other instructions
; - Used for code alignment (improving fetch efficiency)
; - Branch prediction not affected
;
; ============================================================================
; Hardware Implementation
; ============================================================================
;
; In the CPU itself, NOP might:
; - Still allocate a reorder buffer entry
; - Still consume pipeline bandwidth
; - Still update instruction pointer
; - Still consume power (transistor switching)
;
; But logically: changes NO architectural state
;
; ============================================================================
; Speculation and Out-of-Order Execution
; ============================================================================
;
; Even though NOP "does nothing", the CPU may:
; - Speculatively execute it
; - Reorder it with other instructions
; - Eliminate it entirely (at μop level)
;
; This demonstrates: CNO at architectural level ≠ CNO at microarchitectural level
;
; ============================================================================
; Multi-byte NOP Encoding
; ============================================================================
;
; Long NOPs were introduced for code alignment without wasting space.
; Intel recommends specific multi-byte sequences for best performance.
;
; 1-byte:  NOP                    (0x90)
; 2-byte:  66 NOP                 (operand-size prefix + NOP)
; 3-byte:  0F 1F 00               (NOP with ModR/M)
; 4-byte:  0F 1F 40 00            (NOP with SIB)
; 5-byte:  0F 1F 44 00 00         (NOP with SIB + disp8)
; 6-byte:  66 0F 1F 44 00 00      (prefix + 5-byte NOP)
; 7-byte:  0F 1F 80 00 00 00 00   (NOP with disp32)
; 8-byte:  0F 1F 84 00 00 00 00 00
; 9-byte:  66 0F 1F 84 00 00 00 00 00
;
; ============================================================================
; Build & Verify
; ============================================================================
;
; Assemble:
;   nasm -f elf64 nop.asm -o nop.o
;   ld nop.o -o nop
;
; Run:
;   ./nop
;   echo $?  # Should print 0
;
; Disassemble:
;   objdump -d nop
;
; Verify no output:
;   ./nop > output.txt 2>&1
;   wc -c output.txt  # Should be 0 bytes
;
; ============================================================================
; Theoretical Significance
; ============================================================================
;
; The NOP instruction is the purest CNO at the CPU level:
; - Defined in ISA specification
; - Guaranteed to change no architectural state
; - Universal across x86 family
; - Timing is implementation-dependent but predictable
;
; It proves that "doing nothing" is a first-class operation in hardware.
