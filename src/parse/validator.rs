// Internal
use crate::{
    build::{AppSettings as AS, Arg, ArgSettings},
    output::Usage,
    parse::{
        errors::{Error, ErrorKind, Result as ClapResult},
        ArgMatcher, MatchedArg, ParseResult, Parser, ValueType,
    },
    util::{ChildGraph, Id},
    INTERNAL_ERROR_MSG, INVALID_UTF8,
};

pub(crate) struct Validator<'b, 'c, 'z>
where
    'b: 'c,
    'c: 'z,
{
    p: &'z mut Parser<'b, 'c>,
    c: ChildGraph<Id>,
}

impl<'b, 'c, 'z> Validator<'b, 'c, 'z> {
    pub(crate) fn new(p: &'z mut Parser<'b, 'c>) -> Self {
        Validator {
            p,
            c: ChildGraph::with_capacity(5),
        }
    }

    pub(crate) fn validate(
        &mut self,
        needs_val_of: ParseResult,
        is_subcmd: bool,
        matcher: &mut ArgMatcher,
    ) -> ClapResult<()> {
        debug!("Validator::validate");
        let mut reqs_validated = false;
        self.p.add_env(matcher)?;
        self.p.add_defaults(matcher)?;
        if let ParseResult::Opt(a) = needs_val_of {
            debug!("Validator::validate: needs_val_of={:?}", a);
            self.validate_required(matcher)?;

            let o = &self.p.app[&a];
            reqs_validated = true;
            let should_err = if let Some(v) = matcher.0.args.get(&o.id) {
                v.vals.is_empty() && !(o.min_vals.is_some() && o.min_vals.unwrap() == 0)
            } else {
                true
            };
            if should_err {
                return Err(Error::empty_value(
                    o,
                    &*Usage::new(self.p).create_usage_with_title(&[]),
                    self.p.app.color(),
                )?);
            }
        }

        if matcher.is_empty()
            && matcher.subcommand_name().is_none()
            && self.p.is_set(AS::ArgRequiredElseHelp)
        {
            let message = self.p.write_help_err()?;
            return Err(Error {
                cause: String::new(),
                message,
                kind: ErrorKind::MissingArgumentOrSubcommand,
                info: None,
            });
        }
        self.validate_conflicts(matcher)?;
        if !(self.p.is_set(AS::SubcommandsNegateReqs) && is_subcmd || reqs_validated) {
            self.validate_required(matcher)?;
            self.validate_required_unless(matcher)?;
        }
        self.validate_matched_args(matcher)?;

        Ok(())
    }

    fn validate_arg_values(
        &self,
        arg: &Arg,
        ma: &MatchedArg,
        matcher: &ArgMatcher,
    ) -> ClapResult<()> {
        debug!("Validator::validate_arg_values: arg={:?}", arg.name);
        for val in &ma.vals {
            if self.p.is_set(AS::StrictUtf8) && val.to_str().is_none() {
                debug!(
                    "Validator::validate_arg_values: invalid UTF-8 found in val {:?}",
                    val
                );
                return Err(Error::invalid_utf8(
                    &*Usage::new(self.p).create_usage_with_title(&[]),
                    self.p.app.color(),
                )?);
            }
            if !arg.possible_vals.is_empty() {
                debug!(
                    "Validator::validate_arg_values: possible_vals={:?}",
                    arg.possible_vals
                );
                let val_str = val.to_string_lossy();
                let ok = if arg.is_set(ArgSettings::IgnoreCase) {
                    arg.possible_vals
                        .iter()
                        .any(|pv| pv.eq_ignore_ascii_case(&val_str))
                } else {
                    arg.possible_vals.contains(&&*val_str)
                };
                if !ok {
                    let used: Vec<Id> = matcher
                        .arg_names()
                        .filter(|&n| {
                            self.p.app.find(n).map_or(true, |a| {
                                !(a.is_set(ArgSettings::Hidden) || self.p.required.contains(&a.id))
                            })
                        })
                        .cloned()
                        .collect();
                    return Err(Error::invalid_value(
                        val_str,
                        &arg.possible_vals,
                        arg,
                        &*Usage::new(self.p).create_usage_with_title(&*used),
                        self.p.app.color(),
                    )?);
                }
            }
            if !arg.is_set(ArgSettings::AllowEmptyValues)
                && val.is_empty()
                && matcher.contains(&arg.id)
            {
                debug!("Validator::validate_arg_values: illegal empty val found");
                return Err(Error::empty_value(
                    arg,
                    &*Usage::new(self.p).create_usage_with_title(&[]),
                    self.p.app.color(),
                )?);
            }
            if let Some(ref vtor) = arg.validator {
                debug!("Validator::validate_arg_values: checking validator...");
                if let Err(e) = vtor(&*val.to_string_lossy()) {
                    debug!("error");
                    return Err(Error::value_validation(Some(arg), &e, self.p.app.color())?);
                } else {
                    debug!("good");
                }
            }
            if let Some(ref vtor) = arg.validator_os {
                debug!("Validator::validate_arg_values: checking validator_os...");
                if let Err(e) = vtor(val) {
                    debug!("error");
                    return Err(Error::value_validation(
                        Some(arg),
                        &(*e).to_string(),
                        self.p.app.color(),
                    )?);
                } else {
                    debug!("good");
                }
            }
        }
        Ok(())
    }

