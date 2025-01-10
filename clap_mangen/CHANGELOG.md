# Change Log
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

<!-- next-header -->
## [Unreleased] - ReleaseDate

## [0.2.26] - 2025-01-10

### Features

- Respect `help_heading`

## [0.2.25] - 2025-01-07

### Fixes

- Do not generate man pages for hidden subcommands

## [0.2.24] - 2024-10-08

## [0.2.23] - 2024-07-25

## [0.2.22] - 2024-06-28

## [0.2.21] - 2024-06-06

## [0.2.20] - 2024-02-08

### Compatibility

- Update MSRV to 1.74

## [0.2.19] - 2024-02-02

### Features

- Support generating files for man pages

## [0.2.18] - 2024-01-29

### Fixes

- Print full subcommand name in title (ie include parent commands)
- Print full subcommand in usage (ie include parent commands

## [0.2.17] - 2024-01-11

### Fixes

- Correctly show help for fake flags

## [0.2.16] - 2023-12-28

### Performance

- Only ask `TypedValueParser` for possible values if needed

## [0.2.15] - 2023-10-24

## [0.2.14] - 2023-09-18

## [0.2.13] - 2023-08-24

### Compatibility

- Update MSRV to 1.70.0

## [0.2.12] - 2023-06-02

## [0.2.11] - 2023-05-19

## [0.2.10] - 2023-03-16

## [0.2.9] - 2023-02-22

### Fixes

- Only show value names if a value is taken

## [0.2.8] - 2023-02-15

## [0.2.7] - 2023-01-13

### Compatibility

MSRV changed to 1.64.0

## [0.2.6] - 2022-12-22

## [0.2.5] - 2022-11-24

## [0.2.4] - 2022-10-31

### Fixes

- Don't show defaults for flags

## [0.2.3] - 2022-10-18

## [0.2.2] - 2022-09-29

### Fixes

- Reference to subcommand man pages now lists the correct name

## [0.2.1] - 2022-09-28

### Fixes

- Respect hide attributes in more cases

## [0.2.0] - 2022-09-28

## [0.1.10] - 2022-06-28

## [0.1.9] - 2022-06-21

## [0.1.8] - 2022-06-13

## [0.1.7] - 2022-06-13

## [0.1.6] - 2022-04-20

### Fixes

- Split environment variables into separate paragraph

## [0.1.5] - 2022-04-19

## [0.1.4] - 2022-04-19

## [0.1.3] - 2022-04-15

- Use value names for positionals
- Hide hidden flags in synopsis
- Use italics for replaceable args

## [0.1.2] - 2022-02-16

## [0.1.1] - 2022-02-08

### Fixes

- Expanded the documentation

<!-- next-url -->
[Unreleased]: https://github.com/clap-rs/clap/compare/clap_mangen-v0.2.26...HEAD
[0.2.26]: https://github.com/clap-rs/clap/compare/clap_mangen-v0.2.25...clap_mangen-v0.2.26
[0.2.25]: https://github.com/clap-rs/clap/compare/clap_mangen-v0.2.24...clap_mangen-v0.2.25
[0.2.24]: https://github.com/clap-rs/clap/compare/clap_mangen-v0.2.23...clap_mangen-v0.2.24
[0.2.23]: https://github.com/clap-rs/clap/compare/clap_mangen-v0.2.22...clap_mangen-v0.2.23
[0.2.22]: https://github.com/clap-rs/clap/compare/clap_mangen-v0.2.21...clap_mangen-v0.2.22
[0.2.21]: https://github.com/clap-rs/clap/compare/clap_mangen-v0.2.20...clap_mangen-v0.2.21
[0.2.20]: https://github.com/clap-rs/clap/compare/clap_mangen-v0.2.19...clap_mangen-v0.2.20
[0.2.19]: https://github.com/clap-rs/clap/compare/clap_mangen-v0.2.18...clap_mangen-v0.2.19
[0.2.18]: https://github.com/clap-rs/clap/compare/clap_mangen-v0.2.17...clap_mangen-v0.2.18
[0.2.17]: https://github.com/clap-rs/clap/compare/clap_mangen-v0.2.16...clap_mangen-v0.2.17
[0.2.16]: https://github.com/clap-rs/clap/compare/clap_mangen-v0.2.15...clap_mangen-v0.2.16
[0.2.15]: https://github.com/clap-rs/clap/compare/clap_mangen-v0.2.14...clap_mangen-v0.2.15
[0.2.14]: https://github.com/clap-rs/clap/compare/clap_mangen-v0.2.13...clap_mangen-v0.2.14
[0.2.13]: https://github.com/clap-rs/clap/compare/clap_mangen-v0.2.12...clap_mangen-v0.2.13
[0.2.12]: https://github.com/clap-rs/clap/compare/clap_mangen-v0.2.11...clap_mangen-v0.2.12
[0.2.11]: https://github.com/clap-rs/clap/compare/clap_mangen-v0.2.10...clap_mangen-v0.2.11
[0.2.10]: https://github.com/clap-rs/clap/compare/clap_mangen-v0.2.9...clap_mangen-v0.2.10
[0.2.9]: https://github.com/clap-rs/clap/compare/clap_mangen-v0.2.8...clap_mangen-v0.2.9
[0.2.8]: https://github.com/clap-rs/clap/compare/clap_mangen-v0.2.7...clap_mangen-v0.2.8
[0.2.7]: https://github.com/clap-rs/clap/compare/clap_mangen-v0.2.6...clap_mangen-v0.2.7
[0.2.6]: https://github.com/clap-rs/clap/compare/clap_mangen-v0.2.5...clap_mangen-v0.2.6
[0.2.5]: https://github.com/clap-rs/clap/compare/clap_mangen-v0.2.4...clap_mangen-v0.2.5
[0.2.4]: https://github.com/clap-rs/clap/compare/clap_mangen-v0.2.3...clap_mangen-v0.2.4
[0.2.3]: https://github.com/clap-rs/clap/compare/clap_mangen-v0.2.2...clap_mangen-v0.2.3
[0.2.2]: https://github.com/clap-rs/clap/compare/clap_mangen-v0.2.1...clap_mangen-v0.2.2
[0.2.1]: https://github.com/clap-rs/clap/compare/clap_mangen-v0.2.0...clap_mangen-v0.2.1
[0.2.0]: https://github.com/clap-rs/clap/compare/clap_mangen-v0.1.10...clap_mangen-v0.2.0
[0.1.10]: https://github.com/clap-rs/clap/compare/clap_mangen-v0.1.9...clap_mangen-v0.1.10
[0.1.9]: https://github.com/clap-rs/clap/compare/clap_mangen-v0.1.8...clap_mangen-v0.1.9
[0.1.8]: https://github.com/clap-rs/clap/compare/clap_mangen-v0.1.7...clap_mangen-v0.1.8
[0.1.7]: https://github.com/clap-rs/clap/compare/clap_mangen-v0.1.6...clap_mangen-v0.1.7
[0.1.6]: https://github.com/clap-rs/clap/compare/clap_mangen-v0.1.5...clap_mangen-v0.1.6
[0.1.5]: https://github.com/clap-rs/clap/compare/clap_mangen-v0.1.4...clap_mangen-v0.1.5
[0.1.4]: https://github.com/clap-rs/clap/compare/clap_mangen-v0.1.3...clap_mangen-v0.1.4
[0.1.3]: https://github.com/clap-rs/clap/compare/clap_mangen-v0.1.2...clap_mangen-v0.1.3
[0.1.2]: https://github.com/clap-rs/clap/compare/clap_mangen-v0.1.1...clap_mangen-v0.1.2
[0.1.1]: https://github.com/clap-rs/clap/compare/0b045f5d0de9f6c97607be3276f529a14510e94e...clap_mangen-v0.1.1
