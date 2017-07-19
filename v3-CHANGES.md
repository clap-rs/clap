* App::with_defaults removed
* App::from_yaml deprecated/removed (-> serde or App::from)
* App::help_short -> App::mut_arg("help", |a| a.short("H"))
* App::version_short -> App::mut_arg("version", |a| a.short("v"))
* App::help_message -> App::mut_arg("help", |a| a.help("some message"))
* App::version_message -> App::mut_arg("version", |a| a.help("some message"))
* App::arg_from_usage -> App::arg("-v, --version 'some message'")
* App::args_from_usage -> App::args(["-v, --version=[ver]", "-other 'something'"])
* App::write_help takes &mut self
* App::help -> App::override_help
* App::usage -> App::override_usage
* App::get_bin_name -> clap::utils::get_bin_name (now function)
* Arg::with_name -> Arg::new
* Arg::from_usage -> Arg::from
* Arg::from_yaml -> Arg::from
* Arg::*(bool) -> Arg::set(*)
* Restructure Mods
* App::gen_completions -> clap::completions::generate
* App::gen_completions_to -> clap::completions::generate_to
* Mod Structure
* get_matches_from_safe_borrow -> get_matches_from_safe_mut
* App::get_matches_from_mut
* App::get_matches_safe_mut
* clap::utils
* Completions to clap_completions crate
* ArgSettings::AllowLeadingHyphen -> ArgSettings::AllowHyphenValues
* ArgGroup::with_name -> ArgGroup::new
* Arg u64 -> Arg usize
* SubCommand::with_name -> App::new