    fn build_conflict_err(&self, name: &Id, matcher: &ArgMatcher) -> ClapResult<()> {
        debug!("Validator::build_conflict_err: name={:?}", name);
        let usg = Usage::new(self.p).create_usage_with_title(&[]);
        if let Some(other_arg) = self.p.app.find(name) {
            for k in matcher.arg_names() {
                if let Some(a) = self.p.app.find(k) {
                    if a.blacklist.contains(&name) {
                        return Err(Error::argument_conflict(
                            a,
                            Some(other_arg.to_string()),
                            &*usg,
                            self.p.app.color(),
                        )?);
                    }
                }
            }
        } else if let Some(g) = self.p.app.groups.iter().find(|x| x.id == *name) {
            let args_in_group = self.p.app.unroll_args_in_group(&g.id);
            let first = matcher
                .arg_names()
                .find(|x| args_in_group.contains(x))
                .expect(INTERNAL_ERROR_MSG);
            let c_with = matcher
                .arg_names()
                .find(|x| x != &first && args_in_group.contains(x))
                .map(|x| self.p.app[x].to_string());
            debug!("Validator::build_conflict_err: c_with={:?}:group", c_with);
            return Err(Error::argument_conflict(
                &self.p.app[first],
                c_with,
                &*usg,
                self.p.app.color(),
            )?);
        }

        panic!(INTERNAL_ERROR_MSG);
    }

    fn validate_conflicts(&mut self, matcher: &mut ArgMatcher) -> ClapResult<()> {
        debug!("Validator::validate_conflicts");
        self.validate_exclusive(matcher)?;
        self.gather_conflicts(matcher);

        for name in self.c.iter() {
            debug!("Validator::validate_conflicts:iter:{:?}", name);
            let mut should_err = false;
            if let Some(g) = self
                .p
                .app
                .groups
                .iter()
                .filter(|g| !g.multiple)
                .find(|g| &g.id == name)
            {
                let conf_with_self = self
                    .p
                    .app
                    .unroll_args_in_group(&g.id)
                    .iter()
                    .filter(|&a| matcher.contains(a))
                    .count()
                    > 1;

                let conf_with_arg = g.conflicts.iter().any(|x| matcher.contains(x));

                let arg_conf_with_gr = matcher
                    .arg_names()
                    .filter_map(|x| self.p.app.find(x))
                    .any(|x| x.blacklist.iter().any(|c| *c == g.id));

                should_err = conf_with_self || conf_with_arg || arg_conf_with_gr;
            } else if let Some(ma) = matcher.get(name) {
                debug!(
                    "Validator::validate_conflicts:iter:{:?}: matcher contains it...",
                    name
                );
                should_err = ma.occurs > 0;
            }
            if should_err {
                return self.build_conflict_err(name, matcher);
            }
        }
        Ok(())
    }

