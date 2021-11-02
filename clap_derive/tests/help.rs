use clap::{AppSettings, Args, IntoApp, Parser, Subcommand};

#[test]
fn arg_help_heading_applied() {
    #[derive(Debug, Clone, Parser)]
    struct CliOptions {
        #[clap(long)]
        #[clap(help_heading = Some("HEADING A"))]
        should_be_in_section_a: Option<u32>,

        #[clap(long)]
        no_section: Option<u32>,
    }

    let app = CliOptions::into_app();

    let should_be_in_section_a = app
        .get_arguments()
        .find(|a| a.get_name() == "should-be-in-section-a")
        .unwrap();
    assert_eq!(should_be_in_section_a.get_help_heading(), Some("HEADING A"));

    let should_be_in_section_b = app
        .get_arguments()
        .find(|a| a.get_name() == "no-section")
        .unwrap();
    assert_eq!(should_be_in_section_b.get_help_heading(), None);
}

#[test]
fn app_help_heading_applied() {
    #[derive(Debug, Clone, Parser)]
    #[clap(help_heading = "DEFAULT")]
    struct CliOptions {
        #[clap(long)]
        #[clap(help_heading = Some("HEADING A"))]
        should_be_in_section_a: Option<u32>,

        #[clap(long)]
        should_be_in_default_section: Option<u32>,
    }

    let app = CliOptions::into_app();

    let should_be_in_section_a = app
        .get_arguments()
        .find(|a| a.get_name() == "should-be-in-section-a")
        .unwrap();
    assert_eq!(should_be_in_section_a.get_help_heading(), Some("HEADING A"));

    let should_be_in_default_section = app
        .get_arguments()
        .find(|a| a.get_name() == "should-be-in-default-section")
        .unwrap();
    assert_eq!(
        should_be_in_default_section.get_help_heading(),
        Some("DEFAULT")
    );
}

#[test]
fn app_help_heading_flattened() {
    #[derive(Debug, Clone, Parser)]
    struct CliOptions {
        #[clap(flatten)]
        options_a: OptionsA,

        #[clap(flatten)]
        options_b: OptionsB,

        #[clap(subcommand)]
        sub_a: SubA,

        #[clap(long)]
        should_be_in_default_section: Option<u32>,
    }

    #[derive(Debug, Clone, Args)]
    #[clap(help_heading = "HEADING A")]
    struct OptionsA {
        #[clap(long)]
        should_be_in_section_a: Option<u32>,
    }

    #[derive(Debug, Clone, Args)]
    #[clap(help_heading = "HEADING B")]
    struct OptionsB {
        #[clap(long)]
        should_be_in_section_b: Option<u32>,
    }

    #[derive(Debug, Clone, Subcommand)]
    enum SubA {
        #[clap(flatten)]
        SubB(SubB),
        #[clap(subcommand)]
        SubC(SubC),
        SubAOne,
        #[clap(help_heading = "SUB A")]
        SubATwo {
            should_be_in_sub_a: Option<u32>,
        },
    }

    #[derive(Debug, Clone, Subcommand)]
    enum SubB {
        #[clap(help_heading = "SUB B")]
        SubBOne { should_be_in_sub_b: Option<u32> },
    }

    #[derive(Debug, Clone, Subcommand)]
    enum SubC {
        #[clap(help_heading = "SUB C")]
        SubCOne { should_be_in_sub_c: Option<u32> },
    }

    let app = CliOptions::into_app();

    let should_be_in_section_a = app
        .get_arguments()
        .find(|a| a.get_name() == "should-be-in-section-a")
        .unwrap();
    assert_eq!(should_be_in_section_a.get_help_heading(), Some("HEADING A"));

    let should_be_in_section_b = app
        .get_arguments()
        .find(|a| a.get_name() == "should-be-in-section-b")
        .unwrap();
    assert_eq!(should_be_in_section_b.get_help_heading(), Some("HEADING B"));

    let should_be_in_default_section = app
        .get_arguments()
        .find(|a| a.get_name() == "should-be-in-default-section")
        .unwrap();
    assert_eq!(should_be_in_default_section.get_help_heading(), None);

    let sub_a_two = app.find_subcommand("sub-a-two").unwrap();

    let should_be_in_sub_a = sub_a_two
        .get_arguments()
        .find(|a| a.get_name() == "should-be-in-sub-a")
        .unwrap();
    assert_eq!(should_be_in_sub_a.get_help_heading(), Some("SUB A"));

    let sub_b_one = app.find_subcommand("sub-b-one").unwrap();

    let should_be_in_sub_b = sub_b_one
        .get_arguments()
        .find(|a| a.get_name() == "should-be-in-sub-b")
        .unwrap();
    assert_eq!(should_be_in_sub_b.get_help_heading(), Some("SUB B"));

    let sub_c = app.find_subcommand("sub-c").unwrap();
    let sub_c_one = sub_c.find_subcommand("sub-c-one").unwrap();

    let should_be_in_sub_c = sub_c_one
        .get_arguments()
        .find(|a| a.get_name() == "should-be-in-sub-c")
        .unwrap();
    assert_eq!(should_be_in_sub_c.get_help_heading(), Some("SUB C"));
}

#[test]
fn flatten_field_with_help_heading() {
    #[derive(Debug, Clone, Parser)]
    struct CliOptions {
        #[clap(flatten)]
        #[clap(help_heading = "HEADING A")]
        options_a: OptionsA,
    }

    #[derive(Debug, Clone, Args)]
    struct OptionsA {
        #[clap(long)]
        should_be_in_section_a: Option<u32>,
    }

    let app = CliOptions::into_app();

    let should_be_in_section_a = app
        .get_arguments()
        .find(|a| a.get_name() == "should-be-in-section-a")
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
    #[clap(setting(AppSettings::SubcommandsNegateReqs))]
    struct Opts {
        #[clap(long)]
        req_str: String,

        #[clap(subcommand)]
        cmd: Option<SubCommands>,
    }

    #[derive(Debug, Parser)]
    enum SubCommands {
        Sub {
            #[clap(short, long, parse(from_occurrences))]
            verbose: u8,
        },
    }

    let result = Opts::try_parse_from(&["test", "sub"]);
    assert!(
        result.is_err(),
        "`SubcommandsNegateReqs` with non-optional `req_str` should fail: {:?}",
        result.unwrap()
    );

    let expected = r#"error: The following required argument was not provided: req-str

USAGE:
    clap_derive --req-str <REQ_STR>
    clap_derive <SUBCOMMAND>

For more information try --help
"#;
    assert_eq!(result.unwrap_err().to_string(), expected);
}
