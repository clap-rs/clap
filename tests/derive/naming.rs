use clap::Args;
use clap::Parser;

#[test]
fn test_standalone_long_generates_kebab_case() {
    #[derive(Parser, Debug, PartialEq)]
    #[allow(non_snake_case)]
    struct Opt {
        #[arg(long)]
        FOO_OPTION: bool,
    }

    assert_eq!(
        Opt { FOO_OPTION: true },
        Opt::try_parse_from(["test", "--foo-option"]).unwrap()
    );
}

#[test]
fn test_custom_long_overwrites_default_name() {
    #[derive(Parser, Debug, PartialEq)]
    struct Opt {
        #[arg(long = "foo")]
        foo_option: bool,
    }

    assert_eq!(
        Opt { foo_option: true },
        Opt::try_parse_from(["test", "--foo"]).unwrap()
    );
}

#[test]
fn test_standalone_long_uses_previous_defined_custom_name() {
    #[derive(Parser, Debug, PartialEq)]
    struct Opt {
        #[arg(id = "foo", long)]
        foo_option: bool,
    }

    assert_eq!(
        Opt { foo_option: true },
        Opt::try_parse_from(["test", "--foo"]).unwrap()
    );
}

#[test]
fn test_standalone_long_ignores_afterwards_defined_custom_name() {
    #[derive(Parser, Debug, PartialEq)]
    struct Opt {
        #[arg(long, id = "foo")]
        foo_option: bool,
    }

    assert_eq!(
        Opt { foo_option: true },
        Opt::try_parse_from(["test", "--foo-option"]).unwrap()
    );
}

#[test]
fn test_standalone_long_uses_previous_defined_custom_id() {
    #[derive(Parser, Debug, PartialEq)]
    struct Opt {
        #[arg(id = "foo", long)]
        foo_option: bool,
    }

    assert_eq!(
        Opt { foo_option: true },
        Opt::try_parse_from(["test", "--foo"]).unwrap()
    );
}

#[test]
fn test_standalone_long_ignores_afterwards_defined_custom_id() {
    #[derive(Parser, Debug, PartialEq)]
    struct Opt {
        #[arg(long, id = "foo")]
        foo_option: bool,
    }

    assert_eq!(
        Opt { foo_option: true },
        Opt::try_parse_from(["test", "--foo-option"]).unwrap()
    );
}

#[test]
fn test_standalone_short_generates_kebab_case() {
    #[derive(Parser, Debug, PartialEq)]
    #[allow(non_snake_case)]
    struct Opt {
        #[arg(short)]
        FOO_OPTION: bool,
    }

    assert_eq!(
        Opt { FOO_OPTION: true },
        Opt::try_parse_from(["test", "-f"]).unwrap()
    );
}

#[test]
fn test_custom_short_overwrites_default_name() {
    #[derive(Parser, Debug, PartialEq)]
    struct Opt {
        #[arg(short = 'o')]
        foo_option: bool,
    }

    assert_eq!(
        Opt { foo_option: true },
        Opt::try_parse_from(["test", "-o"]).unwrap()
    );
}

#[test]
fn test_standalone_short_uses_previous_defined_custom_name() {
    #[derive(Parser, Debug, PartialEq)]
    struct Opt {
        #[arg(id = "option", short)]
        foo_option: bool,
    }

    assert_eq!(
        Opt { foo_option: true },
        Opt::try_parse_from(["test", "-o"]).unwrap()
    );
}

#[test]
fn test_standalone_short_ignores_afterwards_defined_custom_name() {
    #[derive(Parser, Debug, PartialEq)]
    struct Opt {
        #[arg(short, id = "option")]
        foo_option: bool,
    }

    assert_eq!(
        Opt { foo_option: true },
        Opt::try_parse_from(["test", "-f"]).unwrap()
    );
}

#[test]
fn test_standalone_short_uses_previous_defined_custom_id() {
    #[derive(Parser, Debug, PartialEq)]
    struct Opt {
        #[arg(id = "option", short)]
        foo_option: bool,
    }

    assert_eq!(
        Opt { foo_option: true },
        Opt::try_parse_from(["test", "-o"]).unwrap()
    );
}

#[test]
fn test_standalone_short_ignores_afterwards_defined_custom_id() {
    #[derive(Parser, Debug, PartialEq)]
    struct Opt {
        #[arg(short, id = "option")]
        foo_option: bool,
    }

    assert_eq!(
        Opt { foo_option: true },
        Opt::try_parse_from(["test", "-f"]).unwrap()
    );
}

#[test]
fn test_standalone_long_uses_previous_defined_casing() {
    #[derive(Parser, Debug, PartialEq)]
    struct Opt {
        #[arg(rename_all = "screaming_snake", long)]
        foo_option: bool,
    }

    assert_eq!(
        Opt { foo_option: true },
        Opt::try_parse_from(["test", "--FOO_OPTION"]).unwrap()
    );
}

#[test]
fn test_standalone_short_uses_previous_defined_casing() {
    #[derive(Parser, Debug, PartialEq)]
    struct Opt {
        #[arg(rename_all = "screaming_snake", short)]
        foo_option: bool,
    }

    assert_eq!(
        Opt { foo_option: true },
        Opt::try_parse_from(["test", "-F"]).unwrap()
    );
}

