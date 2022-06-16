use clap::CommandFactory;
use clap::Parser;
#[test]
fn app_name_in_short_help_from_struct() {
    #[derive(Parser)]
    #[clap(name = "my-cmd")]
    struct MyApp {}

    let mut help = Vec::new();
    MyApp::command().write_help(&mut help).unwrap();
    let help = String::from_utf8(help).unwrap();

    assert!(help.contains("my-cmd"));
}

#[test]
fn app_name_in_long_help_from_struct() {
    #[derive(Parser)]
    #[clap(name = "my-cmd")]
    struct MyApp {}

    let mut help = Vec::new();
    MyApp::command().write_long_help(&mut help).unwrap();
    let help = String::from_utf8(help).unwrap();

    assert!(help.contains("my-cmd"));
}

#[test]
fn app_name_in_short_help_from_enum() {
    #[derive(Parser)]
    #[clap(name = "my-cmd")]
    enum MyApp {}

    let mut help = Vec::new();
    MyApp::command().write_help(&mut help).unwrap();
    let help = String::from_utf8(help).unwrap();

    assert!(help.contains("my-cmd"));
}

#[test]
fn app_name_in_long_help_from_enum() {
    #[derive(Parser)]
    #[clap(name = "my-cmd")]
    enum MyApp {}

    let mut help = Vec::new();
    MyApp::command().write_long_help(&mut help).unwrap();
    let help = String::from_utf8(help).unwrap();

    assert!(help.contains("my-cmd"));
}

#[test]
fn app_name_in_short_version_from_struct() {
    #[derive(Parser)]
    #[clap(name = "my-cmd")]
    struct MyApp {}

    let version = MyApp::command().render_version();

    assert!(version.contains("my-cmd"));
}

#[test]
fn app_name_in_long_version_from_struct() {
    #[derive(Parser)]
    #[clap(name = "my-cmd")]
    struct MyApp {}

    let version = MyApp::command().render_long_version();

    assert!(version.contains("my-cmd"));
}

#[test]
fn app_name_in_short_version_from_enum() {
    #[derive(Parser)]
    #[clap(name = "my-cmd")]
    enum MyApp {}

    let version = MyApp::command().render_version();

    assert!(version.contains("my-cmd"));
}

#[test]
fn app_name_in_long_version_from_enum() {
    #[derive(Parser)]
    #[clap(name = "my-cmd")]
    enum MyApp {}

    let version = MyApp::command().render_long_version();

    assert!(version.contains("my-cmd"));
}
