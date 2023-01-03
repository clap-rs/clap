use clap::{ArgAction, Args, CommandFactory, Parser, Subcommand};

#[test]
fn arg_help_heading_applied() {
    #[derive(Debug, Clone, Parser)]
    struct CliOptions {
        #[arg(long)]
        #[arg(help_heading = Some("HEADING A"))]
        should_be_in_section_a: u32,

        #[arg(long)]
        no_section: u32,
    }

    let cmd = CliOptions::command();

    let should_be_in_section_a = cmd
        .get_arguments()
        .find(|a| a.get_id() == "should_be_in_section_a")
        .unwrap();
    assert_eq!(should_be_in_section_a.get_help_heading(), Some("HEADING A"));

    let should_be_in_section_b = cmd
        .get_arguments()
        .find(|a| a.get_id() == "no_section")
        .unwrap();
    assert_eq!(should_be_in_section_b.get_help_heading(), None);
}

#[test]
fn app_help_heading_applied() {
    #[derive(Debug, Clone, Parser)]
    #[command(next_help_heading = "DEFAULT")]
    struct CliOptions {
        #[arg(long)]
        #[arg(help_heading = Some("HEADING A"))]
        should_be_in_section_a: u32,

        #[arg(long)]
        should_be_in_default_section: u32,
    }

    let cmd = CliOptions::command();

    let should_be_in_section_a = cmd
        .get_arguments()
        .find(|a| a.get_id() == "should_be_in_section_a")
        .unwrap();
    assert_eq!(should_be_in_section_a.get_help_heading(), Some("HEADING A"));

    let should_be_in_default_section = cmd
        .get_arguments()
        .find(|a| a.get_id() == "should_be_in_default_section")
        .unwrap();
    assert_eq!(
        should_be_in_default_section.get_help_heading(),
        Some("DEFAULT")
    );
}

#[test]
fn app_help_heading_flattened() {
    // Used to help track the cause in tests
    #![allow(clippy::enum_variant_names)]

    #[derive(Debug, Clone, Parser)]
    struct CliOptions {
        #[command(flatten)]
        options_a: OptionsA,

        #[command(flatten)]
        options_b: OptionsB,

        #[command(subcommand)]
        sub_a: SubA,

        #[arg(long)]
        should_be_in_default_section: u32,
    }

    #[derive(Debug, Clone, Args)]
    #[command(next_help_heading = "HEADING A")]
    struct OptionsA {
        #[arg(long)]
        should_be_in_section_a: u32,
    }

    #[derive(Debug, Clone, Args)]
    #[command(next_help_heading = "HEADING B")]
    struct OptionsB {
        #[arg(long)]
        should_be_in_section_b: u32,
    }

    #[derive(Debug, Clone, Subcommand)]
    enum SubA {
        #[command(flatten)]
        SubB(SubB),
        #[command(subcommand)]
        SubC(SubC),
        SubAOne,
        #[command(next_help_heading = "SUB A")]
        SubATwo {
            should_be_in_sub_a: u32,
        },
    }

    #[derive(Debug, Clone, Subcommand)]
    enum SubB {
        #[command(next_help_heading = "SUB B")]
        SubBOne { should_be_in_sub_b: u32 },
    }

    #[derive(Debug, Clone, Subcommand)]
    enum SubC {
        #[command(next_help_heading = "SUB C")]
        SubCOne { should_be_in_sub_c: u32 },
    }

    let cmd = CliOptions::command();

    let should_be_in_section_a = cmd
        .get_arguments()
        .find(|a| a.get_id() == "should_be_in_section_a")
        .unwrap();
    assert_eq!(should_be_in_section_a.get_help_heading(), Some("HEADING A"));

    let should_be_in_section_b = cmd
        .get_arguments()
        .find(|a| a.get_id() == "should_be_in_section_b")
        .unwrap();
    assert_eq!(should_be_in_section_b.get_help_heading(), Some("HEADING B"));

    let should_be_in_section_b = cmd
        .get_arguments()
        .find(|a| a.get_id() == "should_be_in_default_section")
        .unwrap();
    assert_eq!(should_be_in_section_b.get_help_heading(), Some("HEADING B"));

    let sub_a_two = cmd.find_subcommand("sub-a-two").unwrap();

    let should_be_in_sub_a = sub_a_two
        .get_arguments()
        .find(|a| a.get_id() == "should_be_in_sub_a")
        .unwrap();
    assert_eq!(should_be_in_sub_a.get_help_heading(), Some("SUB A"));

    let sub_b_one = cmd.find_subcommand("sub-b-one").unwrap();

    let should_be_in_sub_b = sub_b_one
        .get_arguments()
        .find(|a| a.get_id() == "should_be_in_sub_b")
        .unwrap();
    assert_eq!(should_be_in_sub_b.get_help_heading(), Some("SUB B"));

    let sub_c = cmd.find_subcommand("sub-c").unwrap();
    let sub_c_one = sub_c.find_subcommand("sub-c-one").unwrap();

    let should_be_in_sub_c = sub_c_one
        .get_arguments()
        .find(|a| a.get_id() == "should_be_in_sub_c")
        .unwrap();
    assert_eq!(should_be_in_sub_c.get_help_heading(), Some("SUB C"));
}

