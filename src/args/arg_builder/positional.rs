// Std
use std::borrow::Cow;
use std::fmt::{Display, Formatter, Result};
use std::rc::Rc;
use std::result::Result as StdResult;
use std::ffi::{OsStr, OsString};

// Third Party
use vec_map::{self, VecMap};

// Internal
use Arg;
use args::{ArgSettings, Base, Valued, ArgKind, AnyArg, DispOrder};

#[allow(missing_debug_implementations)]
#[doc(hidden)]
#[derive(Clone, Default)]
pub struct PosBuilder<'n, 'e>
    where 'n: 'e
{
    pub b: Base<'n, 'e>,
    pub v: Valued<'n, 'e>,
    pub index: u64,
}

impl<'n, 'e> PosBuilder<'n, 'e> {
    pub fn new(name: &'n str, idx: u64) -> Self {
        PosBuilder {
            b: Base::new(name),
            index: idx,
            ..Default::default()
        }
    }

    pub fn from_arg(a: &Arg<'n, 'e>, idx: u64, reqs: &mut Vec<&'n str>) -> Self {
        // Create the Positional Argument Builder with each HashSet = None to only
        // allocate
        // those that require it
        let mut pb = PosBuilder {
            b: Base::from(a),
            v: Valued::from(a),
            index: idx,
        };
        if a.max_vals.is_some() || a.min_vals.is_some() ||
           (a.num_vals.is_some() && a.num_vals.unwrap() > 1) {
            pb.b.settings.set(ArgSettings::Multiple);
        }
        // If the arg is required, add all it's requirements to master required list
        if a.is_set(ArgSettings::Required) {
            if let Some(ref areqs) = a.requires {
                for name in areqs.iter().filter(|&&(val,_)|val.is_none()).map(|&(_, name)| name) {
                    reqs.push(name);
                }
            }
        }
        pb
    }

    pub fn multiple_str(&self) -> &str {
        if self.b.settings.is_set(ArgSettings::Multiple) && self.v.val_names.is_none() {
            "..."
        } else {
            ""
        }
    }

    pub fn name_no_brackets(&self) -> Cow<str> {
        if let Some(ref names) = self.v.val_names {
            Cow::Owned(names.values()
                .map(|n| format!("<{}>", n))
                .collect::<Vec<_>>()
                .join(" "))
        } else {
            Cow::Borrowed(self.b.name)
        }
    }
}

impl<'n, 'e> Display for PosBuilder<'n, 'e> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        if let Some(ref names) = self.v.val_names {
            try!(write!(f,
                        "{}",
                        names.values()
                            .map(|n| format!("<{}>", n))
                            .collect::<Vec<_>>()
                            .join(" ")));
        } else {
            try!(write!(f, "<{}>", self.b.name));
        }
        if self.b.settings.is_set(ArgSettings::Multiple) && self.v.val_names.is_none() {
            try!(write!(f, "..."));
        }

        Ok(())
    }
}

impl<'n, 'e> AnyArg<'n, 'e> for PosBuilder<'n, 'e> {
    fn name(&self) -> &'n str { self.b.name }
    fn id(&self) -> usize { self.b.id }
    fn kind(&self) -> ArgKind { ArgKind::Pos }
    fn overrides(&self) -> Option<&[&'e str]> { self.b.overrides.as_ref().map(|o| &o[..]) }
    fn requires(&self) -> Option<&[(Option<&'e str>, &'n str)]> { self.b.requires.as_ref().map(|o| &o[..]) }
    fn blacklist(&self) -> Option<&[&'e str]> { self.b.blacklist.as_ref().map(|o| &o[..]) }
    fn required_unless(&self) -> Option<&[&'e str]> { self.b.r_unless.as_ref().map(|o| &o[..]) }
    fn val_names(&self) -> Option<&VecMap<&'e str>> { self.v.val_names.as_ref() }
    fn is_set(&self, s: ArgSettings) -> bool { self.b.settings.is_set(s) }
    fn set(&mut self, s: ArgSettings) { self.b.settings.set(s) }
    fn has_switch(&self) -> bool { false }
    fn max_vals(&self) -> Option<u64> { self.v.max_vals }
    fn val_terminator(&self) -> Option<&'e str> { self.v.terminator }
    fn num_vals(&self) -> Option<u64> { self.v.num_vals }
    fn possible_vals(&self) -> Option<&[&'e str]> { self.v.possible_vals.as_ref().map(|o| &o[..]) }
    fn validator(&self) -> Option<&Rc<Fn(String) -> StdResult<(), String>>> {
        self.v.validator.as_ref()
    }
    fn validator_os(&self) -> Option<&Rc<Fn(&OsStr) -> StdResult<(), OsString>>> {
        self.v.validator_os.as_ref()
    }
    fn min_vals(&self) -> Option<u64> { self.v.min_vals }
    fn short(&self) -> Option<char> { None }
    fn long(&self) -> Option<&'e str> { None }
    fn val_delim(&self) -> Option<char> { self.v.val_delim }
    fn takes_value(&self) -> bool { true }
    fn help(&self) -> Option<&'e str> { self.b.help }
    fn default_vals_ifs(&self) -> Option<vec_map::Values<(&'n str, Option<&'e str>, &'e str)>> { self.v.default_vals_ifs.as_ref().map(|vm| vm.values()) }
    fn default_val(&self) -> Option<&'n str> { self.v.default_val }
    fn longest_filter(&self) -> bool { true }
    fn aliases(&self) -> Option<Vec<&'e str>> { None }
}

impl<'n, 'e> DispOrder for PosBuilder<'n, 'e> {
    fn disp_ord(&self) -> usize { self.index as usize }
}

#[cfg(test)]
mod test {
    use args::settings::ArgSettings;
    use super::PosBuilder;
    use vec_map::VecMap;

    #[test]
    fn display_mult() {
        let mut p = PosBuilder::new("pos", 1);
        p.b.settings.set(ArgSettings::Multiple);

        assert_eq!(&*format!("{}", p), "<pos>...");
    }

    #[test]
    fn display_required() {
        let mut p2 = PosBuilder::new("pos", 1);
        p2.b.settings.set(ArgSettings::Required);

        assert_eq!(&*format!("{}", p2), "<pos>");
    }

    #[test]
    fn display_val_names() {
        let mut p2 = PosBuilder::new("pos", 1);
        let mut vm = VecMap::new();
        vm.insert(0, "file1");
        vm.insert(1, "file2");
        p2.v.val_names = Some(vm);

        assert_eq!(&*format!("{}", p2), "<file1> <file2>");
    }

    #[test]
    fn display_val_names_req() {
        let mut p2 = PosBuilder::new("pos", 1);
        p2.b.settings.set(ArgSettings::Required);
        let mut vm = VecMap::new();
        vm.insert(0, "file1");
        vm.insert(1, "file2");
        p2.v.val_names = Some(vm);

        assert_eq!(&*format!("{}", p2), "<file1> <file2>");
    }
}
