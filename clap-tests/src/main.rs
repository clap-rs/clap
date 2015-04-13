extern crate clap;

use clap::{App, Arg, SubCommand};

fn main() {
    let args = "-f --flag... 'tests flags'
                -o --option=[opt]... 'tests options'
                [positional] 'tests positionals'";
    // Test version from Cargo.toml
    let version = format!("{}.{}.{}{}",
                          env!("CARGO_PKG_VERSION_MAJOR"),
                          env!("CARGO_PKG_VERSION_MINOR"),
                          env!("CARGO_PKG_VERSION_PATCH"),
                          option_env!("CARGO_PKG_VERSION_PRE").unwrap_or(""));  
    let matches = App::new("claptests")
                        .version(&version[..])
                        .about("tests clap library")
                        .author("Kevin K. <kbknapp@gmail.com>")
                        .args_from_usage(args)
                        .args(vec![
                            Arg::from_usage("[flag2] -F 'tests flags with exclusions'").mutually_excludes("flag").requires("option2"),
                            Arg::from_usage("--long-option-2 [option2] 'tests long options with exclusions'").mutually_excludes("option").requires("positional2"),
                            Arg::from_usage("[positional2] 'tests positionals with exclusions'"),
                            Arg::from_usage("-O [option3] 'tests options with specific value sets'").possible_values(vec!["fast", "slow"]),
                            Arg::from_usage("[positional3]... 'tests positionals with specific values'").possible_values(vec!["vi", "emacs"])
                        ])
                        .subcommand(SubCommand::new("subcmd")
                                                .about("tests subcommands")
                                                .version("0.1")
                                                .author("Kevin K. <kbknapp@gmail.com>")
                                                .arg_from_usage("[scflag] -f --flag... 'tests flags'")
                                                .arg_from_usage("-o --option [scoption]... 'tests options'")
                                                .arg_from_usage("[scpositional] 'tests positionals'"))
                        .get_matches();

    if matches.is_present("flag") {
        println!("flag present {} times", matches.occurrences_of("flag"));
    } else {
        println!("flag NOT present");
    }

    if matches.is_present("opt") {
        if let Some(v) = matches.value_of("opt") {
            println!("option present {} times with value: {}",matches.occurrences_of("opt"), v);
        }
        if let Some(ref ov) = matches.values_of("opt") {
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

    if matches.is_present("flag2") {
        println!("flag2 present");
        println!("option2 present with value of: {}", matches.value_of("option2").unwrap());
        println!("positional2 present with value of: {}", matches.value_of("positional2").unwrap());
    } else {
        println!("flag2 NOT present");
        println!("option2 maybe present with value of: {}", matches.value_of("option2").unwrap_or("Nothing"));
        println!("positional2 maybe present with value of: {}", matches.value_of("positional2").unwrap_or("Nothing"));
    }

    match matches.value_of("option3").unwrap_or("") {
        "fast" => println!("option3 present quickly"),
        "slow" => println!("option3 present slowly"),
        _      => println!("option3 NOT present")
    }

    match matches.value_of("positional3").unwrap_or("") {
        "vi" => println!("positional3 present in vi mode"),
        "emacs" => println!("positional3 present in emacs mode"),
        _      => println!("positional3 NOT present")
    }

    if matches.is_present("opt") {
        if let Some(v) = matches.value_of("opt") {
            println!("option present {} times with value: {}",matches.occurrences_of("opt"), v);
        }
        if let Some(ref ov) = matches.values_of("opt") {
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
