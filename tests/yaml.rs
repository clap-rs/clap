#[macro_use]
extern crate clap;

use clap::App;

#[test]
#[cfg(feature="yaml")]
fn create_app_from_yaml() {
    let yml = load_yaml!("app.yml");
    App::from_yaml(yml);
}