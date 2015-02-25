use args::{ FlagArg, OptArg, PosArg };

pub struct ArgMatches {
    pub flags: Vec<FlagArg>,
    pub opts: Vec<OptArg>,
    pub positionals: Vec<PosArg>,
    pub required: Vec<&'static str>,
    pub blacklist: Vec<&'static str>,
    pub about: Option<&'static str>,
    pub name: &'static str,
    pub author: Option<&'static str>,
    pub version: Option<&'static str>
}

impl ArgMatches {
	pub fn value_of(&self, name: &'static str) -> Option<String> {
        for o in self.opts.iter() {
            if o.name == name { return o.value.clone(); }
        }
        for p in self.opts.iter() {
            if p.name == name { return p.value.clone(); }
        }
        None
	}

	pub fn is_present(&self, name: &'static str) -> bool {
        for f in self.flags.iter() {
            if f.name == name { return true; }
        }
        false
	}
}