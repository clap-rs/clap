use crate::{
    build::arg::{debug_asserts::assert_arg, ArgProvider},
    mkeymap::KeyType,
    util::Id,
    App, AppSettings, Arg, ArgSettings, ValueHint,
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

    _verify_positionals(app);

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

#[cfg(debug_assertions)]
fn _verify_positionals(app: &App) -> bool {
    debug!("App::_verify_positionals");
    // Because you must wait until all arguments have been supplied, this is the first chance
    // to make assertions on positional argument indexes
    //
    // First we verify that the index highest supplied index, is equal to the number of
    // positional arguments to verify there are no gaps (i.e. supplying an index of 1 and 3
    // but no 2)

    let highest_idx = app
        .args
        .keys()
        .filter_map(|x| {
            if let KeyType::Position(n) = x {
                Some(*n)
            } else {
                None
            }
        })
        .max()
        .unwrap_or(0);

    let num_p = app.args.keys().filter(|x| x.is_position()).count();

    assert!(
        highest_idx == num_p,
        "Found positional argument whose index is {} but there \
             are only {} positional arguments defined",
        highest_idx,
        num_p
    );

    // Next we verify that only the highest index has takes multiple arguments (if any)
    let only_highest = |a: &Arg| a.is_multiple() && (a.index.unwrap_or(0) != highest_idx);
    if app.get_positionals().any(only_highest) {
        // First we make sure if there is a positional that allows multiple values
        // the one before it (second to last) has one of these:
        //  * a value terminator
        //  * ArgSettings::Last
        //  * The last arg is Required

        // We can't pass the closure (it.next()) to the macro directly because each call to
        // find() (iterator, not macro) gets called repeatedly.
        let last = &app.args[&KeyType::Position(highest_idx)];
        let second_to_last = &app.args[&KeyType::Position(highest_idx - 1)];

        // Either the final positional is required
        // Or the second to last has a terminator or .last(true) set
        let ok = last.is_set(ArgSettings::Required)
            || (second_to_last.terminator.is_some() || second_to_last.is_set(ArgSettings::Last))
            || last.is_set(ArgSettings::Last);
        assert!(
            ok,
            "When using a positional argument with .multiple_values(true) that is *not the \
                 last* positional argument, the last positional argument (i.e. the one \
                 with the highest index) *must* have .required(true) or .last(true) set."
        );

        // We make sure if the second to last is Multiple the last is ArgSettings::Last
        let ok = second_to_last.is_multiple() || last.is_set(ArgSettings::Last);
        assert!(
            ok,
            "Only the last positional argument, or second to last positional \
                 argument may be set to .multiple_values(true)"
        );

        // Next we check how many have both Multiple and not a specific number of values set
        let count = app
            .get_positionals()
            .filter(|p| {
                p.settings.is_set(ArgSettings::MultipleOccurrences)
                    || (p.settings.is_set(ArgSettings::MultipleValues) && p.num_vals.is_none())
            })
            .count();
        let ok = count <= 1
            || (last.is_set(ArgSettings::Last)
                && last.is_multiple()
                && second_to_last.is_multiple()
                && count == 2);
        assert!(
            ok,
            "Only one positional argument with .multiple_values(true) set is allowed per \
                 command, unless the second one also has .last(true) set"
        );
    }

    let mut found = false;

    if app.is_set(AppSettings::AllowMissingPositional) {
        // Check that if a required positional argument is found, all positions with a lower
        // index are also required.
        let mut foundx2 = false;

        for p in app.get_positionals() {
            if foundx2 && !p.is_set(ArgSettings::Required) {
                assert!(
                    p.is_set(ArgSettings::Required),
                    "Found non-required positional argument with a lower \
                         index than a required positional argument by two or more: {:?} \
                         index {:?}",
                    p.name,
                    p.index
                );
            } else if p.is_set(ArgSettings::Required) && !p.is_set(ArgSettings::Last) {
                // Args that .last(true) don't count since they can be required and have
                // positionals with a lower index that aren't required
                // Imagine: prog <req1> [opt1] -- <req2>
                // Both of these are valid invocations:
                //      $ prog r1 -- r2
                //      $ prog r1 o1 -- r2
                if found {
                    foundx2 = true;
                    continue;
                }
                found = true;
                continue;
            } else {
                found = false;
            }
        }
    } else {
        // Check that if a required positional argument is found, all positions with a lower
        // index are also required
        for p in (1..=num_p).rev().filter_map(|n| app.args.get(&n)) {
            if found {
                assert!(
                    p.is_set(ArgSettings::Required),
                    "Found non-required positional argument with a lower \
                         index than a required positional argument: {:?} index {:?}",
                    p.name,
                    p.index
                );
            } else if p.is_set(ArgSettings::Required) && !p.is_set(ArgSettings::Last) {
                // Args that .last(true) don't count since they can be required and have
                // positionals with a lower index that aren't required
                // Imagine: prog <req1> [opt1] -- <req2>
                // Both of these are valid invocations:
                //      $ prog r1 -- r2
                //      $ prog r1 o1 -- r2
                found = true;
                continue;
            }
        }
    }
    assert!(
        app.get_positionals()
            .filter(|p| p.is_set(ArgSettings::Last))
            .count()
            < 2,
        "Only one positional argument may have last(true) set. Found two."
    );
    if app
        .get_positionals()
        .any(|p| p.is_set(ArgSettings::Last) && p.is_set(ArgSettings::Required))
        && app.has_subcommands()
        && !app.is_set(AppSettings::SubcommandsNegateReqs)
    {
        panic!(
            "Having a required positional argument with .last(true) set *and* child \
                 subcommands without setting SubcommandsNegateReqs isn't compatible."
        );
    }

    true
}
