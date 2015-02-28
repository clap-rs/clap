extern crate libc;

use std::collections::BTreeMap;
use std::collections::HashMap;
use std::collections::HashSet;
use std::env;

use argmatches::ArgMatches;
use Arg;
use args::OptArg;
use args::FlagArg;
use args::PosArg;

/// Used to create a representation of the program and all possible command line arguments
/// for parsing at runtime.
///
///
/// Stores a list of all posisble arguments, as well as information displayed to the user such as
/// help and versioning information.
///
/// Example:
///
/// ```rust.example
/// let myprog = App::new("myprog")
///                   .author("Me, me@mail.com")
///	                  .version("1.0.2")
///                   .about("Explains in brief what the program does")
///                   .arg(
///                        // Add a possible command line argument
///                    )
///                   .get_matches();
///
/// // Your pogram logic starts here...
/// ```
pub struct App {
	/// The name displayed to the user when showing version and help/usage information
	pub name: &'static str,
	/// A string of author(s) if desired. Displayed when showing help/usage information
	pub author: Option<&'static str>,
	/// The version displayed to the user
	pub version: Option<&'static str>,
	/// A brief explaination of the program that gets displayed to the user when shown help/usage information
	pub about: Option<&'static str>,
	flags: HashMap<&'static str, FlagArg>,
	opts: HashMap<&'static str, OptArg>,
	positionals_idx: BTreeMap<u8, PosArg>,
	positionals_name: HashMap<&'static str, PosArg>,
	needs_long_help: bool,
	needs_long_version: bool,
	needs_short_help: bool,
	needs_short_version: bool,
	required: HashSet<&'static str>,
	arg_list: HashSet<&'static str>,
	blacklist: HashSet<&'static str>,
}

impl App {
	/// Creates a new instance of an application requiring a name (such as the binary). Will be displayed
	/// to the user when they print version or help and usage information.
	///
	/// Example:
	///
	/// ```rust.example
	/// let prog = App::new("myprog");
	/// ```
	pub fn new(n: &'static str) -> App {
		App {
			name: n,
			author: None,
			about: None,
			version: None,
			flags: HashMap::new(),
			opts: HashMap::new(),
			positionals_idx: BTreeMap::new(),
			positionals_name: HashMap::new(),
			needs_long_version: true,
			needs_long_help: true,
			needs_short_help: true,
			needs_short_version: true,
			required: HashSet::new(), 
			arg_list: HashSet::new(),
			blacklist: HashSet::new(),
		}
	}

	/// Sets a string of author(s)
	///
	/// Example:
	///
	/// ```rust.example
	/// .author("Kevin <kbknapp@gmail.com>");
	/// ```
	pub fn author(&mut self, a: &'static str) -> &mut App {
		self.author = Some(a);
		self
	}

	/// Sets a string briefly describing what the program does
	///
	/// Example:
	///
	/// ```rust.example
	/// .about("Does really amazing things to great people");
	/// ```
	pub fn about(&mut self, a: &'static str) -> &mut App {
		self.about = Some(a);
		self
	}

	/// Sets a string of the version number
	///
	/// Example:
	///
	/// ```rust.example
	/// .version("v0.1.24");
	/// ```
	pub fn version(&mut self, v: &'static str)-> &mut App  {
		self.version = Some(v);
		self
	}

	/// Adds an argument to the list of valid possibilties
	///
	/// Example:
	///
	/// ```rust.example
	/// .arg(Arg::new("config")
	///     // Additional argument configuration goes here...
	/// );
	/// ```
	pub fn arg(&mut self, a: &Arg) -> &mut App {
		if self.arg_list.contains(a.name) {
			panic!("Argument name must be unique, \"{}\" is already in use", a.name);
		}
		if a.required {
			self.required.insert(a.name);
		}
		if let Some(i) = a.index {
			self.positionals_name.insert(a.name, PosArg {
				name: a.name,
				index: i,
				required: a.required,
				help: a.help,
				blacklist: a.blacklist.clone(),
				requires: a.requires.clone(),
				value: None
			});
			self.positionals_idx.insert(i, PosArg {
				name: a.name,
				index: i,
				required: a.required,
				blacklist: a.blacklist.clone(),
				requires: a.requires.clone(),
				help: a.help,
				value: None
			});
		} else if a.takes_value {
			if a.short == None && a.long == None {
				panic!("An argument that takes a value must have either a .short() or .long() [or both] assigned");
			}
			self.opts.insert(a.name, OptArg {
				name: a.name,
				short: a.short,
				long: a.long,
				blacklist: a.blacklist.clone(),
				help: a.help,
				requires: a.requires.clone(),
				required: a.required,
				value: None
			});
		} else {
			if let Some(ref l) = a.long {
				if *l == "help" {
					self.needs_long_help = false;
				} else if *l == "version" {
					self.needs_long_version = false;
				}
			}
			if let Some(ref s) = a.short {
				if *s == 'h' {
					self.needs_short_help = false;
				} else if *s == 'v' {
					self.needs_short_version = false;
				}
			}
			if a.short == None && a.long == None {
				panic!("A flag argument must have either a .short() or .long() [or both] assigned");
			}
			// Flags can't be required
			if self.required.contains(a.name) {
				self.required.remove(a.name);
			}
			self.flags.insert(a.name, FlagArg{
				name: a.name,
				short: a.short,
				long: a.long,
				help: a.help,
				blacklist: a.blacklist.clone(),
				multiple: a.multiple,
				requires: a.requires.clone(),
				occurrences: 1
			});
		}
		self
	}

