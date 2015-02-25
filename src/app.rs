use std::env;

use ArgMatches;
use Arg;
use args::OptArg;
use args::FlagArg;
use args::PosArg;

#[derive(Clone)]
pub struct App {
	name: &'static str,
	author: Option<&'static str>,
	version: Option<&'static str>,
	about: Option<&'static str>,
	raw_args: Vec<Arg>,
	flags: Vec<FlagArg>,
	opts: Vec<OptArg>,
	positionals: Vec<PosArg>
}

impl App {
	pub fn new(n: &'static str) -> App {
		App {
			name: n,
			author: None,
			about: None,
			version: None,
			raw_args: vec![],
			flags: vec![],
			opts: vec![],
			positionals: vec![]
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
			self.flags.push(FlagArg{
				name: a.name,
				short: a.short,
				long: a.long,
				help: a.help,
			});
		}
		self
	}

	pub fn get_matches(&mut self) -> ArgMatches {

		let mut matches = ArgMatches {
		    flags: vec![],
    		opts: vec![],
    		positionals: vec![], 
    		required: vec![],
    		blacklist: vec![],
    		about: self.about,
    		name: self.name,
    		author: self.author,
    		version: self.version 
		};

		let mut needs_val = false;
		let mut needs_val_of = String::new();
		let mut pos_counter = 1;
		for a in env::args().collect::<Vec<String>>().tail() {
			let arg_slice = a.as_slice();
			if needs_val {
				for o in self.opts.iter() {
					if needs_val_of == o.name.to_string() {
						matches.opts.push(OptArg{
							name: o.name,
						    short: o.short,
						    long: o.long, 
						    help: o.help,
						    required: o.required,
						    value: Some(a.clone()) 
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
				for f in self.flags.iter() {
					if let Some(l) = f.long {
						if l == p_arg.as_slice() {
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
							if l == p_arg.as_slice() {
								found = true;
								matches.opts.push(OptArg{
									name: o.name,
								    short: o.short,
								    long: o.long, 
								    help: o.help,
								    required: o.required,
								    value: Some(p_argv[1].to_string().clone()) 
								});
								break;
							}
						}
					} 
				} else {
					for o in self.opts.iter() {
						if let Some(l) = o.long {
							if l == p_arg.as_slice() {
								found = true;
								needs_val = true;
								needs_val_of = o.name.to_string();
								break;
							}
						}
					} 
					if ! found { panic!("Arg {} not valid", a); }
					continue;
				}
			} else if arg_slice.starts_with("-") {
				if arg_slice.len() > 2 {
					// Multiple flags using short i.e. -bgHlS
					let p_arg = arg_slice.trim_left_matches(|c| c == '-');
					let mut found = false;
					for c in p_arg.chars() {
						for f in self.flags.iter() {
							if let Some(s) = f.short {
								if c == s.char_at(0) {
									found = true;
									matches.flags.push(f.clone());
								}
							}
						}
						if ! found { panic!("Argument {} isn't valid.", arg_slice); }
						continue;
					}
				} else {
					// Short flag or opt
					let mut found = false;
					let p_arg = arg_slice.char_at(1); 
					for f in self.flags.iter() {
						if let Some(s) = f.short {
							if p_arg == s.char_at(0) {
								found = true;
								matches.flags.push(f.clone());
							}
						}
					}
					for o in self.opts.iter() {
						if let Some(s) = o.short {
							if s.char_at(0) == p_arg {
								found = true;
								needs_val = true;
								needs_val_of = o.name.to_string();
								break;
							}
						}
					} 
					if ! found { panic!("Argument {} isn't valid.", arg_slice); }
					continue;
				}
			} else {
				// Positional
				if self.positionals.is_empty() { panic!("Positional argument {} found but APP doesn't accept any", a); }
				for p in self.positionals.iter() {
					matches.positionals.push(PosArg{
						name: p.name,
						help: p.help,
						required: p.required,
						value: Some(a.clone()),
						index: pos_counter
					});
					pos_counter += 1;
				}
			}

		}
		if needs_val { panic!("Value not provided for argument {}", needs_val_of); }

		matches
	}
}