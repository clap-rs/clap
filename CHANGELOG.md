# v0.2.0 (2018-02-XX)

## Breaking changes

### Don't special case `u64` by @SergioBenitez

If you are using a `u64` in your struct to get the number of occurence of a flag, you should now add `parse(from_occurrences)` on the flag.

For example
```rust
#[structopt(short = "v", long = "verbose")]
verbose: u64,
```
must be changed by
```rust
#[structopt(short = "v", long = "verbose", parse(from_occurrences))]
verbose: u64,
```

This feature was surprising as shown in #30. Using the `parse` feature seems much more natural.

### Change the signature of `Structopt::from_clap` to take its argument by reference by @TeXitoi

There was no reason to take the argument by value. Most of the StructOpt users will not be impacted by this change. If you are using `StructOpt::from_clap`, just add a `&` before the argument.

## New features

* Add `parse(from_occurrences)` parser by @SergioBenitez
* Support 1-uple enum variant as subcommand by @TeXitoi
* structopt-derive crate is now an implementation detail, structopt reexport the custom derive macro by @TeXitoi

## Documentation

* Improve doc by @bestouff
* All the documentation is now on the structopt crate by @TeXitoi

# v0.1.7 (2018-01-23)

* Allow opting out of clap default features by @ski-csis

# v0.1.6 (2017-11-25)

* Improve documentation by @TeXitoi
* Fix bug #31 by @TeXitoi

# v0.1.5 (2017-11-14)

* Fix a bug with optional subsubcommand and Enum by @TeXitoi

# v0.1.4 (2017-11-09)

* Implement custom string parser from either `&str` or `&OsStr` by @kennytm

# v0.1.3 (2017-11-01)

* Improve doc by @TeXitoi

# v0.1.2 (2017-11-01)

* Fix bugs #24 and #25 by @TeXitoi 
* Support of methods with something else that a string as argument thanks to `_raw` suffix by @Flakebi

# v0.1.1 (2017-09-22)

* Better formating of multiple authors by @killercup

# v0.1.0 (2017-07-17)

* Subcommand support by @williamyaoh

# v0.0.5 (2017-06-16)

* Using doc comment to populate help by @killercup

# v0.0.3 (2017-02-11)

* First version with flags, arguments and options support by @TeXitoi
