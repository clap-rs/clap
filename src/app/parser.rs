use std::collections::{BTreeMap, HashMap, VecDeque};
use std::slice::Iter;
use std::io::{self, BufWriter, Write};
use std::ffi::{OsStr, OsString};
use std::fmt::Display;
#[cfg(feature = "debug")]
use std::os::unix::ffi::OsStrExt;

use vec_map::VecMap;

use app::App;
use args::{Arg, FlagBuilder, OptBuilder, ArgGroup, PosBuilder};
use app::settings::{AppSettings, AppFlags};
use args::{AnyArg, ArgMatcher};
use args::settings::{ArgSettings, ArgFlags};
use errors::{ErrorKind, Error};
use errors::Result as ClapResult;
use INVALID_UTF8;
use suggestions;
use INTERNAL_ERROR_MSG;
use SubCommand;
use fmt::Format;
use osstringext::OsStrExt2;
use app::meta::AppMeta;
use args::MatchedArg;

#[doc(hidden)]
pub struct Parser<'a, 'b> where 'a: 'b {
    required: Vec<&'b str>,
    short_list: Vec<char>,
    long_list: Vec<&'b str>,
    blacklist: Vec<&'b str>,
    // A list of possible flags
    flags: Vec<FlagBuilder<'a, 'b>>,
    // A list of possible options
    opts: Vec<OptBuilder<'a, 'b>>,
    // A list of positional arguments
    positionals: VecMap<PosBuilder<'a, 'b>>,
    // A list of subcommands
    subcommands: Vec<App<'a, 'b>>,
    groups: HashMap<&'a str, ArgGroup<'a>>,
    global_args: Vec<Arg<'a, 'b>>,
    overrides: Vec<&'b str>,
    help_short: Option<char>,
    version_short: Option<char>,
    settings: AppFlags,
    version: Option<&'b str>,
    pub meta: AppMeta<'b>,
}

impl<'a, 'b> Default for Parser<'a, 'b> {
    fn default() -> Self {
        Parser {
            flags: vec![],
            opts: vec![],
            positionals: VecMap::new(),
            subcommands: vec![],
            help_short: None,
            version_short: None,
            required: vec![],
            short_list: vec![],
            long_list: vec![],
            blacklist: vec![],
            groups: HashMap::new(),
            global_args: vec![],
            overrides: vec![],
            settings: AppFlags::new(),
            version: None,
            meta: AppMeta::new(),
        }
    }
}

macro_rules! parse_positional {
    ($_self:ident, $p:ident, $arg_os:ident, $pos_only:ident, $pos_counter:ident, $matcher:ident) => {
        debugln!("macro=parse_positional!;");
        validate_multiples!($_self, $p, $matcher);

        if let Err(e) = $_self.add_val_to_arg($p, &$arg_os, $matcher) {
            return Err(e);
        }
        if !$pos_only &&
           ($_self.settings.is_set(AppSettings::TrailingVarArg) &&
            $pos_counter == $_self.positionals.len()) {
            $pos_only = true;
        }

        $matcher.inc_occurrence_of($p.name);
        let _ = $_self.groups_for_arg($p.name).and_then(|vec| Some($matcher.inc_occurrences_of(&*vec)));
        arg_post_processing!($_self, $p, $matcher);
        // Only increment the positional counter if it doesn't allow multiples
        if !$p.settings.is_set(ArgSettings::Multiple) {
            $pos_counter += 1;
        }
    };
}


impl<'a, 'b> Parser<'a, 'b> where 'a: 'b {
    pub fn with_name(n: String) -> Self {
        Parser { meta: AppMeta::with_name(n), ..Default::default() }
    }

    pub fn help_short(&mut self, s: &str) {
        self.help_short = s.trim_left_matches(|c| c == '-')
                           .chars()
                           .nth(0);
    }

    pub fn version_short(&mut self, s: &str) {
        self.version_short = s.trim_left_matches(|c| c == '-')
                              .chars()
                              .nth(0);
    }

    // actually adds the arguments
    pub fn add_arg(&mut self, a: &Arg<'a, 'b>) {
        assert!(!(self.flags.iter().any(|f| &f.name == &a.name)
            || self.opts.iter().any(|o| o.name == a.name)
            || self.positionals.values().any(|p| p.name == a.name)),
            format!("Non-unique argument name: {} is already in use", a.name));
        if let Some(grp) = a.group {
            let ag = self.groups.entry(grp).or_insert_with(|| ArgGroup::with_name(grp));
            ag.args.push(a.name);
        }
        if let Some(s) = a.short {
            assert!(!self.short_list.contains(&s),
                format!("Argument short must be unique\n\n\t-{} is already in use", s));
            self.short_list.push(s);
        }
        if let Some(l) = a.long {
            assert!(!self.long_list.contains(&l),
                format!("Argument long must be unique\n\n\t--{} is already in use", l));
            self.long_list.push(l);
            if l == "help" {
                self.set(AppSettings::NeedsLongHelp);
            } else if l == "version" {
                self.set(AppSettings::NeedsLongVersion);
            }
        }
        if a.is_set(ArgSettings::Required) {
            self.required.push(a.name);
        }
        if a.index.is_some() || (a.short.is_none() && a.long.is_none()) {
            let i = if a.index.is_none() {
                (self.positionals.len() + 1)
            } else {
                a.index.unwrap() as usize
            };
            assert!(!self.positionals.contains_key(&i),
                format!("Argument \"{}\" has the same index as another positional \
                    argument\n\n\tPerhaps try .multiple(true) to allow one positional argument \
                    to take multiple values", a.name));
            let pb = PosBuilder::from_arg(&a, i as u8, &mut self.required);
            self.positionals.insert(i, pb);
        } else if a.is_set(ArgSettings::TakesValue) {
            let ob = OptBuilder::from_arg(&a, &mut self.required);
            self.opts.push(ob);
        } else {
            let fb = FlagBuilder::from(a);
            self.flags.push(fb);
        }
        if a.is_set(ArgSettings::Global) {
            assert!(!a.is_set(ArgSettings::Required),
                format!("Global arguments cannot be required.\n\n\t'{}' is marked as global and \
                        required", a.name));
            self.global_args.push(a.into());
        }
    }

    pub fn add_group(&mut self, group: ArgGroup<'a>) {
        if group.required {
            self.required.push(group.name.into());
            if let Some(ref reqs) = group.requires {
                self.required.extend(reqs);
            }
            if let Some(ref bl) = group.conflicts {
                self.blacklist.extend(bl);
            }
        }
        let mut found = false;
        if let Some(ref mut grp) = self.groups.get_mut(&group.name) {
            grp.args.extend(&group.args);
            grp.requires = group.requires.clone();
            grp.conflicts = group.conflicts.clone();
            grp.required = group.required;
            found = true;
        }
        if !found {
            self.groups.insert(group.name.into(), group);
        }
    }

