extern crate clap;

use clap::{App, Arg, AppSettings};

// ./app grep -i bash /etc/passwd
// ./app -o foo -o bar grep -i bash /etc/passwd
fn main() {
    let matches = App::new("app")
        .setting(AppSettings::AllowExternalSubcommands)
        .arg(
            Arg::with_name("option")
                .short("o")
                .required(false)
                .takes_value(true)
                .multiple(true)
                .number_of_values(1),
        )
        .get_matches();

    println!("{:#?}", matches);
}
