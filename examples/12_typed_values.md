You can use some convenience methods provided by clap to get typed values, so long as the
type you're converting into implements std::str::FromStr

This works for both single, and multiple values (multiple values returns a Vec<T>)

When getting a typed value, a Result is returned which allows you decide what to do upon a failure, whether exit, provide a
default value, etc. You have control. But it also means you have to write the code or boiler plate
to handle those instances.

For example, you could exit:
```bash
$ 12_typed_values 1 2 3
Sequence part 1 + 2: 3
Sequence part 2 + 2: 4
Sequence part 3 + 2: 5
len (10) + 2 = 12
$ 12_typed_values 1 2 3 four
? failed
error: Invalid value for 'seq': The argument 'four' isn't a valid value: invalid digit found in string
```

Or provide a fallback:
```bash
$ 12_typed_values 1 -l 3
Sequence part 1 + 2: 3
len (3) + 2 = 5
$ 12_typed_values 1 -l four
Sequence part 1 + 2: 3
len (10) + 2 = 12
```

Or you can have clap do the error reporting for you, see [15_custom_validator](15_custom_validator.md)
