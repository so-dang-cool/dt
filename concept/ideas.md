Guiding principles:

1. **Left-to-right, top-to-bottom always.** This is a core to giving rail an understandable model. I think this is where other concatenative languages (or functional application in Haskell, etc) starts to require a galaxy brain.

2. **Many UNDERSTANDABLE granular pieces.** Abstraction only works when both the interface and implementation are understandable. An interface MUST be understandable, and an implementation SHOULD be as understandable as possible.

3. **Learn, don't copy.** Many other languages have great features. Learn from, but do not exactly copy features or syntax. What is the actual enjoyable thing behind it?

4. **Reduce complexity.** The essence of computer programming is controlling complexity (BWK, Knuth, others). NEVER make things more complex, only pass on that complexity which is NECESSARY for solving a problem. Don't be afraid to make decisions for others on a default behavior.

# Semantic choices

These are ideas on semantics for a language. Syntax will likely differ from anything here

Primary paradigm:
- Concatenative

Must-haves:
- Strict left-to-right processing
  - `() -> (a, b, c)` can connect to:
    - `(c) -> (any)`
    - `(b, c) -> (any)`
    - `(a, b, c) -> (any)`
    - `(<...before a>, a, b, c) -> (any)`
  - (Above is not different from Forth/Factor)
  - But... Also apply this to type restrictions, generics, etc
    - `(a) -> (b)` can be type restricted as something like `(Str/a) -> (Str/b)`

No clear idea on whether the following can be reconciled at all, but...

Ideas I _like_ from other languages:

- Perl's idea of having semantic, linguistic ideas like singularity/plurality baked in to the language via sigils.
  - ColorForth and [RetroForth](http://www.retroforth.org) had similar ideas
- Haskell's type system (Especially things like `Monad m => a -> m a`)
- Factor's quotations (Granted, this comes from Common Lisp, but I really enjoy the factor implementation)
- Functional must-haves (map, flatMap, reduce, filter, scan)
- Fortran's idea of "calling a function on a single value or collection just works"
  - So... polymorphism? Or maybe single value coerces to single element list? Am I in APL territory?
- Rust's structs/enums/unions (Again, not a first, but maybe best I've seen)
- Rust's Traits
- Rust's "put tests anywhere, probably right in the source file" convention
- Mercury's IO sugar
- Matching in function definitions a-la Erlang/Elixir/Prolog
- Tail call recursion
- Spread operators (esp JavaScript's)
- Actor model from Erlang (esp spawn/receive semantics. PID lacks type information)
- Pipe operator `|>` from languages like Elixir/F#, and `->` and `->>` from Clojure
- Elymas's stack reordering https://github.com/Drahflow/Elymas
- min and mn's lispy style for lambdas, lists, etc https://h3rald.com/mn/Mn_DeveloperGuide.htm
- Zig's "Speak the C ABI" goal
  - Shen's one-language transpiling to many languages goal is similar, but an opposite approach
- Zig's comptime
  - Mostly that it's possible to write macro-ish stuff in the language itself. That it *can*
    be resolved at compile time is nice too.
  - Kind of related to homoiconic languages
- Everything is a ____ paradigms.
  - List: Lisp/Scheme/etc
  - Object: Java/Python/Ruby and their friends
  - String: Tcl
  - File: Unix/Linux/BSD

Like != should have. The above is just a list of things that make me happy.
[Start with No](https://basecamp.com/gettingreal/05.3-start-with-no) is still a good idea.

Ideas in conflict:
- "Convention over Configuration" a la DHH vs. "Explicit better than Implicit" a la Python zen
- Actors vs. Objects -- Can all "Objects" (Assuming something _should_ mix state and logic) really be actors?

# Syntactic choices

## Function signature considerations

- `( ... a b c -- ... d e f)` is not terrible but not easy
- `(Monad m => ... Int -> String -> m String )` Kinda like the Haskell arrow Syntax. (Although hard to grok where input vs outputs)
- `{ a b c -- d e }` forth-like bindings are not great. This binds only the left half, and the right half is unaffected. (i.e. you wouldn't do a Fortran-ish `d = etc`)
- `:: name ( a b -- c ) ...etc... ;` Factor's double `::` for argument binding is far from the signature

 What about this?

`: my-fn (Monad m => Int String/s -> m/String ) ...etc... ;`
- Meaning: define a function `my-fn`
- Have a type variable `m` for `Monad`
  - (Kind of a weird use of a type variable)
- Function will/must consume an int and a String from the stack
- The `String` is bound to a variable `s` (while the previous `Int` is not)
- The function will/must return some `Monad` of `String`

Not sure if it's great to have `/` be the same for `Type/var` and `HigherType/Type`

Also would like (or at least... want to think about):

- optional input through default argument values (if that can even make sense)
- function polymorphism (maybe like Haskell instances)

# Experience

- IDE Suggestions, Tab-completions (Through an LSP I think)
- "Heat map" view, for when stack gets too huge
  - Need to prevent too many things in a stack, no one can manage that
  - Need to be configurable (per project? per file? per function?)
- Need stack status (what types near top?) as hints when hovering
- Rust's compiler is so helpful and positive. Take notes