#[test]
fn flatten_field_with_help_heading() {
    #[derive(Debug, Clone, Parser)]
    struct CliOptions {
        #[command(flatten)]
        #[command(next_help_heading = "HEADING A")]
        options_a: OptionsA,
    }

    #[derive(Debug, Clone, Args)]
    struct OptionsA {
        #[arg(long)]
        should_be_in_section_a: u32,
    }

    let cmd = CliOptions::command();

    let should_be_in_section_a = cmd
        .get_arguments()
        .find(|a| a.get_id() == "should_be_in_section_a")
        .unwrap();
    assert_eq!(should_be_in_section_a.get_help_heading(), Some("HEADING A"));
}

// The challenge with this test is creating an error situation not caught by `clap`'s error checking
// but by the code that `clap_derive` generates.
//
// Ultimately, the easiest way to confirm is to put a debug statement in the desired error path.
#[test]
fn derive_generated_error_has_full_context() {
    #[derive(Debug, Parser)]
    #[command(subcommand_negates_reqs = true)]
    struct Opts {
        #[arg(long)]
        req_str: String,

        #[command(subcommand)]
        cmd: Option<SubCommands>,
    }

    #[derive(Debug, Parser)]
    enum SubCommands {
        Sub {
            #[arg(short, long, action = clap::ArgAction::Count)]
            verbose: u8,
        },
    }

    let result = Opts::try_parse_from(["test", "sub"]);
    assert!(
        result.is_err(),
        "`SubcommandsNegateReqs` with non-optional `req_str` should fail: {:?}",
        result.unwrap()
    );

    let expected = r#"error: The following required argument was not provided: req_str

Usage: clap --req-str <REQ_STR>
       clap <COMMAND>

For more information, try '--help'.
"#;
    assert_eq!(result.unwrap_err().to_string(), expected);
}

#[test]
fn derive_order_next_order() {
    static HELP: &str = "\
Usage: test [OPTIONS]

Options:
      --flag-b               first flag
      --option-b <OPTION_B>  first option
  -h, --help                 Print help
  -V, --version              Print version
      --flag-a               second flag
      --option-a <OPTION_A>  second option
";

    #[derive(Parser, Debug)]
    #[command(name = "test", version = "1.2")]
    struct Args {
        #[command(flatten)]
        a: A,
        #[command(flatten)]
        b: B,
    }

    #[derive(Args, Debug)]
    #[command(next_display_order = 10000)]
    struct A {
        /// second flag
        #[arg(long)]
        flag_a: bool,
        /// second option
        #[arg(long)]
        option_a: Option<String>,
    }

    #[derive(Args, Debug)]
    #[command(next_display_order = 10)]
    struct B {
        /// first flag
        #[arg(long)]
        flag_b: bool,
        /// first option
        #[arg(long)]
        option_b: Option<String>,
    }

    use clap::CommandFactory;
    let mut cmd = Args::command();

    let help = cmd.render_help().to_string();
    snapbox::assert_eq(HELP, help);
}