	/// Adds arguments to the list of valid possibilties
	///
	/// Example:
	///
	/// ```rust.example
	/// .args( vec![Arg::new("config").short("c"),
	///				Arg::new("debug").short("d")]);
	/// ```
	pub fn args(&mut self, args: Vec<&Arg>) -> &mut App {
		for arg in args.iter() {
			self.arg(arg);
		}
		self
	}

	fn exit(&self) {
		unsafe { libc::exit(0); }
	}

	fn report_error(&self, msg: &String, help: bool, quit: bool) {
		println!("{}", msg);
		if help { self.print_help(); }
		if quit {self.exit(); }
	}

	fn print_help(&self) {
		self.print_version(false);
		let mut flags = false;
		let mut pos = false;
		let mut opts = false;

		if let Some(ref author) = self.author {
			println!("{}", author);
		}
		if let Some(ref about) = self.about {
			println!("{}", about);
		}
		println!("");
		print!("USAGE: {} {} {} {}", self.name,
			if ! self.flags.is_empty() {flags = true; "[FLAGS]"} else {""},
			if ! self.opts.is_empty() {opts = true; "[OPTIONS]"} else {""},
			if ! self.positionals_name.is_empty() {pos = true; "[POSITIONAL]"} else {""});
		if flags || opts || pos {
			println!("");
			println!("Where...");
		}
		if flags {
			println!("");
			println!("FLAGS:");
			for (_, v) in self.flags.iter() {
				println!("{}{}\t\t{}",
						if let Some(ref s) = v.short{format!("-{}",s)}else{format!("   ")},
						if let Some(ref l) = v.long {format!(",--{}",l)}else {format!("   ")},
						if let Some(ref h) = v.help {*h} else {"   "} );
			}
		}
		if opts {
			println!("");
			println!("OPTIONS:");
			for (_, v) in self.opts.iter() {
				println!("{}{}{}\t\t{}",
						if let Some(ref s) = v.short{format!("-{}",s)}else{format!("   ")},
						if let Some(ref l) = v.long {format!(",--{}",l)}else {format!("   ")},
						format!(" <{}>", v.name),
						if let Some(ref h) = v.help {*h} else {"   "} );
			}
		}
		if pos {
			println!("");
			println!("POSITIONAL ARGUMENTS:");
			for (_, v) in self.positionals_idx.iter() {
				println!("{}\t\t\t{}", v.name,
						if let Some(ref h) = v.help {*h} else {"   "} );
			}
		}

		self.exit();
	}

	fn print_version(&self, quit: bool) {
		println!("{} {}", self.name, if let Some(ref v) = self.version {*v} else {""} );
		if quit { self.exit(); }
	}

	fn check_for_help_and_version(&self, arg: char) {
		if arg == 'h' && self.needs_short_help {
			self.print_help();
		} else if arg == 'v' && self.needs_short_version {
			self.print_version(true);
		}
	}

	fn parse_long_arg(&mut self, matches: &mut ArgMatches ,full_arg: &String) -> Option<&'static str> {
		let mut arg = full_arg.as_slice().trim_left_matches(|c| c == '-');
		let mut found = false;

		if arg == "help" && self.needs_long_help {
			self.print_help();
		} else if arg == "version" && self.needs_long_version {
			self.print_version(true);
		}

		let mut arg_val: Option<String> = None;

		if arg.contains("=") {
			let arg_vec: Vec<&str> = arg.split("=").collect();
			arg = arg_vec[0];
			arg_val = Some(arg_vec[1].to_string());
		} 

