extern crate clap;

use clap::{App, Arg};

fn main() {
	let matches = App::new("MyApp")
						.version("1.0")
						.author("Kevin K. <kbknapp@gmail.com>")
						.about("Does awesome things")
						.arg(Arg::new("config")
									.short("c")
									.long("config")
									.help("Sets a custom config file")
									.takes_value(true))
						.arg(Arg::new("output")
									.help("Sets an optional output file")
									.index(1))
						.arg(Arg::new("debug")
									.short("d")
 								.multiple(true)
									.help("Turn debugging information on"))
						.get_matches();

	if let Some(o) = matches.value_of("output") {
		println!("Value for output: {}", o);
	}
 
	if let Some(c) = matches.value_of("config") {
		println!("Value for config: {}", c);
	}

	match matches.occurrences_of("debug") {
 		0 => println!("Debug mode is off"),
		1 => println!("Debug mode is kind of on"),
		2 => println!("Debug mode is on"),
		3 | _ => println!("Don't be crazy"),
 	}
	print!("App is running...");
	println!("done.");
}