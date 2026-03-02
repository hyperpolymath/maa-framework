/*
 * sched_yield.c - Scheduler Yield CNO Example
 *
 * CONCEPT:
 * Voluntarily yields the CPU to the scheduler, but if no other runnable
 * tasks exist, the same thread immediately resumes execution. This creates
 * a CNO when the system is idle - maximum scheduler overhead with zero
 * computational progress.
 *
 * KERNEL MODE TRANSITION OVERHEAD:
 * --------------------------------
 * 1. Syscall entry overhead (~50-200 cycles)
 * 2. Scheduler lock acquisition
 * 3. Runqueue examination
 * 4. Priority recalculation (CFS scheduler)
 * 5. Context switch attempt
 * 6. Syscall return overhead
 *
 * CONTEXT SWITCH IMPLICATIONS:
 * ---------------------------
 * CASE 1: Other tasks runnable
 *   - Full context switch occurs
 *   - Save current task state
 *   - Load next task state
 *   - TLB flush (if different address space)
 *   - Cache pollution
 *   - Later: switched back (another context switch)
 *
 * CASE 2: No other tasks runnable (CNO case)
 *   - Scheduler consulted
 *   - No context switch performed
 *   - Same task continues immediately
 *   - Pure overhead, no progress elsewhere
 *
 * TLB FLUSHES:
 * -----------
 * - If context switch occurs: TLB may be flushed
 * - Modern CPUs with tagged TLB (PCID): Selective flush
 * - Older architectures: Full TLB flush on switch
 * - No switch = no TLB flush (but still syscall overhead)
 *
 * SCHEDULER IMPACT:
 * ----------------
 * Linux CFS (Completely Fair Scheduler):
 * 1. Task placed at end of runqueue
 * 2. vruntime not incremented (voluntary yield)
 * 3. Scheduler examines runqueue for next task
 * 4. If alone: immediately rescheduled
 * 5. Scheduling statistics updated
 *
 * Real-time schedulers (SCHED_FIFO, SCHED_RR):
 * - SCHED_FIFO: Yields to same-priority tasks
 * - SCHED_RR: Yields and goes to end of round-robin queue
 * - CNO achieved if no same-priority tasks exist
 *
 * SECURITY CHECKS DESPITE CNO:
 * ---------------------------
 * 1. Syscall number validation
 * 2. RLIMIT_RTTIME enforcement (RT scheduling)
 * 3. CPU affinity mask checking
 * 4. cgroup CPU controller notifications
 * 5. Scheduler policy validation
 * 6. Preemption safety checks
 *
 * PLATFORM COMPATIBILITY:
 * ----------------------
 * Linux:   sched_yield() - POSIX + Linux-specific behavior
 * BSD:     sched_yield() - POSIX compliant, simpler scheduler
 * macOS:   sched_yield() - Mach kernel, different scheduler
 * Windows: SwitchToThread() - similar concept
 *
 * MEASUREMENT:
 * -----------
 * Overhead when alone: 500ns - 2μs (scheduler consult + syscall)
 * With context switch: 1μs - 10μs (full switch cost)
 * Includes: scheduler lock, runqueue scan, decision logic
 */

#include <stdio.h>
#include <sched.h>
#include <time.h>
#include <unistd.h>
#include <pthread.h>
#include <string.h>

#ifdef __linux__
#include <sys/syscall.h>
#include <linux/sched.h>
#endif

/*
 * Pure sched_yield CNO - likely no context switch
 * When system is idle, this is maximum overhead for zero progress
 */
void yield_cno_basic(void) {
    sched_yield();
    // If we're the only runnable task, we immediately continue
    // Scheduler was consulted, locks acquired, decision made: CNO
}

/*
 * Verify CNO condition - check if we're likely alone
 * Returns 1 if we're likely the only task (CNO condition)
 */
int check_cno_condition(void) {
    // Get number of online CPUs
    long num_cpus = sysconf(_SC_NPROCESSORS_ONLN);

    // Get load average
    double loadavg[3];
    if (getloadavg(loadavg, 3) == -1) {
        return -1;
    }

    // If load average < 1.0, likely little contention
    // sched_yield will probably be CNO
    printf("  CPUs: %ld\n", num_cpus);
    printf("  Load average: %.2f %.2f %.2f\n",
           loadavg[0], loadavg[1], loadavg[2]);
    printf("  CNO likely: %s\n",
           loadavg[0] < 1.0 ? "YES (low contention)" : "NO (high contention)");

    return loadavg[0] < 1.0;
}

