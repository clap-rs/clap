#[macro_use]
mod macros;
mod bash;
mod fish;
mod zsh;
mod powershell;
mod shell;

// Std
use std::io::Write;
use std::ffi::OsString; 

// Internal
use parsing::parser::Parser;
use builders::app::App;
use self::bash::BashGen;
use self::fish::FishGen;
use self::zsh::ZshGen;
use self::powershell::PowerShellGen;
pub use self::shell::Shell;

pub struct ComplGen<'a, 'b>
    where 'a: 'b
{
    p: &'b Parser<'a, 'b>,
}

impl<'a, 'b> ComplGen<'a, 'b> {
    pub fn new(p: &'b Parser<'a, 'b>) -> Self { ComplGen { p: p } }

    pub fn generate<W: Write>(&self, for_shell: Shell, buf: &mut W) {
        match for_shell {
            Shell::Bash => BashGen::new(self.p).generate_to(buf),
            Shell::Fish => FishGen::new(self.p).generate_to(buf),
            Shell::Zsh => ZshGen::new(self.p).generate_to(buf),
            Shell::PowerShell => PowerShellGen::new(self.p).generate_to(buf),
        }
    }
}

// Gets all subcommands including child subcommands in the form of 'name' where the name
// is a single word (i.e. "install")  of the path to said subcommand (i.e.
// "rustup toolchain install")
//
// Also note, aliases are treated as their own subcommands but duplicates of whatever they're
// aliasing.
pub fn all_subcommand_names(p: &Parser) -> Vec<String> {
    debugln!("all_subcommand_names;");
    let mut subcmds: Vec<_> = subcommands_of(p).iter().map(|&(ref n, _)| n.clone()).collect();
    for sc_v in p.subcommands.iter().map(|s| all_subcommand_names(&s.p)) {
        subcmds.extend(sc_v);
    }
    subcmds.sort();
    subcmds.dedup();
    subcmds
}

// Gets all subcommands including child subcommands in the form of ('name', 'bin_name') where the name
// is a single word (i.e. "install") of the path and full bin_name of said subcommand (i.e.
// "rustup toolchain install")
//
// Also note, aliases are treated as their own subcommands but duplicates of whatever they're
// aliasing.
pub fn all_subcommands(p: &Parser) -> Vec<(String, String)> {
    debugln!("all_subcommands;");
    let mut subcmds: Vec<_> = subcommands_of(p);
    for sc_v in p.subcommands.iter().map(|s| all_subcommands(&s.p)) {
        subcmds.extend(sc_v);
    }
    subcmds
}

// Gets all subcommands exlcuding child subcommands in the form of (name, bin_name) where the name
// is a single word (i.e. "install") and the bin_name is a space deliniated list of the path to said
// subcommand (i.e. "rustup toolchain install")
//
// Also note, aliases are treated as their own subcommands but duplicates of whatever they're
// aliasing.
pub fn subcommands_of(p: &Parser) -> Vec<(String, String)> {
    debugln!("subcommands_of: name={}, bin_name={}",
             p.meta.name,
             p.meta.bin_name.as_ref().unwrap());
    let mut subcmds = vec![];

    debugln!("subcommands_of: Has subcommands...{:?}", p.has_subcommands());
    if !p.has_subcommands() {
        let mut ret = vec![(p.meta.name.clone(), p.meta.bin_name.as_ref().unwrap().clone())];
        debugln!("subcommands_of: Looking for aliases...");
        if let Some(ref aliases) = p.meta.aliases {
            for &(n, _) in aliases {
                debugln!("subcommands_of:iter:iter: Found alias...{}", n);
                let mut als_bin_name: Vec<_> =
                    p.meta.bin_name.as_ref().unwrap().split(' ').collect();
                als_bin_name.push(n);
                let old = als_bin_name.len() - 2;
                als_bin_name.swap_remove(old);
                ret.push((n.to_owned(), als_bin_name.join(" ")));
            }
        }
        return ret;
    }
    for sc in &p.subcommands {
        debugln!("subcommands_of:iter: name={}, bin_name={}",
                 sc.p.meta.name,
                 sc.p.meta.bin_name.as_ref().unwrap());

        debugln!("subcommands_of:iter: Looking for aliases...");
        if let Some(ref aliases) = sc.p.meta.aliases {
            for &(n, _) in aliases {
                debugln!("subcommands_of:iter:iter: Found alias...{}", n);
                let mut als_bin_name: Vec<_> =
                    p.meta.bin_name.as_ref().unwrap().split(' ').collect();
                als_bin_name.push(n);
                let old = als_bin_name.len() - 2;
                als_bin_name.swap_remove(old);
                subcmds.push((n.to_owned(), als_bin_name.join(" ")));
            }
        }
        subcmds.push((sc.p.meta.name.clone(), sc.p.meta.bin_name.as_ref().unwrap().clone()));
    }
    subcmds
}

pub fn get_all_subcommand_paths(p: &Parser, first: bool) -> Vec<String> {
    debugln!("get_all_subcommand_paths;");
    let mut subcmds = vec![];
    if !p.has_subcommands() {
        if !first {
            let name = &*p.meta.name;
            let path = p.meta.bin_name.as_ref().unwrap().clone().replace(" ", "__");
            let mut ret = vec![path.clone()];
            if let Some(ref aliases) = p.meta.aliases {
                for &(n, _) in aliases {
                    ret.push(path.replace(name, n));
                }
            }
            return ret;
        }
        return vec![];
    }
    for sc in &p.subcommands {
        let name = &*sc.p.meta.name;
        let path = sc.p.meta.bin_name.as_ref().unwrap().clone().replace(" ", "__");
        subcmds.push(path.clone());
        if let Some(ref aliases) = sc.p.meta.aliases {
            for &(n, _) in aliases {
                subcmds.push(path.replace(name, n));
            }
        }
    }
    for sc_v in p.subcommands.iter().map(|s| get_all_subcommand_paths(&s.p, false)) {
        subcmds.extend(sc_v);
    }
    subcmds
}

/// Generate a completions file for a specified shell at compile time.
///
/// **NOTE:** to generate the this file at compile time you must use a `build.rs` "Build Script"
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
/// use clap::{App, Arg, SubCommand};
///
/// pub fn build_cli() -> App<'static, 'static> {
///     App::new("compl")
///         .about("Tests completions")
///         .arg(Arg::with_name("file")
///             .help("some input file"))
///         .subcommand(SubCommand::with_name("test")
///             .about("tests things")
///             .arg(Arg::with_name("case")
///                 .long("case")
///                 .takes_value(true)
///                 .help("the case to test")))
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
/// clap = "2.23"
/// ```
///
/// Next, we place a `build.rs` in our project root.
///
/// ```ignore
/// extern crate clap;
///
/// use clap::Shell;
///
/// include!("src/cli.rs");
///
/// fn main() {
///     let outdir = match env::var_os("OUT_DIR") {
///         None => return,
///         Some(outdir) => outdir,
///     };
///     let mut app = build_cli();
///     app.gen_completions("myapp",      // We need to specify the bin name manually
///                         Shell::Bash,  // Then say which shell to build completions for
///                         outdir);      // Then say where write the completions to
/// }
/// ```
/// Now, once we combile there will be a `{bin_name}.bash-completion` file in the directory.
/// Assuming we compiled with debug mode, it would be somewhere similar to
/// `<project>/target/debug/build/myapp-<hash>/out/myapp.bash-completion`.
///
/// Fish shell completions will use the file format `{bin_name}.fish`
pub fn generate<T: Into<OsString>, S: Into<String>>(app: &mut App,
                                                    bin_name: S,
                                                    for_shell: Shell,
                                                    out_dir: T) {
    // TODO-v3-beta: implement completion initialization
    unimplemented!();

    app.p.meta.bin_name = Some(bin_name.into());
    app.p.gen_completions(for_shell, out_dir.into());
}


/// Generate a completions file for a specified shell at runtime.  Until `cargo install` can
/// install extra files like a completion script, this may be used e.g. in a command that
/// outputs the contents of the completion script, to be redirected into a file by the user.
///
/// # Examples
///
/// Assuming a separate `cli.rs` like the [example above](./struct.App.html#method.gen_completions),
/// we can let users generate a completion script using a command:
///
/// ```ignore
/// // src/main.rs
///
/// mod cli;
/// use std::io;
///
/// fn main() {
///     let matches = cli::build_cli().get_matches();
///
///     if matches.is_present("generate-bash-completions") {
///         cli::build_cli().gen_completions_to("myapp", Shell::Bash, &mut io::stdout());
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
/// $ myapp generate-bash-completions > /etc/bash_completion.d/myapp
/// ```
pub fn generate_to<W: Write, S: Into<String>>(app: &mut App,
                                              bin_name: S,
                                              for_shell: Shell,
                                              buf: &mut W) {
    // TODO-v3-beta: implement completion initialization
    unimplemented!();

    // let p: Parser = app.build();
    // p.meta.bin_name = Some(bin_name.into());
    // p.gen_completions_to(for_shell, buf);
}