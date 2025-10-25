This is an example of using figment in combination with clap.

  * First any default values are loaded in via serde
  * Next values are read in via a toml configuration file
  * Finally any command line arguments override any settings at a top level

For this to work the following libraries and features will be required.
```toml
[dependencies]
clap = { version = "*", features = ["derive"] }
figment = { version = "*", features = ["toml"] }
serde = { version = "*", features = ["serde_derive"] }
serde_default_utils = { version = "*", features = ["inline"] }
```

Help:
```console
$ figment --help

A demo showing the use of figment with clap

Usage: figment[EXE] [OPTIONS]

Options:
  -n, --name <NAME>    Name of the person to greet
  -c, --count <COUNT>  Number of times to greet - default value of 1
  -h, --help           Print help
```
*(version number and `.exe` extension on windows replaced by placeholders)*

A basic argument:
```console
$ figment --count 4

Name: HelloWorld
Count: 4
```

## Optional Fields

All fields must be optional / wrapped within an `Option<>`.    
This is to avoid clap reporting an error or missing option if an option is already set by figment or by serde's defailts
within the figment configuration file.

This means in practice for any required fields these need to be checked via custom validation.

## Setting default values

In order to set defaults for fields we can use the `serde_inline_default` crate.
By performing this at the serde level, we can inject default values prior to them being possibly set via figment
```rust
#[serde_inline_default(Some(1))]
```

## Serde skip serializing flag

This serde flag is necessary so that clap won't overwrite a setting if it determines it to be None
Which means the setting instead will fall back to ether the configuration file or default setting
```rust
#[serde(skip_serializing_if = "::std::option::Option::is_none")]
```
