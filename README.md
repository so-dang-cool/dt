
![License](https://img.shields.io/github/license/hiljusti/rail)
![Lines of code](https://img.shields.io/tokei/lines/github/hiljusti/rail)
![GitHub repo size](https://img.shields.io/github/repo-size/hiljusti/rail)

# Rail

A very experimental [concatenative](https://concatenative.org/wiki/view/Concatenative%20language)
programming language.

```
$ rail
rail 0.7.0
> 1 1 + .s
2
> [ 1 + ] "inc" def
> 1 .s [ inc .s ] 3 times
1
2
3
4
> drop [ [ 0 ] [ "goodbye" ] [ 1 ] [ "hello" ] ] opt .s
"hello"
```

## Installation

```shell
$ cargo install rail-lang
```

## Credits

Available under GPL v2.

A side quest of J.R. Hill | https://so.dang.cool | https://github.com/hiljusti
