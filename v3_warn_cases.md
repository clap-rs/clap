These are cases which could be detected by `clap`, and potentially warn at compile time. Maybe even with a nice web URL to more info.

# Abrv. Syntax

First, a little about the syntax to understand the setup of rule. This syntax will make each rule more concise so we don't have to write an entire `clap` definition.

## Arg Types

* `--opt` is an option
* `--flag` is a flag
* `#` (i.e. `1` or `2`) is a positional argument
* `val` is an option value

## Modifiers

Can be used on `val` or Arg types

* `*`: zero or more
* `+`: one or more
* `?`: zero or one
* `{#,#}`: # to # times (i.e. `{1,4}` is one to four times)
* `<>`: required
* `=`: requires equals
* Ends in `,`: requires delimiter
* `(foo,bar)`: values can only be `foo` or `bar`

# --opt val? 1

## Ambiguous Uses

```
$ prog --opt foo
# is foo option val or positional?
```

## Non-Ambiguous Uses

```
$ prog 1 --opt
$ prog --opt -- 1
```

## Fixes

### Require equals on `--opt`

```
$ prog --opt foo
# foo is positional

$ prog --opt=val foo
```

# --opt val+ 1

## Ambiguous Uses

```
$ prog --opt foo bar
# is bar option val or positional?
```

## Non-Ambiguous Uses

```
$ prog 1 --opt val
$ prog --opt val -- 1
```

## Fixes

### `--opt` only one val per occurrence

```
$ prog --opt foo bar
# bar is positional

$ prog --opt val bar --opt foo
# bar is positional
```

