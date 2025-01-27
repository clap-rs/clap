# Change Log
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

<!-- next-header -->
## [Unreleased] - ReleaseDate

## [4.5.43] - 2025-01-27

### Fixes

- *(powershell)* Correctly escape backtick
- *(powershell)* Improve handling of empty help

## [4.5.42] - 2025-01-09

### Fixes

- *(fish)* Corectly generate `aot` completions for single-value `ValueEnum`s

## [4.5.41] - 2025-01-07

### Fixes

- *(elvish)* Avoid leaking env variables in dynamic completions

## [4.5.40] - 2024-12-17

### Fixes

- *(powershell)* Fix syntax in dynamic registration script

## [4.5.39] - 2024-12-16

### Fixes

- *(bash)* For AOT completions, de-duplicate when subcommand aliases are used

## [4.5.38] - 2024-11-13

## [4.5.37] - 2024-11-04

### Fixes

- *(dynamic)* Be compatible with package's MSRV

## [4.5.36] - 2024-10-29

### Fixes

- *(zsh)* Change `ValueHint::Unknown` to `_default` in static completions

## [4.5.35] - 2024-10-24

## [4.5.34] - 2024-10-24

### Features

- *(dynamic)* Show description in zsh

## [4.5.33] - 2024-10-08

### Features

- *(dynamic)* `SubcommandCandidates` support

## [4.5.32] - 2024-10-02

## [4.5.31] - 2024-10-02

### Compatibility

- *(dynamic)* `CompleteEnv::with_factory` now takes in a `Fn` instead of a `FnOnce`

## [4.5.30] - 2024-10-02

### Fixes

- *(dynamic)* Don't default to `ValueHint::AnyPath` but "no completion"

## [4.5.29] - 2024-09-20

### Features

- *(dynamic)* Change completion order to subcommands, positional values, flags
- *(dynamic)* When completing `-[TAB]`, prioritize shorts over longs
- *(dynamic)* De-duplicate completions that have the same result (longs, shorts, aliases)
- *(dynamic)* Group candidates from the same `CompletionCandidate::tag`
- *(dynamic)* Sort candidates within a `CompletionCandidate::tag` by their `display_order`

## [4.5.28] - 2024-09-17

### Compatibility

- *(dynamic)* The binary called when completing is now `std::env::args_os()[0]`, rather than `Command::name`

### Features

- *(dynamic)* Allow overriding the binary being completed and the binary being called for completions

### Fixes

- *(dynamic)* By default, remove wrappers around a binary when calling for completions

## [4.5.27] - 2024-09-17

### Fixes

- *(dynamic)* Fix completions for bash for at least some users

## [4.5.26] - 2024-09-05

### Features

- *(dynamic)* `allow_hyphen_values` support

## [4.5.25] - 2024-09-04

### Compatibility

- *(dynamic)* Removed `CompleteCommand`

### Fixes

- *(dynamic)* Take over ordering of Fish completions

## [4.5.24] - 2024-08-27

### Fixes

- *(dynamic)* Ensure a new enough `shlex` is required

## [4.5.23] - 2024-08-22

### Compatibility

- *(dynamic)* Rename `CompletionCandidate::get_content` to `CompletionCandidate::get_value`

## [4.5.22] - 2024-08-21

### Fixes

- *(dynamic)* Dir completions now include `.`

## [4.5.21] - 2024-08-21

### Features

- *(dynamic)* Add `ArgValueCompleter`, a more flexible `ArgValueCandidates`
- *(dynamic)* Add `PathCompleter`, a more flexible `ValueHint::*Path`

### Fixes

- *(dynamic)* Sort `ValueHint::*Path` results
- *(dynamic)* Preserve the users path for `ValueHint::*Path` results

## [4.5.20] - 2024-08-20

### Compatibility

- *(dynamic)* Renamed `CustomCompleter` to `ValueCandidates`
- *(dynamic)* Renamed `ArgValueCompleter` to `ArgValueCandidates`

