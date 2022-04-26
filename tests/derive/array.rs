use clap::error::ContextKind;
use clap::error::ContextValue;
use clap::Error;
use clap::ErrorKind;
use clap::Parser;

#[derive(Parser, Eq, PartialEq, Debug)]
struct Issue1682<const N: usize> {
    #[clap(long)]
    foo: [u32; N], // implies `required = true` and `number_of_values = N`

    #[clap(long)]
    bar: Option<[u32; N]>, // `number_of_values = N`
}

#[test]
fn issue_1682_correct() {
    let tmp = Issue1682::<1>::try_parse_from(["test", "--foo", "233"]).unwrap();
    assert_eq!(tmp.foo, [233]);
    assert_eq!(tmp.bar, None);

    let tmp = Issue1682::<2>::try_parse_from(["test", "--foo", "233", "42"]).unwrap();
    assert_eq!(tmp.foo, [233, 42]);
    assert_eq!(tmp.bar, None);

    let tmp = Issue1682::<3>::try_parse_from(["test", "--foo", "233", "42", "43"]).unwrap();
    assert_eq!(tmp.foo, [233, 42, 43]);
    assert_eq!(tmp.bar, None);

    let tmp = Issue1682::<3>::try_parse_from([
        "test", "--foo", "233", "42", "43", "--bar", "1", "2", "3",
    ])
    .unwrap();
    assert_eq!(tmp.foo, [233, 42, 43]);
    assert_eq!(tmp.bar, Some([1, 2, 3]));

    let tmp = Issue1682::<3>::try_parse_from([
        "test", "--bar", "1", "2", "3", "--foo", "233", "42", "43",
    ])
    .unwrap();
    assert_eq!(tmp.foo, [233, 42, 43]);
    assert_eq!(tmp.bar, Some([1, 2, 3]));
}

#[test]
fn issue_1682_incorrect_foo() {
    let e = Issue1682::<3>::try_parse_from(["test", "--foo"]).unwrap_err();
    assert_eq!(e.kind(), ErrorKind::EmptyValue);

    let e = Issue1682::<3>::try_parse_from(["test", "--foo", "12"]).unwrap_err();
    assert_eq!(e.kind(), ErrorKind::WrongNumberOfValues);
    assert_eq!(expected_num(&e), Some(3));
    assert_eq!(actual_num(&e), Some(1));

    let e = Issue1682::<3>::try_parse_from(["test", "--foo", "12", "33"]).unwrap_err();
    assert_eq!(e.kind(), ErrorKind::WrongNumberOfValues);
    assert_eq!(expected_num(&e), Some(3));
    assert_eq!(actual_num(&e), Some(2));

    let e = Issue1682::<3>::try_parse_from(["test", "--foo", "12", "33", "44", "55"]).unwrap_err();
    assert_eq!(e.kind(), ErrorKind::UnknownArgument);

    let e = Issue1682::<3>::try_parse_from(["test", "--foo", "k", "b", "d"]).unwrap_err();
    assert_eq!(e.kind(), ErrorKind::ValueValidation);

    let e = Issue1682::<3>::try_parse_from(["test", "--foo", "1", "b", "d"]).unwrap_err();
    assert_eq!(e.kind(), ErrorKind::ValueValidation);

    let e = Issue1682::<3>::try_parse_from(["test", "--foo", "1", "b", "3"]).unwrap_err();
    assert_eq!(e.kind(), ErrorKind::ValueValidation);
}

#[test]
fn issue_1682_incorrect_bar() {
    let e =
        Issue1682::<3>::try_parse_from(["test", "--foo", "12", "33", "44", "--bar"]).unwrap_err();
    assert_eq!(e.kind(), ErrorKind::EmptyValue);

    let e = Issue1682::<3>::try_parse_from(["test", "--foo", "12", "33", "44", "--bar", "12"])
        .unwrap_err();
    assert_eq!(e.kind(), ErrorKind::WrongNumberOfValues);
    assert_eq!(expected_num(&e), Some(3));
    assert_eq!(actual_num(&e), Some(1));

    let e =
        Issue1682::<3>::try_parse_from(["test", "--foo", "12", "33", "44", "--bar", "12", "33"])
            .unwrap_err();
    assert_eq!(e.kind(), ErrorKind::WrongNumberOfValues);
    assert_eq!(expected_num(&e), Some(3));
    assert_eq!(actual_num(&e), Some(2));

    let e = Issue1682::<3>::try_parse_from([
        "test", "--foo", "12", "33", "44", "--bar", "12", "33", "44", "55",
    ])
    .unwrap_err();
    assert_eq!(e.kind(), ErrorKind::UnknownArgument);

    let e =
        Issue1682::<3>::try_parse_from(["test", "--foo", "12", "33", "44", "--bar", "k", "b", "d"])
            .unwrap_err();
    assert_eq!(e.kind(), ErrorKind::ValueValidation);

    let e =
        Issue1682::<3>::try_parse_from(["test", "--foo", "12", "33", "44", "--bar", "1", "b", "d"])
            .unwrap_err();
    assert_eq!(e.kind(), ErrorKind::ValueValidation);

    let e =
        Issue1682::<3>::try_parse_from(["test", "--foo", "12", "33", "44", "--bar", "1", "b", "3"])
            .unwrap_err();
    assert_eq!(e.kind(), ErrorKind::ValueValidation);
}

