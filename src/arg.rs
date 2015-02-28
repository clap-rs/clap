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
	/// Creates a new instace of `Arg` using a unique string name. 
	/// The name will be used by the library consumer to get information about
	/// whether or not the argument was used at runtime. 
	///
	/// **NOTE:** in the case of arguments that take values (i.e. `takes_value(true)`)
	/// the name will also be displayed when the user prints the usage/help information
	/// of the program.
	///
	/// Example:
	///
	/// ```rust.example
	/// # let matches = App::new("myprog")
	/// #                 .arg(
	/// Arg::new("conifg")
	/// # ).get_matches();
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

	/// Sets the short version of the argument without the preceding `-`.
	///
	/// **NOTE:** Any leading `-` characters will be stripped, and only the first
	/// non `-` chacter will be used as the `short` version, i.e. for when the user
	/// mistakenly sets the short to `-o` or the like.
	/// Example:
	///
	/// ```rust.example
	/// # let matches = App::new("myprog")
	/// #                 .arg(
	/// # Arg::new("conifg")
	///       .short("c")
	/// # ).get_matches();
	pub fn short(&mut self, s: &'static str) -> &mut Arg {
		self.short = Some(s.trim_left_matches(|c| c == '-')
						   .char_at(0));
		self
	}

	/// Sets the long version of the argument without the preceding `--`.
	///
	/// **NOTE:** Any leading `-` characters will be stripped i.e. for 
	/// when the user mistakenly sets the short to `--out` or the like.
	///
	/// Example:
	///
	/// ```rust.example
	/// # let matches = App::new("myprog")
	/// #                 .arg(
	/// # Arg::new("conifg")
	///       .long("config")
	/// # ).get_matches();
	pub fn long(&mut self, l: &'static str) -> &mut Arg {
		self.long = Some(l.trim_left_matches(|c| c == '-'));
		self
	}

	/// Sets the help text of the argument that will be displayed to the user
	/// when they print the usage/help information. 
	///
	/// Example:
	///
	/// ```rust.example
	/// # let matches = App::new("myprog")
	/// #                 .arg(
	/// # Arg::new("conifg")
	///       .help("The config file used by the myprog")
	/// # ).get_matches();
	pub fn help(&mut self, h: &'static str) -> &mut Arg {
		self.help = Some(h);
		self
	}

	/// Sets whether or not the argument is required by default. Required by
	/// default means it is required, when no other mutually exlusive rules have
	/// been evaluated. Mutually exclusive rules take precedence over being required
	/// by default.
	///
	/// **NOTE:** Flags (i.e. not positional, or arguments that take values)
	/// cannot be required by default.
	/// when they print the usage/help information. 
	///
	/// Example:
	///
	/// ```rust.example
	/// # let matches = App::new("myprog")
	/// #                 .arg(
	/// # Arg::new("conifg")
	///       .required(true)
	/// # ).get_matches();
	pub fn required(&mut self, r: bool) -> &mut Arg {
		self.required = r;
		self
	}

	/// Sets a mutually exclusive argument by name. I.e. when using this argument, 
	/// the following argument can't be present.
	///
	/// **NOTE:** Mutually exclusive rules take precedence over being required
	/// by default. Mutually exclusive rules only need to be set for one of the two
	/// arguments, they do not need to be set for each.
	///
	/// Example:
	///
	/// ```rust.example
	/// # let mut myprog = App::new("myprog");
	/// myprog.arg(Arg::new("conifg")
	///                 .mutually_excludes("debug"))
	///       .arg(Arg::new("debug")
	///	                .short("d"))
	/// # .get_matches();
	pub fn mutually_excludes(&mut self, name: &'static str) -> &mut Arg {
		if let Some(ref mut vec) = self.blacklist {
			vec.push(name);
		} else {
			self.blacklist = Some(vec![]);
		}
		self
	}

	/// Sets a mutually exclusive arguments by names. I.e. when using this argument, 
	/// the following argument can't be present.
	///
	/// **NOTE:** Mutually exclusive rules take precedence over being required
	/// by default. Mutually exclusive rules only need to be set for one of the two
	/// arguments, they do not need to be set for each.
	///
	/// Example:
	///
	/// ```rust.example
	/// # let mut myprog = App::new("myprog");
	/// myprog.arg(Arg::new("conifg")
	///                 .mutually_excludes_all(
	///						vec!["debug", "input"]))
	///       .arg(Arg::new("debug")
	///	                .short("d"))
    ///       .arg(Arg::new("input"))
	/// # .get_matches();
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