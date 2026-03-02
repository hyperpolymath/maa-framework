/*
 * futex_nop.c - Futex Wait/Wake CNO Example
 *
 * CONCEPT:
 * Uses Linux futex (fast userspace mutex) to wait and wake with no actual
 * contention. Wake immediately follows wait, creating a CNO that exercises
 * the kernel's futex infrastructure without achieving synchronization.
 *
 * KERNEL MODE TRANSITION OVERHEAD:
 * --------------------------------
 * 1. FUTEX_WAIT syscall:
 *    - Syscall entry overhead (~100-200 cycles)
 *    - Futex hash table lookup
 *    - Waitqueue operations
 *    - (Immediate wake) Return path
 *
 * 2. FUTEX_WAKE syscall:
 *    - Syscall entry overhead
 *    - Futex hash table lookup
 *    - Waitqueue examination
 *    - Wake operations (even if no waiters)
 *    - Return path
 *
 * CONTEXT SWITCH IMPLICATIONS:
 * ---------------------------
 * NORMAL FUTEX USAGE:
 *   - WAIT: May block, context switch to other task
 *   - WAKE: Unblock waiter, may cause context switch back
 *
 * CNO CASE (no contention):
 *   - WAIT: Immediate wake means no blocking
 *   - No context switch occurs
 *   - All futex infrastructure exercised for nothing
 *
 * ALTERNATIVE CNO: FUTEX_WAIT with timeout=0
 *   - Enters kernel
 *   - Checks futex value
 *   - Timeout=0 means immediate return
 *   - Pure overhead, guaranteed CNO
 *
 * TLB FLUSHES:
 * -----------
 * - No context switch = no TLB flush (CNO case)
 * - Futex memory must be accessible (TLB hit/miss)
 * - Kernel's futex hash table access (kernel TLB)
 * - If blocking occurred: potential TLB flush on wake
 *
 * SCHEDULER IMPACT:
 * ----------------
 * CNO case (no actual wait):
 *   - No scheduler interaction
 *   - No runqueue modification
 *   - No priority adjustments
 *
 * Blocking case (non-CNO):
 *   - Task removed from runqueue
 *   - Added to futex waitqueue
 *   - Scheduler picks next task
 *   - On wake: task re-added to runqueue
 *
 * SECURITY CHECKS DESPITE CNO:
 * ---------------------------
 * 1. Syscall number validation
 * 2. Futex address validation (user memory accessible?)
 * 3. Futex operation validation (valid op code?)
 * 4. Futex permissions (shared futex requires special handling)
 * 5. RLIMIT_RTPRIO enforcement (for PI futexes)
 * 6. Namespace checks (PID namespace for FUTEX_WAIT)
 * 7. Audit trail (if syscall auditing enabled)
 *
 * PLATFORM COMPATIBILITY:
 * ----------------------
 * Linux:   Native futex() syscall - full featured
 * FreeBSD: _umtx_op() - similar primitive, different API
 * macOS:   No direct futex - use __ulock_wait/__ulock_wake
 * Windows: WaitOnAddress/WakeByAddress - similar concept
 *
 * Note: This example is Linux-specific as futex is a Linux primitive.
 * Other platforms would need different synchronization primitives.
 *
 * MEASUREMENT:
 * -----------
 * Futex CNO overhead: ~200-600ns per wait/wake pair
 * Includes: hash lookup, waitqueue ops, syscall overhead
 * Compare to: Actual contended futex: 1-10μs (with blocking)
 */

#include <stdio.h>
#include <time.h>
#include <stdint.h>
#include <errno.h>
#include <string.h>

#ifdef __linux__
#include <linux/futex.h>
#include <sys/syscall.h>
#include <unistd.h>

/*
 * Wrapper for futex syscall
 * Linux-specific fast userspace mutex primitive
 */
static long futex(uint32_t *uaddr, int futex_op, uint32_t val,
                  const struct timespec *timeout, uint32_t *uaddr2,
                  uint32_t val3) {
    return syscall(SYS_futex, uaddr, futex_op, val, timeout, uaddr2, val3);
}

/*
 * Pure futex CNO - wait with immediate wake
 * Exercises futex infrastructure for zero synchronization value
 */
void futex_cno_basic(void) {
    uint32_t futex_word = 0;

    // This is a CNO because:
    // 1. futex_word == 0 initially
    // 2. FUTEX_WAIT compares: if (*futex_word != 0) return immediately
    // 3. We pass val=1 (expected value)
    // 4. Mismatch detected, immediate return, no wait
    // 5. All kernel overhead, zero useful work

    futex(&futex_word, FUTEX_WAIT, 1, NULL, NULL, 0);
    // Returned immediately - CNO achieved
}

/*
 * Futex CNO with timeout=0
 * Guaranteed immediate return, pure overhead
 */
