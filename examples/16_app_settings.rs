extern crate clap;

use clap::{App, AppSettings};

fn main() {
    // You can use AppSettings to change the application level behavior of clap. .setting() function
    // of App struct takes AppSettings enum as argument. There is also .settings() function which
    // takes slice of AppSettings enum.  You can learn more about AppSettings in the documentation,
    // which also has examples on each setting.
    //
    // This example will only show usage of one AppSettings setting. See documentation for more
    // information.

    let matches = App::new("myapp")
        .setting(AppSettings::SubcommandsNegateReqs)
        // Negates requirement of parent command.
        .arg("<input> 'input file to use'")
        // Required positional argument called input.  This
        // will be only required if subcommand is not present.
        .subcommand(App::new("test").about("does some testing"))
        // if program is invoked with subcommand, you do not
        // need to specify the <input> argument anymore due to
        // the AppSettings::SubcommandsNegateReqs setting.
        .get_matches();

    // Calling unwrap() on "input" would not be advised here, because although it's required,
    // if the user uses a subcommand, those requirements are no longer required. Hence, we should
    // use some sort of 'if let' construct
    if let Some(inp) = matches.value_of("input") {
        println!("The input file is: {}", inp);
    }

    match matches.subcommand() {
        ("test", _) => println!("The 'test' subcommand was used"),
        _ => unreachable!(),
    }
}
