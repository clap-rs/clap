// Copyright ⓒ 2015-2018 Kevin B. Knapp
//
// `clap_generate` is distributed under the terms of both the MIT license and the Apache License
// (Version 2.0).
// See the [LICENSE-APACHE](LICENSE-APACHE) and [LICENSE-MIT](LICENSE-MIT) files in this repository
// for more information.

//! Generates stuff for [`clap`](https://github.com/clap-rs/clap) based CLIs

#![doc(html_root_url = "https://docs.rs/clap_generate/3.0.0-beta.1")]
#![deny(
    missing_docs,
    trivial_casts,
    unused_import_braces,
    unused_allocation,
    trivial_numeric_casts
)]
#![allow(clippy::needless_doctest_main)]

const INTERNAL_ERROR_MSG: &str = "Fatal internal error. Please consider filing a bug \
                                  report at https://github.com/clap-rs/clap/issues";

#[macro_use]
#[allow(missing_docs)]
mod macros;

/// Contains some popular generators
pub mod generators;

use std::ffi::OsString;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

#[doc(inline)]
pub use generators::Generator;

/// Generate a file for a specified generator at compile time.
///
/// **NOTE:** to generate the file at compile time you must use a `build.rs` "Build Script"
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
///     let m = cli::build_cli().get_matches();
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
/// [build-dependencies]
/// clap = "*"
/// ```
///
/// Next, we place a `build.rs` in our project root.
///
/// ```ignore
/// use clap_generate::{generate_to, generators::Bash};
///
/// include!("src/cli.rs");
///
/// fn main() {
///     let outdir = match env::var_os("OUT_DIR") {
///         None => return,
///         Some(outdir) => outdir,
///     };
///
///     let mut app = build_cli();
///     generate_to::<Bash, _, _>(
///         &mut app,     // We need to specify what generator to use
///         "myapp",      // We need to specify the bin name manually
///         outdir,       // We need to specify where to write to
///     );
/// }
/// ```
///
/// Now, once we compile there will be a `{bin_name}.bash` file in the directory.
/// Assuming we compiled with debug mode, it would be somewhere similar to
/// `<project>/target/debug/build/myapp-<hash>/out/myapp.bash`.
///
/// **NOTE:** Please look at the individual [generators](./generators/index.html)
/// to see the name of the files generated.
pub fn generate_to<G, S, T>(app: &mut clap::App, bin_name: S, out_dir: T)
where
    G: Generator,
    S: Into<String>,
    T: Into<OsString>,
{
    let out_dir = PathBuf::from(out_dir.into());
    let file_name = G::file_name(app.get_bin_name().unwrap());

    let mut file = match File::create(out_dir.join(file_name)) {
        Err(why) => panic!("couldn't create completion file: {}", why),
        Ok(file) => file,
    };

    generate::<G, S>(app, bin_name, &mut file)
}

/// Generate a completions file for a specified shell at runtime.
///
/// Until `cargo install` can install extra files like a completion script, this may be
/// used e.g. in a command that outputs the contents of the completion script, to be
/// redirected into a file by the user.
///
/// # Examples
///
/// Assuming a separate `cli.rs` like the [example above](./fn.generate_to.html),
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
///         generate::<Bash, _>(&mut cli::build_cli(), "myapp", &mut io::stdout());
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
pub fn generate<G, S>(app: &mut clap::App, bin_name: S, buf: &mut dyn Write)
where
    G: Generator,
    S: Into<String>,
{
    app.set_bin_name(bin_name);

    if !app.is_set(clap::AppSettings::Built) {
        app._build();
        app._build_bin_names();
    }

    G::generate(app, buf)
}
