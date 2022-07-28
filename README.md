
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
rail 0.18.1
> 1 1 + p
2
> [ 1 + ] "inc" def
> 1 [ dup p inc ] 2 times p
1
2
3
> [ [ false ] [ "bye" ] [ true ] [ "hi" ] ] opt p
"hi"
```

## Installation

For now...

```shell
$ cargo install rail-lang
$ railup bootstrap
```

## Credits

Available under GPL v2.

A side quest of J.R. Hill | https://so.dang.cool | https://github.com/hiljusti
