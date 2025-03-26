use clap::Parser;
use snapbox::str;

use crate::utils::assert_output;

#[test]
fn test_safely_nest_parser() {
    #[derive(Parser, Debug, PartialEq)]
    struct Opt {
        #[command(flatten)]
        foo: Foo,
    }

    #[derive(Parser, Debug, PartialEq)]
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
fn implicit_struct_group() {
    #[derive(Parser, Debug)]
    struct Opt {
        #[arg(short, long, requires = "Source")]
        add: bool,

        #[command(flatten)]
        source: Source,
    }

    #[derive(clap::Args, Debug)]
    struct Source {
        crates: Vec<String>,
        #[arg(long)]
        path: Option<std::path::PathBuf>,
        #[arg(long)]
        git: Option<String>,
    }

    let output = str![[r#"
error: the following required arguments were not provided:
  <CRATES|--path <PATH>|--git <GIT>>

Usage: prog --add <CRATES|--path <PATH>|--git <GIT>>

For more information, try '--help'.

"#]];
    assert_output::<Opt>("prog --add", output, true);

    use clap::Args;
    assert_eq!(Source::group_id(), Some(clap::Id::from("Source")));
    assert_eq!(Opt::group_id(), Some(clap::Id::from("Opt")));
}

#[test]
fn skip_group_avoids_duplicate_ids() {
    #[derive(Parser, Debug)]
    #[group(skip)]
    struct Opt {
        #[command(flatten)]
        first: Compose<Empty, Empty>,
        #[command(flatten)]
        second: Compose<Empty, Empty>,
    }

    #[derive(clap::Args, Debug)]
    #[group(skip)]
    pub(crate) struct Compose<L: Args, R: Args> {
        #[command(flatten)]
        pub(crate) left: L,
        #[command(flatten)]
        pub(crate) right: R,
    }

    #[derive(clap::Args, Clone, Copy, Debug)]
    #[group(skip)]
    pub(crate) struct Empty;

    use clap::CommandFactory;
    Opt::command().debug_assert();

    use clap::Args;
    assert_eq!(Empty::group_id(), None);
    assert_eq!(Compose::<Empty, Empty>::group_id(), None);
    assert_eq!(Opt::group_id(), None);
}

#[test]
fn optional_flatten() {
    #[derive(Parser, Debug, PartialEq, Eq)]
    struct Opt {
        #[command(flatten)]
        source: Option<Source>,
    }

    #[derive(clap::Args, Debug, PartialEq, Eq)]
    struct Source {
        crates: Vec<String>,
        #[arg(long)]
        path: Option<std::path::PathBuf>,
        #[arg(long)]
        git: Option<String>,
    }

    assert_eq!(Opt { source: None }, Opt::try_parse_from(["test"]).unwrap());
    assert_eq!(
        Opt {
            source: Some(Source {
                crates: vec!["serde".to_owned()],
                path: None,
                git: None,
            }),
        },
        Opt::try_parse_from(["test", "serde"]).unwrap()
    );
    assert_eq!(
        Opt {
            source: Some(Source {
                crates: Vec::new(),
                path: Some("./".into()),
                git: None,
            }),
        },
        Opt::try_parse_from(["test", "--path=./"]).unwrap()
    );
}

#[test]
#[should_panic = "\
Command clap: Argument group name must be unique

\t'Compose' is already in use"]
fn helpful_panic_on_duplicate_groups() {
    #[derive(Parser, Debug)]
    struct Opt {
        #[command(flatten)]
        first: Compose<Empty, Empty>,
        #[command(flatten)]
        second: Compose<Empty, Empty>,
    }

    #[derive(clap::Args, Debug)]
    pub(crate) struct Compose<L: clap::Args, R: clap::Args> {
        #[command(flatten)]
        pub(crate) left: L,
        #[command(flatten)]
        pub(crate) right: R,
    }

    #[derive(clap::Args, Clone, Copy, Debug)]
    pub(crate) struct Empty;

    use clap::CommandFactory;
    Opt::command().debug_assert();
}

#[test]
fn custom_group_id() {
    #[derive(Parser, Debug, PartialEq, Eq)]
    struct Opt {
        #[command(flatten)]
        source: Option<Source>,
    }

    #[derive(clap::Args, Debug, PartialEq, Eq)]
    #[group(id = "source")]
    struct Source {
        crates: Vec<String>,
        #[arg(long)]
        path: Option<std::path::PathBuf>,
        #[arg(long)]
        git: Option<String>,
    }

    assert_eq!(Opt { source: None }, Opt::try_parse_from(["test"]).unwrap());
    assert_eq!(
        Opt {
            source: Some(Source {
                crates: vec!["serde".to_owned()],
                path: None,
                git: None,
            }),
        },
        Opt::try_parse_from(["test", "serde"]).unwrap()
    );
    assert_eq!(
        Opt {
            source: Some(Source {
                crates: Vec::new(),
                path: Some("./".into()),
                git: None,
            }),
        },
        Opt::try_parse_from(["test", "--path=./"]).unwrap()
    );
}

#[test]
fn required_group() {
    #[derive(Parser, Debug, PartialEq, Eq)]
    struct Opt {
        #[command(flatten)]
        source: Source,
    }

    #[derive(clap::Args, Debug, PartialEq, Eq)]
    #[group(required = true, multiple = false)]
    struct Source {
        #[arg(long)]
        path: Option<std::path::PathBuf>,
        #[arg(long)]
        git: Option<String>,
    }

    assert_eq!(
        Opt {
            source: Source {
                path: Some("./".into()),
                git: None,
            },
        },
        Opt::try_parse_from(["test", "--path=./"]).unwrap()
    );

    let output = str![[r#"
error: the following required arguments were not provided:
  <--path <PATH>|--git <GIT>>

Usage: test <--path <PATH>|--git <GIT>>

For more information, try '--help'.

"#]];
    assert_output::<Opt>("test", output, true);
}

#[test]
#[cfg(feature = "error-context")]
#[cfg(feature = "suggestions")]
fn suggestion() {
    #[derive(Parser, Debug)]
    struct Args {
        name: String,

        #[arg(long)]
        hello: Option<u8>,

        #[arg(long)]
        count01: Vec<u8>,
        #[arg(long)]
        count02: Vec<u8>,
        #[arg(long)]
        count03: Vec<u8>,
        #[arg(long)]
        count04: Vec<u8>,
        #[arg(long)]
        count05: Vec<u8>,
        #[arg(long)]
        count06: Vec<u8>,
        #[arg(long)]
        count07: Vec<u8>,
        #[arg(long)]
        count08: Vec<u8>,
        #[arg(long)]
        count09: Vec<u8>,
        #[arg(long)]
        count10: Vec<u8>,
        #[arg(long)]
        count11: Vec<u8>,
        #[arg(long)]
        count12: Vec<u8>,
        #[arg(long)]
        count13: Vec<u8>,
        #[arg(long)]
        count14: Vec<u8>,
        #[arg(long)]
        count15: Vec<u8>,
        #[arg(long)]
        count16: Vec<u8>,
        #[arg(long)]
        count17: Vec<u8>,
        #[arg(long)]
        count18: Vec<u8>,
    }

    let output = str![[r#"
error: unexpected argument '--hell' found

  tip: a similar argument exists: '--hello'

Usage: test <NAME|--hello <HELLO>|--count01 <COUNT01>|--count02 <COUNT02>|--count03 <COUNT03>|--count04 <COUNT04>|--count05 <COUNT05>|--count06 <COUNT06>|--count07 <COUNT07>|--count08 <COUNT08>|--count09 <COUNT09>|--count10 <COUNT10>|--count11 <COUNT11>|--count12 <COUNT12>|--count13 <COUNT13>|--count14 <COUNT14>|--count15 <COUNT15>|--count16 <COUNT16>|--count17 <COUNT17>|--count18 <COUNT18>>

For more information, try '--help'.

"#]];
    assert_output::<Args>("test --hell", output, true);
}
