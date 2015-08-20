#![crate_type= "lib"]
#![cfg_attr(feature = "lints", feature(plugin))]
#![cfg_attr(feature = "lints", plugin(clippy))]
#![cfg_attr(feature = "lints", allow(option_unwrap_used))]
#![cfg_attr(feature = "lints", allow(explicit_iter_loop))]
#![cfg_attr(feature = "lints", deny(warnings))]
// Fix until clippy on crates.io is updated to include needless_lifetimes lint
#![cfg_attr(feature = "lints", allow(unknown_lints))]

// DOCS

#[cfg(feature = "suggestions")]
extern crate strsim;
#[cfg(feature = "color")]
extern crate ansi_term;

pub use args::{Arg, SubCommand, ArgMatches, ArgGroup};
pub use app::{App, AppSettings};
pub use fmt::Format;

#[macro_use]
mod macros;
mod app;
mod args;
mod usageparser;
mod fmt;

#[cfg(test)]
mod tests {
    use super::{App, Arg, SubCommand};
    use std::collections::HashSet;
    use std::vec::Vec;

    arg_enum!{
        #[derive(Debug)]
        enum Val1 {
            ValOne,
            ValTwo
        }
    }
    arg_enum!{
        #[derive(Debug)]
        pub enum Val2 {
            ValOne,
            ValTwo
        }
    }
    arg_enum!{
        enum Val3 {
            ValOne,
            ValTwo
        }
    }
    arg_enum!{
        pub enum Val4 {
            ValOne,
            ValTwo
        }
    }

    #[test]
    #[cfg_attr(feature = "lints", allow(single_match))]
    fn test_enums() {
        let v1_lower = "valone";
        let v1_camel = "ValOne";

        let v1_lp = v1_lower.parse::<Val1>().unwrap();
        let v1_cp = v1_camel.parse::<Val1>().unwrap();
        match v1_lp {
            Val1::ValOne => (),
            _ => panic!("Val1 didn't parse correctly"),
        }
        match v1_cp {
            Val1::ValOne => (),
            _ => panic!("Val1 didn't parse correctly"),
        }
        let v1_lp = v1_lower.parse::<Val2>().unwrap();
        let v1_cp = v1_camel.parse::<Val2>().unwrap();
        match v1_lp {
            Val2::ValOne => (),
            _ => panic!("Val1 didn't parse correctly"),
        }
        match v1_cp {
            Val2::ValOne => (),
            _ => panic!("Val1 didn't parse correctly"),
        }
        let v1_lp = v1_lower.parse::<Val3>().unwrap();
        let v1_cp = v1_camel.parse::<Val3>().unwrap();
        match v1_lp {
            Val3::ValOne => (),
            _ => panic!("Val1 didn't parse correctly"),
        }
        match v1_cp {
            Val3::ValOne => (),
            _ => panic!("Val1 didn't parse correctly"),
        }
        let v1_lp = v1_lower.parse::<Val4>().unwrap();
        let v1_cp = v1_camel.parse::<Val4>().unwrap();
        match v1_lp {
            Val4::ValOne => (),
            _ => panic!("Val1 didn't parse correctly"),
        }
        match v1_cp {
            Val4::ValOne => (),
            _ => panic!("Val1 didn't parse correctly"),
        }
    }

    #[test]
	fn create_app() {
        let _ = App::new("test").version("1.0").author("kevin").about("does awesome things").get_matches();
    }

    #[test]
	fn add_multiple_arg() {
        let _ = App::new("test")
	                .args( vec![
	                    Arg::with_name("test").short("s"),
	                    Arg::with_name("test2").short("l")])
	                .get_matches();
    }

