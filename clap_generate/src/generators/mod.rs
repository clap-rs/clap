mod shells;

// Std
use std::io::Write;

// Internal
use clap::{App, AppSettings, Arg};
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

    /// Gets all subcommands including child subcommands in the form of `("name", "bin_name")`.
    ///
    /// Subcommand `rustup toolchain install` would be converted to
    /// `("install", "rustup toolchain install")`.
    fn all_subcommands(app: &App) -> Vec<(String, String)> {
        let mut subcmds: Vec<_> = Self::subcommands(app);

        for sc_v in app.get_subcommands().map(|s| Self::all_subcommands(&s)) {
            subcmds.extend(sc_v);
        }

        subcmds
    }

    /// Finds the subcommand [`clap::App`][clap] from the given [`clap::App`][clap] with the given path.
    ///
    /// **NOTE:** `path` should not contain the root `bin_name`.
    ///
    /// [clap]: ../clap/struct.App.html
    fn find_subcommand_with_path<'help, 'app>(
        p: &'app App<'help>,
        path: Vec<&str>,
    ) -> &'app App<'help> {
        let mut app = p;

        for sc in path {
            app = app.find_subcommand(sc).unwrap();
        }

        app
    }

    /// Gets subcommands of [`clap::App`](../clap/struct.App.html) in the form of `("name", "bin_name")`.
    ///
    /// Subcommand `rustup toolchain install` would be converted to
    /// `("install", "rustup toolchain install")`.
    fn subcommands(p: &App) -> Vec<(String, String)> {
        debug!("subcommands: name={}", p.get_name());
        debug!("subcommands: Has subcommands...{:?}", p.has_subcommands());

        let mut subcmds = vec![];

        if !p.has_subcommands() {
            return subcmds;
        }

        for sc in p.get_subcommands() {
            let sc_bin_name = sc.get_bin_name().unwrap();

            debug!(
                "subcommands:iter: name={}, bin_name={}",
                sc.get_name(),
                sc_bin_name
            );

            subcmds.push((sc.get_name().to_string(), sc_bin_name.to_string()));
        }

        subcmds
    }

    /// Gets all the short options, their visible aliases and flags of a [`clap::App`](../clap/struct.App.html).
    /// Includes `h` and `V` depending on the [`clap::AppSettings`](../clap/enum.AppSettings.html).
    fn shorts_and_visible_aliases<'help>(p: &App<'help>) -> Vec<char> {
        debug!("shorts: name={}", p.get_name());

        let mut shorts_and_visible_aliases: Vec<char> = p
            .get_arguments()
            .filter_map(|a| {
                if a.get_index().is_none() {
                    if a.get_visible_short_aliases().is_some() && a.get_short().is_some() {
                        let mut shorts_and_visible_aliases = a.get_visible_short_aliases().unwrap();
                        shorts_and_visible_aliases.push(a.get_short().unwrap());
                        Some(shorts_and_visible_aliases)
                    } else if a.get_visible_short_aliases().is_none() && a.get_short().is_some() {
                        Some(vec![a.get_short().unwrap()])
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .flatten()
            .collect();

        if !shorts_and_visible_aliases.iter().any(|&x| x == 'h') {
            shorts_and_visible_aliases.push('h');
        }

        if !p.is_set(AppSettings::DisableVersionFlag)
            && !shorts_and_visible_aliases.iter().any(|&x| x == 'V')
        {
            shorts_and_visible_aliases.push('V');
        }

        shorts_and_visible_aliases
    }

    /// Gets all the long options and flags of a [`clap::App`](../clap/struct.App.html).
    /// Includes `help` and `version` depending on the [`clap::AppSettings`](../clap/enum.AppSettings.html).
    fn longs<'help>(p: &App<'help>) -> Vec<String> {
        debug!("longs: name={}", p.get_name());

        let mut longs: Vec<String> = p
            .get_arguments()
            .filter_map(|a| {
                if a.get_index().is_none() && a.get_long().is_some() {
                    Some(a.get_long().unwrap().to_string())
                } else {
                    None
                }
            })
            .collect();

        if !longs.iter().any(|x| *x == "help") {
            longs.push(String::from("help"));
        }

        if !p.is_set(AppSettings::DisableVersionFlag) && !longs.iter().any(|x| *x == "version") {
            longs.push(String::from("version"));
        }

        longs
    }

    /// Gets all the flags of a [`clap::App`](../clap/struct.App.html).
    /// Includes `help` and `version` depending on the [`clap::AppSettings`](../clap/enum.AppSettings.html).
    fn flags<'help>(p: &App<'help>) -> Vec<Arg<'help>> {
        debug!("flags: name={}", p.get_name());

        let mut flags: Vec<_> = p.get_flags().cloned().collect();

        if !flags.iter().any(|x| x.get_name() == "help") {
            flags.push(
                Arg::new("help")
                    .short('h')
                    .long("help")
                    .about("Prints help information"),
            );
        }

        if !p.is_set(AppSettings::DisableVersionFlag)
            && !flags.iter().any(|x| x.get_name() == "version")
        {
            flags.push(
                Arg::new("version")
                    .short('V')
                    .long("version")
                    .about("Prints version information"),
            );
        }

        flags
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    struct Foo;

    impl Generator for Foo {
        fn generate(_: &App, _: &mut dyn Write) {}

        fn file_name(name: &str) -> String {
            name.to_string()
        }
    }

    fn common() -> App<'static> {
        let mut app = App::new("myapp")
            .subcommand(
                App::new("test").subcommand(App::new("config")).arg(
                    Arg::new("file")
                        .short('f')
                        .short_alias('c')
                        .visible_short_alias('p')
                        .long("file"),
                ),
            )
            .subcommand(App::new("hello"))
            .bin_name("my-app");

        app._build();
        app._build_bin_names();
        app
    }

    #[test]
    fn test_subcommands() {
        let app = common();

        assert_eq!(
            Foo::subcommands(&app),
            vec![
                ("test".to_string(), "my-app test".to_string()),
                ("hello".to_string(), "my-app hello".to_string()),
                ("help".to_string(), "my-app help".to_string()),
            ]
        );
    }

    #[test]
    fn test_all_subcommands() {
        let app = common();

        assert_eq!(
            Foo::all_subcommands(&app),
            vec![
                ("test".to_string(), "my-app test".to_string()),
                ("hello".to_string(), "my-app hello".to_string()),
                ("help".to_string(), "my-app help".to_string()),
                ("config".to_string(), "my-app test config".to_string()),
            ]
        );
    }

    #[test]
    fn test_find_subcommand_with_path() {
        let app = common();
        let sc_app = Foo::find_subcommand_with_path(&app, "test config".split(' ').collect());

        assert_eq!(sc_app.get_name(), "config");
    }

    #[test]
    fn test_flags() {
        let app = common();
        let flags = Foo::flags(&app);

        assert_eq!(flags.len(), 2);
        assert_eq!(flags[0].get_long(), Some("help"));
        assert_eq!(flags[1].get_long(), Some("version"));

        let sc_flags = Foo::flags(Foo::find_subcommand_with_path(&app, vec!["test"]));

        assert_eq!(sc_flags.len(), 3);
        assert_eq!(sc_flags[0].get_long(), Some("file"));
        assert_eq!(sc_flags[1].get_long(), Some("help"));
        assert_eq!(sc_flags[2].get_long(), Some("version"));
    }

    #[test]
    fn test_shorts() {
        let app = common();
        let shorts = Foo::shorts_and_visible_aliases(&app);

        assert_eq!(shorts.len(), 2);
        assert_eq!(shorts[0], 'h');
        assert_eq!(shorts[1], 'V');

        let sc_shorts =
            Foo::shorts_and_visible_aliases(Foo::find_subcommand_with_path(&app, vec!["test"]));

        assert_eq!(sc_shorts.len(), 4);
        assert_eq!(sc_shorts[0], 'p');
        assert_eq!(sc_shorts[1], 'f');
        assert_eq!(sc_shorts[2], 'h');
        assert_eq!(sc_shorts[3], 'V');
    }

    #[test]
    fn test_longs() {
        let app = common();
        let longs = Foo::longs(&app);

        assert_eq!(longs.len(), 2);
        assert_eq!(longs[0], "help");
        assert_eq!(longs[1], "version");

        let sc_longs = Foo::longs(Foo::find_subcommand_with_path(&app, vec!["test"]));

        assert_eq!(sc_longs.len(), 3);
        assert_eq!(sc_longs[0], "file");
        assert_eq!(sc_longs[1], "help");
        assert_eq!(sc_longs[2], "version");
    }
}
