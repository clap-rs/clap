#[allow(unused_imports, dead_code)]
mod test {
    use std::str;
    use std::io::Write;

    use regex::Regex;

    use clap::{App, Arg, SubCommand, ArgGroup};

    pub fn complex_app() -> App<'static, 'static> {
        let args = "-o --option=[opt]... 'tests options'
                    [positional] 'tests positionals'";
        let opt3_vals = ["fast", "slow"];
        let pos3_vals = ["vi", "emacs"];
        App::new("clap-test")
            .version("v1.4.8")
            .about("tests clap library")
            .author("Kevin K. <kbknapp@gmail.com>")
            .args_from_usage(args)
            .arg(Arg::from_usage("-f --flag... 'tests flags'")
                .global(true))
            .args(&[
                Arg::from_usage("[flag2] -F 'tests flags with exclusions'").conflicts_with("flag").requires("long-option-2"),
                Arg::from_usage("--long-option-2 [option2] 'tests long options with exclusions'").conflicts_with("option").requires("positional2"),
                Arg::from_usage("[positional2] 'tests positionals with exclusions'"),
                Arg::from_usage("-O --Option [option3] 'specific vals'").possible_values(&opt3_vals),
                Arg::from_usage("[positional3]... 'tests specific values'").possible_values(&pos3_vals),
                Arg::from_usage("--multvals [one] [two] 'Tests mutliple values, not mult occs'"),
                Arg::from_usage("--multvalsmo... [one] [two] 'Tests mutliple values, and mult occs'"),
                Arg::from_usage("--minvals2 [minvals]... 'Tests 2 min vals'").min_values(2),
                Arg::from_usage("--maxvals3 [maxvals]... 'Tests 3 max vals'").max_values(3)
            ])
            .subcommand(SubCommand::with_name("subcmd")
                                    .about("tests subcommands")
                                    .version("0.1")
                                    .author("Kevin K. <kbknapp@gmail.com>")
                                    .arg_from_usage("-o --option [scoption]... 'tests options'")
                                    .arg_from_usage("[scpositional] 'tests positionals'"))
    }

    pub fn check_err_output(a: App, args: &str, out: &str, use_stderr: bool) {
        let res = a.get_matches_from_safe(args.split(' ').collect::<Vec<_>>());
        let re = Regex::new("\x1b[^m]*m").unwrap();

        let mut w = vec![];
        let err = res.unwrap_err();
        err.write_to(&mut w).unwrap();
        let err_s = str::from_utf8(&w).unwrap();
        assert_eq!(re.replace_all(err_s, ""), out);
        assert_eq!(use_stderr, err.use_stderr());
    }

    pub fn check_help(mut a: App, out: &str) {
        // We call a get_matches method to cause --help and --version to be built
        let _ = a.get_matches_from_safe_borrow(vec![""]);

        // Now we check the output of print_help()
        let mut help = vec![];
        a.write_help(&mut help).ok().expect("failed to print help");
        assert_eq!(str::from_utf8(&help).unwrap(), out);
    }

    pub fn check_version(mut a: App, out: &str) {
        // We call a get_matches method to cause --help and --version to be built
        let _ = a.get_matches_from_safe_borrow(vec![""]);

        // Now we check the output of print_version()
        let mut ver = vec![];
        a.write_version(&mut ver).ok().expect("failed to print help");
        assert_eq!(str::from_utf8(&ver).unwrap(), out);
    }

    pub fn check_complex_output(args: &str, out: &str) {
        let mut w = vec![];
        let matches = complex_app().get_matches_from(args.split(' ').collect::<Vec<_>>());
        if matches.is_present("flag") {
            writeln!(w, "flag present {} times", matches.occurrences_of("flag")).unwrap();
        } else {
            writeln!(w, "flag NOT present").unwrap();
        }

        if matches.is_present("option") {
            if let Some(v) = matches.value_of("option") {
                writeln!(w, "option present {} times with value: {}",matches.occurrences_of("option"), v).unwrap();
            }
            if let Some(ov) = matches.values_of("option") {
                for o in ov {
                    writeln!(w, "An option: {}", o).unwrap();
                }
            }
        } else {
            writeln!(w, "option NOT present").unwrap();
        }

        if let Some(p) = matches.value_of("positional") {
            writeln!(w, "positional present with value: {}", p).unwrap();
        } else {
            writeln!(w, "positional NOT present").unwrap();
        }

        if matches.is_present("flag2") {
            writeln!(w, "flag2 present").unwrap();
            writeln!(w, "option2 present with value of: {}", matches.value_of("long-option-2").unwrap()).unwrap();
            writeln!(w, "positional2 present with value of: {}", matches.value_of("positional2").unwrap()).unwrap();
        } else {
            writeln!(w, "flag2 NOT present").unwrap();
            writeln!(w, "option2 maybe present with value of: {}", matches.value_of("long-option-2").unwrap_or("Nothing")).unwrap();
            writeln!(w, "positional2 maybe present with value of: {}", matches.value_of("positional2").unwrap_or("Nothing")).unwrap();
        }

        let _ = match matches.value_of("Option3").unwrap_or("") {
            "fast" => writeln!(w, "option3 present quickly"),
            "slow" => writeln!(w, "option3 present slowly"),
            _      => writeln!(w, "option3 NOT present")
        };

        let _ = match matches.value_of("positional3").unwrap_or("") {
            "vi" => writeln!(w, "positional3 present in vi mode"),
            "emacs" => writeln!(w, "positional3 present in emacs mode"),
            _      => writeln!(w, "positional3 NOT present")
        };

        if matches.is_present("option") {
            if let Some(v) = matches.value_of("option") {
                writeln!(w, "option present {} times with value: {}",matches.occurrences_of("option"), v).unwrap();
            }
            if let Some(ov) = matches.values_of("option") {
                for o in ov {
                    writeln!(w, "An option: {}", o).unwrap();
                }
            }
        } else {
            writeln!(w, "option NOT present").unwrap();
        }

        if let Some(p) = matches.value_of("positional") {
            writeln!(w, "positional present with value: {}", p).unwrap();
        } else {
            writeln!(w, "positional NOT present").unwrap();
        }
        if matches.is_present("subcmd") {
            writeln!(w, "subcmd present").unwrap();
            if let Some(matches) = matches.subcommand_matches("subcmd") {
                if matches.is_present("flag") {
                    writeln!(w, "flag present {} times", matches.occurrences_of("flag")).unwrap();
                } else {
                    writeln!(w, "flag NOT present").unwrap();
                }

                if matches.is_present("option") {
                    if let Some(v) = matches.value_of("option") {
                        writeln!(w, "scoption present with value: {}", v).unwrap();
                    }
                    if let Some(ov) = matches.values_of("option") {
                        for o in ov {
                            writeln!(w, "An scoption: {}", o).unwrap();
                        }
                    }
                } else {
                    writeln!(w, "scoption NOT present").unwrap();
                }

                if let Some(p) = matches.value_of("scpositional") {
                    writeln!(w, "scpositional present with value: {}", p).unwrap();
                }
            }
        } else {
            writeln!(w, "subcmd NOT present").unwrap();
        }

        let res = str::from_utf8(&w).unwrap();
        assert_eq!(res, out);
    }

}
