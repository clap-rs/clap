# Change Log
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

<!-- next-header -->
## [Unreleased] - ReleaseDate

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
[Unreleased]: https://github.com/clap-rs/clap/compare/clap_complete-v4.2.0...HEAD
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
