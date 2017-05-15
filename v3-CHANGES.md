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