    #[test]
	fn create_flag() {
        let _ = App::new("test")
	                .arg(Arg::with_name("test")
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
        assert!(a.val_names.is_none());
        assert!(a.num_vals.is_none());

        let b = Arg::from_usage("[flag] --flag 'some help info'");
        assert_eq!(b.name, "flag");
        assert_eq!(b.long.unwrap(), "flag");
        assert!(b.short.is_none());
        assert_eq!(b.help.unwrap(), "some help info");
        assert!(!b.multiple);
        assert!(a.val_names.is_none());
        assert!(a.num_vals.is_none());

        let b = Arg::from_usage("--flag 'some help info'");
        assert_eq!(b.name, "flag");
        assert_eq!(b.long.unwrap(), "flag");
        assert!(b.short.is_none());
        assert_eq!(b.help.unwrap(), "some help info");
        assert!(!b.multiple);
        assert!(b.val_names.is_none());
        assert!(b.num_vals.is_none());

        let c = Arg::from_usage("[flag] -f --flag 'some help info'");
        assert_eq!(c.name, "flag");
        assert_eq!(c.short.unwrap(), 'f');
        assert_eq!(c.long.unwrap(), "flag");
        assert_eq!(c.help.unwrap(), "some help info");
        assert!(!c.multiple);
        assert!(c.val_names.is_none());
        assert!(c.num_vals.is_none());

        let d = Arg::from_usage("[flag] -f... 'some help info'");
        assert_eq!(d.name, "flag");
        assert_eq!(d.short.unwrap(), 'f');
        assert!(d.long.is_none());
        assert_eq!(d.help.unwrap(), "some help info");
        assert!(d.multiple);
        assert!(d.val_names.is_none());
        assert!(d.num_vals.is_none());

        let e = Arg::from_usage("[flag] -f --flag... 'some help info'");
        assert_eq!(e.name, "flag");
        assert_eq!(e.long.unwrap(), "flag");
        assert_eq!(e.short.unwrap(), 'f');
        assert_eq!(e.help.unwrap(), "some help info");
        assert!(e.multiple);
        assert!(e.val_names.is_none());
        assert!(e.num_vals.is_none());

        let e = Arg::from_usage("-f --flag... 'some help info'");
        assert_eq!(e.name, "flag");
        assert_eq!(e.long.unwrap(), "flag");
        assert_eq!(e.short.unwrap(), 'f');
        assert_eq!(e.help.unwrap(), "some help info");
        assert!(e.multiple);
        assert!(e.val_names.is_none());
        assert!(e.num_vals.is_none());

        let e = Arg::from_usage("--flags");
        assert_eq!(e.name, "flags");
        assert_eq!(e.long.unwrap(), "flags");
        assert!(e.val_names.is_none());
        assert!(e.num_vals.is_none());

        let e = Arg::from_usage("--flags...");
        assert_eq!(e.name, "flags");
        assert_eq!(e.long.unwrap(), "flags");
        assert!(e.multiple);
        assert!(e.val_names.is_none());
        assert!(e.num_vals.is_none());

        let e = Arg::from_usage("[flags] -f");
        assert_eq!(e.name, "flags");
        assert_eq!(e.short.unwrap(), 'f');
        assert!(e.val_names.is_none());
        assert!(e.num_vals.is_none());

        let e = Arg::from_usage("[flags] -f...");
        assert_eq!(e.name, "flags");
        assert_eq!(e.short.unwrap(), 'f');
        assert!(e.multiple);
        assert!(e.val_names.is_none());
        assert!(e.num_vals.is_none());
    }

    #[test]
	fn create_positional() {
        let _ = App::new("test")
	                .arg(Arg::with_name("test")
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
        assert!(a.val_names.is_none());
        assert!(a.num_vals.is_none());

        let b = Arg::from_usage("<pos> 'some help info'");
        assert_eq!(b.name, "pos");
        assert_eq!(b.help.unwrap(), "some help info");
        assert!(!b.multiple);
        assert!(b.required);
        assert!(b.val_names.is_none());
        assert!(b.num_vals.is_none());

        let c = Arg::from_usage("[pos]... 'some help info'");
        assert_eq!(c.name, "pos");
        assert_eq!(c.help.unwrap(), "some help info");
        assert!(c.multiple);
        assert!(!c.required);
        assert!(c.val_names.is_none());
        assert!(c.num_vals.is_none());

        let d = Arg::from_usage("<pos>... 'some help info'");
        assert_eq!(d.name, "pos");
        assert_eq!(d.help.unwrap(), "some help info");
        assert!(d.multiple);
        assert!(d.required);
        assert!(d.val_names.is_none());
        assert!(d.num_vals.is_none());

        let b = Arg::from_usage("<pos>");
        assert_eq!(b.name, "pos");
        assert!(!b.multiple);
        assert!(b.required);
        assert!(b.val_names.is_none());
        assert!(b.num_vals.is_none());

        let c = Arg::from_usage("[pos]...");
        assert_eq!(c.name, "pos");
        assert!(c.multiple);
        assert!(!c.required);
        assert!(c.val_names.is_none());
        assert!(c.num_vals.is_none());
    }

    #[test]
	fn create_args_tabs_usage() {
        let a = Arg::from_usage("[pos]\t'some help info'");
        assert_eq!(a.name, "pos");
        assert_eq!(a.help.unwrap(), "some help info");
        assert!(!a.multiple);
        assert!(!a.required);
        assert!(a.val_names.is_none());
        assert!(a.num_vals.is_none());

        let b = Arg::from_usage("<pos>\t'some help info'");
        assert_eq!(b.name, "pos");
        assert_eq!(b.help.unwrap(), "some help info");
        assert!(!b.multiple);
        assert!(b.required);
        assert!(b.val_names.is_none());
        assert!(b.num_vals.is_none());

        let c = Arg::from_usage("[pos]...\t'some help info'");
        assert_eq!(c.name, "pos");
        assert_eq!(c.help.unwrap(), "some help info");
        assert!(c.multiple);
        assert!(!c.required);
        assert!(c.val_names.is_none());
        assert!(c.num_vals.is_none());

        let d = Arg::from_usage("<pos>...\t'some help info'");
        assert_eq!(d.name, "pos");
        assert_eq!(d.help.unwrap(), "some help info");
        assert!(d.multiple);
        assert!(d.required);
        assert!(d.val_names.is_none());
        assert!(d.num_vals.is_none());
    }

    #[test]
	fn create_option() {
        let _ = App::new("test")
	                .arg(Arg::with_name("test")
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
        assert!(a.val_names.is_none());
        assert!(a.num_vals.is_none());

        let b = Arg::from_usage("-o [opt] 'some help info'");
        assert_eq!(b.name, "opt");
        assert_eq!(b.short.unwrap(), 'o');
        assert!(b.long.is_none());
        assert_eq!(b.help.unwrap(), "some help info");
        assert!(!b.multiple);
        assert!(b.takes_value);
        assert!(!b.required);
        assert!(b.val_names.is_none());
        assert!(b.num_vals.is_none());

        let c = Arg::from_usage("<option> -o <opt> 'some help info'");
        assert_eq!(c.name, "option");
        assert_eq!(c.short.unwrap(), 'o');
        assert!(c.long.is_none());
        assert_eq!(c.help.unwrap(), "some help info");
        assert!(!c.multiple);
        assert!(c.takes_value);
        assert!(c.required);
        assert!(c.val_names.is_none());
        assert!(c.num_vals.is_none());

        let d = Arg::from_usage("-o <opt> 'some help info'");
        assert_eq!(d.name, "opt");
        assert_eq!(d.short.unwrap(), 'o');
        assert!(d.long.is_none());
        assert_eq!(d.help.unwrap(), "some help info");
        assert!(!d.multiple);
        assert!(d.takes_value);
        assert!(d.required);
        assert!(d.val_names.is_none());
        assert!(d.num_vals.is_none());

        let a = Arg::from_usage("[option] -o [opt]... 'some help info'");
        assert_eq!(a.name, "option");
        assert_eq!(a.short.unwrap(), 'o');
        assert!(a.long.is_none());
        assert_eq!(a.help.unwrap(), "some help info");
        assert!(a.multiple);
        assert!(a.takes_value);
        assert!(!a.required);
        assert!(a.val_names.is_none());
        assert!(a.num_vals.is_none());

        let a = Arg::from_usage("[option]... -o [opt] 'some help info'");
        assert_eq!(a.name, "option");
        assert_eq!(a.short.unwrap(), 'o');
        assert!(a.long.is_none());
        assert_eq!(a.help.unwrap(), "some help info");
        assert!(a.multiple);
        assert!(a.takes_value);
        assert!(!a.required);
        assert!(a.val_names.is_none());
        assert!(a.num_vals.is_none());

        let b = Arg::from_usage("-o [opt]... 'some help info'");
        assert_eq!(b.name, "opt");
        assert_eq!(b.short.unwrap(), 'o');
        assert!(b.long.is_none());
        assert_eq!(b.help.unwrap(), "some help info");
        assert!(b.multiple);
        assert!(b.takes_value);
        assert!(!b.required);
        assert!(b.val_names.is_none());
        assert!(b.num_vals.is_none());

        let c = Arg::from_usage("<option> -o <opt>... 'some help info'");
        assert_eq!(c.name, "option");
        assert_eq!(c.short.unwrap(), 'o');
        assert!(c.long.is_none());
        assert_eq!(c.help.unwrap(), "some help info");
        assert!(c.multiple);
        assert!(c.takes_value);
        assert!(c.required);
        assert!(c.val_names.is_none());
        assert!(c.num_vals.is_none());

        let c = Arg::from_usage("<option>... -o <opt> 'some help info'");
        assert_eq!(c.name, "option");
        assert_eq!(c.short.unwrap(), 'o');
        assert!(c.long.is_none());
        assert_eq!(c.help.unwrap(), "some help info");
        assert!(c.multiple);
        assert!(c.takes_value);
        assert!(c.required);
        assert!(c.val_names.is_none());
        assert!(c.num_vals.is_none());

        let d = Arg::from_usage("-o <opt>... 'some help info'");
        assert_eq!(d.name, "opt");
        assert_eq!(d.short.unwrap(), 'o');
        assert!(d.long.is_none());
        assert_eq!(d.help.unwrap(), "some help info");
        assert!(d.multiple);
        assert!(d.takes_value);
        assert!(d.required);
        assert!(d.val_names.is_none());
        assert!(d.num_vals.is_none());

		// Long only

        let a = Arg::from_usage("[option] --opt [opt] 'some help info'");
        assert_eq!(a.name, "option");
        assert_eq!(a.long.unwrap(), "opt");
        assert!(a.short.is_none());
        assert_eq!(a.help.unwrap(), "some help info");
        assert!(!a.multiple);
        assert!(a.takes_value);
        assert!(!a.required);
        assert!(a.val_names.is_none());
        assert!(a.num_vals.is_none());

        let b = Arg::from_usage("--opt [option] 'some help info'");
        assert_eq!(b.name, "option");
        assert_eq!(b.long.unwrap(), "opt");
        assert!(b.short.is_none());
        assert_eq!(b.help.unwrap(), "some help info");
        assert!(!b.multiple);
        assert!(b.takes_value);
        assert!(!b.required);
        assert!(b.val_names.is_none());
        assert!(b.num_vals.is_none());

        let c = Arg::from_usage("<option> --opt <opt> 'some help info'");
        assert_eq!(c.name, "option");
        assert_eq!(c.long.unwrap(), "opt");
        assert!(c.short.is_none());
        assert_eq!(c.help.unwrap(), "some help info");
        assert!(!c.multiple);
        assert!(c.takes_value);
        assert!(c.required);
        assert!(c.val_names.is_none());
        assert!(c.num_vals.is_none());

        let d = Arg::from_usage("--opt <option> 'some help info'");
        assert_eq!(d.name, "option");
        assert_eq!(d.long.unwrap(), "opt");
        assert!(d.short.is_none());
        assert_eq!(d.help.unwrap(), "some help info");
        assert!(!d.multiple);
        assert!(d.takes_value);
        assert!(d.required);
        assert!(d.val_names.is_none());
        assert!(d.num_vals.is_none());

        let a = Arg::from_usage("[option] --opt [opt]... 'some help info'");
        assert_eq!(a.name, "option");
        assert_eq!(a.long.unwrap(), "opt");
        assert!(a.short.is_none());
        assert_eq!(a.help.unwrap(), "some help info");
        assert!(a.multiple);
        assert!(a.takes_value);
        assert!(!a.required);
        assert!(a.val_names.is_none());
        assert!(a.num_vals.is_none());

        let a = Arg::from_usage("[option]... --opt [opt] 'some help info'");
        assert_eq!(a.name, "option");
        assert_eq!(a.long.unwrap(), "opt");
        assert!(a.short.is_none());
        assert_eq!(a.help.unwrap(), "some help info");
        assert!(a.multiple);
        assert!(a.takes_value);
        assert!(!a.required);
        assert!(a.val_names.is_none());
        assert!(a.num_vals.is_none());

        let b = Arg::from_usage("--opt [option]... 'some help info'");
        assert_eq!(b.name, "option");
        assert_eq!(b.long.unwrap(), "opt");
        assert!(b.short.is_none());
        assert_eq!(b.help.unwrap(), "some help info");
        assert!(b.multiple);
        assert!(b.takes_value);
        assert!(!b.required);
        assert!(b.val_names.is_none());
        assert!(b.num_vals.is_none());

        let c = Arg::from_usage("<option> --opt <opt>... 'some help info'");
        assert_eq!(c.name, "option");
        assert_eq!(c.long.unwrap(), "opt");
        assert!(c.short.is_none());
        assert_eq!(c.help.unwrap(), "some help info");
        assert!(c.multiple);
        assert!(c.takes_value);
        assert!(c.required);
        assert!(c.val_names.is_none());
        assert!(c.num_vals.is_none());

        let c = Arg::from_usage("<option>... --opt <opt> 'some help info'");
        assert_eq!(c.name, "option");
        assert_eq!(c.long.unwrap(), "opt");
        assert!(c.short.is_none());
        assert_eq!(c.help.unwrap(), "some help info");
        assert!(c.multiple);
        assert!(c.takes_value);
        assert!(c.required);
        assert!(c.val_names.is_none());
        assert!(c.num_vals.is_none());

        let d = Arg::from_usage("--opt <option>... 'some help info'");
        assert_eq!(d.name, "option");
        assert_eq!(d.long.unwrap(), "opt");
        assert!(d.short.is_none());
        assert_eq!(d.help.unwrap(), "some help info");
        assert!(d.multiple);
        assert!(d.takes_value);
        assert!(d.required);
        assert!(d.val_names.is_none());
        assert!(d.num_vals.is_none());

		// Long only with '='

        let a = Arg::from_usage("[option] --opt=[opt] 'some help info'");
        assert_eq!(a.name, "option");
        assert_eq!(a.long.unwrap(), "opt");
        assert!(a.short.is_none());
        assert_eq!(a.help.unwrap(), "some help info");
        assert!(!a.multiple);
        assert!(a.takes_value);
        assert!(!a.required);
        assert!(a.val_names.is_none());
        assert!(a.num_vals.is_none());

        let b = Arg::from_usage("--opt=[option] 'some help info'");
        assert_eq!(b.name, "option");
        assert_eq!(b.long.unwrap(), "opt");
        assert!(b.short.is_none());
        assert_eq!(b.help.unwrap(), "some help info");
        assert!(!b.multiple);
        assert!(b.takes_value);
        assert!(!b.required);
        assert!(b.val_names.is_none());
        assert!(b.num_vals.is_none());

        let c = Arg::from_usage("<option> --opt=<opt> 'some help info'");
        assert_eq!(c.name, "option");
        assert_eq!(c.long.unwrap(), "opt");
        assert!(c.short.is_none());
        assert_eq!(c.help.unwrap(), "some help info");
        assert!(!c.multiple);
        assert!(c.takes_value);
        assert!(c.required);
        assert!(c.val_names.is_none());
        assert!(c.num_vals.is_none());

        let d = Arg::from_usage("--opt=<option> 'some help info'");
        assert_eq!(d.name, "option");
        assert_eq!(d.long.unwrap(), "opt");
        assert!(d.short.is_none());
        assert_eq!(d.help.unwrap(), "some help info");
        assert!(!d.multiple);
        assert!(d.takes_value);
        assert!(d.required);
        assert!(d.val_names.is_none());
        assert!(d.num_vals.is_none());

        let a = Arg::from_usage("[option] --opt=[opt]... 'some help info'");
        assert_eq!(a.name, "option");
        assert_eq!(a.long.unwrap(), "opt");
        assert!(a.short.is_none());
        assert_eq!(a.help.unwrap(), "some help info");
        assert!(a.multiple);
        assert!(a.takes_value);
        assert!(!a.required);
        assert!(a.val_names.is_none());
        assert!(a.num_vals.is_none());

        let a = Arg::from_usage("[option]... --opt=[opt] 'some help info'");
        assert_eq!(a.name, "option");
        assert_eq!(a.long.unwrap(), "opt");
        assert!(a.short.is_none());
        assert_eq!(a.help.unwrap(), "some help info");
        assert!(a.multiple);
        assert!(a.takes_value);
        assert!(!a.required);
        assert!(a.val_names.is_none());
        assert!(a.num_vals.is_none());

        let b = Arg::from_usage("--opt=[option]... 'some help info'");
        assert_eq!(b.name, "option");
        assert_eq!(b.long.unwrap(), "opt");
        assert!(b.short.is_none());
        assert_eq!(b.help.unwrap(), "some help info");
        assert!(b.multiple);
        assert!(b.takes_value);
        assert!(!b.required);
        assert!(b.val_names.is_none());
        assert!(b.num_vals.is_none());

        let c = Arg::from_usage("<option> --opt=<opt>... 'some help info'");
        assert_eq!(c.name, "option");
        assert_eq!(c.long.unwrap(), "opt");
        assert!(c.short.is_none());
        assert_eq!(c.help.unwrap(), "some help info");
        assert!(c.multiple);
        assert!(c.takes_value);
        assert!(c.required);
        assert!(c.val_names.is_none());
        assert!(c.num_vals.is_none());

        let c = Arg::from_usage("<option>... --opt=<opt> 'some help info'");
        assert_eq!(c.name, "option");
        assert_eq!(c.long.unwrap(), "opt");
        assert!(c.short.is_none());
        assert_eq!(c.help.unwrap(), "some help info");
        assert!(c.multiple);
        assert!(c.takes_value);
        assert!(c.required);
        assert!(c.val_names.is_none());
        assert!(c.num_vals.is_none());

        let d = Arg::from_usage("--opt=<option>... 'some help info'");
        assert_eq!(d.name, "option");
        assert_eq!(d.long.unwrap(), "opt");
        assert!(d.short.is_none());
        assert_eq!(d.help.unwrap(), "some help info");
        assert!(d.multiple);
        assert!(d.takes_value);
        assert!(d.required);
        assert!(d.val_names.is_none());
        assert!(d.num_vals.is_none());

		// Long and Short

        let a = Arg::from_usage("[option] -o --opt [option] 'some help info'");
        assert_eq!(a.name, "option");
        assert_eq!(a.long.unwrap(), "opt");
        assert_eq!(a.short.unwrap(), 'o');
        assert_eq!(a.help.unwrap(), "some help info");
        assert!(!a.multiple);
        assert!(a.takes_value);
        assert!(!a.required);
        assert!(a.val_names.is_none());
        assert!(a.num_vals.is_none());

        let b = Arg::from_usage("-o --opt [option] 'some help info'");
        assert_eq!(b.name, "option");
        assert_eq!(b.long.unwrap(), "opt");
        assert_eq!(b.short.unwrap(), 'o');
        assert_eq!(b.help.unwrap(), "some help info");
        assert!(!b.multiple);
        assert!(b.takes_value);
        assert!(!b.required);
        assert!(b.val_names.is_none());
        assert!(b.num_vals.is_none());

        let c = Arg::from_usage("<option> -o --opt <opt> 'some help info'");
        assert_eq!(c.name, "option");
        assert_eq!(c.long.unwrap(), "opt");
        assert_eq!(c.short.unwrap(), 'o');
        assert_eq!(c.help.unwrap(), "some help info");
        assert!(!c.multiple);
        assert!(c.takes_value);
        assert!(c.required);
        assert!(c.val_names.is_none());
        assert!(c.num_vals.is_none());

        let d = Arg::from_usage("-o --opt <option> 'some help info'");
        assert_eq!(d.name, "option");
        assert_eq!(d.long.unwrap(), "opt");
        assert_eq!(d.short.unwrap(), 'o');
        assert_eq!(d.help.unwrap(), "some help info");
        assert!(!d.multiple);
        assert!(d.takes_value);
        assert!(d.required);
        assert!(d.val_names.is_none());
        assert!(d.num_vals.is_none());

        let a = Arg::from_usage("[option]... -o --opt [option] 'some help info'");
        assert_eq!(a.name, "option");
        assert_eq!(a.long.unwrap(), "opt");
        assert_eq!(a.short.unwrap(), 'o');
        assert_eq!(a.help.unwrap(), "some help info");
        assert!(a.multiple);
        assert!(a.takes_value);
        assert!(!a.required);
        assert!(a.val_names.is_none());
        assert!(a.num_vals.is_none());

        let b = Arg::from_usage("-o --opt [option]... 'some help info'");
        assert_eq!(b.name, "option");
        assert_eq!(b.long.unwrap(), "opt");
        assert_eq!(b.short.unwrap(), 'o');
        assert_eq!(b.help.unwrap(), "some help info");
        assert!(b.multiple);
        assert!(b.takes_value);
        assert!(!b.required);
        assert!(b.val_names.is_none());
        assert!(b.num_vals.is_none());

        let c = Arg::from_usage("<option>... -o --opt <opt> 'some help info'");
        assert_eq!(c.name, "option");
        assert_eq!(c.long.unwrap(), "opt");
        assert_eq!(c.short.unwrap(), 'o');
        assert_eq!(c.help.unwrap(), "some help info");
        assert!(c.multiple);
        assert!(c.takes_value);
        assert!(c.required);
        assert!(c.val_names.is_none());
        assert!(c.num_vals.is_none());

        let d = Arg::from_usage("-o --opt <option>... 'some help info'");
        assert_eq!(d.name, "option");
        assert_eq!(d.long.unwrap(), "opt");
        assert_eq!(d.short.unwrap(), 'o');
        assert_eq!(d.help.unwrap(), "some help info");
        assert!(d.multiple);
        assert!(d.takes_value);
        assert!(d.required);
        assert!(d.val_names.is_none());
        assert!(d.num_vals.is_none());

		// Long and Short with '='

        let a = Arg::from_usage("[option] -o --opt=[option] 'some help info'");
        assert_eq!(a.name, "option");
        assert_eq!(a.long.unwrap(), "opt");
        assert_eq!(a.short.unwrap(), 'o');
        assert_eq!(a.help.unwrap(), "some help info");
        assert!(!a.multiple);
        assert!(a.takes_value);
        assert!(!a.required);
        assert!(a.val_names.is_none());
        assert!(a.num_vals.is_none());

        let b = Arg::from_usage("-o --opt=[option] 'some help info'");
        assert_eq!(b.name, "option");
        assert_eq!(b.long.unwrap(), "opt");
        assert_eq!(b.short.unwrap(), 'o');
        assert_eq!(b.help.unwrap(), "some help info");
        assert!(!b.multiple);
        assert!(b.takes_value);
        assert!(!b.required);
        assert!(b.val_names.is_none());
        assert!(b.num_vals.is_none());

        let c = Arg::from_usage("<option> -o --opt=<opt> 'some help info'");
        assert_eq!(c.name, "option");
        assert_eq!(c.long.unwrap(), "opt");
        assert_eq!(c.short.unwrap(), 'o');
        assert_eq!(c.help.unwrap(), "some help info");
        assert!(!c.multiple);
        assert!(c.takes_value);
        assert!(c.required);
        assert!(c.val_names.is_none());
        assert!(c.num_vals.is_none());

        let d = Arg::from_usage("-o --opt=<option> 'some help info'");
        assert_eq!(d.name, "option");
        assert_eq!(d.long.unwrap(), "opt");
        assert_eq!(d.short.unwrap(), 'o');
        assert_eq!(d.help.unwrap(), "some help info");
        assert!(!d.multiple);
        assert!(d.takes_value);
        assert!(d.required);
        assert!(d.val_names.is_none());
        assert!(d.num_vals.is_none());

        let a = Arg::from_usage("[option]... -o --opt=[option] 'some help info'");
        assert_eq!(a.name, "option");
        assert_eq!(a.long.unwrap(), "opt");
        assert_eq!(a.short.unwrap(), 'o');
        assert_eq!(a.help.unwrap(), "some help info");
        assert!(a.multiple);
        assert!(a.takes_value);
        assert!(!a.required);
        assert!(a.val_names.is_none());
        assert!(a.num_vals.is_none());

        let b = Arg::from_usage("-o --opt=[option]... 'some help info'");
        assert_eq!(b.name, "option");
        assert_eq!(b.long.unwrap(), "opt");
        assert_eq!(b.short.unwrap(), 'o');
        assert_eq!(b.help.unwrap(), "some help info");
        assert!(b.multiple);
        assert!(b.takes_value);
        assert!(!b.required);
        assert!(b.val_names.is_none());
        assert!(b.num_vals.is_none());

        let c = Arg::from_usage("<option>... -o --opt=<opt> 'some help info'");
        assert_eq!(c.name, "option");
        assert_eq!(c.long.unwrap(), "opt");
        assert_eq!(c.short.unwrap(), 'o');
        assert_eq!(c.help.unwrap(), "some help info");
        assert!(c.multiple);
        assert!(c.takes_value);
        assert!(c.required);
        assert!(c.val_names.is_none());
        assert!(c.num_vals.is_none());

        let d = Arg::from_usage("-o --opt=<option>... 'some help info'");
        assert_eq!(d.name, "option");
        assert_eq!(d.long.unwrap(), "opt");
        assert_eq!(d.short.unwrap(), 'o');
        assert_eq!(d.help.unwrap(), "some help info");
        assert!(d.multiple);
        assert!(d.takes_value);
        assert!(d.required);
        assert!(d.val_names.is_none());
        assert!(d.num_vals.is_none());
    }

    #[test]
    fn create_option_with_vals() {
        let d = Arg::from_usage("-o <opt> <opt> 'some help info'");
        assert_eq!(d.name, "opt");
        assert!(d.long.is_none());
        assert_eq!(d.short.unwrap(), 'o');
        assert_eq!(d.help.unwrap(), "some help info");
        assert!(!d.multiple);
        assert!(d.takes_value);
        assert!(d.required);
        assert!(d.val_names.is_none());
        assert_eq!(d.num_vals.unwrap(), 2);

        let d = Arg::from_usage("-o <opt> <opt>... 'some help info'");
        assert_eq!(d.name, "opt");
        assert!(d.long.is_none());
        assert_eq!(d.short.unwrap(), 'o');
        assert_eq!(d.help.unwrap(), "some help info");
        assert!(d.multiple);
        assert!(d.takes_value);
        assert!(d.required);
        assert!(d.val_names.is_none());
        assert_eq!(d.num_vals.unwrap(), 2);

        let d = Arg::from_usage("--opt <file> <mode>... 'some help info'");
        assert_eq!(d.name, "opt");
        assert!(d.short.is_none());
        assert_eq!(d.long.unwrap(), "opt");
        assert_eq!(d.help.unwrap(), "some help info");
        assert!(d.multiple);
        assert!(d.takes_value);
        assert!(d.required);
        let mut v = d.val_names.unwrap().into_iter().collect::<HashSet<_>>();
        for name in &["mode", "file"] {
            assert!(v.remove(name));
        }
        assert!(v.is_empty());
        assert_eq!(d.num_vals.unwrap(), 2);

        let d = Arg::from_usage("[myopt] --opt <file> <mode> 'some help info'");
        assert_eq!(d.name, "myopt");
        assert!(d.short.is_none());
        assert_eq!(d.long.unwrap(), "opt");
        assert_eq!(d.help.unwrap(), "some help info");
        assert!(!d.multiple);
        assert!(d.takes_value);
        assert!(!d.required);
        let mut v = d.val_names.unwrap().into_iter().collect::<HashSet<_>>();
        for name in &["mode", "file"] {
            assert!(v.remove(name));
        }
        assert!(v.is_empty());
        assert_eq!(d.num_vals.unwrap(), 2);

        let d = Arg::from_usage("--opt <option> <option> 'some help info'");
        assert_eq!(d.name, "option");
        assert!(d.short.is_none());
        assert_eq!(d.long.unwrap(), "opt");
        assert_eq!(d.help.unwrap(), "some help info");
        assert!(!d.multiple);
        assert!(d.takes_value);
        assert!(d.required);
        assert_eq!(d.num_vals.unwrap(), 2);
    }

    #[test]
	fn create_subcommand() {
        let _ = App::new("test")
	                .subcommand(SubCommand::with_name("some")
	                                        .arg(Arg::with_name("test")
	                                            .short("t")
	                                            .long("test")
	                                            .takes_value(true)
	                                            .help("testing testing")))
	                .arg(Arg::with_name("other").long("other"))
	                .get_matches();
    }

    #[test]
	fn create_multiple_subcommands() {
        let _ = App::new("test")
	                .subcommands(vec![ SubCommand::with_name("some")
	                                        .arg(Arg::with_name("test")
	                                            .short("t")
	                                            .long("test")
	                                            .takes_value(true)
	                                            .help("testing testing")),
	                                    SubCommand::with_name("add")
	                                        .arg(Arg::with_name("roster").short("r"))])
	                .arg(Arg::with_name("other").long("other"))
	                .get_matches();
    }

    #[test]
    #[should_panic]
	fn unique_arg_names() {
        App::new("some").args(vec![
	        Arg::with_name("arg").short("a"),
	        Arg::with_name("arg").short("b")
	    ]);
    }

    #[test]
    #[should_panic]
	fn unique_arg_shorts() {
        App::new("some").args(vec![
	        Arg::with_name("arg1").short("a"),
	        Arg::with_name("arg2").short("a")
	    ]);
    }

    #[test]
    #[should_panic]
	fn unique_arg_longs() {
        App::new("some").args(vec![
	        Arg::with_name("arg1").long("long"),
	        Arg::with_name("arg2").long("long")
	    ]);
    }

    fn run_app_test(test: Vec<&'static str>) -> Vec<String> {
        let m_val_names = ["one", "two"];
        let args = "-o --option=[opt]... 'tests options'
                    [positional] 'tests positionals'";
        let opt3_vals = ["fast", "slow"];
        let pos3_vals = ["vi", "emacs"];
        let matches = App::new("claptests")
                            // Test version from Cargo.toml
                            .version(&crate_version!()[..])
                            .about("tests clap library")
                            .author("Kevin K. <kbknapp@gmail.com>")
                            .args_from_usage(args)
                            .arg(Arg::from_usage("-f --flag... 'tests flags'")
                                .global(true))
                            .args(vec![
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
                            .get_matches_from(test);


        let mut result = Vec::<String>::new();

        if matches.is_present("flag") {
            result.push(format!("flag present {} times", matches.occurrences_of("flag")));
        } else {
            result.push(format!("flag NOT present"));
        }

        if matches.is_present("opt") {
            if let Some(v) = matches.value_of("opt") {
                result.push(format!("option present {} times with value: {}",matches.occurrences_of("opt"), v));
            }
            if let Some(ref ov) = matches.values_of("opt") {
                for o in ov {
                    result.push(format!("An option: {}", o));
                }
            }
        } else {
            result.push(format!("option NOT present"));
        }

        if let Some(p) = matches.value_of("positional") {
            result.push(format!("positional present with value: {}", p));
        } else {
            result.push(format!("positional NOT present"));
        }

        if matches.is_present("flag2") {
            result.push(format!("flag2 present"));
            result.push(format!("option2 present with value of: {}", matches.value_of("option2").unwrap()));
            result.push(format!("positional2 present with value of: {}", matches.value_of("positional2").unwrap()));
        } else {
            result.push(format!("flag2 NOT present"));
            result.push(format!("option2 maybe present with value of: {}", matches.value_of("option2").unwrap_or("Nothing")));
            result.push(format!("positional2 maybe present with value of: {}", matches.value_of("positional2").unwrap_or("Nothing")));
        }

        match matches.value_of("option3").unwrap_or("") {
            "fast" => result.push(format!("option3 present quickly")),
            "slow" => result.push(format!("option3 present slowly")),
            _      => result.push(format!("option3 NOT present"))
        }

        match matches.value_of("positional3").unwrap_or("") {
            "vi" => result.push(format!("positional3 present in vi mode")),
            "emacs" => result.push(format!("positional3 present in emacs mode")),
            _      => result.push(format!("positional3 NOT present"))
        }

        if matches.is_present("opt") {
            if let Some(v) = matches.value_of("opt") {
                result.push(format!("option present {} times with value: {}",matches.occurrences_of("opt"), v));
            }
            if let Some(ref ov) = matches.values_of("opt") {
                for o in ov {
                    result.push(format!("An option: {}", o));
                }
            }
        } else {
            result.push(format!("option NOT present"));
        }

        if let Some(p) = matches.value_of("positional") {
            result.push(format!("positional present with value: {}", p));
        } else {
            result.push(format!("positional NOT present"));
        }
        if matches.is_present("subcmd") {
            result.push(format!("subcmd present"));
            if let Some(matches) = matches.subcommand_matches("subcmd") {
                if matches.is_present("flag") {
                    result.push(format!("flag present {} times", matches.occurrences_of("flag")));
                } else {
                    result.push(format!("flag NOT present"));
                }

                if matches.is_present("scoption") {
                    if let Some(v) = matches.value_of("scoption") {
                        result.push(format!("scoption present with value: {}", v));
                    }
                    if let Some(ref ov) = matches.values_of("scoption") {
                        for o in ov {
                            result.push(format!("An scoption: {}", o));
                        }
                    }
                } else {
                    result.push(format!("scoption NOT present"));
                }

                if let Some(p) = matches.value_of("scpositional") {
                    result.push(format!("scpositional present with value: {}", p));
                }
            }
        } else {
            result.push(format!("subcmd NOT present"));
        }
        result
    }

    #[test]
    fn multiple_occurrences_of_flags() {
        assert_eq!(run_app_test(vec!["","value","--flag","--flag","-o", "some"]),
            vec!["flag present 2 times",
                "option present 1 times with value: some",
                "An option: some",
                "positional present with value: value",
                "flag2 NOT present",
                "option2 maybe present with value of: Nothing",
                "positional2 maybe present with value of: Nothing",
                "option3 NOT present",
                "positional3 NOT present",
                "option present 1 times with value: some",
                "An option: some",
                "positional present with value: value",
                "subcmd NOT present"
                ]);
    }

    #[test]
    fn posix_compatible_flags_long() {
        let m = App::new("posix")
                    .arg(Arg::from_usage("--flag  'some flag'").mutually_overrides_with("color"))
                    .arg(Arg::from_usage("--color 'some other flag'"))
                    .get_matches_from(vec!["", "--flag", "--color"]);
        assert!(m.is_present("color"));
        assert!(!m.is_present("flag"));

        let m = App::new("posix")
                    .arg(Arg::from_usage("--flag  'some flag'").mutually_overrides_with("color"))
                    .arg(Arg::from_usage("--color 'some other flag'"))
                    .get_matches_from(vec!["", "--color", "--flag"]);
        assert!(!m.is_present("color"));
        assert!(m.is_present("flag"));
    }

    #[test]
    fn posix_compatible_flags_short() {
        let m = App::new("posix")
                    .arg(Arg::from_usage("-f, --flag  'some flag'").mutually_overrides_with("color"))
                    .arg(Arg::from_usage("-c, --color 'some other flag'"))
                    .get_matches_from(vec!["", "-f", "-c"]);
        assert!(m.is_present("color"));
        assert!(!m.is_present("flag"));

        let m = App::new("posix")
                    .arg(Arg::from_usage("-f, --flag  'some flag'").mutually_overrides_with("color"))
                    .arg(Arg::from_usage("-c, --color 'some other flag'"))
                    .get_matches_from(vec!["", "-c", "-f"]);
        assert!(!m.is_present("color"));
        assert!(m.is_present("flag"));
    }

    #[test]
    fn posix_compatible_opts_long() {
        let m = App::new("posix")
                    .arg(Arg::from_usage("--flag [flag] 'some flag'").mutually_overrides_with("color"))
                    .arg(Arg::from_usage("--color [color] 'some other flag'"))
                    .get_matches_from(vec!["", "--flag", "some" ,"--color", "other"]);
        assert!(m.is_present("color"));
        assert!(!m.is_present("flag"));

        let m = App::new("posix")
                    .arg(Arg::from_usage("--flag [flag] 'some flag'").mutually_overrides_with("color"))
                    .arg(Arg::from_usage("--color [color] 'some other flag'"))
                    .get_matches_from(vec!["", "--color", "some" ,"--flag", "other"]);
        assert!(!m.is_present("color"));
        assert!(m.is_present("flag"));
    }

    #[test]
    fn posix_compatible_opts_long_equals() {
        let m = App::new("posix")
                    .arg(Arg::from_usage("--flag [flag] 'some flag'").mutually_overrides_with("color"))
                    .arg(Arg::from_usage("--color [color] 'some other flag'"))
                    .get_matches_from(vec!["", "--flag=some" ,"--color=other"]);
        assert!(m.is_present("color"));
        assert!(!m.is_present("flag"));

        let m = App::new("posix")
                    .arg(Arg::from_usage("--flag [flag] 'some flag'").mutually_overrides_with("color"))
                    .arg(Arg::from_usage("--color [color] 'some other flag'"))
                    .get_matches_from(vec!["", "--color=some" ,"--flag=other"]);
        assert!(!m.is_present("color"));
        assert!(m.is_present("flag"));
    }

    #[test]
    fn posix_compatible_opts_short() {
        let m = App::new("posix")
                    .arg(Arg::from_usage("-f [flag]  'some flag'").mutually_overrides_with("color"))
                    .arg(Arg::from_usage("-c [color] 'some other flag'"))
                    .get_matches_from(vec!["", "-f", "some", "-c", "other"]);
        assert!(m.is_present("color"));
        assert!(!m.is_present("flag"));

        let m = App::new("posix")
                    .arg(Arg::from_usage("-f [flag]  'some flag'").mutually_overrides_with("color"))
                    .arg(Arg::from_usage("-c [color] 'some other flag'"))
                    .get_matches_from(vec!["", "-c", "some", "-f", "other"]);
        assert!(!m.is_present("color"));
        assert!(m.is_present("flag"));
    }
}