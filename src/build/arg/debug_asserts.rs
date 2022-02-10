use crate::{Arg, ValueHint};

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
            arg.is_takes_value_set(),
            "Argument '{}' has value hint but takes no value",
            arg.name
        );

        if arg.value_hint == ValueHint::CommandWithArguments {
            assert!(
                arg.is_multiple_values_set(),
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

    if arg.is_required_set() {
        assert!(
            arg.default_vals.is_empty(),
            "Argument '{}' is required and can't have a default value",
            arg.name
        );
    }

    assert_arg_flags(arg);

    assert_defaults(arg, "default_value", arg.default_vals.iter().copied());
    assert_defaults(
        arg,
        "default_missing_value",
        arg.default_missing_vals.iter().copied(),
    );
    assert_defaults(
        arg,
        "default_value_if",
        arg.default_vals_ifs
            .iter()
            .filter_map(|(_, _, default)| *default),
    );
}

fn assert_arg_flags(arg: &Arg) {
    macro_rules! checker {
        ($a:ident requires $($b:ident)|+) => {
            if arg.$a() {
                let mut s = String::new();

                $(
                    if !arg.$b() {
                        s.push_str(&format!("  Arg::{} is required when Arg::{} is set.\n", std::stringify!($b), std::stringify!($a)));
                    }
                )+

                if !s.is_empty() {
                    panic!("Argument {:?}\n{}", arg.get_name(), s)
                }
            }
        }
    }

    checker!(is_forbid_empty_values_set requires is_takes_value_set);
    checker!(is_require_value_delimiter_set requires is_takes_value_set);
    checker!(is_require_value_delimiter_set requires is_use_value_delimiter_set);
    checker!(is_hide_possible_values_set requires is_takes_value_set);
    checker!(is_allow_hyphen_values_set requires is_takes_value_set);
    checker!(is_require_equals_set requires is_takes_value_set);
    checker!(is_last_set requires is_takes_value_set);
    checker!(is_hide_default_value_set requires is_takes_value_set);
    checker!(is_multiple_values_set requires is_takes_value_set);
    checker!(is_ignore_case_set requires is_takes_value_set);
    checker!(is_allow_invalid_utf8_set requires is_takes_value_set);
}

fn assert_defaults<'d>(
    arg: &Arg,
    field: &'static str,
    defaults: impl IntoIterator<Item = &'d std::ffi::OsStr>,
) {
    for default_os in defaults {
        if let Some(default_s) = default_os.to_str() {
            if !arg.possible_vals.is_empty() {
                assert!(
                    arg.possible_vals.iter().any(|possible_val| {
                        possible_val.matches(default_s, arg.is_ignore_case_set())
                    }),
                    "Argument `{}`'s {}={} doesn't match possible values",
                    arg.name,
                    field,
                    default_s
                );
            }

            if let Some(validator) = arg.validator.as_ref() {
                let mut validator = validator.lock().unwrap();
                if let Err(err) = validator(default_s) {
                    panic!(
                        "Argument `{}`'s {}={} failed validation: {}",
                        arg.name, field, default_s, err
                    );
                }
            }
        }

        if let Some(validator) = arg.validator_os.as_ref() {
            let mut validator = validator.lock().unwrap();
            if let Err(err) = validator(default_os) {
                panic!(
                    "Argument `{}`'s {}={:?} failed validation: {}",
                    arg.name, field, default_os, err
                );
            }
        }
    }
}
