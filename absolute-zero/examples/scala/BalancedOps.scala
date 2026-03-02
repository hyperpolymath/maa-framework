/**
 * Balanced Operations CNO in Scala
 *
 * Performs operations that cancel out, resulting in no net effect.
 * Demonstrates functional programming patterns with zero net result.
 */

object BalancedOps {
  def main(args: Array[String]): Unit = {
    // All operations cancel out
    var x = 0

    // Increment then decrement
    x = x + 1
    x = x - 1

    // Multiply then divide
    x = x * 2
    x = x / 2

    // XOR with self (always 0)
    x = x ^ x

    // Functional transformations that cancel
    val list = List(1, 2, 3)
    val reversed = list.reverse.reverse

    // Map operations that cancel
    val doubled = list.map(_ * 2).map(_ / 2)

    // Option transformations
    val opt = Some(42)
    val transformed = opt.map(_ + 10).map(_ - 10)

    // Final values: x == 0, reversed == list, doubled == list, transformed == opt
  }
}

/*
 * Scala-specific CNO considerations:
 *
 * Immutability and GC:
 * - Functional operations create new collections
 * - Original collections remain unchanged
 * - Heavy GC pressure from intermediate objects
 * - CNO at semantic level despite allocation storm
 *
 * Lazy Evaluation:
 * - Scala supports lazy vals and views
 * - Could defer some operations
 * - But CNO result guaranteed regardless
 *
 * Type System:
 * - Rich type inference at compile time
 * - Type erasure at runtime (JVM limitation)
 * - Generic operations monomorphized or boxed
 *
 * Pattern Matching:
 * - Not used here, but part of runtime
 * - Compiled to efficient tableswitch/lookupswitch
 * - CNO compatible with exhaustive matching
 *
 * At functional level: pure transformations that compose to identity
 * At runtime level: object allocation, GC, method dispatch overhead
 * At JVM level: full stack of runtime machinery
 */