## [4.5.19] - 2024-08-19

### Compatibility

- *(dynamic)* Renamed `dynamic` to `engine`

### Fixes

- *(dynamic)* *(bash)* Don't cause a completion to make the program un-runnabe
- *(dynamic)* *(zsh)* Don't cause a completion to make the program un-runnabe

## [4.5.18] - 2024-08-16

### Features

- *(dynamic)* Complete the last value in a delimited list

## [4.5.17] - 2024-08-16

### Compatibility

- *(dynamic)* Move `command` and `env` out of `dynamic`

### Fixes

- *(dynamic)* Increase `ArgValueCompleter` precedence over `ValueEnum`
- *(dynamic)* Move `command` and `env` out of `dynamic`
- Move pre-generated completions to `aot` module

## [4.5.16] - 2024-08-12

## [4.5.15] - 2024-08-12

### Compatibility

- *(dynamic)* Moved `dynamic::shells` to `dynamic::command`
- *(dynamic)* Guarded `dynamic::command` with `unstable-command` feature

### Features

- *(dynamic)* Added `env::CompleteEnv` application integration

## [4.5.14] - 2024-08-10

### Compatibility

- *(dynamic)* Renamed `CustomCompleter::completions` to `CustomCompleter::candidates`
- *(dynamic)* Renamed `Completer` to `shells::ShellCompleter`

### Features

- *(dynamic)* Add `shells::CompleteArgs::complete`
- *(dynamic)* Make `--register` optional
- *(dynamic)* Make `--shell` optional
- *(dynamic)* Add powershell support

### Fixes

- *(powershell)* Add missing option hyphens
- *(dynamic)* Expose `shells::CompleteArgs` and `shells::CompleteCommand` at top-level
- *(dynamic)* Improve help output

## [4.5.13] - 2024-08-08

### Features

- *(dynamic)* Added `ArgValueCompleter` for custom completions

### Compatibility

- *(dynamic)* `CompletionCandidate::visible` was renamed to `CompletionCandidate::hide`

## [4.5.12] - 2024-07-31

#### Features

- *(dynamic)* Support completing with `num_args`

## [4.5.11] - 2024-07-25

#### Features

- *(dynamic)* Add support for `-fbar` and `-f=bar` completions

## [4.5.10] - 2024-07-25

## [4.5.9] - 2024-07-23

### Features

- *(dynamic)* Support for zsh, elvis, powershell
- *(dynamic)* Complete `--option val[TAB]`
- *(dynamic)* Complete subcommand aliases
- *(dynamic)* Hide hidden flags, subcommands, values, and aliases, only completeing them if no visible variant is available

## [4.5.8] - 2024-07-11

### Fixes

- *(fish)* Nested subcommand completions

## [4.5.7] - 2024-06-28

### Fixes

- *(fish)* Allow completing positionals when subcommands are present

## [4.5.6] - 2024-06-19

## [4.5.5] - 2024-06-07

### Fixes

- *(zsh)* Don't fail or ignore options consumed by `_arguments`

## [4.5.4] - 2024-06-06

## [4.5.3] - 2024-06-06

## [4.5.2] - 2024-04-09

### Fixes

- *(bash)* Improve compatibility with older bash versions

## [4.5.1] - 2024-02-16

### Fixes

- Correctly handle completion descriptions with newlines

## [4.5.0] - 2024-02-08

### Compatibility

- Update MSRV to 1.74

## [4.4.10] - 2024-02-02

### Fixes

- *(bash)* Allow completing filenames with spaces

## [4.4.9] - 2024-01-22

### Features

- *(bash)* Add support file `ValueHint::FilePath`
- *(bash)* Add support file `ValueHint::DirPath`
- *(bash)* Don't add space for `ValueHint::Other`

## [4.4.8] - 2024-01-19

### Fixes

- *(bash)* be consistent in identifiers when custom bin names are used

## [4.4.7] - 2024-01-15

### Fixes

- *(unstable)* Don't have dynamic completions pollute the parent command's help output

