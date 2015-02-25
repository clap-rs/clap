#[derive(Clone)]
pub struct Arg {
    pub name: &'static str,
    pub short: Option<&'static str>,
    pub long: Option<&'static str>,
    pub help: Option<&'static str>,
    pub required: bool,
    pub takes_value: bool,
    pub index: Option<i32>
    // allow_multiple: bool
    // exclusive_with
    // requires
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
			index: None
		}
	}

	pub fn short(&mut self, s: &'static str) -> &mut Arg {
		self.short = Some(s);
		self
	}

	pub fn long(&mut self, l: &'static str) -> &mut Arg {
		self.long = Some(l);
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

	pub fn takes_value(&mut self, tv: bool) -> &mut Arg {
		self.takes_value = tv;
		self
	}

	pub fn index(&mut self, i: i32) -> &mut Arg {
		self.index = Some(i);
		self
	}
}