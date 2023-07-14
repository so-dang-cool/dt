# Exploring the REPL

Let's start by exploring the language to get a feel for it by opening a REPL
(Read-Eval-Print Loop, an interactive mode) and throw some commands in it.

_These instructions assume you've already [installed dt](./install.md)._

In a shell, just run dt:

```
$ dt
dt 1.x.x
Remember, I'm pulling for ya. We're all in this together!
» 
```

To exit the REPL, type `quit` or use an end-of-file code like `Ctrl+d` or
`command+d`.

> **Pro tip:** For a better REPL experience, it's also recommended to install
[rlwrap](https://github.com/hanslub42/rlwrap) and create an alias in your shell
of choice. Maybe something like `alias dtsh='rlwrap dt'` in `.bashrc` or
`.zsrhc` files, or `alias dtsh 'rlwrap dt' && funcsave dtsh` for fish.

From here on these docs will assume you are in a REPL context.

Code following the `»` prompt is input, and other lines are output. Copy or
type the code as written here, and feel free to test out other things. It's ok
if you make mistake, just open a new REPL and start fresh.


## Say hello! Printing, quotes, and definitions

Ok lightning tour, let's do this!

```
» "Hello world!" pls
Hello world!
```

`pls` is a [command](../lang/glossary.md#command) to print the most recent
value to standard output. In the example above we put one value into dt, the
string `"Hello world!"` and `pls` printed the single value as a single line.

We can also use [quotes](../lang/glossary.md#quote) to define lists of values
and us `pls` to print all of those too.

```
» [ "Hello" "you" "crazy" "world!" ] pls
Hello
you
crazy
world!
```

Getting back to strings, let's get a little more dynamic. First let's bind a
name, feel free to use your own here!

```
» "Harold" \name:
» name pls
Harold
```

Now let's define a `greet` command. We'll use some printing words we haven't
used before, `p` to print a single value as-is, and `nl` to add a newline.

```
» [ "Hello " p name p "!" p nl ] \greet def
» greet
Hello Harold!
```

We can also use `pl` to print a value and a newline at once.

```
» [ "Hello " p name p "!" p nl ] \greet def
» greet
Hello Harold!
```

We've used `:` to bind a single value to `name`, and used `def` to bind (and
re-bind!) the execution of a quote of values (including commands!) to `greet`.
Coming from other languages, `:` will fill the role of binding "variables" and
`def` will fill the role of defining "functions" or "procedures." In dt terms,
we'll call these both definitions of commands.

Let's change the name:

```
» "Bernice" \name:
» greet
Hello Bernice!
```

Here we can see that the `name` we referenced in our `greet` definition is a
lazily-evaluated lookup, and the second definition of `name` shadows the previous
definition going forward.

The re-definition gives a feel of mutation to the definition, but the
underlying value cannot be altered, only re-bound. Let's try it out with
`upcase` and `downcase` real quick:

```
» name upcase pls
BERNICE
» name downcase pls
bernice
» name pls
Bernice
```

> Here we learned to print with `p`, `nl`, `pl`, and `pls`. Use `pls` for
general cases, and the others when you need more control.
>
> Commands in a quote (between the `[` and `]` characters) have execution
> deferred.
>
> Finally we learned how to define new terms. Congratulations, you have already
written dt code! `:` can define a value as-is. (Later we'll see it can define
many values.) `def` defines the _execution_ of one or more values.


## Fibonacci! State, iteration, and conditions

The [Fibonacci sequence](https://en.wikipedia.org/wiki/Fibonacci_sequence) is a
fascinating pattern of numbers that naturally occurs in many places like the
growth of pineapples and flowers, or in how light refracts through transparent
surfaces.

Let's produce some numbers in the Fibonacci sequence. We'll start with two
integers, and use `.s` to check the state of dt.

```
» 0 1 .s
[ 0 1 ]
```

`.s` can be used to check the state of dt at any time. We can see that the
state of `dt` itself is a quote of values. The order is left-to-right just how
we typed it in.

Let's use the state of dt itself to produce a Fibonacci sequence. We'll start
by defining a command that can take two numbers from state, produce those two
numbers, and also produce their sum.

```
» .s
[ 0 1 ]
» [[a b]:   a   b   a b + ] \fib def
» fib .s
[ 0 1 1 ]
» fib .s
[ 0 1 1 2 ]
» fib fib fib .s
[ 0 1 1 2 3 5 8 ]
```

Neat! This is laying out the Fibonacci sequence in dt's main state. Maybe
someday we could 3d print a pineapple or something.

Above we used `:` to bind two values to the `a` and `b`. The first time we
used the command, `a` bound to `0`, `b` bound to `1`. We put them back in the
same order, and then also took `a` and `b` and added them.

The `+` operator is also a command, that takes two values from state and
produces their sum. `a b +` in dt is equivalent to `a + b` in standard math
notation.

> If you want to get nerdy about it (I do, just for a moment) this `a b +`
business is sometimes called "Reverse Polish Notation" in mathematics. In
linguistics it can be called a "Subject-Object-Verb Grammar" kind of like
Japanese or Latin. Bring these up if you want to impress your friends.
>
> Anyway, a more practical way to think about it is we're saying "here's `a`
and here's `b`" and then giving dt a `+` command. In dt there is no
look-ahead parsing, or recursive descent, or fancy grammar or anything.
Everything is interpreted sequentially left-to-right and top-to-bottom.

Let's make a few more!

```
» .s
[ 0 1 1 2 3 5 8 ]
» \fib 3 times .s
[ 0 1 1 2 3 5 8 13 21 34 ]
```

Ahh, good `times`! Here we used a `\` prefix to create a reference to a command
and then used `times` which takes some action, a number, and performs the
action a number of times.

So... now that we've got a bunch of these numbers what should we do with them?
For now let's just print them all out. Here are a couple ways we could do it.

1. Using `10 times` since we know how many exist...
    ```
    » \pl 10 times
    34
    21
    13
    8
    5
    3
    2
    1
    1
    0
    ```
    ...or,

2. Using a new command, `quote-all`, which will convert the state to a single quote in state.
    ```
    » .s
    [ 0 1 1 2 3 5 8 13 21 34 ]
    » quote-all .s
    [ [ 0 1 1 2 3 5 8 13 21 34 ] ]
    » pls
    0
    1
    1
    2
    3
    5
    8
    13
    21
    34
    ```

> Note that these both printed all our values, but did so in a different order.
`times` executed the `pl` command with the most-recent value in state 10 times.
`pls` (and other commands that operate on quotes) follow a left-to-right
ordering.

