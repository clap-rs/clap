pub struct Arg {
    pub name: &'static str,
    pub short: Option<char>,
    pub long: Option<&'static str>,
    pub help: Option<&'static str>,
    pub required: bool,
    pub takes_value: bool,
    pub index: Option<u8>,
    pub multiple: bool,
    // exclusive_with
    pub requires: Option<Vec<&'static str>>
}

impl Arg {
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