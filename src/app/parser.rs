// Std
use std::collections::{BTreeMap, VecDeque};
use std::ffi::{OsStr, OsString};
use std::fmt::Display;
use std::fs::File;
use std::io::{self, BufWriter, Write};
#[cfg(feature = "debug")]
use std::os::unix::ffi::OsStrExt;
use std::path::PathBuf;
use std::slice::Iter;
use std::iter::Peekable;

// Third Party
use vec_map::{self, VecMap};

// Internal
use INTERNAL_ERROR_MSG;
use INVALID_UTF8;
use SubCommand;
use app::App;
use app::help::Help;
use app::meta::AppMeta;
use app::settings::AppFlags;
use args::{AnyArg, ArgMatcher, Base, Switched, Arg, ArgGroup, FlagBuilder, OptBuilder, PosBuilder,
           MatchedArg};
use args::settings::ArgSettings;
use completions::ComplGen;
use errors::{Error, ErrorKind};
use errors::Result as ClapResult;
use fmt::{Colorizer, ColorWhen};
use osstringext::OsStrExt2;
use completions::Shell;
use suggestions;
use app::settings::AppSettings as AS;

#[allow(missing_debug_implementations)]
#[doc(hidden)]
#[derive(Clone, Default)]
pub struct Parser<'a, 'b>
    where 'a: 'b
{
    pub meta: AppMeta<'b>,
    settings: AppFlags,
    pub g_settings: AppFlags,
    pub flags: Vec<FlagBuilder<'a, 'b>>,
    pub opts: Vec<OptBuilder<'a, 'b>>,
    pub positionals: VecMap<PosBuilder<'a, 'b>>,
    pub subcommands: Vec<App<'a, 'b>>,
    groups: Vec<ArgGroup<'a>>,
    pub global_args: Vec<Arg<'a, 'b>>,
    required: Vec<&'a str>,
    r_ifs: Vec<(&'a str, &'b str, &'a str)>,
    blacklist: Vec<&'b str>,
    overrides: Vec<&'b str>,
    help_short: Option<char>,
    version_short: Option<char>,
    cache: Option<&'a str>,
    help_message: Option<&'a str>,
    version_message: Option<&'a str>,
}

impl<'a, 'b> Parser<'a, 'b>
    where 'a: 'b
{
    pub fn with_name(n: String) -> Self {
        Parser { meta: AppMeta::with_name(n), ..Default::default() }
    }

    pub fn help_short(&mut self, s: &str) {
        let c = s.trim_left_matches(|c| c == '-')
            .chars()
            .nth(0)
            .unwrap_or('h');
        self.help_short = Some(c);
    }

    pub fn version_short(&mut self, s: &str) {
        let c = s.trim_left_matches(|c| c == '-')
            .chars()
            .nth(0)
            .unwrap_or('V');
        self.version_short = Some(c);
    }

    pub fn gen_completions_to<W: Write>(&mut self, for_shell: Shell, buf: &mut W) {
        if !self.is_set(AS::Propogated) {
            self.propogate_help_version();
            self.build_bin_names();
            self.propogate_globals();
            self.propogate_settings();
            self.set(AS::Propogated);
        }

        ComplGen::new(self).generate(for_shell, buf)
    }

    pub fn gen_completions(&mut self, for_shell: Shell, od: OsString) {
        use std::error::Error;

        let out_dir = PathBuf::from(od);
        let name = &*self.meta.bin_name.as_ref().unwrap().clone();
        let file_name = match for_shell {
            Shell::Bash => format!("{}.bash-completion", name),
            Shell::Fish => format!("{}.fish", name),
            Shell::Zsh => format!("_{}", name),
            Shell::PowerShell => format!("_{}.ps1", name),
        };

        let mut file = match File::create(out_dir.join(file_name)) {
            Err(why) => panic!("couldn't create completion file: {}", why.description()),
            Ok(file) => file,
        };
        self.gen_completions_to(for_shell, &mut file)
    }

    #[inline]
    fn debug_asserts(&self, a: &Arg) {
        debug_assert!(!arg_names!(self).any(|name| name == a.b.name),
                      format!("Non-unique argument name: {} is already in use", a.b.name));
        if let Some(l) = a.s.long {
            debug_assert!(!self.contains_long(l),
                          format!("Argument long must be unique\n\n\t--{} is already in use",
                                  l));
        }
        if let Some(s) = a.s.short {
            debug_assert!(!self.contains_short(s),
                          format!("Argument short must be unique\n\n\t-{} is already in use",
                                  s));
        }
        let i = if a.index.is_none() {
            (self.positionals.len() + 1)
        } else {
            a.index.unwrap() as usize
        };
        debug_assert!(!self.positionals.contains_key(i),
                      format!("Argument \"{}\" has the same index as another positional \
                    argument\n\n\tPerhaps try .multiple(true) to allow one positional argument \
                    to take multiple values",
                              a.b.name));
        debug_assert!(!(a.is_set(ArgSettings::Required) && a.is_set(ArgSettings::Global)),
                      format!("Global arguments cannot be required.\n\n\t'{}' is marked as \
                          global and required",
                              a.b.name));
    }

    #[inline]
    fn add_conditional_reqs(&mut self, a: &Arg<'a, 'b>) {
        if let Some(ref r_ifs) = a.r_ifs {
            for &(arg, val) in r_ifs {
                self.r_ifs.push((arg, val, a.b.name));
            }
        }
    }

    #[inline]
    fn add_arg_groups(&mut self, a: &Arg<'a, 'b>) {
        if let Some(ref grps) = a.b.groups {
            for g in grps {
                let mut found = false;
                if let Some(ref mut ag) = self.groups.iter_mut().find(|grp| &grp.name == g) {
                    ag.args.push(a.b.name);
                    found = true;
                }
                if !found {
                    let mut ag = ArgGroup::with_name(g);
                    ag.args.push(a.b.name);
                    self.groups.push(ag);
                }
            }
        }
    }

    #[inline]
    fn add_reqs(&mut self, a: &Arg<'a, 'b>) {
        if a.is_set(ArgSettings::Required) {
            // If the arg is required, add all it's requirements to master required list
            if let Some(ref areqs) = a.b.requires {
                for name in areqs.iter().filter(|&&(val, _)| val.is_none()).map(|&(_, name)| name) {
                    self.required.push(name);
                }
            }
            self.required.push(a.b.name);
        }
    }

    // actually adds the arguments
    pub fn add_arg(&mut self, a: Arg<'a, 'b>) {
        // if it's global we have to clone anyways
        if a.is_set(ArgSettings::Global) {
            return self.add_arg_ref(&a);
        }
        self.debug_asserts(&a);
        self.add_conditional_reqs(&a);
        self.add_arg_groups(&a);
        self.add_reqs(&a);
        if a.index.is_some() || (a.s.short.is_none() && a.s.long.is_none()) {
            let i = if a.index.is_none() {
                (self.positionals.len() + 1)
            } else {
                a.index.unwrap() as usize
            };
            self.positionals.insert(i, PosBuilder::from_arg(a, i as u64));
        } else if a.is_set(ArgSettings::TakesValue) {
            let mut ob = OptBuilder::from(a);
            ob.s.unified_ord = self.flags.len() + self.opts.len();
            self.opts.push(ob);
        } else {
            let mut fb = FlagBuilder::from(a);
            fb.s.unified_ord = self.flags.len() + self.opts.len();
            self.flags.push(fb);
        }
    }
    // actually adds the arguments but from a borrow (which means we have to do some clonine)
    pub fn add_arg_ref(&mut self, a: &Arg<'a, 'b>) {
        self.debug_asserts(&a);
        self.add_conditional_reqs(&a);
        self.add_arg_groups(&a);
        self.add_reqs(&a);
        if a.index.is_some() || (a.s.short.is_none() && a.s.long.is_none()) {
            let i = if a.index.is_none() {
                (self.positionals.len() + 1)
            } else {
                a.index.unwrap() as usize
            };
            let pb = PosBuilder::from_arg_ref(a, i as u64);
            self.positionals.insert(i, pb);
        } else if a.is_set(ArgSettings::TakesValue) {
            let mut ob = OptBuilder::from(a);
            ob.s.unified_ord = self.flags.len() + self.opts.len();
            self.opts.push(ob);
        } else {
            let mut fb = FlagBuilder::from(a);
            fb.s.unified_ord = self.flags.len() + self.opts.len();
            self.flags.push(fb);
        }
        if a.is_set(ArgSettings::Global) {
            self.global_args.push(a.into());
        }
    }

    pub fn add_group(&mut self, group: ArgGroup<'a>) {
        if group.required {
            self.required.push(group.name);
            if let Some(ref reqs) = group.requires {
                self.required.extend_from_slice(reqs);
            }
            if let Some(ref bl) = group.conflicts {
                self.blacklist.extend_from_slice(bl);
            }
        }
        if self.groups.iter().any(|g| g.name == group.name) {
            let grp =
                self.groups.iter_mut().find(|g| g.name == group.name).expect(INTERNAL_ERROR_MSG);
            grp.args.extend_from_slice(&group.args);
            grp.requires = group.requires.clone();
            grp.conflicts = group.conflicts.clone();
            grp.required = group.required;
        } else {
            self.groups.push(group);
        }
    }

    pub fn add_subcommand(&mut self, mut subcmd: App<'a, 'b>) {
        debugln!("Parser::add_subcommand: term_w={:?}, name={}",
                 self.meta.term_w,
                 subcmd.p.meta.name);
        subcmd.p.meta.term_w = self.meta.term_w;
        if subcmd.p.meta.name == "help" {
            self.unset(AS::NeedsSubcommandHelp);
        }

        self.subcommands.push(subcmd);
    }

    pub fn propogate_settings(&mut self) {
        debugln!("Parser::propogate_settings: self={}, g_settings={:#?}",
                 self.meta.name,
                 self.g_settings);
        for sc in &mut self.subcommands {
            debugln!("Parser::propogate_settings: sc={}, settings={:#?}, g_settings={:#?}",
                     sc.p.meta.name,
                     sc.p.settings,
                     sc.p.g_settings);
            // We have to create a new scope in order to tell rustc the borrow of `sc` is
            // done and to recursively call this method
            {
                let vsc = self.settings.is_set(AS::VersionlessSubcommands);
                let gv = self.settings.is_set(AS::GlobalVersion);

                if vsc {
                    sc.p.set(AS::DisableVersion);
                }
                if gv && sc.p.meta.version.is_none() && self.meta.version.is_some() {
                    sc.p.set(AS::GlobalVersion);
                    sc.p.meta.version = Some(self.meta.version.unwrap());
                }
                sc.p.settings = sc.p.settings | self.g_settings;
                sc.p.g_settings = sc.p.g_settings | self.g_settings;
            }
            sc.p.propogate_settings();
        }
    }

    #[cfg_attr(feature = "lints", allow(needless_borrow))]
    pub fn derive_display_order(&mut self) {
        if self.is_set(AS::DeriveDisplayOrder) {
            let unified = self.is_set(AS::UnifiedHelpMessage);
            for (i, o) in self.opts
                .iter_mut()
                .enumerate()
                .filter(|&(_, ref o)| o.s.disp_ord == 999) {
                o.s.disp_ord = if unified { o.s.unified_ord } else { i };
            }
            for (i, f) in self.flags
                .iter_mut()
                .enumerate()
                .filter(|&(_, ref f)| f.s.disp_ord == 999) {
                f.s.disp_ord = if unified { f.s.unified_ord } else { i };
            }
            for (i, sc) in &mut self.subcommands
                .iter_mut()
                .enumerate()
                .filter(|&(_, ref sc)| sc.p.meta.disp_ord == 999) {
                sc.p.meta.disp_ord = i;
            }
        }
        for sc in &mut self.subcommands {
            sc.p.derive_display_order();
        }
    }

    pub fn required(&self) -> Iter<&str> { self.required.iter() }

    #[cfg_attr(feature = "lints", allow(needless_borrow))]
    pub fn get_required_from(&self,
                             reqs: &[&'a str],
                             matcher: Option<&ArgMatcher<'a>>,
                             extra: Option<&str>)
                             -> VecDeque<String> {
        debugln!("Parser::get_required_from: reqs={:?}, extra={:?}",
                 reqs,
                 extra);
        let mut desc_reqs: Vec<&str> = vec![];
        desc_reqs.extend(extra);
        let mut new_reqs: Vec<&str> = vec![];
        macro_rules! get_requires {
            (@group $a: ident, $v:ident, $p:ident) => {{
                if let Some(rl) = self.groups.iter()
                                             .filter(|g| g.requires.is_some())
                                             .find(|g| &g.name == $a)
                                             .map(|g| g.requires.as_ref().unwrap()) {
                    for r in rl {
                        if !$p.contains(&r) {
                            debugln!("Parser::get_required_from:iter:{}: adding group req={:?}",
                                $a, r);
                            $v.push(r);
                        }
                    }
                }
            }};
            ($a:ident, $what:ident, $how:ident, $v:ident, $p:ident) => {{
                if let Some(rl) = self.$what.$how()
                                            .filter(|a| a.b.requires.is_some())
                                            .find(|arg| &arg.b.name == $a)
                                            .map(|a| a.b.requires.as_ref().unwrap()) {
                    for &(_, r) in rl.iter() {
                        if !$p.contains(&r) {
                            debugln!("Parser::get_required_from:iter:{}: adding arg req={:?}",
                                $a, r);
                            $v.push(r);
                        }
                    }
                }
            }};
        }
        // initialize new_reqs
        for a in reqs {
            get_requires!(a, flags, iter, new_reqs, reqs);
            get_requires!(a, opts, iter, new_reqs, reqs);
            get_requires!(a, positionals, values, new_reqs, reqs);
            get_requires!(@group a, new_reqs, reqs);
        }
        desc_reqs.extend_from_slice(&*new_reqs);
        debugln!("Parser::get_required_from: after init desc_reqs={:?}",
                 desc_reqs);
        loop {
            let mut tmp = vec![];
            for a in &new_reqs {
                get_requires!(a, flags, iter, tmp, desc_reqs);
                get_requires!(a, opts, iter, tmp, desc_reqs);
                get_requires!(a, positionals, values, tmp, desc_reqs);
                get_requires!(@group a, tmp, desc_reqs);
            }
            if tmp.is_empty() {
                debugln!("Parser::get_required_from: no more children");
                break;
            } else {
                debugln!("Parser::get_required_from: after iter tmp={:?}", tmp);
                debugln!("Parser::get_required_from: after iter new_reqs={:?}",
                         new_reqs);
                desc_reqs.extend_from_slice(&*new_reqs);
                new_reqs.clear();
                new_reqs.extend_from_slice(&*tmp);
                debugln!("Parser::get_required_from: after iter desc_reqs={:?}",
                         desc_reqs);
            }
        }
        desc_reqs.extend_from_slice(reqs);
        desc_reqs.sort();
        desc_reqs.dedup();
        debugln!("Parser::get_required_from: final desc_reqs={:?}", desc_reqs);
        let mut ret_val = VecDeque::new();
        let args_in_groups = self.groups
            .iter()
            .filter(|gn| desc_reqs.contains(&gn.name))
            .flat_map(|g| self.arg_names_in_group(&g.name))
            .collect::<Vec<_>>();

        let pmap = if let Some(ref m) = matcher {
            desc_reqs.iter()
                .filter(|a| self.positionals.values().any(|p| &&p.b.name == a))
                .filter(|&p| !m.contains(p))
                .filter_map(|p| self.positionals.values().find(|x| &x.b.name == p))
                .filter(|p| !args_in_groups.contains(&p.b.name))
                .map(|p| (p.index, p))
                .collect::<BTreeMap<u64, &PosBuilder>>() // sort by index
        } else {
            desc_reqs.iter()
                .filter(|a| self.positionals.values().any(|p| &&p.b.name == a))
                .filter_map(|p| self.positionals.values().find(|x| &x.b.name == p))
                .filter(|p| !args_in_groups.contains(&p.b.name))
                .map(|p| (p.index, p))
                .collect::<BTreeMap<u64, &PosBuilder>>() // sort by index
        };
        debugln!("Parser::get_required_from: args_in_groups={:?}",
                 args_in_groups);
        for &p in pmap.values() {
            let s = p.to_string();
            if args_in_groups.is_empty() || !args_in_groups.contains(&&*s) {
                ret_val.push_back(s);
            }
        }
        for a in desc_reqs.iter()
            .filter(|name| !self.positionals.values().any(|p| &&p.b.name == name))
            .filter(|name| !self.groups.iter().any(|g| &&g.name == name))
            .filter(|name| !args_in_groups.contains(name))
            .filter(|name| !(matcher.is_some() && matcher.as_ref().unwrap().contains(name))) {
            debugln!("Parser::get_required_from:iter:{}:", a);
            let arg = find_by_name!(self, a, flags, iter)
                .map(|f| f.to_string())
                .unwrap_or_else(|| {
                    find_by_name!(self, a, opts, iter)
                        .map(|o| o.to_string())
                        .expect(INTERNAL_ERROR_MSG)
                });
            ret_val.push_back(arg);
        }
        let mut g_vec = vec![];
        for g in desc_reqs.iter().filter(|n| self.groups.iter().any(|g| &&g.name == n)) {
            let g_string = self.args_in_group(g)
                .join("|");
            g_vec.push(format!("<{}>", &g_string[..g_string.len()]));
        }
        g_vec.sort();
        g_vec.dedup();
        for g in g_vec {
            ret_val.push_back(g);
        }

        ret_val
    }

    // Gets the `[ARGS]` tag for the usage string
    pub fn get_args_tag(&self) -> Option<String> {
        debugln!("Parser::get_args_tag;");
        let mut count = 0;
        'outer: for p in self.positionals.values().filter(|p| !p.is_set(ArgSettings::Required) && 
            !p.is_set(ArgSettings::Hidden)) {
            debugln!("Parser::get_args_tag:iter:{}:", p.b.name);
            if let Some(g_vec) = self.groups_for_arg(p.b.name) {
                for grp_s in &g_vec {
                    debugln!("Parser::get_args_tag:iter:{}:iter:{};", p.b.name, grp_s);
                    // if it's part of a required group we don't want to count it
                    if self.groups.iter().any(|g| g.required && (&g.name == grp_s)) {
                        continue 'outer;
                    }
                }
            }
            count += 1;
            debugln!("Parser::get_args_tag:iter: {} Args not required or hidden", count);
        }
        if !self.is_set(AS::DontCollapseArgsInUsage) && count > 1 {
            return None; // [ARGS]
        } else if count == 1 {
            let p = self.positionals
                .values()
                .find(|p| !p.is_set(ArgSettings::Required) && !p.is_set(ArgSettings::Hidden))
                .expect(INTERNAL_ERROR_MSG);
            return Some(format!(" [{}]{}", p.name_no_brackets(), p.multiple_str()));
        } else if self.is_set(AS::DontCollapseArgsInUsage) && !self.positionals.is_empty() {
            return Some(self.positionals
                .values()
                .filter(|p| !p.is_set(ArgSettings::Required))
                .filter(|p| !p.is_set(ArgSettings::Hidden))
                .map(|p| format!(" [{}]{}", p.name_no_brackets(), p.multiple_str()))
                .collect::<Vec<_>>()
                .join(""));
        }
        Some("".into())
    }

    // Determines if we need the `[FLAGS]` tag in the usage string
    pub fn needs_flags_tag(&self) -> bool {
        debugln!("Parser::needs_flags_tag;");
        'outer: for f in &self.flags {
            debugln!("Parser::needs_flags_tag:iter: f={};", f.b.name);
            if let Some(l) = f.s.long {
                if l == "help" || l == "version" {
                    // Don't print `[FLAGS]` just for help or version
                    continue;
                }
            }
            if let Some(g_vec) = self.groups_for_arg(f.b.name) {
                for grp_s in &g_vec {
                    debugln!("Parser::needs_flags_tag:iter:iter: grp_s={};", grp_s);
                    if self.groups.iter().any(|g| &g.name == grp_s && g.required) {
                        debug!("Parser::needs_flags_tag:iter:iter: Group is required");
                        continue 'outer;
                    }
                }
            }
            if f.is_set(ArgSettings::Hidden) {
                continue;
            }
            debugln!("Parser::needs_flags_tag:iter: [FLAGS] required");
            return true;
        }

        debugln!("Parser::needs_flags_tag: [FLAGS] not required");
        false
    }

    #[inline]
    pub fn has_args(&self) -> bool {
        !(self.flags.is_empty() && self.opts.is_empty() && self.positionals.is_empty())
    }

    #[inline]
    pub fn has_opts(&self) -> bool { !self.opts.is_empty() }

    #[inline]
    pub fn has_flags(&self) -> bool { !self.flags.is_empty() }

    #[inline]
    pub fn has_positionals(&self) -> bool { !self.positionals.is_empty() }
    
    #[inline]
    pub fn has_subcommands(&self) -> bool { !self.subcommands.is_empty() }

    #[inline]
    pub fn has_visible_opts(&self) -> bool { 
        if self.opts.is_empty() { return false; }  
        self.opts.iter().any(|o| !o.is_set(ArgSettings::Hidden))
    }

    #[inline]
    pub fn has_visible_flags(&self) -> bool { 
        if self.flags.is_empty() { return false; }  
        self.flags.iter().any(|f| !f.is_set(ArgSettings::Hidden))
    }

    #[inline]
    pub fn has_visible_positionals(&self) -> bool { 
        if self.positionals.is_empty() { return false; }  
        self.positionals.values().any(|p| !p.is_set(ArgSettings::Hidden))
    }

    #[inline]
    pub fn has_visible_subcommands(&self) -> bool { 
        if self.subcommands.is_empty() { return false; }  
        self.subcommands.iter().any(|s| !s.p.is_set(AS::Hidden))
    }

    #[inline]
    pub fn is_set(&self, s: AS) -> bool { self.settings.is_set(s) }

    #[inline]
    pub fn set(&mut self, s: AS) { self.settings.set(s) }

    #[inline]
    pub fn unset(&mut self, s: AS) { self.settings.unset(s) }

    #[cfg_attr(feature = "lints", allow(block_in_if_condition_stmt))]
    pub fn verify_positionals(&mut self) {
        // Because you must wait until all arguments have been supplied, this is the first chance
        // to make assertions on positional argument indexes
        //
        // Firt we verify that the index highest supplied index, is equal to the number of
        // positional arguments to verify there are no gaps (i.e. supplying an index of 1 and 3
        // but no 2)
        if let Some((idx, p)) = self.positionals.iter().rev().next() {
            debug_assert!(!(idx != self.positionals.len()),
                          format!("Found positional argument \"{}\" who's index is {} but there \
                          are only {} positional arguments defined",
                                  p.b.name,
                                  idx,
                                  self.positionals.len()));
        }

        // Next we verify that only the highest index has a .multiple(true) (if any)
        if 
            self.positionals
                .values()
                .any(|a| {
                    a.is_set(ArgSettings::Multiple) && (a.index as usize != self.positionals.len())
                }) {

            debug_assert!({
                              let mut it = self.positionals.values().rev();
                              // Either the final positional is required
                              it.next().unwrap().is_set(ArgSettings::Required)
                    // Or the second to last has a terminator set
                    || it.next().unwrap().v.terminator.is_some()
                          },
                          "When using a positional argument with .multiple(true) that is *not the \
                          last* positional argument, the last positional argument (i.e the one \
                          with the highest index) *must* have .required(true) set.");

            debug_assert!({
                              let num = self.positionals.len() - 1;
                              self.positionals.get(num).unwrap().is_set(ArgSettings::Multiple)
                          },
                          "Only the last positional argument, or second to last positional \
                          argument may be set to .multiple(true)");

            self.set(AS::LowIndexMultiplePositional);
        }

        debug_assert!(self.positionals
                          .values()
                          .filter(|p| {
                              p.b.settings.is_set(ArgSettings::Multiple) && p.v.num_vals.is_none()
                          })
                          .map(|_| 1)
                          .sum::<u64>() <= 1,
                      "Only one positional argument with .multiple(true) set is allowed per \
                      command");

        // If it's required we also need to ensure all previous positionals are
        // required too
        if self.is_set(AS::AllowMissingPositional) {
            let mut found = false;
            let mut foundx2 = false;
            for p in self.positionals.values().rev() {
                if foundx2 && !p.b.settings.is_set(ArgSettings::Required) {
                    // [arg1] <arg2> is Ok
                    // [arg1] <arg2> <arg3> Is not
                    debug_assert!(p.b.settings.is_set(ArgSettings::Required),
                                  "Found positional argument which is not required with a lower \
                                  index than a required positional argument by two or more: {:?} \
                                  index {}",
                                  p.b.name,
                                  p.index);
                } else if p.b.settings.is_set(ArgSettings::Required) {
                    if found {
                        foundx2 = true;
                        continue;
                    }
                    found = true;
                    continue;
                } else {
                    found = false;
                }
            }
        } else {
            let mut found = false;
            for p in self.positionals.values().rev() {
                if found {
                    debug_assert!(p.b.settings.is_set(ArgSettings::Required),
                                  "Found positional argument which is not required with a lower \
                                  index than a required positional argument: {:?} index {}",
                                  p.b.name,
                                  p.index);
                } else if p.b.settings.is_set(ArgSettings::Required) {
                    found = true;
                    continue;
                }
            }
        }
    }

    pub fn propogate_globals(&mut self) {
        for sc in &mut self.subcommands {
            // We have to create a new scope in order to tell rustc the borrow of `sc` is
            // done and to recursively call this method
            {
                for a in &self.global_args {
                    sc.p.add_arg_ref(a);
                }
            }
            sc.p.propogate_globals();
        }
    }

    // Checks if the arg matches a subcommand name, or any of it's aliases (if defined)
    fn possible_subcommand(&self, arg_os: &OsStr) -> (bool, Option<&str>) {
        debugln!("Parser::possible_subcommand: arg={:?}", arg_os);
        fn starts(h: &str, n: &OsStr) -> bool {
            #[cfg(not(target_os = "windows"))]
            use std::os::unix::ffi::OsStrExt;
            #[cfg(target_os = "windows")]
            use ossstringext::OsStrExt3;

            let n_bytes = n.as_bytes();
            let h_bytes = OsStr::new(h).as_bytes();

            h_bytes.starts_with(n_bytes)
        }

        if self.is_set(AS::ArgsNegateSubcommands) && self.is_set(AS::ValidArgFound) {
            return (false, None);
        }
        if !self.is_set(AS::InferSubcommands) {
            if let Some(sc) = find_subcmd!(self, arg_os) {
                return (true, Some(&sc.p.meta.name));
            }
        } else {
            let v = self.subcommands
                .iter()
                .filter(|s| {
                    starts(&s.p.meta.name[..], &*arg_os) ||
                    (s.p.meta.aliases.is_some() &&
                     s.p
                        .meta
                        .aliases
                        .as_ref()
                        .unwrap()
                        .iter()
                        .filter(|&&(a, _)| starts(a, &*arg_os))
                        .count() == 1)
                })
                .map(|sc| &sc.p.meta.name)
                .collect::<Vec<_>>();

            if v.len() == 1 {
                return (true, Some(v[0]));
            }
        }
        return (false, None);
    }

    fn parse_help_subcommand<I, T>(&self, it: &mut I) -> ClapResult<()>
        where I: Iterator<Item = T>,
              T: Into<OsString>
    {
        debugln!("Parser::parse_help_subcommand;");
        let cmds: Vec<OsString> = it.map(|c| c.into()).collect();
        let mut help_help = false;
        let mut bin_name = self.meta
            .bin_name
            .as_ref()
            .unwrap_or(&self.meta.name)
            .clone();
        let mut sc = {
            let mut sc: &Parser = self;
            for (i, cmd) in cmds.iter().enumerate() {
                if &*cmd.to_string_lossy() == "help" {
                    // cmd help help
                    help_help = true;
                }
                if let Some(c) = sc.subcommands
                    .iter()
                    .find(|s| &*s.p.meta.name == cmd)
                    .map(|sc| &sc.p) {
                    sc = c;
                    if i == cmds.len() - 1 {
                        break;
                    }
                } else if let Some(c) = sc.subcommands
                    .iter()
                    .find(|s| if let Some(ref als) = s.p
                        .meta
                        .aliases {
                        als.iter()
                            .any(|&(a, _)| &a == &&*cmd.to_string_lossy())
                    } else {
                        false
                    })
                    .map(|sc| &sc.p) {
                    sc = c;
                    if i == cmds.len() - 1 {
                        break;
                    }
                } else {
                    return Err(Error::unrecognized_subcommand(cmd.to_string_lossy().into_owned(),
                                                              self.meta
                                                                  .bin_name
                                                                  .as_ref()
                                                                  .unwrap_or(&self.meta.name),
                                                              self.color()));
                }
                bin_name = format!("{} {}", bin_name, &*sc.meta.name);
            }
            sc.clone()
        };
        if help_help {
            let mut pb = PosBuilder::new("subcommand", 1);
            pb.b.help = Some("The subcommand whose help message to display");
            pb.set(ArgSettings::Multiple);
            sc.positionals.insert(1, pb);
            sc.settings = sc.settings | self.g_settings;
        } else {
            sc.create_help_and_version();
        }
        if sc.meta.bin_name != self.meta.bin_name {
            sc.meta.bin_name = Some(format!("{} {}", bin_name, sc.meta.name));
        }
        sc._help()
    }

    // allow wrong self convention due to self.valid_neg_num = true and it's a private method
    #[cfg_attr(feature = "lints", allow(wrong_self_convention))]
    #[inline]
    fn is_new_arg(&mut self, arg_os: &OsStr, needs_val_of: Option<&'a str>) -> bool {
        debugln!("Parser::is_new_arg: arg={:?}, Needs Val of={:?}",
                 arg_os,
                 needs_val_of);
        let app_wide_settings = if self.is_set(AS::AllowLeadingHyphen) {
            true
        } else if self.is_set(AS::AllowNegativeNumbers) {
            let a = arg_os.to_string_lossy();
            if a.parse::<i64>().is_ok() || a.parse::<f64>().is_ok() {
                self.set(AS::ValidNegNumFound);
                true
            } else {
                false
            }
        } else {
            false
        };
        let arg_allows_tac = if let Some(name) = needs_val_of {
            if let Some(o) = find_by_name!(self, &name, opts, iter) {
                (o.is_set(ArgSettings::AllowLeadingHyphen) || app_wide_settings)
            } else if let Some(p) = find_by_name!(self, &name, positionals, values) {
                (p.is_set(ArgSettings::AllowLeadingHyphen) || app_wide_settings)
            } else {
                false
            }
        } else {
            false
        };
        debugln!("Parser::is_new_arg: Arg::allow_leading_hyphen({:?})",
                 arg_allows_tac);

        // Is this a new argument, or values from a previous option?
        let mut ret = if arg_os.starts_with(b"--") {
            debugln!("Parser::is_new_arg: -- found");
            if arg_os.len_() == 2 {
                return true; // We have to return true so override everything else
            }
            true
        } else if arg_os.starts_with(b"-") {
            debugln!("Parser::is_new_arg: - found");
            // a singe '-' by itself is a value and typically means "stdin" on unix systems
            !(arg_os.len_() == 1)
        } else {
            debugln!("Parser::is_new_arg: probably value");
            false
        };

        ret = ret && !arg_allows_tac;

        debugln!("Parser::is_new_arg: starts_new_arg={:?}", ret);
        ret
    }

    // The actual parsing function
    #[cfg_attr(feature = "lints", allow(while_let_on_iterator, collapsible_if))]
    pub fn get_matches_with<I, T>(&mut self,
                                  matcher: &mut ArgMatcher<'a>,
                                  it: &mut Peekable<I>)
                                  -> ClapResult<()>
        where I: Iterator<Item = T>,
              T: Into<OsString> + Clone
    {
        debugln!("Parser::get_matches_with;");
        // Verify all positional assertions pass
        self.verify_positionals();
        let has_args = self.has_args();

        // Next we create the `--help` and `--version` arguments and add them if
        // necessary
        self.create_help_and_version();

        let mut subcmd_name: Option<String> = None;
        let mut needs_val_of: Option<&'a str> = None;
        let mut pos_counter = 1;
        while let Some(arg) = it.next() {
            let arg_os = arg.into();
            debugln!("Parser::get_matches_with: Begin parsing '{:?}' ({:?})",
                     arg_os,
                     &*arg_os.as_bytes());

            self.unset(AS::ValidNegNumFound);
            // Is this a new argument, or values from a previous option?
            let starts_new_arg = self.is_new_arg(&arg_os, needs_val_of);

            // Has the user already passed '--'? Meaning only positional args follow
            if !self.is_set(AS::TrailingValues) {
                // Does the arg match a subcommand name, or any of it's aliases (if defined)
                {
                    let (is_match, sc_name) = self.possible_subcommand(&arg_os);
                    if is_match {
                        let sc_name = sc_name.expect(INTERNAL_ERROR_MSG);
                        if sc_name == "help" && self.is_set(AS::NeedsSubcommandHelp) {
                            try!(self.parse_help_subcommand(it));
                        }
                        subcmd_name = Some(sc_name.to_owned());
                        break;
                    }
                }

                if !starts_new_arg {
                    if let Some(name) = needs_val_of {
                        // Check to see if parsing a value from a previous arg
                        if let Some(arg) = find_by_name!(self, &name, opts, iter) {
                            // get the OptBuilder so we can check the settings
                            needs_val_of = try!(self.add_val_to_arg(&*arg, &arg_os, matcher));
                            // get the next value from the iterator
                            continue;
                        }
                    }
                } else {
                    if arg_os.starts_with(b"--") {
                        if arg_os.len_() == 2 {
                            // The user has passed '--' which means only positional args follow no
                            // matter what they start with
                            debugln!("Parser::get_matches_with: found '--'");
                            debugln!("Parser::get_matches_with: setting TrailingVals=true");
                            self.set(AS::TrailingValues);
                            continue;
                        }

                        needs_val_of = try!(self.parse_long_arg(matcher, &arg_os));
                        if !(needs_val_of.is_none() && self.is_set(AS::AllowLeadingHyphen)) {
                            continue;
                        }
                    } else if arg_os.starts_with(b"-") && arg_os.len_() != 1 {
                        // Try to parse short args like normal, if AllowLeadingHyphen or
                        // AllowNegativeNumbers is set, parse_short_arg will *not* throw
                        // an error, and instead return Ok(None)
                        needs_val_of = try!(self.parse_short_arg(matcher, &arg_os));
                        // If it's None, we then check if one of those two AppSettings was set
                        if needs_val_of.is_none() {
                            if self.is_set(AS::AllowNegativeNumbers) {
                                if !(arg_os.to_string_lossy().parse::<i64>().is_ok() ||
                                     arg_os.to_string_lossy().parse::<f64>().is_ok()) {
                                    return Err(Error::unknown_argument(&*arg_os.to_string_lossy(),
                                        "",
                                        &*self.create_current_usage(matcher, None),
                                        self.color()));
                                }
                            } else if !self.is_set(AS::AllowLeadingHyphen) {
                                continue;
                            }
                        } else {
                            continue;
                        }
                    }
                }

                if !(self.is_set(AS::ArgsNegateSubcommands) && self.is_set(AS::ValidArgFound)) &&
                   !self.is_set(AS::InferSubcommands) {
                    if let Some(cdate) = suggestions::did_you_mean(&*arg_os.to_string_lossy(),
                                                                   sc_names!(self)) {
                        return Err(Error::invalid_subcommand(arg_os.to_string_lossy()
                                                                 .into_owned(),
                                                             cdate,
                                                             self.meta
                                                                 .bin_name
                                                                 .as_ref()
                                                                 .unwrap_or(&self.meta.name),
                                                             &*self.create_current_usage(matcher,
                                                                                         None),
                                                             self.color()));
                    }
                }
            }

            let low_index_mults = self.is_set(AS::LowIndexMultiplePositional) &&
                                  pos_counter == (self.positionals.len() - 1);
            let missing_pos = self.is_set(AS::AllowMissingPositional) &&
                              pos_counter == (self.positionals.len() - 1);
            debugln!("Parser::get_matches_with: Positional counter...{}",
                     pos_counter);
            debugln!("Parser::get_matches_with: Low index multiples...{:?}",
                     low_index_mults);
            if low_index_mults || missing_pos {
                if let Some(na) = it.peek() {
                    let n = (*na).clone().into();
                    needs_val_of = if needs_val_of.is_none() {
                        if let Some(p) = self.positionals.get(pos_counter) {
                            Some(p.b.name)
                        } else {
                            None
                        }
                    } else {
                        None
                    };
                    let sc_match = {
                        self.possible_subcommand(&n).0
                    };
                    if self.is_new_arg(&n, needs_val_of) || sc_match ||
                       suggestions::did_you_mean(&n.to_string_lossy(), sc_names!(self)).is_some() {
                        debugln!("Parser::get_matches_with: Bumping the positional counter...");
                        pos_counter += 1;
                    }
                } else {
                    debugln!("Parser::get_matches_with: Bumping the positional counter...");
                    pos_counter += 1;
                }
            }
            if let Some(p) = self.positionals.get(pos_counter) {
                parse_positional!(self, p, arg_os, pos_counter, matcher);
                self.settings.set(AS::ValidArgFound);
            } else if self.is_set(AS::AllowExternalSubcommands) {
                // Get external subcommand name
                let sc_name = match arg_os.to_str() {
                    Some(s) => s.to_string(),
                    None => {
                        if !self.is_set(AS::StrictUtf8) {
                            return Err(Error::invalid_utf8(&*self.create_current_usage(matcher,
                                                                                       None),
                                                           self.color()));
                        }
                        arg_os.to_string_lossy().into_owned()
                    }
                };

                // Collect the external subcommand args
                let mut sc_m = ArgMatcher::new();
                while let Some(v) = it.next() {
                    let a = v.into();
                    if a.to_str().is_none() && !self.is_set(AS::StrictUtf8) {
                        return Err(Error::invalid_utf8(&*self.create_current_usage(matcher, None),
                                                       self.color()));
                    }
                    sc_m.add_val_to("", &a);
                }

                matcher.subcommand(SubCommand {
                    name: sc_name,
                    matches: sc_m.into(),
                });
            } else if !(self.is_set(AS::AllowLeadingHyphen) ||
                        self.is_set(AS::AllowNegativeNumbers)) &&
                      !self.is_set(AS::InferSubcommands) {
                return Err(Error::unknown_argument(&*arg_os.to_string_lossy(),
                                                   "",
                                                   &*self.create_current_usage(matcher, None),
                                                   self.color()));
            } else if !(has_args) && self.has_subcommands() {
                if let Some(cdate) = suggestions::did_you_mean(&*arg_os.to_string_lossy(),
                                                               sc_names!(self)) {
                    return Err(Error::invalid_subcommand(arg_os.to_string_lossy()
                                                                .into_owned(),
                                                            cdate,
                                                            self.meta
                                                                .bin_name
                                                                .as_ref()
                                                                .unwrap_or(&self.meta.name),
                                                            &*self.create_current_usage(matcher,
                                                                                        None),
                                                            self.color()));
                }
            }
        }

        if let Some(ref pos_sc_name) = subcmd_name {
            let sc_name = {
                find_subcmd!(self, pos_sc_name).expect(INTERNAL_ERROR_MSG).p.meta.name.clone()
            };
            try!(self.parse_subcommand(&*sc_name, matcher, it));
        } else if self.is_set(AS::SubcommandRequired) {
            let bn = self.meta.bin_name.as_ref().unwrap_or(&self.meta.name);
            return Err(Error::missing_subcommand(bn,
                                                 &self.create_current_usage(matcher, None),
                                                 self.color()));
        } else if self.is_set(AS::SubcommandRequiredElseHelp) {
            debugln!("parser::get_matches_with: SubcommandRequiredElseHelp=true");
            let mut out = vec![];
            try!(self.write_help_err(&mut out));
            return Err(Error {
                message: String::from_utf8_lossy(&*out).into_owned(),
                kind: ErrorKind::MissingArgumentOrSubcommand,
                info: None,
            });
        }

        self.validate(needs_val_of, subcmd_name, matcher)
    }

    fn validate(&mut self,
                needs_val_of: Option<&'a str>,
                subcmd_name: Option<String>,
                matcher: &mut ArgMatcher<'a>)
                -> ClapResult<()> {
        debugln!("Parser::validate;");
        let mut reqs_validated = false;
        try!(self.add_defaults(matcher));
        if let Some(a) = needs_val_of {
            debugln!("Parser::validate: needs_val_of={:?}", a);
            if let Some(o) = find_by_name!(self, &a, opts, iter) {
                try!(self.validate_required(matcher));
                reqs_validated = true;
                let should_err = if let Some(v) = matcher.0.args.get(&*o.b.name) {
                    v.vals.is_empty() && !(o.v.min_vals.is_some() && o.v.min_vals.unwrap() == 0)
                } else {
                    true
                };
                if should_err {
                    return Err(Error::empty_value(o,
                                                  &*self.create_current_usage(matcher, None),
                                                  self.color()));
                }
            }
        }

        try!(self.validate_blacklist(matcher));
        if !(self.is_set(AS::SubcommandsNegateReqs) && subcmd_name.is_some()) && !reqs_validated {
            try!(self.validate_required(matcher));
        }
        try!(self.validate_matched_args(matcher));
        matcher.usage(self.create_usage(&[]));

        if matcher.is_empty() && matcher.subcommand_name().is_none() &&
           self.is_set(AS::ArgRequiredElseHelp) {
            let mut out = vec![];
            try!(self.write_help_err(&mut out));
            return Err(Error {
                message: String::from_utf8_lossy(&*out).into_owned(),
                kind: ErrorKind::MissingArgumentOrSubcommand,
                info: None,
            });
        }
        Ok(())
    }

    fn propogate_help_version(&mut self) {
        debugln!("Parser::propogate_help_version;");
        self.create_help_and_version();
        for sc in &mut self.subcommands {
            sc.p.propogate_help_version();
        }
    }

    fn build_bin_names(&mut self) {
        debugln!("Parser::build_bin_names;");
        for sc in &mut self.subcommands {
            debug!("Parser::build_bin_names:iter: bin_name set...");
            if sc.p.meta.bin_name.is_none() {
                sdebugln!("No");
                let bin_name = format!("{}{}{}",
                                       self.meta
                                           .bin_name
                                           .as_ref()
                                           .unwrap_or(&self.meta.name.clone()),
                                       if self.meta.bin_name.is_some() {
                                           " "
                                       } else {
                                           ""
                                       },
                                       &*sc.p.meta.name);
                debugln!("Parser::build_bin_names:iter: Setting bin_name of {} to {}",
                         self.meta.name,
                         bin_name);
                sc.p.meta.bin_name = Some(bin_name);
            } else {
                sdebugln!("yes ({:?})", sc.p.meta.bin_name);
            }
            debugln!("Parser::build_bin_names:iter: Calling build_bin_names from...{}",
                     sc.p.meta.name);
            sc.p.build_bin_names();
        }
    }

    fn parse_subcommand<I, T>(&mut self,
                              sc_name: &str,
                              matcher: &mut ArgMatcher<'a>,
                              it: &mut Peekable<I>)
                              -> ClapResult<()>
        where I: Iterator<Item = T>,
              T: Into<OsString> + Clone
    {
        use std::fmt::Write;
        debugln!("Parser::parse_subcommand;");
        let mut mid_string = String::new();
        if !self.is_set(AS::SubcommandsNegateReqs) {
            let mut hs: Vec<&str> = self.required.iter().map(|n| &**n).collect();
            for k in matcher.arg_names() {
                hs.push(k);
            }
            let reqs = self.get_required_from(&hs, Some(matcher), None);

            for s in &reqs {
                write!(&mut mid_string, " {}", s).expect(INTERNAL_ERROR_MSG);
            }
        }
        mid_string.push_str(" ");
        if let Some(ref mut sc) = self.subcommands
            .iter_mut()
            .find(|s| &s.p.meta.name == &sc_name) {
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
                                              self.meta
                                                  .bin_name
                                                  .as_ref()
                                                  .unwrap_or(&String::new()),
                                              if self.meta.bin_name.is_some() {
                                                  " "
                                              } else {
                                                  ""
                                              },
                                              &*sc.p.meta.name));
            debugln!("Parser::parse_subcommand: About to parse sc={}",
                     sc.p.meta.name);
            debugln!("Parser::parse_subcommand: sc settings={:#?}", sc.p.settings);
            try!(sc.p.get_matches_with(&mut sc_matcher, it));
            matcher.subcommand(SubCommand {
                name: sc.p.meta.name.clone(),
                matches: sc_matcher.into(),
            });
        }
        Ok(())
    }


    fn groups_for_arg(&self, name: &str) -> Option<Vec<&'a str>> {
        debugln!("Parser::groups_for_arg: name={}", name);

        if self.groups.is_empty() {
            debugln!("Parser::groups_for_arg: No groups defined");
            return None;
        }
        let mut res = vec![];
        debugln!("Parser::groups_for_arg: Searching through groups...");
        for grp in &self.groups {
            for a in &grp.args {
                if a == &name {
                    sdebugln!("\tFound '{}'", grp.name);
                    res.push(&*grp.name);
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

        for n in &self.groups.iter().find(|g| g.name == group).expect(INTERNAL_ERROR_MSG).args {
            if let Some(f) = self.flags.iter().find(|f| &f.b.name == n) {
                args.push(f.to_string());
            } else if let Some(f) = self.opts.iter().find(|o| &o.b.name == n) {
                args.push(f.to_string());
            } else if let Some(p) = self.positionals
                .values()
                .find(|p| &p.b.name == n) {
                args.push(p.b.name.to_owned());
            } else {
                g_vec.push(*n);
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

        for n in &self.groups.iter().find(|g| g.name == group).expect(INTERNAL_ERROR_MSG).args {
            if self.groups.iter().any(|g| &g.name == &*n) {
                args.extend(self.arg_names_in_group(&*n));
                g_vec.push(*n);
            } else {
                args.push(*n);
            }
        }

        // TODO: faster way to sort/dedup?
        args.sort();
        args.dedup();
        args.iter().map(|s| *s).collect()
    }

    pub fn create_help_and_version(&mut self) {
        debugln!("Parser::create_help_and_version;");
        // name is "hclap_help" because flags are sorted by name
        if !self.contains_long("help") {
            debugln!("Parser::create_help_and_version: Building --help");
            if self.help_short.is_none() && !self.contains_short('h') {
                self.help_short = Some('h');
            }
            let arg = FlagBuilder {
                b: Base {
                    name: "hclap_help",
                    help: Some("Prints help information"),
                    ..Default::default()
                },
                s: Switched {
                    short: self.help_short,
                    long: Some("help"),
                    ..Default::default()
                },
            };
            self.flags.push(arg);
        }
        if !self.is_set(AS::DisableVersion) && !self.contains_long("version") {
            debugln!("Parser::create_help_and_version: Building --version");
            if self.version_short.is_none() && !self.contains_short('V') {
                self.version_short = Some('V');
            }
            // name is "vclap_version" because flags are sorted by name
            let arg = FlagBuilder {
                b: Base {
                    name: "vclap_version",
                    help: Some("Prints version information"),
                    ..Default::default()
                },
                s: Switched {
                    short: self.version_short,
                    long: Some("version"),
                    ..Default::default()
                },
            };
            self.flags.push(arg);
        }
        if !self.subcommands.is_empty() && !self.is_set(AS::DisableHelpSubcommand) &&
           self.is_set(AS::NeedsSubcommandHelp) {
            debugln!("Parser::create_help_and_version: Building help");
            self.subcommands
                .push(App::new("help")
                    .about("Prints this message or the help of the given subcommand(s)"));
        }
    }

    // Retrieves the names of all args the user has supplied thus far, except required ones
    // because those will be listed in self.required
    pub fn create_current_usage(&self, matcher: &'b ArgMatcher<'a>, extra: Option<&str>) -> String {
        let mut args: Vec<_> = matcher.arg_names()
            .iter()
            .filter(|n| {
                if let Some(o) = find_by_name!(self, *n, opts, iter) {
                    !o.b.settings.is_set(ArgSettings::Required)
                } else if let Some(p) = find_by_name!(self, *n, positionals, values) {
                    !p.b.settings.is_set(ArgSettings::Required)
                } else {
                    true // flags can't be required, so they're always true
                }
            })
            .map(|&n| n)
            .collect();
        if let Some(r) = extra {
            args.push(r);
        }
        self.create_usage(&*args)
    }

    fn check_for_help_and_version_str(&self, arg: &OsStr) -> ClapResult<()> {
        debugln!("Parser::check_for_help_and_version_str;");
        debug!("Parser::check_for_help_and_version_str: Checking if --{} is help or version...",
               arg.to_str().unwrap());
        if arg == "help" && self.is_set(AS::NeedsLongHelp) {
            sdebugln!("Help");
            try!(self._help());
        }
        if arg == "version" && self.is_set(AS::NeedsLongVersion) {
            sdebugln!("Version");
            try!(self._version());
        }
        sdebugln!("Neither");

        Ok(())
    }

    fn check_for_help_and_version_char(&self, arg: char) -> ClapResult<()> {
        debugln!("Parser::check_for_help_and_version_char;");
        debug!("Parser::check_for_help_and_version_char: Checking if -{} is help or version...",
               arg);
        if let Some(h) = self.help_short {
            if arg == h && self.is_set(AS::NeedsLongHelp) {
                sdebugln!("Help");
                try!(self._help());
            }
        }
        if let Some(v) = self.version_short {
            if arg == v && self.is_set(AS::NeedsLongVersion) {
                sdebugln!("Version");
                try!(self._version());
            }
        }
        sdebugln!("Neither");
        Ok(())
    }

    fn _help(&self) -> ClapResult<()> {
        let mut buf = vec![];
        try!(Help::write_parser_help(&mut buf, self));
        Err(Error {
            message: unsafe { String::from_utf8_unchecked(buf) },
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
                      -> ClapResult<Option<&'a str>> {
        // maybe here lifetime should be 'a
        debugln!("Parser::parse_long_arg;");
        let mut val = None;
        debug!("Parser::parse_long_arg: Does it contain '='...");
        let arg = if full_arg.contains_byte(b'=') {
            let (p0, p1) = full_arg.trim_left_matches(b'-').split_at_byte(b'=');
            sdebugln!("Yes '{:?}'", p1);
            val = Some(p1);
            p0
        } else {
            sdebugln!("No");
            full_arg.trim_left_matches(b'-')
        };

        if let Some(opt) = find_opt_by_long!(@os self, &arg) {
            debugln!("Parser::parse_long_arg: Found valid opt '{}'",
                     opt.to_string());
            self.settings.set(AS::ValidArgFound);
            let ret = try!(self.parse_opt(val, opt, val.is_some(), matcher));
            if self.cache.map_or(true, |name| name != opt.b.name) {
                arg_post_processing!(self, opt, matcher);
                self.cache = Some(opt.b.name);
            }

            return Ok(ret);
        } else if let Some(flag) = find_flag_by_long!(@os self, &arg) {
            debugln!("Parser::parse_long_arg: Found valid flag '{}'",
                     flag.to_string());
            self.settings.set(AS::ValidArgFound);
            // Only flags could be help or version, and we need to check the raw long
            // so this is the first point to check
            try!(self.check_for_help_and_version_str(arg));

            try!(self.parse_flag(flag, matcher));

            // Handle conflicts, requirements, etc.
            if self.cache.map_or(true, |name| name != flag.b.name) {
                arg_post_processing!(self, flag, matcher);
                self.cache = Some(flag.b.name);
            }

            return Ok(None);
        } else if self.is_set(AS::AllowLeadingHyphen) {
            return Ok(None);
        } else if self.is_set(AS::ValidNegNumFound) {
            return Ok(None);
        }

        debugln!("Parser::parse_long_arg: Didn't match anything");
        self.did_you_mean_error(arg.to_str().expect(INVALID_UTF8), matcher).map(|_| None)
    }

    fn parse_short_arg(&mut self,
                       matcher: &mut ArgMatcher<'a>,
                       full_arg: &OsStr)
                       -> ClapResult<Option<&'a str>> {
        debugln!("Parser::parse_short_arg: full_arg={:?}", full_arg);
        let arg_os = full_arg.trim_left_matches(b'-');
        let arg = arg_os.to_string_lossy();

        // If AllowLeadingHyphen is set, we want to ensure `-val` gets parsed as `-val` and not
        // `-v` `-a` `-l` assuming `v` `a` and `l` are all, or mostly, valid shorts.
        if self.is_set(AS::AllowLeadingHyphen) {
            if arg.chars().any(|c| !self.contains_short(c)) {
                debugln!("Parser::parse_short_arg: LeadingHyphenAllowed yet -{} isn't valid",
                         arg);
                return Ok(None);
            }
        } else if self.is_set(AS::ValidNegNumFound) {
            // TODO: Add docs about having AllowNegativeNumbers and `-2` as a valid short
            // May be better to move this to *after* not finding a valid flag/opt?
            debugln!("Parser::parse_short_arg: Valid negative num...");
            return Ok(None);
        }

        for c in arg.chars() {
            debugln!("Parser::parse_short_arg:iter:{}", c);
            // Check for matching short options, and return the name if there is no trailing
            // concatenated value: -oval
            // Option: -o
            // Value: val
            if let Some(opt) = find_opt_by_short!(self, c) {
                debugln!("Parser::parse_short_arg:iter:{}: Found valid opt", c);
                self.settings.set(AS::ValidArgFound);
                // Check for trailing concatenated value
                let p: Vec<_> = arg.splitn(2, c).collect();
                debugln!("Parser::parse_short_arg:iter:{}: p[0]={:?}, p[1]={:?}",
                         c,
                         p[0].as_bytes(),
                         p[1].as_bytes());
                let i = p[0].as_bytes().len() + 1;
                let val = if p[1].as_bytes().len() > 0 {
                    debugln!("Parser::parse_short_arg:iter:{}: val={:?} (bytes), val={:?} (ascii)",
                             c,
                             arg_os.split_at(i).1.as_bytes(),
                             arg_os.split_at(i).1);
                    Some(arg_os.split_at(i).1)
                } else {
                    None
                };

                // Default to "we're expecting a value later"
                let ret = try!(self.parse_opt(val, opt, false, matcher));

                if self.cache.map_or(true, |name| name != opt.b.name) {
                    arg_post_processing!(self, opt, matcher);
                    self.cache = Some(opt.b.name);
                }

                return Ok(ret);
            } else if let Some(flag) = find_flag_by_short!(self, c) {
                debugln!("Parser::parse_short_arg:iter:{}: Found valid flag", c);
                self.settings.set(AS::ValidArgFound);
                // Only flags can be help or version
                try!(self.check_for_help_and_version_char(c));
                try!(self.parse_flag(flag, matcher));

                // Handle conflicts, requirements, overrides, etc.
                // Must be called here due to mutablilty
                if self.cache.map_or(true, |name| name != flag.b.name) {
                    arg_post_processing!(self, flag, matcher);
                    self.cache = Some(flag.b.name);
                }
            } else {
                let arg = format!("-{}", c);
                return Err(Error::unknown_argument(&*arg,
                                                   "",
                                                   &*self.create_current_usage(matcher, None),
                                                   self.color()));
            }
        }
        Ok(None)
    }

    fn parse_opt(&self,
                 val: Option<&OsStr>,
                 opt: &OptBuilder<'a, 'b>,
                 had_eq: bool,
                 matcher: &mut ArgMatcher<'a>)
                 -> ClapResult<Option<&'a str>> {
        debugln!("Parser::parse_opt; opt={}, val={:?}", opt.b.name, val);
        debugln!("Parser::parse_opt; opt.settings={:?}", opt.b.settings);
        let mut has_eq = false;

        debug!("Parser::parse_opt; Checking for val...");
        if let Some(fv) = val {
            has_eq = fv.starts_with(&[b'=']) || had_eq;
            let v = fv.trim_left_matches(b'=');
            if !opt.is_set(ArgSettings::EmptyValues) &&
               (v.len_() == 0 || (opt.is_set(ArgSettings::RequireEquals) && !has_eq)) {
                sdebugln!("Found Empty - Error");
                return Err(Error::empty_value(opt,
                                              &*self.create_current_usage(matcher, None),
                                              self.color()));
            }
            sdebugln!("Found - {:?}, len: {}", v, v.len_());
            debugln!("Parser::parse_opt: {:?} contains '='...{:?}",
                     fv,
                     fv.starts_with(&[b'=']));
            try!(self.add_val_to_arg(opt, v, matcher));
        } else if opt.is_set(ArgSettings::RequireEquals) && !opt.is_set(ArgSettings::EmptyValues) {
            sdebugln!("None, but requires equals...Error");
            return Err(Error::empty_value(opt,
                                          &*self.create_current_usage(matcher, None),
                                          self.color()));

        } else {
            sdebugln!("None");
        }

        matcher.inc_occurrence_of(opt.b.name);
        // Increment or create the group "args"
        self.groups_for_arg(opt.b.name).and_then(|vec| Some(matcher.inc_occurrences_of(&*vec)));

        if val.is_none() ||
           !has_eq &&
           (opt.is_set(ArgSettings::Multiple) && !opt.is_set(ArgSettings::RequireDelimiter) &&
            matcher.needs_more_vals(opt)) {
            debugln!("Parser::parse_opt: More arg vals required...");
            return Ok(Some(opt.b.name));
        }
        debugln!("Parser::parse_opt: More arg vals not required...");
        Ok(None)
    }

    fn add_val_to_arg<A>(&self,
                         arg: &A,
                         val: &OsStr,
                         matcher: &mut ArgMatcher<'a>)
                         -> ClapResult<Option<&'a str>>
        where A: AnyArg<'a, 'b> + Display
    {
        debugln!("Parser::add_val_to_arg; arg={}, val={:?}", arg.name(), val);
        let mut ret = None;
        debugln!("Parser::add_val_to_arg; trailing_vals={:?}, DontDelimTrailingVals={:?}",
                 self.is_set(AS::TrailingValues),
                 self.is_set(AS::DontDelimitTrailingValues));
        if !(self.is_set(AS::TrailingValues) && self.is_set(AS::DontDelimitTrailingValues)) {
            if let Some(delim) = arg.val_delim() {
                if val.is_empty_() {
                    ret = try!(self.add_single_val_to_arg(arg, val, matcher));
                } else {
                    for v in val.split(delim as u32 as u8) {
                        ret = try!(self.add_single_val_to_arg(arg, v, matcher));
                    }
                    // If there was a delimiter used, we're not looking for more values
                    if val.contains_byte(delim as u32 as u8) ||
                       arg.is_set(ArgSettings::RequireDelimiter) {
                        ret = None;
                    }
                }
            } else {
                ret = try!(self.add_single_val_to_arg(arg, val, matcher));
            }
        } else {
            ret = try!(self.add_single_val_to_arg(arg, val, matcher));
        }
        Ok(ret)
    }

    fn add_single_val_to_arg<A>(&self,
                                arg: &A,
                                v: &OsStr,
                                matcher: &mut ArgMatcher<'a>)
                                -> ClapResult<Option<&'a str>>
        where A: AnyArg<'a, 'b> + Display
    {
        debugln!("Parser::add_single_val_to_arg;");
        debugln!("Parser::add_single_val_to_arg: adding val...{:?}", v);
        if let Some(t) = arg.val_terminator() {
            if t == v {
                return Ok(None);
            }
        }
        matcher.add_val_to(arg.name(), v);

        // Increment or create the group "args"
        if let Some(grps) = self.groups_for_arg(arg.name()) {
            for grp in grps {
                matcher.add_val_to(&*grp, v);
            }
        }

        if matcher.needs_more_vals(arg) {
            return Ok(Some(arg.name()));
        }
        Ok(None)
    }

    fn validate_values<A>(&self,
                          arg: &A,
                          ma: &MatchedArg,
                          matcher: &ArgMatcher<'a>)
                          -> ClapResult<()>
        where A: AnyArg<'a, 'b> + Display
    {
        debugln!("Parser::validate_values: arg={:?}", arg.name());
        for val in &ma.vals {
            if self.is_set(AS::StrictUtf8) && val.to_str().is_none() {
                debugln!("Parser::validate_values: invalid UTF-8 found in val {:?}",
                         val);
                return Err(Error::invalid_utf8(&*self.create_current_usage(matcher, None),
                                               self.color()));
            }
            if let Some(p_vals) = arg.possible_vals() {
                debugln!("Parser::validate_values: possible_vals={:?}", p_vals);
                let val_str = val.to_string_lossy();
                if !p_vals.contains(&&*val_str) {
                    return Err(Error::invalid_value(val_str,
                                                    p_vals,
                                                    arg,
                                                    &*self.create_current_usage(matcher, None),
                                                    self.color()));
                }
            }
            if !arg.is_set(ArgSettings::EmptyValues) && val.is_empty_() &&
               matcher.contains(&*arg.name()) {
                debugln!("Parser::validate_values: illegal empty val found");
                return Err(Error::empty_value(arg,
                                              &*self.create_current_usage(matcher, None),
                                              self.color()));
            }
            if let Some(vtor) = arg.validator() {
                debug!("Parser::validate_values: checking validator...");
                if let Err(e) = vtor(val.to_string_lossy().into_owned()) {
                    sdebugln!("error");
                    return Err(Error::value_validation(Some(arg), e, self.color()));
                } else {
                    sdebugln!("good");
                }
            }
            if let Some(vtor) = arg.validator_os() {
                debug!("Parser::validate_values: checking validator_os...");
                if let Err(e) = vtor(&val) {
                    sdebugln!("error");
                    return Err(Error::value_validation(Some(arg),
                                                       (*e).to_string_lossy().to_string(),
                                                       self.color()));
                } else {
                    sdebugln!("good");
                }
            }
        }
        Ok(())
    }

    fn parse_flag(&self,
                  flag: &FlagBuilder<'a, 'b>,
                  matcher: &mut ArgMatcher<'a>)
                  -> ClapResult<()> {
        debugln!("Parser::parse_flag;");

        matcher.inc_occurrence_of(flag.b.name);
        // Increment or create the group "args"
        self.groups_for_arg(flag.b.name).and_then(|vec| Some(matcher.inc_occurrences_of(&*vec)));

        Ok(())
    }

    fn validate_blacklist(&self, matcher: &mut ArgMatcher) -> ClapResult<()> {
        debugln!("Parser::validate_blacklist: blacklist={:?}", self.blacklist);
        macro_rules! build_err {
            ($me:ident, $name:expr, $matcher:ident) => ({
                debugln!("build_err!: name={}", $name);
                let mut c_with = find_from!($me, $name, blacklist, &$matcher);
                c_with = c_with.or(
                    $me.find_any_arg($name).map_or(None, |aa| aa.blacklist())
                                           .map_or(None, 
                                                |bl| bl.iter().find(|arg| $matcher.contains(arg)))
                                           .map_or(None, |an| $me.find_any_arg(an))
                                           .map_or(None, |aa| Some(format!("{}", aa)))
                );
                debugln!("build_err!: '{:?}' conflicts with '{}'", c_with, $name);
                $matcher.remove($name);
                let usg = $me.create_current_usage($matcher, None);
                if let Some(f) = find_by_name!($me, $name, flags, iter) {
                    debugln!("build_err!: It was a flag...");
                    Error::argument_conflict(f, c_with, &*usg, self.color())
                } else if let Some(o) = find_by_name!($me, $name, opts, iter) {
                   debugln!("build_err!: It was an option...");
                    Error::argument_conflict(o, c_with, &*usg, self.color())
                } else {
                    match find_by_name!($me, $name, positionals, values) {
                        Some(p) => {
                            debugln!("build_err!: It was a positional...");
                            Error::argument_conflict(p, c_with, &*usg, self.color())
                        },
                        None    => panic!(INTERNAL_ERROR_MSG)
                    }
                }
            });
        }

        for name in &self.blacklist {
            debugln!("Parser::validate_blacklist:iter: Checking blacklisted name: {}",
                     name);
            if self.groups.iter().any(|g| &g.name == name) {
                debugln!("Parser::validate_blacklist:iter: groups contains it...");
                for n in self.arg_names_in_group(name) {
                    debugln!("Parser::validate_blacklist:iter:iter: Checking arg '{}' in group...",
                             n);
                    if matcher.contains(n) {
                        debugln!("Parser::validate_blacklist:iter:iter: matcher contains it...");
                        return Err(build_err!(self, &n, matcher));
                    }
                }
            } else if matcher.contains(name) {
                debugln!("Parser::validate_blacklist:iter: matcher contains it...");
                return Err(build_err!(self, name, matcher));
            }
        }
        Ok(())
    }

    fn validate_matched_args(&self, matcher: &mut ArgMatcher) -> ClapResult<()> {
        debugln!("Parser::validate_matched_args;");
        for (name, ma) in matcher.iter() {
            debugln!("Parser::validate_matched_args:iter:{}: vals={:#?}",
                     name,
                     ma.vals);
            if let Some(opt) = find_by_name!(self, name, opts, iter) {
                try!(self.validate_arg_num_vals(opt, ma, matcher));
                try!(self.validate_values(opt, ma, matcher));
                try!(self.validate_arg_requires(opt, ma, matcher));
                try!(self.validate_arg_num_occurs(opt, ma, matcher));
            } else if let Some(flag) = find_by_name!(self, name, flags, iter) {
                try!(self.validate_arg_requires(flag, ma, matcher));
                try!(self.validate_arg_num_occurs(flag, ma, matcher));
            } else if let Some(pos) = find_by_name!(self, name, positionals, values) {
                try!(self.validate_arg_num_vals(pos, ma, matcher));
                try!(self.validate_arg_num_occurs(pos, ma, matcher));
                try!(self.validate_values(pos, ma, matcher));
                try!(self.validate_arg_requires(pos, ma, matcher));
            } else {
                let grp = self.groups.iter().find(|g| &g.name == name).expect(INTERNAL_ERROR_MSG);
                if let Some(ref g_reqs) = grp.requires {
                    if g_reqs.iter().any(|&n| !matcher.contains(n)) {
                        return self.missing_required_error(matcher, None);
                    }
                }
            }
        }
        Ok(())
    }

    fn validate_arg_num_occurs<A>(&self,
                                  a: &A,
                                  ma: &MatchedArg,
                                  matcher: &ArgMatcher)
                                  -> ClapResult<()>
        where A: AnyArg<'a, 'b> + Display
    {
        debugln!("Parser::validate_arg_num_occurs: a={};", a.name());
        if ma.occurs > 1 && !a.is_set(ArgSettings::Multiple) {
            // Not the first time, and we don't allow multiples
            return Err(Error::unexpected_multiple_usage(a,
                                                        &*self.create_current_usage(matcher, None),
                                                        self.color()));
        }
        Ok(())
    }

    fn validate_arg_num_vals<A>(&self,
                                a: &A,
                                ma: &MatchedArg,
                                matcher: &ArgMatcher)
                                -> ClapResult<()>
        where A: AnyArg<'a, 'b> + Display
    {
        debugln!("Parser::validate_arg_num_vals;");
        if let Some(num) = a.num_vals() {
            debugln!("Parser::validate_arg_num_vals: num_vals set...{}", num);
            let should_err = if a.is_set(ArgSettings::Multiple) {
                ((ma.vals.len() as u64) % num) != 0
            } else {
                num != (ma.vals.len() as u64)
            };
            if should_err {
                debugln!("Parser::validate_arg_num_vals: Sending error WrongNumberOfValues");
                return Err(Error::wrong_number_of_values(a,
                                                         num,
                                                         if a.is_set(ArgSettings::Multiple) {
                                                             (ma.vals.len() % num as usize)
                                                         } else {
                                                             ma.vals.len()
                                                         },
                                                         if ma.vals.len() == 1 ||
                                                            (a.is_set(ArgSettings::Multiple) &&
                                                             (ma.vals.len() % num as usize) ==
                                                             1) {
                                                             "as"
                                                         } else {
                                                             "ere"
                                                         },
                                                         &*self.create_current_usage(matcher, None),
                                                         self.color()));
            }
        }
        if let Some(num) = a.max_vals() {
            debugln!("Parser::validate_arg_num_vals: max_vals set...{}", num);
            if (ma.vals.len() as u64) > num {
                debugln!("Parser::validate_arg_num_vals: Sending error TooManyValues");
                return Err(Error::too_many_values(ma.vals
                                                      .iter()
                                                      .last()
                                                      .expect(INTERNAL_ERROR_MSG)
                                                      .to_str()
                                                      .expect(INVALID_UTF8),
                                                  a,
                                                  &*self.create_current_usage(matcher, None),
                                                  self.color()));
            }
        }
        if let Some(num) = a.min_vals() {
            debugln!("Parser::validate_arg_num_vals: min_vals set: {}", num);
            if (ma.vals.len() as u64) < num {
                debugln!("Parser::validate_arg_num_vals: Sending error TooFewValues");
                return Err(Error::too_few_values(a,
                                                 num,
                                                 ma.vals.len(),
                                                 &*self.create_current_usage(matcher, None),
                                                 self.color()));
            }
        }
        // Issue 665 (https://github.com/kbknapp/clap-rs/issues/665)
        if a.takes_value() && !a.is_set(ArgSettings::EmptyValues) && ma.vals.is_empty() {
            return Err(Error::empty_value(a,
                                          &*self.create_current_usage(matcher, None),
                                          self.color()));
        }
        Ok(())
    }

    fn validate_arg_requires<A>(&self,
                                a: &A,
                                ma: &MatchedArg,
                                matcher: &ArgMatcher)
                                -> ClapResult<()>
        where A: AnyArg<'a, 'b> + Display
    {
        debugln!("Parser::validate_arg_requires;");
        if let Some(a_reqs) = a.requires() {
            for &(val, name) in a_reqs.iter().filter(|&&(val, _)| val.is_some()) {
                if ma.vals
                    .iter()
                    .any(|v| v == val.expect(INTERNAL_ERROR_MSG) && !matcher.contains(name)) {
                    return self.missing_required_error(matcher, None);
                }
            }
        }
        Ok(())
    }

    #[inline]
    fn missing_required_error(&self, matcher: &ArgMatcher, extra: Option<&str>) -> ClapResult<()> {
        debugln!("Parser::missing_required_error: extra={:?}", extra);
        let c = Colorizer {
            use_stderr: true,
            when: self.color(),
        };
        let mut reqs = self.required.iter().map(|&r| &*r).collect::<Vec<_>>();
        if let Some(r) = extra {
            reqs.push(r);
        }
        reqs.retain(|n| !matcher.contains(n));
        reqs.dedup();
        debugln!("Parser::missing_required_error: reqs={:#?}", reqs);
        Err(Error::missing_required_argument(&*self.get_required_from(&reqs[..],
                                                                    Some(matcher),
                                                                    extra)
                                                 .iter()
                                                 .fold(String::new(), |acc, s| {
                                                     acc + &format!("\n    {}", c.error(s))[..]
                                                 }),
                                             &*self.create_current_usage(matcher, extra),
                                             self.color()))
    }

    fn validate_required(&self, matcher: &ArgMatcher) -> ClapResult<()> {
        debugln!("Parser::validate_required: required={:?};", self.required);
        'outer: for name in &self.required {
            debugln!("Parser::validate_required:iter:{}:", name);
            if matcher.contains(name) {
                continue 'outer;
            }
            if let Some(a) = find_by_name!(self, name, flags, iter) {
                if self.is_missing_required_ok(a, matcher) {
                    continue 'outer;
                }
            } else if let Some(a) = find_by_name!(self, name, opts, iter) {
                if self.is_missing_required_ok(a, matcher) {
                    continue 'outer;
                }
            } else if let Some(a) = find_by_name!(self, name, positionals, values) {
                if self.is_missing_required_ok(a, matcher) {
                    continue 'outer;
                }
            }
            return self.missing_required_error(matcher, None);
        }

        // Validate the conditionally required args
        for &(a, v, r) in &self.r_ifs {
            if let Some(ma) = matcher.get(a) {
                if matcher.get(r).is_none() {
                    if ma.vals.iter().any(|val| val == v) {
                        return self.missing_required_error(matcher, Some(r));
                    }
                }
            }
        }
        Ok(())
    }

    fn check_conflicts<A>(&self, a: &A, matcher: &ArgMatcher) -> Option<bool>
        where A: AnyArg<'a, 'b>
    {
        debugln!("Parser::check_conflicts: a={:?};", a.name());
        a.blacklist().map(|bl| {
            bl.iter().any(|conf| {
                matcher.contains(conf) ||
                self.groups
                    .iter()
                    .find(|g| &g.name == conf)
                    .map_or(false, |g| g.args.iter().any(|arg| matcher.contains(arg)))
            })
        })
    }

    fn check_required_unless<A>(&self, a: &A, matcher: &ArgMatcher) -> Option<bool>
        where A: AnyArg<'a, 'b>
    {
        debugln!("Parser::check_required_unless: a={:?};", a.name());
        macro_rules! check {
            ($how:ident, $_self:ident, $a:ident, $m:ident) => {{
                $a.required_unless().map(|ru| {
                    ru.iter().$how(|n| {
                        $m.contains(n) || {
                            if let Some(grp) = $_self.groups.iter().find(|g| &g.name == n) {
                                     grp.args.iter().any(|arg| $m.contains(arg))
                            } else {
                                false
                            }
                        }
                    })
                })
            }}; 
        }
        if a.is_set(ArgSettings::RequiredUnlessAll) {
            check!(all, self, a, matcher)
        } else {
            check!(any, self, a, matcher)
        }
    }

    #[inline]
    fn is_missing_required_ok<A>(&self, a: &A, matcher: &ArgMatcher) -> bool
        where A: AnyArg<'a, 'b>
    {
        debugln!("Parser::is_missing_required_ok: a={}", a.name());
        self.check_conflicts(a, matcher).unwrap_or(false) ||
        self.check_required_unless(a, matcher).unwrap_or(false)
    }

    fn did_you_mean_error(&self, arg: &str, matcher: &mut ArgMatcher<'a>) -> ClapResult<()> {
        // Didn't match a flag or option...maybe it was a typo and close to one
        let suffix =
            suggestions::did_you_mean_suffix(arg,
                                             longs!(self),
                                             suggestions::DidYouMeanMessageStyle::LongFlag);

        // Add the arg to the matches to build a proper usage string
        if let Some(name) = suffix.1 {
            if let Some(opt) = find_opt_by_long!(self, &name) {
                self.groups_for_arg(&*opt.b.name)
                    .and_then(|grps| Some(matcher.inc_occurrences_of(&*grps)));
                matcher.insert(&*opt.b.name);
            } else if let Some(flg) = find_flag_by_long!(self, &name) {
                self.groups_for_arg(&*flg.b.name)
                    .and_then(|grps| Some(matcher.inc_occurrences_of(&*grps)));
                matcher.insert(&*flg.b.name);
            }
        }

        let used_arg = format!("--{}", arg);
        Err(Error::unknown_argument(&*used_arg,
                                    &*suffix.0,
                                    &*self.create_current_usage(matcher, None),
                                    self.color()))
    }

    // Creates a usage string if one was not provided by the user manually. This happens just
    // after all arguments were parsed, but before any subcommands have been parsed
    // (so as to give subcommands their own usage recursively)
    pub fn create_usage(&self, used: &[&str]) -> String {
        debugln!("Parser::create_usage;");
        let mut usage = String::with_capacity(75);
        usage.push_str("USAGE:\n    ");
        usage.push_str(&self.create_usage_no_title(used));
        usage
    }

    // Creates a usage string (*without title*) if one was not provided by the user
    // manually. This happens just
    // after all arguments were parsed, but before any subcommands have been parsed
    // (so as to give subcommands their own usage recursively)
    pub fn create_usage_no_title(&self, used: &[&str]) -> String {
        debugln!("Parser::create_usage_no_title;");
        let mut usage = String::with_capacity(75);
        if let Some(u) = self.meta.usage_str {
            usage.push_str(&*u);
        } else if used.is_empty() {
            let name = self.meta
                .usage
                .as_ref()
                .unwrap_or_else(|| {
                    self.meta
                        .bin_name
                        .as_ref()
                        .unwrap_or(&self.meta.name)
                });
            usage.push_str(&*name);
            let mut reqs: Vec<&str> = self.required().map(|r| &**r).collect();
            reqs.dedup();
            let req_string = self.get_required_from(&reqs, None, None)
                .iter()
                .fold(String::new(), |a, s| a + &format!(" {}", s)[..]);

            let flags = self.needs_flags_tag();
            if flags && !self.is_set(AS::UnifiedHelpMessage) {
                usage.push_str(" [FLAGS]");
            } else if flags {
                usage.push_str(" [OPTIONS]");
            }
            if !self.is_set(AS::UnifiedHelpMessage) && 
               self.opts.iter().any(|o| !o.is_set(ArgSettings::Required) && 
               !o.is_set(ArgSettings::Hidden)) {
                usage.push_str(" [OPTIONS]");
            }

            usage.push_str(&req_string[..]);

            // places a '--' in the usage string if there are args and options
            // supporting multiple values
            if self.has_positionals() &&
               self.opts.iter().any(|o| o.is_set(ArgSettings::Multiple)) &&
               self.positionals.values().any(|p| !p.is_set(ArgSettings::Required)) &&
               !self.has_visible_subcommands() {
                usage.push_str(" [--]")
            }
            if self.has_positionals() &&
               self.positionals.values().any(|p| !p.is_set(ArgSettings::Required) && 
                    !p.is_set(ArgSettings::Hidden)) {
                if let Some(args_tag) = self.get_args_tag() {
                    usage.push_str(&*args_tag);
                } else {
                    usage.push_str(" [ARGS]");
                }
            }


            if self.is_set(AS::SubcommandsNegateReqs) || self.is_set(AS::ArgsNegateSubcommands) {
                if self.has_visible_subcommands() {
                    usage.push_str("\n    ");
                    usage.push_str(&*name);
                    usage.push_str(" <SUBCOMMAND>");
                }
            } else {
                if self.has_visible_subcommands() && !self.is_set(AS::SubcommandRequired) {
                    usage.push_str(" [SUBCOMMAND]");
                } else if (self.is_set(AS::SubcommandRequired) || 
                    self.is_set(AS::SubcommandRequiredElseHelp)) && self.has_subcommands() {
                    usage.push_str(" <SUBCOMMAND>");
                }
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
        debugln!("Parser::smart_usage;");
        let mut hs: Vec<&str> = self.required().map(|s| &**s).collect();
        hs.extend_from_slice(used);

        let r_string = self.get_required_from(&hs, None, None)
            .iter()
            .fold(String::new(), |acc, s| acc + &format!(" {}", s)[..]);

        usage.push_str(&self.meta
            .usage
            .as_ref()
            .unwrap_or_else(|| {
                self.meta
                    .bin_name
                    .as_ref()
                    .unwrap_or(&self.meta
                        .name)
            })
                            [..]);
        usage.push_str(&*r_string);
        if self.is_set(AS::SubcommandRequired) {
            usage.push_str(" <SUBCOMMAND>");
        }
    }

    // Prints the version to the user and exits if quit=true
    fn print_version<W: Write>(&self, w: &mut W) -> ClapResult<()> {
        try!(self.write_version(w));
        w.flush().map_err(Error::from)
    }

    pub fn write_version<W: Write>(&self, w: &mut W) -> io::Result<()> {
        if let Some(bn) = self.meta.bin_name.as_ref() {
            if bn.contains(' ') {
                // Incase we're dealing with subcommands i.e. git mv is translated to git-mv
                write!(w,
                       "{} {}",
                       bn.replace(" ", "-"),
                       self.meta.version.unwrap_or("".into()))
            } else {
                write!(w,
                       "{} {}",
                       &self.meta.name[..],
                       self.meta.version.unwrap_or("".into()))
            }
        } else {
            write!(w,
                   "{} {}",
                   &self.meta.name[..],
                   self.meta.version.unwrap_or("".into()))
        }
    }

    pub fn print_help(&self) -> ClapResult<()> {
        let out = io::stdout();
        let mut buf_w = BufWriter::new(out.lock());
        self.write_help(&mut buf_w)
    }

    pub fn write_help<W: Write>(&self, w: &mut W) -> ClapResult<()> {
        Help::write_parser_help(w, self)
    }

    pub fn write_help_err<W: Write>(&self, w: &mut W) -> ClapResult<()> {
        Help::write_parser_help_to_stderr(w, self)
    }

    fn add_defaults(&mut self, matcher: &mut ArgMatcher<'a>) -> ClapResult<()> {
        macro_rules! add_val {
            (@default $_self:ident, $a:ident, $m:ident) => {
                if let Some(ref val) = $a.v.default_val {
                    if $m.get($a.b.name).is_none() {
                        try!($_self.add_val_to_arg($a, OsStr::new(val), $m));

                        if $_self.cache.map_or(true, |name| name != $a.name()) {
                            arg_post_processing!($_self, $a, $m);
                            $_self.cache = Some($a.name());
                        }
                    }
                }
            };
            ($_self:ident, $a:ident, $m:ident) => {
                if let Some(ref vm) = $a.v.default_vals_ifs {
                    let mut done = false;
                    if $m.get($a.b.name).is_none() {
                        for &(arg, val, default) in vm.values() {
                            let add = if let Some(a) = $m.get(arg) {
                                if let Some(v) = val {
                                    a.vals.iter().any(|value| v == value)
                                } else {
                                    true
                                }
                            } else {
                                false
                            };
                            if add {
                                try!($_self.add_val_to_arg($a, OsStr::new(default), $m));
                                if $_self.cache.map_or(true, |name| name != $a.name()) {
                                    arg_post_processing!($_self, $a, $m);
                                    $_self.cache = Some($a.name());
                                }
                                done = true;
                                break;
                            }
                        }
                    }

                    if done {
                        continue; // outer loop (outside macro)
                    }
                }
                add_val!(@default $_self, $a, $m)
            };
        }

        for o in &self.opts {
            add_val!(self, o, matcher);
        }
        for p in self.positionals.values() {
            add_val!(self, p, matcher);
        }
        Ok(())
    }

    pub fn flags(&self) -> Iter<FlagBuilder<'a, 'b>> { self.flags.iter() }

    pub fn opts(&self) -> Iter<OptBuilder<'a, 'b>> { self.opts.iter() }

    pub fn positionals(&self) -> vec_map::Values<PosBuilder<'a, 'b>> { self.positionals.values() }

    pub fn subcommands(&self) -> Iter<App> { self.subcommands.iter() }

    // Should we color the output? None=determined by output location, true=yes, false=no
    #[doc(hidden)]
    pub fn color(&self) -> ColorWhen {
        debugln!("Parser::color;");
        debug!("Parser::color: Color setting...");
        if self.is_set(AS::ColorNever) {
            sdebugln!("Never");
            ColorWhen::Never
        } else if self.is_set(AS::ColorAlways) {
            sdebugln!("Always");
            ColorWhen::Always
        } else {
            sdebugln!("Auto");
            ColorWhen::Auto
        }
    }

    pub fn find_any_arg(&self, name: &str) -> Option<&AnyArg> {
        if let Some(f) = find_by_name!(self, &name, flags, iter) {
            return Some(f);
        }
        if let Some(o) = find_by_name!(self, &name, opts, iter) {
            return Some(o);
        }
        if let Some(p) = find_by_name!(self, &name, positionals, values) {
            return Some(p);
        }
        None
    }

    // Only used for completion scripts due to bin_name messiness
    #[cfg_attr(feature = "lints", allow(explicit_iter_loop))]
    pub fn find_subcommand(&'b self, sc: &str) -> Option<&'b App<'a, 'b>> {
        debugln!("Parser::find_subcommand: sc={}", sc);
        debugln!("Parser::find_subcommand: Currently in Parser...{}",
                 self.meta.bin_name.as_ref().unwrap());
        for s in self.subcommands.iter() {
            if s.p.meta.bin_name.as_ref().unwrap_or(&String::new()) == sc ||
               (s.p.meta.aliases.is_some() &&
                s.p
                .meta
                .aliases
                .as_ref()
                .unwrap()
                .iter()
                .any(|&(s, _)| s == sc.split(' ').rev().next().expect(INTERNAL_ERROR_MSG))) {
                return Some(s);
            }
            if let Some(app) = s.p.find_subcommand(sc) {
                return Some(app);
            }
        }
        None
    }

    #[inline]
    fn contains_long(&self, l: &str) -> bool { longs!(self).any(|al| al == &l) }

    #[inline]
    fn contains_short(&self, s: char) -> bool { shorts!(self).any(|arg_s| arg_s == &s) }
}
