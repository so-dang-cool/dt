# Pipes

Pipes are what dt was created for.

You don't have to be an expert or anything, but we're going to assume here that
you've got an idea of what dt code looks like now. If not, consider spending a
little time [exploring the REPL](./repl.md).

> Note: Code blocks on this page prefixed with `$` indicate they can be run
from some shell program, and it is not required to run as root.

dt can read code passed as its arguments.

```
$ dt \"hello\" pls quit
hello
```

Since there's no pipe in or out, and it's not in a shebang invocation, dt
would start a REPL if we don't also say `quit`. (If you don't believe me,
try it!) Running some code before joining a REPL can also be useful, but we
will leave that as an exercise for the reader for now.

Let's pipe something into dt.

```
$ echo hello | dt pls
hello
```

Let's pipe a little more.

```
$ seq 3 | dt pls
1
2
3
```

And let's use `.s` to take a look at dt's state immediately after receiving
piped input here:

```
$ seq 3 | dt .s
[ [ "1" "2" "3" ] ]
```

When piping standard input into dt, it receives a quote of all lines before
proceeding. The lines are read as strings and not interpreted.

> Note: This means dt will load all input before evaluating code it receives as
arguments. This is best for most small scripts. If you have very big data,
you'll want to avoid loading it all in memory at once. For this use
`dt stream ...`. Here's a quick example:
>
>     seq 100 | dev/dt 'stream   [ rl  red pl g ] \r def   [ rl  green pl r ] \g def   r'


# Incomplete

This guide is still being written!


---

Note to self: While writing this, don't earn [the award](https://porkmail.org/era/unix/award)

