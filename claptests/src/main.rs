extern crate clap;

use clap::{App, Arg, SubCommand};

fn main() {
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
                        .args(vec![
                            Arg::new("flag2").short("F").mutually_excludes("flag").help("tests flags with exclusions").requires("option2"),
                            Arg::new("option2").takes_value(true).long("long-option-2").mutually_excludes("option").help("tests long options with exclusions and requirements").requires("positional2"),
                            Arg::new("positional2").index(2).help("tests positionals with exclusions and multiple"),
                            Arg::new("option3").takes_value(true).short("O").possible_values(vec!["fast", "slow"]).help("test options with specific value sets"),
                            Arg::new("positional3").index(3).multiple(true).possible_values(vec!["vi", "emacs"]).help("tests positionals with specific value sets")
                        ])
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
            println!("option present {} times with value: {}",matches.occurrences_of("option"), v);
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

    if matches.is_present("option") {
        if let Some(v) = matches.value_of("option") {
            println!("option present {} times with value: {}",matches.occurrences_of("option"), v);
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
