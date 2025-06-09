use clap::arg;
use snapbox::assert_data_eq;
use snapbox::prelude::*;
use snapbox::str;

#[test]
fn arg_long() {
    let arg = arg!(--long);
    assert_data_eq!(arg.get_id().to_debug(), str![[r#"
"long"

"#]]);
    assert_data_eq!(arg.get_short().to_debug(), str![[r#"
None

"#]]);
    assert_data_eq!(arg.get_long().to_debug(), str![[r#"
Some(
    "long",
)

"#]]);
    assert_data_eq!(arg.get_action().to_debug(), str![[r#"
SetTrue

"#]]);
    assert_data_eq!(arg.is_required_set().to_debug(), str![[r#"
false

"#]]);
    assert_data_eq!(arg.get_value_names().to_debug(), str![[r#"
None

"#]]);
    assert_data_eq!(arg.get_help().to_debug(), str![[r#"
None

"#]]);
}

#[test]
#[should_panic = "Without a value or long flag, the `name:` prefix is required"]
fn arg_short() {
    arg!(-s);
}

#[test]
fn arg_long_dashed() {
    let arg = arg!(--"long-flag");
    assert_data_eq!(arg.get_id().to_debug(), str![[r#"
"long-flag"

"#]]);
    assert_data_eq!(arg.get_short().to_debug(), str![[r#"
None

"#]]);
    assert_data_eq!(arg.get_long().to_debug(), str![[r#"
Some(
    "long-flag",
)

"#]]);
    assert_data_eq!(arg.get_action().to_debug(), str![[r#"
SetTrue

"#]]);
    assert_data_eq!(arg.is_required_set().to_debug(), str![[r#"
false

"#]]);
    assert_data_eq!(arg.get_value_names().to_debug(), str![[r#"
None

"#]]);
    assert_data_eq!(arg.get_help().to_debug(), str![[r#"
None

"#]]);
}

#[test]
fn arg_long_optional_value() {
    let arg = arg!(--long[VALUE]);
    assert_data_eq!(arg.get_id().to_debug(), str![[r#"
"long"

"#]]);
    assert_data_eq!(arg.get_short().to_debug(), str![[r#"
None

"#]]);
    assert_data_eq!(arg.get_long().to_debug(), str![[r#"
Some(
    "long",
)

"#]]);
    assert_data_eq!(arg.get_action().to_debug(), str![[r#"
Set

"#]]);
    assert_data_eq!(arg.is_required_set().to_debug(), str![[r#"
false

"#]]);
    assert_data_eq!(arg.get_value_names().to_debug(), str![[r#"
Some(
    [
        "VALUE",
    ],
)

"#]]);
    assert_data_eq!(arg.get_help().to_debug(), str![[r#"
None

"#]]);
}

#[test]
fn arg_long_required_value() {
    let arg = arg!(--long <VALUE>);
    assert_data_eq!(arg.get_id().to_debug(), str![[r#"
"long"

"#]]);
    assert_data_eq!(arg.get_short().to_debug(), str![[r#"
None

"#]]);
    assert_data_eq!(arg.get_long().to_debug(), str![[r#"
Some(
    "long",
)

"#]]);
    assert_data_eq!(arg.get_action().to_debug(), str![[r#"
Set

"#]]);
    assert_data_eq!(arg.is_required_set().to_debug(), str![[r#"
false

"#]]);
    assert_data_eq!(arg.get_value_names().to_debug(), str![[r#"
Some(
    [
        "VALUE",
    ],
)

"#]]);
    assert_data_eq!(arg.get_help().to_debug(), str![[r#"
None

"#]]);
}

#[test]
#[should_panic = "Multiple values not yet supported"]
fn arg_long_multiple_values() {
    arg!(--long <VALUE1> <VALUE2> <VALUE3> [VALUE4] [VALUE5]);
}

#[test]
fn arg_optional_value() {
    let arg = arg!([VALUE]);
    assert_data_eq!(arg.get_id().to_debug(), str![[r#"
"VALUE"

"#]]);
    assert_data_eq!(arg.get_short().to_debug(), str![[r#"
None

"#]]);
    assert_data_eq!(arg.get_long().to_debug(), str![[r#"
None

"#]]);
    assert_data_eq!(arg.get_action().to_debug(), str![[r#"
Set

"#]]);
    assert_data_eq!(arg.is_required_set().to_debug(), str![[r#"
false

"#]]);
    assert_data_eq!(arg.get_value_names().to_debug(), str![[r#"
Some(
    [
        "VALUE",
    ],
)

"#]]);
    assert_data_eq!(arg.get_help().to_debug(), str![[r#"
None

"#]]);
}

#[test]
fn arg_required_value() {
    let arg = arg!(<VALUE>);
    assert_data_eq!(arg.get_id().to_debug(), str![[r#"
"VALUE"

"#]]);
    assert_data_eq!(arg.get_short().to_debug(), str![[r#"
None

"#]]);
    assert_data_eq!(arg.get_long().to_debug(), str![[r#"
None

"#]]);
    assert_data_eq!(arg.get_action().to_debug(), str![[r#"
Set

"#]]);
    assert_data_eq!(arg.is_required_set().to_debug(), str![[r#"
true

"#]]);
    assert_data_eq!(arg.get_value_names().to_debug(), str![[r#"
Some(
    [
        "VALUE",
    ],
)

"#]]);
    assert_data_eq!(arg.get_help().to_debug(), str![[r#"
None

"#]]);
}

#[test]
#[should_panic = "Multiple values not yet supported"]
fn arg_multiple_values() {
    arg!(<VALUE1> <VALUE2> <VALUE3> [VALUE4] [VALUE5]);
}

#[test]
fn arg_named_positional() {
    let arg = arg!(name: <VALUE>);
    assert_data_eq!(arg.get_id().to_debug(), str![[r#"
"name"

"#]]);
    assert_data_eq!(arg.get_short().to_debug(), str![[r#"
None

"#]]);
    assert_data_eq!(arg.get_long().to_debug(), str![[r#"
None

"#]]);
    assert_data_eq!(arg.get_action().to_debug(), str![[r#"
Set

"#]]);
    assert_data_eq!(arg.is_required_set().to_debug(), str![[r#"
true

"#]]);
    assert_data_eq!(arg.get_value_names().to_debug(), str![[r#"
Some(
    [
        "VALUE",
    ],
)

"#]]);
    assert_data_eq!(arg.get_help().to_debug(), str![[r#"
None

"#]]);
}

#[test]
fn arg_named_long() {
    let arg = arg!(name: --long <VALUE>);
    assert_data_eq!(arg.get_id().to_debug(), str![[r#"
"name"

"#]]);
    assert_data_eq!(arg.get_short().to_debug(), str![[r#"
None

"#]]);
    assert_data_eq!(arg.get_long().to_debug(), str![[r#"
Some(
    "long",
)

"#]]);
    assert_data_eq!(arg.get_action().to_debug(), str![[r#"
Set

"#]]);
    assert_data_eq!(arg.is_required_set().to_debug(), str![[r#"
false

"#]]);
    assert_data_eq!(arg.get_value_names().to_debug(), str![[r#"
Some(
    [
        "VALUE",
    ],
)

"#]]);
    assert_data_eq!(arg.get_help().to_debug(), str![[r#"
None

"#]]);
}

#[test]
fn arg_named_dashed() {
    let arg = arg!("named-arg": --long <VALUE>);
    assert_data_eq!(arg.get_id().to_debug(), str![[r#"
"named-arg"

"#]]);
    assert_data_eq!(arg.get_short().to_debug(), str![[r#"
None

"#]]);
    assert_data_eq!(arg.get_long().to_debug(), str![[r#"
Some(
    "long",
)

"#]]);
    assert_data_eq!(arg.get_action().to_debug(), str![[r#"
Set

"#]]);
    assert_data_eq!(arg.is_required_set().to_debug(), str![[r#"
false

"#]]);
    assert_data_eq!(arg.get_value_names().to_debug(), str![[r#"
Some(
    [
        "VALUE",
    ],
)

"#]]);
    assert_data_eq!(arg.get_help().to_debug(), str![[r#"
None

"#]]);
}