    pub fn add_subcommand(&mut self, mut subcmd: App<'a, 'b>) {
        if subcmd.0.meta.name == "help" {
            self.settings.set(AppSettings::NeedsSubcommandHelp);
        }
        if self.settings.is_set(AppSettings::VersionlessSubcommands) {
            subcmd.0.settings.set(AppSettings::DisableVersion);
        }
        if self.settings.is_set(AppSettings::GlobalVersion) && subcmd.0.meta.version.is_none() &&
           self.version.is_some() {
            subcmd.0.meta.version = Some(self.version.unwrap());
        }
        self.subcommands.push(subcmd);
    }

    pub fn required(&self) -> Iter<&str> {
        self.required.iter()
    }

    pub fn get_required_from(&self,
                         reqs: &[&'a str],
                         matcher: Option<&ArgMatcher<'a>>)
                         -> VecDeque<String> {
        let mut c_flags: Vec<&str> = vec![];
        let mut c_pos: Vec<&str> = vec![];
        let mut c_opt: Vec<&str> = vec![];
        let mut grps: Vec<&str> = vec![];
        for name in reqs {
            if self.flags.iter().any(|f| &f.name == name) {
                c_flags.push(name);
            } else if self.opts.iter().any(|o| &o.name == name) {
                c_opt.push(name);
            } else if self.groups.contains_key(name) {
                grps.push(name);
            } else {
                c_pos.push(name);
            }
        }
        let mut tmp_f = vec![];
        for f in &c_flags {
            if let Some(f) = self.flags.iter().filter(|flg| &flg.name == f).next() {
                if let Some(ref rl) = f.requires {
                    for r in rl {
                        if !reqs.contains(r) {
                            if self.flags.iter().any(|f| &f.name == r) {
                                tmp_f.push(r);
                            } else if self.opts.iter().any(|o| &o.name == r) {
                                c_opt.push(r);
                            } else if self.groups.contains_key(r) {
                                grps.push(r);
                            } else {
                                c_pos.push(r);
                            }
                        }
                    }
                }
            }
        }
        c_flags.extend(tmp_f);
        let mut tmp_o = vec![];
        for f in &c_opt {
            if let Some(f) = self.opts.iter().filter(|o| &o.name == f).next() {
                if let Some(ref rl) = f.requires {
                    for r in rl {
                        if !reqs.contains(&r) {
                            if self.flags.iter().any(|f| &f.name == r) {
                                c_flags.push(r);
                            } else if self.opts.iter().any(|o| &o.name == r) {
                                tmp_o.push(r);
                            } else if self.groups.contains_key(r) {
                                grps.push(&r);
                            } else {
                                c_pos.push(r);
                            }
                        }
                    }
                }
            }
        }
        c_opt.extend(tmp_o);
        let mut tmp_p = vec![];
        for p in &c_pos {
            if let Some(p) = self.positionals.values().filter(|pos| &pos.name == p).next() {
                if let Some(ref rl) = p.requires {
                    for r in rl {
                        if !reqs.contains(&&**r) {
                            if self.flags.iter().any(|f| &f.name == r) {
                                c_flags.push(r);
                            } else if self.opts.iter().any(|o| &o.name == r) {
                                c_opt.push(r);
                            } else if self.groups.contains_key(r) {
                                grps.push(&&**r);
                            } else {
                                tmp_p.push(r);
                            }
                        }
                    }
                }
            }
        }
        c_pos.extend(tmp_p);

        let mut ret_val = VecDeque::new();

        let mut pmap = BTreeMap::new();
        for p in c_pos.into_iter() {
            if matcher.is_some() && matcher.as_ref().unwrap().contains(p) {
                continue;
            }
            if let Some(p) = self.positionals.values().filter(|x| &x.name == &p).next() {
                pmap.insert(p.index, format!("{}", p));
            }
        }
        for (_, s) in pmap {
            ret_val.push_back(s);
        }
        for f in c_flags.into_iter() {
            if matcher.is_some() && matcher.as_ref().unwrap().contains(f) {
                continue;
            }
            ret_val.push_back(format!("{}", self.flags
                                                .iter()
                                                .filter(|flg| &flg.name == &f)
                                                .next()
                                                .unwrap()));
        }
        for o in c_opt.into_iter() {
            if matcher.is_some() && matcher.as_ref().unwrap().contains(o) {
                continue;
            }
            ret_val.push_back(format!("{}",
                                      self.opts
                                          .iter()
                                          .filter(|opt| &opt.name == &o)
                                          .next()
                                          .unwrap()));
        }
        for g in grps.into_iter() {
            let g_string = self.args_in_group(g)
                               .join("|");
            ret_val.push_back(format!("[{}]", &g_string[..g_string.len() - 1]));
        }

        ret_val
    }

    pub fn has_flags(&self) -> bool {
        self.flags.is_empty()
    }

    pub fn has_opts(&self) -> bool {
        self.opts.is_empty()
    }

    pub fn has_positionals(&self) -> bool {
        self.positionals.is_empty()
    }

    pub fn has_subcommands(&self) -> bool {
        self.subcommands.is_empty()
    }

    pub fn is_set(&self, s: AppSettings) -> bool {
        self.settings.is_set(s)
    }

    pub fn set(&mut self, s: AppSettings) {
        self.settings.set(s)
    }

    pub fn verify_positionals(&mut self) {
        // Because you must wait until all arguments have been supplied, this is the first chance
        // to make assertions on positional argument indexes
        //
        // Firt we verify that the index highest supplied index, is equal to the number of
        // positional arguments to verify there are no gaps (i.e. supplying an index of 1 and 3
        // but no 2)
        if let Some((idx, ref p)) = self.positionals.iter().rev().next() {
            assert!(!(idx != self.positionals.len()),
                format!("Found positional argument \"{}\" who's index is {} but there are only {} \
                    positional arguments defined", p.name, idx, self.positionals.len()));
        }

        // Next we verify that only the highest index has a .multiple(true) (if any)
        assert!(!self.positionals.values()
                     .any(|a|
                         a.settings.is_set(ArgSettings::Multiple) &&
                         (a.index as usize != self.positionals.len())
                     ),
                "Only the positional argument with the highest index may accept multiple values");

        // If it's required we also need to ensure all previous positionals are
        // required too
        let mut found = false;
        for p in self.positionals.values().rev() {
            if !found {
                if p.settings.is_set(ArgSettings::Required) {
                    found = true;
                    continue;
                }
            } else {
                assert!(p.settings.is_set(ArgSettings::Required),
                    "Found positional argument which is not required with a lower index than a \
                    required positional argument: {:?} index {}",
                    p.name,
                    p.index);
            }
        }
    }

    pub fn propogate_globals(&mut self) {
        for sc in &mut self.subcommands {
            // We have to create a new scope in order to tell rustc the borrow of `sc` is
            // done and to recursively call this method
            {
                for a in &self.global_args {
                    sc.0.add_arg(a);
                }
            }
            sc.0.propogate_globals();
        }
    }

    // The actual parsing function
    #[cfg_attr(feature = "lints", allow(while_let_on_iterator))]
    pub fn get_matches_with<I, T>(&mut self,
                              matcher: &mut ArgMatcher<'a>,
                              it: &mut I)
                              -> ClapResult<()>
        where I: Iterator<Item = T>,
              T: Into<OsString>
    {
        debugln!("fn=get_matches_with;");
        // First we create the `--help` and `--version` arguments and add them if
        // necessary
        self.create_help_and_version();

        let mut pos_only = false;
        let mut subcmd_name: Option<String> = None;
        let mut needs_val_of: Option<&str> = None;
        let mut pos_counter = 1;
        while let Some(arg) = it.next() {
            let arg_os = arg.into();
            debugln!("Begin parsing '{:?}' ({:?})", arg_os, &*arg_os.as_bytes());

            // Is this a new argument, or values from a previous option?
            debug!("Starts new arg...");
            let starts_new_arg = if arg_os.starts_with(b"-") {
                sdebugln!("Yes");
                !(arg_os.len() == 1)
            } else {
                sdebugln!("No");
                false
            };

            // Has the user already passed '--'?
            if !pos_only {
                let pos_sc = self.subcommands.iter().any(|s| &s.0.meta.name[..] == &*arg_os);
                if (!starts_new_arg || self.is_set(AppSettings::AllowLeadingHyphen)) && !pos_sc {
                    // Check to see if parsing a value from an option
                    if let Some(nvo) = needs_val_of {
                        // get the OptBuilder so we can check the settings
                        if let Some(opt) = self.opts.iter().filter(|o| &o.name == &nvo).next() {
                            needs_val_of = try!(self.add_val_to_arg(opt, &arg_os, matcher));
                            // get the next value from the iterator
                            continue;
                        }
                    }
                }
                if arg_os.starts_with(b"--") {
                    if arg_os.len() == 2 {
                        // The user has passed '--' which means only positional args follow no matter
                        // what they start with
                        pos_only = true;
                        continue;
                    }

                    needs_val_of = try!(self.parse_long_arg(matcher, &arg_os));
                    continue;
                } else if arg_os.starts_with(b"-") && arg_os.len() != 1 {
                    needs_val_of = try!(self.parse_short_arg(matcher, &arg_os));
                    if !(needs_val_of.is_none() && self.is_set(AppSettings::AllowLeadingHyphen)) {
                        continue;
                    }
                }

                if pos_sc {
                    if &*arg_os == "help" &&
                       self.settings.is_set(AppSettings::NeedsSubcommandHelp) {
                        return self._help();
                    }
                    subcmd_name = Some(arg_os.to_str().expect(INVALID_UTF8).to_owned());
                    break;
                } else if let Some(candidate) = suggestions::did_you_mean(
                                                        &*arg_os.to_string_lossy(),
                                                        self.subcommands.iter().map(|s| &s.0.meta.name)) {
                    return Err(
                        Error::invalid_subcommand(arg_os.to_string_lossy().into_owned(),
                                                candidate,
                                                self.meta.bin_name.as_ref().unwrap_or(&self.meta.name),
                                                &*self.create_current_usage(matcher)));
                }
            }

            if let Some(p) = self.positionals.get(&pos_counter) {
                parse_positional!(self, p, arg_os, pos_only, pos_counter, matcher);
            } else {
                if self.settings.is_set(AppSettings::AllowExternalSubcommands) {
                    // let arg_str = arg_os.to_str().expect(INVALID_UTF8);
                    let mut sc_m = ArgMatcher::new();
                    while let Some(v) = it.next() {
                        let a = v.into();
                        if let None = a.to_str() {
                            if !self.settings.is_set(AppSettings::StrictUtf8)  {
                                return Err(
                                    Error::invalid_utf8(&*self.create_current_usage(matcher))
                                );
                            }
                        }
                        sc_m.add_val_to("EXTERNAL_SUBCOMMAND", &a);
                    }

                    matcher.subcommand(SubCommand {
                        name: "EXTERNAL_SUBCOMMAND".into(),
                        matches: sc_m.into(),
                    });
                } else {
                    return Err(Error::unknown_argument(
                        &*arg_os.to_string_lossy(),
                        "", //self.meta.bin_name.as_ref().unwrap_or(&self.meta.name),
                        &*self.create_current_usage(matcher)));
                }
            }
        }

        let mut reqs_validated = false;
        if let Some(a) = needs_val_of {
            if let Some(o) = self.opts.iter().filter(|o| &o.name == &a).next() {
                try!(self.validate_required(matcher));
                reqs_validated = true;
                let should_err = if let Some(ref v) = matcher.0.args.get(&*o.name) {
                    v.vals.is_empty() && !(o.min_vals.is_some() && o.min_vals.unwrap() == 0)
                } else {
                    true
                };
                if should_err {
                    return Err(Error::empty_value(o, &*self.create_current_usage(matcher)));
                }
            } else {
                return Err(Error::empty_value(self.positionals
                                                   .values()
                                                   .filter(|p| &p.name == &a)
                                                   .next()
                                                   .expect(INTERNAL_ERROR_MSG),
                                                &*self.create_current_usage(matcher)));
            }
        }

        try!(self.validate_blacklist(matcher));
        try!(self.validate_num_args(matcher));
        matcher.usage(self.create_usage(&[]));

        if !(self.settings.is_set(AppSettings::SubcommandsNegateReqs) && subcmd_name.is_some()) &&
            !reqs_validated {
            try!(self.validate_required(matcher));
        }
        if let Some(sc_name) = subcmd_name {
            try!(self.parse_subcommand(sc_name, matcher, it));
        } else if self.is_set(AppSettings::SubcommandRequired) {
            let bn = self.meta.bin_name.as_ref().unwrap_or(&self.meta.name);
            return Err(Error::missing_subcommand(bn, &self.create_current_usage(matcher)));
        } else if self.is_set(AppSettings::SubcommandRequiredElseHelp) {
            let mut out = vec![];
            try!(self.write_help(&mut out));
            return Err(Error {
                message: String::from_utf8_lossy(&*out).into_owned(),
                kind: ErrorKind::MissingArgumentOrSubcommand,
                info: None,
            });
        }
        if matcher.is_empty() && matcher.subcommand_name().is_none() &&
           self.is_set(AppSettings::ArgRequiredElseHelp) {
            let mut out = vec![];
            try!(self.write_help(&mut out));
            return Err(Error {
                message: String::from_utf8_lossy(&*out).into_owned(),
                kind: ErrorKind::MissingArgumentOrSubcommand,
                info: None,
            });
        }
        Ok(())
    }

    fn parse_subcommand<I, T>(&mut self, sc_name: String, matcher: &mut ArgMatcher<'a>, it: &mut I) -> ClapResult<()>
        where I: Iterator<Item = T>,
              T: Into<OsString>
    {
        use std::fmt::Write;
        debugln!("fn=parse_subcommand;");
        let mut mid_string = String::new();
        if !self.settings.is_set(AppSettings::SubcommandsNegateReqs) {
            let mut hs: Vec<&str> = self.required.iter().map(|n| &**n).collect();
            for k in matcher.arg_names() {
                hs.push(k);
            }
            let reqs = self.get_required_from(&hs, Some(matcher));

            for s in &reqs {
                write!(&mut mid_string, " {}", s).expect(INTERNAL_ERROR_MSG);
            }
        }
        mid_string.push_str(" ");
        if let Some(ref mut sc) = self.subcommands
                                      .iter_mut()
                                      .filter(|s| &s.0.meta.name == &sc_name)
                                      .next() {
            let mut sc_matcher = ArgMatcher::new();
            // bin_name should be parent's bin_name + [<reqs>] + the sc's name separated by
            // a space
            sc.0.meta.usage = Some(format!("{}{}{}",
                                    self.meta.bin_name.as_ref().unwrap_or(&String::new()),
                                    if self.meta.bin_name.is_some() {
                                        &*mid_string
                                    } else {
                                        ""
                                    },
                                    &*sc.0.meta.name));
            sc.0.meta.bin_name = Some(format!("{}{}{}",
                                       self.meta.bin_name.as_ref().unwrap_or(&String::new()),
                                       if self.meta.bin_name.is_some() {
                                           " "
                                       } else {
                                           ""
                                       },
                                       &*sc.0.meta.name));
            try!(sc.0.get_matches_with(&mut sc_matcher, it));
            matcher.subcommand(SubCommand {
                name: sc.0.meta.name.clone(),
                matches: sc_matcher.into(),
            });
        }
        Ok(())
    }

    fn blacklisted_from(&self, name: &str, matcher: &ArgMatcher) -> Option<String> {
        for k in matcher.arg_names() {
            if let Some(f) = self.flags.iter().filter(|f| &f.name == &k).next() {
                if let Some(ref bl) = f.blacklist {
                    if bl.contains(&name) {
                        return Some(format!("{}", f));
                    }
                }
            }
            if let Some(o) = self.opts.iter().filter(|o| &o.name == &k).next() {
                if let Some(ref bl) = o.blacklist {
                    if bl.contains(&name) {
                        return Some(format!("{}", o));
                    }
                }
            }
            if let Some(pos) = self.positionals.values().filter(|p| &p.name == &k).next() {
                if let Some(ref bl) = pos.blacklist {
                    if bl.contains(&name) {
                        return Some(format!("{}", pos));
                    }
                }
            }
        }
        None
    }

    fn overriden_from(&self, name: &str, matcher: &ArgMatcher) -> Option<&'a str> {
        for k in matcher.arg_names() {
            if let Some(f) = self.flags.iter().filter(|f| &f.name == &k).next() {
                if let Some(ref bl) = f.overrides {
                    if bl.contains(&name.into()) {
                        return Some(f.name);
                    }
                }
            }
            if let Some(o) = self.opts.iter().filter(|o| &o.name == &k).next() {
                if let Some(ref bl) = o.overrides {
                    if bl.contains(&name.into()) {
                        return Some(o.name);
                    }
                }
            }
            if let Some(pos) = self.positionals.values().filter(|p| &p.name == &k).next() {
                if let Some(ref bl) = pos.overrides {
                    if bl.contains(&name.into()) {
                        return Some(pos.name);
                    }
                }
            }
        }
        None
    }

    fn groups_for_arg(&self, name: &str) -> Option<Vec<&'a str>> {
        debugln!("fn=groups_for_arg;");

        if self.groups.is_empty() {
            debugln!("No groups defined");
            return None;
        }
        let mut res = vec![];
        debugln!("Searching through groups...");
        for (gn, grp) in &self.groups {
            for a in &grp.args {
                if a == &name {
                    sdebugln!("\tFound '{}'", gn);
                    res.push(*gn);
                }
            }
        }
        if res.is_empty() {
            return None;
        }

        Some(res)
    }

    fn args_in_group(&self, group: &str) -> Vec<String> {
        let mut g_vec = vec![];
        let mut args = vec![];

        for n in &self.groups.get(group).unwrap().args {
            if let Some(f) = self.flags.iter().filter(|f| &f.name == n).next() {
                args.push(f.to_string());
            } else if let Some(f) = self.opts.iter().filter(|o| &o.name == n).next() {
                args.push(f.to_string());
            } else if self.groups.contains_key(&**n) {
                g_vec.push(*n);
            } else if let Some(p) = self.positionals
                                     .values()
                                     .filter(|p| &p.name == n)
                                     .next() {
                args.push(p.to_string());
            }
        }

        for av in g_vec.iter().map(|g| self.args_in_group(g)) {
            args.extend(av);
        }
        args.dedup();
        args.iter().map(ToOwned::to_owned).collect()
    }

    fn arg_names_in_group(&self, group: &str) -> Vec<&'a str> {
        let mut g_vec = vec![];
        let mut args = vec![];

        for n in &self.groups.get(group).unwrap().args {
            if self.flags.iter().any(|f| &f.name == n) {
                args.push(*n);
            } else if self.opts.iter().any(|o| &o.name == n) {
                args.push(*n);
            } else if self.groups.contains_key(&**n) {
                g_vec.push(*n);
            } else if self.positionals.values().any(|p| &p.name == n) {
                args.push(*n);
            }
        }

        for av in g_vec.iter().map(|g| self.arg_names_in_group(g)) {
            args.extend(av);
        }
        args.dedup();
        args.iter().map(|s| *s).collect()
    }

