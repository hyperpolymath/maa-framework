/*
 * syscall_nop.c - System Call CNO Example
 *
 * CONCEPT:
 * Demonstrates a system call that transitions to kernel mode, performs
 * a legitimate operation (getpid), but immediately discards the result,
 * achieving computational no-operation despite significant overhead.
 *
 * KERNEL MODE TRANSITION OVERHEAD:
 * --------------------------------
 * 1. User mode → Kernel mode transition (~50-200 CPU cycles)
 *    - Save user-space registers
 *    - Switch to kernel stack
 *    - Load kernel page tables
 *    - Execute system call handler
 *
 * 2. Kernel mode → User mode transition (~50-200 CPU cycles)
 *    - Restore user-space registers
 *    - Switch back to user stack
 *    - Return to user-space page tables
 *
 * CONTEXT SWITCH IMPLICATIONS:
 * ---------------------------
 * - No actual context switch occurs (same thread continues)
 * - However, CPU mode switch happens (user → kernel → user)
 * - Processor state is saved/restored across privilege boundaries
 * - Kernel stack is activated for duration of syscall
 *
 * TLB FLUSHES:
 * -----------
 * - Modern CPUs (x86-64 with PCID): Minimal TLB impact
 * - Older architectures: Potential partial TLB flush
 * - Page table entries for kernel must be accessible
 * - TLB entries for user pages remain mostly intact
 *
 * SCHEDULER IMPACT:
 * ----------------
 * - Syscall handler checks for pending signals
 * - Scheduler may preempt if higher priority task is ready
 * - Time accounting: syscall time charged to process
 * - No voluntary yield, but preemption point exists
 *
 * SECURITY CHECKS DESPITE CNO:
 * ---------------------------
 * 1. Syscall number validation (is getpid valid?)
 * 2. Capability checks (can this process query its PID?)
 * 3. Namespace isolation (PID namespace awareness)
 * 4. Audit logging (if enabled - syscall traced)
 * 5. Seccomp filters (if active - syscall policy checked)
 * 6. SELinux/AppArmor hooks (mandatory access control)
 *
 * PLATFORM COMPATIBILITY:
 * ----------------------
 * Linux:   syscall(__NR_getpid) - syscall number varies by arch
 * BSD:     getpid() - direct syscall wrapper
 * macOS:   getpid() - BSD-like syscall interface
 *
 * MEASUREMENT:
 * -----------
 * Typical overhead: 50-300ns per call on modern hardware
 * Includes: mode switch, security checks, syscall dispatch
 * Compare to: function call (~1-5ns), pure computation (~0.3ns/op)
 */

#include <stdio.h>
#include <unistd.h>
#include <time.h>
#include <sys/types.h>

#ifdef __linux__
#include <sys/syscall.h>
#endif

/*
 * Pure syscall CNO - result discarded immediately
 * Demonstrates maximum overhead for zero computational value
 */
void syscall_cno_basic(void) {
    // System call executed, result ignored
    // Kernel still performs full security check and PID lookup
    getpid();
}

/*
 * Explicit syscall using raw syscall() interface (Linux)
 * Shows the actual syscall mechanism without libc wrapper
 */
void syscall_cno_raw(void) {
#ifdef __linux__
    // Direct syscall - bypasses libc but not kernel overhead
    syscall(SYS_getpid);
#else
    // BSD/macOS use getpid() wrapper which is already thin
    getpid();
#endif
}

/*
 * Syscall with volatile sink to prevent compiler optimization
 * Ensures the call isn't optimized away
 */
void syscall_cno_volatile(void) {
    volatile pid_t sink;
    sink = getpid();
    // Value in 'sink' is never used - pure CNO
    (void)sink; // Suppress unused variable warning
}

/*
 * Benchmark: Measure syscall overhead
 * Quantifies the cost of kernel mode transition
 */
void benchmark_syscall_overhead(long iterations) {
    struct timespec start, end;
    long i;
    volatile pid_t sink;

    clock_gettime(CLOCK_MONOTONIC, &start);

    for (i = 0; i < iterations; i++) {
        sink = getpid();
    }

    clock_gettime(CLOCK_MONOTONIC, &end);

    long long ns_elapsed = (end.tv_sec - start.tv_sec) * 1000000000LL +
                           (end.tv_nsec - start.tv_nsec);

    printf("Syscall CNO Benchmark:\n");
    printf("  Iterations: %ld\n", iterations);
    printf("  Total time: %lld ns\n", ns_elapsed);
    printf("  Per syscall: %.2f ns\n", (double)ns_elapsed / iterations);
    printf("  Overhead includes:\n");
    printf("    - User→Kernel mode switch\n");
    printf("    - Security validation\n");
    printf("    - PID lookup in kernel\n");
    printf("    - Kernel→User mode switch\n");
    printf("    - Result ignored (CNO achieved)\n");
}

