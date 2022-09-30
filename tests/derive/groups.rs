use clap::Parser;

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
        Opt::try_parse_from(&["test", "--foo"]).unwrap()
    );
}

#[test]
fn skip_group_avoids_duplicate_ids() {
    #[derive(Parser, Debug)]
    struct Opt {
        #[command(flatten)]
        first: Compose<Empty, Empty>,
        #[command(flatten)]
        second: Compose<Empty, Empty>,
    }

    #[derive(clap::Args, Debug)]
    #[group(skip)]
    pub struct Compose<L: clap::Args, R: clap::Args> {
        #[clap(flatten)]
        pub left: L,
        #[clap(flatten)]
        pub right: R,
    }

    #[derive(clap::Args, Clone, Copy, Debug)]
    #[group(skip)]
    pub struct Empty;

    use clap::CommandFactory;
    Opt::command().debug_assert();
}
