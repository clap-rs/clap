#[macro_use]
extern crate clap;

use clap::{App, Arg, SubCommand};


fn main() {
    let m_val_names = ["one", "two"];
    let args = "-o --option=[opt]... 'tests options'
                [positional] 'tests positionals'";
    let opt3_vals = ["fast", "slow"];
    let pos3_vals = ["vi", "emacs"];
    let matches = App::new("claptests")
                        .version("v1.4.8")
                        .about("tests clap library")
                        .author("Kevin K. <kbknapp@gmail.com>")
                        .args_from_usage(args)
                        .arg(Arg::from_usage("-f --flag... 'tests flags'")
                            .global(true))
                        .args(&[
                            Arg::from_usage("[flag2] -F 'tests flags with exclusions'").conflicts_with("flag").requires("option2"),
                            Arg::from_usage("--long-option-2 [option2] 'tests long options with exclusions'").conflicts_with("option").requires("positional2"),
                            Arg::from_usage("[positional2] 'tests positionals with exclusions'"),
                            Arg::from_usage("-O --Option [option3] 'tests options with specific value sets'").possible_values(&opt3_vals),
                            Arg::from_usage("[positional3]... 'tests positionals with specific values'").possible_values(&pos3_vals),
                            Arg::from_usage("--multvals [multvals] 'Tests mutliple values, not mult occs'").value_names(&m_val_names),
                            Arg::from_usage("--multvalsmo [multvalsmo]... 'Tests mutliple values, not mult occs'").value_names(&m_val_names),
                            Arg::from_usage("--minvals2 [minvals]... 'Tests 2 min vals'").min_values(2),
                            Arg::from_usage("--maxvals3 [maxvals]... 'Tests 3 max vals'").max_values(3)
                        ])
                        .subcommand(SubCommand::with_name("subcmd")
                                                .about("tests subcommands")
                                                .version("0.1")
                                                .author("Kevin K. <kbknapp@gmail.com>")
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
        if let Some(ov) = matches.values_of("opt") {
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
        if let Some(ov) = matches.values_of("opt") {
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
            if matches.is_present("flag") {
                println!("flag present {} times", matches.occurrences_of("flag"));
            } else {
                println!("flag NOT present");
            }

            if matches.is_present("scoption") {
                if let Some(v) = matches.value_of("scoption") {
                    println!("scoption present with value: {}", v);
                }
                if let Some(ov) = matches.values_of("scoption") {
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
