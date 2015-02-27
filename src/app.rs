extern crate libc;

use std::collections::HashMap;
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
	// raw_args: Vec<Arg>,
	flags: HashMap<&'static str, FlagArg>,
	opts: HashMap<&'static str, OptArg>,
	positionals_idx: HashMap<u8, PosArg>,
	positionals_name: HashMap<&'static str, PosArg>,
	needs_long_help: bool,
	needs_long_version: bool,
	needs_short_help: bool,
	needs_short_version: bool,
	matches: ArgMatches
}

impl App {
	pub fn new(n: &'static str) -> App {
		App {
			name: n,
			author: None,
			about: None,
			version: None,
			// raw_args: vec![],
			flags: HashMap::new(),
			opts: HashMap::new(),
			positionals_idx: HashMap::new(),
			positionals_name: HashMap::new(),
			needs_long_version: true,
			needs_long_help: true,
			needs_short_help: true,
			needs_short_version: true,
			matches: ArgMatches::new(n)
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
		if let Some(i) = a.index {
			self.positionals_name.insert(a.name, PosArg {
				name: a.name,
				index: i,
				required: a.required,
				help: a.help,
				value: None
			});
			self.positionals_idx.insert(i, PosArg {
				name: a.name,
				index: i,
				required: a.required,
				help: a.help,
				value: None
			});
		} else if a.takes_value {
			self.opts.insert(a.name, OptArg {
				name: a.name,
				short: a.short,
				long: a.long,
				help: a.help,
				required: a.required,
				value: None
			});
		} else {
			if let Some(l) = a.long {
				if l == "help" {
					self.needs_long_help = false;
				} else if l == "version" {
					self.needs_long_version = false;
				}
			}
			if let Some(s) = a.short {
				if s == 'h' {
					self.needs_short_help = false;
				} else if s == 'v' {
					self.needs_short_version = false;
				}
			}
			self.flags.insert(a.name, FlagArg{
				name: a.name,
				short: a.short,
				long: a.long,
				help: a.help,
				multiple: a.multiple,
				occurrences: 1
			});
		}
		self
	}

	fn print_help(&self) {
		println!("Help info!");
		unsafe { libc::exit(0); }
	}

	fn print_version(&self) {
		let ver = match self.version { 
			Some(v) => v,
			None => ""
		};
		println!("{} {}", self.name, ver);
		unsafe { libc::exit(0); }
	}

	fn parse_single_short_flag(&mut self, arg: char) -> bool {
		for (k, v) in self.flags.iter() {
			if let Some(s) = v.short {
				if s != arg { continue; }

				let mut multi = false;
				if let Some(f) = self.matches.flags.get_mut(k) {
					f.occurrences = if f.multiple { f.occurrences + 1 } else { 1 };
					multi = true;
				} 

				if ! multi { 
					self.matches.flags.insert(k, v.clone());
				}
				return true;
			}
		}
		false
	}

	fn parse_long_arg(&mut self, full_arg: &'static str) -> Option<&'static str> {
		let mut arg = full_arg.trim_left_matches(|c| c == '-');
		let mut found = false;
		// let mut needs_val = false;

		if arg == "help" && self.needs_long_help {
			self.print_help();
		} else if arg == "version" && self.needs_long_version {
			self.print_version();
		}

		let mut arg_val: Option<&str> = None;

		if arg.contains("=") {
			let arg_vec: Vec<&str> = arg.split_str("=").collect();
			arg = arg_vec[0];
			arg_val = Some(arg_vec[1]);
		} 

		for (k, v) in self.opts.iter() {
			if let Some(l) = v.long {
				if l == arg {
					found = true;
					self.matches.opts.insert(k, OptArg{
						name: v.name,
					    short: v.short,
					    long: v.long, 
					    help: v.help,
					    required: v.required,
					    value: arg_val 
					});
				}
			}
		} 
		
		match arg_val {
			Some(_) => return arg_val,
			None => {}
		}	

		for (k, v) in self.flags.iter() {
			if let Some(l) = v.long {
				if l != arg { continue; }
				found = true;
				let mut multi = false; 
				if let Some(f) = self.matches.flags.get_mut(k) {
					f.occurrences = if f.multiple { f.occurrences + 1 } else { 1 };
					multi = true;
				} 

				if ! multi { 
					self.matches.flags.insert(k, v.clone());
				}
			}
		}


		// Fails if argument supplied to binary isn't valid
		assert!(found == false);

		None
	}

	fn check_for_help_and_version(&self, arg: char) {
		if arg == 'h' && self.needs_short_help {
			self.print_help();
		} else if arg == 'v' && self.needs_short_version {
			self.print_version();
		}
	}

	fn parse_short_arg(&mut self, full_arg: &'static str) -> Option<&'static str> {
		let mut found = false;
		let arg = full_arg.trim_left_matches(|c| c == '-');

		if arg.len() > 1 { 
			// Multiple flags using short i.e. -bgHlS
			for c in arg.chars() {
				self.check_for_help_and_version(c);
				found = self.parse_single_short_flag(c);
				if found { break; }
			}
		} else {
			// Short flag or opt
			let arg_c = arg.char_at(0);
			self.check_for_help_and_version(arg_c);
			found = self.parse_single_short_flag(arg_c);

			if found { return None; }

			for (k, v) in self.opts.iter() {
				if let Some(s) = v.short {
					if s == arg_c {
						return Some(v.name);
					}
				}
			} 
		}
		// Fails if argument supplied to binary isn't valid
		assert!(found == true);

		None
	}

	pub fn get_matches(&mut self) -> ArgMatches {

		let mut matches = ArgMatches::new(self.name);

		// let mut needs_val = false;
		let mut needs_val_of: Option<&'static str> = None; 
		let mut pos_counter = 1;
		for arg in env::args().collect::<Vec<String>>().tail() {
			let arg_slice = arg.as_slice();
			if let Some(nvo) = needs_val_of {
				if let Some(opt) = self.opts.get(nvo) {
					self.matches.opts.insert(nvo, OptArg{
						name: opt.name,
					    short: opt.short,
					    long: opt.long, 
					    help: opt.help,
					    required: opt.required,
					    value: Some(arg.as_slice()) 
					});
					needs_val_of = None;
					continue;
				}
			}
			if arg_slice.starts_with("--") {
				// Single flag, or option long version
				needs_val_of = self.parse_long_arg(arg_slice);

			} else if arg_slice.starts_with("-") {
				needs_val_of = self.parse_short_arg(arg_slice);
			} else {
				// Positional

				// Fails if no positionals are expected/possible
				assert!(self.positionals_idx.is_empty() == false);
				assert!(self.positionals_name.is_empty() == false);

				if let Some(p) = self.positionals_idx.get(&pos_counter) {
					self.matches.positionals.insert(p.name, PosArg{
						name: p.name,
						help: p.help,
						required: p.required,
						value: Some(arg),
						index: pos_counter
					});
					pos_counter += 1;
				}
			}
		}

		// Fails if we reached the end of args() but were still
		// expecting a value, such as ./fake -c
		// where -c takes a value
		assert!(needs_val_of == None);

		matches.fill_with(self);
		matches
	}
}