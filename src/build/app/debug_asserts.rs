use crate::{
    build::arg::{debug_asserts::assert_arg, ArgProvider},
    util::Id,
    App, AppSettings, ArgSettings, ValueHint,
};
use std::cmp::Ordering;

pub(crate) fn assert_app(app: &App) {
    debug!("App::_debug_asserts");

    let mut short_flags = vec![];
    let mut long_flags = vec![];

    // Invalid version flag settings
    if app.version.is_none() && app.long_version.is_none() {
        // PropagateVersion is meaningless if there is no version
        assert!(
            !app.settings.is_set(AppSettings::PropagateVersion),
            "App {}: No version information via App::version or App::long_version to propagate",
            app.get_name(),
        );

        // Used `App::mut_arg("version", ..) but did not provide any version information to display
        let has_mutated_version = app
            .args
            .args()
            .any(|x| x.id == Id::version_hash() && x.provider == ArgProvider::GeneratedMutated);

        if has_mutated_version {
            assert!(app.settings.is_set(AppSettings::NoAutoVersion),
                "App {}: Used App::mut_arg(\"version\", ..) without providing App::version, App::long_version or using AppSettings::NoAutoVersion"
            ,app.get_name()
                );
        }
    }

    for sc in &app.subcommands {
        if let Some(s) = sc.short_flag.as_ref() {
            short_flags.push(Flag::App(format!("-{}", s), &sc.name));
        }

        for (short_alias, _) in &sc.short_flag_aliases {
            short_flags.push(Flag::App(format!("-{}", short_alias), &sc.name));
        }

        if let Some(l) = sc.long_flag.as_ref() {
            long_flags.push(Flag::App(format!("--{}", l), &sc.name));
        }

        for (long_alias, _) in &sc.long_flag_aliases {
            long_flags.push(Flag::App(format!("--{}", long_alias), &sc.name));
        }
    }

    for arg in app.args.args() {
        assert_arg(arg);

        if let Some(s) = arg.short.as_ref() {
            short_flags.push(Flag::Arg(format!("-{}", s), &*arg.name));
        }

        for (short_alias, _) in &arg.short_aliases {
            short_flags.push(Flag::Arg(format!("-{}", short_alias), arg.name));
        }

        if let Some(l) = arg.long.as_ref() {
            long_flags.push(Flag::Arg(format!("--{}", l), &*arg.name));
        }

        for (long_alias, _) in &arg.aliases {
            long_flags.push(Flag::Arg(format!("--{}", long_alias), arg.name));
        }

        // Name conflicts
        assert!(
            app.two_args_of(|x| x.id == arg.id).is_none(),
            "App {}: Argument names must be unique, but '{}' is in use by more than one argument or group",
            app.get_name(),
            arg.name,
        );

        // Long conflicts
        if let Some(l) = arg.long {
            if let Some((first, second)) = app.two_args_of(|x| x.long == Some(l)) {
                panic!(
                    "App {}: Long option names must be unique for each argument, \
                        but '--{}' is in use by both '{}' and '{}'",
                    app.get_name(),
                    l,
                    first.name,
                    second.name
                )
            }
        }

        // Short conflicts
        if let Some(s) = arg.short {
            if let Some((first, second)) = app.two_args_of(|x| x.short == Some(s)) {
                panic!(
                    "App {}: Short option names must be unique for each argument, \
                        but '-{}' is in use by both '{}' and '{}'",
                    app.get_name(),
                    s,
                    first.name,
                    second.name
                )
            }
        }

        // Index conflicts
        if let Some(idx) = arg.index {
            if let Some((first, second)) =
                app.two_args_of(|x| x.is_positional() && x.index == Some(idx))
            {
                panic!(
                    "App {}: Argument '{}' has the same index as '{}' \
                    and they are both positional arguments\n\n\t \
                    Use Arg::multiple_values(true) to allow one \
                    positional argument to take multiple values",
                    app.get_name(),
                    first.name,
                    second.name
                )
            }
        }

        // requires, r_if, r_unless
        for req in &arg.requires {
            assert!(
                app.id_exists(&req.1),
                "App {}: Argument or group '{:?}' specified in 'requires*' for '{}' does not exist",
                app.get_name(),
                req.1,
                arg.name,
            );
        }

        for req in &arg.r_ifs {
            assert!(
                app.id_exists(&req.0),
                "App {}: Argument or group '{:?}' specified in 'required_if_eq*' for '{}' does not exist",
                    app.get_name(),
                req.0,
                arg.name
            );
        }

        for req in &arg.r_ifs_all {
            assert!(
                app.id_exists(&req.0),
                "App {}: Argument or group '{:?}' specified in 'required_if_eq_all' for '{}' does not exist",
                    app.get_name(),
                req.0,
                arg.name
            );
        }

        for req in &arg.r_unless {
            assert!(
                app.id_exists(req),
                "App {}: Argument or group '{:?}' specified in 'required_unless*' for '{}' does not exist",
                    app.get_name(),
                req,
                arg.name,
            );
        }

        // blacklist
        for req in &arg.blacklist {
            assert!(
                app.id_exists(req),
                "App {}: Argument or group '{:?}' specified in 'conflicts_with*' for '{}' does not exist",
                    app.get_name(),
                req,
                arg.name,
            );
        }

        if arg.is_set(ArgSettings::Last) {
            assert!(
                arg.long.is_none(),
                "App {}: Flags or Options cannot have last(true) set. '{}' has both a long and last(true) set.",
                    app.get_name(),
                arg.name
            );
            assert!(
                arg.short.is_none(),
                "App {}: Flags or Options cannot have last(true) set. '{}' has both a short and last(true) set.",
                    app.get_name(),
                arg.name
            );
        }

        assert!(
            !(arg.is_set(ArgSettings::Required) && arg.get_global()),
            "App {}: Global arguments cannot be required.\n\n\t'{}' is marked as both global and required",
                    app.get_name(),
            arg.name
        );

        // validators
        assert!(
            arg.validator.is_none() || arg.validator_os.is_none(),
            "App {}: Argument '{}' has both `validator` and `validator_os` set which is not allowed",
                    app.get_name(),
            arg.name
        );

        if arg.value_hint == ValueHint::CommandWithArguments {
            assert!(
                arg.is_positional(),
                "App {}: Argument '{}' has hint CommandWithArguments and must be positional.",
                app.get_name(),
                arg.name
            );

            assert!(
                app.is_set(AppSettings::TrailingVarArg),
                "App {}: Positional argument '{}' has hint CommandWithArguments, so App must have TrailingVarArg set.",
                    app.get_name(),
                arg.name
            );
        }
    }

    for group in &app.groups {
        // Name conflicts
        assert!(
            app.groups.iter().filter(|x| x.id == group.id).count() < 2,
            "App {}: Argument group name must be unique\n\n\t'{}' is already in use",
            app.get_name(),
            group.name,
        );

        // Groups should not have naming conflicts with Args
        assert!(
            !app.args.args().any(|x| x.id == group.id),
            "App {}: Argument group name '{}' must not conflict with argument name",
            app.get_name(),
            group.name,
        );

        // Required groups should have at least one arg without default values
        if group.required && !group.args.is_empty() {
            assert!(
                group.args.iter().any(|arg| {
                    app.args
                        .args()
                        .any(|x| x.id == *arg && x.default_vals.is_empty())
                }),
                "App {}: Argument group '{}' is required but all of it's arguments have a default value.",
                    app.get_name(),
                group.name
            )
        }

        for arg in &group.args {
            // Args listed inside groups should exist
            assert!(
                app.args.args().any(|x| x.id == *arg),
                "App {}: Argument group '{}' contains non-existent argument '{:?}'",
                app.get_name(),
                group.name,
                arg
            );
        }
    }

    // Conflicts between flags and subcommands

    long_flags.sort_unstable();
    short_flags.sort_unstable();

    detect_duplicate_flags(&long_flags, "long");
    detect_duplicate_flags(&short_flags, "short");

    if let Some(help_template) = app.template {
        assert!(
            !help_template.contains("{flags}"),
            "App {}: {}",
                    app.get_name(),
            "`{flags}` template variable was removed in clap3, they are now included in `{options}`",
        );
        assert!(
            !help_template.contains("{unified}"),
            "App {}: {}",
            app.get_name(),
            "`{unified}` template variable was removed in clap3, use `{options}` instead"
        );
    }

    app._panic_on_missing_help(app.g_settings.is_set(AppSettings::HelpExpected));
    assert_app_flags(app);
}

