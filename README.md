![License](https://img.shields.io/github/license/hiljusti/dt)
![Lines of code](https://img.shields.io/tokei/lines/github/hiljusti/dt)
![GitHub repo size](https://img.shields.io/github/repo-size/hiljusti/dt)

# `dt`

It's duck tape for your unix pipe.

`dt` is an experimental [concatenative](https://concatenative.org/wiki/view/Concatenative%20language)
programming language. It is under wild development and zero stability between
versions is guaranteed.

```
$ dtsh
dt 0.3.3

> 1 1 + print
2

> [ [ n ] -> n print " " print n 2 * ] "print-and-double" def

> 1 [ print-and-double ] 7 times
1 2 4 8 16 32 64 

> [ [ false ] [ "bye" ] [ true ] [ "hi" ] ] opt println
hi
```

## Installation

```shell
$ cargo install dt-tool
$ dtup bootstrap
```

## Credits

Available under GPL v2.

A side quest of J.R. Hill | https://so.dang.cool | https://github.com/hiljusti
