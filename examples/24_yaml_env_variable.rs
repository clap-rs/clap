#[cfg(feature="yaml")]
fn main() {
    use clap::{load_yaml, App};
    use std::env;

    // The the env variable used in the yaml file
    env::set_var("TEST_NAME", "This is an enviroment variable value");

    // Load yaml file
    let yaml = load_yaml!("24_yaml.yml");
    let m = App::from(yaml).get_matches();
    println!("Type --help and check that the version is the one in the env");
}

#[cfg(not(feature = "yaml"))]
fn main() {
    // As stated above, if clap is not compiled with the YAML feature, it is disabled.
    println!("YAML feature is disabled.");
    println!("Pass --features yaml to cargo when trying this example.");
}
