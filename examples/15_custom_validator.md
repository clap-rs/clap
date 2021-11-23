You can define a function (or a closure) to use as a validator to argument values. The
function must accept a `&str` and return `Result<(), String>` where `Err(String)` is the
message displayed to the user.

```bash
$ 15_custom_validator input.png
The .PNG file is: input.png
$ 15_custom_validator input.txt
? failed
error: Invalid value for '<input>': the file format must be png.

For more information try --help
```

This is especially useful when using [custom types](12_types_values.md).