    fn validate_exclusive(&mut self, matcher: &mut ArgMatcher) -> ClapResult<()> {
        debug!("Validator::validate_exclusive");
        let args_count = matcher.arg_names().count();
        for name in matcher.arg_names() {
            debug!("Validator::validate_exclusive:iter:{:?}", name);
            if let Some(arg) = self.p.app.find(name) {
                if arg.exclusive && args_count > 1 {
                    let c_with: Option<String> = None;
                    return Err(Error::argument_conflict(
                        arg,
                        c_with,
                        Usage::new(self.p).create_usage_with_title(&[]),
                        self.p.app.color(),
                    )?);
                }
            }
        }
        Ok(())
    }

    // Gathers potential conflicts based on used argument, but without considering requirements
    // and such
    fn gather_conflicts(&mut self, matcher: &mut ArgMatcher) {
        debug!("Validator::gather_conflicts");
        for name in matcher.arg_names() {
            debug!("Validator::gather_conflicts:iter: id={:?}", name);
            // if arg is "present" only because it got default value
            // it doesn't conflict with anything
            //
            // TODO: @refactor Do it in a more elegant way
            if matcher
                .get(name)
                .map_or(false, |a| a.ty == ValueType::DefaultValue)
            {
                debug!("Validator::gather_conflicts:iter: This is default value, skipping.",);
                continue;
            }

            if let Some(arg) = self.p.app.find(name) {
                // Since an arg was used, every arg it conflicts with is added to the conflicts
                for conf in &arg.blacklist {
                    if self.p.app.find(conf).is_some() {
                        if conf != name {
                            self.c.insert(conf.clone());
                        }
                    } else {
                        // for g_arg in self.p.app.unroll_args_in_group(conf) {
                        //     if &g_arg != name {
                        self.c.insert(conf.clone()); // TODO ERROR is here - groups allow one arg but this line disallows all group args
                                                     //     }
                                                     // }
                    }
                }

                // Now we need to know which groups this arg was a member of, to add all other
                // args in that group to the conflicts, as well as any args those args conflict
                // with

                for grp in self.p.app.groups_for_arg(&name) {
                    if let Some(g) = self
                        .p
                        .app
                        .groups
                        .iter()
                        .filter(|g| !g.multiple)
                        .find(|g| g.id == grp)
                    {
                        // for g_arg in self.p.app.unroll_args_in_group(&g.name) {
                        //     if &g_arg != name {
                        self.c.insert(g.id.clone());
                        //     }
                        // }
                    }
                }
            } else if let Some(g) = self
                .p
                .app
                .groups
                .iter()
                .filter(|g| !g.multiple)
                .find(|grp| grp.id == *name)
            {
                debug!("Validator::gather_conflicts:iter:{:?}:group", name);
                self.c.insert(g.id.clone());
            }
        }
    }

    fn gather_requirements(&mut self, matcher: &ArgMatcher) {
        debug!("Validator::gather_requirements");
        for name in matcher.arg_names() {
            debug!("Validator::gather_requirements:iter:{:?}", name);
            if let Some(arg) = self.p.app.find(name) {
                for req in self.p.app.unroll_requirements_for_arg(&arg.id, matcher) {
                    self.p.required.insert(req);
                }
            } else if let Some(g) = self.p.app.groups.iter().find(|grp| grp.id == *name) {
                debug!("Validator::gather_conflicts:iter:{:?}:group", name);
                for r in &g.requires {
                    self.p.required.insert(r.clone());
                }
            }
        }
    }

    fn validate_matched_args(&self, matcher: &mut ArgMatcher) -> ClapResult<()> {
        debug!("Validator::validate_matched_args");
        for (name, ma) in matcher.iter() {
            debug!(
                "Validator::validate_matched_args:iter:{:?}: vals={:#?}",
                name, ma.vals
            );
            if let Some(arg) = self.p.app.find(name) {
                self.validate_arg_num_vals(arg, ma)?;
                self.validate_arg_values(arg, ma, matcher)?;
                self.validate_arg_requires(arg, ma, matcher)?;
                self.validate_arg_num_occurs(arg, ma)?;
            } else {
                let grp = self
                    .p
                    .app
                    .groups
                    .iter()
                    .find(|g| g.id == *name)
                    .expect(INTERNAL_ERROR_MSG);
                if grp.requires.iter().any(|n| !matcher.contains(n)) {
                    return self.missing_required_error(matcher, Some(name));
                }
            }
        }
        Ok(())
    }

