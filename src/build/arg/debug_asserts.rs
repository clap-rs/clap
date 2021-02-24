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
            arg.short.is_none() && arg.long.is_none(),
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

    assert_app_flag(arg);
}

fn assert_app_flag(arg: &Arg) {
    use ArgSettings::*;
    if arg.is_set(AllowEmptyValues) {
        assert!(arg.is_set(TakesValue));
    }
    if arg.is_set(RequireDelimiter) {
        assert!(arg.is_set(TakesValue));
        assert!(arg.is_set(UseValueDelimiter));
    }
    if arg.is_set(HidePossibleValues) {
        assert!(arg.is_set(TakesValue));
    }
    if arg.is_set(AllowHyphenValues) {
        assert!(arg.is_set(TakesValue));
    }
    if arg.is_set(RequireEquals) {
        assert!(arg.is_set(TakesValue));
    }
    if arg.is_set(Last) {
        assert!(arg.is_set(TakesValue));
    }
    if arg.is_set(HideDefaultValue) {
        assert!(arg.is_set(TakesValue));
    }
    if arg.is_set(MultipleValues) {
        assert!(arg.is_set(TakesValue));
    }
    if arg.is_set(HideEnv) {
        assert!(arg.is_set(TakesValue));
    }
    if arg.is_set(HideEnvValues) {
        assert!(arg.is_set(TakesValue));
    }
    if arg.is_set(IgnoreCase) {
        assert!(arg.is_set(TakesValue));
    }
}
