use clap::Clap;

#[test]
fn test_single_word_enum_variant_is_default_renamed_into_kebab_case() {
    #[derive(Clap, Debug, PartialEq)]
    enum Opt {
        Command { foo: u32 },
    }

    assert_eq!(
        Opt::Command { foo: 0 },
        Opt::parse_from(&["test", "command", "0"])
    );
}

#[test]
fn test_multi_word_enum_variant_is_renamed() {
    #[derive(Clap, Debug, PartialEq)]
    enum Opt {
        FirstCommand { foo: u32 },
    }

    assert_eq!(
        Opt::FirstCommand { foo: 0 },
        Opt::parse_from(&["test", "first-command", "0"])
    );
}

#[test]
fn test_standalone_long_generates_kebab_case() {
    #[derive(Clap, Debug, PartialEq)]
    #[allow(non_snake_case)]
    struct Opt {
        #[clap(long)]
        FOO_OPTION: bool,
    }

    assert_eq!(
        Opt { FOO_OPTION: true },
        Opt::parse_from(&["test", "--foo-option"])
    );
}

#[test]
fn test_custom_long_overwrites_default_name() {
    #[derive(Clap, Debug, PartialEq)]
    struct Opt {
        #[clap(long = "foo")]
        foo_option: bool,
    }

    assert_eq!(
        Opt { foo_option: true },
        Opt::parse_from(&["test", "--foo"])
    );
}

#[test]
fn test_standalone_long_uses_previous_defined_custom_name() {
    #[derive(Clap, Debug, PartialEq)]
    struct Opt {
        #[clap(name = "foo", long)]
        foo_option: bool,
    }

    assert_eq!(
        Opt { foo_option: true },
        Opt::parse_from(&["test", "--foo"])
    );
}

#[test]
fn test_standalone_long_ignores_afterwards_defined_custom_name() {
    #[derive(Clap, Debug, PartialEq)]
    struct Opt {
        #[clap(long, name = "foo")]
        foo_option: bool,
    }

    assert_eq!(
        Opt { foo_option: true },
        Opt::parse_from(&["test", "--foo-option"])
    );
}

#[test]
fn test_standalone_short_generates_kebab_case() {
    #[derive(Clap, Debug, PartialEq)]
    #[allow(non_snake_case)]
    struct Opt {
        #[clap(short)]
        FOO_OPTION: bool,
    }

    assert_eq!(Opt { FOO_OPTION: true }, Opt::parse_from(&["test", "-f"]));
}

#[test]
fn test_custom_short_overwrites_default_name() {
    #[derive(Clap, Debug, PartialEq)]
    struct Opt {
        #[clap(short = "o")]
        foo_option: bool,
    }

    assert_eq!(Opt { foo_option: true }, Opt::parse_from(&["test", "-o"]));
}

#[test]
fn test_standalone_short_uses_previous_defined_custom_name() {
    #[derive(Clap, Debug, PartialEq)]
    struct Opt {
        #[clap(name = "option", short)]
        foo_option: bool,
    }

    assert_eq!(Opt { foo_option: true }, Opt::parse_from(&["test", "-o"]));
}

#[test]
fn test_standalone_short_ignores_afterwards_defined_custom_name() {
    #[derive(Clap, Debug, PartialEq)]
    struct Opt {
        #[clap(short, name = "option")]
        foo_option: bool,
    }

    assert_eq!(Opt { foo_option: true }, Opt::parse_from(&["test", "-f"]));
}

#[test]
fn test_standalone_long_uses_previous_defined_casing() {
    #[derive(Clap, Debug, PartialEq)]
    struct Opt {
        #[clap(rename_all = "screaming_snake", long)]
        foo_option: bool,
    }

    assert_eq!(
        Opt { foo_option: true },
        Opt::parse_from(&["test", "--FOO_OPTION"])
    );
}

