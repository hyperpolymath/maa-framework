/*
 * ioctl_nop.c - ioctl Query CNO Example
 *
 * CONCEPT:
 * Performs ioctl system call to query device/file information, then
 * immediately discards the result. This exercises the ioctl infrastructure
 * including device driver dispatch, but achieves CNO by ignoring the data.
 *
 * KERNEL MODE TRANSITION OVERHEAD:
 * --------------------------------
 * 1. Syscall entry overhead (~100-200 cycles)
 * 2. File descriptor lookup and validation
 * 3. VFS layer dispatch to device driver
 * 4. Device driver ioctl handler execution
 * 5. Data copy from kernel to user space
 * 6. Syscall return overhead
 *
 * ioctl is particularly heavyweight due to:
 * - File descriptor reference counting
 * - VFS layer indirection
 * - Device driver function pointer dispatch
 * - Potential device lock acquisition
 * - Data marshaling between kernel and user space
 *
 * CONTEXT SWITCH IMPLICATIONS:
 * ---------------------------
 * - No context switch (same thread continues)
 * - BUT: Some ioctl operations may block
 *   - Waiting for device I/O
 *   - Waiting for locks
 *   - Waiting for DMA completion
 *
 * - CNO case: Non-blocking queries
 *   - FIONREAD: Query bytes available
 *   - TIOCGWINSZ: Query terminal size
 *   - FIONBIO: Query/set non-blocking mode
 *   - These return immediately, no blocking
 *
 * TLB FLUSHES:
 * -----------
 * - No TLB flush (same address space)
 * - Device memory mapping (if any) uses TLB
 * - MMIO regions may bypass cache entirely
 * - Copy_to_user may cause TLB misses
 *
 * SCHEDULER IMPACT:
 * ----------------
 * - ioctl is a syscall, thus a preemption point
 * - Scheduler may preempt during ioctl
 * - Long-running ioctl may be interrupted by signals
 * - Device drivers may call schedule() internally
 * - CNO case: Quick return, minimal scheduler interaction
 *
 * SECURITY CHECKS DESPITE CNO:
 * ---------------------------
 * 1. Syscall validation:
 *    - Valid syscall number?
 *    - Valid file descriptor?
 *    - File descriptor refers to open file?
 *
 * 2. File permissions:
 *    - Read/write permissions on fd
 *    - ioctl command requires specific permissions
 *    - Device node permissions (if device file)
 *
 * 3. Capability checks:
 *    - Some ioctl operations require capabilities
 *    - CAP_SYS_ADMIN, CAP_NET_ADMIN, etc.
 *    - Restricted ioctl commands
 *
 * 4. Namespace isolation:
 *    - Network namespace for network devices
 *    - User namespace for device access
 *    - Mount namespace for device nodes
 *
 * 5. LSM hooks:
 *    - SELinux: file_ioctl hook
 *    - AppArmor: file ioctl rules
 *    - Mandatory access control
 *
 * 6. Seccomp filters:
 *    - ioctl may be restricted by seccomp
 *    - Arguments may be filtered
 *    - Syscall auditing
 *
 * PLATFORM COMPATIBILITY:
 * ----------------------
 * Linux:   Rich ioctl interface, many device types
 * BSD:     Similar ioctl, different device drivers
 * macOS:   BSD-style ioctl + macOS-specific extensions
 * Windows: DeviceIoControl() - similar concept
 *
 * ioctl commands vary by platform:
 * - Terminal: TIOCGWINSZ (POSIX-ish, widely supported)
 * - Socket: FIONREAD (BSD origin, widely supported)
 * - Network: SIOCGIFCONF (Linux-specific)
 * - Device-specific: Highly platform-dependent
 *
 * MEASUREMENT:
 * -----------
 * ioctl CNO overhead: 300ns - 2μs
 * Depends on:
 * - Device type (terminal, socket, block device)
 * - Driver complexity
 * - Data copy size
 * - Lock contention
 *
 * Compare to: Simple syscall ~100-300ns
 * Ratio: 1-10x overhead due to VFS/driver layers
 */