#[test]
fn issue_1682_incorrect_multi_occ() {
    // multiple occurrences is not allowed
    {
        let e = Issue1682::<3>::try_parse_from(["test", "--foo", "12", "33", "--foo", "44"])
            .unwrap_err();
        assert_eq!(e.kind(), ErrorKind::UnexpectedMultipleUsage);

        let e = Issue1682::<3>::try_parse_from(["test", "--foo", "12", "--foo", "33", "44"])
            .unwrap_err();
        assert_eq!(e.kind(), ErrorKind::UnexpectedMultipleUsage);
    }

    // multiple occurrences is not allowed
    {
        let e = Issue1682::<3>::try_parse_from([
            "test", "--foo", "12", "33", "44", "--bar", "12", "33", "--bar", "44",
        ])
        .unwrap_err();
        assert_eq!(e.kind(), ErrorKind::UnexpectedMultipleUsage);

        let e = Issue1682::<3>::try_parse_from([
            "test", "--foo", "12", "33", "44", "--bar", "12", "--bar", "33", "44",
        ])
        .unwrap_err();
        assert_eq!(e.kind(), ErrorKind::UnexpectedMultipleUsage);
    }
}

#[derive(Parser, Eq, PartialEq, Debug)]
struct Issue1682Lit {
    #[clap(long)]
    foo: [u32; 3], // implies `required = true` and `number_of_values = N`

    #[clap(long)]
    bar: Option<[u32; 3]>, // `number_of_values = N`
}

#[test]
fn issue_1682_literal_correct() {
    let tmp = Issue1682Lit::try_parse_from(["test", "--foo", "233", "42", "43"]).unwrap();
    assert_eq!(tmp.foo, [233, 42, 43]);
    assert_eq!(tmp.bar, None);

    let tmp =
        Issue1682Lit::try_parse_from(["test", "--foo", "233", "42", "43", "--bar", "1", "2", "3"])
            .unwrap();
    assert_eq!(tmp.foo, [233, 42, 43]);
    assert_eq!(tmp.bar, Some([1, 2, 3]));

    let tmp =
        Issue1682Lit::try_parse_from(["test", "--bar", "1", "2", "3", "--foo", "233", "42", "43"])
            .unwrap();
    assert_eq!(tmp.foo, [233, 42, 43]);
    assert_eq!(tmp.bar, Some([1, 2, 3]));
}

#[derive(Parser, Eq, PartialEq, Debug)]
struct Positional {
    foo: [u32; 3], // implies `required = true` and `number_of_values = N`
}

#[test]
fn positional_fixed_array() {
    let tmp = Positional::try_parse_from(["tmp", "233", "42", "43"]).unwrap();
    assert_eq!(tmp.foo, [233, 42, 43]);

    let e = Positional::try_parse_from(["test", "12"]).unwrap_err();
    assert_eq!(e.kind(), ErrorKind::WrongNumberOfValues);
    assert_eq!(expected_num(&e), Some(3));
    assert_eq!(actual_num(&e), Some(1));

    let e = Positional::try_parse_from(["test", "12", "33"]).unwrap_err();
    assert_eq!(e.kind(), ErrorKind::WrongNumberOfValues);
    assert_eq!(expected_num(&e), Some(3));
    assert_eq!(actual_num(&e), Some(2));

    let e = Positional::try_parse_from(["test", "12", "33", "44", "55"]).unwrap_err();
    assert_eq!(e.kind(), ErrorKind::WrongNumberOfValues);
    assert_eq!(expected_num(&e), Some(3));
    assert_eq!(actual_num(&e), Some(4));
}

fn expected_num(e: &Error) -> Option<isize> {
    e.context().find_map(|(kind, value)| match (kind, value) {
        (ContextKind::ExpectedNumValues, ContextValue::Number(num)) => Some(*num),
        _ => None,
    })
}

fn actual_num(e: &Error) -> Option<isize> {
    e.context().find_map(|(kind, value)| match (kind, value) {
        (ContextKind::ActualNumValues, ContextValue::Number(num)) => Some(*num),
        _ => None,
    })
}
