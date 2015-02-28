/// The abstract representation of a command line argument used by the consumer of the library.
/// 
///
/// This struct is used by the library consumer and describes the command line arguments for 
/// their program.
/// and then evaluates the settings the consumer provided and determines the concret
/// argument struct to use when parsing.
///
/// Example:
///
/// ```rust.example
/// # let matches = App::new("myprog")
/// #                 .arg(
/// Arg::new("conifg")
///       .short("c")
///       .long("config")
///       .takes_value(true)
///       .help("Provides a config file to myprog")
/// # ).get_matches();
pub struct Arg {
	/// The unique name of the argument, required
    pub name: &'static str,
    /// The short version (i.e. single character) of the argument, no preceding `-`
    /// **NOTE:** `short` is mutually exclusive with `index`
    pub short: Option<char>,
    /// The long version of the flag (i.e. word) without the preceding `--`
    /// **NOTE:** `long` is mutually exclusive with `index`
    pub long: Option<&'static str>,
    /// The string of text that will displayed to the user when the application's
    /// `help` text is displayed
    pub help: Option<&'static str>,
    /// If this is a required by default when using the command line program
    /// i.e. a configuration file that's required for the program to function
    /// **NOTE:** required by default means, it is required *until* mutually
    /// exclusive arguments are evaluated.
    pub required: bool,
    /// Determines if this argument is an option, vice a flag or positional and
    /// is mutually exclusive with `index` and `multiple`
    pub takes_value: bool,
    /// The index of the argument. `index` is mutually exclusive with `takes_value`
    /// and `multiple`
    pub index: Option<u8>,
    /// Determines if multiple instances of the same flag are allowed. `multiple` 
    /// is mutually exclusive with `index` and `takes_value`.
    /// I.e. `-v -v -v` or `-vvv`
    pub multiple: bool,
    /// A list of names for other arguments that *may not* be used with this flag
   	pub blacklist: Option<Vec<&'static str>>, 
    /// A list of names of other arguments that are *required* to be used when 
    /// this flag is used
    pub requires: Option<Vec<&'static str>>
}

impl Arg {
	/// Creates a new instace of and `Arg`
	pub fn new(n: &'static str) -> Arg {
		Arg {
			name: n,
			short: None,
			long: None,
			help: None,
			required: false,
			takes_value: false,
			multiple: false,
			index: None,
			blacklist: Some(vec![]),
			requires: Some(vec![]),
		}
	}

	pub fn short(&mut self, s: &'static str) -> &mut Arg {
		self.short = Some(s.trim_left_matches(|c| c == '-')
						   .char_at(0));
		self
	}

	pub fn long(&mut self, l: &'static str) -> &mut Arg {
		self.long = Some(l.trim_left_matches(|c| c == '-'));
		self
	}

	pub fn help(&mut self, h: &'static str) -> &mut Arg {
		self.help = Some(h);
		self
	}

	pub fn required(&mut self, r: bool) -> &mut Arg {
		self.required = r;
		self
	}

	pub fn mutually_excludes(&mut self, name: &'static str) -> &mut Arg {
		if let Some(ref mut vec) = self.blacklist {
			vec.push(name);
		} else {
			self.blacklist = Some(vec![]);
		}
		self
	}

	pub fn mutually_excludes_all(&mut self, names: Vec<&'static str>) -> &mut Arg {
		if let Some(ref mut vec) = self.blacklist {
			for n in names {
				vec.push(n);
			}
		} else {
			self.blacklist = Some(vec![]);
		}
		self
	}

	pub fn requires(&mut self, name: &'static str) -> &mut Arg {
		if let Some(ref mut vec) = self.requires {
			vec.push(name);
		} else {
			self.requires = Some(vec![]);
		}
		self
	}

	pub fn requires_all(&mut self, names: Vec<&'static str>) -> &mut Arg {
		if let Some(ref mut vec) = self.requires {
			for n in names {
				vec.push(n);
			}
		} else {
			self.requires = Some(vec![]);
		}
		self
	}

	pub fn takes_value(&mut self, tv: bool) -> &mut Arg {
		assert!(self.index == None);
		self.takes_value = tv;
		self
	}

	pub fn index(&mut self, idx: u8) -> &mut Arg {
		assert!(self.takes_value == false);
		if idx < 1 { panic!("Argument index must start at 1"); }
		self.index = Some(idx);
		self
	}

	pub fn multiple(&mut self, multi: bool) -> &mut Arg {
		assert!(self.takes_value == false);
		assert!(self.index == None);
		self.multiple = multi;
		self
	}
}