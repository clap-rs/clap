use std::fmt::{Display, Formatter, Result};
use std::result::Result as StdResult;
use std::rc::Rc;
use std::io;

use vec_map::VecMap;

use Arg;
use args::AnyArg;
use args::settings::{ArgFlags, ArgSettings};

#[allow(missing_debug_implementations)]
#[doc(hidden)]
pub struct PosBuilder<'n, 'e> {
    pub name: &'n str,
    pub help: Option<&'e str>,
    pub requires: Option<Vec<&'e str>>,
    pub blacklist: Option<Vec<&'e str>>,
    pub possible_vals: Option<Vec<&'e str>>,
    pub index: u8,
    pub num_vals: Option<u8>,
    pub max_vals: Option<u8>,
    pub min_vals: Option<u8>,
    pub val_names: Option<VecMap<&'e str>>,
    pub validator: Option<Rc<Fn(String) -> StdResult<(), String>>>,
    pub overrides: Option<Vec<&'e str>>,
    pub settings: ArgFlags,
    pub val_delim: Option<char>,
}

impl<'n, 'e> Default for PosBuilder<'n, 'e> {
    fn default() -> Self {
        PosBuilder {
            name: "",
            help: None,
            requires: None,
            blacklist: None,
            possible_vals: None,
            index: 0,
            num_vals: None,
            min_vals: None,
            max_vals: None,
            val_names: None,
            validator: None,
            overrides: None,
            settings: ArgFlags::new(),
            val_delim: Some(','),
        }
    }
}

impl<'n, 'e> PosBuilder<'n, 'e> {
    pub fn new(name: &'n str, idx: u8) -> Self {
        PosBuilder {
            name: name,
            index: idx,
            ..Default::default()
        }
    }

    pub fn from_arg(a: &Arg<'n, 'e>, idx: u8, reqs: &mut Vec<&'e str>) -> Self {
        assert!(a.short.is_none() || a.long.is_none(),
            format!("Argument \"{}\" has conflicting requirements, both index() and short(), \
                or long(), were supplied", a.name));

        // Create the Positional Argument Builder with each HashSet = None to only
        // allocate
        // those that require it
        let mut pb = PosBuilder {
            name: a.name,
            index: idx,
            num_vals: a.num_vals,
            min_vals: a.min_vals,
            max_vals: a.max_vals,
            val_names: a.val_names.clone(),
            help: a.help,
            val_delim: a.val_delim,
            ..Default::default()
        };
        if a.is_set(ArgSettings::Multiple) || a.num_vals.is_some() || a.max_vals.is_some() || a.min_vals.is_some() {
            pb.settings.set(ArgSettings::Multiple);
        }
        if a.is_set(ArgSettings::Required) {
            pb.settings.set(ArgSettings::Required);
        }
        if a.is_set(ArgSettings::Global) {
            pb.settings.set(ArgSettings::Global);
        }
        if a.is_set(ArgSettings::Hidden) {
            pb.settings.set(ArgSettings::Hidden);
        }
        // Check if there is anything in the blacklist (mutually excludes list) and add
        // any
        // values
        if let Some(ref bl) = a.blacklist {
            let mut bhs = vec![];
            // without derefing n = &&str
            for n in bl {
                bhs.push(*n);
            }
            pb.blacklist = Some(bhs);
        }
        if let Some(ref or) = a.overrides {
            let mut bhs = vec![];
            // without derefing n = &&str
            for n in or {
                bhs.push(*n);
            }
            pb.overrides = Some(bhs);
        }
        // Check if there is anything in the possible values and add those as well
        if let Some(ref p) = a.possible_vals {
            let mut phs = vec![];
            // without derefing n = &&str
            for n in p {
                phs.push(*n);
            }
            pb.possible_vals = Some(phs);
        }
        if let Some(ref p) = a.validator {
            pb.validator = Some(p.clone());
        }
        // Check if there is anything in the requires list and add any values
        if let Some(ref r) = a.requires {
            let mut rhs: Vec<&'e str> = vec![];
            // without derefing n = &&str
            for n in r {
                rhs.push(n);
                if a.is_set(ArgSettings::Required) {
                    reqs.push(n);
                }
            }
            pb.requires = Some(rhs);
        }
        pb
    }

    pub fn write_help<W: io::Write>(&self, w: &mut W, tab: &str, longest: usize) -> io::Result<()> {
        try!(write!(w, "{}", tab));
        try!(write!(w, "{}", self.name));
        if self.settings.is_set(ArgSettings::Multiple) {
            try!(write!(w, "..."));
        }
        write_spaces!((longest + 4) - (self.to_string().len()), w);
        if let Some(h) = self.help {
            if h.contains("{n}") {
                let mut hel = h.split("{n}");
                while let Some(part) = hel.next() {
                    try!(write!(w, "{}\n", part));
                    write_spaces!(longest + 6, w);
                    try!(write!(w, "{}", hel.next().unwrap_or("")));
                }
            } else {
                try!(write!(w, "{}", h));
            }
        }
        write!(w, "\n")
    }
}

impl<'n, 'e> Display for PosBuilder<'n, 'e> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        if self.settings.is_set(ArgSettings::Required) {
            try!(write!(f, "<{}>", self.name));
        } else {
            try!(write!(f, "[{}]", self.name));
        }
        if self.settings.is_set(ArgSettings::Multiple) {
            try!(write!(f, "..."));
        }

        Ok(())
    }
}

impl<'n, 'e> AnyArg<'n, 'e> for PosBuilder<'n, 'e> {
    fn name(&self) -> &'n str { self.name }
    fn overrides(&self) -> Option<&[&'e str]> { self.overrides.as_ref().map(|o| &o[..]) }
    fn requires(&self) -> Option<&[&'e str]> { self.requires.as_ref().map(|o| &o[..]) }
    fn blacklist(&self) -> Option<&[&'e str]> { self.blacklist.as_ref().map(|o| &o[..]) }
    fn is_set(&self, s: ArgSettings) -> bool { self.settings.is_set(s) }
    fn set(&mut self, s: ArgSettings) { self.settings.set(s) }
    fn has_switch(&self) -> bool { false }
    fn max_vals(&self) -> Option<u8> { self.max_vals }
    fn num_vals(&self) -> Option<u8> { self.num_vals }
    fn possible_vals(&self) -> Option<&[&'e str]> { self.possible_vals.as_ref().map(|o| &o[..]) }
    fn validator(&self) -> Option<&Rc<Fn(String) -> StdResult<(), String>>> {
        self.validator.as_ref()
    }
    fn min_vals(&self) -> Option<u8> { self.min_vals }
    fn short(&self) -> Option<char> { None }
    fn long(&self) -> Option<&'e str> { None }
    fn val_delim(&self) -> Option<char> { self.val_delim }
}

#[cfg(test)]
mod test {
    use super::PosBuilder;
    use args::settings::ArgSettings;

    #[test]
    fn posbuilder_display() {
        let mut p = PosBuilder::new("pos", 1);
        p.settings.set(ArgSettings::Multiple);

        assert_eq!(&*format!("{}", p), "[pos]...");

        let mut p2 = PosBuilder::new("pos", 1);
        p2.settings.set(ArgSettings::Required);

        assert_eq!(&*format!("{}", p2), "<pos>");
    }
}
