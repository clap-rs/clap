use crate::App;

#[test]
fn propagate_version() {
    let mut app = App::new("test")
        .propagate_version(true)
        .version("1.1")
        .subcommand(App::new("sub1"));
    app._propagate();
    assert_eq!(
        app.get_subcommands().next().unwrap().get_version(),
        Some("1.1")
    );
}

#[test]
fn global_setting() {
    let mut app = App::new("test")
        .disable_version_flag(true)
        .subcommand(App::new("subcmd"));
    app._propagate();
    assert!(app
        .get_subcommands()
        .find(|s| s.get_name() == "subcmd")
        .unwrap()
        .is_disable_version_flag_set());
}

// This test will *fail to compile* if App is not Send + Sync
#[test]
fn app_send_sync() {
    fn foo<T: Send + Sync>(_: T) {}
    foo(App::new("test"))
}

#[test]
fn issue_2090() {
    let mut app = App::new("app")
        .disable_version_flag(true)
        .subcommand(App::new("sub"));
    app._build();

    assert!(app
        .get_subcommands()
        .next()
        .unwrap()
        .is_disable_version_flag_set());
}
