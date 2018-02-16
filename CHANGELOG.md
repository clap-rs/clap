# v0.2.3 (2018-02-16)

* An empty line in a doc comment will result in a double linefeed in the generated about/help call by [@TeXitoi](https://github.com/TeXitoi)

# v0.2.2 (2018-02-12)

* Fix [#66](https://github.com/TeXitoi/structopt/issues/66) by [@TeXitoi](https://github.com/TeXitoi)

# v0.2.1 (2018-02-11)

* Fix a bug around enum tuple and the about message in the global help by [@TeXitoi](https://github.com/TeXitoi)
* Fix [#65](https://github.com/TeXitoi/structopt/issues/65) by [@TeXitoi](https://github.com/TeXitoi)

# v0.2.0 (2018-02-10)

## Breaking changes

### Don't special case `u64` by [@SergioBenitez](https://github.com/SergioBenitez)

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

This feature was surprising as shown in [#30](https://github.com/TeXitoi/structopt/issues/30). Using the `parse` feature seems much more natural.

### Change the signature of `Structopt::from_clap` to take its argument by reference by [@TeXitoi](https://github.com/TeXitoi)

There was no reason to take the argument by value. Most of the StructOpt users will not be impacted by this change. If you are using `StructOpt::from_clap`, just add a `&` before the argument.

### Fail if attributes are not used by [@TeXitoi](https://github.com/TeXitoi)

StructOpt was quite fuzzy in its attribute parsing: it was only searching for interresting things, e. g. something like `#[structopt(foo(bar))]` was accepted but not used. It now fails the compilation.

You should have nothing to do here. This breaking change may highlight some missuse that can be bugs.

In future versions, if there is cases that are not highlighed, they will be considerated as bugs, not breaking changes.

### Use `raw()` wrapping instead of `_raw` suffixing by [@TeXitoi](https://github.com/TeXitoi)

The syntax of raw attributes is changed to improve the syntax.

You have to change `foo_raw = "bar", baz_raw = "foo"` by `raw(foo = "bar", baz = "foo")` or `raw(foo = "bar"), raw(baz = "foo")`.

## New features

* Add `parse(from_occurrences)` parser by [@SergioBenitez](https://github.com/SergioBenitez)
* Support 1-uple enum variant as subcommand by [@TeXitoi](https://github.com/TeXitoi)
* structopt-derive crate is now an implementation detail, structopt reexport the custom derive macro by [@TeXitoi](https://github.com/TeXitoi)
* Add the `StructOpt::from_iter` method by [@Kerollmops](https://github.com/Kerollmops)

## Documentation

* Improve doc by [@bestouff](https://github.com/bestouff)
* All the documentation is now on the structopt crate by [@TeXitoi](https://github.com/TeXitoi)

# v0.1.7 (2018-01-23)

* Allow opting out of clap default features by [@ski-csis](https://github.com/ski-csis)

# v0.1.6 (2017-11-25)

* Improve documentation by [@TeXitoi](https://github.com/TeXitoi)
* Fix bug [#31](https://github.com/TeXitoi/structopt/issues/31) by [@TeXitoi](https://github.com/TeXitoi)

# v0.1.5 (2017-11-14)

* Fix a bug with optional subsubcommand and Enum by [@TeXitoi](https://github.com/TeXitoi)

# v0.1.4 (2017-11-09)

* Implement custom string parser from either `&str` or `&OsStr` by [@kennytm](https://github.com/kennytm)

# v0.1.3 (2017-11-01)

* Improve doc by [@TeXitoi](https://github.com/TeXitoi)

# v0.1.2 (2017-11-01)

* Fix bugs [#24](https://github.com/TeXitoi/structopt/issues/24) and [#25](https://github.com/TeXitoi/structopt/issues/25) by [@TeXitoi](https://github.com/TeXitoi)
* Support of methods with something else that a string as argument thanks to `_raw` suffix by [@Flakebi](https://github.com/Flakebi)

# v0.1.1 (2017-09-22)

* Better formating of multiple authors by [@killercup](https://github.com/killercup)

# v0.1.0 (2017-07-17)

* Subcommand support by [@williamyaoh](https://github.com/williamyaoh)

# v0.0.5 (2017-06-16)

* Using doc comment to populate help by [@killercup](https://github.com/killercup)

# v0.0.3 (2017-02-11)

* First version with flags, arguments and options support by [@TeXitoi](https://github.com/TeXitoi)