#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>
#include <fcntl.h>
#include <time.h>
#include <sys/ioctl.h>
#include <sys/types.h>
#include <sys/stat.h>
#include <errno.h>
#include <string.h>

#ifdef __linux__
#include <linux/fs.h>
#include <sys/socket.h>
#include <net/if.h>
#endif

/*
 * Terminal window size query CNO
 * Queries terminal size, discards result
 */
void ioctl_cno_terminal_size(int fd) {
    struct winsize ws;

    /* Query terminal window size */
    /* Data returned, immediately discarded */
    if (ioctl(fd, TIOCGWINSZ, &ws) == 0) {
        /* We have ws.ws_row, ws.ws_col */
        /* But we ignore them - CNO! */
        (void)ws; /* Suppress unused warning */
    }
}

/*
 * Socket bytes available query CNO
 * Queries bytes available in socket, discards count
 */
void ioctl_cno_socket_fionread(int fd) {
    int bytes_available;

    /* Query bytes available to read */
    if (ioctl(fd, FIONREAD, &bytes_available) == 0) {
        /* We know bytes available, but ignore it */
        /* CNO achieved */
        (void)bytes_available;
    }
}

/*
 * File non-blocking mode query CNO
 * Queries/sets non-blocking mode, discards result
 */
void ioctl_cno_nonblocking(int fd) {
    int flags = 1;

    /* Set non-blocking mode (or query) */
    /* Note: FIONBIO sets mode, not pure query */
    /* For pure CNO, we'd query then ignore */
    /* But this demonstrates the overhead */
    ioctl(fd, FIONBIO, &flags);

    /* Reset to blocking */
    flags = 0;
    ioctl(fd, FIONBIO, &flags);

    /* All overhead, no net change - CNO */
}

/*
 * Benchmark: Terminal ioctl overhead
 */
void benchmark_terminal_ioctl(int fd, long iterations) {
    struct timespec start, end;
    struct winsize ws;
    long i;
    long success = 0;

    clock_gettime(CLOCK_MONOTONIC, &start);

    for (i = 0; i < iterations; i++) {
        if (ioctl(fd, TIOCGWINSZ, &ws) == 0) {
            success++;
        }
    }

    clock_gettime(CLOCK_MONOTONIC, &end);

    long long ns_elapsed = (end.tv_sec - start.tv_sec) * 1000000000LL +
                           (end.tv_nsec - start.tv_nsec);

    printf("Terminal ioctl CNO Benchmark:\n");
    printf("  Command: TIOCGWINSZ (get window size)\n");
    printf("  Iterations: %ld\n", iterations);
    printf("  Successful: %ld\n", success);
    printf("  Total time: %lld ns (%.3f ms)\n", ns_elapsed, ns_elapsed / 1e6);
    printf("  Per ioctl: %.2f ns (%.3f μs)\n",
           (double)ns_elapsed / iterations,
           (double)ns_elapsed / iterations / 1000.0);
    printf("  Overhead includes:\n");
    printf("    - Syscall entry/exit\n");
    printf("    - FD lookup and validation\n");
    printf("    - VFS layer dispatch\n");
    printf("    - TTY driver ioctl handler\n");
    printf("    - Data copy to user space\n");
    printf("    - Result ignored - CNO!\n");
}

/*
 * Benchmark: Pipe ioctl overhead
 */
