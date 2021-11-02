These are ideas on semantics for a language. Syntax will likely differ

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
- Rust's structs/enums/unions (Again, not a first, but maybe best I've seen)
- Rust's Traits
- Mercury's IO
- Matching in function definitions a-la Erlang/Elixir
- Tail call recursion
- Spread operators (esp JavaScript's)
- Actor model from Erlang (esp spawn/receive semantics. PID lacks type information)



