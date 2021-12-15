# Examples

- Basic demo: [derive](demo.md)
- Key-value pair arguments: [derive](keyvalue-derive.md)
- Custom cargo command: [builder](cargo-example.md), [derive](cargo-example-derive.md)
- git-like interface: [builder](git.md), [derive](git-derive.md)
- pacman-like interface: [builder](pacman.md)
- Escaped positionals with `--`: [builder](escaped-positional.md), [derive](escaped-positional-derive.md)
- Multi-call
  - busybox: [builder](multicall-busybox.md)
  - hostname: [builder](multicall-hostname.md)

## Contributing

New examples:
- Building: They must be added to [Cargo.toml](../../Cargo.toml) with the appropriate `required-features`.
- Testing: Ensure there is a markdown file with [trycmd](https://docs.rs/trycmd) syntax
- Link the `.md` file from here

See also the general [CONTRIBUTING](../../CONTRIBUTING.md).
