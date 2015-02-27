extern crate libc;

use std::collections::BTreeMap;
use std::collections::HashMap;
use std::collections::HashSet;
use std::env;

use ArgMatches;
use Arg;
use args::OptArg;
use args::FlagArg;
use args::PosArg;

pub struct App {
	pub name: &'static str,
	pub author: Option<&'static str>,
	pub version: Option<&'static str>,
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
	// blacklist: HashMap<&'static str, Vec<&'static str>>
}

impl App {
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
			arg_list: HashSet::new()
		}
	}

	pub fn author(&mut self, a: &'static str) -> &mut App {
		self.author = Some(a);
		self
	}

	pub fn about(&mut self, a: &'static str) -> &mut App {
		self.about = Some(a);
		self
	}

	pub fn version(&mut self, v: &'static str)-> &mut App  {
		self.version = Some(v);
		self
	}

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
				requires: a.requires.clone(),
				value: None
			});
			self.positionals_idx.insert(i, PosArg {
				name: a.name,
				index: i,
				required: a.required,
				requires: a.requires.clone(),
				help: a.help,
				value: None
			});
		} else if a.takes_value {
			self.opts.insert(a.name, OptArg {
				name: a.name,
				short: a.short,
				long: a.long,
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
			// Flags can't be required
			if self.required.contains(a.name) {
				self.required.remove(a.name);
			}
			self.flags.insert(a.name, FlagArg{
				name: a.name,
				short: a.short,
				long: a.long,
				help: a.help,
				multiple: a.multiple,
				requires: a.requires.clone(),
				occurrences: 1
			});
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
				println!("{}{}\t\t{}",
						if let Some(ref s) = v.short{format!("-{}",s)}else{format!("   ")},
						if let Some(ref l) = v.long {format!(",--{}",l)}else {format!("   ")},
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

	fn parse_single_short_flag(&mut self, matches: &mut ArgMatches, arg: char) -> bool {
		for (k, v) in self.flags.iter() {
			if let Some(s) = v.short {
				if s != arg { continue; }

				if !matches.flags.contains_key(k) {
					matches.flags.insert(k, v.clone());
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
				} else if matches.flags.get(k).unwrap().multiple { 
					matches.flags.get_mut(k).unwrap().occurrences += 1
				}

				return true;
			}
		}
		false
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
			let arg_vec: Vec<&str> = arg.split_str("=").collect();
			arg = arg_vec[0];
			arg_val = Some(arg_vec[1].to_string());
		} 

		for (k, v) in self.opts.iter() {
			if let Some(ref l) = v.long {
				if *l == arg {
					matches.opts.insert(k, OptArg{
						name: v.name,
					    short: v.short,
					    long: v.long, 
					    help: v.help,
					    required: v.required,
					    requires: v.requires.clone(),
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
				// Cannot borrow mutable twice at same time 
				// so the 'if let' must finish it's scope first
				// before calling .insert()
				if ! multi { 
					matches.flags.insert(k, v.clone());
					if self.required.contains(k) {
						self.required.remove(k);
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

	fn check_for_help_and_version(&self, arg: char) {
		if arg == 'h' && self.needs_short_help {
			self.print_help();
		} else if arg == 'v' && self.needs_short_version {
			self.print_version(true);
		}
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
					matches.opts.insert(nvo, OptArg{
						name: opt.name,
					    short: opt.short,
					    long: opt.long, 
					    help: opt.help,
					    requires: opt.requires.clone(),
					    required: opt.required,
					    value: Some(arg.clone()) 
					});
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
					matches.positionals.insert(p.name, PosArg{
						name: p.name,
						help: p.help,
						required: p.required,
						requires: p.requires.clone(),
						value: Some(arg.clone()),
						index: pos_counter
					});
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
		matches
	}
}