void benchmark_pipe_ioctl(long iterations) {
    int pipefd[2];
    struct timespec start, end;
    int bytes_available;
    long i;
    long success = 0;

    /* Create pipe for ioctl testing */
    if (pipe(pipefd) == -1) {
        perror("pipe");
        return;
    }

    /* Write some data to query later */
    const char *data = "test";
    write(pipefd[1], data, 4);

    clock_gettime(CLOCK_MONOTONIC, &start);

    for (i = 0; i < iterations; i++) {
        if (ioctl(pipefd[0], FIONREAD, &bytes_available) == 0) {
            success++;
        }
    }

    clock_gettime(CLOCK_MONOTONIC, &end);

    long long ns_elapsed = (end.tv_sec - start.tv_sec) * 1000000000LL +
                           (end.tv_nsec - start.tv_nsec);

    printf("\nPipe ioctl CNO Benchmark:\n");
    printf("  Command: FIONREAD (bytes available)\n");
    printf("  Iterations: %ld\n", iterations);
    printf("  Successful: %ld\n", success);
    printf("  Total time: %lld ns (%.3f ms)\n", ns_elapsed, ns_elapsed / 1e6);
    printf("  Per ioctl: %.2f ns (%.3f μs)\n",
           (double)ns_elapsed / iterations,
           (double)ns_elapsed / iterations / 1000.0);
    printf("  CNO: Bytes available queried but ignored\n");

    close(pipefd[0]);
    close(pipefd[1]);
}

/*
 * Demonstrate ioctl with /dev/null
 * Most ioctls fail on /dev/null, but still incur overhead
 */
void ioctl_cno_devnull(void) {
    int fd = open("/dev/null", O_RDONLY);
    if (fd == -1) {
        perror("open /dev/null");
        return;
    }

    struct winsize ws;

    /* This will likely fail (ENOTTY - inappropriate ioctl) */
    /* But still goes through full syscall machinery */
    int result = ioctl(fd, TIOCGWINSZ, &ws);

    printf("ioctl on /dev/null:\n");
    printf("  Result: %d\n", result);
    if (result == -1) {
        printf("  Error: %s\n", strerror(errno));
        printf("  Note: Full syscall overhead incurred despite failure\n");
    }
    printf("  CNO: Even failure path exercises kernel infrastructure\n");

    close(fd);
}

/*
 * Analysis: ioctl call path
 *
 * ioctl() system call execution flow:
 *
 * 1. SYSCALL ENTRY:
 *    - sys_ioctl() in fs/ioctl.c
 *    - Parameters: fd, cmd, arg
 *    - Validate fd is valid
 *
 * 2. FILE DESCRIPTOR LOOKUP:
 *    - fget_light() or fget()
 *    - Lookup fd in current->files
 *    - Increment file reference count
 *    - Validate file is open
 *
 * 3. VFS LAYER:
 *    - Check if file ops has ioctl
 *    - vfs_ioctl() dispatch
 *    - Generic ioctl handling (FIONREAD, etc.)
 *    - Or call file->f_op->unlocked_ioctl()
 *
 * 4. DEVICE DRIVER:
 *    - Driver-specific ioctl handler
 *    - For terminal: tty_ioctl()
 *    - For socket: sock_ioctl()
 *    - For block device: blkdev_ioctl()
 *    - Each has different overhead
 *
 * 5. DATA MARSHALING:
 *    - copy_from_user() for input
 *    - Driver processes request
 *    - copy_to_user() for output
 *    - CNO: Data copied then ignored!
 *
 * 6. CLEANUP:
 *    - Decrement file reference count
 *    - Return result to user space
 *
 * Example: TIOCGWINSZ on terminal:
 * - tty_ioctl() in drivers/tty/tty_io.c
 * - Calls tty_tiocgwinsz()
 * - Reads tty->winsize structure
 * - copy_to_user(&ws, &tty->winsize)
 * - All this for data we discard!
 */

