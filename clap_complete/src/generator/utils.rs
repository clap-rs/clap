//! Helpers for writing generators

use clap::{App, Arg};

/// Gets all subcommands including child subcommands in the form of `("name", "bin_name")`.
///
/// Subcommand `rustup toolchain install` would be converted to
/// `("install", "rustup toolchain install")`.
pub fn all_subcommands(app: &App) -> Vec<(String, String)> {
    let mut subcmds: Vec<_> = subcommands(app);

    for sc_v in app.get_subcommands().map(all_subcommands) {
        subcmds.extend(sc_v);
    }

    subcmds
}

/// Finds the subcommand [`clap::App`] from the given [`clap::App`] with the given path.
///
/// **NOTE:** `path` should not contain the root `bin_name`.
pub fn find_subcommand_with_path<'help, 'app>(
    p: &'app App<'help>,
    path: Vec<&str>,
) -> &'app App<'help> {
    let mut app = p;

    for sc in path {
        app = app.find_subcommand(sc).unwrap();
    }

    app
}

/// Gets subcommands of [`clap::App`] in the form of `("name", "bin_name")`.
///
/// Subcommand `rustup toolchain install` would be converted to
/// `("install", "rustup toolchain install")`.
pub fn subcommands(p: &App) -> Vec<(String, String)> {
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

/// Gets all the short options, their visible aliases and flags of a [`clap::App`].
/// Includes `h` and `V` depending on the [`clap::AppSettings`].
pub fn shorts_and_visible_aliases(p: &App) -> Vec<char> {
    debug!("shorts: name={}", p.get_name());

    p.get_arguments()
        .filter_map(|a| {
            if !a.is_positional() {
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
        .collect()
}

/// Gets all the long options, their visible aliases and flags of a [`clap::App`].
/// Includes `help` and `version` depending on the [`clap::AppSettings`].
pub fn longs_and_visible_aliases(p: &App) -> Vec<String> {
    debug!("longs: name={}", p.get_name());

    p.get_arguments()
        .filter_map(|a| {
            if !a.is_positional() {
                if a.get_visible_aliases().is_some() && a.get_long().is_some() {
                    let mut visible_aliases: Vec<_> = a
                        .get_visible_aliases()
                        .unwrap()
                        .into_iter()
                        .map(|s| s.to_string())
                        .collect();
                    visible_aliases.push(a.get_long().unwrap().to_string());
                    Some(visible_aliases)
                } else if a.get_visible_aliases().is_none() && a.get_long().is_some() {
                    Some(vec![a.get_long().unwrap().to_string()])
                } else {
                    None
                }
            } else {
                None
            }
        })
        .flatten()
        .collect()
}

/// Gets all the flags of a [`clap::App`](App).
/// Includes `help` and `version` depending on the [`clap::AppSettings`].
pub fn flags<'help>(p: &App<'help>) -> Vec<Arg<'help>> {
    debug!("flags: name={}", p.get_name());
    p.get_arguments()
        .filter(|a| !a.is_takes_value_set() && !a.is_positional())
        .cloned()
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::Arg;
    use pretty_assertions::assert_eq;

    fn common_app() -> App<'static> {
        App::new("myapp")
            .subcommand(
                App::new("test").subcommand(App::new("config")).arg(
                    Arg::new("file")
                        .short('f')
                        .short_alias('c')
                        .visible_short_alias('p')
                        .long("file")
                        .visible_alias("path"),
                ),
            )
            .subcommand(App::new("hello"))
            .bin_name("my-app")
    }

    fn built() -> App<'static> {
        let mut app = common_app();

        app._build_all();
        app
    }

    fn built_with_version() -> App<'static> {
        let mut app = common_app().version("3.0");

        app._build_all();
        app
    }

    #[test]
    fn test_subcommands() {
        let app = built_with_version();

        assert_eq!(
            subcommands(&app),
            vec![
                ("test".to_string(), "my-app test".to_string()),
                ("hello".to_string(), "my-app hello".to_string()),
                ("help".to_string(), "my-app help".to_string()),
            ]
        );
    }

    #[test]
    fn test_all_subcommands() {
        let app = built_with_version();

        assert_eq!(
            all_subcommands(&app),
            vec![
                ("test".to_string(), "my-app test".to_string()),
                ("hello".to_string(), "my-app hello".to_string()),
                ("help".to_string(), "my-app help".to_string()),
                ("config".to_string(), "my-app test config".to_string()),
                ("help".to_string(), "my-app test help".to_string()),
            ]
        );
    }

    #[test]
    fn test_find_subcommand_with_path() {
        let app = built_with_version();
        let sc_app = find_subcommand_with_path(&app, "test config".split(' ').collect());

        assert_eq!(sc_app.get_name(), "config");
    }

    #[test]
    fn test_flags() {
        let app = built_with_version();
        let actual_flags = flags(&app);

        assert_eq!(actual_flags.len(), 2);
        assert_eq!(actual_flags[0].get_long(), Some("help"));
        assert_eq!(actual_flags[1].get_long(), Some("version"));

        let sc_flags = flags(find_subcommand_with_path(&app, vec!["test"]));

        assert_eq!(sc_flags.len(), 2);
        assert_eq!(sc_flags[0].get_long(), Some("file"));
        assert_eq!(sc_flags[1].get_long(), Some("help"));
    }

    #[test]
    fn test_flag_subcommand() {
        let app = built();
        let actual_flags = flags(&app);

        assert_eq!(actual_flags.len(), 1);
        assert_eq!(actual_flags[0].get_long(), Some("help"));

        let sc_flags = flags(find_subcommand_with_path(&app, vec!["test"]));

        assert_eq!(sc_flags.len(), 2);
        assert_eq!(sc_flags[0].get_long(), Some("file"));
        assert_eq!(sc_flags[1].get_long(), Some("help"));
    }

    #[test]
    fn test_shorts() {
        let app = built_with_version();
        let shorts = shorts_and_visible_aliases(&app);

        assert_eq!(shorts.len(), 2);
        assert_eq!(shorts[0], 'h');
        assert_eq!(shorts[1], 'V');

        let sc_shorts = shorts_and_visible_aliases(find_subcommand_with_path(&app, vec!["test"]));

        assert_eq!(sc_shorts.len(), 3);
        assert_eq!(sc_shorts[0], 'p');
        assert_eq!(sc_shorts[1], 'f');
        assert_eq!(sc_shorts[2], 'h');
    }

    #[test]
    fn test_longs() {
        let app = built_with_version();
        let longs = longs_and_visible_aliases(&app);

        assert_eq!(longs.len(), 2);
        assert_eq!(longs[0], "help");
        assert_eq!(longs[1], "version");

        let sc_longs = longs_and_visible_aliases(find_subcommand_with_path(&app, vec!["test"]));

        assert_eq!(sc_longs.len(), 3);
        assert_eq!(sc_longs[0], "path");
        assert_eq!(sc_longs[1], "file");
        assert_eq!(sc_longs[2], "help");
    }
}
