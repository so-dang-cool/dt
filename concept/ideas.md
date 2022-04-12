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

Ideas I like from other languages:
- Perl's idea of having ideas like singularity/plurality baked in to the language
- Haskell's type system (Especially things like `Monad m => a -> m a`)
- Factor's quotations (Granted, this comes from Lisp, but I really like the factor implementation)
- Functional must-haves (map, flatMap, reduce, filter, scan)
- Fortran's idea of "calling a function on a single value or collection just works"
  - So... polymorphism? Or maybe single value coerces to single element list? Am I in APL territory?
- Rust's structs/enums/unions (Again, not a first, but maybe best I've seen)
- Rust's Traits
- Mercury's IO
- Matching in function definitions a-la Erlang/Elixir
- Tail call recursion
- Spread operators (esp JavaScript's)
- Actor model from Erlang (esp spawn/receive semantics. PID lacks type information)

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