#[derive(Eq)]
enum Flag<'a> {
    App(String, &'a str),
    Arg(String, &'a str),
}

impl PartialEq for Flag<'_> {
    fn eq(&self, other: &Flag) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl PartialOrd for Flag<'_> {
    fn partial_cmp(&self, other: &Flag) -> Option<Ordering> {
        use Flag::*;

        match (self, other) {
            (App(s1, _), App(s2, _))
            | (Arg(s1, _), Arg(s2, _))
            | (App(s1, _), Arg(s2, _))
            | (Arg(s1, _), App(s2, _)) => {
                if s1 == s2 {
                    Some(Ordering::Equal)
                } else {
                    s1.partial_cmp(s2)
                }
            }
        }
    }
}

impl Ord for Flag<'_> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

fn detect_duplicate_flags(flags: &[Flag], short_or_long: &str) {
    use Flag::*;

    for (one, two) in find_duplicates(flags) {
        match (one, two) {
            (App(flag, one), App(_, another)) if one != another => panic!(
                "the '{}' {} flag is specified for both '{}' and '{}' subcommands",
                flag, short_or_long, one, another
            ),

            (Arg(flag, one), Arg(_, another)) if one != another => panic!(
                "{} option names must be unique, but '{}' is in use by both '{}' and '{}'",
                short_or_long, flag, one, another
            ),

            (Arg(flag, arg), App(_, sub)) | (App(flag, sub), Arg(_, arg)) => panic!(
                "the '{}' {} flag for the '{}' argument conflicts with the short flag \
                     for '{}' subcommand",
                flag, short_or_long, arg, sub
            ),

            _ => {}
        }
    }
}

