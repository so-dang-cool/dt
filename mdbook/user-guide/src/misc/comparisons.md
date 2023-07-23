# Comparisons with Other Languages

## AWK

AWK is a major inspiration for dt. Not only am I (J.R.) a user, I'm a huge fan.
This is fantastic software.

I recommend listening to the wealth of guidance that Brian Kernighan has put
out specific to AWK, but also for C and Go, and his many other books and
presentations. He's a gifted teacher and speaker. Alfred Aho and Peter
Weinberger are also fascinating, influential people. The book _Masterminds of
Programming_ contains many enlightening interviews with programming language
designers, including an interview with each of the AWK trio.

Both AWK and dt can be considered primarily DSLs for text processing. They
differ in strategy and paradigm, and much of this is informed by the eras they
were conceived in. AWK was concieved in an era of computational constraint,
where dt was conceived in an era of computational abundance.

| Aspect | AWK | dt |
|--------|-----|----|
| Default paradigm | Conditionally execute code on the lines of input text that match a regular expression. | Process all lines as string data, and allow general-purpose programming. |
| Everything is a... | segment of text. | sequence of commands. |
| Primary data structure | Associative Array (aka HashMap) | Quote (aka Stack) |
| Relation to text | It's very strongly modeled. Whitespace is a default delimiter, tokens from a line are bound to positional identifiers. | Text is string data, which can be parsed into other types as needed. |
| Design goals | Ease of use, soundness of implementation, utility. (from Aho) | Ease of use, utility, hackability, and fun. |

## Forth

Forth is another major inspiration for dt. Chuck Moore is at least [144 years
ahead of his time](https://www.youtube.com/watch?v=0PclgBd6_Zs), and Leo Brodie
informs many stylistic choices of dt.

Forth has a minimalistic ethos that to me (J.R.) has a feeling of "Why would I
need more than this?" Forth does not require even so much as an operating
system. It can be as close to the metal as you can imagine: the core of a Forth
is often implemented directly in assembly or machine code. The semantics of
Forth's deepest code maps directly to machine instructions and the only builtin
data type, if you can even call it a type, is the underlying hardware's word of
memory.

Before moving on, see also:

* [jonesforth.S](https://github.com/nornagon/jonesforth/blob/master/jonesforth.S)
* [Assembly Nights](https://ratfactor.com/assembly-nights)

Now... Forth does not have to be _so_ direct in so many ways. There are
implementations built in other languages like C, and targets that are higher
level like Linux Kernel processes, and those implementations will look somewhat
like dt.

Some similarities will be obvious after using both. They use a similar ordering
of operations, and many Forth-isms are present in dt.

Perhaps the biggest difference is that Forth is compiled, and when compiling a
new "word" (Forth's term for a procedure) all references to other words are
compiled directly to the address in memory of the actions it performs. Forth
does have ways to be more dynamic, but it's not the primary interaction. On the
other hand, dt is never compiled; all "commands" (dt's term for a procedure)
are always resolved each time they're needed. This gives dt a far more
malleable and hackable form, with the tradeoff that it will not achieve the raw
performance of Forth.

Another major difference is dt has a more Lispy approach in general. It's more
common that things will be structured as lists, there's more support for
first-class functions, and all parsing and delayed computation is _always_
understood from left-to-right.

## Other languages

### Japanese

J.R. here. My family is half-Japanese. I was born in Guam, and my wife is a
native to Kyuushuu. We speak this language at home every day. (My wife and kids
speak better than me, though!)

English most often uses a subject-verb-object (SVO) grammatical ordering: "The
cow says moo." Sometimes we'll use an imperative verb-object-subject (VOS)
form: "Say moo, cow." In the most popular programming languages, which are most
influenced by English, we find an SVO style like `Cow.say("moo")` which turns
out linguistically to be fairly declarative, even though we're trying to be
imperative and tell that `Cow` what to do and when!

Japanese uses a subject-object-verb (SOV) grammatical ordering: 「牛がモーと鳴く」
or "The cow moo says." It's also a language with a culture of active listening;
context is often completely elided and you really have to "read the air" in
more ways than one. In practice, both in writing but much much more in spoken
conversations, many full sentences are only a subject or an object or a verb,
or even less! ね。

> As a side note, [SOV word order](https://en.wikipedia.org/wiki/Subject%E2%80%93object%E2%80%93verb_word_order)
is the most common word order on Earth by proportion of natural languages.
It's not most common by population of living speakers, though! That will go to
SVO. But in any case, it's been pretty dang common in human history; people can
speak and think this way!

A lot of this reminds me of concatenative languages, and I think has probably
trained my brain to think in a way that's receptive to concatenative languages.
Not just the ordering, but the way that so much can be left unsaid.

### Haskell, Joy, Factor

Haskell taught me to think without mutation, and was my gateway to language
families like Lisp and ML.

Joy gave birth to the "concatenative programming" name, and solidified it as a
way to do functional programming. Writing on Joy from Martin von Thun and Brent
Kerby in particular helped me think through a lot of design decisions.

Factor taught me just how far concatenative programming can go. It is the most
practical of all the concatenative languages out there, and it's extremely fun
to use the whole tool chain. Its IDE is fantastic, and the choices it makes on
what it borrows from who (Especially: Forth, Common Lisp, SmallTalk) make for a
really unique experience.

