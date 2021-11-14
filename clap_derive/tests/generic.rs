mod utils;

use clap::{Args, Parser};

#[test]
fn generic() {
    #[derive(Args, PartialEq, Debug)]
    struct A {
        arg: i32,
    }

    #[derive(Args, PartialEq, Debug)]
    struct B {
        arg: String,
    }

    #[derive(Parser, PartialEq, Debug)]
    struct Opt<T: Args> {
        #[clap(flatten)]
        inner: T,
    }
    assert_eq!(
        Opt {
            inner: A { arg: 42 }
        },
        Opt::try_parse_from(&["test", "42"]).unwrap()
    );
    assert_eq!(
        Opt {
            inner: B {
                arg: "42".to_owned()
            }
        },
        Opt::try_parse_from(&["test", "42"]).unwrap()
    );
}
