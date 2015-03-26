extern crate clap;

use clap::{App, Arg, SubCommand};

fn main() {
	let version = format!("{}.{}.{}{}",
                          env!("CARGO_PKG_VERSION_MAJOR"),
                          env!("CARGO_PKG_VERSION_MINOR"),
                          env!("CARGO_PKG_VERSION_PATCH"),
                          option_env!("CARGO_PKG_VERSION_PRE").unwrap_or(""));	
    let matches = App::new("claptests")
    					.version(&version[..])
    					.about("tests clap library")
    					.author("Kevin K. <kbknapp@gmail.com>")
    					.arg(Arg::new("flag")
    								.short("f")
    								.long("flag")
    								.help("tests flags")
    								.multiple(true))
    					.arg(Arg::new("option")
    								.short("o")
    								.long("option")
    								.help("tests options")
    								.takes_value(true)
    								.multiple(true))
    					.arg(Arg::new("positional")
    								.index(1)
    								.help("tests positionals"))
    					.subcommand(SubCommand::new("subcmd")
    											.about("tests subcommands")
    											.version("0.1")
    											.author("Kevin K. <kbknapp@gmail.com>")
						    					.arg(Arg::new("scflag")
						    								.short("f")
						    								.long("flag")
						    								.help("tests flags")
						    								.multiple(true))
						    					.arg(Arg::new("scoption")
						    								.short("o")
						    								.long("option")
						    								.help("tests options")
						    								.takes_value(true)
						    								.multiple(true))
						    					.arg(Arg::new("scpositional")
						    								.index(1)
						    								.help("tests positionals")))
    					.get_matches();

    if matches.is_present("flag") {
    	println!("flag present {} times", matches.occurrences_of("flag"));
    } else {
    	println!("flag NOT present");
    }

    if matches.is_present("option") {
        if let Some(v) = matches.value_of("option") {
        	println!("option present with value: {}", v);
        }
        if let Some(ref ov) = matches.values_of("option") {
        	for o in ov {
        		println!("An option: {}", o);
        	}
        }
    } else {
        println!("option NOT present");
    }

    if let Some(p) = matches.value_of("positional") {
    	println!("positional present with value: {}", p);
    } else {
        println!("positional NOT present");
    }

    if matches.is_present("subcmd") {
        println!("subcmd present");
        if let Some(matches) = matches.subcommand_matches("subcmd") {
            if matches.is_present("scflag") {
                println!("scflag present {} times", matches.occurrences_of("scflag"));
            } else {
                println!("scflag NOT present");
            }

            if matches.is_present("scoption") {
                if let Some(v) = matches.value_of("scoption") {
                    println!("scoption present with value: {}", v);
                }
                if let Some(ref ov) = matches.values_of("scoption") {
                    for o in ov {
                        println!("An scoption: {}", o);
                    }
                }
            } else {
                println!("scoption NOT present");
            }

            if let Some(p) = matches.value_of("scpositional") {
                println!("scpositional present with value: {}", p);
            }
        }
    } else {
        println!("subcmd NOT present");
    }
}
