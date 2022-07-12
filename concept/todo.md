# TODO

1. Streams (e.g. stdin; large files)
  - lazy lists?
  - generators?

1. Remove warnings/errors/panics with Result

1. Typed builtins
  - Step 1: Anything not higher-order
  - Step 2: What type is a quotation? What about `opt` or `times`?
    - Comptime checks should be possible for some states
    - Runtime only is possible when IO comes into play. Monads...?

1. Stack traces
  - See: https://concatenative.org/wiki/view/Concatenative%20language/Continuations

1. Compilation
  - (x86_64 first? arm64?)

1. Conversion to/from string

1. Smaller executables 
  - Required to target no-OS environments. Glean from https://en.wikipedia.org/wiki/Espruino and similar projects?

1. Tail-call optimization
  - Maybe use this technique: https://docs.factorcode.org/content/article-tail-call-opt.html

1. Other data structures
  - Maps, Sets, Heaps
  - Allow anything as arbitrary as JSON?

1. "Green thread" processes
  - Erlang's `spawn`, `receive`, and PIDs are the way to go
