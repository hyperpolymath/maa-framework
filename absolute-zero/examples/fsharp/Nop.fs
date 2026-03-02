(*
 * Certified Null Operation in F#
 *
 * A functional program that does absolutely nothing at the application level.
 * Returns exit code 0 without any observable side effects.
 *
 * Properties:
 * - Terminates immediately
 * - No I/O operations
 * - No side effects (pure functional CNO)
 * - Exit code 0
 *
 * Compile: fsharpc Nop.fs
 * Run: ./Nop.exe (or mono Nop.exe)
 *)

[<EntryPoint>]
let main argv =
    // The minimal CNO in F#
    // Returns 0 to indicate successful termination
    // No effectful computation occurs
    0

(*
 * Verification notes:
 * - F# runtime initializes (.NET CLR)
 * - Functional programming emphasizes referential transparency
 * - This program has no side effects - a "pure" CNO
 * - The return value 0 is the only output
 *
 * In functional programming terms, this is:
 * main : string[] -> int
 * main = λargs. 0
 *
 * The function ignores its input and returns zero,
 * performing no effectful computation in the process.
 *)
