# Language classification

> _Certified language nerds only beyond this point, and I will be checking your
references! Also, please keep me honest in correctly categorizing the language.
The focus is on usefulness more than advancing PLDI, but I'm open to criticism,
suggestions, and crazy ideas._

The `dt` language is in the [concatenative language](https://concatenative.org)
family. That means it's a functional programming language (functions are
first-class, values have immutable semantics) with a concatenative
style rather than the traditional applicative style.

The `dt` language has an imperative _feel_ in the sense that all "functions"
are linguistically imperative "commands." There is no distinguishing from pure
and impure logic; side-effects are allowed and not managed.

_For the adept: The language is point-free with opt-in bindings. Everything
is evaluated in strict left-to-right sequence, and all operations can have
arbitrary arity both in and out, including runtime-dynamic arity. Typing is
dynamic, and the language is homoiconic._

It's inspired by many other tools and languages like Unix-style pipes and
shells, `awk`, Forth, Joy, Factor, Haskell, ML, Lisps, Lua, Tcl, Ruby, and
Perl. `dt` does not intended to be better or replace any of these, they're all
fantastic and have their place! It's simply meant to be a best tool for
different kinds of jobs.

Linguistically, `dt` command definitions follow a convention of using
[subject-object-verb (SOV)](https://en.wikipedia.org/wiki/Subject%E2%80%93object%E2%80%93verb_word_order)
grammatical order similar to Japanese, Korean, or Latin. _(But with much more
context elision, even more than Japanese!)_

See also:

* Jon Purdy's [_Why Concatenative Programming Matters_](https://evincarofautumn.blogspot.com/2012/02/why-concatenative-programming-matters.html)
* [Comparisons of dt to similar tools/languages](./comparisons.md)

