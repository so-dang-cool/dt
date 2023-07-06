![License](https://img.shields.io/github/license/booniepepper/dt)

# `dt`

It's duct tape for your unix pipes. A shell-friendly concatenative functional
programming language for when you don't have a better tool.

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
the file.

A naive tee implementation:

`tee.dt`

```
#!/usr/bin/env dt

get-lines unlines   \stdin :
get-args pop   \file :

stdin pl
stdin file writef
```

Then use like:

```
cat wish-list | sed 's/red/green/g' | tee.dt new-wish-list
```

## Explore the REPL

```
$ dt
dt 0.10.0
» # Comments start with #
» 
» 1 1 + println
2
» 
» # "p" is print, "pl" is print line, "pls" is print lines (i.e. of a list of values)
» # Let's define a command that consumes a value, prints it, then returns its double
» 
» [ \n :   n p " " p   n 2 *] \print-and-double def
» 
» 
» # And let's do it... 7 times!
» 
» 1 \print-and-double 7 times   pl
1 2 4 8 16 32 64 128
» 
» 
» # Also there are conditions
» 
» ["hi" pl] false ?   ["bye" pl] true ?
bye
```

## Downloads

Download assets from [releases](https://github.com/booniepepper/dt/releases/) and
put them somewhere on your PATH.

An installation script will come soon.

## Building from source

To build from source, clone the repo and run `./build help` for instructions.
The project currently builds with Zig 0.11.+ and a recent Cargo toolchain. The
resulting binary for your machine will be produced in `./zig-out/bin/dt`

## Credits

Shared as open source software, distributed under the terms of [the 3-Clause BSD License](https://opensource.org/license/BSD-3-clause/).

A side quest of J.R. Hill | https://so.dang.cool | https://github.com/booniepepper