		for (k, v) in self.opts.iter() {
			if let Some(ref l) = v.long {
				if *l == arg {
					if self.blacklist.contains(k) {
						self.report_error(&format!("The argument --{} is mutually exclusive with one or more other arguments", arg),
							false, true);
					}
					matches.opts.insert(k, OptArg{
						name: v.name,
					    short: v.short,
					    long: v.long, 
					    help: v.help,
					    required: v.required,
					    blacklist: None,
					    requires: None,
					    value: arg_val.clone() 
					});
					match arg_val {
						None => { return Some(v.name); },
						_ => { return None; }
					}	
				}
			}
		} 

		for (k, v) in self.flags.iter() {
			if let Some(ref l) = v.long {
				if *l != arg { continue; }
				found = true;
				let mut multi = false;
				if let Some(ref mut f) = matches.flags.get_mut(k) {
					f.occurrences = if f.multiple { f.occurrences + 1 } else { 1 };
					multi = true;
				}  
				if ! multi { 
					if self.blacklist.contains(k) {
						self.report_error(&format!("The argument --{} is mutually exclusive with one or more other arguments", arg),
							false, true);
					}
					matches.flags.insert(k, FlagArg{
					    name: v.name,
					    short: v.short,
					    long: v.long,
					    help: v.help,
					    multiple: v.multiple,
					    occurrences: v.occurrences,
					    blacklist: None, 
					    requires: None
					});
					if self.required.contains(k) {
						self.required.remove(k);
					}
					if let Some(ref bl) = v.blacklist {
						if ! bl.is_empty() {
							for name in bl.iter() {
								self.blacklist.insert(name);
							}
						}
					}
				}
				if let Some(ref reqs) = v.requires {
					if ! reqs.is_empty() {
						for n in reqs.iter() {
							if matches.opts.contains_key(n) { continue; }
							if matches.flags.contains_key(n) { continue; }
							if matches.positionals.contains_key(n) { continue; }
							self.required.insert(n);
						}
					}
				}
				break;
			}
		}

