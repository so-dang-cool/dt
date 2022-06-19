# Rail

A very experimental [concatenative](https://concatenative.org/wiki/view/Concatenative%20language)
programming language.

```
$ rail
rail 0.3.0
> 1 1 + .s
2
> [ 1 + ] "inc" def
> 1 .s [ inc .s ] 3 times
1
2
3
4
```

## Installation

```shell
$ cargo install rail-lang
```

## Credits

Available under GPL v2.

A side quest of J.R. Hill | https://so.dang.cool | https://github.com/hiljusti