/// Find duplicates in a sorted array.
///
/// The algorithm is simple: the array is sorted, duplicates
/// must be placed next to each other, we can check only adjacent elements.
fn find_duplicates<T: PartialEq>(slice: &[T]) -> impl Iterator<Item = (&T, &T)> {
    slice.windows(2).filter_map(|w| {
        if w[0] == w[1] {
            Some((&w[0], &w[1]))
        } else {
            None
        }
    })
}

fn assert_app_flags(app: &App) {
    use AppSettings::*;

    macro_rules! checker {
        ($a:ident requires $($b:ident)|+) => {
            if app.is_set($a) {
                let mut s = String::new();

                $(
                    if !app.is_set($b) {
                        s.push_str(&format!("  AppSettings::{} is required when AppSettings::{} is set.\n", std::stringify!($b), std::stringify!($a)));
                    }
                )+

                if !s.is_empty() {
                    panic!("{}", s)
                }
            }
        };
        ($a:ident conflicts $($b:ident)|+) => {
            if app.is_set($a) {
                let mut s = String::new();

                $(
                    if app.is_set($b) {
                        s.push_str(&format!("  AppSettings::{} conflicts with AppSettings::{}.\n", std::stringify!($b), std::stringify!($a)));
                    }
                )+

                if !s.is_empty() {
                    panic!("{}\n{}", app.get_name(), s)
                }
            }
        };
    }

    checker!(AllowInvalidUtf8ForExternalSubcommands requires AllowExternalSubcommands);
    #[cfg(feature = "unstable-multicall")]
    checker!(Multicall conflicts NoBinaryName);
}
