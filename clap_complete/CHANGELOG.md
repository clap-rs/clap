# Change Log
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

<!-- next-header -->
## [Unreleased] - ReleaseDate

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
[Unreleased]: https://github.com/clap-rs/clap/compare/clap_complete-v4.5.18...HEAD
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