/*
 * Get current thread's scheduling policy and priority
 */
void print_scheduling_info(void) {
    struct sched_param param;
    int policy = sched_getscheduler(0);

    if (policy == -1) {
        perror("sched_getscheduler");
        return;
    }

    if (sched_getparam(0, &param) == -1) {
        perror("sched_getparam");
        return;
    }

    const char *policy_name;
    switch (policy) {
        case SCHED_OTHER: policy_name = "SCHED_OTHER (CFS)"; break;
        case SCHED_FIFO:  policy_name = "SCHED_FIFO (RT)"; break;
        case SCHED_RR:    policy_name = "SCHED_RR (RT)"; break;
#ifdef SCHED_BATCH
        case SCHED_BATCH: policy_name = "SCHED_BATCH"; break;
#endif
#ifdef SCHED_IDLE
        case SCHED_IDLE:  policy_name = "SCHED_IDLE"; break;
#endif
        default:          policy_name = "UNKNOWN"; break;
    }

    printf("  Policy: %s\n", policy_name);
    printf("  Priority: %d\n", param.sched_priority);
}

/*
 * Benchmark: Measure yield overhead in CNO scenario
 * Best performed on idle system where CNO is guaranteed
 */
void benchmark_yield_overhead(long iterations) {
    struct timespec start, end;
    long i;

    clock_gettime(CLOCK_MONOTONIC, &start);

    for (i = 0; i < iterations; i++) {
        sched_yield();
    }

    clock_gettime(CLOCK_MONOTONIC, &end);

    long long ns_elapsed = (end.tv_sec - start.tv_sec) * 1000000000LL +
                           (end.tv_nsec - start.tv_nsec);

    printf("Sched_yield CNO Benchmark:\n");
    printf("  Iterations: %ld\n", iterations);
    printf("  Total time: %lld ns (%.3f ms)\n", ns_elapsed, ns_elapsed / 1e6);
    printf("  Per yield: %.2f ns (%.3f μs)\n",
           (double)ns_elapsed / iterations,
           (double)ns_elapsed / iterations / 1000.0);
    printf("  Overhead includes:\n");
    printf("    - Syscall entry/exit\n");
    printf("    - Scheduler lock acquisition\n");
    printf("    - Runqueue examination\n");
    printf("    - Scheduling decision\n");
    printf("    - (Likely) immediate resume - CNO!\n");
}

/*
 * Demonstrate yield with contention
 * Creates a competing thread to show non-CNO case
 */
void *spinning_thread(void *arg) {
    volatile long *stop = (volatile long *)arg;
    long count = 0;

    while (!*stop) {
        count++;
        // Tight loop - creates contention
    }

    return (void *)count;
}

void benchmark_yield_with_contention(long iterations) {
    pthread_t thread;
    volatile long stop = 0;
    struct timespec start, end;
    long i;

    // Create competing thread
    pthread_create(&thread, NULL, spinning_thread, (void *)&stop);

    // Give thread time to start
    usleep(10000);

    clock_gettime(CLOCK_MONOTONIC, &start);

    for (i = 0; i < iterations; i++) {
        sched_yield();
        // With contention, this may cause actual context switches
    }

    clock_gettime(CLOCK_MONOTONIC, &end);

    // Stop competing thread
    stop = 1;
    pthread_join(thread, NULL);

    long long ns_elapsed = (end.tv_sec - start.tv_sec) * 1000000000LL +
                           (end.tv_nsec - start.tv_nsec);

    printf("\nSched_yield WITH Contention:\n");
    printf("  Iterations: %ld\n", iterations);
    printf("  Total time: %lld ns (%.3f ms)\n", ns_elapsed, ns_elapsed / 1e6);
    printf("  Per yield: %.2f ns (%.3f μs)\n",
           (double)ns_elapsed / iterations,
           (double)ns_elapsed / iterations / 1000.0);
    printf("  Note: May include actual context switches (NOT pure CNO)\n");
}