/*
 * Analysis: What actually happens in kernel
 *
 * getpid() syscall execution flow:
 *
 * 1. USER SPACE:
 *    - Program calls getpid()
 *    - Libc wrapper invokes syscall instruction
 *    - CPU switches to kernel mode (ring 0)
 *
 * 2. KERNEL ENTRY:
 *    - entry_SYSCALL_64 (x86-64 Linux)
 *    - Save user registers to kernel stack
 *    - Load kernel GS base (per-CPU data)
 *    - Check syscall number validity
 *
 * 3. SECURITY CHECKS:
 *    - Seccomp filter evaluation (if active)
 *    - Capability checks (CAP_SYS_ADMIN, etc.)
 *    - Namespace isolation (PID namespace)
 *    - LSM hooks (SELinux, AppArmor)
 *
 * 4. SYSCALL EXECUTION:
 *    - sys_getpid() in kernel
 *    - Read current->tgid (thread group ID)
 *    - Translate PID if in namespace
 *    - Return value to syscall return path
 *
 * 5. KERNEL EXIT:
 *    - Check for pending signals
 *    - Check for reschedule flag
 *    - Restore user registers
 *    - Switch back to user mode (ring 3)
 *
 * 6. USER SPACE:
 *    - getpid() returns with PID value
 *    - Our code immediately discards it (CNO!)
 */

int main(void) {
    printf("=== OS-Level CNO: System Call No-Operation ===\n\n");

    printf("Concept: Execute system calls that transition to kernel mode\n");
    printf("         but produce no computational effect.\n\n");

    // Demonstrate basic syscall CNO
    printf("Executing syscall CNO (basic)...\n");
    syscall_cno_basic();
    printf("  ✓ Kernel mode transition occurred\n");
    printf("  ✓ Security checks performed\n");
    printf("  ✓ Result discarded - CNO achieved\n\n");

    // Demonstrate raw syscall
    printf("Executing syscall CNO (raw syscall)...\n");
    syscall_cno_raw();
    printf("  ✓ Direct syscall interface used\n");
    printf("  ✓ Full kernel overhead incurred\n");
    printf("  ✓ Zero computational value delivered\n\n");

    // Run benchmark
    printf("Measuring overhead...\n");
    benchmark_syscall_overhead(100000);
    printf("\n");

    printf("Platform Details:\n");
#ifdef __linux__
    printf("  OS: Linux\n");
    printf("  Syscall mechanism: syscall instruction (x86-64)\n");
    printf("  Syscall number: %ld (SYS_getpid)\n", (long)SYS_getpid);
#elif defined(__APPLE__)
    printf("  OS: macOS\n");
    printf("  Syscall mechanism: BSD-style syscall\n");
#elif defined(__FreeBSD__)
    printf("  OS: FreeBSD\n");
    printf("  Syscall mechanism: BSD syscall\n");
#else
    printf("  OS: Unknown POSIX\n");
#endif
    printf("  CNO achieved: System resources consumed, no computation performed\n");

    return 0;
}

/*
 * DEEPER DIVE: Why This Is CNO
 * ============================
 *
 * 1. WORK PERFORMED:
 *    - CPU mode switch (2x per call)
 *    - Security validation
 *    - Kernel data structure access
 *    - Register save/restore
 *
 * 2. RESULT ACHIEVED:
 *    - PID value obtained... then ignored
 *    - No state change in program
 *    - No observable effect
 *    - Pure overhead
 *
 * 3. SECURITY IMPLICATIONS:
 *    - Every syscall is a potential vulnerability
 *    - Security checks run even for CNO
 *    - Audit logs generated (if enabled)
 *    - Syscall counts tracked by kernel
 *
 * 4. PERFORMANCE COST:
 *    - ~100-300ns per call (typical)
 *    - Compare to: function call ~1-5ns
 *    - Ratio: 20-300x overhead for CNO
 *
 * 5. USE CASES:
 *    - Timing attacks (syscall as timer)
 *    - Side-channel analysis
 *    - Performance testing
 *    - Educational demonstration
 */
