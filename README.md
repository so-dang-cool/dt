![License](https://img.shields.io/github/license/hiljusti/dt)

# `dt`

It's duct tape for your unix pipes. Use it when you don't have a better tool.

In the words of [Red Green](https://www.redgreen.com):

> Remember, it's only temporary... unless it works!

## For pipes:

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

## Running as an interactive shell:

`dt` is an experimental [concatenative](https://concatenative.org/wiki/view/Concatenative%20language)
programming language.

```
$ dtsh
dt 0.7.0

> 1 1 + print
2

> [[ n ]: n print " " print n 2 *] [print-and-double] def

> 1 [print-and-double] 7 times
1 2 4 8 16 32 64 

> [[false] ["bye"] [true] ["hi"]] ? println
hi
```

## Installation

```shell
$ cargo install dt-tool
$ dtup bootstrap
```

## Credits

Shared under GPL v2.

A side quest of J.R. Hill | https://so.dang.cool | https://github.com/hiljusti