#[test]
fn derive_order_next_order_flatten() {
    static HELP: &str = "\
Usage: test [OPTIONS]

Options:
      --flag-b               first flag
      --option-b <OPTION_B>  first option
  -h, --help                 Print help
  -V, --version              Print version
      --flag-a               second flag
      --option-a <OPTION_A>  second option
";

    #[derive(Parser, Debug)]
    #[command(name = "test", version = "1.2")]
    struct Args {
        #[command(flatten)]
        #[command(next_display_order = 10000)]
        a: A,
        #[command(flatten)]
        #[command(next_display_order = 10)]
        b: B,
    }

    #[derive(Args, Debug)]
    struct A {
        /// second flag
        #[arg(long)]
        flag_a: bool,
        /// second option
        #[arg(long)]
        option_a: Option<String>,
    }

    #[derive(Args, Debug)]
    struct B {
        /// first flag
        #[arg(long)]
        flag_b: bool,
        /// first option
        #[arg(long)]
        option_b: Option<String>,
    }

    use clap::CommandFactory;
    let mut cmd = Args::command();

    let help = cmd.render_help().to_string();
    snapbox::assert_eq(HELP, help);
}

#[test]
fn derive_order_no_next_order() {
    static HELP: &str = "\
Usage: test [OPTIONS]

Options:
      --flag-a               first flag
      --flag-b               second flag
  -h, --help                 Print help
      --option-a <OPTION_A>  first option
      --option-b <OPTION_B>  second option
  -V, --version              Print version
";

    #[derive(Parser, Debug)]
    #[command(name = "test", version = "1.2")]
    #[command(next_display_order = None)]
    struct Args {
        #[command(flatten)]
        a: A,
        #[command(flatten)]
        b: B,
    }

    #[derive(Args, Debug)]
    struct A {
        /// first flag
        #[arg(long)]
        flag_a: bool,
        /// first option
        #[arg(long)]
        option_a: Option<String>,
    }

    #[derive(Args, Debug)]
    struct B {
        /// second flag
        #[arg(long)]
        flag_b: bool,
        /// second option
        #[arg(long)]
        option_b: Option<String>,
    }

    use clap::CommandFactory;
    let mut cmd = Args::command();

    let help = cmd.render_help().to_string();
    snapbox::assert_eq(HELP, help);
}

#[test]
fn derive_possible_value_help() {
    static HELP: &str = "\
Application help

Usage: clap <ARG>

Arguments:
  <ARG>
          Argument help

          Possible values:
          - foo: Foo help
          - bar: Bar help

Options:
  -h, --help
          Print help (see a summary with '-h')
";

    /// Application help
    #[derive(Parser, PartialEq, Debug)]
    struct Args {
        /// Argument help
        #[arg(value_enum)]
        arg: ArgChoice,
    }

    #[derive(clap::ValueEnum, PartialEq, Debug, Clone)]
    enum ArgChoice {
        /// Foo help
        Foo,
        /// Bar help
        Bar,
    }

    use clap::CommandFactory;
    let mut cmd = Args::command();

    let help = cmd.render_long_help().to_string();
    snapbox::assert_eq(HELP, help);
}

#[test]
fn custom_help_flag() {
    #[derive(Debug, Clone, Parser)]
    #[command(disable_help_flag = true)]
    struct CliOptions {
        #[arg(short = 'h', long = "verbose-help", action = ArgAction::Help, value_parser = clap::value_parser!(bool))]
        help: (),
    }

    let result = CliOptions::try_parse_from(["cmd", "--verbose-help"]);
    let err = result.unwrap_err();
    assert_eq!(err.kind(), clap::error::ErrorKind::DisplayHelp);

    CliOptions::try_parse_from(["cmd"]).unwrap();
}

#[test]
fn custom_version_flag() {
    #[derive(Debug, Clone, Parser)]
    #[command(disable_version_flag = true, version = "2.0.0")]
    struct CliOptions {
        #[arg(short = 'V', long = "verbose-version", action = ArgAction::Version, value_parser = clap::value_parser!(bool))]
        version: (),
    }

    let result = CliOptions::try_parse_from(["cmd", "--verbose-version"]);
    let err = result.unwrap_err();
    assert_eq!(err.kind(), clap::error::ErrorKind::DisplayVersion);

    CliOptions::try_parse_from(["cmd"]).unwrap();
}
