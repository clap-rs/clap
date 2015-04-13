#![crate_type= "lib"]

// DOCS

pub use args::{Arg, SubCommand, ArgMatches};
pub use app::App;

#[macro_use]
mod macros;
mod app;
mod args;
mod usageparser;

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
	fn create_flag_usage() {
		let a = Arg::from_usage("[flag] -f 'some help info'");
		assert_eq!(a.name, "flag");
		assert_eq!(a.short.unwrap(), 'f');
		assert!(a.long.is_none());
		assert_eq!(a.help.unwrap(), "some help info");
		assert!(!a.multiple);

		let b = Arg::from_usage("[flag] --flag 'some help info'");
		assert_eq!(b.name, "flag");
		assert_eq!(b.long.unwrap(), "flag");
		assert!(b.short.is_none());
		assert_eq!(b.help.unwrap(), "some help info");
		assert!(!b.multiple);

		let b = Arg::from_usage("--flag 'some help info'");
		assert_eq!(b.name, "flag");
		assert_eq!(b.long.unwrap(), "flag");
		assert!(b.short.is_none());
		assert_eq!(b.help.unwrap(), "some help info");
		assert!(!b.multiple);

		let c = Arg::from_usage("[flag] -f --flag 'some help info'");
		assert_eq!(c.name, "flag");
		assert_eq!(c.short.unwrap(), 'f');
		assert_eq!(c.long.unwrap(), "flag");
		assert_eq!(c.help.unwrap(), "some help info");
		assert!(!c.multiple);

		let d = Arg::from_usage("[flag] -f... 'some help info'");
		assert_eq!(d.name, "flag");
		assert_eq!(d.short.unwrap(), 'f');
		assert!(d.long.is_none());
		assert_eq!(d.help.unwrap(), "some help info");
		assert!(d.multiple);

		let e = Arg::from_usage("[flag] -f --flag... 'some help info'");
		assert_eq!(e.name, "flag");
		assert_eq!(e.long.unwrap(), "flag");
		assert_eq!(e.short.unwrap(), 'f');
		assert_eq!(e.help.unwrap(), "some help info");
		assert!(e.multiple);

		let e = Arg::from_usage("-f --flag... 'some help info'");
		assert_eq!(e.name, "flag");
		assert_eq!(e.long.unwrap(), "flag");
		assert_eq!(e.short.unwrap(), 'f');
		assert_eq!(e.help.unwrap(), "some help info");
		assert!(e.multiple);
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
	fn create_positional_usage() {
		let a = Arg::from_usage("[pos] 'some help info'");
		assert_eq!(a.name, "pos");
		assert_eq!(a.help.unwrap(), "some help info");
		assert!(!a.multiple);
		assert!(!a.required);

		let b = Arg::from_usage("<pos> 'some help info'");
		assert_eq!(b.name, "pos");
		assert_eq!(b.help.unwrap(), "some help info");
		assert!(!b.multiple);
		assert!(b.required);

		let c = Arg::from_usage("[pos]... 'some help info'");
		assert_eq!(c.name, "pos");
		assert_eq!(c.help.unwrap(), "some help info");
		assert!(c.multiple);
		assert!(!c.required);

		let d = Arg::from_usage("<pos>... 'some help info'");
		assert_eq!(d.name, "pos");
		assert_eq!(d.help.unwrap(), "some help info");
		assert!(d.multiple);
		assert!(d.required);
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
	fn create_option_usage() {
		// Short only
		let a = Arg::from_usage("[option] -o [opt] 'some help info'");
		assert_eq!(a.name, "option");
		assert_eq!(a.short.unwrap(), 'o');
		assert!(a.long.is_none());
		assert_eq!(a.help.unwrap(), "some help info");
		assert!(!a.multiple);
		assert!(a.takes_value);
		assert!(!a.required);

		let b = Arg::from_usage("-o [opt] 'some help info'");
		assert_eq!(b.name, "opt");
		assert_eq!(b.short.unwrap(), 'o');
		assert!(b.long.is_none());
		assert_eq!(b.help.unwrap(), "some help info");
		assert!(!b.multiple);
		assert!(b.takes_value);
		assert!(!b.required);

		let c = Arg::from_usage("<option> -o <opt> 'some help info'");
		assert_eq!(c.name, "option");
		assert_eq!(c.short.unwrap(), 'o');
		assert!(c.long.is_none());
		assert_eq!(c.help.unwrap(), "some help info");
		assert!(!c.multiple);
		assert!(c.takes_value);
		assert!(c.required);

		let d = Arg::from_usage("-o <opt> 'some help info'");
		assert_eq!(d.name, "opt");
		assert_eq!(d.short.unwrap(), 'o');
		assert!(d.long.is_none());
		assert_eq!(d.help.unwrap(), "some help info");
		assert!(!d.multiple);
		assert!(d.takes_value);
		assert!(d.required);

		let a = Arg::from_usage("[option] -o [opt]... 'some help info'");
		assert_eq!(a.name, "option");
		assert_eq!(a.short.unwrap(), 'o');
		assert!(a.long.is_none());
		assert_eq!(a.help.unwrap(), "some help info");
		assert!(a.multiple);
		assert!(a.takes_value);
		assert!(!a.required);

		let a = Arg::from_usage("[option]... -o [opt] 'some help info'");
		assert_eq!(a.name, "option");
		assert_eq!(a.short.unwrap(), 'o');
		assert!(a.long.is_none());
		assert_eq!(a.help.unwrap(), "some help info");
		assert!(a.multiple);
		assert!(a.takes_value);
		assert!(!a.required);

		let b = Arg::from_usage("-o [opt]... 'some help info'");
		assert_eq!(b.name, "opt");
		assert_eq!(b.short.unwrap(), 'o');
		assert!(b.long.is_none());
		assert_eq!(b.help.unwrap(), "some help info");
		assert!(b.multiple);
		assert!(b.takes_value);
		assert!(!b.required);

		let c = Arg::from_usage("<option> -o <opt>... 'some help info'");
		assert_eq!(c.name, "option");
		assert_eq!(c.short.unwrap(), 'o');
		assert!(c.long.is_none());
		assert_eq!(c.help.unwrap(), "some help info");
		assert!(c.multiple);
		assert!(c.takes_value);
		assert!(c.required);

		let c = Arg::from_usage("<option>... -o <opt> 'some help info'");
		assert_eq!(c.name, "option");
		assert_eq!(c.short.unwrap(), 'o');
		assert!(c.long.is_none());
		assert_eq!(c.help.unwrap(), "some help info");
		assert!(c.multiple);
		assert!(c.takes_value);
		assert!(c.required);

		let d = Arg::from_usage("-o <opt>... 'some help info'");
		assert_eq!(d.name, "opt");
		assert_eq!(d.short.unwrap(), 'o');
		assert!(d.long.is_none());
		assert_eq!(d.help.unwrap(), "some help info");
		assert!(d.multiple);
		assert!(d.takes_value);
		assert!(d.required);

		// Long only

		let a = Arg::from_usage("[option] --opt [opt] 'some help info'");
		assert_eq!(a.name, "option");
		assert_eq!(a.long.unwrap(), "opt");
		assert!(a.short.is_none());
		assert_eq!(a.help.unwrap(), "some help info");
		assert!(!a.multiple);
		assert!(a.takes_value);
		assert!(!a.required);

		let b = Arg::from_usage("--opt [option] 'some help info'");
		assert_eq!(b.name, "option");
		assert_eq!(b.long.unwrap(), "opt");
		assert!(b.short.is_none());
		assert_eq!(b.help.unwrap(), "some help info");
		assert!(!b.multiple);
		assert!(b.takes_value);
		assert!(!b.required);

		let c = Arg::from_usage("<option> --opt <opt> 'some help info'");
		assert_eq!(c.name, "option");
		assert_eq!(c.long.unwrap(), "opt");
		assert!(c.short.is_none());
		assert_eq!(c.help.unwrap(), "some help info");
		assert!(!c.multiple);
		assert!(c.takes_value);
		assert!(c.required);

		let d = Arg::from_usage("--opt <option> 'some help info'");
		assert_eq!(d.name, "option");
		assert_eq!(d.long.unwrap(), "opt");
		assert!(d.short.is_none());
		assert_eq!(d.help.unwrap(), "some help info");
		assert!(!d.multiple);
		assert!(d.takes_value);
		assert!(d.required);

		let a = Arg::from_usage("[option] --opt [opt]... 'some help info'");
		assert_eq!(a.name, "option");
		assert_eq!(a.long.unwrap(), "opt");
		assert!(a.short.is_none());
		assert_eq!(a.help.unwrap(), "some help info");
		assert!(a.multiple);
		assert!(a.takes_value);
		assert!(!a.required);

		let a = Arg::from_usage("[option]... --opt [opt] 'some help info'");
		assert_eq!(a.name, "option");
		assert_eq!(a.long.unwrap(), "opt");
		assert!(a.short.is_none());
		assert_eq!(a.help.unwrap(), "some help info");
		assert!(a.multiple);
		assert!(a.takes_value);
		assert!(!a.required);

		let b = Arg::from_usage("--opt [option]... 'some help info'");
		assert_eq!(b.name, "option");
		assert_eq!(b.long.unwrap(), "opt");
		assert!(b.short.is_none());
		assert_eq!(b.help.unwrap(), "some help info");
		assert!(b.multiple);
		assert!(b.takes_value);
		assert!(!b.required);

		let c = Arg::from_usage("<option> --opt <opt>... 'some help info'");
		assert_eq!(c.name, "option");
		assert_eq!(c.long.unwrap(), "opt");
		assert!(c.short.is_none());
		assert_eq!(c.help.unwrap(), "some help info");
		assert!(c.multiple);
		assert!(c.takes_value);
		assert!(c.required);

		let c = Arg::from_usage("<option>... --opt <opt> 'some help info'");
		assert_eq!(c.name, "option");
		assert_eq!(c.long.unwrap(), "opt");
		assert!(c.short.is_none());
		assert_eq!(c.help.unwrap(), "some help info");
		assert!(c.multiple);
		assert!(c.takes_value);
		assert!(c.required);

		let d = Arg::from_usage("--opt <option>... 'some help info'");
		assert_eq!(d.name, "option");
		assert_eq!(d.long.unwrap(), "opt");
		assert!(d.short.is_none());
		assert_eq!(d.help.unwrap(), "some help info");
		assert!(d.multiple);
		assert!(d.takes_value);
		assert!(d.required);

		// Long only with '='

		let a = Arg::from_usage("[option] --opt=[opt] 'some help info'");
		assert_eq!(a.name, "option");
		assert_eq!(a.long.unwrap(), "opt");
		assert!(a.short.is_none());
		assert_eq!(a.help.unwrap(), "some help info");
		assert!(!a.multiple);
		assert!(a.takes_value);
		assert!(!a.required);

		let b = Arg::from_usage("--opt=[option] 'some help info'");
		assert_eq!(b.name, "option");
		assert_eq!(b.long.unwrap(), "opt");
		assert!(b.short.is_none());
		assert_eq!(b.help.unwrap(), "some help info");
		assert!(!b.multiple);
		assert!(b.takes_value);
		assert!(!b.required);

		let c = Arg::from_usage("<option> --opt=<opt> 'some help info'");
		assert_eq!(c.name, "option");
		assert_eq!(c.long.unwrap(), "opt");
		assert!(c.short.is_none());
		assert_eq!(c.help.unwrap(), "some help info");
		assert!(!c.multiple);
		assert!(c.takes_value);
		assert!(c.required);

		let d = Arg::from_usage("--opt=<option> 'some help info'");
		assert_eq!(d.name, "option");
		assert_eq!(d.long.unwrap(), "opt");
		assert!(d.short.is_none());
		assert_eq!(d.help.unwrap(), "some help info");
		assert!(!d.multiple);
		assert!(d.takes_value);
		assert!(d.required);

		let a = Arg::from_usage("[option] --opt=[opt]... 'some help info'");
		assert_eq!(a.name, "option");
		assert_eq!(a.long.unwrap(), "opt");
		assert!(a.short.is_none());
		assert_eq!(a.help.unwrap(), "some help info");
		assert!(a.multiple);
		assert!(a.takes_value);
		assert!(!a.required);

		let a = Arg::from_usage("[option]... --opt=[opt] 'some help info'");
		assert_eq!(a.name, "option");
		assert_eq!(a.long.unwrap(), "opt");
		assert!(a.short.is_none());
		assert_eq!(a.help.unwrap(), "some help info");
		assert!(a.multiple);
		assert!(a.takes_value);
		assert!(!a.required);

		let b = Arg::from_usage("--opt=[option]... 'some help info'");
		assert_eq!(b.name, "option");
		assert_eq!(b.long.unwrap(), "opt");
		assert!(b.short.is_none());
		assert_eq!(b.help.unwrap(), "some help info");
		assert!(b.multiple);
		assert!(b.takes_value);
		assert!(!b.required);

		let c = Arg::from_usage("<option> --opt=<opt>... 'some help info'");
		assert_eq!(c.name, "option");
		assert_eq!(c.long.unwrap(), "opt");
		assert!(c.short.is_none());
		assert_eq!(c.help.unwrap(), "some help info");
		assert!(c.multiple);
		assert!(c.takes_value);
		assert!(c.required);

		let c = Arg::from_usage("<option>... --opt=<opt> 'some help info'");
		assert_eq!(c.name, "option");
		assert_eq!(c.long.unwrap(), "opt");
		assert!(c.short.is_none());
		assert_eq!(c.help.unwrap(), "some help info");
		assert!(c.multiple);
		assert!(c.takes_value);
		assert!(c.required);

		let d = Arg::from_usage("--opt=<option>... 'some help info'");
		assert_eq!(d.name, "option");
		assert_eq!(d.long.unwrap(), "opt");
		assert!(d.short.is_none());
		assert_eq!(d.help.unwrap(), "some help info");
		assert!(d.multiple);
		assert!(d.takes_value);
		assert!(d.required);

		// Long and Short

		let a = Arg::from_usage("[option] -o --opt [option] 'some help info'");
		assert_eq!(a.name, "option");
		assert_eq!(a.long.unwrap(), "opt");
		assert_eq!(a.short.unwrap(), 'o');
		assert_eq!(a.help.unwrap(), "some help info");
		assert!(!a.multiple);
		assert!(a.takes_value);
		assert!(!a.required);

		let b = Arg::from_usage("-o --opt [option] 'some help info'");
		assert_eq!(b.name, "option");
		assert_eq!(a.long.unwrap(), "opt");
		assert_eq!(b.short.unwrap(), 'o');
		assert_eq!(b.help.unwrap(), "some help info");
		assert!(!b.multiple);
		assert!(b.takes_value);
		assert!(!b.required);

		let c = Arg::from_usage("<option> -o --opt <opt> 'some help info'");
		assert_eq!(c.name, "option");
		assert_eq!(a.long.unwrap(), "opt");
		assert_eq!(a.short.unwrap(), 'o');
		assert_eq!(c.help.unwrap(), "some help info");
		assert!(!c.multiple);
		assert!(c.takes_value);
		assert!(c.required);

		let d = Arg::from_usage("-o --opt <option> 'some help info'");
		assert_eq!(d.name, "option");
		assert_eq!(a.long.unwrap(), "opt");
		assert_eq!(a.short.unwrap(), 'o');
		assert_eq!(d.help.unwrap(), "some help info");
		assert!(!d.multiple);
		assert!(d.takes_value);
		assert!(d.required);

		let a = Arg::from_usage("[option]... -o --opt [option] 'some help info'");
		assert_eq!(a.name, "option");
		assert_eq!(a.long.unwrap(), "opt");
		assert_eq!(a.short.unwrap(), 'o');
		assert_eq!(a.help.unwrap(), "some help info");
		assert!(a.multiple);
		assert!(a.takes_value);
		assert!(!a.required);

		let b = Arg::from_usage("-o --opt [option]... 'some help info'");
		assert_eq!(b.name, "option");
		assert_eq!(a.long.unwrap(), "opt");
		assert_eq!(b.short.unwrap(), 'o');
		assert_eq!(b.help.unwrap(), "some help info");
		assert!(b.multiple);
		assert!(b.takes_value);
		assert!(!b.required);

		let c = Arg::from_usage("<option>... -o --opt <opt> 'some help info'");
		assert_eq!(c.name, "option");
		assert_eq!(a.long.unwrap(), "opt");
		assert_eq!(a.short.unwrap(), 'o');
		assert_eq!(c.help.unwrap(), "some help info");
		assert!(c.multiple);
		assert!(c.takes_value);
		assert!(c.required);

		let d = Arg::from_usage("-o --opt <option>... 'some help info'");
		assert_eq!(d.name, "option");
		assert_eq!(a.long.unwrap(), "opt");
		assert_eq!(a.short.unwrap(), 'o');
		assert_eq!(d.help.unwrap(), "some help info");
		assert!(d.multiple);
		assert!(d.takes_value);
		assert!(d.required);

		// Long and Short with '='

		let a = Arg::from_usage("[option] -o --opt=[option] 'some help info'");
		assert_eq!(a.name, "option");
		assert_eq!(a.long.unwrap(), "opt");
		assert_eq!(a.short.unwrap(), 'o');
		assert_eq!(a.help.unwrap(), "some help info");
		assert!(!a.multiple);
		assert!(a.takes_value);
		assert!(!a.required);

		let b = Arg::from_usage("-o --opt=[option] 'some help info'");
		assert_eq!(b.name, "option");
		assert_eq!(a.long.unwrap(), "opt");
		assert_eq!(b.short.unwrap(), 'o');
		assert_eq!(b.help.unwrap(), "some help info");
		assert!(!b.multiple);
		assert!(b.takes_value);
		assert!(!b.required);

		let c = Arg::from_usage("<option> -o --opt=<opt> 'some help info'");
		assert_eq!(c.name, "option");
		assert_eq!(a.long.unwrap(), "opt");
		assert_eq!(a.short.unwrap(), 'o');
		assert_eq!(c.help.unwrap(), "some help info");
		assert!(!c.multiple);
		assert!(c.takes_value);
		assert!(c.required);

		let d = Arg::from_usage("-o --opt=<option> 'some help info'");
		assert_eq!(d.name, "option");
		assert_eq!(a.long.unwrap(), "opt");
		assert_eq!(a.short.unwrap(), 'o');
		assert_eq!(d.help.unwrap(), "some help info");
		assert!(!d.multiple);
		assert!(d.takes_value);
		assert!(d.required);

		let a = Arg::from_usage("[option]... -o --opt=[option] 'some help info'");
		assert_eq!(a.name, "option");
		assert_eq!(a.long.unwrap(), "opt");
		assert_eq!(a.short.unwrap(), 'o');
		assert_eq!(a.help.unwrap(), "some help info");
		assert!(a.multiple);
		assert!(a.takes_value);
		assert!(!a.required);

		let b = Arg::from_usage("-o --opt=[option]... 'some help info'");
		assert_eq!(b.name, "option");
		assert_eq!(a.long.unwrap(), "opt");
		assert_eq!(b.short.unwrap(), 'o');
		assert_eq!(b.help.unwrap(), "some help info");
		assert!(b.multiple);
		assert!(b.takes_value);
		assert!(!b.required);

		let c = Arg::from_usage("<option>... -o --opt=<opt> 'some help info'");
		assert_eq!(c.name, "option");
		assert_eq!(a.long.unwrap(), "opt");
		assert_eq!(a.short.unwrap(), 'o');
		assert_eq!(c.help.unwrap(), "some help info");
		assert!(c.multiple);
		assert!(c.takes_value);
		assert!(c.required);

		let d = Arg::from_usage("-o --opt=<option>... 'some help info'");
		assert_eq!(d.name, "option");
		assert_eq!(a.long.unwrap(), "opt");
		assert_eq!(a.short.unwrap(), 'o');
		assert_eq!(d.help.unwrap(), "some help info");
		assert!(d.multiple);
		assert!(d.takes_value);
		assert!(d.required);
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