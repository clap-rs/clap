mod shells;

// Std
use std::io::Write;

// Internal
use clap::*;

pub use shells::*;

/// Generator trait which can be used to write generators
pub trait Generator {
    /// Returns the file name that is created when this generator is called during compile time.
    ///
    /// # Examples
    ///
    /// ```
    /// # use std::io::Write;
    /// # use clap::App;
    /// use clap_generate::Generator;
    ///
    /// pub struct Fish;
    ///
    /// impl Generator for Fish {
    /// #   fn generate(app: &App, buf: &mut dyn Write) {}
    ///     fn file_name(name: &str) -> String {
    ///         format!("{}.fish", name)
    ///     }
    /// }
    /// ```
    fn file_name(name: &str) -> String;

    /// Generates output out of [`clap::App`](../clap/struct.App.html).
    ///
    /// # Examples
    ///
    /// The following example generator displays the [`clap::App`](../clap/struct.App.html)
    /// as if it is printed using [`std::println`](https://doc.rust-lang.org/std/macro.println.html).
    ///
    /// ```
    /// use std::{io::Write, fmt::write};
    /// use clap::App;
    /// use clap_generate::Generator;
    ///
    /// pub struct ClapDebug;
    ///
    /// impl Generator for ClapDebug {
    ///     fn generate(app: &App, buf: &mut dyn Write) {
    ///         write!(buf, "{}", app).unwrap();
    ///     }
    /// #   fn file_name(name: &str) -> String {
    /// #    name.into()
    /// #   }
    /// }
    /// ```
    fn generate(app: &App, buf: &mut dyn Write);

    /// Gets all subcommands including child subcommands in the form of 'name' where the name
    /// is a single word (i.e. "install") of the path to said subcommand (i.e.
    /// "rustup toolchain install")
    ///
    /// Also note, aliases are treated as their own subcommands but duplicates of whatever they're
    /// aliasing.
    fn all_subcommand_names(app: &App) -> Vec<String> {
        debugln!("all_subcommand_names;");

        let mut subcmds: Vec<_> = Self::subcommands_of(app)
            .iter()
            .map(|&(ref n, _)| n.clone())
            .collect();

        for sc_v in subcommands!(app).map(|s| Self::all_subcommand_names(&s)) {
            subcmds.extend(sc_v);
        }

        subcmds.sort();
        subcmds.dedup();
        subcmds
    }

    /// Gets all subcommands including child subcommands in the form of ('name', 'bin_name') where the name
    /// is a single word (i.e. "install") of the path and full bin_name of said subcommand (i.e.
    /// "rustup toolchain install")
    ///
    /// Also note, aliases are treated as their own subcommands but duplicates of whatever they're
    /// aliasing.
    fn all_subcommands(app: &App) -> Vec<(String, String)> {
        debugln!("all_subcommands;");

        let mut subcmds: Vec<_> = Self::subcommands_of(app);

        for sc_v in subcommands!(app).map(|s| Self::all_subcommands(&s)) {
            subcmds.extend(sc_v);
        }

        subcmds
    }

    /// Gets all subcommands exlcuding child subcommands in the form of (name, bin_name) where the name
    /// is a single word (i.e. "install") and the bin_name is a space deliniated list of the path to said
    /// subcommand (i.e. "rustup toolchain install")
    ///
    /// Also note, aliases are treated as their own subcommands but duplicates of whatever they're
    /// aliasing.
    fn subcommands_of(p: &App) -> Vec<(String, String)> {
        debugln!(
            "subcommands_of: name={}, bin_name={}",
            p.name,
            p.bin_name.as_ref().unwrap()
        );
        debugln!(
            "subcommands_of: Has subcommands...{:?}",
            p.has_subcommands()
        );

        let mut subcmds = vec![];

        if !p.has_subcommands() {
            let mut ret = vec![];

            debugln!("subcommands_of: Looking for aliases...");

            if let Some(ref aliases) = p.aliases {
                for &(n, _) in aliases {
                    debugln!("subcommands_of:iter:iter: Found alias...{}", n);

                    let mut als_bin_name: Vec<_> =
                        p.bin_name.as_ref().unwrap().split(' ').collect();

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

                    let mut als_bin_name: Vec<_> =
                        p.bin_name.as_ref().unwrap().split(' ').collect();

                    als_bin_name.push(n);

                    let old = als_bin_name.len() - 2;

                    als_bin_name.swap_remove(old);
                    subcmds.push((n.to_owned(), als_bin_name.join(" ")));
                }
            }

            subcmds.push((sc.name.clone(), sc.get_bin_name().unwrap().to_string()));
        }

        subcmds
    }

    /// TODO
    fn get_all_subcommand_paths(p: &App, first: bool) -> Vec<String> {
        debugln!("get_all_subcommand_paths;");

        let mut subcmds = vec![];

        if !p.has_subcommands() {
            if !first {
                let name = &*p.name;
                let path = p.get_bin_name().unwrap().to_string().replace(" ", "__");
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
            let path = sc.get_bin_name().unwrap().to_string().replace(" ", "__");

            subcmds.push(path.clone());

            if let Some(ref aliases) = sc.aliases {
                for &(n, _) in aliases {
                    subcmds.push(path.replace(name, n));
                }
            }
        }

        for sc_v in subcommands!(p).map(|s| Self::get_all_subcommand_paths(&s, false)) {
            subcmds.extend(sc_v);
        }

        subcmds
    }
}
