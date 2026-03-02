%%%-------------------------------------------------------------------
%%% @doc
%%% nop.erl - Computing No Output in Erlang
%%% Copyright: Absolute Zero Project
%%% License: AGPL-3.0 / PALIMPS
%%%
%%% ERLANG CNO: FUNCTIONAL PURITY IN A CONCURRENT WORLD
%%%
%%% Erlang is a functional language designed for concurrent, distributed
%%% systems. Despite being built for message-passing and side effects,
%%% Erlang's core is purely functional:
%%%
%%% FUNCTIONAL CHARACTERISTICS:
%%% - Immutable data structures (all variables are single-assignment)
%%% - First-class functions and higher-order functions
%%% - Pattern matching and recursion instead of loops
%%% - No shared state (processes communicate via messages)
%%% - Process isolation ensures functional purity within processes
%%%
%%% THE IDENTITY FUNCTION - ERLANG'S PURE CNO:
%%%
%%% identity(X) -> X.
%%%
%%% This is the purest CNO in Erlang:
%%% - Works for any term (Erlang's universal type)
%%% - No side effects, no message passing
%%% - Referentially transparent
%%% - The identity element of function composition
%%%
%%% IMMUTABILITY AS CNO FOUNDATION:
%%% In Erlang, once a variable is bound, it cannot change:
%%%   X = 42,
%%%   X = 43.  % This would crash with a badmatch error
%%%
%%% This single-assignment property means all computation is pure
%%% transformation. Variables don't vary - they're immutable bindings.
%%% This is CNO at the semantic level: no observable state mutation.
%%%
%%% CNO STRATEGIES IN ERLANG:
%%% 1. Empty module (no exported functions, no startup)
%%% 2. Module with functions but no execution
%%% 3. Identity function: fun(X) -> X end
%%% 4. Functions returning atoms: ok, undefined, nil
%%% 5. Processes that receive but don't send messages
%%%
%%% This example demonstrates pure functional CNO in Erlang's
%%% concurrent context.
%%% @end
%%%-------------------------------------------------------------------

-module(nop).

%% No exported functions means this module compiles but does nothing
%% Uncomment to create executable entry points:
%% -export([identity/1, constant_ok/1, nop_process/0, main/0]).

%%%===================================================================
%%% Pure CNO Functions
%%%===================================================================

%% @doc The identity function - purest CNO
%% Spec: any() -> any()
%% Returns its argument unchanged. No side effects, no messages.
%% This is the mathematical identity function in Erlang.
identity(X) -> X.

%% @doc Constant function returning 'ok'
%% Discards input, returns the atom 'ok' (Erlang's unit-like value)
constant_ok(_) -> ok.

%% @doc Constant function returning 'undefined'
%% Another common Erlang "nothing" value
constant_undefined(_) -> undefined.

%% @doc Higher-order identity - returns function unchanged
%% Demonstrates CNO at the functional level
functional_identity(F) when is_function(F) -> F.

%% @doc Composition of identities
%% identity ∘ identity = identity
%% Perfect example of CNO composition
composed_identity(X) -> identity(identity(X)).

%% @doc Anonymous identity function
%% Lambda form of identity
anonymous_identity() -> fun(X) -> X end.

%% @doc The const function - returns a function that always returns X
%% const(X) returns fun(_) -> X end
%% Demonstrates partial application and closure
const(X) -> fun(_) -> X end.

%% @doc Apply identity to a value
%% Still CNO because identity produces no effects
apply_identity(X) ->
    Id = fun(Y) -> Y end,
    Id(X).

%%%===================================================================
%%% Concurrent CNO (Processes that do nothing)
%%%===================================================================

%% @doc A process that receives messages but does nothing with them
%% This is CNO in the concurrent domain: computation without output
%% The process exists, receives, but produces no observable effects
nop_process() ->
    receive
        _ -> nop_process()  % Ignore all messages, recurse
    end.

%% @doc Spawn a CNO process and return its PID
%% Creates a process that computes (exists, receives) but outputs nothing
spawn_nop() ->
    spawn(fun nop_process/0).

%% @doc Send a message to a process and ignore the result
%% The message is sent (side effect) but we get no output
send_nop(Pid, Message) ->
    Pid ! Message,
    ok.  % Return ok, but the process does nothing with the message

%%%===================================================================
%%% Main Entry Point (for executable CNO)
%%%===================================================================

%% @doc Main function for escript execution
%% Uncomment the -export directive above to enable
%%
%% Compile: erlc nop.erl
%% Run: erl -noshell -s nop main -s init stop
%% Output: (nothing)
%% Exit: 0
main() ->
    %% Execute identity on various values
    _ = identity(ok),
    _ = identity(42),
    _ = identity("CNO"),

    %% Execute constant functions
    _ = constant_ok(anything),
    _ = constant_undefined([1,2,3]),

    %% Compose identities
    _ = composed_identity(pure),

    %% All results ignored - pure CNO execution
    ok.

%%%===================================================================
%%% Documentation and Philosophical Notes
%%%===================================================================

%% VERIFICATION OF CNO:
%%
%% $ erlc nop.erl
%% $ erl -noshell -s nop main -s init stop > output.txt 2>&1
%% $ wc -c output.txt
%% 0 output.txt

%% FUNCTIONAL PURITY IN ERLANG:
%%
%% 1. IMMUTABILITY:
%%    All variables are single-assignment. Once bound, they never change.
%%    This eliminates an entire class of side effects and makes all
%%    computation pure transformation.
%%
%%    Example:
%%      X = 1,        % Bind X to 1
%%      X = X + 1.    % ERROR: badmatch (X is already 1)
%%
%% 2. PATTERN MATCHING:
%%    Erlang uses pattern matching instead of assignment:
%%      {ok, Value} = get_value()
%%    This is unification, not mutation. Pure and declarative.
%%
%% 3. REFERENTIAL TRANSPARENCY (mostly):
%%    Pure functions can be replaced with their results:
%%      identity(42) can be replaced with 42 everywhere
%%    This holds for all pure Erlang functions.
%%
%% 4. PROCESS ISOLATION:
%%    Processes share nothing. All communication is via message copying.
%%    This enforces functional purity at the architectural level.
%%    A process's local computation can be completely pure.
%%
%% 5. TAIL RECURSION:
%%    Erlang optimizes tail recursion into loops. The nop_process/0
%%    function demonstrates infinite recursion that's memory-safe.
%%    Pure functional iteration.

%% THE IDENTITY FUNCTION AS ERLANG CNO:
%%
%% identity(X) -> X.
%%
%% Properties that make it the perfect CNO:
%%
%% - POLYMORPHIC: Works for any Erlang term
%%   identity(42) -> 42
%%   identity([1,2,3]) -> [1,2,3]
%%   identity(fun(X) -> X end) -> fun(X) -> X end
%%
%% - PURE: No side effects, no message passing, no I/O
%%   Calling identity has no observable effects except the return value
%%
%% - TOTAL: Defined for all inputs (never crashes)
%%
%% - OPTIMAL: Compiler can optimize it away entirely
%%
%% - COMPOSABLE: Identity element of function composition
%%   compose(F, identity) = F
%%   compose(identity, F) = F
%%
%% - MATHEMATICAL: It's the identity morphism in category theory

%% ERLANG CNO HIERARCHY (by purity):
%%
%% throw(error)           : Exception (side effect)
%% io:format("~p~n", [X]) : I/O (side effect)
%% self() ! msg           : Message send (side effect)
%% ok                     : Atom value (minimal computation)
%% undefined              : Atom representing absence
%% fun(X) -> X end        : Identity function (pure transformation)
%% fun(_) -> ok end       : Constant function (pure, discards input)
%%
%% The identity function is the Platonic ideal:
%% - It represents computation (a transformation)
%% - It produces no new information (output equals input)
%% - It has no side effects (pure)
%% - It's the neutral element (identity of composition)

%% CONCURRENCY AND CNO:
%%
%% Erlang's process model allows for "concurrent CNO":
%%
%% nop_process() ->
%%     receive
%%         _ -> nop_process()
%%     end.
%%
%% This process:
%% - Exists (uses resources)
%% - Computes (receives messages, pattern matches, recurses)
%% - Produces no output (messages are ignored)
%% - Has no observable effects (no sends, no I/O)
%%
%% It's a concurrent black hole: receiving but never responding.
%% This demonstrates CNO in the distributed, concurrent domain.

%% CATEGORY THEORY PERSPECTIVE:
%%
%% In the category of Erlang terms and functions:
%% - Objects: All Erlang terms (integers, atoms, lists, tuples, etc.)
%% - Morphisms: Functions (f :: a -> b)
%% - Identity: identity :: a -> a for each type
%% - Composition: (f ∘ g)(x) = f(g(x))
%%
%% The identity function satisfies the category laws:
%% - identity ∘ f = f (left identity)
%% - f ∘ identity = f (right identity)
%%
%% This makes identity the categorical "do nothing" that preserves
%% all structure while representing computation.

%% FUNCTIONAL PROGRAMMING AS CNO PHILOSOPHY:
%%
%% Erlang demonstrates that functional programming is naturally aligned
%% with CNO principles:
%%
%% 1. Immutability = No state mutation output
%% 2. Pure functions = No side effect output
%% 3. Pattern matching = Declarative, not imperative
%% 4. Recursion = Transformation without loops
%% 5. Process isolation = No shared state pollution
%%
%% The identity function crystallizes all these properties:
%% It's the essence of pure, effect-free computation.