    fn create_help_and_version(&mut self) {
        debugln!("fn=create_help_and_version;");
        // name is "hclap_help" because flags are sorted by name
        if !self.flags.iter().any(|a| a.long.is_some() && a.long.unwrap() == "help") {
            if self.help_short.is_none() && !self.short_list.contains(&'h') {
                self.help_short = Some('h');
            }
            let arg = FlagBuilder {
                name: "hclap_help".into(),
                short: self.help_short,
                long: Some("help".into()),
                help: Some("Prints help information".into()),
                blacklist: None,
                requires: None,
                overrides: None,
                settings: ArgFlags::new(),
            };
            self.long_list.push("help".into());
            self.flags.push(arg);
        }
        if !self.settings.is_set(AppSettings::DisableVersion) &&
           !self.flags.iter().any(|a| a.long.is_some() && a.long.unwrap() == "version") {
            if self.version_short.is_none() && !self.short_list.contains(&'V') {
                self.version_short = Some('V');
            }
            // name is "vclap_version" because flags are sorted by name
            let arg = FlagBuilder {
                name: "vclap_version".into(),
                short: self.version_short,
                long: Some("version".into()),
                help: Some("Prints version information".into()),
                blacklist: None,
                requires: None,
                overrides: None,
                settings: ArgFlags::new(),
            };
            self.long_list.push("version".into());
            self.flags.push(arg);
        }
        if !self.subcommands.is_empty() &&
           !self.subcommands
                .iter()
                .any(|s| &s.0.meta.name[..] == "help") {
            self.subcommands.push(App::new("help").about("Prints this message"));
        }
    }

