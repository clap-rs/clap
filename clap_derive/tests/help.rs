use clap::{Args, IntoApp, Parser};

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
}