## [4.4.6] - 2024-01-02

### Fixes

- Work with older Bash versions, particularly for MacOS

## [4.4.5] - 2023-12-27

### Documentation

- *(unstable)* Include / expand `dynamic` on docs.rs

## [4.4.4] - 2023-10-24

## [4.4.3] - 2023-09-28

## [4.4.2] - 2023-09-25

### Fixes

- *(unstable)* Complete positionals in subcommands

## [4.4.1] - 2023-09-07

### Fixes

- *(fish)* Properly escape `()` in descriptions

## [4.4.0] - 2023-08-24

### Compatibility

- Update MSRV to 1.70.0

### Features

- *(unstable)* Fish dynamic completion support

### Fixes

- *(unstable)* Manual control over sort order
- *(unstable)* Don't recursively report subcommands
- *(unstable)* Get bash support working again
- *(unstable)* Ensure shorts keep their leading `-`

## [4.3.2] - 2023-07-05

### Fixes

- *(powershell)* Attempt to allow completing `-s` separately from `-S`

## [4.3.1] - 2023-06-02

## [4.3.0] - 2023-05-19

## [4.2.3] - 2023-05-12

### Fixes

- *(zsh)* Avoid error when mixing multiple values with subcommands

## [4.2.2] - 2023-05-09

- *(bash)* Respect `ValueHint::Other`

## [4.2.1] - 2023-04-21

- *(zsh)* Improved escaping

## [4.2.0] - 2023-03-28

## [4.1.6] - 2023-03-28

## [4.1.5] - 2023-03-16

## [4.1.4] - 2023-02-27

### Features

- *(zsh)* Allow sourcing completion

## [4.1.3] - 2023-02-23

### Fixes

- *(zsh)* Improve handling of multi-valued arguments

## [4.1.2] - 2023-02-15

## [4.1.1] - 2023-01-23

### Fixes

- *(bash)* Mark `cmd` variable as local

## [4.1.0] - 2023-01-13

### Compatibility

MSRV changed to 1.64.0

## [4.0.7] - 2022-12-22

## [4.0.6] - 2022-11-24

## [4.0.5] - 2022-11-07

### Features

- Added `Shell::from_env`

## [4.0.4] - 2022-11-07

## [4.0.3] - 2022-10-18

## [4.0.2] - 2022-09-29

### Fixes

- *(bash)* Fix `git diff git <TAB>` completions
- *(bash)* Fix `git diff log <TAB>` completions
- *(bash)* Fix command alias

## [4.0.1] - 2022-09-28

### Fixes

- *(zsh)* Fix multiple-items regression introduced in v4

## [4.0.0] - 2022-09-28

## [3.2.3] - 2022-06-28

## [3.2.2] - 2022-06-21

## [3.2.1] - 2022-06-13

## [3.2.0] - 2022-06-13

## [3.1.4] - 2022-05-06

### Fixes

- *(bash)* Complete visible `PossibleValue`s, rather than hidden

## [3.1.3] - 2022-04-30

### Fixes

- Minimal rust-implemented clap completion engine, behind `unstable-dynamic` feature flag

## [3.1.2] - 2022-04-19

## [3.1.1] - 2022-03-02

## [3.1.0] - 2022-02-16

### Fixes