/*
 * Analysis: Scheduler internals during yield
 *
 * Linux CFS scheduler flow for sched_yield():
 *
 * 1. SYSCALL ENTRY:
 *    - sys_sched_yield() in kernel/sched/core.c
 *    - Acquire runqueue lock (per-CPU or global)
 *
 * 2. YIELD OPERATION:
 *    - yield_task() for current scheduler class
 *    - CFS: yield_task_fair()
 *      - Dequeue task from CFS runqueue
 *      - Set skip buddy hint (skip this task next)
 *      - Re-enqueue at end (vruntime unchanged)
 *
 * 3. SCHEDULE DECISION:
 *    - schedule() called
 *    - Pick next task: pick_next_task()
 *    - CFS: pick_next_task_fair()
 *      - Examine leftmost task in rbtree
 *      - If alone: picks same task! (CNO)
 *      - If others: picks next by vruntime
 *
 * 4. CONTEXT SWITCH (if needed):
 *    - context_switch() in kernel/sched/core.c
 *    - Switch memory context (mm_struct)
 *    - Switch thread context (registers)
 *    - Architecture-specific: switch_to()
 *
 * 5. CNO CASE:
 *    - Same task selected immediately
 *    - No context_switch() call
 *    - Runqueue lock released
 *    - Return to user space
 *    - All overhead, zero progress
 */

int main(void) {
    printf("=== OS-Level CNO: Scheduler Yield No-Operation ===\n\n");

    printf("Concept: Yield CPU to scheduler, but immediately resume\n");
    printf("         if no other tasks are runnable (CNO condition).\n\n");

    // Check CNO conditions
    printf("Checking CNO conditions:\n");
    check_cno_condition();
    printf("\n");

    // Show scheduling info
    printf("Current scheduling configuration:\n");
    print_scheduling_info();
    printf("\n");

    // Demonstrate basic yield CNO
    printf("Executing sched_yield CNO (basic)...\n");
    yield_cno_basic();
    printf("  ✓ Scheduler consulted\n");
    printf("  ✓ No context switch (likely)\n");
    printf("  ✓ Same thread continues - CNO achieved\n\n");

    // Run benchmark
    printf("Measuring overhead without contention (CNO case)...\n");
    benchmark_yield_overhead(10000);
    printf("\n");

    // Compare with contention
    printf("Comparing with contention scenario...\n");
    benchmark_yield_with_contention(1000);
    printf("\n");

    printf("Platform Details:\n");
#ifdef __linux__
    printf("  OS: Linux\n");
    printf("  Scheduler: CFS (Completely Fair Scheduler)\n");
    printf("  Yield behavior: Move to end of runqueue, reselect\n");
#elif defined(__APPLE__)
    printf("  OS: macOS\n");
    printf("  Scheduler: Mach scheduler\n");
    printf("  Yield behavior: Yield to equal or higher priority\n");
#elif defined(__FreeBSD__)
    printf("  OS: FreeBSD\n");
    printf("  Scheduler: ULE scheduler\n");
    printf("  Yield behavior: Yield quantum to other tasks\n");
#else
    printf("  OS: Unknown POSIX\n");
#endif
    printf("  CNO achieved when: No other runnable tasks exist\n");
    printf("  Result: Maximum scheduler overhead, zero forward progress\n");

    return 0;
}

/*
 * DEEPER DIVE: Why sched_yield() Can Be CNO
 * =========================================
 *
 * 1. INTENDED PURPOSE:
 *    - Cooperative multitasking
 *    - Allow other tasks to run
 *    - Reduce priority inversion
 *
 * 2. CNO SCENARIO:
 *    - System is idle
 *    - No competing tasks
 *    - Scheduler has nothing else to run
 *    - Current task immediately reselected
 *
 * 3. WORK PERFORMED (despite CNO):
 *    - Runqueue lock acquisition
 *    - Runqueue scan/examination
 *    - Scheduling decision logic
 *    - Statistics updates
 *    - Lock release
 *
 * 4. OVERHEAD BREAKDOWN:
 *    - Syscall overhead: ~100-300ns
 *    - Scheduler overhead: ~200-1000ns
 *    - Total: ~300-1300ns typical
 *    - Context switch (if occurs): +1000-5000ns
 *
 * 5. SECURITY IMPLICATIONS:
 *    - Yield can be abused for timing
 *    - Side-channel via scheduling behavior
 *    - DoS: excessive yields consume scheduler time
 *    - Rate limiting may apply (cgroups)
 *
 * 6. COMPARISON WITH ALTERNATIVES:
 *    - sleep(0): Similar but may enforce minimum delay
 *    - nanosleep(0): Also CNO if no other tasks
 *    - pthread_yield(): Alias for sched_yield on many systems
 *    - usleep(0): May be CNO or no-op depending on OS
 *
 * 7. REAL-WORLD USAGE:
 *    - Spinlock backoff strategies
 *    - Userspace synchronization
 *    - Load testing scheduler
 *    - Educational demonstrations
 */
