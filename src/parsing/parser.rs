// Std
use std::ffi::{OsStr, OsString};
use std::io::{self, BufWriter, Write};
#[cfg(feature = "debug")]
use std::os::unix::ffi::OsStrExt;
use std::iter::Peekable;

// Third Party
use vec_map::VecMap;

// Internal
use INTERNAL_ERROR_MSG;
use INVALID_UTF8;
use {ArgSettings, SubCommand, App, Arg, ArgGroup};
use AppSettings as AS;
use output::HelpWriter;
use output::ErrorKind;
use output::Error as ClapError;
use Result as ClapResult;
use output::fmt::ColorWhen;
use output::suggestions;
use parsing::{OsStrExt2, ArgMatcher};

#[derive(Debug, PartialEq, Copy, Clone)]
#[doc(hidden)]
pub enum ParseResult<'a> {
    Flag,
    Opt(&'a str),
    Pos(&'a str),
    MaybeHyphenValue,
    MaybeNegNum,
    NotFound,
    ValuesDone,
}

#[derive(Debug)]
#[doc(hidden)]
pub struct Parser<'a, 'b, 'c>
where
    'a: 'b,
    'b: 'c
{
    pub app: &'c mut App<'a, 'b>,
    pub positionals: VecMap<&'a str>,
    pub required: Vec<&'a str>,
    pub req_ifs: Vec<(&'a str, &'b str, &'a str)>,
    pub conflicts: Vec<&'b str>,
    pub overrides: Vec<&'b str>,
    cache: Option<&'a str>,
}

impl<'a, 'b, 'c> Parser<'a, 'b, 'c>
{
    pub fn new(app:  &'c mut App<'a, 'b>) -> Self {
        app._build();

        let mut p = Parser {
            app: app,
            positionals: VecMap::new(),
            required: Vec::new(),
            req_ifs: Vec::new(),
            conflicts: Vec::new(),
            overrides: Vec::new(),
            cache: None,
        };

        // @DESIGN theres a lot of duplication of functions just to satisfy borrowck

        for grp in p.app.groups.iter_mut() {
            if grp.required {
                p.required.push(grp.name);
                if let Some(ref reqs) = grp.requires {
                    p.required.extend_from_slice(reqs);
                }
                if let Some(ref bl) = grp.conflicts {
                    p.conflicts.extend_from_slice(bl);
                }
            }
        }
        // Global args are first because of derived display orders
        // an alternative would be to use some form of timestamp (but that would require a new dep)
        for a in p.app.global_args.iter().chain(p.app.args.iter()) {
            debug_assert!(p.debug_asserts(a));
            if let Some(ref req_ifs) = a.required_ifs {
                for &(arg, val) in req_ifs {
                    p.req_ifs.push((arg, val, a.name));
                }
            }

            if let Some(ref grps) = a.groups {
                for g in grps {
                    if let None = p.app.groups.iter().find(|gr| &gr.name == g) { 
                        p.app.groups.push(ArgGroup::new(g).arg(a.name));

                    } else {
                        let mut ag = p.app.groups.iter_mut().find(|gr| &gr.name == g).unwrap();
                        if !ag.args.contains(&a.name) {
                            ag.args.push(a.name);
                        }
                    }
                }
            }

            if a.is_set(ArgSettings::Required) {
                // If the arg is required, add all it's requirements to master required list
                if let Some(ref areqs) = a.requires {
                    p.required.extend_from_slice(&*areqs);
                }
            }

            if a.is_set(ArgSettings::Last) {
                // if an arg has `Last` set, we need to imply DontCollapseArgsInUsage so that args
                // in the usage string don't get confused or left out.
                p.app._settings.set(AS::DontCollapseArgsInUsage);
                p.app._settings.set(AS::ContainsLast);
            }
            if let Some(l) = a.long {
                if l == "version" {
                    p.app._settings.unset(AS::NeedsLongVersion);
                } else if l == "help" {
                    p.app._settings.unset(AS::NeedsLongHelp);
                }
            }
            if a.index.is_some() || (a.short.is_none() && a.long.is_none()) {
                let i = if a.index.is_none() {
                    (p.positionals.len() + 1)
                } else {
                    a.index.unwrap()
                };
                p.positionals.insert(i, a.name);
            }
        }
        p
    }

    //
    // ---------- Asserts
    //

    #[inline]
    fn app_debug_asserts(&mut self) -> bool {
        assert!(self.verify_positionals());
        let should_err = self.app.groups.iter().all(|g| {
            g.args.iter().all(|arg| {
                (args!(self.app).any(|a| &a.name == arg) ||
                     self.app.groups.iter().any(|g| &g.name == arg))
            })
        });
        let g = self.app.groups.iter().find(|g| {
            g.args.iter().any(|arg| {
                !(args!(self.app).any(|f| &f.name == arg) ||
                      self.app.groups.iter().any(|g| &g.name == arg))
            })
        });
        assert!(
            should_err,
            "The group '{}' contains the arg '{}' that doesn't actually exist.",
            g.unwrap().name,
            g.unwrap()
                .args
                .iter()
                .find(|arg| {
                    !(args!(self.app).any(|f| &&f.name == arg) ||
                          self.app.groups.iter().any(|g| &&g.name == arg))
                })
                .unwrap()
        );
        true
    }

    #[inline]
    fn debug_asserts(&self, a: &Arg) -> bool {
        assert!(
            arg_names!(self.app).filter(|name| name == &a.name).count() == 1,
            format!("Non-unique argument name: {} is already in use", a.name)
        );
        if let Some(l) = a.long {
            assert!(
                longs!(self.app).filter(|long| long == &&l).count() == 1,
                "Argument long must be unique\n\n\t--{} is already in use",
                l
            );
        }
        if let Some(s) = a.short {
            assert!(
                shorts!(self.app).filter(|short| short == &&s).count() == 1,
                "Argument short must be unique\n\n\t-{} is already in use",
                s
            );
        }
        let i = if a.index.is_none() {
            (self.positionals.len() + 1)
        } else {
            a.index.unwrap() 
        };
        assert!(
            !self.positionals.contains_key(i),
            "Argument \"{}\" has the same index as another positional \
                    argument\n\n\tPerhaps try .multiple(true) to allow one positional argument \
                    to take multiple values",
            a.name
        );
        assert!(
            !(a.is_set(ArgSettings::Required) && a.is_set(ArgSettings::Global)),
            "Global arguments cannot be required.\n\n\t'{}' is marked as \
                          global and required",
            a.name
        );
        if a.is_set(ArgSettings::Last) {
            assert!(
                !positionals!(self.app).any(|a| a.is_set(ArgSettings::Last)),
                "Only one positional argument may have last(true) set. Found two."
            );
            assert!(
                a.long.is_none(),
                "Flags or Options may not have last(true) set. {} has both a long and last(true) set.",
                a.name
            );
            assert!(
                a.short.is_none(),
                "Flags or Options may not have last(true) set. {} has both a short and last(true) set.",
                a.name
            );
        }
        true
    }

    #[cfg_attr(feature = "lints", allow(block_in_if_condition_stmt))]
    pub fn verify_positionals(&mut self) -> bool {
        // Because you must wait until all arguments have been supplied, this is the first chance
        // to make assertions on positional argument indexes
        //
        // Firt we verify that the index highest supplied index, is equal to the number of
        // positional arguments to verify there are no gaps (i.e. supplying an index of 1 and 3
        // but no 2)
        if let Some(p) = positionals!(self.app).rev().next() {
            let idx = p.index.unwrap();
            assert!(
                !(idx != self.positionals.len()),
                "Found positional argument \"{}\" whose index is {} but there \
                          are only {} positional arguments defined",
                p,
                idx,
                self.positionals.len()
            );
        }

        // Next we verify that only the highest index has a .multiple(true) (if any)
        if positionals!(self.app).any(|a| {
            a.is_set(ArgSettings::Multiple) && (a.index.unwrap() != self.positionals.len())
        })
        {
            let mut it = positionals!(self.app).rev();
            let last = it.next().unwrap();
            let second_to_last = it.next().unwrap();
            // Either the final positional is required
            // Or the second to last has a terminator or .last(true) set
            let ok = last.is_set(ArgSettings::Required) ||
                (second_to_last.value_terminator.is_some() ||
                     second_to_last.is_set(ArgSettings::Last)) ||
                last.is_set(ArgSettings::Last);
            assert!(
                ok,
                "When using a positional argument with .multiple(true) that is *not the \
                          last* positional argument, the last positional argument (i.e the one \
                          with the highest index) *must* have .required(true) or .last(true) set."
            );
            let ok = second_to_last.is_set(ArgSettings::Multiple) || last.is_set(ArgSettings::Last);
            assert!(
                ok,
                "Only the last positional argument, or second to last positional \
                          argument may be set to .multiple(true)"
            );

            let count = positionals!(self.app).filter(|p| {
                    p.settings.contains(&ArgSettings::Multiple) && p.number_of_values.is_none()
                })
                .count();
            let ok = count <= 1 ||
                (last.is_set(ArgSettings::Last) && last.is_set(ArgSettings::Multiple) &&
                     second_to_last.is_set(ArgSettings::Multiple) && count == 2);
            assert!(
                ok,
                "Only one positional argument with .multiple(true) set is allowed per \
                        command, unless the second one also has .last(true) set"
            );
        }


        if self.is_set(AS::AllowMissingPositional) {
            // Check that if a required positional argument is found, all positions with a lower
            // index are also required.
            let mut found = false;
            let mut foundx2 = false;
            for p in positionals!(self.app).rev() {
                if foundx2 && !p.settings.contains(&ArgSettings::Required) {
                    assert!(
                        p.is_set(ArgSettings::Required),
                        "Found positional argument which is not required with a lower \
                                  index than a required positional argument by two or more: {:?} \
                                  index {:?}",
                        p.name,
                        p.index
                    );
                } else if p.is_set(ArgSettings::Required) && !p.is_set(ArgSettings::Last) {
                    // Args that .last(true) don't count since they can be required and have
                    // positionals with a lower index that aren't required
                    // Imagine: prog <req1> [opt1] -- <req2>
                    // Both of these are valid invocations:
                    //      $ prog r1 -- r2
                    //      $ prog r1 o1 -- r2
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
            // Check that if a required positional argument is found, all positions with a lower
            // index are also required
            let mut found = false;
            for p in positionals!(self.app).rev() {
                if found {
                    assert!(
                        p.is_set(ArgSettings::Required),
                        "Found positional argument which is not required with a lower \
                                  index than a required positional argument: {:?} index {:?}",
                        p.name,
                        p.index
                    );
                } else if p.is_set(ArgSettings::Required) && !p.is_set(ArgSettings::Last) {
                    // Args that .last(true) don't count since they can be required and have
                    // positionals with a lower index that aren't required
                    // Imagine: prog <req1> [opt1] -- <req2>
                    // Both of these are valid invocations:
                    //      $ prog r1 -- r2
                    //      $ prog r1 o1 -- r2
                    found = true;
                    continue;
                }
            }
        }
        if positionals!(self.app).any(|p| {
            p.is_set(ArgSettings::Last) && p.is_set(ArgSettings::Required)
        }) && self.has_subcommands() && !self.is_set(AS::SubcommandsNegateReqs)
        {
            panic!(
                "Having a required positional argument with .last(true) set *and* child \
            subcommands without setting SubcommandsNegateReqs isn't compatible."
            );
        }

        true
    }

    //
    // -------- Derive and Propagate
    //

    #[cfg_attr(feature = "lints", allow(needless_borrow))]
    pub fn derive_display_order(&mut self) {
        if self.is_set(AS::DeriveDisplayOrder) {
            let unified = self.is_set(AS::UnifiedHelpMessage);
            for (i, a) in parser_args_mut!(self)
                .enumerate()
                .filter(|&(_, ref a)| a.display_order == 999)
            {
                a.display_order = if unified { a._unified_order } else { i };
            }
            for (i, sc) in &mut self.app.subcommands.iter_mut().enumerate().filter(
                |&(_, ref sc)| {
                    sc.display_order == 999
                },
            )
            {
                sc.display_order = i;
            }
        }
        // @TODO-v3-alpha: display order for children shouldn't be derived unless we need to display
        // it!
        //
        // for sc in &mut self.app.subcommands {
        //     sc.derive_display_order();
        // }
    }

    // @TODO-v3-alpha: This should only propagate to a particular SC, not all
    pub fn propagate_settings_to(&mut self, sc_name: &str) {
        debugln!(
            "Parser::propogate_settings: self={}, g_settings={:#?}",
            self.app.name,
            self.app._g_settings
        );
        if let Some(sc) = self.app
            .subcommands
            .iter_mut()
            .find(|sc| sc.name == sc_name)
        {
            debugln!(
                "Parser::propogate_settings: sc={}, settings={:#?}, g_settings={:#?}",
                sc.name,
                sc._settings,
                sc._g_settings
            );
            // We have to create a new scope in order to tell rustc the borrow of `sc` is
            // done and to recursively call this method
            {
                let vsc = self.app._settings.is_set(AS::VersionlessSubcommands);
                let gv = self.app._settings.is_set(AS::GlobalVersion);

                if vsc {
                    sc.setb(AS::DisableVersion);
                }
                if gv && sc.version.is_none() && self.app.version.is_some() {
                    sc.setb(AS::GlobalVersion);
                    sc.version = Some(self.app.version.unwrap());
                }
                for set in &self.app.settings {
                    sc.settings.push(*set);
                }
                for set in &self.app.global_settings {
                    sc.settings.push(*set);
                    sc.global_settings.push(*set);
                }
                sc.term_width = self.app.term_width;
                sc.max_term_width = self.app.max_term_width;
            }
        }
    }

    //
    // -------- Parse!!! Yay!
    //

    // The actual parsing function
    #[cfg_attr(feature = "lints", allow(while_let_on_iterator, collapsible_if))]
    pub fn get_matches_with<I, T>(
        &mut self,
        matcher: &mut ArgMatcher<'a>,
        it: &mut Peekable<I>,
    ) -> ClapResult<()>
    where
        I: Iterator<Item = T>,
        T: Into<OsString> + Clone,
    {
        debugln!("Parser::get_matches_with;");

        // @TODO-v3-alpha:
        // globals should only be propagated on completions...consider moving this call
        // self.propagate_globals();

        self.derive_display_order();

        // Verify all positional assertions pass
        debug_assert!(self.app_debug_asserts());
        if positionals!(self.app).any(|a| {
            a.is_set(ArgSettings::Multiple) && (a.index.unwrap() != positionals!(self.app).count())
        }) &&
            positionals!(self.app).last()
                .map_or(false, |p| !p.is_set(ArgSettings::Last))
        {
            self.set(AS::LowIndexMultiplePositional);
        }
        let has_args = self.has_args();

        let mut subcmd_name: Option<String> = None;
        let mut needs_val_of: ParseResult<'a> = ParseResult::NotFound;
        let mut pos_counter = 1;
        while let Some(arg) = it.next() {
            let arg_os = arg.into();
            debugln!(
                "Parser::get_matches_with: Begin parsing '{:?}' ({:?})",
                arg_os,
                &*arg_os.as_bytes()
            );

            self.unset(AS::ValidNegNumFound);
            // Is this a new argument, or values from a previous option?
            let starts_new_arg = self.is_new_arg(&arg_os, needs_val_of);
            if arg_os.starts_with(b"--") && arg_os.len_() == 2 && starts_new_arg {
                debugln!("Parser::get_matches_with: setting TrailingVals=true");
                self.set(AS::TrailingValues);
                continue;
            }

            // Has the user already passed '--'? Meaning only positional args follow
            if !self.is_set(AS::TrailingValues) {
                // Does the arg match a subcommand name, or any of it's aliases (if defined)
                {
                    let (is_match, sc_name) = self.possible_subcommand(&arg_os);
                    debugln!(
                        "Parser::get_matches_with: possible_sc={:?}, sc={:?}",
                        is_match,
                        sc_name
                    );
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
                    if let ParseResult::Opt(name) = needs_val_of {
                        // Check to see if parsing a value from a previous arg
                        let arg = opts!(self.app).find(|o| o.name == name)
                            .expect(INTERNAL_ERROR_MSG);
                        // get the Opt so we can check the settings
                        needs_val_of = try!(self.add_val_to_arg(arg, &arg_os, matcher));
                        // get the next value from the iterator
                        continue;
                    }
                } else if arg_os.starts_with(b"--") {
                    needs_val_of = try!(self.parse_long_arg(matcher, &arg_os));
                    debugln!(
                        "Parser:get_matches_with: After parse_long_arg {:?}",
                        needs_val_of
                    );
                    match needs_val_of {
                        ParseResult::Flag |
                        ParseResult::Opt(..) |
                        ParseResult::ValuesDone => continue,
                        _ => (),
                    }
                } else if arg_os.starts_with(b"-") && arg_os.len_() != 1 {
                    // Try to parse short args like normal, if AllowLeadingHyphen or
                    // AllowNegativeNumbers is set, parse_short_arg will *not* throw
                    // an error, and instead return Ok(None)
                    needs_val_of = try!(self.parse_short_arg(matcher, &arg_os));
                    // If it's None, we then check if one of those two AppSettings was set
                    debugln!(
                        "Parser:get_matches_with: After parse_short_arg {:?}",
                        needs_val_of
                    );
                    match needs_val_of {
                        ParseResult::MaybeNegNum => {
                            if !(arg_os.to_string_lossy().parse::<i64>().is_ok() ||
                                     arg_os.to_string_lossy().parse::<f64>().is_ok())
                            {
                                return Err(ClapError::unknown_argument(
                                    &*arg_os.to_string_lossy(),
                                    "",
                                    &*self.create_error_usage(matcher, None),
                                    self.color(),
                                ));
                            }
                        }
                        ParseResult::Opt(..) |
                        ParseResult::Flag |
                        ParseResult::ValuesDone => continue,
                        _ => (),
                    }
                }

                if !(self.is_set(AS::ArgsNegateSubcommands) && self.is_set(AS::ValidArgFound)) &&
                    !self.is_set(AS::InferSubcommands)
                {
                    if let Some(cdate) = suggestions::did_you_mean(
                        &*arg_os.to_string_lossy(),
                        sc_names!(self.app),
                    )
                    {
                        return Err(ClapError::invalid_subcommand(
                            arg_os.to_string_lossy().into_owned(),
                            cdate,
                            self.app.bin_name.as_ref().unwrap_or(&self.app.name),
                            &*self.create_error_usage(matcher, None),
                            self.color(),
                        ));
                    }
                }
            }

            let low_index_mults = self.is_set(AS::LowIndexMultiplePositional) &&
                pos_counter == (self.positionals.len() - 1);
            let missing_pos = self.is_set(AS::AllowMissingPositional) &&
                pos_counter == (self.positionals.len() - 1);
            debugln!(
                "Parser::get_matches_with: Positional counter...{}",
                pos_counter
            );
            debugln!(
                "Parser::get_matches_with: Low index multiples...{:?}",
                low_index_mults
            );
            if low_index_mults || missing_pos {
                if let Some(na) = it.peek() {
                    let n = (*na).clone().into();
                    needs_val_of = if needs_val_of != ParseResult::ValuesDone {
                        if let Some(p) = self.positionals.get(pos_counter) {
                            ParseResult::Pos(p)
                        } else {
                            ParseResult::ValuesDone
                        }
                    } else {
                        ParseResult::ValuesDone
                    };
                    let sc_match = {
                        self.possible_subcommand(&n).0
                    };
                    if self.is_new_arg(&n, needs_val_of) || sc_match ||
                        suggestions::did_you_mean(&n.to_string_lossy(), sc_names!(self.app)).is_some()
                    {
                        debugln!("Parser::get_matches_with: Bumping the positional counter...");
                        pos_counter += 1;
                    }
                } else {
                    debugln!("Parser::get_matches_with: Bumping the positional counter...");
                    pos_counter += 1;
                }
            } else if self.is_set(AS::ContainsLast) && self.is_set(AS::TrailingValues) {
                // Came to -- and one postional has .last(true) set, so we go immediately
                // to the last (highest index) positional
                debugln!("Parser::get_matches_with: .last(true) and --, setting last pos");
                pos_counter = self.positionals.len();
            }
            if let Some(p_name) = self.positionals.get(pos_counter) {
                let p = args!(self.app)
                    .find(|pa| &pa.name == p_name)
                    .expect(INTERNAL_ERROR_MSG);
                if p.is_set(ArgSettings::Last) && !self.is_set(AS::TrailingValues) {
                    return Err(ClapError::unknown_argument(
                        &*arg_os.to_string_lossy(),
                        "",
                        &*self.create_error_usage(matcher, None),
                        self.color(),
                    ));
                }
                parse_positional!(self, p, arg_os, pos_counter, matcher);
                self.app._settings.set(AS::ValidArgFound);
            } else if self.is_set(AS::AllowExternalSubcommands) {
                // Get external subcommand name
                let sc_name = match arg_os.to_str() {
                    Some(s) => s.to_string(),
                    None => {
                        if !self.is_set(AS::StrictUtf8) {
                            return Err(ClapError::invalid_utf8(
                                &*self.create_error_usage(matcher, None),
                                self.color(),
                            ));
                        }
                        arg_os.to_string_lossy().into_owned()
                    }
                };

                // Collect the external subcommand args
                let mut sc_m = ArgMatcher::new();
                while let Some(v) = it.next() {
                    let a = v.into();
                    if a.to_str().is_none() && !self.is_set(AS::StrictUtf8) {
                        return Err(ClapError::invalid_utf8(
                            &*self.create_error_usage(matcher, None),
                            self.color(),
                        ));
                    }
                    sc_m.add_val_to("", &a);
                }

                matcher.subcommand(SubCommand {
                    name: sc_name,
                    matches: sc_m.into(),
                });
            } else if !(self.is_set(AS::AllowLeadingHyphen) ||
                            self.is_set(AS::AllowNegativeNumbers)) &&
                       !self.is_set(AS::InferSubcommands)
            {
                return Err(ClapError::unknown_argument(
                    &*arg_os.to_string_lossy(),
                    "",
                    &*self.create_error_usage(matcher, None),
                    self.color(),
                ));
            } else if !has_args || self.is_set(AS::InferSubcommands) && self.has_subcommands() {
                if let Some(cdate) = suggestions::did_you_mean(
                    &*arg_os.to_string_lossy(),
                    sc_names!(self.app),
                )
                {
                    return Err(ClapError::invalid_subcommand(
                        arg_os.to_string_lossy().into_owned(),
                        cdate,
                        self.app.bin_name.as_ref().unwrap_or(&self.app.name),
                        &*self.create_error_usage(matcher, None),
                        self.color(),
                    ));
                } else {
                    return Err(ClapError::unrecognized_subcommand(
                        arg_os.to_string_lossy().into_owned(),
                        self.app.bin_name.as_ref().unwrap_or(&self.app.name),
                        self.color(),
                    ));
                }
            }
        }

        if let Some(ref pos_sc_name) = subcmd_name {
            let sc_name = {
                find_subcommand!(self.app, pos_sc_name)
                    .expect(INTERNAL_ERROR_MSG)
                    .name
                    .clone()
            };
            try!(self.parse_subcommand(&*sc_name, matcher, it));
        } else if self.is_set(AS::SubcommandRequired) {
            let bn = self.app.bin_name.as_ref().unwrap_or(&self.app.name);
            return Err(ClapError::missing_subcommand(
                bn,
                &self.create_error_usage(matcher, None),
                self.color(),
            ));
        } else if self.is_set(AS::SubcommandRequiredElseHelp) {
            debugln!("Parser::get_matches_with: SubcommandRequiredElseHelp=true");
            let mut out = vec![];
            try!(self.write_help_err(&mut out));
            return Err(ClapError {
                message: String::from_utf8_lossy(&*out).into_owned(),
                kind: ErrorKind::MissingArgumentOrSubcommand,
                info: None,
            });
        }

        self.validate(needs_val_of, subcmd_name, matcher)
    }

    // Checks if the arg matches a subcommand name, or any of it's aliases (if defined)
    fn possible_subcommand(&self, arg_os: &OsStr) -> (bool, Option<&str>) {
        debugln!("Parser::possible_subcommand: arg={:?}", arg_os);
        fn starts(h: &str, n: &OsStr) -> bool {
            #[cfg(not(target_os = "windows"))]
            use std::os::unix::ffi::OsStrExt;
            #[cfg(target_os = "windows")]
            use osstringext::OsStrExt3;

            let n_bytes = n.as_bytes();
            let h_bytes = OsStr::new(h).as_bytes();

            h_bytes.starts_with(n_bytes)
        }

        if self.is_set(AS::ArgsNegateSubcommands) && self.is_set(AS::ValidArgFound) {
            return (false, None);
        }
        if !self.is_set(AS::InferSubcommands) {
            if let Some(sc) = find_subcommand!(self.app, arg_os) {
                return (true, Some(&sc.name));
            }
        } else {
            let v = self.app
                .subcommands
                .iter()
                .filter(|s| {
                    starts(&s.name[..], &*arg_os) ||
                        (s.aliases.is_some() &&
                             s.aliases.as_ref() // @TODO-v3-alpha: consider visible aliases too
                                 .unwrap()
                                 .iter()
                                 .filter(|a| starts(a, &*arg_os))
                                 .count() == 1)
                })
                .map(|sc| &sc.name)
                .collect::<Vec<_>>();

            if v.len() == 1 {
                return (true, Some(v[0]));
            }
        }
        (false, None)
    }

    fn parse_help_subcommand<I, T>(&self, it: &mut I) -> ClapResult<ParseResult<'a>>
    where
        I: Iterator<Item = T>,
        T: Into<OsString>,
    {
        debugln!("Parser::parse_help_subcommand;");
        let cmds: Vec<OsString> = it.map(|c| c.into()).collect();
        let mut help_help = false;
        let mut bin_name = self.app.bin_name.as_ref().unwrap_or(&self.app.name).clone();
        let mut sc = {
            // @PERF cloning all these Apps ins't great, but since it's just displaying the help
            // message there are bigger fish to fry
            let mut sc = self.app.clone();
            for (i, cmd) in cmds.iter().enumerate() {
                if &*cmd.to_string_lossy() == "help" {
                    // cmd help help
                    help_help = true;
                    break; // Maybe?
                }
                if let Some(c) = find_subcommand_cloned!(sc, cmd) {
                    sc = c;
                    if i == cmds.len() - 1 {
                        break;
                    }
                } else if let Some(c) = find_subcommand_cloned!(sc, &*cmd.to_string_lossy()) {
                    sc = c;
                    if i == cmds.len() - 1 {
                        break;
                    }
                } else {
                    return Err(ClapError::unrecognized_subcommand(
                        cmd.to_string_lossy().into_owned(),
                        self.app.bin_name.as_ref().unwrap_or(&self.app.name),
                        self.color(),
                    ));
                }
                bin_name = format!("{} {}", bin_name, &*sc.name);
            }
            sc
        };
        let mut parser = Parser::new(&mut sc);
        if help_help {
            let mut pb = Arg::new("subcommand")
                .index(1)
                .setting(ArgSettings::Multiple)
                .help("The subcommand whose help message to display");
            pb._build();
            parser.positionals.insert(1, pb.name);
            parser.app._settings = parser.app._settings | self.app._g_settings;
            parser.app._g_settings = self.app._g_settings;
        }
        if parser.app.bin_name != self.app.bin_name {
            parser.app.bin_name = Some(format!("{} {}", bin_name, parser.app.name));
        }
        Err(parser._help(false))
    }

    // allow wrong self convention due to self.valid_neg_num = true and it's a private method
    #[cfg_attr(feature = "lints", allow(wrong_self_convention))]
    fn is_new_arg(&mut self, arg_os: &OsStr, needs_val_of: ParseResult<'a>) -> bool {
        debugln!(
            "Parser::is_new_arg: arg={:?}, Needs Val of={:?}",
            arg_os,
            needs_val_of
        );
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
        let arg_allows_tac = match needs_val_of {
            ParseResult::Opt(name) => {
                let o = opts!(self.app).find(|o| o.name == name)
                    .expect(INTERNAL_ERROR_MSG);
                (o.is_set(ArgSettings::AllowHyphenValues) || app_wide_settings)
            }
            ParseResult::Pos(name) => {
                let p = positionals!(self.app).find(|p| p.name == name).expect(INTERNAL_ERROR_MSG);
                (p.is_set(ArgSettings::AllowHyphenValues) || app_wide_settings)
            }
            _ => false,
        };
        debugln!(
            "Parser::is_new_arg: Arg::allow_leading_hyphen({:?})",
            arg_allows_tac
        );

        // Is this a new argument, or values from a previous option?
        let mut ret = if arg_os.starts_with(b"--") {
            debugln!("Parser::is_new_arg: -- found");
            if arg_os.len_() == 2 && !arg_allows_tac {
                return true; // We have to return true so override everything else
            } else if arg_allows_tac {
                return false;
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

    fn parse_subcommand<I, T>(
        &mut self,
        sc_name: &str,
        matcher: &mut ArgMatcher<'a>,
        it: &mut Peekable<I>,
    ) -> ClapResult<()>
    where
        I: Iterator<Item = T>,
        T: Into<OsString> + Clone,
    {
        use std::fmt::Write;
        debugln!("Parser::parse_subcommand;");
        let mut mid_string = String::new();
        if !self.is_set(AS::SubcommandsNegateReqs) {
            let mut hs: Vec<&str> = self.required.iter().map(|n| &**n).collect();
            for k in matcher.arg_names() {
                hs.push(k);
            }
            let reqs = self.get_required_usage_from(&hs, Some(matcher), None, false);

            for s in &reqs {
                write!(&mut mid_string, " {}", s).expect(INTERNAL_ERROR_MSG);
            }
        }
        mid_string.push_str(" ");
        self.propagate_settings_to(sc_name);
        if let Some(ref mut sc) = find_subcommand_mut!(self.app, sc_name)
        {
            let mut sc_matcher = ArgMatcher::new();
            // bin_name should be parent's bin_name + [<reqs>] + the sc's name separated by
            // a space
            sc._usage = Some(format!(
                "{}{}{}",
                self.app.bin_name.as_ref().unwrap_or(&String::new()),
                if self.app.bin_name.is_some() {
                    &*mid_string
                } else {
                    ""
                },
                &*sc.name
            ));
            sc.bin_name = Some(format!(
                "{}{}{}",
                self.app.bin_name.as_ref().unwrap_or(&String::new()),
                if self.app.bin_name.is_some() { " " } else { "" },
                &*sc.name
            ));
            debugln!(
                "Parser::parse_subcommand: About to parse sc={}",
                sc.name
            );
            debugln!("Parser::parse_subcommand: sc settings={:#?}", sc.settings);
            let name = sc.name.clone();
            let mut p = Parser::new(sc);
            try!(p.get_matches_with(&mut sc_matcher, it));
            matcher.subcommand(SubCommand {
                name: name,
                matches: sc_matcher.into(),
            });
        }
        Ok(())
    }

    fn parse_long_arg(
        &mut self,
        matcher: &mut ArgMatcher<'a>,
        full_arg: &OsStr,
    ) -> ClapResult<ParseResult<'a>> {
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

        if let Some(opt) = find_by_long!(self.app, arg, opts) {
            debugln!(
                "Parser::parse_long_arg: Found valid opt '{}'",
                opt.to_string()
            );
            self.app._settings.set(AS::ValidArgFound);
            let ret = try!(self.parse_opt(val, opt, val.is_some(), matcher));
            if self.cache.map_or(true, |name| name != opt.name) {
                arg_post_processing!(self, opt, matcher);
                self.cache = Some(opt.name);
            }

            return Ok(ret);
        } else if let Some(flag) = find_by_long!(self.app, arg, flags) {
            debugln!(
                "Parser::parse_long_arg: Found valid flag '{}'",
                flag.to_string()
            );
            self.app._settings.set(AS::ValidArgFound);
            // Only flags could be help or version, and we need to check the raw long
            // so this is the first point to check
            try!(self.check_for_help_and_version_str(arg));

            try!(self.parse_flag(flag, matcher));

            // Handle conflicts, requirements, etc.
            // if self.cache.map_or(true, |name| name != flag.name) {
            arg_post_processing!(self, flag, matcher);
            // self.cache = Some(flag.name);
            // }

            return Ok(ParseResult::Flag);
        } else if self.is_set(AS::AllowLeadingHyphen) {
            return Ok(ParseResult::MaybeHyphenValue);
        } else if self.is_set(AS::ValidNegNumFound) {
            return Ok(ParseResult::MaybeNegNum);
        }

        debugln!("Parser::parse_long_arg: Didn't match anything");
        self.did_you_mean_error(arg.to_str().expect(INVALID_UTF8), matcher)
            .map(|_| ParseResult::NotFound)
    }

    #[cfg_attr(feature = "lints", allow(len_zero))]
    fn parse_short_arg(
        &mut self,
        matcher: &mut ArgMatcher<'a>,
        full_arg: &OsStr,
    ) -> ClapResult<ParseResult<'a>> {
        debugln!("Parser::parse_short_arg: full_arg={:?}", full_arg);
        let arg_os = full_arg.trim_left_matches(b'-');
        let arg = arg_os.to_string_lossy();

        // If AllowLeadingHyphen is set, we want to ensure `-val` gets parsed as `-val` and not
        // `-v` `-a` `-l` assuming `v` `a` and `l` are all, or mostly, valid shorts.
        if self.is_set(AS::AllowLeadingHyphen) {
            if arg.chars().any(|c| !self.contains_short(c)) {
                debugln!(
                    "Parser::parse_short_arg: LeadingHyphenAllowed yet -{} isn't valid",
                    arg
                );
                return Ok(ParseResult::MaybeHyphenValue);
            }
        } else if self.is_set(AS::ValidNegNumFound) {
            // @TODO: Add docs about having AllowNegativeNumbers and `-2` as a valid short
            // May be better to move this to *after* not finding a valid flag/opt?
            debugln!("Parser::parse_short_arg: Valid negative num...");
            return Ok(ParseResult::MaybeNegNum);
        }

        let mut ret = ParseResult::NotFound;
        for c in arg.chars() {
            debugln!("Parser::parse_short_arg:iter:{}", c);
            // Check for matching short options, and return the name if there is no trailing
            // concatenated value: -oval
            // Option: -o
            // Value: val
            if let Some(opt) = find_by_short!(self.app, c, opts) {
                debugln!("Parser::parse_short_arg:iter:{}: Found valid opt", c);
                self.app._settings.set(AS::ValidArgFound);
                // Check for trailing concatenated value
                let p: Vec<_> = arg.splitn(2, c).collect();
                debugln!(
                    "Parser::parse_short_arg:iter:{}: p[0]={:?}, p[1]={:?}",
                    c,
                    p[0].as_bytes(),
                    p[1].as_bytes()
                );
                let i = p[0].as_bytes().len() + 1;
                let val = if p[1].as_bytes().len() > 0 {
                    debugln!(
                        "Parser::parse_short_arg:iter:{}: val={:?} (bytes), val={:?} (ascii)",
                        c,
                        arg_os.split_at(i).1.as_bytes(),
                        arg_os.split_at(i).1
                    );
                    Some(arg_os.split_at(i).1)
                } else {
                    None
                };

                // Default to "we're expecting a value later"
                let ret = try!(self.parse_opt(val, opt, false, matcher));

                if self.cache.map_or(true, |name| name != opt.name) {
                    arg_post_processing!(self, opt, matcher);
                    self.cache = Some(opt.name);
                }

                return Ok(ret);
            } else if let Some(flag) = find_by_short!(self.app, c, flags) {
                debugln!("Parser::parse_short_arg:iter:{}: Found valid flag", c);
                self.app._settings.set(AS::ValidArgFound);
                // Only flags can be help or version
                try!(self.check_for_help_and_version_char(c));
                ret = try!(self.parse_flag(flag, matcher));

                // Handle conflicts, requirements, overrides, etc.
                // Must be called here due to mutablilty
                if self.cache.map_or(true, |name| name != flag.name) {
                    arg_post_processing!(self, flag, matcher);
                    self.cache = Some(flag.name);
                }
            } else {
                let arg = format!("-{}", c);
                return Err(ClapError::unknown_argument(
                    &*arg,
                    "",
                    &*self.create_error_usage(matcher, None),
                    self.color(),
                ));
            }
        }
        Ok(ret)
    }

    fn parse_opt(
        &self,
        val: Option<&OsStr>,
        opt: &Arg<'a, 'b>,
        had_eq: bool,
        matcher: &mut ArgMatcher<'a>,
    ) -> ClapResult<ParseResult<'a>> {
        debugln!("Parser::parse_opt; opt={}, val={:?}", opt.name, val);
        debugln!("Parser::parse_opt; opt.settings={:?}", opt.settings);
        let mut has_eq = false;

        debug!("Parser::parse_opt; Checking for val...");
        if let Some(fv) = val {
            has_eq = fv.starts_with(&[b'=']) || had_eq;
            let v = fv.trim_left_matches(b'=');
            if !opt.is_set(ArgSettings::EmptyValues) &&
                (v.len_() == 0 || (opt.is_set(ArgSettings::RequireEquals) && !has_eq))
            {
                sdebugln!("Found Empty - Error");
                return Err(ClapError::empty_value(
                    opt,
                    &*self.create_error_usage(matcher, None),
                    self.color(),
                ));
            }
            sdebugln!("Found - {:?}, len: {}", v, v.len_());
            debugln!(
                "Parser::parse_opt: {:?} contains '='...{:?}",
                fv,
                fv.starts_with(&[b'='])
            );
            try!(self.add_val_to_arg(opt, v, matcher));
        } else if opt.is_set(ArgSettings::RequireEquals) && !opt.is_set(ArgSettings::EmptyValues) {
            sdebugln!("None, but requires equals...Error");
            return Err(ClapError::empty_value(
                opt,
                &*self.create_error_usage(matcher, None),
                self.color(),
            ));

        } else {
            sdebugln!("None");
        }

        matcher.inc_occurrence_of(opt.name);
        // Increment or create the group "args"
        self.groups_for_arg(opt.name)
            .and_then(|vec| Some(matcher.inc_occurrences_of(&*vec)));

        if val.is_none() ||
            !has_eq &&
                (opt.is_set(ArgSettings::Multiple) && !opt.is_set(ArgSettings::RequireDelimiter) &&
                     matcher.needs_more_vals(opt))
        {
            debugln!("Parser::parse_opt: More arg vals required...");
            return Ok(ParseResult::Opt(opt.name));
        }
        debugln!("Parser::parse_opt: More arg vals not required...");
        Ok(ParseResult::ValuesDone)
    }

    fn add_val_to_arg(
        &self,
        arg: &Arg<'a, 'b>,
        val: &OsStr,
        matcher: &mut ArgMatcher<'a>,
    ) -> ClapResult<ParseResult<'a>> {
        debugln!("Parser::add_val_to_arg; arg={}, val={:?}", arg.name, val);
        debugln!(
            "Parser::add_val_to_arg; trailing_vals={:?}, DontDelimTrailingVals={:?}",
            self.is_set(AS::TrailingValues),
            self.is_set(AS::DontDelimitTrailingValues)
        );
        if !(self.is_set(AS::TrailingValues) && self.is_set(AS::DontDelimitTrailingValues)) {
            if let Some(delim) = arg.value_delimiter {
                if val.is_empty_() {
                    Ok(try!(self.add_single_val_to_arg(arg, val, matcher)))
                } else {
                    let mut iret = ParseResult::ValuesDone;
                    for v in val.split(delim as u32 as u8) {
                        iret = try!(self.add_single_val_to_arg(arg, v, matcher));
                    }
                    // If there was a delimiter used, we're not looking for more values
                    if val.contains_byte(delim as u32 as u8) ||
                        arg.is_set(ArgSettings::RequireDelimiter)
                    {
                        iret = ParseResult::ValuesDone;
                    }
                    Ok(iret)
                }
            } else {
                self.add_single_val_to_arg(arg, val, matcher)
            }
        } else {
            self.add_single_val_to_arg(arg, val, matcher)
        }
    }

    fn add_single_val_to_arg(
        &self,
        arg: &Arg<'a, 'b>,
        v: &OsStr,
        matcher: &mut ArgMatcher<'a>,
    ) -> ClapResult<ParseResult<'a>> {
        debugln!("Parser::add_single_val_to_arg;");
        debugln!("Parser::add_single_val_to_arg: adding val...{:?}", v);
        if let Some(t) = arg.value_terminator {
            if t == v {
                return Ok(ParseResult::ValuesDone);
            }
        }
        matcher.add_val_to(arg.name, v);

        // Increment or create the group "args"
        if let Some(grps) = self.groups_for_arg(arg.name) {
            for grp in grps {
                matcher.add_val_to(&*grp, v);
            }
        }

        if matcher.needs_more_vals(arg) {
            return Ok(ParseResult::Opt(arg.name));
        }
        Ok(ParseResult::ValuesDone)
    }

    fn parse_flag(
        &self,
        flag: &Arg<'a, 'b>,
        matcher: &mut ArgMatcher<'a>,
    ) -> ClapResult<ParseResult<'a>> {
        debugln!("Parser::parse_flag;");

        matcher.inc_occurrence_of(flag.name);
        // Increment or create the group "args"
        self.groups_for_arg(flag.name)
            .and_then(|vec| Some(matcher.inc_occurrences_of(&*vec)));

        Ok(ParseResult::Flag)
    }

    // Retrieves the names of all args the user has supplied thus far, except required ones
    // because those will be listed in self.required
    fn check_for_help_and_version_str(&self, arg: &OsStr) -> ClapResult<()> {
        debugln!("Parser::check_for_help_and_version_str;");
        debug!(
            "Parser::check_for_help_and_version_str: Checking if --{} is help or version...",
            arg.to_str().unwrap()
        );
        if arg == "help" && self.is_set(AS::NeedsLongHelp) {
            sdebugln!("Help");
            return Err(self._help(true));
        }
        if arg == "version" && self.is_set(AS::NeedsLongVersion) {
            sdebugln!("Version");
            return Err(self._version(true));
        }
        sdebugln!("Neither");

        Ok(())
    }

    fn check_for_help_and_version_char(&self, arg: char) -> ClapResult<()> {
        debugln!("Parser::check_for_help_and_version_char;");
        debug!(
            "Parser::check_for_help_and_version_char: Checking if -{} is help or version...",
            arg
        );
        if let Some(h) = self.app.help_short {
            if arg == h && self.is_set(AS::NeedsLongHelp) {
                sdebugln!("Help");
                return Err(self._help(false));
            }
        }
        if let Some(v) = self.app.version_short {
            if arg == v && self.is_set(AS::NeedsLongVersion) {
                sdebugln!("Version");
                return Err(self._version(false));
            }
        }
        sdebugln!("Neither");
        Ok(())
    }

    //
    // ------ Finalization Phase
    //

    pub fn add_defaults(&mut self, matcher: &mut ArgMatcher<'a>) -> ClapResult<()> {
        macro_rules! add_val {
            (@default $_self:ident, $a:ident, $m:ident) => {
                if let Some(ref val) = $a.default_value {
                    if $m.get($a.name).is_none() {
                        try!($_self.add_val_to_arg($a, OsStr::new(val), $m));

                        if $_self.cache.map_or(true, |name| name != $a.name) {
                            arg_post_processing!($_self, $a, $m);
                            $_self.cache = Some($a.name);
                        }
                    }
                }
            };
            ($_self:ident, $a:ident, $m:ident) => {
                if let Some(ref vm) = $a.default_value_ifs {
                    let mut done = false;
                    if $m.get($a.name).is_none() {
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
                                if $_self.cache.map_or(true, |name| name != $a.name) {
                                    arg_post_processing!($_self, $a, $m);
                                    $_self.cache = Some($a.name);
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

        for o in opts!(self.app) {
            add_val!(self, o, matcher);
        }
        for p in positionals!(self.app) {
            add_val!(self, p, matcher);
        }
        Ok(())
    }

    // Didn't match a flag or option
    fn did_you_mean_error(&self, arg: &str, matcher: &mut ArgMatcher<'a>) -> ClapResult<()> {
        // @PERF there should be a way to do this without collecting but since it's
        // displaying an error, it's ok-ish
        let longs: Vec<&str> = longs!(self.app).map(|l| *l).collect(); 
        let suffix =
            suggestions::did_you_mean_flag_suffix(arg, longs.iter(), &self.app.subcommands);

        // Add the arg to the matches to build a proper usage string
        if let Some(name) = suffix.1 {
            if let Some(opt) = find_by_long!(self.app, name, opts) {
                self.groups_for_arg(&*opt.name)
                    .and_then(|grps| Some(matcher.inc_occurrences_of(&*grps)));
                matcher.insert(&*opt.name);
            } else if let Some(flg) = find_by_long!(self.app, name, flags) {
                self.groups_for_arg(&*flg.name)
                    .and_then(|grps| Some(matcher.inc_occurrences_of(&*grps)));
                matcher.insert(&*flg.name);
            }
        }

        let used_arg = format!("--{}", arg);
        Err(ClapError::unknown_argument(
            &*used_arg,
            &*suffix.0,
            &*self.create_error_usage(matcher, None),
            self.color(),
        ))
    }

    //
    // ------- Display Help / Version --------
    //

    fn _help(&self, mut use_long: bool) -> ClapError {
        debugln!("Parser::_help: use_long={:?}", use_long);
        use_long = use_long && self.use_long_help();
        let mut buf = vec![];
        let mut hw = HelpWriter::new(self.clone(), false);
        match hw._write_help(&mut buf, use_long) {
            Err(e) => e,
            _ => ClapError {
                message: unsafe { String::from_utf8_unchecked(buf) },
                kind: ErrorKind::HelpDisplayed,
                info: None,
            },
        }
    }

    fn _version(&self, use_long: bool) -> ClapError {
        debugln!("Parser::_version: ");
        let out = io::stdout();
        let mut buf_w = BufWriter::new(out.lock());
        match self.print_version(&mut buf_w, use_long) {
            Err(e) => e,
            _ => ClapError {
                message: String::new(),
                kind: ErrorKind::VersionDisplayed,
                info: None,
            },
        }
    }

    fn print_version<W: Write>(&self, w: &mut W, use_long: bool) -> ClapResult<()> {
        try!(self.write_version(w, use_long));
        w.flush().map_err(ClapError::from)
    }

    pub fn write_version<W: Write>(&self, w: &mut W, use_long: bool) -> io::Result<()> {
        let ver = if use_long {
            self.app
                .long_version
                .unwrap_or_else(|| self.app.version.unwrap_or(""))
        } else {
            self.app
                .version
                .unwrap_or_else(|| self.app.long_version.unwrap_or(""))
        };
        if let Some(bn) = self.app.bin_name.as_ref() {
            if bn.contains(' ') {
                // Incase we're dealing with subcommands i.e. git mv is translated to git-mv
                write!(w, "{} {}", bn.replace(" ", "-"), ver)
            } else {
                write!(w, "{} {}", &self.app.name[..], ver)
            }
        } else {
            write!(w, "{} {}", &self.app.name[..], ver)
        }
    }

    pub fn write_help_err<W: Write>(&self, w: &mut W) -> ClapResult<()> {
        let mut hw = HelpWriter::new(self, true);
        hw.write_help(w)
    }

    //
    // -------- Getters / Setters
    //

    #[inline]
    pub fn is_set(&self, s: AS) -> bool { self.app._settings.is_set(s) }

    #[inline]
    pub fn set(&mut self, s: AS) { self.app._settings.set(s) }

    #[inline]
    pub fn unset(&mut self, s: AS) { self.app._settings.unset(s) }

    //
    // ------ Queries -----------
    //

    pub fn groups_for_arg(&self, name: &str) -> Option<Vec<&'a str>> {
        debugln!("Parser::groups_for_arg: name={}", name);

        if self.app.groups.is_empty() {
            debugln!("Parser::groups_for_arg: No groups defined");
            return None;
        }
        let mut res = vec![];
        debugln!("Parser::groups_for_arg: Searching through groups...");
        for grp in &self.app.groups {
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

    pub fn args_in_group(&self, group: &str) -> Vec<String> {
        let mut g_vec = vec![];
        let mut args = vec![];

        for n in &self.app.groups
            .iter()
            .find(|g| g.name == group)
            .expect(INTERNAL_ERROR_MSG)
            .args
        {
            if let Some(f) = flags!(self.app).find(|f| &f.name == n) {
                args.push(f.to_string());
            } else if let Some(f) = opts!(self.app).find(|o| &o.name == n) {
                args.push(f.to_string());
            } else if let Some(p) = positionals!(self.app).find(|p| &p.name == n) {
                args.push(p.name.to_owned());
            } else {
                g_vec.push(n);
            }
        }

        for av in g_vec.iter().map(|g| self.args_in_group(g)) {
            args.extend(av);
        }
        args.dedup();
        args.iter().map(ToOwned::to_owned).collect()
    }

    pub fn arg_names_in_group(&self, group: &str) -> Vec<&'a str> {
        let mut g_vec = vec![];
        let mut args = vec![];

        for n in &self.app.groups
            .iter()
            .find(|g| g.name == group)
            .expect(INTERNAL_ERROR_MSG)
            .args
        {
            if self.app.groups.iter().any(|g| g.name == *n) {
                args.extend(self.arg_names_in_group(n));
                g_vec.push(*n);
            } else if !args.contains(n) {
                args.push(*n);
            }
        }

        args.iter().map(|s| *s).collect()
    }

    #[cfg_attr(feature = "cargo-clippy", allow(let_and_return))]
    fn use_long_help(&self) -> bool {
        let ul = args!(self.app).any(|f| f.long_help.is_some()) ||
            self.app
                .subcommands
                .iter()
                .any(|s| s.long_about.is_some());
        debugln!("Parser::use_long_help: ret={:?}", ul);
        ul
    }

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

    #[inline]
    fn contains_long(&self, l: &str) -> bool { longs!(self.app).any(|arg_l| arg_l == &l) }

    #[inline]
    fn contains_short(&self, s: char) -> bool { shorts!(self.app).any(|arg_s| arg_s == &s) }

    #[inline]
    pub fn has_args(&self) -> bool { args!(self.app).next().is_some() }

    #[inline]
    pub fn has_opts(&self) -> bool { opts!(self.app).next().is_some() }

    #[inline]
    pub fn has_flags(&self) -> bool { flags!(self.app).next().is_some() }

    #[inline]
    pub fn has_positionals(&self) -> bool { positionals!(self.app).next().is_some() }

    #[inline]
    pub fn has_subcommands(&self) -> bool { !self.app.subcommands.is_empty() }

    // #[inline]
    // pub fn has_visible_opts(&self) -> bool {
    //     opts!(self.app).any(|o| !o._settings.is_set(ArgSettings::Hidden))
    // }

    // #[inline]
    // pub fn has_visible_flags(&self) -> bool {
    //     flags!(self.app).any(|f| !f._settings.is_set(ArgSettings::Hidden))
    // }

    // #[inline]
    // pub fn has_visible_positionals(&self) -> bool {
    //     positionals!(self.app).any(|p| !p._settings.is_set(ArgSettings::Hidden))
    // }

    #[inline]
    pub fn has_visible_subcommands(&self) -> bool {
        if self.app.subcommands.is_empty() {
            return false;
        }
        self.app.subcommands.iter().any(|s| !s.is_set(AS::Hidden))
    }
}