void futex_cno_timeout_zero(void) {
    uint32_t futex_word = 0;
    struct timespec timeout = {0, 0};

    // Timeout=0 guarantees immediate return
    // Futex infrastructure exercised, no wait occurs
    // Perfect CNO
    futex(&futex_word, FUTEX_WAIT, 0, &timeout, NULL, 0);
}

/*
 * Futex wake CNO - wake when no waiters exist
 * Kernel examines waitqueue, finds nothing, returns
 */
void futex_cno_wake_empty(void) {
    uint32_t futex_word = 0;

    // Wake up to 1 waiter... but there are no waiters
    // Kernel still:
    // - Computes futex hash
    // - Locks hash bucket
    // - Examines waitqueue
    // - Finds no waiters
    // - Unlocks and returns
    // Pure overhead for CNO
    futex(&futex_word, FUTEX_WAKE, 1, NULL, NULL, 0);
}

/*
 * Benchmark: Measure futex CNO overhead
 */
void benchmark_futex_wait_cno(long iterations) {
    uint32_t futex_word = 0;
    struct timespec start, end;
    struct timespec timeout = {0, 0};
    long i;

    clock_gettime(CLOCK_MONOTONIC, &start);

    for (i = 0; i < iterations; i++) {
        // Immediate return - CNO
        futex(&futex_word, FUTEX_WAIT, 0, &timeout, NULL, 0);
    }

    clock_gettime(CLOCK_MONOTONIC, &end);

    long long ns_elapsed = (end.tv_sec - start.tv_sec) * 1000000000LL +
                           (end.tv_nsec - start.tv_nsec);

    printf("Futex WAIT CNO Benchmark:\n");
    printf("  Iterations: %ld\n", iterations);
    printf("  Total time: %lld ns (%.3f ms)\n", ns_elapsed, ns_elapsed / 1e6);
    printf("  Per futex: %.2f ns\n", (double)ns_elapsed / iterations);
    printf("  Overhead includes:\n");
    printf("    - Syscall entry/exit\n");
    printf("    - Futex hash computation\n");
    printf("    - Waitqueue examination\n");
    printf("    - Timeout check (immediate)\n");
    printf("    - No actual wait - CNO!\n");
}

/*
 * Benchmark: Measure futex wake CNO overhead
 */
void benchmark_futex_wake_cno(long iterations) {
    uint32_t futex_word = 0;
    struct timespec start, end;
    long i;

    clock_gettime(CLOCK_MONOTONIC, &start);

    for (i = 0; i < iterations; i++) {
        // Wake nobody - CNO
        futex(&futex_word, FUTEX_WAKE, 1, NULL, NULL, 0);
    }

    clock_gettime(CLOCK_MONOTONIC, &end);

    long long ns_elapsed = (end.tv_sec - start.tv_sec) * 1000000000LL +
                           (end.tv_nsec - start.tv_nsec);

    printf("\nFutex WAKE CNO Benchmark:\n");
    printf("  Iterations: %ld\n", iterations);
    printf("  Total time: %lld ns (%.3f ms)\n", ns_elapsed, ns_elapsed / 1e6);
    printf("  Per futex: %.2f ns\n", (double)ns_elapsed / iterations);
    printf("  Overhead includes:\n");
    printf("    - Syscall entry/exit\n");
    printf("    - Futex hash computation\n");
    printf("    - Hash bucket lock\n");
    printf("    - Waitqueue scan (empty)\n");
    printf("    - No waiters to wake - CNO!\n");
}

/*
 * Analysis: Futex internals during CNO
 *
 * FUTEX_WAIT with immediate return:
 *
 * 1. SYSCALL ENTRY:
 *    - sys_futex() in kernel/futex.c
 *    - Validate futex operation
 *    - Validate user address
 *
 * 2. HASH COMPUTATION:
 *    - Compute futex hash: hash_futex(&key)
 *    - Hash based on: address + address space
 *    - Private futex: current->mm
 *    - Shared futex: inode + offset
 *
 * 3. VALUE CHECK:
 *    - get_futex_value_locked(&curval, uaddr)
 *    - Compare curval with expected value
 *    - Mismatch: return -EAGAIN (CNO case!)
 *    - Match: proceed to wait (non-CNO)
 *
 * 4. CNO PATH:
 *    - Value mismatch detected
 *    - No waitqueue operations
 *    - Return immediately to user space
 *    - All setup overhead, zero wait
 *
 * FUTEX_WAKE with no waiters:
 *
 * 1. SYSCALL ENTRY:
 *    - sys_futex() with FUTEX_WAKE
 *    - Validate operation and address
 *
 * 2. HASH BUCKET LOCK:
 *    - Compute futex hash
 *    - Acquire hash bucket spinlock
 *
 * 3. WAITQUEUE SCAN:
 *    - Scan waitqueue for waiters
 *    - Find none (CNO case)
 *    - No wake operations needed
 *
 * 4. CLEANUP:
 *    - Release hash bucket lock
 *    - Return 0 (no waiters woken)
 *    - All overhead, zero synchronization
 */