#[test]
fn test_standalone_short_uses_previous_defined_casing() {
    #[derive(Clap, Debug, PartialEq)]
    struct Opt {
        #[clap(rename_all = "screaming_snake", short)]
        foo_option: bool,
    }

    assert_eq!(Opt { foo_option: true }, Opt::parse_from(&["test", "-F"]));
}

#[test]
fn test_standalone_long_works_with_verbatim_casing() {
    #[derive(Clap, Debug, PartialEq)]
    #[allow(non_snake_case)]
    struct Opt {
        #[clap(rename_all = "verbatim", long)]
        _fOO_oPtiON: bool,
    }

    assert_eq!(
        Opt { _fOO_oPtiON: true },
        Opt::parse_from(&["test", "--_fOO_oPtiON"])
    );
}

#[test]
fn test_standalone_short_works_with_verbatim_casing() {
    #[derive(Clap, Debug, PartialEq)]
    struct Opt {
        #[clap(rename_all = "verbatim", short)]
        _foo: bool,
    }

    assert_eq!(Opt { _foo: true }, Opt::parse_from(&["test", "-_"]));
}

#[test]
fn test_rename_all_is_propagated_from_struct_to_fields() {
    #[derive(Clap, Debug, PartialEq)]
    #[clap(rename_all = "screaming_snake")]
    struct Opt {
        #[clap(long)]
        foo: bool,
    }

    assert_eq!(Opt { foo: true }, Opt::parse_from(&["test", "--FOO"]));
}

#[test]
fn test_rename_all_is_not_propagated_from_struct_into_flattened() {
    #[derive(Clap, Debug, PartialEq)]
    #[clap(rename_all = "screaming_snake")]
    struct Opt {
        #[clap(flatten)]
        foo: Foo,
    }

    #[derive(Clap, Debug, PartialEq)]
    struct Foo {
        #[clap(long)]
        foo: bool,
    }

    assert_eq!(
        Opt {
            foo: Foo { foo: true }
        },
        Opt::parse_from(&["test", "--foo"])
    );
}

#[test]
fn test_rename_all_is_not_propagated_from_struct_into_subcommand() {
    #[derive(Clap, Debug, PartialEq)]
    #[clap(rename_all = "screaming_snake")]
    struct Opt {
        #[clap(subcommand)]
        foo: Foo,
    }

    #[derive(Clap, Debug, PartialEq)]
    enum Foo {
        Command {
            #[clap(long)]
            foo: bool,
        },
    }

    assert_eq!(
        Opt {
            foo: Foo::Command { foo: true }
        },
        Opt::parse_from(&["test", "command", "--foo"])
    );
}

#[test]
fn test_rename_all_is_propagated_from_enum_to_variants_and_their_fields() {
    #[derive(Clap, Debug, PartialEq)]
    #[clap(rename_all = "screaming_snake")]
    enum Opt {
        FirstVariant,
        SecondVariant {
            #[clap(long)]
            foo: bool,
        },
    }

    assert_eq!(
        Opt::FirstVariant,
        Opt::parse_from(&["test", "FIRST_VARIANT"])
    );

    assert_eq!(
        Opt::SecondVariant { foo: true },
        Opt::parse_from(&["test", "SECOND_VARIANT", "--FOO"])
    );
}

#[test]
fn test_rename_all_is_propagation_can_be_overridden() {
    #[derive(Clap, Debug, PartialEq)]
    #[clap(rename_all = "screaming_snake")]
    enum Opt {
        #[clap(rename_all = "kebab_case")]
        FirstVariant {
            #[clap(long)]
            foo_option: bool,
        },
        SecondVariant {
            #[clap(rename_all = "kebab_case", long)]
            foo_option: bool,
        },
    }

    assert_eq!(
        Opt::FirstVariant { foo_option: true },
        Opt::parse_from(&["test", "first-variant", "--foo-option"])
    );

    assert_eq!(
        Opt::SecondVariant { foo_option: true },
        Opt::parse_from(&["test", "SECOND_VARIANT", "--foo-option"])
    );
}