- Fish: escape possible values (#3467)

## [3.0.6] - 2022-02-05

### Fixes

- Powershell: Allow completion of partial commands

## [3.0.5] - 2022-01-24

### Fixes

- Clarified panics

## [3.0.4] - 2022-01-15

### Fixes

- Unescaped chars in zsh completions

## [3.0.3] - 2022-01-12

### Fixes

- Use new Elvish 0.17 syntax
- Add newline at end of zsh output

## [3.0.2] - 2022-01-04

## [3.0.1] - 2022-01-03

<!-- next-url -->
[Unreleased]: https://github.com/clap-rs/clap/compare/clap_complete-v4.5.43...HEAD
[4.5.43]: https://github.com/clap-rs/clap/compare/clap_complete-v4.5.42...clap_complete-v4.5.43
[4.5.42]: https://github.com/clap-rs/clap/compare/clap_complete-v4.5.41...clap_complete-v4.5.42
[4.5.41]: https://github.com/clap-rs/clap/compare/clap_complete-v4.5.40...clap_complete-v4.5.41
[4.5.40]: https://github.com/clap-rs/clap/compare/clap_complete-v4.5.39...clap_complete-v4.5.40
[4.5.39]: https://github.com/clap-rs/clap/compare/clap_complete-v4.5.38...clap_complete-v4.5.39
[4.5.38]: https://github.com/clap-rs/clap/compare/clap_complete-v4.5.37...clap_complete-v4.5.38
[4.5.37]: https://github.com/clap-rs/clap/compare/clap_complete-v4.5.36...clap_complete-v4.5.37
[4.5.36]: https://github.com/clap-rs/clap/compare/clap_complete-v4.5.35...clap_complete-v4.5.36
[4.5.35]: https://github.com/clap-rs/clap/compare/clap_complete-v4.5.34...clap_complete-v4.5.35
[4.5.34]: https://github.com/clap-rs/clap/compare/clap_complete-v4.5.33...clap_complete-v4.5.34
[4.5.33]: https://github.com/clap-rs/clap/compare/clap_complete-v4.5.32...clap_complete-v4.5.33
[4.5.32]: https://github.com/clap-rs/clap/compare/clap_complete-v4.5.31...clap_complete-v4.5.32
[4.5.31]: https://github.com/clap-rs/clap/compare/clap_complete-v4.5.30...clap_complete-v4.5.31
[4.5.30]: https://github.com/clap-rs/clap/compare/clap_complete-v4.5.29...clap_complete-v4.5.30
[4.5.29]: https://github.com/clap-rs/clap/compare/clap_complete-v4.5.28...clap_complete-v4.5.29
[4.5.28]: https://github.com/clap-rs/clap/compare/clap_complete-v4.5.27...clap_complete-v4.5.28
[4.5.27]: https://github.com/clap-rs/clap/compare/clap_complete-v4.5.26...clap_complete-v4.5.27
[4.5.26]: https://github.com/clap-rs/clap/compare/clap_complete-v4.5.25...clap_complete-v4.5.26
[4.5.25]: https://github.com/clap-rs/clap/compare/clap_complete-v4.5.24...clap_complete-v4.5.25
[4.5.24]: https://github.com/clap-rs/clap/compare/clap_complete-v4.5.23...clap_complete-v4.5.24
[4.5.23]: https://github.com/clap-rs/clap/compare/clap_complete-v4.5.22...clap_complete-v4.5.23
[4.5.22]: https://github.com/clap-rs/clap/compare/clap_complete-v4.5.21...clap_complete-v4.5.22
[4.5.21]: https://github.com/clap-rs/clap/compare/clap_complete-v4.5.20...clap_complete-v4.5.21
[4.5.20]: https://github.com/clap-rs/clap/compare/clap_complete-v4.5.19...clap_complete-v4.5.20
[4.5.19]: https://github.com/clap-rs/clap/compare/clap_complete-v4.5.18...clap_complete-v4.5.19
[4.5.18]: https://github.com/clap-rs/clap/compare/clap_complete-v4.5.17...clap_complete-v4.5.18
[4.5.17]: https://github.com/clap-rs/clap/compare/clap_complete-v4.5.16...clap_complete-v4.5.17
[4.5.16]: https://github.com/clap-rs/clap/compare/clap_complete-v4.5.15...clap_complete-v4.5.16
[4.5.15]: https://github.com/clap-rs/clap/compare/clap_complete-v4.5.14...clap_complete-v4.5.15
[4.5.14]: https://github.com/clap-rs/clap/compare/clap_complete-v4.5.13...clap_complete-v4.5.14
[4.5.13]: https://github.com/clap-rs/clap/compare/clap_complete-v4.5.12...clap_complete-v4.5.13
[4.5.12]: https://github.com/clap-rs/clap/compare/clap_complete-v4.5.11...clap_complete-v4.5.12
[4.5.11]: https://github.com/clap-rs/clap/compare/clap_complete-v4.5.10...clap_complete-v4.5.11
[4.5.10]: https://github.com/clap-rs/clap/compare/clap_complete-v4.5.9...clap_complete-v4.5.10
[4.5.9]: https://github.com/clap-rs/clap/compare/clap_complete-v4.5.8...clap_complete-v4.5.9
[4.5.8]: https://github.com/clap-rs/clap/compare/clap_complete-v4.5.7...clap_complete-v4.5.8
[4.5.7]: https://github.com/clap-rs/clap/compare/clap_complete-v4.5.6...clap_complete-v4.5.7
[4.5.6]: https://github.com/clap-rs/clap/compare/clap_complete-v4.5.5...clap_complete-v4.5.6
[4.5.5]: https://github.com/clap-rs/clap/compare/clap_complete-v4.5.4...clap_complete-v4.5.5
[4.5.4]: https://github.com/clap-rs/clap/compare/clap_complete-v4.5.3...clap_complete-v4.5.4
[4.5.3]: https://github.com/clap-rs/clap/compare/clap_complete-v4.5.2...clap_complete-v4.5.3
[4.5.2]: https://github.com/clap-rs/clap/compare/clap_complete-v4.5.1...clap_complete-v4.5.2
[4.5.1]: https://github.com/clap-rs/clap/compare/clap_complete-v4.5.0...clap_complete-v4.5.1
[4.5.0]: https://github.com/clap-rs/clap/compare/clap_complete-v4.4.10...clap_complete-v4.5.0
[4.4.10]: https://github.com/clap-rs/clap/compare/clap_complete-v4.4.9...clap_complete-v4.4.10
[4.4.9]: https://github.com/clap-rs/clap/compare/clap_complete-v4.4.8...clap_complete-v4.4.9
[4.4.8]: https://github.com/clap-rs/clap/compare/clap_complete-v4.4.7...clap_complete-v4.4.8
[4.4.7]: https://github.com/clap-rs/clap/compare/clap_complete-v4.4.6...clap_complete-v4.4.7
[4.4.6]: https://github.com/clap-rs/clap/compare/clap_complete-v4.4.5...clap_complete-v4.4.6
[4.4.5]: https://github.com/clap-rs/clap/compare/clap_complete-v4.4.4...clap_complete-v4.4.5
[4.4.4]: https://github.com/clap-rs/clap/compare/clap_complete-v4.4.3...clap_complete-v4.4.4
[4.4.3]: https://github.com/clap-rs/clap/compare/clap_complete-v4.4.2...clap_complete-v4.4.3
[4.4.2]: https://github.com/clap-rs/clap/compare/clap_complete-v4.4.1...clap_complete-v4.4.2
[4.4.1]: https://github.com/clap-rs/clap/compare/clap_complete-v4.4.0...clap_complete-v4.4.1
[4.4.0]: https://github.com/clap-rs/clap/compare/clap_complete-v4.3.2...clap_complete-v4.4.0
[4.3.2]: https://github.com/clap-rs/clap/compare/clap_complete-v4.3.1...clap_complete-v4.3.2
[4.3.1]: https://github.com/clap-rs/clap/compare/clap_complete-v4.3.0...clap_complete-v4.3.1
[4.3.0]: https://github.com/clap-rs/clap/compare/clap_complete-v4.2.3...clap_complete-v4.3.0
[4.2.3]: https://github.com/clap-rs/clap/compare/clap_complete-v4.2.2...clap_complete-v4.2.3
[4.2.2]: https://github.com/clap-rs/clap/compare/clap_complete-v4.2.1...clap_complete-v4.2.2
[4.2.1]: https://github.com/clap-rs/clap/compare/clap_complete-v4.2.0...clap_complete-v4.2.1
[4.2.0]: https://github.com/clap-rs/clap/compare/clap_complete-v4.1.6...clap_complete-v4.2.0
[4.1.6]: https://github.com/clap-rs/clap/compare/clap_complete-v4.1.5...clap_complete-v4.1.6
[4.1.5]: https://github.com/clap-rs/clap/compare/clap_complete-v4.1.4...clap_complete-v4.1.5
[4.1.4]: https://github.com/clap-rs/clap/compare/clap_complete-v4.1.3...clap_complete-v4.1.4
[4.1.3]: https://github.com/clap-rs/clap/compare/clap_complete-v4.1.2...clap_complete-v4.1.3
[4.1.2]: https://github.com/clap-rs/clap/compare/clap_complete-v4.1.1...clap_complete-v4.1.2
[4.1.1]: https://github.com/clap-rs/clap/compare/clap_complete-v4.1.0...clap_complete-v4.1.1
[4.1.0]: https://github.com/clap-rs/clap/compare/clap_complete-v4.0.7...clap_complete-v4.1.0
[4.0.7]: https://github.com/clap-rs/clap/compare/clap_complete-v4.0.6...clap_complete-v4.0.7
[4.0.6]: https://github.com/clap-rs/clap/compare/clap_complete-v4.0.5...clap_complete-v4.0.6
[4.0.5]: https://github.com/clap-rs/clap/compare/clap_complete-v4.0.4...clap_complete-v4.0.5
[4.0.4]: https://github.com/clap-rs/clap/compare/clap_complete-v4.0.3...clap_complete-v4.0.4
[4.0.3]: https://github.com/clap-rs/clap/compare/clap_complete-v4.0.2...clap_complete-v4.0.3
[4.0.2]: https://github.com/clap-rs/clap/compare/clap_complete-v4.0.1...clap_complete-v4.0.2
[4.0.1]: https://github.com/clap-rs/clap/compare/clap_complete-v4.0.0...clap_complete-v4.0.1
[4.0.0]: https://github.com/clap-rs/clap/compare/clap_complete-v3.2.3...clap_complete-v4.0.0
[3.2.3]: https://github.com/clap-rs/clap/compare/clap_complete-v3.2.2...clap_complete-v3.2.3
[3.2.2]: https://github.com/clap-rs/clap/compare/clap_complete-v3.2.1...clap_complete-v3.2.2
[3.2.1]: https://github.com/clap-rs/clap/compare/clap_complete-v3.2.0...clap_complete-v3.2.1
[3.2.0]: https://github.com/clap-rs/clap/compare/clap_complete-v3.1.4...clap_complete-v3.2.0
[3.1.4]: https://github.com/clap-rs/clap/compare/clap_complete-v3.1.3...clap_complete-v3.1.4
[3.1.3]: https://github.com/clap-rs/clap/compare/clap_complete-v3.1.2...clap_complete-v3.1.3
[3.1.2]: https://github.com/clap-rs/clap/compare/clap_complete-v3.1.1...clap_complete-v3.1.2
[3.1.1]: https://github.com/clap-rs/clap/compare/clap_complete-v3.1.0...clap_complete-v3.1.1
[3.1.0]: https://github.com/clap-rs/clap/compare/clap_complete-v3.0.6...clap_complete-v3.1.0
[3.0.6]: https://github.com/clap-rs/clap/compare/clap_complete-v3.0.5...clap_complete-v3.0.6
[3.0.5]: https://github.com/clap-rs/clap/compare/clap_complete-v3.0.4...clap_complete-v3.0.5
[3.0.4]: https://github.com/clap-rs/clap/compare/clap_complete-v3.0.3...clap_complete-v3.0.4
[3.0.3]: https://github.com/clap-rs/clap/compare/v3.0.2...clap_complete-v3.0.3
[3.0.2]: https://github.com/clap-rs/clap/compare/v3.0.1...v3.0.2
[3.0.1]: https://github.com/clap-rs/clap/compare/v3.0.0...v3.0.1
