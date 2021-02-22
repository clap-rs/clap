use clap::Clap;
use clap::IntoApp;
#[test]
fn app_name_in_short_help_from_struct() {
    #[derive(Clap)]
    #[clap(name = "my-app")]
    struct MyApp {}

    let mut help = Vec::new();
    MyApp::into_app().write_help(&mut help).unwrap();
    let help = String::from_utf8(help).unwrap();

    assert!(help.contains("my-app"));
}

#[test]
fn app_name_in_long_help_from_struct() {
    #[derive(Clap)]
    #[clap(name = "my-app")]
    struct MyApp {}

    let mut help = Vec::new();
    MyApp::into_app().write_long_help(&mut help).unwrap();
    let help = String::from_utf8(help).unwrap();

    assert!(help.contains("my-app"));
}

#[test]
fn app_name_in_short_help_from_enum() {
    #[derive(Clap)]
    #[clap(name = "my-app")]
    enum MyApp {}

    let mut help = Vec::new();
    MyApp::into_app().write_help(&mut help).unwrap();
    let help = String::from_utf8(help).unwrap();

    assert!(help.contains("my-app"));
}

#[test]
fn app_name_in_long_help_from_enum() {
    #[derive(Clap)]
    #[clap(name = "my-app")]
    enum MyApp {}

    let mut help = Vec::new();
    MyApp::into_app().write_long_help(&mut help).unwrap();
    let help = String::from_utf8(help).unwrap();

    assert!(help.contains("my-app"));
}

#[test]
fn app_name_in_short_version_from_struct() {
    #[derive(Clap)]
    #[clap(name = "my-app")]
    struct MyApp {}

    let version = MyApp::into_app().render_version();

    assert!(version.contains("my-app"));
}

#[test]
fn app_name_in_long_version_from_struct() {
    #[derive(Clap)]
    #[clap(name = "my-app")]
    struct MyApp {}

    let version = MyApp::into_app().render_long_version();

    assert!(version.contains("my-app"));
}

#[test]
fn app_name_in_short_version_from_enum() {
    #[derive(Clap)]
    #[clap(name = "my-app")]
    enum MyApp {}

    let version = MyApp::into_app().render_version();

    assert!(version.contains("my-app"));
}

#[test]
fn app_name_in_long_version_from_enum() {
    #[derive(Clap)]
    #[clap(name = "my-app")]
    enum MyApp {}

    let version = MyApp::into_app().render_long_version();

    assert!(version.contains("my-app"));
}
