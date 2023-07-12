![License](https://img.shields.io/github/license/booniepepper/dt)

# `dt`

It's duct tape for your unix pipes. A programming language for doing small
stuff fast and easy.

In the words of [Red Green](https://www.redgreen.com):

> Remember, it's only temporary... unless it works!

## Use in pipes

When piping in/out, the REPL is skipped. If something is piping into `dt` then
standard input is fed into `dt` as a list of lines.

```
$ echo -e "3\n2\n1" | dt rev pls
1
2
3

$ alias scream-lines="dt [upcase words unlines] map pls"
$ echo "hey you pikachu" | scream-lines
HEY
YOU
PIKACHU
```

## Use as a shebang

When the first argument to `dt` is a file starting with `#!` it will interpret
the file. In short: `dt` supports shebang scripting.

A naive tee implementation:

`tee.dt`

```
#!/usr/bin/env dt

read-lines unlines   \stdin :
args pop   \file :

stdin pl
stdin file writef
```

Then use like:

```
cat wish-list | sed 's/red/green/g' | tee.dt new-wish-list
```

## Interactive mode

Running `dt` by itself with no pipes in or out starts a read-eval-print loop
(REPL).

```
$ dt
dt 1.x.x
Now, this is only temporary... unless it works.
» # Comments start with #
» 
» 1 1 + println
2
» 
» # Printing is common, so there are a bunch of printing shorthands.
» # "p" is print, "pl" is print line, "pls" is print lines (i.e. of a list of values)
» # Let's define a command that consumes a value, prints it, then returns its double.
» 
» [ \n :   n p " " p   n 2 *] \print-and-double def
» 
» 
» # And let's do it... 7 times!
» 
» 1 \print-and-double 7 times   drop
1 2 4 8 16 32 64
» 
» 
» # You can conditionally execute code
» 
» ["hi" pl] false do?
» ["bye" pl] true do?
bye
» 
» quit
```

For a best experience, also install
[`rlwrap`](https://github.com/hanslub42/rlwrap) and set a shell alias like so:

```
$ alias dtsh='rlwrap dt'
$ dtsh
dt 1.x.x
Now, this is only temporary... unless it works.
» 
```

The above example assumes a bash-like shell. Details on the syntax and
configuration files to set an alias that persists will vary by your shell.

## Downloads

Download assets from [releases](https://github.com/booniepepper/dt/releases/) and
put them somewhere on your PATH.

An installation script may come soon.

## Building from source

To build from source, clone the repo and run `./build help` for instructions.
The project currently builds with Zig 0.11.+ and a recent Cargo toolchain. The
resulting binary for your machine will be produced in `./zig-out/bin/dt`

## The nerdy stuff

**⚠️ Most people should skip this section! ⚠️**

_Certified language nerds only beyond this point, and I will be checking your
references! Also, please keep me honest in correctly categorizing the language.
The focus is on usefulness more than advancing PLDI, but I'm open to criticism,
suggestions, and crazy ideas._

The `dt` language is in the [concatenative language](https://concatenative.org)
family. That means it's a functional programming language (functions are
first-class, values have immutable semantics) with a concatenative
style rather than the traditional applicative style.

See also Jon Purdy's [_Why Concatanative Programming Matters_](https://evincarofautumn.blogspot.com/2012/02/why-concatenative-programming-matters.html)

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

`dt` is implemented in Zig with no plans to self-host or rewrite in Rust or Go.
Please do suggest better strategies for memory management and optimizations! I
have some experience working at this level, but still much to learn. The
current implementation is still fairly naive.

## Credits

Shared as open source software, distributed under the terms of [the 3-Clause BSD License](https://opensource.org/license/BSD-3-clause/).

By J.R. Hill | https://so.dang.cool | https://github.com/booniepepper
