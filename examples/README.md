# Examples

- Basic demo: [derive](demo.md)
- Key-value pair arguments: [derive](keyvalue_derive.md)
- git-like interface: [builder](git.md), [derive](git_derive.md)
- pacman-like interface: [builder](pacman.md)
- Escaped positionals with `--`: [builder](escaped_positional.md), [derive](escaped_positional_derive.md)
- Multi-call
  - busybox: [builder](multicall_busybox.md)
  - hostname: [builder](multicall_hostname.md)

## Contributing

New examples:
- Building: They must be added to [Cargo.toml](../../Cargo.toml) with the appropriate `required-features`.
- Testing: Ensure there is a markdown file with [trycmd](https://docs.rs/trycmd) syntax
- Link the `.md` file from here

See also the general [CONTRIBUTING](../../CONTRIBUTING.md).