#[test]
fn test_standalone_long_works_with_verbatim_casing() {
    #[derive(Parser, Debug, PartialEq)]
    #[allow(non_snake_case)]
    struct Opt {
        #[arg(rename_all = "verbatim", long)]
        _fOO_oPtiON: bool,
    }

    assert_eq!(
        Opt { _fOO_oPtiON: true },
        Opt::try_parse_from(["test", "--_fOO_oPtiON"]).unwrap()
    );
}

#[test]
fn test_standalone_short_works_with_verbatim_casing() {
    #[derive(Parser, Debug, PartialEq)]
    struct Opt {
        #[arg(rename_all = "verbatim", short)]
        _foo: bool,
    }

    assert_eq!(
        Opt { _foo: true },
        Opt::try_parse_from(["test", "-_"]).unwrap()
    );
}

#[test]
fn test_rename_all_is_propagated_from_struct_to_fields() {
    #[derive(Parser, Debug, PartialEq)]
    #[command(rename_all = "screaming_snake")]
    struct Opt {
        #[arg(long)]
        foo: bool,
    }

    assert_eq!(
        Opt { foo: true },
        Opt::try_parse_from(["test", "--FOO"]).unwrap()
    );
}

#[test]
fn test_rename_all_is_not_propagated_from_struct_into_flattened() {
    #[derive(Parser, Debug, PartialEq)]
    #[command(rename_all = "screaming_snake")]
    struct Opt {
        #[command(flatten)]
        foo: Foo,
    }

    #[derive(Args, Debug, PartialEq)]
    struct Foo {
        #[arg(long)]
        foo: bool,
    }

    assert_eq!(
        Opt {
            foo: Foo { foo: true }
        },
        Opt::try_parse_from(["test", "--foo"]).unwrap()
    );
}

#[test]
fn test_lower_is_renamed() {
    #[derive(Parser, Debug, PartialEq)]
    struct Opt {
        #[arg(rename_all = "lower", long)]
        foo_option: bool,
    }

    assert_eq!(
        Opt { foo_option: true },
        Opt::try_parse_from(["test", "--foooption"]).unwrap()
    );
}

#[test]
fn test_upper_is_renamed() {
    #[derive(Parser, Debug, PartialEq)]
    struct Opt {
        #[arg(rename_all = "upper", long)]
        foo_option: bool,
    }

    assert_eq!(
        Opt { foo_option: true },
        Opt::try_parse_from(["test", "--FOOOPTION"]).unwrap()
    );
}

#[test]
fn test_single_word_enum_variant_is_default_renamed_into_kebab_case() {
    #[derive(Parser, Debug, PartialEq)]
    enum Opt {
        Command { foo: u32 },
    }

    assert_eq!(
        Opt::Command { foo: 0 },
        Opt::try_parse_from(["test", "command", "0"]).unwrap()
    );
}

#[test]
fn test_multi_word_enum_variant_is_renamed() {
    #[derive(Parser, Debug, PartialEq)]
    enum Opt {
        FirstCommand { foo: u32 },
    }

    assert_eq!(
        Opt::FirstCommand { foo: 0 },
        Opt::try_parse_from(["test", "first-command", "0"]).unwrap()
    );
}

#[test]
fn test_rename_all_is_not_propagated_from_struct_into_subcommand() {
    #[derive(Parser, Debug, PartialEq)]
    #[command(rename_all = "screaming_snake")]
    struct Opt {
        #[command(subcommand)]
        foo: Foo,
    }

    #[derive(Parser, Debug, PartialEq)]
    enum Foo {
        Command {
            #[arg(long)]
            foo: bool,
        },
    }

    assert_eq!(
        Opt {
            foo: Foo::Command { foo: true }
        },
        Opt::try_parse_from(["test", "command", "--foo"]).unwrap()
    );
}

#[test]
fn test_rename_all_is_propagated_from_enum_to_variants() {
    #[derive(Parser, Debug, PartialEq)]
    #[command(rename_all = "screaming_snake")]
    enum Opt {
        FirstVariant,
        SecondVariant {
            #[arg(long)]
            foo: String,
        },
    }

    assert_eq!(
        Opt::FirstVariant,
        Opt::try_parse_from(["test", "FIRST_VARIANT"]).unwrap()
    );
}

#[test]
fn test_rename_all_is_propagated_from_enum_to_variant_fields() {
    #[derive(Parser, Debug, PartialEq)]
    #[command(rename_all = "screaming_snake")]
    enum Opt {
        FirstVariant,
        SecondVariant {
            #[arg(long)]
            foo: String,
        },
    }

    assert_eq!(
        Opt::SecondVariant {
            foo: "value".into()
        },
        Opt::try_parse_from(["test", "SECOND_VARIANT", "--FOO", "value"]).unwrap()
    );
}

#[test]
fn test_rename_all_is_propagation_can_be_overridden() {
    #[derive(Parser, Debug, PartialEq)]
    #[command(rename_all = "screaming_snake")]
    enum Opt {
        #[command(rename_all = "kebab_case")]
        FirstVariant {
            #[arg(long)]
            foo_option: bool,
        },
        SecondVariant {
            #[arg(rename_all = "kebab_case", long)]
            foo_option: bool,
        },
    }

    assert_eq!(
        Opt::FirstVariant { foo_option: true },
        Opt::try_parse_from(["test", "first-variant", "--foo-option"]).unwrap()
    );

    assert_eq!(
        Opt::SecondVariant { foo_option: true },
        Opt::try_parse_from(["test", "SECOND_VARIANT", "--foo-option"]).unwrap()
    );
}
