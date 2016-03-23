use std::rc::Rc;
use std::fmt::{Display, Formatter, Result};
use std::result::Result as StdResult;
use std::io;

use vec_map::VecMap;

use args::{AnyArg, Arg, HelpWriter};
use args::settings::{ArgFlags, ArgSettings};

#[allow(missing_debug_implementations)]
#[doc(hidden)]
pub struct OptBuilder<'n, 'e> {
    pub name: &'n str,
    pub short: Option<char>,
    pub long: Option<&'e str>,
    pub help: Option<&'e str>,
    pub blacklist: Option<Vec<&'e str>>,
    pub possible_vals: Option<Vec<&'e str>>,
    pub requires: Option<Vec<&'e str>>,
    pub num_vals: Option<u64>,
    pub min_vals: Option<u64>,
    pub max_vals: Option<u64>,
    pub val_names: Option<VecMap<&'e str>>,
    pub validator: Option<Rc<Fn(String) -> StdResult<(), String>>>,
    pub overrides: Option<Vec<&'e str>>,
    pub settings: ArgFlags,
    pub val_delim: Option<char>,
    pub default_val: Option<&'n str>,
    pub disp_ord: usize,
}

impl<'n, 'e> Default for OptBuilder<'n, 'e> {
    fn default() -> Self {
        OptBuilder {
            name: "",
            short: None,
            long: None,
            help: None,
            blacklist: None,
            possible_vals: None,
            requires: None,
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

impl<'n, 'e> OptBuilder<'n, 'e> {
    pub fn new(name: &'n str) -> Self {
        OptBuilder {
            name: name,
            ..Default::default()
        }
    }

    pub fn from_arg(a: &Arg<'n, 'e>, reqs: &mut Vec<&'e str>) -> Self {
        assert!(a.short.is_some() || a.long.is_some(),
            format!("Argument \"{}\" has takes_value(true), yet neither a short() or long() \
                was supplied", a.name));

        // No need to check for .index() as that is handled above
        let mut ob = OptBuilder {
            name: a.name,
            short: a.short,
            long: a.long,
            help: a.help,
            num_vals: a.num_vals,
            min_vals: a.min_vals,
            max_vals: a.max_vals,
            val_names: a.val_names.clone(),
            val_delim: a.val_delim,
            blacklist: a.blacklist.clone(),
            overrides: a.overrides.clone(),
            requires: a.requires.clone(),
            possible_vals: a.possible_vals.clone(),
            settings: a.settings,
            default_val: a.default_val,
            disp_ord: a.disp_ord,
            ..Default::default()
        };
        if let Some(ref vec) = ob.val_names {
            if vec.len() > 1 {
                ob.num_vals = Some(vec.len() as u64);
            }
        }
        if let Some(ref p) = a.validator {
            ob.validator = Some(p.clone());
        }
        // If the arg is required, add all it's requirements to master required list
        if a.is_set(ArgSettings::Required) {
            if let Some(ref areqs) = a.requires {
                for r in areqs { reqs.push(*r); }
            }
        }
        ob
    }

    pub fn write_help<W: io::Write>(&self, w: &mut W, longest: usize, skip_pv: bool, nlh: bool) -> io::Result<()> {
        let mut hw = HelpWriter::new(self, longest, nlh);
        hw.skip_pv = skip_pv;
        hw.write_to(w)
    }
}

impl<'n, 'e> Display for OptBuilder<'n, 'e> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        debugln!("fn=fmt");
        // Write the name such --long or -l
        if let Some(l) = self.long {
            try!(write!(f, "--{} ", l));
        } else {
            try!(write!(f, "-{} ", self.short.unwrap()));
        }

        // Write the values such as <name1> <name2>
        if let Some(ref vec) = self.val_names {
            let mut it = vec.iter().peekable();
            while let Some((_, val)) = it.next() {
                try!(write!(f, "<{}>", val));
                if it.peek().is_some() { try!(write!(f, " ")); }
            }
            let num = vec.len();
            if self.is_set(ArgSettings::Multiple) && num == 1 {
                try!(write!(f, "..."));
            }
        } else if let Some(num) = self.num_vals {
            let mut it = (0..num).peekable();
            while let Some(_) = it.next() {
                try!(write!(f, "<{}>", self.name));
                if it.peek().is_some() { try!(write!(f, " ")); }
            }
        } else {
            try!(write!(f, "<{}>{}", self.name, if self.is_set(ArgSettings::Multiple) { "..." } else { "" }));
        }

        Ok(())
    }
}

impl<'n, 'e> Clone for OptBuilder<'n, 'e> {
    fn clone(&self) -> Self {
        OptBuilder {
            name: self.name,
            short: self.short,
            long: self.long,
            help: self.help,
            blacklist: self.blacklist.clone(),
            overrides: self.overrides.clone(),
            requires: self.requires.clone(),
            settings: self.settings,
            disp_ord: self.disp_ord,
            num_vals: self.num_vals,
            min_vals: self.min_vals,
            max_vals: self.max_vals,
            val_names: self.val_names.clone(),
            val_delim: self.val_delim,
            possible_vals: self.possible_vals.clone(),
            default_val: self.default_val,
            validator: self.validator.clone(),
        }
    }
}

impl<'n, 'e> AnyArg<'n, 'e> for OptBuilder<'n, 'e> {
    fn name(&self) -> &'n str { self.name }
    fn overrides(&self) -> Option<&[&'e str]> { self.overrides.as_ref().map(|o| &o[..]) }
    fn requires(&self) -> Option<&[&'e str]> { self.requires.as_ref().map(|o| &o[..]) }
    fn blacklist(&self) -> Option<&[&'e str]> { self.blacklist.as_ref().map(|o| &o[..]) }
    #[cfg_attr(feature = "lints", allow(map_clone))]
    fn val_names(&self) -> Option<&VecMap<&'e str>> { self.val_names.as_ref().map(|o| o) }
    fn is_set(&self, s: ArgSettings) -> bool { self.settings.is_set(s) }
    fn has_switch(&self) -> bool { true }
    fn set(&mut self, s: ArgSettings) { self.settings.set(s) }
    fn max_vals(&self) -> Option<u64> { self.max_vals }
    fn num_vals(&self) -> Option<u64> { self.num_vals }
    fn possible_vals(&self) -> Option<&[&'e str]> { self.possible_vals.as_ref().map(|o| &o[..]) }
    fn validator(&self) -> Option<&Rc<Fn(String) -> StdResult<(), String>>> {
        self.validator.as_ref()
    }
    fn min_vals(&self) -> Option<u64> { self.min_vals }
    fn short(&self) -> Option<char> { self.short }
    fn long(&self) -> Option<&'e str> { self.long }
    fn val_delim(&self) -> Option<char> { self.val_delim }
    fn takes_value(&self) -> bool { true }
    fn help(&self) -> Option<&'e str> { self.help }
    fn default_val(&self) -> Option<&'n str> { self.default_val }
}

#[cfg(test)]
mod test {
    use super::OptBuilder;
    use vec_map::VecMap;
    use args::settings::ArgSettings;

    #[test]
    fn optbuilder_display1() {
        let mut o = OptBuilder::new("opt");
        o.long = Some("option");
        o.settings.set(ArgSettings::Multiple);

        assert_eq!(&*format!("{}", o), "--option <opt>...");
    }

    #[test]
    fn optbuilder_display2() {
        let mut v_names = VecMap::new();
        v_names.insert(0, "file");
        v_names.insert(1, "name");

        let mut o2 = OptBuilder::new("opt");
        o2.short = Some('o');
        o2.val_names = Some(v_names);

        assert_eq!(&*format!("{}", o2), "-o <file> <name>");
    }

    #[test]
    fn optbuilder_display3() {
        let mut v_names = VecMap::new();
        v_names.insert(0, "file");
        v_names.insert(1, "name");

        let mut o2 = OptBuilder::new("opt");
        o2.short = Some('o');
        o2.val_names = Some(v_names);
        o2.settings.set(ArgSettings::Multiple);

        assert_eq!(&*format!("{}", o2), "-o <file> <name>");
    }
}