		if ! found {
			self.report_error(
				&format!("Argument --{} isn't valid", arg),
				false, true);
		}
		None
	}

	fn parse_short_arg(&mut self, matches: &mut ArgMatches ,full_arg: &String) -> Option<&'static str> {
		let arg = full_arg.as_slice().trim_left_matches(|c| c == '-');
		if arg.len() > 1 { 
			// Multiple flags using short i.e. -bgHlS
			for c in arg.chars() {
				self.check_for_help_and_version(c);
				if ! self.parse_single_short_flag(matches, c) { 
					self.report_error(
						&format!("Argument -{} isn't valid",c),
						false, true);
				}
			}
		} else {
			// Short flag or opt
			let arg_c = arg.char_at(0);
			self.check_for_help_and_version(arg_c);

			if ! self.parse_single_short_flag(matches, arg_c) { 
				for (k, v) in self.opts.iter() {
					if let Some(s) = v.short {
						if s == arg_c {
							return Some(k)
						}
					}
				} 

				self.report_error(
					&format!("Argument -{} isn't valid",arg_c),
					false, true);
			}

		}
		None
	}

	fn parse_single_short_flag(&mut self, matches: &mut ArgMatches, arg: char) -> bool {
		for (k, v) in self.flags.iter() {
			if let Some(s) = v.short {
				if s != arg { continue; }

				if !matches.flags.contains_key(k) {
					if self.blacklist.contains(k) {
						self.report_error(&format!("The argument -{} is mutually exclusive with one or more other arguments", arg),
							false, true);
					}
					matches.flags.insert(k, FlagArg{
					    name: v.name,
					    short: v.short,
					    long: v.long,
					    help: v.help,
					    multiple: v.multiple,
					    occurrences: v.occurrences,
					    blacklist: None, 
					    requires: None
					});
					if self.required.contains(k) {
						self.required.remove(k);
					}
					if let Some(ref reqs) = v.requires {
						if ! reqs.is_empty() {
							for n in reqs.iter() {
								if matches.opts.contains_key(n) { continue; }
								if matches.flags.contains_key(n) { continue; }
								if matches.positionals.contains_key(n) { continue; }
								self.required.insert(n);
							}
						}
					}
					if let Some(ref bl) = v.blacklist {
						if ! bl.is_empty() {
							for name in bl.iter() {
								self.blacklist.insert(name);
							}
						}
					}
				} else if matches.flags.get(k).unwrap().multiple { 
					matches.flags.get_mut(k).unwrap().occurrences += 1
				}

				return true;
			}
		}
		false
	}

	fn validate_blacklist(&self, matches: &ArgMatches) {
		if ! self.blacklist.is_empty() {
			for name in self.blacklist.iter() {
				for (k, v) in matches.flags.iter() {
					if k == name {
						self.report_error(&format!("The argument {} is mutually exclusive with one or more other arguments",
							if let Some(s) = v.short {
								format!("-{}", s)
							} else if let Some(l) = v.long {
								format!("--{}", l)
							} else {
								format!("\"{}\"", v.name)
							}),
							false, true);
					}
				}
				for (k, v) in matches.opts.iter() {
					if k == name {
						self.report_error(&format!("The argument {} is mutually exclusive with one or more other arguments",
							if let Some(s) = v.short {
								format!("-{}", s)
							} else if let Some(l) = v.long {
								format!("--{}", l)
							} else {
								format!("\"{}\"", v.name)
							}),
							false, true);
					}
				}
				for (k, v) in matches.positionals.iter() {
					if k == name {
						self.report_error(&format!("The argument \"{}\" is mutually exclusive with one or more other arguments",v.name),
							false, true);
					}
				}
			}
		}
	}

	pub fn get_matches(&mut self) -> ArgMatches {
		let mut matches = ArgMatches::new(self);

		// let mut needs_val = false;
		let mut needs_val_of: Option<&'static str> = None; 
		let mut pos_counter = 1;
		for arg in env::args().collect::<Vec<String>>().tail() {
			let arg_slice = arg.as_slice();
			let mut skip = false;
			if let Some(ref nvo) = needs_val_of {
				if let Some(ref opt) = self.opts.get(nvo) {
					if self.blacklist.contains(opt.name) {
						self.report_error(
							&format!("The argument {} is mutually exclusive with one or more other arguments", 
							if let Some(long) = opt.long {
								format!("--{}",long)
							}else{
								format!("-{}",opt.short.unwrap())
							}),false, true);
					}
					matches.opts.insert(nvo, OptArg{
						name: opt.name,
					    short: opt.short,
					    long: opt.long, 
					    help: opt.help,
					    requires: None,
					    blacklist: None,
					    required: opt.required,
					    value: Some(arg.clone()) 
					});
					if let Some(ref bl) = opt.blacklist {
						if ! bl.is_empty() {
							for name in bl.iter() {
								self.blacklist.insert(name);
							}
						}
					}
					if self.required.contains(opt.name) {
						self.required.remove(opt.name);
					}
					if let Some(ref reqs) = opt.requires {
						if ! reqs.is_empty() {
							for n in reqs.iter() {
								if matches.opts.contains_key(n) { continue; }
								if matches.flags.contains_key(n) { continue; }
								if matches.positionals.contains_key(n) { continue; }
								self.required.insert(n);
							}
						}
					}
					skip = true;
				}
			}
			if skip {
				needs_val_of = None;
				continue;
			}
			if arg_slice.starts_with("--") {
				// Single flag, or option long version
				needs_val_of = self.parse_long_arg(&mut matches, &arg);

			} else if arg_slice.starts_with("-") {
				needs_val_of = self.parse_short_arg(&mut matches, &arg);
			} else {
				// Positional

				if self.positionals_idx.is_empty() || self.positionals_name.is_empty() {
					self.report_error(
						&format!("Found positional argument {}, but {} doesn't accept any", arg, self.name),
						false, true);
				}
				if let Some(ref p) = self.positionals_idx.get(&pos_counter) {
					if self.blacklist.contains(p.name) {
						self.report_error(&format!("The argument \"{}\" is mutually exclusive with one or more other arguments", arg),
							false, true);
					}
					matches.positionals.insert(p.name, PosArg{
						name: p.name,
						help: p.help,
						required: p.required,
						blacklist: None,
						requires: None,
						value: Some(arg.clone()),
						index: pos_counter
					});
					if let Some(ref bl) = p.blacklist {
						if ! bl.is_empty() {
							for name in bl.iter() {
								self.blacklist.insert(name);
							}
						}
					}
					if self.required.contains(p.name) {
						self.required.remove(p.name);
					}
					if let Some(ref reqs) = p.requires {
						if ! reqs.is_empty() {
							for n in reqs.iter() {
								if matches.opts.contains_key(n) { continue; }
								if matches.flags.contains_key(n) { continue; }
								if matches.positionals.contains_key(n) { continue; }
								self.required.insert(n);
							}
						}
					}
					pos_counter += 1;
				} else {
					self.report_error(&format!("Positional argument \"{}\" was found, but {} wasn't expecting any", arg, self.name), false, true);
				}
			}
		}

		match needs_val_of {
			Some(ref a) => {
				self.report_error(
					&format!("Argument \"{}\" requires a value but none was supplied", a),
					false, true);
			}
			_ => {}
		}
		if ! self.required.is_empty() {
			self.report_error(&"One or more required arguments were not supplied".to_string(),
					false, true);
		}

		self.validate_blacklist(&matches);

		matches
	}
}