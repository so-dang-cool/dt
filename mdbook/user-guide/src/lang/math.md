# Math

Coming soon...

In the meantime, here are some relevant commands:

```
%	( <a> <b> -- <c> ) Modulo two numeric values. In standard notation: a % b = c
*	( <a> <b> -- <c> ) Multiply two numeric values.
+	( <a> <b> -- <c> ) Add two numeric values.
-	( <a> <b> -- <c> ) Subtract two numeric values. In standard notation: a - b = c
/	( <a> <b> -- <c> ) Divide two numeric values. In standard notation: a / b = c
abs	( <a> -- <b> ) Determine the absolute value of a number.
divisor?	( <a> <b> -- <bool> ) Determine if a number a is evenly divisible by number b.
eq?	( <a> <b> -- <bool> ) Determine if two values are equal. Works for most types with coercion.
even?	( <a> -- <bool> ) Determine if a number is even.
gt?	( <a> <b> -- <bool> ) Determine if a value is greater than another. In standard notation: a > b
gte?	( <a> <b> -- <bool> ) Determine if a value is greater-than/equal-to another. In standard notation: a ≧ b
help	( -- ) Print commands and their usage
inspire	( -- <wisdom> ) Get inspiration.
lt?	( <a> <b> -- <bool> ) Determine if a value is less than another. In standard notation: a < b
lte?	( <a> <b> -- <bool> ) Determine if a value is less-than/equal-to another. In standard notation: a ≦ b
neq?	( <a> <b> -- <bool> ) Determine if two values are unequal.
odd?	( <a> -- <bool> ) Determine if a number is odd.
sort	( [<a>] -- [<b>] ) Sort a list of values. When values are of different type, they are sorted in the following order: bool, int, float, string, command, deferred command, quote.
to-float	( <a> -- <float> ) Coerce a value to a floating-point number.
to-int	( <a> -- <int> ) Coerce a value to an integer.
to-string	( <a> -- <string> ) Coerce a value to a string.
```