    // Retrieves the names of all args the user has supplied thus far, except required ones
    // because those will be listed in self.required
    pub fn create_current_usage(&self, matcher: &'b ArgMatcher<'a>) -> String {
        self.create_usage(
            &*matcher.arg_names()
                   .iter()
                   .filter(|n| {
                        if let Some(o) = self.opts
                                             .iter()
                                             .filter(|&o| &&o.name == n)
                                             .next() {
                            !o.settings.is_set(ArgSettings::Required)
                        } else if let Some(p) = self.positionals
                                                    .values()
                                                    .filter(|&p| &&p.name == n)
                                                    .next() {
                            !p.settings.is_set(ArgSettings::Required)
                        } else {
                            true // flags can't be required, so they're always true
                        }})
                    .map(|&n| n)
                    .collect::<Vec<_>>())
    }

    fn check_for_help_and_version_str(&self, arg: &OsStr) -> ClapResult<()> {
        debug!("Checking if --{} is help or version...", arg.to_str().unwrap());
        if arg == "help" && self.settings.is_set(AppSettings::NeedsLongHelp) {
            try!(self._help());
        }
        if arg == "version" && self.settings.is_set(AppSettings::NeedsLongVersion) {
            try!(self._version());
        }
        sdebugln!("Neither");

        Ok(())
    }

