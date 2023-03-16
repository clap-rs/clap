<!-- omit in TOC -->
# clap_mangen

> **Manpage generation for `clap`**

[![Crates.io](https://img.shields.io/crates/v/clap_mangen?style=flat-square)](https://crates.io/crates/clap_mangen)
[![Crates.io](https://img.shields.io/crates/d/clap_mangen?style=flat-square)](https://crates.io/crates/clap_mangen)
[![License](https://img.shields.io/badge/license-Apache%202.0-blue?style=flat-square)](https://github.com/clap-rs/clap/blob/clap_mangen-v0.2.10/LICENSE-APACHE)
[![License](https://img.shields.io/badge/license-MIT-blue?style=flat-square)](https://github.com/clap-rs/clap/blob/clap_mangen-v0.2.10/LICENSE-MIT)

Dual-licensed under [Apache 2.0](LICENSE-APACHE) or [MIT](LICENSE-MIT).

1. [About](#about)
2. [API Reference](https://docs.rs/clap_mangen)
3. [Questions & Discussions](https://github.com/clap-rs/clap/discussions)
4. [CONTRIBUTING](https://github.com/clap-rs/clap/blob/clap_mangen-v0.2.10/clap_mangen/CONTRIBUTING.md)
5. [Sponsors](https://github.com/clap-rs/clap/blob/clap_mangen-v0.2.10/README.md#sponsors)

## About

Generate [ROFF](https://en.wikipedia.org/wiki/Roff_(software)) from a `clap::Command`.

### Example

We're going to assume you want to generate your man page as part of your
development rather than your shipped program having a flag to generate it.

Run
```console
$ cargo add --build clap_mangen
```

In your `build.rs`:
```rust,no_run
fn main() -> std::io::Result<()> {
    let out_dir = std::path::PathBuf::from(std::env::var_os("OUT_DIR").ok_or(std::io::ErrorKind::NotFound)?);

    let cmd = clap::Command::new("mybin")
        .arg(clap::arg!(-n --name <NAME>))
        .arg(clap::arg!(-c --count <NUM>));

    let man = clap_mangen::Man::new(cmd);
    let mut buffer: Vec<u8> = Default::default();
    man.render(&mut buffer)?;

    std::fs::write(out_dir.join("mybin.1"), buffer)?;

    Ok(())
}
```

Tip: Consider a [cargo xtask](https://github.com/matklad/cargo-xtask) instead of a `build.rs` to reduce build costs.
