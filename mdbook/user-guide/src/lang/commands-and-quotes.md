# Commands and Quotes

## Commands

You tell dt what to do by giving it commands.

From a fresh install of dt with no other files loaded, you'll get built-in
commands _(implemented in [Zig](https://ziglang.org))_ and commands from
defined in dt as quotes. Together these make up [the dt standard library](./stdlib.md).

If you want to know what commands are defined, you can use the `defs` command,
which will produce a [quote](#quotes) of all defined command names. You can
see what a command does by using the `usage` command.

To see all defined commands and their usage at the same time, you can use the
`help` command.

```
❯ dt help quit
%	( <a> <b> -- <c> ) Modulo two numeric values. In standard notation: a % b = c
*	( <a> <b> -- <c> ) Multiply two numeric values.
+	( <a> <b> -- <c> ) Add two numeric values.
-	( <a> <b> -- <c> ) Subtract two numeric values. In standard notation: a - b = c
...	( <original> -- ? ) Unpack a quote.
<etc>
```

The above is truncated, but the format here is:

1. The command name, for example `%`

2. The "stack effect" on dt's state, for example `( <a> <b> -- <c> )` which
   indicates it will take as input the two most-recent values as input (`<a>`
   and `<b>`) and produce one new value (`<c>`). The syntax for effects may be
   refined over time. A `?` in a stack effects indicates an unknowable effect,
   it could be 0, 1, or many values.

3. A description that tries to be helpful. It's doing its best!

> **Everything can be considered a command.** Semantically, everything that dt
code defines is a command regardless of how it may be implemented. If you write
`5` it's a command to dt to produce an integer value of 5. If you write `[`
it's a command to dt to begin a quote and `]` is a command to end a quote.
>
> This isn't just theory mumbo-jumbo, it's practical. Everything in dt is
always strictly evaluated left to right. There is no forward parsing, no
function lifting, and there is no fancy grammar with lookaheads. Given enough
time, it's always possible to analyze a dt program and understand (IO aside)
exactly what the state should be based on the code that precedes it.

## Deferred Commands, Quotes, and Defining Commands

Quoted code is parsed as-is. Booleans, numbers, and Strings are parsed normally
and any commands in a quote are not immediately executed.

There are two forms of delayed execution:

1. A deferred command, which is a `\` character prefixed on a command name.

2. A quote, which is an opening bracket `[` and a closing bracket `]` with any
   number of values and commands in between.

### `def`

The most common use of both of these forms is in defining new commands:

```
» [ "Hello world!" pl ] \greet def
» greet
Hello world!
```

Here we have quoted `"Hello world!" pl` which does not immediately execute, and
the deferred the command `greet`. We use it like you'd use a "symbol" in other
languages, and `def` to define it as a new command.

`def` takes an action and a name, and defines a command that can be used for the
rest of the calling scope. If the action is a deferred command, using the new
command in the future will execute the deferred command. If the action is a
value like a boolean, number, or string, using the command will produce that
value.

### `define`

If you'd like to define a command and its usage as well, use the longer
`define` command like so:

```
» [ "Hello world!" pl ]
» "( -- ) Greet the whole world."
» \greet define
» greet
Hello world!
» \greet usage pl
( -- ) Greet the whole world.
```

### `:`

If you'd like to bind one or more values to a command name, _without_ executing
them when the command name is used, use the `:` command.

```
» 1 \a:
» a pl
1
» \pl [b]:
» b pl
\pl
» # Unlike def, : also works for multiple terms. It binds left-to-right
» 3 [4] "five" [c d e]:
» c pl
3
» d pl
[ 4 ]
» e pl
five
```

### `do`

To immediately execute a deferred command or a quote, use `do`.

```
» [ "Hi friend." pl ] do
Hi friend.
```

### `alias`

To copy one command to another, use `alias`. This also copies the usage
instructions.

```
» \pls \PLZ alias
» [ "Hello" "my "friend" ] PLZ
Hello
my
friend
```

This can be used in other scopes to keep the prior definition around but
redefine the canonical definition.

### Scoping

`def` and `:` bindings are scoped to the calling context and sub-contexts.

This means you can have local definitions that don't escape or redefine things
they don't intend to.

```
» [ "Hi friend." \greeting:   greeting pl ] do
Hi friend.
» greeting
warning(dt.handleCmd): Undefined: greeting
```

This also means you can locally "shadow" a definition that already exists and
not worry about that definition leaking out.

```
» 5 \n:
» [ "N" \n:   n pl ] do    
N
» n pl   
5
```
