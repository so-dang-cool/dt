# Shebangs

dt is shebang-friendly. When you have something that gets too long for a
one-liner, or you have something you want to save and not type out a lot, you
can put that into a shebang file.

> Note: It's been more than 20 years since I've been familiar with Windows. I
_think_ the equivalent in a Windows environment these days would be writing a
[PowerShell Script Module](https://learn.microsoft.com/en-us/powershell/scripting/developer/module/how-to-write-a-powershell-script-module?view=powershell-7.3)
with a `.psm1` extension. If we get Windows support and anyone knows a good
pattern let us know!

## An introduction

If you're familiar with Unix-like systems, you can skip this section.

The "Hash Bang" -- `#!` -- more often called "shebang" is a magic prefix for
_executable_` files that tell operating systems like Linux, BSD, or OSX to use
a specific interpreter to interpret the file.

More specifically, let's write a file named `greet` with contents like:

```
#!/usr/bin/env dt

"Hello world!" println
```

Mark it executable (`chmod +x ./greet`) and you should be ready to run it:

```
$ ./greet
Hello world!
```

If you've been following along in the past sections, you've now greeted the
world at least 3 times. Hope the world has noticed you by now!

The `#!` says the rest of the line is something to execute as a process.
`/usr/bin/env dt` helps locate a program called `dt` and execute _it_ with the
remaining arguments. `dt` in turn will get the filename as an argument and
start to interpret the file contents.

If you had installed `dt` in a place like `/home/me/.local/bin/dt` you could
also do something like:

```
#!/home/me/.local/bin/dt

"Hello world!" println
```

Shared scripts tend to use the `/usr/bin/env` lookup just in case the install
location can't be known. I don't know about you, but I tend to share scripts,
even if it's just with my future self! Who knows what that guy will do.

Anyway the point here is that you can put dt code in a file, and that can be an
executable.

## Shebang scripting with dt

Let's start our shebang examples by making a couple naive implementations of
other common tools. (Of course: Do use the tools instead of these scripts!)
We'll skip a lot of the niceties like usage messages and flag parsing, and just
implement the core use case.

Let's create a `head.dt N` that can print the first N lines of its input. Put
the following in a file called `head.dt` and mark it as executable.

```
#!/usr/bin/env dt

shebang-args first \n:

[readln println] n times
```

We use `shebang-args` to get the arguments passed to the shebang file. This is
just the args of dt minus the first arg which is the script being interpreted.
We bound the first argument to `n` and then did a read/print loop `n` times.

It can be used similar to the classic `head -n N` pattern:

```
$ seq 100 | head.dt 5
1
2
3
4
5
```

Now let's create a `scream.dt` that can be an equivalent of `tr a-z A-Z`.

```
#!/usr/bin/env dt

[readln upcase println] loop
```

```
$ echo -e "i\ncan't\nhear\nyou" | scream.dt
I
CAN'T
HEAR
YOU
```

Ok folks I get it, it's short, but we're here to demonstrate, not to
optimize! The big thing to know is you can `loop` which is a "forever" kind
of iteration, and when the pipe completes, dt will exit gracefully.

> Note: Very short things also work as shell aliases. Here's a way to do the
same thing in POSIX conventions: `alias scream='dt stream [rl upcase pl] loop`

## Shebangs into REPLs

Another pattern that can be helpful is doing some pre-work like defining some
commands or reading files and then dropping into a REPL with that state.

We won't go into a ton of detail here, but we'll demonstrate with a custom
prompt and a single command definition.

```
#!/usr/bin/env dt

"MyShell 0.1" println

["/home/me/myshell.log" appendf]
"( s -- ) Save a string to the myshell log."
\log define

repl
```

