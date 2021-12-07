// Copyright ⓒ 2015-2018 Kevin B. Knapp
//
// `clap_generate` is distributed under the terms of both the MIT license and the Apache License
// (Version 2.0).
// See the [LICENSE-APACHE](LICENSE-APACHE) and [LICENSE-MIT](LICENSE-MIT) files in this repository
// for more information.

#![doc(html_logo_url = "https://clap.rs/images/media/clap.png")]
#![doc(html_root_url = "https://docs.rs/clap_generate/3.0.0-rc.0")]
#![doc = include_str!("../README.md")]
#![deny(missing_docs, trivial_casts, unused_allocation, trivial_numeric_casts)]
#![forbid(unsafe_code)]
#![allow(clippy::needless_doctest_main)]

//! ## Quick Start
//!
//! - For generating at compile-time, see [`generate_to`]
//! - For generating at runtime, see [`generate`]
//!
//! [`Shell`] is a convenience `enum` for an argument value type that implements `Generator`
//! for each natively-supported shell type.
//!
//! ## Example
//!
//! ```rust,no_run
//! use clap::{App, AppSettings, Arg, ValueHint};
//! use clap_generate::{generate, Generator, Shell};
//! use std::io;
//!
//! fn build_cli() -> App<'static> {
//!     App::new("example")
//!          .arg(Arg::new("file")
//!              .about("some input file")
//!                 .value_hint(ValueHint::AnyPath),
//!         )
//!        .arg(
//!            Arg::new("generator")
//!                .long("generate")
//!                .possible_values(Shell::possible_values()),
//!        )
//! }
//!
//! fn print_completions<G: Generator>(gen: G, app: &mut App) {
//!     generate(gen, app, app.get_name().to_string(), &mut io::stdout());
//! }
//!
//! fn main() {
//!     let matches = build_cli().get_matches();
//!
//!     if let Ok(generator) = matches.value_of_t::<Shell>("generator") {
//!         let mut app = build_cli();
//!         eprintln!("Generating completion file for {}...", generator);
//!         print_completions(generator, &mut app);
//!     }
//! }
//! ```

const INTERNAL_ERROR_MSG: &str = "Fatal internal error. Please consider filing a bug \
                                  report at https://github.com/clap-rs/clap/issues";

#[macro_use]
#[allow(missing_docs)]
mod macros;

/// Contains some popular generators
pub mod generators;
/// Contains supported shells for auto-completion scripts
mod shell;
/// Helpers for writing generators
pub mod utils;

use std::ffi::OsString;
use std::fs::File;
use std::io::Error;
use std::io::Write;
use std::path::PathBuf;

#[doc(inline)]
pub use generators::Generator;
#[doc(inline)]
pub use shell::Shell;

/// Generate a completions file for a specified shell at compile-time.
///
/// **NOTE:** to generate the file at compile time you must use a `build.rs` "Build Script" or a
/// [`cargo-xtask`](https://github.com/matklad/cargo-xtask)
///
/// # Examples
///
/// The following example generates a bash completion script via a `build.rs` script. In this
/// simple example, we'll demo a very small application with only a single subcommand and two
/// args. Real applications could be many multiple levels deep in subcommands, and have tens or
/// potentially hundreds of arguments.
///
/// First, it helps if we separate out our `App` definition into a separate file. Whether you
/// do this as a function, or bare App definition is a matter of personal preference.
///
/// ```
/// // src/cli.rs
///
/// use clap::{App, Arg};
///
/// pub fn build_cli() -> App<'static> {
///     App::new("compl")
///         .about("Tests completions")
///         .arg(Arg::new("file")
///             .about("some input file"))
///         .subcommand(App::new("test")
///             .about("tests things")
///             .arg(Arg::new("case")
///                 .long("case")
///                 .takes_value(true)
///                 .about("the case to test")))
/// }
/// ```
///
/// In our regular code, we can simply call this `build_cli()` function, then call
/// `get_matches()`, or any of the other normal methods directly after. For example:
///
/// ```ignore
/// // src/main.rs
///
/// mod cli;
///
/// fn main() {
///     let _m = cli::build_cli().get_matches();
///
///     // normal logic continues...
/// }
/// ```
///
/// Next, we set up our `Cargo.toml` to use a `build.rs` build script.
///
/// ```toml
/// # Cargo.toml
/// build = "build.rs"
///
/// [dependencies]
/// clap = "*"
///
/// [build-dependencies]
/// clap = "*"
/// clap_generate = "*"
/// ```
///
/// Next, we place a `build.rs` in our project root.
///
/// ```ignore
/// use clap_generate::{generate_to, generators::Bash};
/// use std::env;
/// use std::io::Error;
///
/// include!("src/cli.rs");
///
/// fn main() -> Result<(), Error> {
///     let outdir = match env::var_os("OUT_DIR") {
///         None => return Ok(()),
///         Some(outdir) => outdir,
///     };
///
///     let mut app = build_cli();
///     let path = generate_to(
///         Bash,
///         &mut app, // We need to specify what generator to use
///         "myapp",  // We need to specify the bin name manually
///         outdir,   // We need to specify where to write to
///     )?;
///
///     println!("cargo:warning=completion file is generated: {:?}", path);
///
///     Ok(())
/// }
/// ```
///
/// Now, once we compile there will be a `{bin_name}.bash` file in the directory.
/// Assuming we compiled with debug mode, it would be somewhere similar to
/// `<project>/target/debug/build/myapp-<hash>/out/myapp.bash`.
///
/// **NOTE:** Please look at the individual [generators]
/// to see the name of the files generated.
pub fn generate_to<G, S, T>(
    gen: G,
    app: &mut clap::App,
    bin_name: S,
    out_dir: T,
) -> Result<PathBuf, Error>
where
    G: Generator,
    S: Into<String>,
    T: Into<OsString>,
{
    app.set_bin_name(bin_name);

    let out_dir = PathBuf::from(out_dir.into());
    let file_name = gen.file_name(app.get_bin_name().unwrap());

    let path = out_dir.join(file_name);
    let mut file = File::create(&path)?;

    _generate::<G, S>(gen, app, &mut file);
    Ok(path)
}

/// Generate a completions file for a specified shell at runtime.
///
/// Until `cargo install` can install extra files like a completion script, this may be
/// used e.g. in a command that outputs the contents of the completion script, to be
/// redirected into a file by the user.
///
/// # Examples
///
/// Assuming a separate `cli.rs` like the [example above](generate_to()),
/// we can let users generate a completion script using a command:
///
/// ```ignore
/// // src/main.rs
///
/// mod cli;
/// use std::io;
/// use clap_generate::{generate, generators::Bash};
///
/// fn main() {
///     let matches = cli::build_cli().get_matches();
///
///     if matches.is_present("generate-bash-completions") {
///         generate(Bash, &mut cli::build_cli(), "myapp", &mut io::stdout());
///     }
///
///     // normal logic continues...
/// }
///
/// ```
///
/// Usage:
///
/// ```shell
/// $ myapp generate-bash-completions > /usr/share/bash-completion/completions/myapp.bash
/// ```
pub fn generate<G, S>(gen: G, app: &mut clap::App, bin_name: S, buf: &mut dyn Write)
where
    G: Generator,
    S: Into<String>,
{
    app.set_bin_name(bin_name);
    _generate::<G, S>(gen, app, buf)
}

fn _generate<G, S>(gen: G, app: &mut clap::App, buf: &mut dyn Write)
where
    G: Generator,
    S: Into<String>,
{
    app._build_all();

    gen.generate(app, buf)
}
