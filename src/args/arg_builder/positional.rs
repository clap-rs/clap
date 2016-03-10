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
    pub index: u64,
    pub num_vals: Option<u64>,
    pub max_vals: Option<u64>,
    pub min_vals: Option<u64>,
    pub val_names: Option<VecMap<&'e str>>,
    pub validator: Option<Rc<Fn(String) -> StdResult<(), String>>>,
    pub overrides: Option<Vec<&'e str>>,
    pub settings: ArgFlags,
    pub val_delim: Option<char>,
    pub default_val: Option<&'n str>,
    pub disp_ord: usize,
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
            default_val: None,
            disp_ord: 999,
        }
    }
}

impl<'n, 'e> PosBuilder<'n, 'e> {
    pub fn new(name: &'n str, idx: u64) -> Self {
        PosBuilder {
            name: name,
            index: idx,
            ..Default::default()
        }
    }

    pub fn from_arg(a: &Arg<'n, 'e>, idx: u64, reqs: &mut Vec<&'e str>) -> Self {
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
            blacklist: a.blacklist.clone(),
            overrides: a.overrides.clone(),
            requires: a.requires.clone(),
            possible_vals: a.possible_vals.clone(),
            help: a.help,
            val_delim: a.val_delim,
            settings: a.settings,
            default_val: a.default_val,
            disp_ord: a.disp_ord,
            ..Default::default()
        };
        if a.max_vals.is_some()
            || a.min_vals.is_some()
            || (a.num_vals.is_some() && a.num_vals.unwrap() > 1) {
            pb.settings.set(ArgSettings::Multiple);
        }
        if let Some(ref p) = a.validator {
            pb.validator = Some(p.clone());
        }
        // If the arg is required, add all it's requirements to master required list
        if a.is_set(ArgSettings::Required) {
            if let Some(ref areqs) = a.requires {
                for r in areqs { reqs.push(*r); }
            }
        }
        pb
    }

    pub fn write_help<W: io::Write>(&self, w: &mut W, tab: &str, longest: usize, skip_pv: bool, nlh: bool) -> io::Result<()> {
        write_arg_help!(@pos self, w, tab, longest, skip_pv, nlh);
        write!(w, "\n")
    }
}

impl<'n, 'e> Display for PosBuilder<'n, 'e> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        if self.settings.is_set(ArgSettings::Required) {
            if let Some(ref names) = self.val_names {
                try!(write!(f, "{}", names.values().map(|n| format!("<{}>", n)).collect::<Vec<_>>().join(" ")));
            } else {
                try!(write!(f, "<{}>", self.name));
            }
        } else {
            if let Some(ref names) = self.val_names {
                try!(write!(f, "{}", names.values().map(|n| format!("[{}]", n)).collect::<Vec<_>>().join(" ")));
            } else {
                try!(write!(f, "[{}]", self.name));
            }
        }
        if self.settings.is_set(ArgSettings::Multiple) && self.val_names.is_none() {
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
    fn max_vals(&self) -> Option<u64> { self.max_vals }
    fn num_vals(&self) -> Option<u64> { self.num_vals }
    fn possible_vals(&self) -> Option<&[&'e str]> { self.possible_vals.as_ref().map(|o| &o[..]) }
    fn validator(&self) -> Option<&Rc<Fn(String) -> StdResult<(), String>>> {
        self.validator.as_ref()
    }
    fn min_vals(&self) -> Option<u64> { self.min_vals }
    fn short(&self) -> Option<char> { None }
    fn long(&self) -> Option<&'e str> { None }
    fn val_delim(&self) -> Option<char> { self.val_delim }
}

#[cfg(test)]
mod test {
    use super::PosBuilder;
    use args::settings::ArgSettings;
    use vec_map::VecMap;

    #[test]
    fn display_mult() {
        let mut p = PosBuilder::new("pos", 1);
        p.settings.set(ArgSettings::Multiple);

        assert_eq!(&*format!("{}", p), "[pos]...");
    }

    #[test]
    fn display_required() {
        let mut p2 = PosBuilder::new("pos", 1);
        p2.settings.set(ArgSettings::Required);

        assert_eq!(&*format!("{}", p2), "<pos>");
    }

    #[test]
    fn display_val_names() {
        let mut p2 = PosBuilder::new("pos", 1);
        let mut vm = VecMap::new();
        vm.insert(0, "file1");
        vm.insert(1, "file2");
        p2.val_names = Some(vm);

        assert_eq!(&*format!("{}", p2), "[file1] [file2]");
    }

    #[test]
    fn display_val_names_req() {
        let mut p2 = PosBuilder::new("pos", 1);
        p2.settings.set(ArgSettings::Required);
        let mut vm = VecMap::new();
        vm.insert(0, "file1");
        vm.insert(1, "file2");
        p2.val_names = Some(vm);

        assert_eq!(&*format!("{}", p2), "<file1> <file2>");
    }
}