    fn check_for_help_and_version_char(&self, arg: char) -> ClapResult<()> {
        debug!("Checking if -{} is help or version...", arg);
        if let Some(h) = self.help_short { if arg == h { try!(self._help()); } }
        if let Some(v) = self.version_short { if arg == v { try!(self._version()); } }
        sdebugln!("Neither");
        Ok(())
    }

    fn _help(&self) -> ClapResult<()> {
        try!(self.print_help());
        Err(Error {
            message: String::new(),
            kind: ErrorKind::HelpDisplayed,
            info: None,
        })
    }

    fn _version(&self) -> ClapResult<()> {
        let out = io::stdout();
        let mut buf_w = BufWriter::new(out.lock());
        try!(self.print_version(&mut buf_w));
        Err(Error {
            message: String::new(),
            kind: ErrorKind::VersionDisplayed,
            info: None,
        })
    }

    fn parse_long_arg(&mut self,
                      matcher: &mut ArgMatcher<'a>,
                      full_arg: &OsStr)
                      -> ClapResult<Option<&'b str>> { // maybe here lifetime should be 'a
        debugln!("fn=parse_long_arg;");
        let mut val = None;
        debug!("Does it contain '='...");
        let arg = if full_arg.contains_byte(b'=') {
            let (p0, p1) = full_arg.trim_left_matches(b'-').split_at_byte(b'=');
            sdebugln!("Yes '{:?}'", p1);
            val = Some(p1);
            p0
        } else {
            sdebugln!("No");
            full_arg.trim_left_matches(b'-')
        };

        if let Some(opt) = self.opts
                    .iter()
                    .filter(|v| v.long.is_some() && &*v.long.unwrap() == arg)
                    .next() {
            debugln!("Found valid opt '{}'", opt.to_string());
            let ret = try!(self.parse_opt(val, opt, matcher));
            arg_post_processing!(self, opt, matcher);

            return Ok(ret);
        } else if let Some(flag) = self.flags
                    .iter()
                    .filter(|v| v.long.is_some() && &*v.long.unwrap() == arg)
                    .next() {
            debugln!("Found valid flag '{}'", flag.to_string());
            // Only flags could be help or version, and we need to check the raw long
            // so this is the first point to check
            try!(self.check_for_help_and_version_str(&arg));

            try!(self.parse_flag(flag, matcher));

            // Handle conflicts, requirements, etc.
            arg_post_processing!(self, flag, matcher);

            return Ok(None);
        }

