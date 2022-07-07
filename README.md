
![License](https://img.shields.io/github/license/hiljusti/rail)
![Lines of code](https://img.shields.io/tokei/lines/github/hiljusti/rail)
![GitHub repo size](https://img.shields.io/github/repo-size/hiljusti/rail)

# Rail

A straightforward programming language.

Rail is an experimental [concatenative](https://concatenative.org/wiki/view/Concatenative%20language)
programming language. It is under wild development and zero stability between
versions is guaranteed.

```
$ rail i
rail 0.11.2
> 1 1 + .s
2
> [ 1 + ] "inc" def
> 1 .s [ inc .s ] 3 times drop
1
2
3
4
> [ [ false ] [ "goodbye" ] [ true ] [ "hello" ] ] opt .s
"hello"
```

## Installation

```shell
$ cargo install rail-lang
```

## Credits

Available under GPL v2.

A side quest of J.R. Hill | https://so.dang.cool | https://github.com/hiljusti