int main(void) {
    printf("=== OS-Level CNO: ioctl Query No-Operation ===\n\n");

    printf("Concept: Perform ioctl queries that retrieve device information,\n");
    printf("         then immediately discard results, achieving CNO through\n");
    printf("         exercising VFS and driver infrastructure.\n\n");

    /* Check if stdout is a terminal */
    if (isatty(STDOUT_FILENO)) {
        printf("Testing with stdout (terminal):\n");

        /* Demonstrate terminal ioctl CNO */
        printf("\nExecuting terminal ioctl CNO...\n");
        ioctl_cno_terminal_size(STDOUT_FILENO);
        printf("  ✓ Window size queried\n");
        printf("  ✓ Result discarded - CNO achieved\n\n");

        /* Benchmark terminal ioctl */
        printf("Measuring terminal ioctl overhead...\n");
        benchmark_terminal_ioctl(STDOUT_FILENO, 100000);
    } else {
        printf("Note: stdout is not a terminal, skipping terminal ioctl tests\n");
    }

    /* Test pipe ioctl */
    printf("\nTesting with pipe:\n");
    benchmark_pipe_ioctl(100000);

    /* Test /dev/null */
    printf("\nTesting with /dev/null:\n");
    ioctl_cno_devnull();

    printf("\n");
    printf("Platform Details:\n");
#ifdef __linux__
    printf("  OS: Linux\n");
    printf("  VFS: Virtual File System layer\n");
    printf("  ioctl: Unified device control interface\n");
#elif defined(__APPLE__)
    printf("  OS: macOS\n");
    printf("  VFS: BSD-style VFS\n");
    printf("  ioctl: BSD ioctl interface\n");
#elif defined(__FreeBSD__)
    printf("  OS: FreeBSD\n");
    printf("  VFS: BSD VFS\n");
    printf("  ioctl: BSD ioctl interface\n");
#else
    printf("  OS: Unknown POSIX\n");
#endif
    printf("  CNO achieved: Maximum ioctl overhead, zero data utilization\n");
    printf("  Overhead: ~300ns-2μs depending on device/driver\n");

    return 0;
}

/*
 * DEEPER DIVE: Why ioctl CNO Is Interesting
 * =========================================
 *
 * 1. OVERHEAD BREAKDOWN:
 *    Component              | Time (ns) | Purpose
 *    -----------------------|-----------|------------------
 *    Syscall entry/exit     | 100-300   | Mode transition
 *    FD lookup              | 20-50     | Find file struct
 *    VFS dispatch           | 30-100    | Route to driver
 *    Driver handler         | 50-500    | Device-specific
 *    Data copy              | 20-100    | Kernel↔User
 *    Reference counting     | 10-30     | File lifecycle
 *    -----------------------|-----------|------------------
 *    Total                  | 230-1080  | For CNO!
 *
 * 2. DEVICE-SPECIFIC VARIATIONS:
 *    Device Type     | ioctl Cost | Notes
 *    ----------------|------------|---------------------------
 *    Terminal        | 300-800ns  | TTY layer overhead
 *    Socket          | 400-1000ns | Network stack involved
 *    Pipe            | 200-500ns  | Simpler than socket
 *    Block device    | 500-2000ns | Device queue locks
 *    Character dev   | 300-1000ns | Driver-dependent
 *    /dev/null       | 200-400ns  | Minimal driver
 *
 * 3. SECURITY IMPLICATIONS:
 *    - ioctl is major attack surface
 *    - Historical vulnerabilities in drivers
 *    - Privilege escalation via ioctl bugs
 *    - Input validation critical
 *    - Even CNO ioctl exercises security checks
 *
 * 4. REAL-WORLD USAGE:
 *    - Device configuration
 *    - Terminal control
 *    - Network interface setup
 *    - Hardware control
 *    - Don't use for hot path queries!
 *
 * 5. ALTERNATIVES TO ioctl:
 *    - sysfs/procfs: Read-based queries
 *    - netlink: Structured network config
 *    - eBPF: Programmable kernel interface
 *    - All generally more structured than ioctl
 *
 * 6. WHY THIS IS CNO:
 *    - Full kernel path traversed
 *    - Driver code executed
 *    - Data copied to user space
 *    - Result immediately discarded
 *    - Zero computational value
 *    - Pure overhead demonstration
 *
 * 7. EDUCATIONAL VALUE:
 *    - Shows VFS layer overhead
 *    - Demonstrates driver dispatch
 *    - Illustrates data marshaling cost
 *    - Highlights syscall complexity
 *    - Motivates optimization techniques
 */
