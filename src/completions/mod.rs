#[macro_use]
mod macros;
mod bash;
mod fish;
mod zsh;
mod powershell;
mod elvish;
mod shell;

// Std
use std::io::Write;

// Internal
use build::App;
use self::bash::BashGen;
use self::fish::FishGen;
use self::zsh::ZshGen;
use self::powershell::PowerShellGen;
use self::elvish::ElvishGen;
pub use self::shell::Shell;

pub struct ComplGen<'a, 'b>(&'b App<'a, 'b>)
where
    'a: 'b;

impl<'a, 'b> ComplGen<'a, 'b> {
    pub fn new(app: &'b App<'a, 'b>) -> Self { ComplGen(app) }

    pub fn generate<W: Write>(&self, for_shell: Shell, buf: &mut W) {
        match for_shell {
            Shell::Elvish => ElvishGen::new(self.p).generate_to(buf),
            Shell::Bash => BashGen::new(self.0).generate_to(buf),
            Shell::Fish => FishGen::new(self.0).generate_to(buf),
            Shell::Zsh => ZshGen::new(self.0).generate_to(buf),
            Shell::PowerShell => PowerShellGen::new(self.0).generate_to(buf),
        }
    }
}

// Gets all subcommands including child subcommands in the form of 'name' where the name
// is a single word (i.e. "install")  of the path to said subcommand (i.e.
// "rustup toolchain install")
//
// Also note, aliases are treated as their own subcommands but duplicates of whatever they're
// aliasing.
pub fn all_subcommand_names(p: &App) -> Vec<String> {
    debugln!("all_subcommand_names;");
    let mut subcmds: Vec<_> = subcommands_of(p)
        .iter()
        .map(|&(ref n, _)| n.clone())
        .collect();
    for sc_v in subcommands!(p).map(|s| all_subcommand_names(&s)) {
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
pub fn all_subcommands(p: &App) -> Vec<(String, String)> {
    debugln!("all_subcommands;");
    let mut subcmds: Vec<_> = subcommands_of(p);
    for sc_v in subcommands!(p).map(|s| all_subcommands(&s)) {
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
pub fn subcommands_of(p: &App) -> Vec<(String, String)> {
    debugln!(
        "subcommands_of: name={}, bin_name={}",
        p.name,
        p.bin_name.as_ref().unwrap()
    );
    let mut subcmds = vec![];

    debugln!(
        "subcommands_of: Has subcommands...{:?}",
        p.has_subcommands()
    );
    if !p.has_subcommands() {
        let mut ret = vec![];
        debugln!("subcommands_of: Looking for aliases...");
        if let Some(ref aliases) = p.aliases {
            for &(n, _) in aliases {
                debugln!("subcommands_of:iter:iter: Found alias...{}", n);
                let mut als_bin_name: Vec<_> = p.bin_name.as_ref().unwrap().split(' ').collect();
                als_bin_name.push(n);
                let old = als_bin_name.len() - 2;
                als_bin_name.swap_remove(old);
                ret.push((n.to_owned(), als_bin_name.join(" ")));
            }
        }
        return ret;
    }
    for sc in subcommands!(p) {
        debugln!(
            "subcommands_of:iter: name={}, bin_name={}",
            sc.name,
            sc.bin_name.as_ref().unwrap()
        );

        debugln!("subcommands_of:iter: Looking for aliases...");
        if let Some(ref aliases) = sc.aliases {
            for &(n, _) in aliases {
                debugln!("subcommands_of:iter:iter: Found alias...{}", n);
                let mut als_bin_name: Vec<_> = p.bin_name.as_ref().unwrap().split(' ').collect();
                als_bin_name.push(n);
                let old = als_bin_name.len() - 2;
                als_bin_name.swap_remove(old);
                subcmds.push((n.to_owned(), als_bin_name.join(" ")));
            }
        }
        subcmds.push((sc.name.clone(), sc.bin_name.as_ref().unwrap().clone()));
    }
    subcmds
}

pub fn get_all_subcommand_paths(p: &App, first: bool) -> Vec<String> {
    debugln!("get_all_subcommand_paths;");
    let mut subcmds = vec![];
    if !p.has_subcommands() {
        if !first {
            let name = &*p.name;
            let path = p.bin_name.as_ref().unwrap().clone().replace(" ", "__");
            let mut ret = vec![path.clone()];
            if let Some(ref aliases) = p.aliases {
                for &(n, _) in aliases {
                    ret.push(path.replace(name, n));
                }
            }
            return ret;
        }
        return vec![];
    }
    for sc in subcommands!(p) {
        let name = &*sc.name;
        let path = sc.bin_name.as_ref().unwrap().clone().replace(" ", "__");
        subcmds.push(path.clone());
        if let Some(ref aliases) = sc.aliases {
            for &(n, _) in aliases {
                subcmds.push(path.replace(name, n));
            }
        }
    }
    for sc_v in subcommands!(p).map(|s| get_all_subcommand_paths(&s, false)) {
        subcmds.extend(sc_v);
    }
    subcmds
}
