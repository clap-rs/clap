# How to Contribute

See the [clap-wide CONTRIBUTING.md](../CONTRIBUTING.md).  This will contain `clap_derive` specific notes.

## Derive Gotchas

- Always prefix generated variables with `__clap_` to minimize clashes with the user's variables, see [#2934](https://github.com/clap-rs/clap/issues/2934).
- Prefer the path `clap` over `::clap` to allow users to re-export clap, see [#2258](https://github.com/clap-rs/clap/pull/2258).
- Prefer substituting variable names to avoid problems with `macro_rules`, see [#2823](https://github.com/clap-rs/clap/pull/2823).
- Prefer `::std::result::Result` and `::std::option::Option`, see [#3092](https://github.com/clap-rs/clap/pull/3092).
- Put whitespace between `#quoted #variables`.