    fn validate_arg_num_occurs(&self, a: &Arg, ma: &MatchedArg) -> ClapResult<()> {
        debug!(
            "Validator::validate_arg_num_occurs: {:?}={}",
            a.name, ma.occurs
        );
        if ma.occurs > 1 && !a.is_set(ArgSettings::MultipleOccurrences) {
            // Not the first time, and we don't allow multiples
            return Err(Error::unexpected_multiple_usage(
                a,
                &*Usage::new(self.p).create_usage_with_title(&[]),
                self.p.app.color(),
            )?);
        }
        Ok(())
    }

    fn validate_arg_num_vals(&self, a: &Arg, ma: &MatchedArg) -> ClapResult<()> {
        debug!("Validator::validate_arg_num_vals");
        if let Some(num) = a.num_vals {
            debug!("Validator::validate_arg_num_vals: num_vals set...{}", num);
            let should_err = if a.is_set(ArgSettings::MultipleValues) {
                ((ma.vals.len() as u64) % num) != 0
            } else {
                num != (ma.vals.len() as u64)
            };
            if should_err {
                debug!("Validator::validate_arg_num_vals: Sending error WrongNumberOfValues");
                return Err(Error::wrong_number_of_values(
                    a,
                    num,
                    if a.is_set(ArgSettings::MultipleValues) {
                        ma.vals.len() % num as usize
                    } else {
                        ma.vals.len()
                    },
                    &*Usage::new(self.p).create_usage_with_title(&[]),
                    self.p.app.color(),
                )?);
            }
        }
        if let Some(num) = a.max_vals {
            debug!("Validator::validate_arg_num_vals: max_vals set...{}", num);
            if (ma.vals.len() as u64) > num {
                debug!("Validator::validate_arg_num_vals: Sending error TooManyValues");
                return Err(Error::too_many_values(
                    ma.vals
                        .iter()
                        .last()
                        .expect(INTERNAL_ERROR_MSG)
                        .to_str()
                        .expect(INVALID_UTF8),
                    a,
                    &*Usage::new(self.p).create_usage_with_title(&[]),
                    self.p.app.color(),
                )?);
            }
        }
        let min_vals_zero = if let Some(num) = a.min_vals {
            debug!("Validator::validate_arg_num_vals: min_vals set: {}", num);
            if (ma.vals.len() as u64) < num && num != 0 {
                debug!("Validator::validate_arg_num_vals: Sending error TooFewValues");
                return Err(Error::too_few_values(
                    a,
                    num,
                    ma.vals.len(),
                    &*Usage::new(self.p).create_usage_with_title(&[]),
                    self.p.app.color(),
                )?);
            }
            num == 0
        } else {
            false
        };
        // Issue 665 (https://github.com/kbknapp/clap-rs/issues/665)
        // Issue 1105 (https://github.com/kbknapp/clap-rs/issues/1105)
        if a.is_set(ArgSettings::TakesValue) && !min_vals_zero && ma.vals.is_empty() {
            return Err(Error::empty_value(
                a,
                &*Usage::new(self.p).create_usage_with_title(&[]),
                self.p.app.color(),
            )?);
        }
        Ok(())
    }

    fn validate_arg_requires(
        &self,
        a: &Arg<'b>,
        ma: &MatchedArg,
        matcher: &ArgMatcher,
    ) -> ClapResult<()> {
        debug!("Validator::validate_arg_requires:{:?}", a.name);
        for (val, name) in &a.requires {
            if let Some(val) = val {
                let missing_req = |v| v == val && !matcher.contains(&name);
                if ma.vals.iter().any(missing_req) {
                    return self.missing_required_error(matcher, Some(&a.id));
                }
            } else if !matcher.contains(&name) {
                return self.missing_required_error(matcher, Some(&name));
            }
        }
        Ok(())
    }

