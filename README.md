# clap-rs
Command Line Argument Parser written in Rust

 A simply library for parsing command line arguments when writing 
 command line and console applications.


 You can use `clap` to lay out a list of possible valid command line arguments and let `clap` parse the string given by the user at runtime.
 When using `clap` you define a set of parameters and rules for your arguments and at runtime `clap` will determine their validity.
 Also, `clap` provides the traditional version and help switches 'for free' by parsing the list of possible valid arguments lazily at runtime.
 i.e. only when it's been determined that the user wants or needs to see the help and version information.
 
 After defining a list of possible valid arguments you get a list of matches that the user supplied at runtime. You can then use this list to
 determine the functioning of your program.

 Example:
 
 ```rust
 extern crate clap;
 use clap::{Arg, App};

 // ...
 
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
 
 // more porgram logic goes here...
 ```

 If you were to compile the above program and run it with the flag `--help` or `-h` the following output woud be presented

 ```sh
 $ myprog --help
 MyApp 1.0
 Kevin K. <kbknapp@gmail.com>
 Does awesome things
 
 USAGE:
 	MyApp [FLAGS] [OPTIONS] [POSITIONAL]
 
 FLAGS:
 	-d   			Turn debugging information on
 	-h,--help		Prints this message
 	-v,--version	Prints version information
 
 OPTIONS:
	 -c,--config <config>		Sets a custom config file

 POSITIONAL ARGUMENTS:
	 output			Sets an optional output file
 ```

## Installation
Simply add `clap` as a dependecy in your `Cargo.toml` file to use from crates.io:

 ```
 [dependencies]
 clap = "*"
 ```
 Or to simply track the latest on the master branch at github:

```
[dependencies.clap]
git = "https://github.com/kbknapp/clap-rs.git"
```
Then run `cargo build` or `cargo update` for your project.
