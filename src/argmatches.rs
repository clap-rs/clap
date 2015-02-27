use std::collections::HashMap;
use std::collections::HashSet;

use app::App;
use args::{ FlagArg, OptArg, PosArg };

pub struct ArgMatches {
    pub required: Vec<&'static str>,
    pub blacklist: HashSet<&'static str>,
    pub about: Option<&'static str>,
    pub name: &'static str,
    pub author: Option<&'static str>,
    pub version: Option<&'static str>,
    pub flags: HashMap<&'static str, FlagArg>,
    pub opts: HashMap<&'static str, OptArg>,
    pub positionals: HashMap<&'static str, PosArg>,
}

impl ArgMatches {
	pub fn new(app: &App) -> ArgMatches {
		ArgMatches {
            flags: HashMap::new(),
            opts: HashMap::new(),
            positionals: HashMap::new(),
    		required: vec![],
    		blacklist: HashSet::new(),
    		about: app.about,
    		name: app.name,
    		author: app.author,
    		version: app.version,
    	}
	}

	pub fn value_of(&self, name: &'static str) -> Option<&String> {
        if let Some(ref opt) = self.opts.get(name) {
        	if let Some(ref v) = opt.value {
        		return Some(v);
        	} 
        }
        if let Some(ref pos) = self.positionals.get(name) {
        	if let Some(ref v) = pos.value {
        		return Some(v);
        	} 
        }
        None
	}

	pub fn is_present(&self, name: &'static str) -> bool {
        if let Some(_) = self.flags.get(name) {
            return true;
        }
        false
	}

    pub fn occurrences_of(&self, name: &'static str) -> u8 {
        if let Some(ref f) = self.flags.get(name) {
            return f.occurrences;
        }
        0
    }
}