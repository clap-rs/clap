#![crate_type= "lib"]

// DOCS

pub use args::{Arg, SubCommand, ArgMatches};
pub use app::App;

mod app;
mod args;

#[cfg(test)]
mod tests {
    use super::{App, Arg, SubCommand};

    #[test]
	fn create_app() {
	    let _ = App::new("test").version("1.0").author("kevin").about("does awesome things").get_matches();
	}

	#[test]
	fn add_multiple_arg() {
	    let _ = App::new("test")
	                .args( vec![
	                    Arg::new("test").short("s"),
	                    Arg::new("test2").short("l")])
	                .get_matches();
	}

	#[test]
	fn create_flag() {
	    let _ = App::new("test")
	                .arg(Arg::new("test")
	                            .short("t")
	                            .long("test")
	                            .help("testing testing"))
	                .get_matches();
	}

	#[test]
	fn create_positional() {
	    let _ = App::new("test")
	                .arg(Arg::new("test")
	                            .index(1)
	                            .help("testing testing"))
	                .get_matches();
	}

	#[test]
	fn create_option() {
	    let _ = App::new("test")
	                .arg(Arg::new("test")
	                            .short("t")
	                            .long("test")
	                            .takes_value(true)
	                            .help("testing testing"))
	                .get_matches();
	}

	#[test]
	fn create_subcommand() {
	    let _ = App::new("test")
	                .subcommand(SubCommand::new("some")
	                                        .arg(Arg::new("test")
	                                            .short("t")
	                                            .long("test")
	                                            .takes_value(true)
	                                            .help("testing testing")))
	                .arg(Arg::new("other").long("other"))
	                .get_matches();
	}

	#[test]
	fn create_multiple_subcommands() {
	    let _ = App::new("test")
	                .subcommands(vec![ SubCommand::new("some")
	                                        .arg(Arg::new("test")
	                                            .short("t")
	                                            .long("test")
	                                            .takes_value(true)
	                                            .help("testing testing")),
	                                    SubCommand::new("add")
	                                        .arg(Arg::new("roster").short("r"))])
	                .arg(Arg::new("other").long("other"))
	                .get_matches();
	}

	#[test]
	#[should_panic]
	fn unique_arg_names(){
	    App::new("some").args(vec![
	        Arg::new("arg").short("a"),
	        Arg::new("arg").short("b")
	    ]);
	}

	#[test]
	#[should_panic]
	fn unique_arg_shorts(){
	    App::new("some").args(vec![
	        Arg::new("arg1").short("a"),
	        Arg::new("arg2").short("a")
	    ]);
	}

	#[test]
	#[should_panic]
	fn unique_arg_longs(){
	    App::new("some").args(vec![
	        Arg::new("arg1").long("long"),
	        Arg::new("arg2").long("long")
	    ]);
	}
}