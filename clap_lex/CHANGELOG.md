# Change Log
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

<!-- next-header -->
## [Unreleased] - ReleaseDate

## [0.7.4] - 2024-12-05

### Fixes

- Support `E` in numbers, not just `e`

## [0.7.3] - 2024-11-13

## [0.7.2] - 2024-07-25

## [0.7.1] - 2024-06-06

## [0.7.0] - 2024-02-08

### Compatibility

- Update MSRV to 1.74

### Fixes

- Improve `unsafe` code by using new `OsStr` API

## [0.6.0] - 2023-10-24

### Breaking Change

- Renamed `is_number` to more focused `is_negative_number`

### Performance

- Reduced code size

## [0.5.1] - 2023-08-24

### Compatibility

- Update MSRV to 1.70.0

## [0.5.0] - 2023-05-19

### Breaking Change

- Removed `OsStrExt::split_at`

## [0.4.1] - 2023-03-28

### Compatibility

- Deprecated `OsStrExt::split_at` as its unsound

## [0.4.0] - 2023-03-25

### Breaking Change

- `RawOsStr` and `RawOsString` are no long exported
- Return types were changed from `RawOsStr` to `OsStr`

### Features

- `OsStrExt` trait added to help with processing `OsStr`s

### Performance

- `os_str_bytes` dependency was dropped to improve build times and reduce binary size

## [0.3.3] - 2023-03-16

## [0.3.2] - 2023-02-23

## [0.3.1] - 2023-01-13

### Compatibility

MSRV changed to 1.64.0

## [0.3.0] - 2022-09-20

### Breaking Changes

- `RawArgs::insert` now takes owned values

### Compatibility

- MSRV changed from 1.56.1 to 1.60.0

## [0.2.4] - 2022-06-28

## [0.2.3] - 2022-06-21

## [0.2.2] - 2022-06-13

## [0.2.1] - 2022-06-13

### Features

- Allow checking if at end of input

## [0.2.0] - 2022-04-30

### Breaking Changes

- Don't do prefix matching by default

## [0.1.1] - 2022-04-15

- Drop `memchr` dependency

<!-- next-url -->
[Unreleased]: https://github.com/clap-rs/clap/compare/clap_lex-v0.7.4...HEAD
[0.7.4]: https://github.com/clap-rs/clap/compare/clap_lex-v0.7.3...clap_lex-v0.7.4
[0.7.3]: https://github.com/clap-rs/clap/compare/clap_lex-v0.7.2...clap_lex-v0.7.3
[0.7.2]: https://github.com/clap-rs/clap/compare/clap_lex-v0.7.1...clap_lex-v0.7.2
[0.7.1]: https://github.com/clap-rs/clap/compare/clap_lex-v0.7.0...clap_lex-v0.7.1
[0.7.0]: https://github.com/clap-rs/clap/compare/clap_lex-v0.6.0...clap_lex-v0.7.0
[0.6.0]: https://github.com/clap-rs/clap/compare/clap_lex-v0.5.1...clap_lex-v0.6.0
[0.5.1]: https://github.com/clap-rs/clap/compare/clap_lex-v0.5.0...clap_lex-v0.5.1
[0.5.0]: https://github.com/clap-rs/clap/compare/clap_lex-v0.4.1...clap_lex-v0.5.0
[0.4.1]: https://github.com/clap-rs/clap/compare/clap_lex-v0.4.0...clap_lex-v0.4.1
[0.4.0]: https://github.com/clap-rs/clap/compare/clap_lex-v0.3.3...clap_lex-v0.4.0
[0.3.3]: https://github.com/clap-rs/clap/compare/clap_lex-v0.3.2...clap_lex-v0.3.3
[0.3.2]: https://github.com/clap-rs/clap/compare/clap_lex-v0.3.1...clap_lex-v0.3.2
[0.3.1]: https://github.com/clap-rs/clap/compare/clap_lex-v0.3.0...clap_lex-v0.3.1
[0.3.0]: https://github.com/clap-rs/clap/compare/clap_lex-v0.2.4...clap_lex-v0.3.0
[0.2.4]: https://github.com/clap-rs/clap/compare/clap_lex-v0.2.3...clap_lex-v0.2.4
[0.2.3]: https://github.com/clap-rs/clap/compare/clap_lex-v0.2.2...clap_lex-v0.2.3
[0.2.2]: https://github.com/clap-rs/clap/compare/clap_lex-v0.2.1...clap_lex-v0.2.2
[0.2.1]: https://github.com/clap-rs/clap/compare/clap_lex-v0.2.0...clap_lex-v0.2.1
[0.2.0]: https://github.com/clap-rs/clap/compare/clap_lex-v0.1.1...clap_lex-v0.2.0
[0.1.1]: https://github.com/clap-rs/clap/compare/ce71b08a3fe28c640dc6e17f6f5bb1452bd6d6d8...clap_lex-v0.1.1
