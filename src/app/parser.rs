use std::collections::{BTreeMap, HashMap, VecDeque};
use std::slice::Iter;
use std::io::{self, BufWriter, Write};
use std::ffi::{OsStr, OsString};
use std::fmt::Display;
#[cfg(feature = "debug")]
use std::os::unix::ffi::OsStrExt;

use vec_map::{self, VecMap};

use app::help::Help;
use app::App;
use args::{Arg, FlagBuilder, OptBuilder, ArgGroup, PosBuilder};
use app::settings::{AppSettings, AppFlags};
use args::{AnyArg, ArgMatcher};
use args::settings::ArgSettings;
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

#[allow(missing_debug_implementations)]
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
    #[doc(hidden)]
    pub subcommands: Vec<App<'a, 'b>>,
    groups: HashMap<&'a str, ArgGroup<'a>>,
    global_args: Vec<Arg<'a, 'b>>,
    overrides: Vec<&'b str>,
    help_short: Option<char>,
    version_short: Option<char>,
    settings: AppFlags,
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
        debug_assert!(!(self.flags.iter().any(|f| &f.name == &a.name)
            || self.opts.iter().any(|o| o.name == a.name)
            || self.positionals.values().any(|p| p.name == a.name)),
            format!("Non-unique argument name: {} is already in use", a.name));
        if let Some(grp) = a.group {
            let ag = self.groups.entry(grp).or_insert_with(|| ArgGroup::with_name(grp));
            ag.args.push(a.name);
        }
        if let Some(s) = a.short {
            debug_assert!(!self.short_list.contains(&s),
                format!("Argument short must be unique\n\n\t-{} is already in use", s));
            self.short_list.push(s);
        }
        if let Some(l) = a.long {
            debug_assert!(!self.long_list.contains(&l),
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
            debug_assert!(!self.positionals.contains_key(i),
                format!("Argument \"{}\" has the same index as another positional \
                    argument\n\n\tPerhaps try .multiple(true) to allow one positional argument \
                    to take multiple values", a.name));
            let pb = PosBuilder::from_arg(&a, i as u64, &mut self.required);
            self.positionals.insert(i, pb);
        } else if a.is_set(ArgSettings::TakesValue) {
            let mut ob = OptBuilder::from_arg(&a, &mut self.required);
            if self.settings.is_set(AppSettings::DeriveDisplayOrder) && a.disp_ord == 999 {
                ob.disp_ord = if self.settings.is_set(AppSettings::UnifiedHelpMessage) {
                    self.flags.len() + self.opts.len()
                } else {
                    self.opts.len()
                };
            }
            self.opts.push(ob);
        } else {
            let mut fb = FlagBuilder::from(a);
            if self.settings.is_set(AppSettings::DeriveDisplayOrder) && a.disp_ord == 999 {
                fb.disp_ord = if self.settings.is_set(AppSettings::UnifiedHelpMessage) {
                    self.flags.len() + self.opts.len()
                } else {
                    self.flags.len()
                };
            }
            self.flags.push(fb);
        }
        if a.is_set(ArgSettings::Global) {
            debug_assert!(!a.is_set(ArgSettings::Required),
                format!("Global arguments cannot be required.\n\n\t'{}' is marked as global and \
                        required", a.name));
            self.global_args.push(a.into());
        }
    }

    pub fn add_group(&mut self, group: ArgGroup<'a>) {
        if group.required {
            self.required.push(group.name.into());
            if let Some(ref reqs) = group.requires {
                self.required.extend_from_slice(reqs);
            }
            if let Some(ref bl) = group.conflicts {
                self.blacklist.extend_from_slice(bl);
            }
        }
        let mut found = false;
        if let Some(ref mut grp) = self.groups.get_mut(&group.name) {
            grp.args.extend_from_slice(&group.args);
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
        debugln!("fn=Parser::add_subcommand;");
        debug!("Is help...");
        if subcmd.p.meta.name == "help" {
            sdebugln!("Yes");
            self.settings.set(AppSettings::NeedsSubcommandHelp);
        } else { sdebugln!("No"); }
        debug!("Using Setting VersionlessSubcommands...");
        if self.settings.is_set(AppSettings::VersionlessSubcommands) {
            sdebugln!("Yes");
            subcmd.p.settings.set(AppSettings::DisableVersion);
        } else { sdebugln!("No"); }
        debug!("Using Setting GlobalVersion...");
        if self.settings.is_set(AppSettings::GlobalVersion) && subcmd.p.meta.version.is_none() &&
           self.meta.version.is_some() {
            sdebugln!("Yes");
            subcmd = subcmd.setting(AppSettings::GlobalVersion)
                           .version(self.meta.version.unwrap());
        } else { sdebugln!("No"); }
        if self.settings.is_set(AppSettings::DeriveDisplayOrder) {
            subcmd.p.meta.disp_ord = self.subcommands.len();
        }
        self.subcommands.push(subcmd);
    }

    pub fn required(&self) -> Iter<&str> {
        self.required.iter()
    }

    #[cfg_attr(feature = "lints", allow(for_kv_map))]
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
        macro_rules! fill_vecs {
            ($_self:ident {
                $t1:ident => $v1:ident => $i1:ident,
                $t2:ident => $v2:ident => $i2:ident,
                $t3:ident => $v3:ident => $i3:ident,
                $gv:ident, $tmp:ident
            }) => {
                for a in &$v1 {
                    if let Some(a) = self.$t1.$i1().filter(|arg| &arg.name == a).next() {
                        if let Some(ref rl) = a.requires {
                            for r in rl {
                                if !reqs.contains(r) {
                                    if $_self.$t1.$i1().any(|t| &t.name == r) {
                                        $tmp.push(*r);
                                    } else if $_self.$t2.$i2().any(|t| &t.name == r) {
                                        $v2.push(r);
                                    } else if $_self.$t3.$i3().any(|t| &t.name == r) {
                                        $v3.push(r);
                                    } else if $_self.groups.contains_key(r) {
                                        $gv.push(r);
                                    }
                                }
                            }
                        }
                    }
                }
                $v1.extend(&$tmp);
            };
        }

        let mut tmp = vec![];
        fill_vecs!(self {
            flags       => c_flags => iter,
            opts        => c_opt   => iter,
            positionals => c_pos   => values,
            grps, tmp
        });
        tmp.clear();
        fill_vecs!(self {
            opts        => c_opt   => iter,
            flags       => c_flags => iter,
            positionals => c_pos   => values,
            grps, tmp
        });
        tmp.clear();
        fill_vecs!(self {
            positionals => c_pos   => values,
            opts        => c_opt   => iter,
            flags       => c_flags => iter,
            grps, tmp
        });
        let mut ret_val = VecDeque::new();

        let mut pmap = BTreeMap::new();
        for p in c_pos.into_iter() {
            if matcher.is_some() && matcher.as_ref().unwrap().contains(p) {
                continue;
            }
            if let Some(p) = self.positionals.values().filter(|x| &x.name == &p).next() {
                pmap.insert(p.index, p.to_string());
            }
        }
        for (_, s) in pmap {
            ret_val.push_back(s);
        }
        macro_rules! write_arg {
            ($i:expr, $m:ident, $v:ident, $r:ident) => {
                for f in $v.into_iter() {
                    if $m.is_some() && $m.as_ref().unwrap().contains(f) {
                        continue;
                    }
                    $r.push_back($i.filter(|flg| &flg.name == &f).next().unwrap().to_string());
                }
            }
        }
        write_arg!(self.flags.iter(), matcher, c_flags, ret_val);
        write_arg!(self.opts.iter(), matcher, c_opt, ret_val);
        for g in grps.into_iter() {
            let g_string = self.args_in_group(g)
                               .join("|");
            ret_val.push_back(format!("[{}]", &g_string[..g_string.len()]));
        }

        ret_val
    }

    pub fn has_flags(&self) -> bool {
        !self.flags.is_empty()
    }

    pub fn has_opts(&self) -> bool {
        !self.opts.is_empty()
    }

    pub fn has_positionals(&self) -> bool {
        !self.positionals.is_empty()
    }

    pub fn has_subcommands(&self) -> bool {
        !self.subcommands.is_empty()
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
            if found {
                debug_assert!(p.settings.is_set(ArgSettings::Required),
                    "Found positional argument which is not required with a lower index than a \
                    required positional argument: {:?} index {}",
                    p.name,
                    p.index);
            } else if p.settings.is_set(ArgSettings::Required) {
                found = true;
                continue;
            }
        }
    }

    pub fn propogate_globals(&mut self) {
        for sc in &mut self.subcommands {
            // We have to create a new scope in order to tell rustc the borrow of `sc` is
            // done and to recursively call this method
            {
                for a in &self.global_args {
                    sc.p.add_arg(a);
                }
            }
            sc.p.propogate_globals();
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
                !(arg_os.len_() == 1)
            } else {
                sdebugln!("No");
                false
            };

            // Has the user already passed '--'?
            if !pos_only {
                let pos_sc = self.subcommands.iter().any(|s| &s.p.meta.name[..] == &*arg_os);
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
                    if arg_os.len_() == 2 {
                        // The user has passed '--' which means only positional args follow no matter
                        // what they start with
                        pos_only = true;
                        continue;
                    }

                    needs_val_of = try!(self.parse_long_arg(matcher, &arg_os));
                    continue;
                } else if arg_os.starts_with(b"-") && arg_os.len_() != 1 {
                    needs_val_of = try!(self.parse_short_arg(matcher, &arg_os));
                    if !(needs_val_of.is_none() && self.is_set(AppSettings::AllowLeadingHyphen)) {
                        continue;
                    }
                }

                if pos_sc {
                    if &*arg_os == "help" &&
                       self.settings.is_set(AppSettings::NeedsSubcommandHelp) {
                        let cmds: Vec<OsString> = it.map(|c| c.into()).collect();
                        let mut sc = {
                            let mut sc: &Parser = self;
                            for (i, cmd) in cmds.iter().enumerate() {
                                if let Some(c) = sc.subcommands.iter().filter(|s| &*s.p.meta.name == cmd).next().map(|sc| &sc.p) {
                                    sc = c;
                                    if i == cmds.len() - 1 {
                                        break;
                                    }
                                } else {
                                    return Err(
                                        Error::unrecognized_subcommand(
                                            cmd.to_string_lossy().into_owned(),
                                            self.meta.bin_name.as_ref().unwrap_or(&self.meta.name)));
                                }
                            }
                            sc.clone()
                        };
                        sc.create_help_and_version();
                        return sc._help();
                    }
                    subcmd_name = Some(arg_os.to_str().expect(INVALID_UTF8).to_owned());
                    break;
                } else if let Some(candidate) = suggestions::did_you_mean(
                                                        &*arg_os.to_string_lossy(),
                                                        self.subcommands.iter().map(|s| &s.p.meta.name)) {
                    return Err(
                        Error::invalid_subcommand(arg_os.to_string_lossy().into_owned(),
                                                candidate,
                                                self.meta.bin_name.as_ref().unwrap_or(&self.meta.name),
                                                &*self.create_current_usage(matcher)));
                }
            }

            if let Some(p) = self.positionals.get(pos_counter) {
                parse_positional!(self, p, arg_os, pos_only, pos_counter, matcher);
            } else {
                if self.settings.is_set(AppSettings::AllowExternalSubcommands) {
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
                        "",
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

        try!(self.add_defaults(matcher));
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
                                      .filter(|s| &s.p.meta.name == &sc_name)
                                      .next() {
            let mut sc_matcher = ArgMatcher::new();
            // bin_name should be parent's bin_name + [<reqs>] + the sc's name separated by
            // a space
            sc.p.meta.usage = Some(format!("{}{}{}",
                                    self.meta.bin_name.as_ref().unwrap_or(&String::new()),
                                    if self.meta.bin_name.is_some() {
                                        &*mid_string
                                    } else {
                                        ""
                                    },
                                    &*sc.p.meta.name));
            sc.p.meta.bin_name = Some(format!("{}{}{}",
                                       self.meta.bin_name.as_ref().unwrap_or(&String::new()),
                                       if self.meta.bin_name.is_some() {
                                           " "
                                       } else {
                                           ""
                                       },
                                       &*sc.p.meta.name));
            try!(sc.p.get_matches_with(&mut sc_matcher, it));
            matcher.subcommand(SubCommand {
                name: sc.p.meta.name.clone(),
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
                        return Some(f.to_string());
                    }
                }
            }
            if let Some(o) = self.opts.iter().filter(|o| &o.name == &k).next() {
                if let Some(ref bl) = o.blacklist {
                    if bl.contains(&name) {
                        return Some(o.to_string());
                    }
                }
            }
            if let Some(pos) = self.positionals.values().filter(|p| &p.name == &k).next() {
                if let Some(ref bl) = pos.blacklist {
                    if bl.contains(&name) {
                        return Some(pos.to_string());
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
            if self.groups.contains_key(&*n) {
                g_vec.push(*n);
            } else {
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
            debugln!("Building --help");
            if self.help_short.is_none() && !self.short_list.contains(&'h') {
                self.help_short = Some('h');
            }
            let arg = FlagBuilder {
                name: "hclap_help",
                short: self.help_short,
                long: Some("help"),
                help: Some("Prints help information"),
                ..Default::default()
            };
            self.long_list.push("help");
            self.flags.push(arg);
        }
        if !self.settings.is_set(AppSettings::DisableVersion) &&
           !self.flags.iter().any(|a| a.long.is_some() && a.long.unwrap() == "version") {
            debugln!("Building --version");
            if self.version_short.is_none() && !self.short_list.contains(&'V') {
                self.version_short = Some('V');
            }
            // name is "vclap_version" because flags are sorted by name
            let arg = FlagBuilder {
                name: "vclap_version",
                short: self.version_short,
                long: Some("version"),
                help: Some("Prints version information"),
                ..Default::default()
            };
            self.long_list.push("version");
            self.flags.push(arg);
        }
        if !self.subcommands.is_empty() &&
           !self.subcommands
                .iter()
                .any(|s| &s.p.meta.name[..] == "help") {
            debugln!("Building help");
            self.subcommands.push(App::new("help").about("Prints this message or the help of the given subcommand(s)"));
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
            sdebugln!("Help");
            try!(self._help());
        }
        if arg == "version" && self.settings.is_set(AppSettings::NeedsLongVersion) {
            sdebugln!("Version");
            try!(self._version());
        }
        sdebugln!("Neither");

        Ok(())
    }

    fn check_for_help_and_version_char(&self, arg: char) -> ClapResult<()> {
        debug!("Checking if -{} is help or version...", arg);
        if let Some(h) = self.help_short {
            sdebugln!("Help");
            if arg == h { try!(self._help()); }
        }
        if let Some(v) = self.version_short {
            sdebugln!("Help");
            if arg == v { try!(self._version()); }
        }
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
            if !opt.is_set(ArgSettings::EmptyValues) && v.len_() == 0 {
                sdebugln!("Found Empty - Error");
                return Err(Error::empty_value(opt, &*self.create_current_usage(matcher)));
            }
            sdebugln!("Found - {:?}, len: {}", v, v.len_());
            try!(self.add_val_to_arg(opt, v, matcher));
        } else { sdebugln!("None"); }

        matcher.inc_occurrence_of(opt.name);
        // Increment or create the group "args"
        self.groups_for_arg(opt.name).and_then(|vec| Some(matcher.inc_occurrences_of(&*vec)));

        if val.is_none() || (opt.is_set(ArgSettings::Multiple) && matcher.needs_more_vals(opt)) {
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
            if val.is_empty_() {
                ret = try!(self.add_single_val_to_arg(arg, val, matcher));
            } else {
                for v in val.split(delim as u32 as u8) {
                    ret = try!(self.add_single_val_to_arg(arg, v, matcher));
                }
            }
        } else {
            ret = try!(self.add_single_val_to_arg(arg, val, matcher));
        }
        Ok(ret)
    }

    fn add_single_val_to_arg<A>(&self, arg: &A, v: &OsStr, matcher: &mut ArgMatcher<'a>) -> ClapResult<Option<&'a str>>
        where A: AnyArg<'a, 'b> + Display
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
        where A: AnyArg<'a, 'b> + Display {
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
            val.is_empty_() &&
            matcher.contains(&*arg.name()) {
            return Err(Error::empty_value(arg, &*self.create_current_usage(matcher)));
        }
        if let Some(ref vtor) = arg.validator() {
            if let Err(e) = vtor(val.to_string_lossy().into_owned()) {
                return Err(Error::value_validation(e));
            }
        }
        if matcher.needs_more_vals(arg) {
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
        where A: AnyArg<'a, 'b> + Display
    {
        debugln!("fn=_validate_num_vals;");
        if let Some(num) = a.num_vals() {
            debugln!("num_vals set: {}", num);
            let should_err = if a.is_set(ArgSettings::Multiple) {
                ((ma.vals.len() as u64) % num) != 0
            } else {
                num != (ma.vals.len() as u64)
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
            if (ma.vals.len() as u64) > num {
                debugln!("Sending error TooManyValues");
                return Err(Error::too_many_values(
                    ma.vals.get(ma.vals.keys()
                                .last()
                                .expect(INTERNAL_ERROR_MSG))
                        .expect(INTERNAL_ERROR_MSG).to_str().expect(INVALID_UTF8),
                    a,
                    &*self.create_current_usage(matcher)));
            }
        }
        if let Some(num) = a.min_vals() {
            debugln!("min_vals set: {}", num);
            if (ma.vals.len() as u64) < num {
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
                if self.is_missing_required_ok(a, matcher) { continue 'outer; }
            } else if let Some(a) = self.opts.iter().filter(|o| &o.name == name).next() {
                if self.is_missing_required_ok(a, matcher) { continue 'outer; }
            } else if let Some(a) = self.positionals.values().filter(|p| &p.name == name).next() {
                if self.is_missing_required_ok(a, matcher) { continue 'outer; }
            }
            let err = if self.settings.is_set(AppSettings::ArgRequiredElseHelp) && matcher.is_empty() {
                self._help().unwrap_err()
            } else {
                let mut reqs = self.required.iter().map(|&r| &*r).collect::<Vec<_>>();
                reqs.dedup();
                Error::missing_required_argument(
                &*self.get_required_from(&*reqs, Some(matcher))
                      .iter()
                      .fold(String::new(),
                          |acc, s| acc + &format!("\n    {}", Format::Error(s))[..]),
                &*self.create_current_usage(matcher))
            };
            return Err(err);
        }
        Ok(())
    }

    fn is_missing_required_ok<A>(&self, a: &A, matcher: &ArgMatcher) -> bool where A: AnyArg<'a, 'b> {
        if let Some(bl) = a.blacklist() {
            for n in bl.iter() {
                if matcher.contains(n)
                    || self.groups
                            .get(n)
                            .map_or(false, |g| g.args.iter().any(|an| matcher.contains(an))) {
                    return true;
                }
            }
        } else if let Some(ru) = a.required_unless() {
            for n in ru.iter() {
                if matcher.contains(n)
                    || self.groups
                            .get(n)
                            .map_or(false, |g| g.args.iter().any(|an| matcher.contains(an))) {
                    if !a.is_set(ArgSettings::RequiredUnlessAll) {
                        return true;
                    }
                } else {
                    return false;
                }
            }
            return true;
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
                matcher.insert(&*opt.name);
            } else if let Some(flg) = self.flags
                                       .iter()
                                       .filter(|f| f.long.is_some() && f.long.unwrap() == name)
                                       .next() {
                self.groups_for_arg(&*flg.name).and_then(|grps| Some(matcher.inc_occurrences_of(&*grps)));
                matcher.insert(&*flg.name);
            }
        }

        let used_arg = format!("--{}", arg);
        Err(Error::unknown_argument(&*used_arg, &*suffix.0, &*self.create_current_usage(matcher)))
    }

    // Creates a usage string if one was not provided by the user manually. This happens just
    // after all arguments were parsed, but before any subcommands have been parsed
    // (so as to give subcommands their own usage recursively)
    pub fn create_usage(&self, used: &[&str]) -> String {
        debugln!("fn=create_usage;");
        let mut usage = String::with_capacity(75);
        usage.push_str("USAGE:\n    ");
        usage.push_str(&self.create_usage_no_title(&used));
        usage
    }

    // Creates a usage string (*without title*) if one was not provided by the user
    // manually. This happens just
    // after all arguments were parsed, but before any subcommands have been parsed
    // (so as to give subcommands their own usage recursively)
    pub fn create_usage_no_title(&self, used: &[&str]) -> String {
        debugln!("fn=create_usage_no_title;");
        let mut usage = String::with_capacity(75);
        if let Some(u) = self.meta.usage_str {
            usage.push_str(&*u);
        } else if used.is_empty() {
            usage.push_str(&*self.meta.usage
                                 .as_ref()
                                 .unwrap_or(self.meta.bin_name
                                                .as_ref()
                                                .unwrap_or(&self.meta.name)));
            let mut reqs: Vec<&str> = self.required().map(|r| &**r).collect();
            reqs.dedup();
            let req_string = self.get_required_from(&reqs, None)
                                 .iter()
                                 .fold(String::new(), |a, s| a + &format!(" {}", s)[..]);

            if self.has_flags() && !self.is_set(AppSettings::UnifiedHelpMessage) {
                usage.push_str(" [FLAGS]");
            } else {
                usage.push_str(" [OPTIONS]");
            }
            if !self.is_set(AppSettings::UnifiedHelpMessage)
                && self.has_opts()
                && self.opts.iter().any(|a| !a.settings.is_set(ArgSettings::Required)) {
                usage.push_str(" [OPTIONS]");
            }

            usage.push_str(&req_string[..]);

            // places a '--' in the usage string if there are args and options
            // supporting multiple values
            if self.has_positionals()
                && self.opts.iter().any(|a| a.settings.is_set(ArgSettings::Multiple))
                   // || self.positionals.values().any(|a| a.settings.is_set(ArgSettings::Multiple)))
                && self.positionals.values().any(|a| !a.settings.is_set(ArgSettings::Required))
                && !self.has_subcommands() {
                usage.push_str(" [--]")
            }
            if self.has_positionals()
                && self.positionals.values().any(|a| !a.settings.is_set(ArgSettings::Required)) {
                usage.push_str(" [ARGS]");
            }


            if self.has_subcommands() && !self.is_set(AppSettings::SubcommandRequired) {
                usage.push_str(" [SUBCOMMAND]");
            } else if self.is_set(AppSettings::SubcommandRequired) && self.has_subcommands() {
                usage.push_str(" <SUBCOMMAND>");
            }
        } else {
            self.smart_usage(&mut usage, used);
        }

        usage.shrink_to_fit();
        usage
    }

    // Creates a context aware usage string, or "smart usage" from currently used
    // args, and requirements
    fn smart_usage(&self, usage: &mut String, used: &[&str]) {
        debugln!("fn=smart_usage;");
        let mut hs: Vec<&str> = self.required().map(|s| &**s).collect();
        hs.extend_from_slice(used);

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
            if bn.contains(' ') {
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

    #[cfg_attr(feature = "lints", allow(for_kv_map))]
    pub fn write_help<W: Write>(&self, w: &mut W) -> ClapResult<()> {
        Help::write_parser_help(w, &self)
    }

    fn add_defaults(&mut self, matcher: &mut ArgMatcher<'a>) -> ClapResult<()> {
        macro_rules! add_val {
            ($_self:ident, $a:ident, $m:ident) => {
                if $m.get($a.name).is_none() {
                    try!($_self.add_val_to_arg($a, OsStr::new($a.default_val.as_ref().unwrap()), $m));
                    arg_post_processing!($_self, $a, $m);
                }
            };
        }
        for o in self.opts.iter().filter(|o| o.default_val.is_some()) {
            add_val!(self, o, matcher);
        }
        for p in self.positionals.values().filter(|p| p.default_val.is_some()) {
            add_val!(self, p, matcher);
        }
        Ok(())
    }

    pub fn iter_flags(&self) -> Iter<FlagBuilder> {
        self.flags.iter()
    }

    pub fn iter_opts(&self) -> Iter<OptBuilder> {
        self.opts.iter()
    }

    pub fn iter_positionals(&self) -> vec_map::Values<PosBuilder> {
        self.positionals.values()
    }
}

impl<'a, 'b> Clone for Parser<'a, 'b> where 'a: 'b {
    fn clone(&self) -> Self {
        Parser {
            required: self.required.clone(),
            short_list: self.short_list.clone(),
            long_list: self.long_list.clone(),
            blacklist: self.blacklist.clone(),
            flags: self.flags.clone(),
            opts: self.opts.clone(),
            positionals: self.positionals.clone(),
            subcommands: self.subcommands.clone(),
            groups: self.groups.clone(),
            global_args: self.global_args.clone(),
            overrides: self.overrides.clone(),
            help_short: self.help_short,
            version_short: self.version_short,
            settings: self.settings.clone(),
            meta: self.meta.clone(),
        }
    }
}
