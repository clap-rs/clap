use std::rc::Rc;
use std::fmt::{Display, Formatter, Result};
use std::result::Result as StdResult;
use std::io;

use vec_map::VecMap;

use args::{AnyArg, Arg};
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
    pub num_vals: Option<u8>,
    pub min_vals: Option<u8>,
    pub max_vals: Option<u8>,
    pub val_names: Option<VecMap<&'e str>>,
    pub validator: Option<Rc<Fn(String) -> StdResult<(), String>>>,
    pub overrides: Option<Vec<&'e str>>,
    pub settings: ArgFlags,
    pub val_delim: Option<char>,
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
        debugln!("fn=from_arg;");
        if a.short.is_none() && a.long.is_none() {
            panic!("Argument \"{}\" has takes_value(true), yet neither a short() or long() \
                was supplied",
                   a.name);
        }
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
            ..Default::default()
        };
        if a.is_set(ArgSettings::Multiple) {
            ob.settings.set(ArgSettings::Multiple);
        }
        if a.is_set(ArgSettings::Required) {
            ob.settings.set(ArgSettings::Required);
        }
        if a.is_set(ArgSettings::Global) {
            ob.settings.set(ArgSettings::Global);
        }
        if !a.is_set(ArgSettings::EmptyValues) {
            ob.settings.unset(ArgSettings::Global);
        }
        if a.is_set(ArgSettings::Hidden) {
            ob.settings.set(ArgSettings::Hidden);
        }
        if let Some(ref vec) = ob.val_names {
            if vec.len() > 1 {
                ob.num_vals = Some(vec.len() as u8);
            }
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
            ob.blacklist = Some(bhs);
        }
        if let Some(ref p) = a.validator {
            ob.validator = Some(p.clone());
        }
        // Check if there is anything in the requires list and add any values
        if let Some(ref r) = a.requires {
            let mut rhs = vec![];
            // without derefing n = &&str
            for n in r {
                rhs.push(*n);
                if a.is_set(ArgSettings::Required) {
                    reqs.push(*n);
                }
            }
            ob.requires = Some(rhs);
        }
        if let Some(ref or) = a.overrides {
            let mut bhs = vec![];
            // without derefing n = &&str
            for n in or {
                bhs.push(*n);
            }
            ob.overrides = Some(bhs);
        }
        // Check if there is anything in the possible values and add those as well
        if let Some(ref p) = a.possible_vals {
            let mut phs = vec![];
            // without derefing n = &&str
            for n in p {
                phs.push(*n);
            }
            ob.possible_vals = Some(phs);
        }

        ob
    }

    pub fn write_help<W: io::Write>(&self, w: &mut W, tab: &str, longest: usize) -> io::Result<()> {
        debugln!("fn=write_help");
        // if it supports multiple we add '...' i.e. 3 to the name length
        try!(write!(w, "{}", tab));
        if let Some(s) = self.short {
            try!(write!(w, "-{}", s));
        } else {
            try!(write!(w, "{}", tab));
        }
        if let Some(l) = self.long {
            try!(write!(w,
                        "{}--{}",
                        if self.short.is_some() {
                            ", "
                        } else {
                            ""
                        },
                        l));
        }
        if let Some(ref vec) = self.val_names {
            for (_, val) in vec {
                try!(write!(w, " <{}>", val));
            }
            let num = vec.len();
            if self.settings.is_set(ArgSettings::Multiple) && num == 1 {
                try!(write!(w, "..."));
            }
        } else if let Some(num) = self.num_vals {
            for _ in 0..num {
                try!(write!(w, " <{}>", self.name));
            }
        } else {
            try!(write!(w,
                        " <{}>{}",
                        self.name,
                        if self.settings.is_set(ArgSettings::Multiple) {
                            "..."
                        } else {
                            ""
                        }));
        }
        if self.long.is_some() {
            write_spaces!((longest + 4) - (self.to_string().len()), w);
        } else {
            // 8 = tab + '-a, '.len()
            write_spaces!((longest + 8) - (self.to_string().len()), w);
        }
        print_opt_help!(self, longest + 12, w);
        write!(w, "\n")
    }
}

impl<'n, 'e> Display for OptBuilder<'n, 'e> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        debugln!("fn=fmt");
        // Write the name such --long or -l
        if let Some(l) = self.long {
            try!(write!(f, "--{}", l));
        } else {
            try!(write!(f, "-{}", self.short.unwrap()));
        }

        // Write the values such as <name1> <name2>
        if let Some(ref vec) = self.val_names {
            for (_, n) in vec {
                debugln!("writing val_name: {}", n);
                try!(write!(f, " <{}>", n));
            }
            let num = vec.len();
            if self.settings.is_set(ArgSettings::Multiple) && num == 1 {
                try!(write!(f, "..."));
            }
        } else {
            let num = self.num_vals.unwrap_or(1);
            for _ in 0..num {
                try!(write!(f, " <{}>", self.name));
            }
            if self.settings.is_set(ArgSettings::Multiple) && num == 1 {
                try!(write!(f, "..."));
            }
        }

        Ok(())
    }
}

impl<'n, 'e> AnyArg<'n, 'e> for OptBuilder<'n, 'e> {
    fn name(&self) -> &'n str { self.name }
    fn overrides(&self) -> Option<&[&'e str]> { self.overrides.as_ref().map(|o| &o[..]) }
    fn requires(&self) -> Option<&[&'e str]> { self.requires.as_ref().map(|o| &o[..]) }
    fn blacklist(&self) -> Option<&[&'e str]> { self.blacklist.as_ref().map(|o| &o[..]) }
    fn is_set(&self, s: ArgSettings) -> bool { self.settings.is_set(s) }
    fn has_switch(&self) -> bool { true }
    fn set(&mut self, s: ArgSettings) { self.settings.set(s) }
    fn max_vals(&self) -> Option<u8> { self.max_vals }
    fn num_vals(&self) -> Option<u8> { self.num_vals }
    fn possible_vals(&self) -> Option<&[&'e str]> { self.possible_vals.as_ref().map(|o| &o[..]) }
    fn validator(&self) -> Option<&Rc<Fn(String) -> StdResult<(), String>>> {
        self.validator.as_ref()
    }
    fn min_vals(&self) -> Option<u8> { self.min_vals }
    fn short(&self) -> Option<char> { self.short }
    fn long(&self) -> Option<&'e str> { self.long }
    fn val_delim(&self) -> Option<char> { self.val_delim }
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
