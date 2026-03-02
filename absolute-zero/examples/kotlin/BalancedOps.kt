/**
 * Balanced Operations CNO in Kotlin
 *
 * Performs operations that cancel out, resulting in no net effect.
 * Demonstrates Kotlin idioms with zero net result.
 */

fun main() {
    // All operations cancel out
    var x = 0

    // Increment then decrement
    x += 1
    x -= 1

    // Multiply then divide
    x *= 2
    x /= 2

    // XOR with self (always 0)
    x = x xor x

    // Collection transformations that cancel
    val list = listOf(1, 2, 3)
    val reversed = list.reversed().reversed()

    // Map operations that cancel
    val doubled = list.map { it * 2 }.map { it / 2 }

    // Scope functions that cancel
    val result = 42.let { it + 10 }.let { it - 10 }

    // Nullable operations
    val nullable: Int? = 42
    val transformed = nullable?.let { it + 10 }?.let { it - 10 }

    // String operations
    var s = "test"
    s = s.uppercase()
    s = s.lowercase()

    // Final values unchanged: x == 0, reversed == list, doubled == list,
    // result == 42, transformed == 42, s == "test"
}

/*
 * Kotlin-specific CNO considerations:
 *
 * Null Safety:
 * - Runtime checks for nullable operations
 * - Safe call operator (?.) adds branching
 * - CNO maintained even with null safety overhead
 *
 * Extension Functions:
 * - reversed(), uppercase(), etc. are extensions
 * - Compiled to static method calls
 * - No performance penalty vs Java
 *
 * Lambda Expressions:
 * - map { } and let { } use lambdas
 * - May be inlined by compiler
 * - Creates synthetic classes if not inlined
 * - GC impact from lambda objects
 *
 * Immutable Collections:
 * - listOf() creates immutable list
 * - Each transformation creates new collection
 * - Heavy allocation despite CNO result
 *
 * Coroutines:
 * - Not used here, but runtime available
 * - suspend functions would add continuation overhead
 * - CNO achievable even with async operations
 *
 * Inline Classes:
 * - Could use inline classes for zero-overhead wrappers
 * - Still CNO with type safety benefits
 *
 * At semantic level: identity transformations
 * At runtime level: object creation, method dispatch, null checks
 * At JVM level: same overhead as Java plus Kotlin runtime
 */
