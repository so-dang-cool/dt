# Pipes

Pipes are what dt was created for.

You don't have to be an expert or anything. If you do want to know more before
continuing, consider spending a little time [exploring the REPL](./repl.md).

> Note: Code blocks on this page prefixed with `$` indicate they can be run
from some shell program, and it is not required to run as root.

dt can read code passed as its arguments.

```
$ dt '"hello" pls quit'
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
`dt stream ...` and read lines manually (`rl`). Here's a quick example:
>
>     seq 100 | dt 'stream   [ rl  red pl g ] \r def   [ rl  green pl r ] \g def   r'

Let's do some simple filtering. Here instead of a pipe we'll just set stdin to
a file to avoid earning [_that_ award](https://porkmail.org/era/unix/award). On
my computer, there's a file at `/usr/share/dict/words` that has a dictionary of
the English language.

```
$ dt stream \
    '[rl \w:    [w pl]    w "duct" starts-with?   do?]' \
    loop \
    < /usr/share/dict/words
duct
duct's
ductile
ductility
ductility's
ducting
ductless
ducts
```

Those are some words that start with "duct" all right.

Here we bind a line at a time to `w` and use `"duct" starts-with?` to determine
if a line from our dictionary starts with "duct". We use `do?` to conditionally
execute the `[w pl]` when it should.

> **New to the shell?** The `\` characters at the end of the lines tell the
executing shell (bash, zsh, whatever; not dt!) the next line is part of the same
command.
>
> We are quoting some of the dt code because shells typically have special
processing for characters like `[`, `]`, `?`, and `*`. Quoting in single quotes
also helps avoid the need to escape the `\` and `"` characters.
>
> When in doubt, add more ~~duct tape~~ _quotation marks_!

On my machine, the great "duct" search above took about 1.6 seconds. Let's try
that same thing again without using the `stream [<etc>...] loop` pattern.

```
$ dt '["duct" starts-with?]' filter pls \
    < /usr/share/dict/words
```

Oof, that was more succinct, but much slower. It took about 30 seconds on my
machine. In this case we have a somewhat large file (on my machine, 123k lines)
which isn't the hugest thing in the world, but it's expensive to operate on the
whole thing all at once -- streaming line by line is much less expensive.

# Aliasing

TODO

# Shebangs

TODO

