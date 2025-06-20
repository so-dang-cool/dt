# Larry Wall

## [_Diligence, Patience, and Humility_](https://www.oreilly.com/openbook/opensources/book/larry.html)

> The yinyang represents a dualistic philosophy, much like The Force in Star
Wars. You know, how is The Force like duct tape? Answer: it has a light side, a
dark side, and it holds the universe together. I'm not a dualist myself,
because I believe the light is stronger than the darkness. Nevertheless, the
concept of balanced forces is useful at times, especially to engineers. When an
engineer wants to balance forces, and wants them to stay balanced, he reaches
for the duct tape.

---

> I have upon more than one occasion been requested to eject someone from the
Perl community, generally for being offensive in some fashion or other. So far
I have consistently refused. I believe this is the right policy. At least, it's
worked so far, on a practical level. Either the offensive person has left
eventually of their own accord, or they've settled down and learned to deal
with others more constructively. It's odd. People understand instinctively that
the best way for computer programs to communicate with each other is for each
of the them to be strict in what they emit, and liberal in what they accept.
The odd thing is that people themselves are not willing to be strict in how
they speak and liberal in how they listen. You'd think that would also be
obvious. Instead, we're taught to express ourselves.

---

> But I have to tell you that I don't evaluate the success of Perl in terms of
how many people like me. When I integrate these curves, I count the number of
people I've helped get their job done.

# Edsger W. Dijkstra

## [The humble programmer](https://www.cs.utexas.edu/users/EWD/ewd03xx/EWD340.PDF)

> As an aside I would like to insert a warning to those who identify the
difficulty of the programming task with the struggle against the inadequacies
of our current tools, because they might conclude that, once our tools will be
much more adequate, programming will no longer be a problem. Programming will
remain very difficult, because once we have freed ourselves from the
circumstantial cumbersomeness, we will find ourselves free to tackle the
problems that are now well beyond our programming capacity.

## [How do we tell truths that might hurt?](https://www.cs.utexas.edu/users/EWD/ewd04xx/EWD498.PDF)

> The tools we use have a profound (and devious!) influence on our thinking
habits, and, therefore, our thinking abilities.

---

> It is practically impossible to teach good programming to students that have
had a prior exposure to BASIC: as potential programmers they are mentally
mutilated beyond hope of regeneration.

_Note: [My](https://github.com/so-dang-cool) first language was BASIC._

## [The end of Computing Science?](https://www.cs.utexas.edu/users/EWD/ewd13xx/EWD1304.PDF)

> I would therefore like to posit that computing's central challenge, viz. "How
not to make a mess of it", has <u>not</u> been met. On the contrary, most of
our systems are much more complicated than can be considered healthy, and are
too messy and chaotic to be used in comfort and confidence. The average
customer of the computing industry has been served so poorly that he expects
his system to crash all the time, and we witness a massive world-wide
distribution of bug-ridden software for which we should be deeply ashamed.

# Roberto Ierusalimschy

## [Programming in Lua (5.x) 16.4 – Privacy](https://www.lua.org/pil/16.4.html)

> But this also reflects some basic design decisions behind Lua. Lua is not
intended for building huge programs, where many programmers are involved for
long periods. Quite the opposite, Lua aims at small to medium programs, usually
part of a larger system, typically developed by one or a few programmers, or
even by non programmers.
>
> ...
>
> Nevertheless, another aim of Lua is to be flexible, offering to the
programmer meta-mechanisms through which she can emulate many different
mechanisms.

_Note: I believe Lua has been largely (though comparatively quietly) successful
because its goals were not only _achieved_ but also a _valuable_ combination._

## [The Evolution of Lua](https://onlinelibrary.wiley.com/doi/abs/10.1002/(SICI)1097-024X(199606)26:6%3C635::AID-SPE26%3E3.0.CO;2-P)

Co-authors: Luiz Henrique de Figueiredo, Waldemar Celes

> One thing that has worked really well was the early decision (made in Lua
1.0) to have tables as the sole data-structuring mechanism in Lua. Tables have
proved to be powerful and efficient. The central role of tables in the language
and in its implementation is one of the main characteristics of Lua. We have
resisted user pressure to include other data structures, mainly “real” arrays
and tuples, first by being stubborn, but also by providing tables with an
efficient implementation and a flexible design. For instance, we can represent
a set in Lua by storing its elements as indices of a table. This is possible
only because Lua tables accept any value as index.

---

> Despite our “mechanisms, not policy” rule — which we have found valuable in
guiding the evolution of Lua — we should have provided a precise set of
policies for modules and packages earlier. The lack of a common policy for
building modules and installing packages prevents different groups from sharing
code and discourages the development of a community code base. Lua 5.1 provides
a set of policies for modules and packages that we hope will remedy this
situation.

_Note: I find this interesting, but also wonder at the outcome a larger Lua
community would have been. Could it have been the "Go" before Go? Would a
larger ecosystem prevent or cause positive change of the language? The effects
of functional purists (who came, dictated, then left) on the Scala ecosystem
and its community, fractures and abandonware, come to mind._
