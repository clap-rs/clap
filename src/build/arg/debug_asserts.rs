use crate::{Arg, ArgSettings, ValueHint};

pub(crate) fn assert_arg(arg: &Arg) {
    debug!("Arg::_debug_asserts:{}", arg.name);

    // Self conflict
    // TODO: this check should be recursive
    assert!(
        !arg.blacklist.iter().any(|x| *x == arg.id),
        "Argument '{}' cannot conflict with itself",
        arg.name,
    );

    if arg.value_hint != ValueHint::Unknown {
        assert!(
            arg.is_set(ArgSettings::TakesValue),
            "Argument '{}' has value hint but takes no value",
            arg.name
        );

        if arg.value_hint == ValueHint::CommandWithArguments {
            assert!(
                arg.is_set(ArgSettings::MultipleValues),
                "Argument '{}' uses hint CommandWithArguments and must accept multiple values",
                arg.name
            )
        }
    }

    if arg.index.is_some() {
        assert!(
            arg.is_positional(),
            "Argument '{}' is a positional argument and can't have short or long name versions",
            arg.name
        );
    }

    if arg.is_set(ArgSettings::Required) {
        assert!(
            arg.default_vals.is_empty(),
            "Argument '{}' is required and can't have a default value",
            arg.name
        );
    }

    assert_arg_flags(arg);
}

fn assert_arg_flags(arg: &Arg) {
    use ArgSettings::*;

    macro_rules! checker {
        ($a:ident requires $($b:ident)|+) => {
            if arg.is_set($a) {
                let mut s = String::new();

                $(
                    if !arg.is_set($b) {
                        s.push_str(&format!("  ArgSettings::{} is required when ArgSettings::{} is set.\n", std::stringify!($b), std::stringify!($a)));
                    }
                )+

                if !s.is_empty() {
                    panic!("Argument {:?}\n{}", arg.get_name(), s)
                }
            }
        }
    }

    checker!(ForbidEmptyValues requires TakesValue);
    checker!(RequireDelimiter requires TakesValue | UseValueDelimiter);
    checker!(HidePossibleValues requires TakesValue);
    checker!(AllowHyphenValues requires TakesValue);
    checker!(RequireEquals requires TakesValue);
    checker!(Last requires TakesValue);
    checker!(HideDefaultValue requires TakesValue);
    checker!(MultipleValues requires TakesValue);
    checker!(IgnoreCase requires TakesValue);
    checker!(AllowInvalidUtf8 requires TakesValue);
}
