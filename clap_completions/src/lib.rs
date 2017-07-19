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
use parsing::Parser;
use App;
use self::bash::BashGen;
use self::fish::FishGen;
use self::zsh::ZshGen;
use self::powershell::PowerShellGen;
pub use self::shell::Shell;

pub struct ComplGen<'a, 'b>
where
    'a: 'b,
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
    let mut subcmds: Vec<_> = subcommands_of(p)
        .iter()
        .map(|&(ref n, _)| n.clone())
        .collect();
    for sc_v in p.app.subcommands.iter().map(|s| all_subcommand_names(&s.p)) {
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
    for sc_v in p.app.subcommands.iter().map(|s| all_subcommands(&s.p)) {
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
    debugln!(
        "subcommands_of: name={}, bin_name={}",
        p.app.name,
        p.app.bin_name.as_ref().unwrap()
    );
    let mut subcmds = vec![];

    debugln!(
        "subcommands_of: Has subcommands...{:?}",
        p.has_subcommands()
    );
    if !p.has_subcommands() {
        let mut ret = vec![
            (p.app.name.clone(), p.app.bin_name.as_ref().unwrap().clone()),
        ];
        debugln!("subcommands_of: Looking for aliases...");
        if let Some(ref aliases) = p.app.aliases {
            for &(n, _) in aliases {
                debugln!("subcommands_of:iter:iter: Found alias...{}", n);
                let mut als_bin_name: Vec<_> =
                    p.app.bin_name.as_ref().unwrap().split(' ').collect();
                als_bin_name.push(n);
                let old = als_bin_name.len() - 2;
                als_bin_name.swap_remove(old);
                ret.push((n.to_owned(), als_bin_name.join(" ")));
            }
        }
        return ret;
    }
    for sc in &p.subcommands {
        debugln!(
            "subcommands_of:iter: name={}, bin_name={}",
            sc.p.app.name,
            sc.p.app.bin_name.as_ref().unwrap()
        );

        debugln!("subcommands_of:iter: Looking for aliases...");
        if let Some(ref aliases) = sc.p.app.aliases {
            for &(n, _) in aliases {
                debugln!("subcommands_of:iter:iter: Found alias...{}", n);
                let mut als_bin_name: Vec<_> =
                    p.app.bin_name.as_ref().unwrap().split(' ').collect();
                als_bin_name.push(n);
                let old = als_bin_name.len() - 2;
                als_bin_name.swap_remove(old);
                subcmds.push((n.to_owned(), als_bin_name.join(" ")));
            }
        }
        subcmds.push((
            sc.p.app.name.clone(),
            sc.p.app.bin_name.as_ref().unwrap().clone(),
        ));
    }
    subcmds
}

pub fn get_all_subcommand_paths(p: &Parser, first: bool) -> Vec<String> {
    debugln!("get_all_subcommand_paths;");
    let mut subcmds = vec![];
    if !p.has_subcommands() {
        if !first {
            let name = &*p.app.name;
            let path = p.app.bin_name.as_ref().unwrap().clone().replace(" ", "__");
            let mut ret = vec![path.clone()];
            if let Some(ref aliases) = p.app.aliases {
                for &(n, _) in aliases {
                    ret.push(path.replace(name, n));
                }
            }
            return ret;
        }
        return vec![];
    }
    for sc in &p.subcommands {
        let name = &*sc.p.app.name;
        let path = sc.p.app.bin_name.as_ref().unwrap().clone().replace(
            " ",
            "__",
        );
        subcmds.push(path.clone());
        if let Some(ref aliases) = sc.p.app.aliases {
            for &(n, _) in aliases {
                subcmds.push(path.replace(name, n));
            }
        }
    }
    for sc_v in p.subcommands
        .iter()
        .map(|s| get_all_subcommand_paths(&s.p, false))
    {
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
///         .arg(Arg::new("file")
///             .help("some input file"))
///         .subcommand(SubCommand::with_name("test")
///             .about("tests things")
///             .arg(Arg::new("case")
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
pub fn generate<T: Into<OsString>, S: Into<String>>(
    app: &mut App,
    bin_name: S,
    for_shell: Shell,
    out_dir: T,
) {
    // @TODO-v3-beta: implement completion initialization
    unimplemented!();

    app.p.app.bin_name = Some(bin_name.into());
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
pub fn generate_to<W: Write, S: Into<String>>(
    app: &mut App,
    bin_name: S,
    for_shell: Shell,
    buf: &mut W,
) {
    // @TODO-v3-beta: implement completion initialization
    unimplemented!();

    // let p: Parser = app.build();
    // p.app.bin_name = Some(bin_name.into());
    // p.gen_completions_to(for_shell, buf);
}

fn gen_completions_to<W: Write>(p: &mut Parser, for_shell: Shell, buf: &mut W) {
    if !p.is_set(AS::Propogated) {
        p.propogate_help_version();
        p.build_bin_names();
        p.propogate_globals();
        p.propogate_settings();
        p.set(AS::Propogated);
    }

    ComplGen::new(self).generate(for_shell, buf)
}

fn gen_completions(p: &mut Parser, for_shell: Shell, od: OsString) {
    use std::error::Error;

    let out_dir = PathBuf::from(od);
    let name = &*p.app.bin_name.as_ref().unwrap().clone();
    let file_name = match for_shell {
        Shell::Bash => format!("{}.bash-completion", name),
        Shell::Fish => format!("{}.fish", name),
        Shell::Zsh => format!("_{}", name),
        Shell::PowerShell => format!("_{}.ps1", name),
    };

    let mut file = match File::create(out_dir.join(file_name)) {
        Err(why) => panic!("couldn't create completion file: {}", why.description()),
        Ok(file) => file,
    };
    p.gen_completions_to(for_shell, &mut file)
}

// @TODO-v3-alpha: This should only happen when required, not for all recursively with the
// exception of completions
fn propagate_help_version(p: &mut Parser) {
    debugln!("Parser::propogate_help_version;");
    p.create_help_and_version();
    for sc in &mut p.app.subcommands {
        sc.p.propogate_help_version();
    }
}

fn build_bin_names(p: &mut Parser) {
    debugln!("Parser::build_bin_names;");
    for sc in &mut p.app.subcommands {
        debug!("Parser::build_bin_names:iter: bin_name set...");
        if sc.p.app.bin_name.is_none() {
            sdebugln!("No");
            let bin_name = format!(
                "{}{}{}",
                p.app.bin_name.as_ref().unwrap_or(&self.app.name.clone()),
                if p.app.bin_name.is_some() { " " } else { "" },
                &*sc.p.app.name
            );
            debugln!(
                "Parser::build_bin_names:iter: Setting bin_name of {} to {}",
                p.app.name,
                bin_name
            );
            sc.p.app.bin_name = Some(bin_name);
        } else {
            sdebugln!("yes ({:?})", sc.p.app.bin_name);
        }
        debugln!(
            "Parser::build_bin_names:iter: Calling build_bin_names from...{}",
            sc.p.app.name
        );
        sc.p.build_bin_names();
    }
}

impl<'a, 'b, 'c> Parser<'a, 'b, 'c> {
    /// Check is a given string matches the binary name for this parser
    fn is_bin_name(&self, value: &str) -> bool {
        self.app
            .bin_name
            .as_ref()
            .and_then(|name| Some(value == name))
            .unwrap_or(false)
    }

    // Used for completions:
    /// Check is a given string is an alias for this parser
    fn is_alias(&self, value: &str) -> bool {
        aliases!(self.app).any(|als| als == &value)
    }

    // // Only used for completion scripts due to bin_name messiness
    // #[cfg_attr(feature = "lints", allow(block_in_if_condition_stmt))]
    // pub fn find_subcommand(&'b self, sc: &str) -> Option<&'b App<'a, 'b>> {
    //     debugln!("Parser::find_subcommand: sc={}", sc);
    //     debugln!(
    //         "Parser::find_subcommand: Currently in Parser...{}",
    //         self.app.bin_name.as_ref().unwrap()
    //     );
    //     for s in &self.app.subcommands {
    //         if s.is_bin_name(sc) {
    //             return Some(s);
    //         }
    //         // XXX: why do we split here?
    //         // isn't `sc` supposed to be single word already?
    //         let last = sc.split(' ').rev().next().expect(INTERNAL_ERROR_MSG);
    //         if s.is_alias(last) {
    //             return Some(s);
    //         }

    //         if let Some(app) = s.find_subcommand(sc) {
    //             return Some(app);
    //         }
    //     }
    //     None
    // }

}