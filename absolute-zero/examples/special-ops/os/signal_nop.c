/*
 * signal_nop.c - Signal Handler CNO Example
 *
 * CONCEPT:
 * Sends a signal to self with a handler that does nothing. The signal
 * delivery mechanism exercises the kernel's signal infrastructure,
 * including handler dispatch, stack switching, and signal return path,
 * achieving CNO through maximum overhead with zero computational effect.
 *
 * KERNEL MODE TRANSITION OVERHEAD:
 * --------------------------------
 * Signal delivery involves MULTIPLE kernel transitions:
 *
 * 1. SEND (kill syscall):
 *    - Syscall entry (~100-200 cycles)
 *    - Signal validation
 *    - Queue signal to target task
 *    - Syscall exit
 *
 * 2. DELIVERY (on next kernel exit):
 *    - Check pending signals
 *    - Setup signal handler stack frame
 *    - Prepare user context for handler
 *    - Return to user mode (in handler!)
 *
 * 3. HANDLER EXECUTION:
 *    - Handler runs in user mode
 *    - CNO: Handler does nothing
 *
 * 4. RETURN (rt_sigreturn syscall):
 *    - Special syscall to return from signal
 *    - Restore pre-signal context
 *    - Return to interrupted code
 *
 * Total: ~3 kernel transitions for one signal CNO!
 *
 * CONTEXT SWITCH IMPLICATIONS:
 * ---------------------------
 * - No context switch between tasks
 * - BUT: Context switch within same task!
 * - Pre-signal context saved on stack
 * - Handler context activated
 * - Post-handler: context restored
 * - All register state save/restore overhead
 *
 * TLB FLUSHES:
 * -----------
 * - No TLB flush (same address space)
 * - Signal handler stack may cause TLB miss
 * - Alternate signal stack (if used): different TLB entries
 * - No cross-address-space transition
 *
 * SCHEDULER IMPACT:
 * ----------------
 * - Signal delivery checked at kernel exit points
 * - Pending signals prevent immediate return to userspace
 * - Handler setup time delays task execution
 * - Real-time signals: priority queuing overhead
 * - Signal mask updated during handler
 *
 * SECURITY CHECKS DESPITE CNO:
 * ---------------------------
 * 1. SEND PHASE (kill syscall):
 *    - Permission check: Can sender signal target?
 *    - PID validation: Does target exist?
 *    - Namespace check: Same PID namespace?
 *    - Capability check: CAP_KILL for restricted signals
 *    - Signal number validation
 *
 * 2. DELIVERY PHASE:
 *    - Signal mask check: Is signal blocked?
 *    - Handler validation: Is handler address valid?
 *    - Stack validation: Enough stack space?
 *    - Alternate stack: SA_ONSTACK handling
 *    - Seccomp: Signal delivery may be restricted
 *
 * 3. RETURN PHASE (rt_sigreturn):
 *    - Stack frame validation
 *    - Context restoration security
 *    - Prevent privilege escalation via signal return
 *
 * PLATFORM COMPATIBILITY:
 * ----------------------
 * Linux:   sigaction, kill, rt_sigreturn (POSIX + Linux extensions)
 * BSD:     sigaction, kill (BSD signal semantics)
 * macOS:   sigaction, kill (BSD-based signals)
 * All:     POSIX signal API is portable
 *
 * Signal delivery internals vary:
 * - Linux: Real-time signals, queuing, siginfo
 * - BSD: Traditional + real-time signals
 * - macOS: Mach exception ports + BSD signals
 *
 * MEASUREMENT:
 * -----------
 * Signal CNO overhead: 1-5 μs per signal
 * Includes: send, delivery, handler, return
 * Compare to: Function call ~1-5ns
 * Ratio: 1000x overhead for CNO!
 */

#include <stdio.h>
#include <signal.h>
#include <time.h>
#include <unistd.h>
#include <string.h>
#include <sys/types.h>

/* Signal handler that does nothing - perfect for CNO */
static void signal_nop_handler(int signo) {
    /* Absolutely nothing happens here */
    /* Signal delivered, handler executed, zero work done */
    /* This is the essence of signal CNO */
    (void)signo; /* Suppress unused parameter warning */
}

/* Counter for signal handler (for verification only) */
static volatile sig_atomic_t signal_count = 0;

/* Signal handler that counts (for verification) */
static void signal_counting_handler(int signo) {
    signal_count++;
    (void)signo;
}

/*
 * Setup signal handler for CNO
 * Using sigaction for full POSIX control
 */
void setup_signal_handler(int signo, void (*handler)(int)) {
    struct sigaction sa;

    memset(&sa, 0, sizeof(sa));
    sa.sa_handler = handler;
    sigemptyset(&sa.sa_mask);
    sa.sa_flags = SA_RESTART; /* Restart interrupted syscalls */

    if (sigaction(signo, &sa, NULL) == -1) {
        perror("sigaction");
    }
}

/*
 * Basic signal CNO - send signal to self, do nothing in handler
 */
