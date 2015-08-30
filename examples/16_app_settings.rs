extern crate clap;

use clap::{App, AppSettings, SubCommand};

#[allow(unused_variables)]
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

                        .arg_from_usage("<input> 'input file to use'")
                                            // Required positional argument called input.  This
                                            // will be only required if subcommand is not present.

                        .subcommand(SubCommand::with_name("help")
                                                .about("shows help message"))
                                            // if program is invoked with subcommand, you do not
                                            // need to specify the <input> argument anymore due to
                                            // the AppSettings::SubcommandsNegateReqs setting.

                        .get_matches();

    // Contiued program logic goes here...
}
