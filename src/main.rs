extern crate clap;

use clap::{App, Arg, ArgMatches};

fn main() {
	let matches = App::new("MyApp")
						.version("1.0")
						.author("Jostmon <jostmon2@gmail.com>")
						.about("Does awesome things")
						.arg(Arg::new("config")
									.short("c")
									.long("config")
									.required(true)
									.help("Sets a custom config file")
									.takes_value(true))
						.arg(Arg::new("output")
									.short("o")
									.long("output")
									.help("Sets an optional output file")
									.takes_value(true))
						.arg(Arg::new("debug")
									.short("d")
									.help("Turn debugging information on"))
						.get_matches();

	if let Some(o) = matches.value_of("output") {
		println!("Value for output: {}", o);
	}

	println!("Config file: {}", matches.value_of("config").unwrap());

	if matches.is_present("debug") {
		println!("Debug mode on");
	}

	println!("App is running...");
	println!("Done.");
}