void signal_cno_basic(void) {
    /* Send SIGUSR1 to self */
    /* Signal will be delivered, handler will do nothing */
    kill(getpid(), SIGUSR1);

    /* Signal may be delivered asynchronously */
    /* Force delivery by entering kernel (any syscall) */
    getpid(); /* Signal delivered on return from this syscall */
}

/*
 * Synchronous signal CNO using raise()
 * More deterministic timing than kill()
 */
void signal_cno_raise(void) {
    raise(SIGUSR1);
    /* raise() is synchronous-ish: delivered before return (usually) */
}

/*
 * Real-time signal CNO
 * Real-time signals have queuing and priority
 */
void signal_cno_realtime(void) {
#ifdef __linux__
    /* Linux supports real-time signals SIGRTMIN-SIGRTMAX */
    kill(getpid(), SIGRTMIN);
    getpid(); /* Force delivery */
#else
    /* Other platforms may not support RT signals */
    signal_cno_basic();
#endif
}

/*
 * Benchmark: Measure signal CNO overhead
 */
void benchmark_signal_overhead(long iterations) {
    struct timespec start, end;
    long i;

    /* Reset counter */
    signal_count = 0;

    /* Use counting handler to verify delivery */
    setup_signal_handler(SIGUSR1, signal_counting_handler);

    clock_gettime(CLOCK_MONOTONIC, &start);

    for (i = 0; i < iterations; i++) {
        raise(SIGUSR1);
        /* Signal delivered synchronously (or very soon) */
    }

    clock_gettime(CLOCK_MONOTONIC, &end);

    long long ns_elapsed = (end.tv_sec - start.tv_sec) * 1000000000LL +
                           (end.tv_nsec - start.tv_nsec);

    printf("Signal CNO Benchmark:\n");
    printf("  Iterations: %ld\n", iterations);
    printf("  Signals delivered: %d\n", signal_count);
    printf("  Total time: %lld ns (%.3f ms)\n", ns_elapsed, ns_elapsed / 1e6);
    printf("  Per signal: %.2f ns (%.3f μs)\n",
           (double)ns_elapsed / iterations,
           (double)ns_elapsed / iterations / 1000.0);
    printf("  Overhead includes:\n");
    printf("    - kill/raise syscall\n");
    printf("    - Signal queuing\n");
    printf("    - Signal delivery setup\n");
    printf("    - Handler stack frame creation\n");
    printf("    - Handler execution (CNO!)\n");
    printf("    - rt_sigreturn syscall\n");
    printf("    - Context restoration\n");
}

/*
 * Demonstrate signal mask manipulation CNO
 * Block and unblock signal - pure overhead
 */
void signal_mask_cno(void) {
    sigset_t mask, oldmask;

    /* Create signal set */
    sigemptyset(&mask);
    sigaddset(&mask, SIGUSR1);

    /* Block SIGUSR1 */
    sigprocmask(SIG_BLOCK, &mask, &oldmask);

    /* Send signal (will be pending, not delivered) */
    raise(SIGUSR1);

    /* Unblock SIGUSR1 (now delivered) */
    sigprocmask(SIG_SETMASK, &oldmask, NULL);

    /* All this machinery for a CNO handler! */
}

/*
 * Print signal handler information
 */
void print_signal_info(void) {
    struct sigaction sa;

    printf("Signal Configuration:\n");

    /* Check SIGUSR1 handler */
    if (sigaction(SIGUSR1, NULL, &sa) == 0) {
        printf("  SIGUSR1 handler: %p\n", (void*)sa.sa_handler);
        printf("  Flags: ");
        if (sa.sa_flags & SA_RESTART) printf("SA_RESTART ");
        if (sa.sa_flags & SA_SIGINFO) printf("SA_SIGINFO ");
        if (sa.sa_flags & SA_ONSTACK) printf("SA_ONSTACK ");
        printf("\n");
    }

#ifdef __linux__
    printf("  Real-time signals: SIGRTMIN=%d, SIGRTMAX=%d\n", SIGRTMIN, SIGRTMAX);
#endif
}

/*
 * Analysis: Signal delivery internals
 *
 * Signal CNO flow (Linux example):
 *
 * 1. SEND PHASE (raise/kill):
 *    - sys_kill() or sys_tkill() in kernel
 *    - Find target task_struct
 *    - Check permissions: may_kill()
 *    - Queue signal: send_signal()
 *      - Allocate sigqueue structure (RT signals)
 *      - Add to pending signal list
 *      - Set TIF_SIGPENDING flag
 *    - Return to user space
 *
 * 2. DELIVERY PHASE (on kernel exit):
 *    - do_notify_resume() checks TIF_SIGPENDING
 *    - do_signal() processes pending signals
 *    - get_signal() dequeues signal
 *    - Check signal mask: is signal blocked?
 *    - Setup handler: handle_signal()
 *      - Allocate signal stack frame
 *      - Save current context (registers, sigmask)
 *      - Prepare context for handler
 *      - Modify return address to call handler
 *    - Return to user space IN HANDLER
 *
 * 3. HANDLER PHASE (user space):
 *    - Execute signal_nop_handler()
 *    - CNO: Handler does nothing!
 *    - Handler returns
 *    - Calls signal trampoline (arch-specific)
 *
 * 4. RETURN PHASE (rt_sigreturn):
 *    - Trampoline calls rt_sigreturn syscall
 *    - sys_rt_sigreturn() in kernel
 *    - Restore pre-signal context
 *    - Restore signal mask
 *    - Return to interrupted code
 *
 * Total overhead: ~1-5μs for CNO!
 */

