# What to Expect

There are the best tools for the job, and then there's duct tape.

* Sometimes you can't afford the best tool for the job
* Sometimes it's a hassle to use the best tool for the job
* Sometimes beauty and elegance is just not the top priority

And if you've got some duct tape... well that might be all you need. It can be
cheap, easy to use, and good enough for the something-or-other that needs to
get done. It's not a _best-in-class_ tool, but it's a _best-in-many-situations_
kind of tool.

> Is duct tape the best tool for any job at all? _Even for patching ducts,_
you'd be better off getting an HVAC sealing tape made of aluminum foil.
Well [it was originally "duck tape"](../misc/duct-tape.md) but anyways...

**dt is born from a similar philosophy.** It's intended to fill _so many
roles_ by being a very malleable substance that's practical for small-to-medium
use cases. In some cases it will be temporary, in other cases it will be just
good enough, and in the rest of the cases, it will have been intended to be
temporary and turned out to be just good enough after all. (Maybe this is true
of all software anyway, but it's especially true of dt.)

## dt is a malleable tool

Duct tape is malleable in that you can make it adhere to all sorts of surfaces:
smooth or bumpy, flat or curved, all sorts of materials, you get the picture.

dt is malleable in that you can reach pretty high levels of abstraction, and
even update the definitions of existing commands to your heart's content.

Malleable things are really not meant to build huge structures. Pillow forts are
lots of fun, but you don't see them on actual battlefields for a few good
reasons. Just think of the amount of laundry you'd have to do!

As a quick example of how malleable dt is, you could launch a REPL by
[installing dt](install.md) and running it. (If you're following along, use
`Ctrl+d` or `command+d` to quickly exit.)

```
$ dt
dt 1.x.x
Remember, you may have to grow old, but you don't have to mature.
» 
```

What it's doing here isn't much of a secret. (This is open source software after
all!) dt is printing "dt" and its `version`, printing an inspirational quote
(`inspire`), and then running a `repl`.

Defining commands is cheap in dt, and all commands are lazily evaluated every
time they're executed. Let's redefine the `version` command with some dt code
in the arguments for kicks. We'll look at the syntax later on in the tutorial.

```
$ dt '9001 \version:'
dt 9001
If it ain't broke, you're not trying.
»
```

Welcome to the future, I guess!

## dt is straightforward

All code in dt is straightforward. This doesn't mean you can't make horribly
impossible-to-understand code, but is very literal: dt only ever evaluates
a line of code from left-to-right. If dt gets multiple lines of code, they're
evaluated from top-to-bottom. There is _no_ forward parsing, there are _zero_
fancy constructs for definitions, conditions, or loops, for example. There are
no keywords that put dt in some special mode.

(None of those things are bad, they're just things that dt does not do.)

The only things that dt can work with are values and commands that were defined
already. The only way to get anything that feels like a lookahead is to have a
deferred command (either bare or in a quote) that gets defined later. If you
try to execute a command before it's defined, you'll simply get an error.

`def greet [ println "hello" ]` is fine for some other language. In dt, though,
this would be so many StackUnderflow errors. Everything flows _left-to-right,_
with no exceptions. That's why we will flip the world backwords, and proudly say
`["hello" println] \greet def`. In dt `println` can't know what to print unless
a value is already present. Likewise `def` is just another command. It can't
know the body (`["hello" println]`) or the command name (here a deferred
`greet`) unless they _precede_ it.

> Don't get arrogant about it, but yes it's true, to work straightforward is
also to work backward. How can that be? Well it's all perspective, of course;
maybe it's the rest of the world that's been backward all this time?

