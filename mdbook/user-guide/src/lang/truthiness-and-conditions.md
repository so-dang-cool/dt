# Truthiness

In dt many values can [coerce](./coersion.md) into other types. You can also
directly convert something to a boolean value with `to-bool`.

```
» "hello" to-bool pls
true
```

The rules for truthiness in dt are:

| type    | `true` when:          | `false` when:     |
|---------|-----------------------|-------------------|
| bool    | `true`                | `false`           |
| int     | non-zero positive     | zero or negative  |
| float   | non-zero positive     | zero or negative  |
| string  | not empty             | `""` empty string |
| command | defined               | undefined         |
| deferred command | defined      | undefined         |
| quote   | not empty             | `[ ]` empty quote |

## Conditions

Coming soon...