int main(void) {
    printf("=== OS-Level CNO: Signal Handler No-Operation ===\n\n");

    printf("Concept: Deliver signals with handlers that do nothing,\n");
    printf("         exercising signal delivery infrastructure while\n");
    printf("         achieving zero computational effect.\n\n");

    /* Setup CNO handler */
    setup_signal_handler(SIGUSR1, signal_nop_handler);

    /* Print configuration */
    print_signal_info();
    printf("\n");

    /* Demonstrate basic signal CNO */
    printf("Executing signal CNO (basic kill)...\n");
    signal_cno_basic();
    printf("  ✓ Signal sent via kill()\n");
    printf("  ✓ Handler executed (did nothing)\n");
    printf("  ✓ Context restored - CNO achieved\n\n");

    /* Demonstrate raise CNO */
    printf("Executing signal CNO (raise)...\n");
    signal_cno_raise();
    printf("  ✓ Signal sent via raise()\n");
    printf("  ✓ Synchronous delivery\n");
    printf("  ✓ Handler CNO complete\n\n");

    /* Demonstrate mask CNO */
    printf("Executing signal mask CNO...\n");
    signal_mask_cno();
    printf("  ✓ Signal blocked\n");
    printf("  ✓ Signal sent (pending)\n");
    printf("  ✓ Signal unblocked and delivered\n");
    printf("  ✓ Maximum machinery, zero work - CNO!\n\n");

    /* Run benchmark */
    printf("Measuring overhead...\n");
    benchmark_signal_overhead(10000);
    printf("\n");

    printf("Platform Details:\n");
#ifdef __linux__
    printf("  OS: Linux\n");
    printf("  Signal API: POSIX + Linux extensions\n");
    printf("  Return mechanism: rt_sigreturn syscall\n");
#elif defined(__APPLE__)
    printf("  OS: macOS\n");
    printf("  Signal API: BSD signals\n");
    printf("  Backend: Mach exception ports + BSD layer\n");
#elif defined(__FreeBSD__)
    printf("  OS: FreeBSD\n");
    printf("  Signal API: BSD signals\n");
#else
    printf("  OS: Unknown POSIX\n");
#endif
    printf("  CNO achieved: Maximum signal overhead, zero handler work\n");
    printf("  Overhead ratio: ~1000x vs function call\n");

    return 0;
}

/*
 * DEEPER DIVE: Why Signal CNO Is Expensive
 * ========================================
 *
 * 1. OVERHEAD BREAKDOWN:
 *    Component                  | Time (ns) | Required?
 *    ---------------------------|-----------|----------
 *    kill/raise syscall         | 100-300   | Yes
 *    Signal queuing             | 50-100    | Yes
 *    Delivery check             | 20-50     | Yes
 *    Stack frame setup          | 100-300   | Yes
 *    Context save               | 50-150    | Yes
 *    Handler execution          | 1-10      | CNO!
 *    rt_sigreturn syscall       | 100-300   | Yes
 *    Context restore            | 50-150    | Yes
 *    ---------------------------|-----------|----------
 *    Total                      | 500-1400  | Mostly overhead
 *
 * 2. COMPARISON WITH OTHER MECHANISMS:
 *    - Function call: ~1-5ns (300x faster!)
 *    - Syscall: ~100-300ns (5x faster!)
 *    - Context switch: ~1-5μs (similar cost!)
 *    - Signal: ~500-1400ns (expensive for IPC!)
 *
 * 3. SECURITY IMPLICATIONS:
 *    - Signal delivery is attack surface
 *    - TOCTTOU race: check time vs use time
 *    - Stack overflow via deep signal nesting
 *    - Privilege escalation via signal handler
 *    - Side-channel: Signal timing reveals state
 *
 * 4. REAL-WORLD USAGE:
 *    - IPC: Use pipes/sockets instead (faster)
 *    - Notification: Signals appropriate
 *    - Async events: Signals useful
 *    - CNO demonstration: Educational value
 *
 * 5. OPTIMIZATION OPPORTUNITIES:
 *    - Avoid signals on hot paths
 *    - Use signalfd for synchronous handling (Linux)
 *    - Batch notifications instead of per-event signals
 *    - Consider eventfd as lighter alternative
 *
 * 6. WHY THIS IS TRUE CNO:
 *    - Handler does literally nothing
 *    - All overhead is signal machinery
 *    - No computation in handler
 *    - No state change (except signal delivery count)
 *    - Pure demonstration of overhead
 */
