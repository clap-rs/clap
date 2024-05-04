use clap::{Args, Parser};

#[test]
fn generic_struct_flatten() {
    #[derive(Args, PartialEq, Debug)]
    struct Inner {
        pub(crate) answer: isize,
    }

    #[derive(Parser, PartialEq, Debug)]
    struct Outer<T: Args> {
        #[command(flatten)]
        pub(crate) inner: T,
    }

    assert_eq!(
        Outer {
            inner: Inner { answer: 42 }
        },
        Outer::parse_from(["--answer", "42"])
    );
}

#[test]
fn generic_struct_flatten_w_where_clause() {
    #[derive(Args, PartialEq, Debug)]
    struct Inner {
        pub(crate) answer: isize,
    }

    #[derive(Parser, PartialEq, Debug)]
    struct Outer<T>
    where
        T: Args,
    {
        #[command(flatten)]
        pub(crate) inner: T,
    }

    assert_eq!(
        Outer {
            inner: Inner { answer: 42 }
        },
        Outer::parse_from(["--answer", "42"])
    );
}

#[test]
fn generic_enum() {
    #[derive(Args, PartialEq, Debug)]
    struct Inner {
        pub(crate) answer: isize,
    }

    #[derive(Parser, PartialEq, Debug)]
    enum GenericEnum<T: Args> {
        Start(T),
        Stop,
    }

    assert_eq!(
        GenericEnum::Start(Inner { answer: 42 }),
        GenericEnum::parse_from(["test", "start", "42"])
    );
}

#[test]
fn generic_enum_w_where_clause() {
    #[derive(Args, PartialEq, Debug)]
    struct Inner {
        pub(crate) answer: isize,
    }

    #[derive(Parser, PartialEq, Debug)]
    enum GenericEnum<T>
    where
        T: Args,
    {
        Start(T),
        Stop,
    }

    assert_eq!(
        GenericEnum::Start(Inner { answer: 42 }),
        GenericEnum::parse_from(["test", "start", "42"])
    );
}

#[test]
fn generic_w_fromstr_trait_bound() {
    use std::str::FromStr;

    #[derive(Parser, PartialEq, Debug)]
    struct Opt<T>
    where
        T: FromStr + Send + Sync + Clone + 'static,
        <T as FromStr>::Err: std::error::Error + Sync + Send + 'static,
    {
        answer: T,
    }

    assert_eq!(
        Opt::<isize> { answer: 42 },
        Opt::<isize>::parse_from(["--answer", "42"])
    );
}

#[test]
fn generic_wo_trait_bound() {
    use std::time::Duration;

    #[derive(Parser, PartialEq, Debug)]
    struct Opt<T> {
        answer: isize,
        #[arg(skip)]
        took: Option<T>,
    }

    assert_eq!(
        Opt::<Duration> {
            answer: 42,
            took: None
        },
        Opt::<Duration>::parse_from(["--answer", "42"])
    );
}

#[test]
fn generic_where_clause_w_trailing_comma() {
    use std::str::FromStr;

    #[derive(Parser, PartialEq, Debug)]
    struct Opt<T>
    where
        T: FromStr + Send + Sync + Clone + 'static,
        <T as FromStr>::Err: std::error::Error + Sync + Send + 'static,
    {
        pub(crate) answer: T,
    }

    assert_eq!(
        Opt::<isize> { answer: 42 },
        Opt::<isize>::parse_from(["--answer", "42"])
    );
}
