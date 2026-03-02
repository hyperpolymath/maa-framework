/**
 * Balanced Operations CNO in C
 *
 * Performs operations that cancel out, resulting in no net effect.
 */

#include <stdint.h>

int main(void) {
    // All operations cancel out
    volatile int x = 0;  // volatile prevents optimization

    // Increment then decrement
    x = x + 1;
    x = x - 1;

    // Multiply then divide
    x = x * 2;
    x = x / 2;

    // XOR with self (always 0)
    x = x ^ x;

    // Final value: x == 0 (unchanged)
    return 0;
}
