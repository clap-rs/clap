use crate::{build::app::Propagation, App, AppSettings};

#[test]
fn global_version() {
    let mut app = App::new("global_version")
        .setting(AppSettings::GlobalVersion)
        .version("1.1")
        .subcommand(App::new("sub1"));
    app._propagate(Propagation::NextLevel);
    assert_eq!(app.subcommands[0].version, Some("1.1"));
}

#[test]
fn global_setting() {
    let mut app = App::new("test")
        .global_setting(AppSettings::ColoredHelp)
        .subcommand(App::new("subcmd"));
    app._propagate(Propagation::NextLevel);
    assert!(app
        .subcommands
        .iter()
        .find(|s| s.name == "subcmd")
        .unwrap()
        .is_set(AppSettings::ColoredHelp));
}

#[test]
fn global_settings() {
    let mut app = App::new("test")
        .global_setting(AppSettings::ColoredHelp)
        .global_setting(AppSettings::TrailingVarArg)
        .subcommand(App::new("subcmd"));
    app._propagate(Propagation::NextLevel);
    assert!(app
        .subcommands
        .iter()
        .find(|s| s.name == "subcmd")
        .unwrap()
        .is_set(AppSettings::ColoredHelp));
    assert!(app
        .subcommands
        .iter()
        .find(|s| s.name == "subcmd")
        .unwrap()
        .is_set(AppSettings::TrailingVarArg));
}
