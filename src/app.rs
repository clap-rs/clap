extern crate libc;

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
	flags: Vec<FlagArg>,
	opts: Vec<OptArg>,
	positionals: Vec<PosArg>,
	needs_long_help: bool,
	needs_long_version: bool,
	needs_short_help: bool,
	needs_short_version: bool
}

impl App {
	pub fn new(n: &'static str) -> App {
		App {
			name: n,
			author: None,
			about: None,
			version: None,
			// raw_args: vec![],
			flags: vec![],
			opts: vec![],
			needs_long_version: true,
			needs_long_help: true,
			positionals: vec![],
			needs_short_help: true,
			needs_short_version: true 
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
			self.positionals.push(PosArg {
				name: a.name,
				index: i,
				required: a.required,
				help: a.help,
				value: None

			});
		} else if a.takes_value {
			self.opts.push(OptArg {
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
			self.flags.push(FlagArg{
				name: a.name,
				short: a.short,
				long: a.long,
				help: a.help,
			});
		}
		self
	}

	pub fn print_help(&self) {
		println!("Help info!");
		unsafe { libc::exit(0); }
	}

	pub fn print_version(&self) {
		println!("Version Info!");
		unsafe { libc::exit(0); }
	}

	pub fn get_matches(&mut self) -> ArgMatches {

		let mut matches = ArgMatches::new(self);

		let mut needs_val = false;
		let mut needs_val_of = String::new();
		let mut pos_counter = 1;
		for arg in env::args().collect::<Vec<String>>().tail() {
			let arg_slice = arg.as_slice();
			if needs_val {
				for o in self.opts.iter() {
					if needs_val_of == o.name.to_string() {
						matches.opts.push(OptArg{
							name: o.name,
						    short: o.short,
						    long: o.long, 
						    help: o.help,
						    required: o.required,
						    value: Some(arg.clone()) 
						});
						needs_val = false;
						needs_val_of.clear();
						break;
					}
				}
				continue;
			}
			if arg_slice.starts_with("--") {
				// Single flag, or option
				let mut p_arg = arg_slice.trim_left_matches(|c| c == '-');
				let mut found = false;
				if p_arg == "help" && self.needs_long_help {
					self.print_help();
				} else if p_arg == "version" && self.needs_long_version {
					self.print_version();
				}
				for f in self.flags.iter() {
					if let Some(l) = f.long {
						if l == p_arg {
							matches.flags.push(f.clone());
							found = true;
							break;
						}
					}
				}
				if found { continue; }
				if p_arg.as_slice().contains("=") {
					let p_argv: Vec<&str> = p_arg.split_str("=").collect();
					p_arg = p_argv[0];
					for o in self.opts.iter() {
						if let Some(l) = o.long {
							if l == p_arg {
								// found = true;
								matches.opts.push(OptArg{
									name: o.name,
								    short: o.short,
								    long: o.long, 
								    help: o.help,
								    required: o.required,
								    value: Some(p_argv[1].to_string()) 
								});
								break;
							}
						}
					} 
				} else {
					for o in self.opts.iter() {
						if let Some(l) = o.long {
							if l == p_arg {
								found = true;
								needs_val = true;
								needs_val_of = o.name.to_string();
								break;
							}
						}
					} 
					// Fails if argument supplied to binary isn't valid
					assert!(found == false);
					continue;
				}
			} else if arg_slice.starts_with("-") {
				if arg_slice.len() > 2 {
					// Multiple flags using short i.e. -bgHlS
					let p_arg = arg_slice.trim_left_matches(|c| c == '-');
					let mut found = false;
					for c in p_arg.chars() {
						if c == 'h' && self.needs_short_help {
							self.print_help();
						} else if c == 'v' && self.needs_short_version {
							self.print_version();
						}
						for f in self.flags.iter() {
							if let Some(s) = f.short {
								if c == s {
									found = true;
									matches.flags.push(f.clone());
								}
							}
						}
						// Fails if argument supplied to binary isn't valid
						assert!(found == false);
						continue;
					}
				} else {
					// Short flag or opt
					let mut found = false;
					let p_arg = arg_slice.char_at(1); 
					if p_arg == 'h' && self.needs_short_help {
						self.print_help();
					} else if p_arg == 'v' && self.needs_short_version {
						self.print_version();
					}
					for f in self.flags.iter() {
						if let Some(s) = f.short {
							if p_arg == s {
								found = true;
								matches.flags.push(f.clone());
							}
						}
					}
					for o in self.opts.iter() {
						if let Some(s) = o.short {
							if s == p_arg {
								found = true;
								needs_val = true;
								needs_val_of = o.name.to_string();
								break;
							}
						}
					} 
					// Fails if argument supplied to binary isn't valid
					assert!(found == false);
					continue;
				}
			} else {
				// Positional

				// Fails if no positionals are expected/possible
				assert!(self.positionals.is_empty() == false);
				for p in self.positionals.iter() {
					matches.positionals.push(PosArg{
						name: p.name,
						help: p.help,
						required: p.required,
						value: Some(arg.clone()),
						index: pos_counter
					});
					pos_counter += 1;
				}
			}

		}

		// Fails if we reached the end of args() but were still
		// expecting a value, such as ./fake -c
		// where -c takes a value
		assert!(needs_val == false);

		matches
	}
}