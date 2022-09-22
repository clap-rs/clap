use clap::CommandFactory;
use clap::Parser;

use crate::utils::get_help;
use crate::utils::get_long_help;

#[test]
fn app_name_in_short_help_from_struct() {
    #[derive(Parser)]
    #[command(name = "my-cmd")]
    struct MyApp {}

    let help = get_help::<MyApp>();

    assert!(help.contains("my-cmd"));
}

#[test]
fn app_name_in_long_help_from_struct() {
    #[derive(Parser)]
    #[command(name = "my-cmd")]
    struct MyApp {}

    let help = get_help::<MyApp>();

    assert!(help.contains("my-cmd"));
}

#[test]
fn app_name_in_short_help_from_enum() {
    #[derive(Parser)]
    #[command(name = "my-cmd")]
    enum MyApp {}

    let help = get_help::<MyApp>();

    assert!(help.contains("my-cmd"));
}

#[test]
fn app_name_in_long_help_from_enum() {
    #[derive(Parser)]
    #[command(name = "my-cmd")]
    enum MyApp {}

    let help = get_long_help::<MyApp>();

    assert!(help.contains("my-cmd"));
}

#[test]
fn app_name_in_short_version_from_struct() {
    #[derive(Parser)]
    #[command(name = "my-cmd")]
    struct MyApp {}

    let version = MyApp::command().render_version();

    assert!(version.contains("my-cmd"));
}

#[test]
fn app_name_in_long_version_from_struct() {
    #[derive(Parser)]
    #[command(name = "my-cmd")]
    struct MyApp {}

    let version = MyApp::command().render_long_version();

    assert!(version.contains("my-cmd"));
}

#[test]
fn app_name_in_short_version_from_enum() {
    #[derive(Parser)]
    #[command(name = "my-cmd")]
    enum MyApp {}

    let version = MyApp::command().render_version();

    assert!(version.contains("my-cmd"));
}

#[test]
fn app_name_in_long_version_from_enum() {
    #[derive(Parser)]
    #[command(name = "my-cmd")]
    enum MyApp {}

    let version = MyApp::command().render_long_version();

    assert!(version.contains("my-cmd"));
}