    fn validate_required(&mut self, matcher: &ArgMatcher) -> ClapResult<()> {
        debug!(
            "Validator::validate_required: required={:?}",
            self.p.required
        );
        self.gather_requirements(matcher);

        for arg_or_group in self.p.required.iter().filter(|r| !matcher.contains(r)) {
            debug!("Validator::validate_required:iter:aog={:?}", arg_or_group);
            if let Some(arg) = self.p.app.find(&arg_or_group) {
                debug!("Validator::validate_required:iter: This is an arg");
                if !self.is_missing_required_ok(arg, matcher) {
                    return self.missing_required_error(matcher, None);
                }
            } else if let Some(group) = self.p.app.groups.iter().find(|g| g.id == *arg_or_group) {
                debug!("Validator::validate_required:iter: This is a group");
                if !self
                    .p
                    .app
                    .unroll_args_in_group(&group.id)
                    .iter()
                    .any(|a| matcher.contains(a))
                {
                    return self.missing_required_error(matcher, None);
                }
            }
        }

        // Validate the conditionally required args
        for a in self.p.app.args.args.iter() {
            for (other, val) in &a.r_ifs {
                if let Some(ma) = matcher.get(other) {
                    if ma.contains_val(val) && !matcher.contains(&a.id) {
                        return self.missing_required_error(matcher, Some(&a.id));
                    }
                }
            }
        }
        Ok(())
    }

    fn is_missing_required_ok(&self, a: &Arg<'b>, matcher: &ArgMatcher) -> bool {
        debug!("Validator::is_missing_required_ok: {}", a.name);
        self.validate_arg_conflicts(a, matcher) || self.p.overridden.contains(&a.id)
    }

    fn validate_arg_conflicts(&self, a: &Arg<'b>, matcher: &ArgMatcher) -> bool {
        debug!("Validator::validate_arg_conflicts: a={:?}", a.name);
        a.blacklist.iter().any(|conf| {
            matcher.contains(conf)
                || self
                    .p
                    .app
                    .groups
                    .iter()
                    .find(|g| g.id == *conf)
                    .map_or(false, |g| g.args.iter().any(|arg| matcher.contains(arg)))
        })
    }

    fn validate_required_unless(&self, matcher: &ArgMatcher) -> ClapResult<()> {
        debug!("Validator::validate_required_unless");
        for a in self
            .p
            .app
            .args
            .args
            .iter()
            .filter(|a| !a.r_unless.is_empty())
            .filter(|a| !matcher.contains(&a.id))
        {
            debug!("Validator::validate_required_unless:iter:{}", a.name);
            if self.fails_arg_required_unless(a, matcher) {
                return self.missing_required_error(matcher, Some(&a.id));
            }
        }

        Ok(())
    }

    // Failing a required unless means, the arg's "unless" wasn't present, and neither were they
    fn fails_arg_required_unless(&self, a: &Arg<'b>, matcher: &ArgMatcher) -> bool {
        debug!("Validator::fails_arg_required_unless: a={:?}", a.name);
        if a.is_set(ArgSettings::RequiredUnlessAll) {
            debug!("Validator::fails_arg_required_unless:{}:All", a.name);
            !a.r_unless.iter().all(|id| matcher.contains(id))
        } else {
            debug!("Validator::fails_arg_required_unless:{}:Any", a.name);
            !a.r_unless.iter().any(|id| matcher.contains(id))
        }
    }

    // `incl`: an arg to include in the error even if not used
    fn missing_required_error(&self, matcher: &ArgMatcher, incl: Option<&Id>) -> ClapResult<()> {
        debug!("Validator::missing_required_error; incl={:?}", incl);
        debug!(
            "Validator::missing_required_error: reqs={:?}",
            self.p.required
        );

        let usg = Usage::new(self.p);

        let req_args = if let Some(x) = incl {
            usg.get_required_usage_from(&[x.clone()], Some(matcher), true)
        } else {
            usg.get_required_usage_from(&[], Some(matcher), true)
        };

        debug!(
            "Validator::missing_required_error: req_args={:#?}",
            req_args
        );

        let used: Vec<Id> = matcher
            .arg_names()
            .filter(|n| {
                self.p.app.find(n).map_or(true, |a| {
                    !(a.is_set(ArgSettings::Hidden) || self.p.required.contains(&a.id))
                })
            })
            .cloned()
            .chain(incl.cloned())
            .collect();

        Err(Error::missing_required_argument(
            req_args,
            &*usg.create_usage_with_title(&*used),
            self.p.app.color(),
        )?)
    }
}