        debugln!("Didn't match anything");
        self.did_you_mean_error(arg.to_str().expect(INVALID_UTF8), matcher).map(|_| None)
    }

    fn parse_short_arg(&mut self,
                       matcher: &mut ArgMatcher<'a>,
                       full_arg: &OsStr)
                       -> ClapResult<Option<&'a str>> {
        debugln!("fn=parse_short_arg;");
        // let mut utf8 = true;
        let arg_os = full_arg.trim_left_matches(b'-');
        let arg = arg_os.to_string_lossy();

        for c in arg.chars() {
            // Check for matching short options, and return the name if there is no trailing
            // concatenated value: -oval
            // Option: -o
            // Value: val
            if let Some(opt) = self.opts
                        .iter()
                        .filter(|&v| v.short.is_some() && v.short.unwrap() == c)
                        .next() {
                debugln!("Found valid short opt -{} in '{}'", c, arg);
                // Check for trailing concatenated value
                let p: Vec<_> = arg.splitn(2, c).collect();
                debugln!("arg: {:?}, arg_os: {:?}, full_arg: {:?}", arg, arg_os, full_arg);
                debugln!("p[0]: {:?}, p[1]: {:?}", p[0].as_bytes(), p[1].as_bytes());
                let i = p[0].as_bytes().len() + 1;
                let val = if p[1].as_bytes().len() > 0 {
                    debugln!("setting val: {:?} (bytes), {:?} (ascii)", arg_os.split_at(i).1.as_bytes(), arg_os.split_at(i).1);
                    Some(arg_os.split_at(i).1)
                } else {
                    None
                };

                // Default to "we're expecting a value later"
                let ret = try!(self.parse_opt(val, opt, matcher));

                arg_post_processing!(self, opt, matcher);

                return Ok(ret);
            } else if let Some(flag) = self.flags.iter().filter(|&v| v.short.is_some() && v.short.unwrap() == c).next() {
                debugln!("Found valid short flag -{}", c);
                // Only flags can be help or version
                try!(self.check_for_help_and_version_char(c));
                try!(self.parse_flag(flag, matcher));
                // Handle conflicts, requirements, overrides, etc.
                // Must be called here due to mutablilty
                arg_post_processing!(self, flag, matcher);
            } else if !self.is_set(AppSettings::AllowLeadingHyphen) {
                let mut arg = String::new();
                arg.push('-');
                arg.push(c);
                return Err(
                    Error::unknown_argument(&*arg, "", &*self.create_current_usage(matcher)));
            }
        }
        Ok(None)
    }

    fn parse_opt(&self,
                 val: Option<&OsStr>,
                 opt: &OptBuilder<'a, 'b>,
                 matcher: &mut ArgMatcher<'a>)
                 -> ClapResult<Option<&'a str>> {
        debugln!("fn=parse_opt;");
        validate_multiples!(self, opt, matcher);

        debug!("Checking for val...");
        if let Some(fv) = val {
            let v = fv.trim_left_matches(b'=');
            if !opt.is_set(ArgSettings::EmptyValues) && v.len() == 0 {
                sdebugln!("Found Empty - Error");
                return Err(Error::empty_value(opt, &*self.create_current_usage(matcher)));
            }
            sdebugln!("Found - {:?}, len: {}", v, v.len());
            try!(self.add_val_to_arg(opt, v, matcher));
        } else { sdebugln!("None"); }

        matcher.inc_occurrence_of(opt.name);
        // Increment or create the group "args"
        self.groups_for_arg(opt.name).and_then(|vec| Some(matcher.inc_occurrences_of(&*vec)));

        if val.is_none() || opt.is_set(ArgSettings::Multiple) {
            return Ok(Some(opt.name));
        }
        Ok(None)
    }

    fn add_val_to_arg<A>(&self,
                        arg: &A,
                        val: &OsStr,
                        matcher: &mut ArgMatcher<'a>)
                        -> ClapResult<Option<&'a str>>
        where A: AnyArg<'a, 'b> + Display {
        debugln!("fn=add_val_to_arg;");
        let mut ret = None;
        if let Some(delim) = arg.val_delim() {
            for v in val.split(delim as u32 as u8) {
                ret = try!(self.add_single_val_to_arg(arg, v, matcher));
            }
        } else {
            ret = try!(self.add_single_val_to_arg(arg, val, matcher));
        }
        Ok(ret)
    }

    fn add_single_val_to_arg<A>(&self, arg: &A, v: &OsStr, matcher: &mut ArgMatcher<'a>) -> ClapResult<Option<&'a str>>
        where A: AnyArg<'a, 'b>
    {
        debugln!("adding val: {:?}", v);
        matcher.add_val_to(arg.name(), v);

        // Increment or create the group "args"
        if let Some(grps) = self.groups_for_arg(arg.name()) {
            for grp in grps {
                matcher.add_val_to(&*grp, v);
            }
        }

        // The validation must come AFTER inserting into 'matcher' or the usage string
        // can't be built
        self.validate_value(arg, v, matcher)
    }

    fn validate_value<A>(&self, arg: &A, val: &OsStr, matcher: &ArgMatcher<'a>) -> ClapResult<Option<&'a str>>
        where A: AnyArg<'a, 'b> {
        debugln!("fn=validate_value; val={:?}", val);
        if self.is_set(AppSettings::StrictUtf8) && val.to_str().is_none() {
            return Err(Error::invalid_utf8(&*self.create_current_usage(matcher)));
        }
        if let Some(ref p_vals) = arg.possible_vals() {
            let val_str = val.to_string_lossy();
            if !p_vals.contains(&&*val_str) {
                return Err(
                    Error::invalid_value(val_str,
                                        p_vals,
                                        arg,
                                        &*self.create_current_usage(matcher)));
            }
        }
        if !arg.is_set(ArgSettings::EmptyValues) &&
            val.is_empty() &&
            matcher.contains(&*arg.name()) {
            return Err(Error::empty_value(arg, &*self.create_current_usage(matcher)));
        }
        if let Some(ref vtor) = arg.validator() {
            if let Err(e) = vtor(val.to_string_lossy().into_owned()) {
                return Err(Error::value_validation(e));
            }
        }
        let vals = matcher.get(&*arg.name())
                          .expect(INTERNAL_ERROR_MSG)
                          .vals.len();
        if let Some(max) = arg.max_vals() {
            if (vals as u8) < max {
                return Ok(Some(arg.name()));
            } else {
                return Ok(None);
            }
        }
        if let Some(..) = arg.min_vals() {
            return Ok(Some(arg.name()));
        }
        if let Some(num) = arg.num_vals() {
            if arg.is_set(ArgSettings::Multiple) {
                if (vals as u8) < num {
                    return Ok(Some(arg.name()));
                }
            } else {
                if (vals as u8 % num) != 0 {
                    return Ok(Some(arg.name()));
                }
            }
        }
        if arg.is_set(ArgSettings::Multiple) {
            return Ok(Some(arg.name()));
        }
        Ok(None)
    }

    fn parse_flag(&self, flag: &FlagBuilder<'a, 'b>, matcher: &mut ArgMatcher<'a>) -> ClapResult<()> {
        debugln!("fn=parse_flag;");
        validate_multiples!(self, flag, matcher);

        matcher.inc_occurrence_of(flag.name);
        // Increment or create the group "args"
        self.groups_for_arg(flag.name).and_then(|vec| Some(matcher.inc_occurrences_of(&*vec)));

        Ok(())
    }

    fn validate_blacklist(&self, matcher: &mut ArgMatcher) -> ClapResult<()> {
        debugln!("fn=validate_blacklist;");
        macro_rules! build_err {
            ($me:ident, $name:expr, $matcher:ident) => ({
                debugln!("macro=build_err;");
                let c_with = $me.blacklisted_from($name, &$matcher);
                debugln!("'{:?}' conflicts with '{}'", c_with, $name);
                let usg = $me.create_current_usage($matcher);
                if let Some(f) = $me.flags.iter().filter(|f| f.name == $name).next() {
                    debugln!("It was a flag...");
                    Error::argument_conflict(f, c_with, &*usg)
                } else if let Some(o) = $me.opts.iter()
                                                 .filter(|o| o.name == $name)
                                                 .next() {
                    debugln!("It was an option...");
                    Error::argument_conflict(o, c_with, &*usg)
                } else {
                    match $me.positionals.values()
                                            .filter(|p| p.name == $name)
                                            .next() {
                        Some(p) => {
                            debugln!("It was a positional...");
                            Error::argument_conflict(p, c_with, &*usg)
                        },
                        None    => panic!(INTERNAL_ERROR_MSG)
                    }
                }
            });
        }
        for name in &self.blacklist {
            debugln!("Checking blacklisted name: {}", name);
            if self.groups.contains_key(name) {
                debugln!("groups contains it...");
                for n in self.arg_names_in_group(name) {
                    debugln!("Checking arg '{}' in group...", n);
                    if matcher.contains(&n) {
                        debugln!("matcher contains it...");
                        return Err(build_err!(self, n, matcher));
                    }
                }
            } else if matcher.contains(name) {
                debugln!("matcher contains it...");
                return Err(build_err!(self, *name, matcher));
            }
        }
        Ok(())
    }

    fn validate_num_args(&self, matcher: &mut ArgMatcher) -> ClapResult<()> {
        debugln!("fn=validate_num_args;");
        for (name, ma) in matcher.iter() {
            if self.groups.contains_key(&**name) {
                continue;
            } else {
                if let Some(opt) = self.opts
                                     .iter()
                                     .filter(|o| &o.name == name)
                                     .next() {
                    try!(self._validate_num_vals(opt, ma, matcher));
                } else if let Some(pos) = self.positionals
                                     .values()
                                     .filter(|p| &p.name == name)
                                     .next() {
                    try!(self._validate_num_vals(pos, ma, matcher));
                }
            }
        }
        Ok(())
    }

    fn _validate_num_vals<A>(&self, a: &A, ma: &MatchedArg, matcher: &ArgMatcher) -> ClapResult<()>
        where A: AnyArg<'a, 'b>
    {
        debugln!("fn=_validate_num_vals;");
        if let Some(num) = a.num_vals() {
            debugln!("num_vals set: {}", num);
            let should_err = if a.is_set(ArgSettings::Multiple) {
                ((ma.vals.len() as u8) % num) != 0
            } else {
                num != (ma.vals.len() as u8)
            };
            if should_err {
                debugln!("Sending error WrongNumberOfValues");
                return Err(Error::wrong_number_of_values(
                    a,
                    num,
                    if a.is_set(ArgSettings::Multiple) {
                        (ma.vals.len() % num as usize)
                    } else {
                        ma.vals.len()
                    },
                    if ma.vals.len() == 1 ||
                        (a.is_set(ArgSettings::Multiple) &&
                            (ma.vals.len() % num as usize) == 1) {
                        "as"
                    } else {
                        "ere"
                    },
                    &*self.create_current_usage(matcher)));
            }
        }
        if let Some(num) = a.max_vals() {
            debugln!("max_vals set: {}", num);
            if (ma.vals.len() as u8) > num {
                debugln!("Sending error TooManyValues");
                return Err(Error::too_many_values(
                    ma.vals.get(&ma.vals.keys()
                                 .last()
                                 .expect(INTERNAL_ERROR_MSG))
                        .expect(INTERNAL_ERROR_MSG).to_str().expect(INVALID_UTF8),
                    a,
                    &*self.create_current_usage(matcher)));
            }
        }
        if let Some(num) = a.min_vals() {
            debugln!("min_vals set: {}", num);
            if (ma.vals.len() as u8) < num {
                debugln!("Sending error TooFewValues");
                return Err(Error::too_few_values(
                    a,
                    num,
                    ma.vals.len(),
                    &*self.create_current_usage(matcher)));
            }
        }
        Ok(())
    }

    fn validate_required(&self, matcher: &ArgMatcher) -> ClapResult<()> {
        'outer: for name in &self.required {
            if matcher.contains(name) {
                continue 'outer;
            }
            if let Some(grp) = self.groups.get(name) {
                for arg in &grp.args {
                    if matcher.contains(arg) { continue 'outer; }
                }
            }
            if self.groups.values().any(|g| g.args.contains(name)) {
                continue 'outer;
            }
            if let Some(a) = self.flags.iter().filter(|f| &f.name == name).next() {
                if self._validate_blacklist_required(a, matcher) { continue 'outer; }
            } else if let Some(a) = self.opts.iter().filter(|o| &o.name == name).next() {
                if self._validate_blacklist_required(a, matcher) { continue 'outer; }
            } else if let Some(a) = self.positionals.values().filter(|p| &p.name == name).next() {
                if self._validate_blacklist_required(a, matcher) { continue 'outer; }
            }
            let err = if self.settings.is_set(AppSettings::ArgRequiredElseHelp) && matcher.is_empty() {
                self._help().unwrap_err()
            } else {
                Error::missing_required_argument(
                &*self.get_required_from(&*self.required.iter().map(|&r| &*r).collect::<Vec<_>>(), Some(matcher))
                      .iter()
                      .fold(String::new(),
                          |acc, s| acc + &format!("\n\t{}", Format::Error(s))[..]),
                &*self.create_current_usage(matcher))
            };
            return Err(err);
        }
        Ok(())
    }

    fn _validate_blacklist_required<A>(&self, a: &A, matcher: &ArgMatcher) -> bool where A: AnyArg<'a, 'b> {
        if let Some(bl) = a.blacklist() {
            for n in bl.iter() {
                if matcher.contains(n) {
                    return true;
                } else if self.groups
                              .get(n)
                              .map_or(false, |g| g.args.iter().any(|an| matcher.contains(an))) {
                    return true;
                }
            }
        }
        false
    }

    fn did_you_mean_error(&self, arg: &str, matcher: &mut ArgMatcher<'a>) -> ClapResult<()> {
        // Didn't match a flag or option...maybe it was a typo and close to one
        let suffix = suggestions::did_you_mean_suffix(arg,
                                              self.long_list.iter(),
                                              suggestions::DidYouMeanMessageStyle::LongFlag);

        // Add the arg to the matches to build a proper usage string
        if let Some(name) = suffix.1 {
            if let Some(opt) = self.opts
                                    .iter()
                                    .filter(|o| o.long.is_some() && o.long.unwrap() == name)
                                    .next() {
                self.groups_for_arg(&*opt.name).and_then(|grps| Some(matcher.inc_occurrences_of(&*grps)));
                matcher.insert(&*opt.name.clone());
            } else if let Some(flg) = self.flags
                                       .iter()
                                       .filter(|f| f.long.is_some() && f.long.unwrap() == name)
                                       .next() {
                self.groups_for_arg(&*flg.name).and_then(|grps| Some(matcher.inc_occurrences_of(&*grps)));
                matcher.insert(&*flg.name.clone());
            }
        }

        let used_arg = format!("--{}", arg);
        Err(Error::unknown_argument(&*used_arg, &*suffix.0, &*self.create_current_usage(matcher)))
    }

    // Creates a usage string if one was not provided by the user manually. This happens just
    // after all arguments were parsed, but before any subcommands have been parsed
    // (so as to give subcommands their own usage recursively)
    fn create_usage(&self, used: &[&str]) -> String {
        debugln!("fn=create_usage;");
        let mut usage = String::with_capacity(75);
        usage.push_str("USAGE:\n\t");
        if let Some(u) = self.meta.usage_str {
            usage.push_str(&*u);
        } else if !used.is_empty() {
            self.smart_usage(&mut usage, used);
        } else {
            usage.push_str(&*self.meta.usage
                                 .as_ref()
                                 .unwrap_or(self.meta.bin_name
                                                .as_ref()
                                                .unwrap_or(&self.meta.name)));
            let reqs: Vec<&str> = self.required().map(|r| &**r).collect();
            let req_string = self.get_required_from(&reqs, None)
                                 .iter()
                                 .fold(String::new(), |a, s| a + &format!(" {}", s)[..]);

            if !self.has_flags() && !self.is_set(AppSettings::UnifiedHelpMessage) {
                usage.push_str(" [FLAGS]");
            } else {
                usage.push_str(" [OPTIONS]");
            }
            if !self.is_set(AppSettings::UnifiedHelpMessage) && !self.has_opts() &&
               self.opts.iter().any(|a| !a.settings.is_set(ArgSettings::Required)) {
                usage.push_str(" [OPTIONS]");
            }

            usage.push_str(&req_string[..]);

            // places a '--' in the usage string if there are args and options
            // supporting multiple values
            if !self.has_positionals()
                && (self.opts.iter().any(|a| a.settings.is_set(ArgSettings::Multiple))
                    || self.positionals.values().any(|a| a.settings.is_set(ArgSettings::Multiple)))
                && !self.opts.iter().any(|a| a.settings.is_set(ArgSettings::Required))
                && self.has_subcommands() {
                usage.push_str(" [--]")
            }
            if !self.has_positionals()
                && self.positionals.values().any(|a| !a.settings.is_set(ArgSettings::Required)) {
                usage.push_str(" [ARGS]");
            }


            if !self.has_subcommands() && !self.is_set(AppSettings::SubcommandRequired) {
                usage.push_str(" [SUBCOMMAND]");
            } else if self.is_set(AppSettings::SubcommandRequired) && !self.has_subcommands() {
                usage.push_str(" <SUBCOMMAND>");
            }
        }

        usage.shrink_to_fit();
        usage
    }

    // Creates a context aware usage string, or "smart usage" from currently used
    // args, and requirements
    fn smart_usage(&self, usage: &mut String, used: &[&str]) {
        let mut hs: Vec<&str> = self.required().map(|s| &**s).collect();
        hs.extend(used);
        let r_string = self.get_required_from(&hs, None)
                                     .iter()
                                     .fold(String::new(), |acc, s| acc + &format!(" {}", s)[..]);

        usage.push_str(&self.meta.usage.as_ref()
                                  .unwrap_or(self.meta.bin_name.as_ref().unwrap_or(&self.meta.name))[..]);
        usage.push_str(&*r_string);
        if self.is_set(AppSettings::SubcommandRequired) {
            usage.push_str(" <SUBCOMMAND>");
        }
    }

    // Prints the version to the user and exits if quit=true
    fn print_version<W: Write>(&self, w: &mut W) -> ClapResult<()> {
        try!(self.write_version(w));
        w.flush().map_err(Error::from)
    }

    fn write_version<W: Write>(&self, w: &mut W) -> io::Result<()> {
        if let Some(bn) = self.meta.bin_name.as_ref() {
            if bn.contains(" ") {
                // Incase we're dealing with subcommands i.e. git mv is translated to git-mv
                writeln!(w, "{} {}", bn.replace(" ", "-"), self.meta.version.unwrap_or("".into()))
            } else {
                writeln!(w, "{} {}", &self.meta.name[..], self.meta.version.unwrap_or("".into()))
            }
        } else {
            writeln!(w, "{} {}", &self.meta.name[..], self.meta.version.unwrap_or("".into()))
        }
    }

    pub fn print_help(&self) -> ClapResult<()> {
        let out = io::stdout();
        let mut buf_w = BufWriter::new(out.lock());
        self.write_help(&mut buf_w)
    }

    pub fn write_help<W: Write>(&self, w: &mut W) -> ClapResult<()> {
        if let Some(h) = self.meta.help_str {
            return writeln!(w, "{}", h).map_err(Error::from);
        }

        // Print the version
        try!(self.write_version(w));
        if let Some(author) = self.meta.author {
            try!(write!(w, "{}\n", author));
        }
        if let Some(about) = self.meta.about {
            try!(write!(w, "{}\n", about));
        }

        try!(write!(w, "\n{}", self.create_usage(&[])));

        let flags = !self.has_flags();
        let pos = !self.has_positionals();
        let opts = !self.has_opts();
        let subcmds = !self.has_subcommands();
        let unified_help = self.is_set(AppSettings::UnifiedHelpMessage);

        let mut longest_flag = 0;
        for fl in self.flags.iter()
                      .filter(|f| f.long.is_some() && !f.settings.is_set(ArgSettings::Hidden))
                      .map(|a| a.to_string().len()) {
            if fl > longest_flag {
                longest_flag = fl;
            }
        }
        let mut longest_opt = 0;
        for ol in self.opts.iter()
                      .filter(|o| !o.settings.is_set(ArgSettings::Hidden))
                      .map(|a| a.to_string().len()) {
            if ol > longest_opt {
                longest_opt = ol;
            }
        }
        let mut longest_pos = 0;
        for pl in self.positionals
                      .values()
                      .filter(|p| !p.settings.is_set(ArgSettings::Hidden))
                      .map(|f| f.to_string().len()) {
            if pl > longest_pos {
                longest_pos = pl;
            }
        }
        let mut longest_sc = 0;
        for scl in self.subcommands
                       .iter()
                       .filter(|s| !s.0.is_set(AppSettings::Hidden))
                       .map(|s| s.0.meta.name.len()) {
            if scl > longest_sc {
                longest_sc = scl;
            }
        }

        if flags || opts || pos || subcmds {
            try!(write!(w, "\n"));
        }

        let tab = "    ";
        let longest = if !unified_help || longest_opt == 0 {
            longest_flag
        } else {
            longest_opt
        };
        if unified_help && (flags || opts) {
            try!(write!(w, "\nOPTIONS:\n"));
            let mut combined = BTreeMap::new();
            for f in self.flags.iter().filter(|f| !f.settings.is_set(ArgSettings::Hidden)) {
                let mut v = vec![];
                try!(f.write_help(&mut v, tab, longest));
                combined.insert(f.name, v);
            }
            for o in self.opts.iter().filter(|o| !o.settings.is_set(ArgSettings::Hidden)) {
                let mut v = vec![];
                try!(o.write_help(&mut v, tab, longest, self.is_set(AppSettings::HidePossibleValuesInHelp)));
                combined.insert(o.name, v);
            }
            for (_, a) in combined {
                // Only valid UTF-8 is supported, so panicing on invalid UTF-8 is ok
                try!(write!(w, "{}", unsafe { String::from_utf8_unchecked(a) }));
            }
        } else {
            if flags {
                try!(write!(w, "\nFLAGS:\n"));
                for (_, f) in self.flags.iter()
                                  .filter(|f| !f.settings.is_set(ArgSettings::Hidden))
                                  .map(|f| (f.name, f))
                                  .collect::<BTreeMap<_, _>>() {
                    try!(f.write_help(w, tab, longest));
                }
            }
            if opts {
                try!(write!(w, "\nOPTIONS:\n"));
                for (_, o) in self.opts.iter()
                                  .filter(|o| !o.settings.is_set(ArgSettings::Hidden))
                                  .map(|o| (o.name, o))
                                  .collect::<BTreeMap<_, _>>() {
                    try!(o.write_help(w, tab, longest_opt, self.is_set(AppSettings::HidePossibleValuesInHelp)));
                }
            }
        }
        if pos {
            try!(write!(w, "\nARGS:\n"));
            for v in self.positionals.values()
                         .filter(|p| !p.settings.is_set(ArgSettings::Hidden)) {
                try!(v.write_help(w, tab, longest_pos, self.is_set(AppSettings::HidePossibleValuesInHelp)));
            }
        }
        if subcmds {
            try!(write!(w, "\nSUBCOMMANDS:\n"));
            for (name, sc) in self.subcommands.iter()
                                  .filter(|s| !s.0.is_set(AppSettings::Hidden))
                                  .map(|s| (&s.0.meta.name[..], s))
                                  .collect::<BTreeMap<_, _>>() {
                try!(write!(w, "{}{}", tab, name));
                write_spaces!((longest_sc + 4) - (name.len()), w);
                if let Some(a) = sc.0.meta.about {
                    if a.contains("{n}") {
                        let mut ab = a.split("{n}");
                        while let Some(part) = ab.next() {
                            try!(write!(w, "{}\n", part));
                            write_spaces!(longest_sc + 8, w);
                            try!(write!(w, "{}", ab.next().unwrap_or("")));
                        }
                    } else {
                        try!(write!(w, "{}", a));
                    }
                }
                try!(write!(w, "\n"));
            }
        }

        if let Some(h) = self.meta.more_help {
            try!(write!(w, "\n{}", h));
        }
        w.flush().map_err(Error::from)
    }
}