int main(void) {
    printf("=== OS-Level CNO: Futex No-Operation ===\n\n");

    printf("Concept: Use futex primitives with no actual contention,\n");
    printf("         exercising kernel synchronization infrastructure\n");
    printf("         while achieving zero synchronization value.\n\n");

    printf("Note: This example is Linux-specific (futex is a Linux primitive)\n\n");

    // Demonstrate basic futex CNO
    printf("Executing futex WAIT CNO (value mismatch)...\n");
    futex_cno_basic();
    printf("  ✓ Futex hash computed\n");
    printf("  ✓ Value check performed\n");
    printf("  ✓ Immediate return - CNO achieved\n\n");

    // Demonstrate timeout=0 CNO
    printf("Executing futex WAIT CNO (timeout=0)...\n");
    futex_cno_timeout_zero();
    printf("  ✓ Zero timeout specified\n");
    printf("  ✓ No blocking possible\n");
    printf("  ✓ Pure overhead - CNO achieved\n\n");

    // Demonstrate wake CNO
    printf("Executing futex WAKE CNO (no waiters)...\n");
    futex_cno_wake_empty();
    printf("  ✓ Waitqueue examined\n");
    printf("  ✓ No waiters found\n");
    printf("  ✓ No wake performed - CNO achieved\n\n");

    // Run benchmarks
    printf("Measuring overhead...\n");
    benchmark_futex_wait_cno(100000);
    benchmark_futex_wake_cno(100000);
    printf("\n");

    printf("Futex Details:\n");
    printf("  Futex hash table: Kernel-global structure\n");
    printf("  Hash buckets: Typically 256 (configurable)\n");
    printf("  Waitqueue: Per hash bucket\n");
    printf("  Locking: Spinlock per hash bucket\n");
    printf("  CNO achieved: Maximum futex overhead, zero synchronization\n");

    return 0;
}

#else
/* Non-Linux platform */
int main(void) {
    printf("=== OS-Level CNO: Futex No-Operation ===\n\n");
    printf("ERROR: This example requires Linux (futex is Linux-specific)\n\n");
    printf("Alternative primitives on other platforms:\n");
    printf("  FreeBSD: _umtx_op() - userspace mutex operations\n");
    printf("  macOS:   __ulock_wait/__ulock_wake - userspace locks\n");
    printf("  Windows: WaitOnAddress/WakeByAddress - similar concept\n\n");
    printf("The CNO concept applies to all these primitives:\n");
    printf("  - Wait with no contention = immediate return\n");
    printf("  - Wake with no waiters = no wake performed\n");
    printf("  - All kernel overhead, zero synchronization value\n");

    return 1;
}
#endif

/*
 * DEEPER DIVE: Why Futex CNO Matters
 * ==================================
 *
 * 1. FUTEX DESIGN PHILOSOPHY:
 *    - "Fast" in uncontended case (userspace only)
 *    - Kernel involved only when contention exists
 *    - CNO explores the boundary: kernel called, but no contention
 *
 * 2. OVERHEAD BREAKDOWN:
 *    Component              | Time (ns) | CNO Impact
 *    -----------------------|-----------|------------
 *    Syscall entry/exit     | 100-200   | Always paid
 *    Futex hash computation | 20-50     | Always paid
 *    Hash bucket lock       | 10-30     | Paid for WAKE
 *    Value check            | 10-20     | Paid for WAIT
 *    Waitqueue operations   | 50-200    | Skipped in CNO!
 *    Context switch         | 1000-5000 | Skipped in CNO!
 *    -----------------------|-----------|------------
 *    Total CNO              | 150-400   | Pure overhead
 *    Total contended        | 1000-5000 | Actual work
 *
 * 3. SECURITY IMPLICATIONS:
 *    - Futex operations are syscalls (attack surface)
 *    - Hash table collisions can cause contention
 *    - DoS: Excessive futex calls (even CNO) consume CPU
 *    - Side-channel: Futex timing reveals synchronization
 *
 * 4. REAL-WORLD USAGE:
 *    - pthread mutexes use futex internally
 *    - Condition variables use futex
 *    - Many userspace locks built on futex
 *    - Understanding CNO helps optimize these primitives
 *
 * 5. PERFORMANCE OPTIMIZATION:
 *    - Uncontended path: userspace only (no syscall)
 *    - Light contention: futex CNO overhead
 *    - Heavy contention: full futex blocking
 *    - Goal: Keep contention low to avoid non-CNO overhead
 *
 * 6. COMPARISON WITH OTHER PRIMITIVES:
 *    - Mutex: Higher-level, may use futex internally
 *    - Semaphore: Can also use futex
 *    - Spinlock: Userspace only, no syscall (but wastes CPU)
 *    - Futex: Best of both worlds when used correctly
 */
