// In order to use YAML to define your CLI you must compile clap with the "yaml" feature becasue
// it's **not** included by default.
//
// In order to do this, ensure your Cargo.toml looks like one of the following:
//
// [dependencies.clap]
// features = ["yaml"]
//
// __OR__
//
// [dependencies]
// clap = { features = ["yaml"] }

// Using yaml requires calling a clap macro `load_yaml!()` so we must use the '#[macro_use]'
// directive
// Note: If you're using clap as a dependency and don't have a feature for your users called
// "yaml", you'll need to remove the #[cfg(feature = "yaml")] conditional compilation attribute
#[cfg(feature = "yaml")]
#[macro_use]
extern crate clap;

#[cfg(feature = "yaml")]
fn main() {
    use clap::App;

    // To load a yaml file containing our CLI definition such as the example '17_yaml.yml' we can
    // use the convenience macro which loads the file at compile relative to the current file
    // similiar to how modules are found.
    //
    // Then we pass that yaml object to App to build the CLI.
    //
    // Finally we call get_matches() to start the parsing process. We use the matches just as we
    // normally would
    let yml = load_yaml!("17_yaml.yml");
    let m = App::from(yml).get_matches();

    // Because the example 17_yaml.yml is rather large we'll just look a single arg so you can
    // see that it works...
    if let Some(mode) = m.value_of("mode") {
        match mode {
            "vi" => println!("You are using vi"),
            "emacs" => println!("You are using emacs..."),
            _ => unreachable!(),
        }
    } else {
        println!("--mode <MODE> wasn't used...");
    }
}

#[cfg(not(feature = "yaml"))]
fn main() {
    // As stated above, if clap is not compiled with the YAML feature, it is disabled.
    println!("YAML feature is disabled.");
    println!("Pass --features yaml to cargo when trying this example.");
}
