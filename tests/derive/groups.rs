use clap::Parser;

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

    const OUTPUT: &str = "\
error: the following required arguments were not provided:
  <CRATES|--path <PATH>|--git <GIT>>

Usage: prog --add <CRATES|--path <PATH>|--git <GIT>>

For more information, try '--help'.
";
    assert_output::<Opt>("prog --add", OUTPUT, true);

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

    use clap::Args;
    assert_eq!(Empty::group_id(), None);
    assert_eq!(Compose::<Empty, Empty>::group_id(), None);
    assert_eq!(Opt::group_id(), None);
}

#[test]
#[should_panic = "\
Command clap: Argument group name must be unique

\t'Compose' is already in use (note: `Args` implicitly creates `ArgGroup`s; disable with `#[group(skip)]`"]
fn helpful_panic_on_duplicate_groups() {
    #[derive(Parser, Debug)]
    struct Opt {
        #[command(flatten)]
        first: Compose<Empty, Empty>,
        #[command(flatten)]
        second: Compose<Empty, Empty>,
    }

    #[derive(clap::Args, Debug)]
    pub struct Compose<L: clap::Args, R: clap::Args> {
        #[clap(flatten)]
        pub left: L,
        #[clap(flatten)]
        pub right: R,
    }

    #[derive(clap::Args, Clone, Copy, Debug)]
    pub struct Empty;

    use clap::CommandFactory;
    Opt::command().debug_assert();
}

#[test]
fn required_group() {
    #[derive(Parser, Debug)]
    struct Opt {
        #[command(flatten)]
        source: Source,

        #[arg(long)]
        alt_source: String,
    }

    #[derive(clap::Args, Debug)]
    #[group(required = true, multiple = false, conflicts_with = "alt_source")]
    struct Source {
        #[arg(long)]
        path: Option<std::path::PathBuf>,
        #[arg(long)]
        git: Option<String>,
    }

    const OUTPUT: &str = "\
error: the following required arguments were not provided:
  --alt-source <ALT_SOURCE>
  <--path <PATH>|--git <GIT>>

Usage: prog --alt-source <ALT_SOURCE> <--path <PATH>|--git <GIT>>

For more information, try '--help'.
";
    assert_output::<Opt>("prog", OUTPUT, true);

    use clap::Args;
    assert_eq!(Opt::group_id(), Some(clap::Id::from("Opt")));
    assert_eq!(Source::group_id(), Some(clap::Id::from("Source")));
    use clap::CommandFactory;
    let source_id = clap::Id::from("Source");
    let opt_command = Opt::command();
    let source_group = opt_command
        .get_groups()
        .find(|g| g.get_id() == &source_id)
        .unwrap();
    assert!(source_group.is_required_set());
    // assert!(source_group.is_multiple()); currently broken. Fixed by PR #4704
}
