// Internal
use crate::build::{AppSettings, Arg, ArgPredicate, Command, PossibleValue};
use crate::error::{Error, Result as ClapResult};
use crate::output::Usage;
use crate::parse::{ArgMatcher, MatchedArg, ParseState, Parser};
use crate::util::ChildGraph;
use crate::util::Id;
use crate::{INTERNAL_ERROR_MSG, INVALID_UTF8};

pub(crate) struct Validator<'help, 'cmd, 'parser> {
    p: &'parser mut Parser<'help, 'cmd>,
    required: ChildGraph<Id>,
}

impl<'help, 'cmd, 'parser> Validator<'help, 'cmd, 'parser> {
    pub(crate) fn new(p: &'parser mut Parser<'help, 'cmd>) -> Self {
        let required = p.cmd.required_graph();
        Validator { p, required }
    }

    pub(crate) fn validate(
        &mut self,
        parse_state: ParseState,
        matcher: &mut ArgMatcher,
        trailing_values: bool,
    ) -> ClapResult<()> {
        debug!("Validator::validate");
        let has_subcmd = matcher.subcommand_name().is_some();

        #[cfg(feature = "env")]
        self.p.add_env(matcher, trailing_values)?;

        self.p.add_defaults(matcher, trailing_values);

        if let ParseState::Opt(a) = parse_state {
            debug!("Validator::validate: needs_val_of={:?}", a);

            let o = &self.p.cmd[&a];
            let should_err = if let Some(v) = matcher.args.get(&o.id) {
                v.all_val_groups_empty() && !(o.min_vals.is_some() && o.min_vals.unwrap() == 0)
            } else {
                true
            };
            if should_err {
                return Err(Error::empty_value(
                    self.p.cmd,
                    &o.possible_vals
                        .iter()
                        .filter(|pv| !pv.is_hide_set())
                        .map(PossibleValue::get_name)
                        .collect::<Vec<_>>(),
                    o,
                    Usage::new(self.p.cmd)
                        .required(&self.required)
                        .create_usage_with_title(&[]),
                ));
            }
        }

        if !has_subcmd && self.p.cmd.is_arg_required_else_help_set() {
            let num_user_values = matcher
                .arg_names()
                .filter(|arg_id| matcher.check_explicit(arg_id, ArgPredicate::IsPresent))
                .count();
            if num_user_values == 0 {
                let message = self.p.write_help_err()?;
                return Err(Error::display_help_error(self.p.cmd, message));
            }
        }
        #[allow(deprecated)]
        if !has_subcmd && self.p.cmd.is_subcommand_required_set() {
            let bn = self
                .p
                .cmd
                .get_bin_name()
                .unwrap_or_else(|| self.p.cmd.get_name());
            return Err(Error::missing_subcommand(
                self.p.cmd,
                bn.to_string(),
                Usage::new(self.p.cmd)
                    .required(&self.required)
                    .create_usage_with_title(&[]),
            ));
        } else if !has_subcmd && self.p.cmd.is_set(AppSettings::SubcommandRequiredElseHelp) {
            debug!("Validator::new::get_matches_with: SubcommandRequiredElseHelp=true");
            let message = self.p.write_help_err()?;
            return Err(Error::display_help_error(self.p.cmd, message));
        }

        self.validate_conflicts(matcher)?;
        if !(self.p.cmd.is_subcommand_negates_reqs_set() && has_subcmd) {
            self.validate_required(matcher)?;
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
        for val in ma.vals_flatten() {
            if !arg.is_allow_invalid_utf8_set() && val.to_str().is_none() {
                debug!(
                    "Validator::validate_arg_values: invalid UTF-8 found in val {:?}",
                    val
                );
                return Err(Error::invalid_utf8(
                    self.p.cmd,
                    Usage::new(self.p.cmd)
                        .required(&self.required)
                        .create_usage_with_title(&[]),
                ));
            }
            if !arg.possible_vals.is_empty() {
                debug!(
                    "Validator::validate_arg_values: possible_vals={:?}",
                    arg.possible_vals
                );
                let val_str = val.to_string_lossy();
                let ok = arg
                    .possible_vals
                    .iter()
                    .any(|pv| pv.matches(&val_str, arg.is_ignore_case_set()));
                if !ok {
                    let used: Vec<Id> = matcher
                        .arg_names()
                        .filter(|arg_id| matcher.check_explicit(arg_id, ArgPredicate::IsPresent))
                        .filter(|&n| {
                            self.p.cmd.find(n).map_or(true, |a| {
                                !(a.is_hide_set() || self.required.contains(&a.id))
                            })
                        })
                        .cloned()
                        .collect();
                    return Err(Error::invalid_value(
                        self.p.cmd,
                        val_str.into_owned(),
                        &arg.possible_vals
                            .iter()
                            .filter(|pv| !pv.is_hide_set())
                            .map(PossibleValue::get_name)
                            .collect::<Vec<_>>(),
                        arg,
                        Usage::new(self.p.cmd)
                            .required(&self.required)
                            .create_usage_with_title(&used),
                    ));
                }
            }
            if arg.is_forbid_empty_values_set() && val.is_empty() && matcher.contains(&arg.id) {
                debug!("Validator::validate_arg_values: illegal empty val found");
                return Err(Error::empty_value(
                    self.p.cmd,
                    &arg.possible_vals
                        .iter()
                        .filter(|pv| !pv.is_hide_set())
                        .map(PossibleValue::get_name)
                        .collect::<Vec<_>>(),
                    arg,
                    Usage::new(self.p.cmd)
                        .required(&self.required)
                        .create_usage_with_title(&[]),
                ));
            }

            if let Some(ref vtor) = arg.validator {
                debug!("Validator::validate_arg_values: checking validator...");
                let mut vtor = vtor.lock().unwrap();
                if let Err(e) = vtor(&*val.to_string_lossy()) {
                    debug!("error");
                    return Err(Error::value_validation(
                        arg.to_string(),
                        val.to_string_lossy().into_owned(),
                        e,
                    )
                    .with_cmd(self.p.cmd));
                } else {
                    debug!("good");
                }
            }
            if let Some(ref vtor) = arg.validator_os {
                debug!("Validator::validate_arg_values: checking validator_os...");
                let mut vtor = vtor.lock().unwrap();
                if let Err(e) = vtor(val) {
                    debug!("error");
                    return Err(Error::value_validation(
                        arg.to_string(),
                        val.to_string_lossy().into(),
                        e,
                    )
                    .with_cmd(self.p.cmd));
                } else {
                    debug!("good");
                }
            }
        }
        Ok(())
    }

    fn validate_conflicts(&self, matcher: &ArgMatcher) -> ClapResult<()> {
        debug!("Validator::validate_conflicts");

        self.validate_exclusive(matcher)?;

        let mut conflicts = Conflicts::new();
        for arg_id in matcher
            .arg_names()
            .filter(|arg_id| matcher.check_explicit(arg_id, ArgPredicate::IsPresent))
            .filter(|arg_id| self.p.cmd.find(arg_id).is_some())
        {
            debug!("Validator::validate_conflicts::iter: id={:?}", arg_id);
            let conflicts = conflicts.gather_conflicts(self.p.cmd, matcher, arg_id);
            self.build_conflict_err(arg_id, &conflicts, matcher)?;
        }

        Ok(())
    }

    fn validate_exclusive(&self, matcher: &ArgMatcher) -> ClapResult<()> {
        debug!("Validator::validate_exclusive");
        // Not bothering to filter for `check_explicit` since defaults shouldn't play into this
        let args_count = matcher.arg_names().count();
        matcher
            .arg_names()
            .filter_map(|name| {
                debug!("Validator::validate_exclusive:iter:{:?}", name);
                self.p
                    .cmd
                    .find(name)
                    // Find `arg`s which are exclusive but also appear with other args.
                    .filter(|&arg| arg.is_exclusive_set() && args_count > 1)
            })
            // Throw an error for the first conflict found.
            .try_for_each(|arg| {
                Err(Error::argument_conflict(
                    self.p.cmd,
                    arg,
                    Vec::new(),
                    Usage::new(self.p.cmd)
                        .required(&self.required)
                        .create_usage_with_title(&[]),
                ))
            })
    }

    fn build_conflict_err(
        &self,
        name: &Id,
        conflict_ids: &[Id],
        matcher: &ArgMatcher,
    ) -> ClapResult<()> {
        if conflict_ids.is_empty() {
            return Ok(());
        }

        debug!("Validator::build_conflict_err: name={:?}", name);
        let mut seen = std::collections::HashSet::new();
        let conflicts = conflict_ids
            .iter()
            .flat_map(|c_id| {
                if self.p.cmd.find_group(c_id).is_some() {
                    self.p.cmd.unroll_args_in_group(c_id)
                } else {
                    vec![c_id.clone()]
                }
            })
            .filter_map(|c_id| {
                seen.insert(c_id.clone()).then(|| {
                    let c_arg = self.p.cmd.find(&c_id).expect(INTERNAL_ERROR_MSG);
                    c_arg.to_string()
                })
            })
            .collect();

        let former_arg = self.p.cmd.find(name).expect(INTERNAL_ERROR_MSG);
        let usg = self.build_conflict_err_usage(matcher, conflict_ids);
        Err(Error::argument_conflict(
            self.p.cmd, former_arg, conflicts, usg,
        ))
    }

    fn build_conflict_err_usage(&self, matcher: &ArgMatcher, conflicting_keys: &[Id]) -> String {
        let used_filtered: Vec<Id> = matcher
            .arg_names()
            .filter(|arg_id| matcher.check_explicit(arg_id, ArgPredicate::IsPresent))
            .filter(|key| !conflicting_keys.contains(key))
            .cloned()
            .collect();
        let required: Vec<Id> = used_filtered
            .iter()
            .filter_map(|key| self.p.cmd.find(key))
            .flat_map(|arg| arg.requires.iter().map(|item| &item.1))
            .filter(|key| !used_filtered.contains(key) && !conflicting_keys.contains(key))
            .chain(used_filtered.iter())
            .cloned()
            .collect();
        Usage::new(self.p.cmd)
            .required(&self.required)
            .create_usage_with_title(&required)
    }

    fn gather_requires(&mut self, matcher: &ArgMatcher) {
        debug!("Validator::gather_requires");
        for name in matcher
            .arg_names()
            .filter(|arg_id| matcher.check_explicit(arg_id, ArgPredicate::IsPresent))
        {
            debug!("Validator::gather_requires:iter:{:?}", name);
            if let Some(arg) = self.p.cmd.find(name) {
                let is_relevant = |(val, req_arg): &(ArgPredicate<'_>, Id)| -> Option<Id> {
                    let required = matcher.check_explicit(&arg.id, *val);
                    required.then(|| req_arg.clone())
                };

                for req in self.p.cmd.unroll_arg_requires(is_relevant, &arg.id) {
                    self.required.insert(req);
                }
            } else if let Some(g) = self.p.cmd.find_group(name) {
                debug!("Validator::gather_requires:iter:{:?}:group", name);
                for r in &g.requires {
                    self.required.insert(r.clone());
                }
            }
        }
    }

    fn validate_matched_args(&self, matcher: &ArgMatcher) -> ClapResult<()> {
        debug!("Validator::validate_matched_args");
        matcher.iter().try_for_each(|(name, ma)| {
            debug!(
                "Validator::validate_matched_args:iter:{:?}: vals={:#?}",
                name,
                ma.vals_flatten()
            );
            if let Some(arg) = self.p.cmd.find(name) {
                self.validate_arg_num_vals(arg, ma)?;
                self.validate_arg_values(arg, ma, matcher)?;
                self.validate_arg_num_occurs(arg, ma)?;
            }
            Ok(())
        })
    }

    fn validate_arg_num_occurs(&self, a: &Arg, ma: &MatchedArg) -> ClapResult<()> {
        debug!(
            "Validator::validate_arg_num_occurs: {:?}={}",
            a.name,
            ma.get_occurrences()
        );
        // Occurrence of positional argument equals to number of values rather
        // than number of grouped values.
        if ma.get_occurrences() > 1 && !a.is_multiple_occurrences_set() && !a.is_positional() {
            // Not the first time, and we don't allow multiples
            return Err(Error::unexpected_multiple_usage(
                self.p.cmd,
                a,
                Usage::new(self.p.cmd)
                    .required(&self.required)
                    .create_usage_with_title(&[]),
            ));
        }
        if let Some(max_occurs) = a.max_occurs {
            debug!(
                "Validator::validate_arg_num_occurs: max_occurs set...{}",
                max_occurs
            );
            let occurs = ma.get_occurrences() as usize;
            if occurs > max_occurs {
                return Err(Error::too_many_occurrences(
                    self.p.cmd,
                    a,
                    max_occurs,
                    occurs,
                    Usage::new(self.p.cmd)
                        .required(&self.required)
                        .create_usage_with_title(&[]),
                ));
            }
        }

        Ok(())
    }

    fn validate_arg_num_vals(&self, a: &Arg, ma: &MatchedArg) -> ClapResult<()> {
        debug!("Validator::validate_arg_num_vals");
        if let Some(num) = a.num_vals {
            let total_num = ma.num_vals();
            debug!("Validator::validate_arg_num_vals: num_vals set...{}", num);
            let should_err = if a.is_multiple_occurrences_set() {
                total_num % num != 0
            } else {
                num != total_num
            };
            if should_err {
                debug!("Validator::validate_arg_num_vals: Sending error WrongNumberOfValues");
                return Err(Error::wrong_number_of_values(
                    self.p.cmd,
                    a,
                    num,
                    if a.is_multiple_occurrences_set() {
                        total_num % num
                    } else {
                        total_num
                    },
                    Usage::new(self.p.cmd)
                        .required(&self.required)
                        .create_usage_with_title(&[]),
                ));
            }
        }
        if let Some(num) = a.max_vals {
            debug!("Validator::validate_arg_num_vals: max_vals set...{}", num);
            if ma.num_vals() > num {
                debug!("Validator::validate_arg_num_vals: Sending error TooManyValues");
                return Err(Error::too_many_values(
                    self.p.cmd,
                    ma.vals_flatten()
                        .last()
                        .expect(INTERNAL_ERROR_MSG)
                        .to_str()
                        .expect(INVALID_UTF8)
                        .to_string(),
                    a.to_string(),
                    Usage::new(self.p.cmd)
                        .required(&self.required)
                        .create_usage_with_title(&[]),
                ));
            }
        }
        let min_vals_zero = if let Some(num) = a.min_vals {
            debug!("Validator::validate_arg_num_vals: min_vals set: {}", num);
            if ma.num_vals() < num && num != 0 {
                debug!("Validator::validate_arg_num_vals: Sending error TooFewValues");
                return Err(Error::too_few_values(
                    self.p.cmd,
                    a,
                    num,
                    ma.num_vals(),
                    Usage::new(self.p.cmd)
                        .required(&self.required)
                        .create_usage_with_title(&[]),
                ));
            }
            num == 0
        } else {
            false
        };
        // Issue 665 (https://github.com/clap-rs/clap/issues/665)
        // Issue 1105 (https://github.com/clap-rs/clap/issues/1105)
        if a.is_takes_value_set() && !min_vals_zero && ma.all_val_groups_empty() {
            return Err(Error::empty_value(
                self.p.cmd,
                &a.possible_vals
                    .iter()
                    .filter(|pv| !pv.is_hide_set())
                    .map(PossibleValue::get_name)
                    .collect::<Vec<_>>(),
                a,
                Usage::new(self.p.cmd)
                    .required(&self.required)
                    .create_usage_with_title(&[]),
            ));
        }
        Ok(())
    }

    fn validate_required(&mut self, matcher: &ArgMatcher) -> ClapResult<()> {
        debug!("Validator::validate_required: required={:?}", self.required);
        self.gather_requires(matcher);

        for arg_or_group in self.required.iter().filter(|r| !matcher.contains(r)) {
            debug!("Validator::validate_required:iter:aog={:?}", arg_or_group);
            if let Some(arg) = self.p.cmd.find(arg_or_group) {
                debug!("Validator::validate_required:iter: This is an arg");
                if !self.is_missing_required_ok(arg, matcher) {
                    return self.missing_required_error(matcher, vec![]);
                }
            } else if let Some(group) = self.p.cmd.find_group(arg_or_group) {
                debug!("Validator::validate_required:iter: This is a group");
                if !self
                    .p
                    .cmd
                    .unroll_args_in_group(&group.id)
                    .iter()
                    .any(|a| matcher.contains(a))
                {
                    return self.missing_required_error(matcher, vec![]);
                }
            }
        }

        // Validate the conditionally required args
        for a in self.p.cmd.get_arguments() {
            for (other, val) in &a.r_ifs {
                if matcher.check_explicit(other, ArgPredicate::Equals(std::ffi::OsStr::new(*val)))
                    && !matcher.contains(&a.id)
                {
                    return self.missing_required_error(matcher, vec![a.id.clone()]);
                }
            }

            let match_all = a.r_ifs_all.iter().all(|(other, val)| {
                matcher.check_explicit(other, ArgPredicate::Equals(std::ffi::OsStr::new(*val)))
            });
            if match_all && !a.r_ifs_all.is_empty() && !matcher.contains(&a.id) {
                return self.missing_required_error(matcher, vec![a.id.clone()]);
            }
        }

        self.validate_required_unless(matcher)?;

        Ok(())
    }

    fn is_missing_required_ok(&self, a: &Arg<'help>, matcher: &ArgMatcher) -> bool {
        debug!("Validator::is_missing_required_ok: {}", a.name);
        self.validate_arg_conflicts(a, matcher) || self.p.overridden.borrow().contains(&a.id)
    }

    fn validate_arg_conflicts(&self, a: &Arg<'help>, matcher: &ArgMatcher) -> bool {
        debug!("Validator::validate_arg_conflicts: a={:?}", a.name);
        a.blacklist.iter().any(|conf| {
            matcher.contains(conf)
                || self
                    .p
                    .cmd
                    .find_group(conf)
                    .map_or(false, |g| g.args.iter().any(|arg| matcher.contains(arg)))
        })
    }

    fn validate_required_unless(&self, matcher: &ArgMatcher) -> ClapResult<()> {
        debug!("Validator::validate_required_unless");
        let failed_args: Vec<_> = self
            .p
            .cmd
            .get_arguments()
            .filter(|&a| {
                (!a.r_unless.is_empty() || !a.r_unless_all.is_empty())
                    && !matcher.contains(&a.id)
                    && self.fails_arg_required_unless(a, matcher)
            })
            .map(|a| a.id.clone())
            .collect();
        if failed_args.is_empty() {
            Ok(())
        } else {
            self.missing_required_error(matcher, failed_args)
        }
    }

    // Failing a required unless means, the arg's "unless" wasn't present, and neither were they
    fn fails_arg_required_unless(&self, a: &Arg<'help>, matcher: &ArgMatcher) -> bool {
        debug!("Validator::fails_arg_required_unless: a={:?}", a.name);
        let exists = |id| matcher.check_explicit(id, ArgPredicate::IsPresent);

        (a.r_unless_all.is_empty() || !a.r_unless_all.iter().all(exists))
            && !a.r_unless.iter().any(exists)
    }

    // `incl`: an arg to include in the error even if not used
    fn missing_required_error(&self, matcher: &ArgMatcher, incl: Vec<Id>) -> ClapResult<()> {
        debug!("Validator::missing_required_error; incl={:?}", incl);
        debug!(
            "Validator::missing_required_error: reqs={:?}",
            self.required
        );

        let usg = Usage::new(self.p.cmd).required(&self.required);

        let req_args = usg.get_required_usage_from(&incl, Some(matcher), true);

        debug!(
            "Validator::missing_required_error: req_args={:#?}",
            req_args
        );

        let used: Vec<Id> = matcher
            .arg_names()
            .filter(|arg_id| matcher.check_explicit(arg_id, ArgPredicate::IsPresent))
            .filter(|n| {
                // Filter out the args we don't want to specify.
                self.p
                    .cmd
                    .find(n)
                    .map_or(true, |a| !a.is_hide_set() && !self.required.contains(&a.id))
            })
            .cloned()
            .chain(incl)
            .collect();

        Err(Error::missing_required_argument(
            self.p.cmd,
            req_args,
            usg.create_usage_with_title(&used),
        ))
    }
}

#[derive(Default, Clone, Debug)]
struct Conflicts {
    potential: std::collections::HashMap<Id, Vec<Id>>,
}

impl Conflicts {
    fn new() -> Self {
        Self::default()
    }

    fn gather_conflicts(&mut self, cmd: &Command, matcher: &ArgMatcher, arg_id: &Id) -> Vec<Id> {
        debug!("Conflicts::gather_conflicts");
        let mut conflicts = Vec::new();
        for other_arg_id in matcher
            .arg_names()
            .filter(|arg_id| matcher.check_explicit(arg_id, ArgPredicate::IsPresent))
        {
            if arg_id == other_arg_id {
                continue;
            }

            if self
                .gather_direct_conflicts(cmd, arg_id)
                .contains(other_arg_id)
            {
                conflicts.push(other_arg_id.clone());
            }
            if self
                .gather_direct_conflicts(cmd, other_arg_id)
                .contains(arg_id)
            {
                conflicts.push(other_arg_id.clone());
            }
        }
        conflicts
    }

    fn gather_direct_conflicts(&mut self, cmd: &Command, arg_id: &Id) -> &[Id] {
        self.potential.entry(arg_id.clone()).or_insert_with(|| {
            let conf = if let Some(arg) = cmd.find(arg_id) {
                let mut conf = arg.blacklist.clone();
                for group_id in cmd.groups_for_arg(arg_id) {
                    let group = cmd.find_group(&group_id).expect(INTERNAL_ERROR_MSG);
                    conf.extend(group.conflicts.iter().cloned());
                    if !group.multiple {
                        for member_id in &group.args {
                            if member_id != arg_id {
                                conf.push(member_id.clone());
                            }
                        }
                    }
                }
                conf
            } else if let Some(group) = cmd.find_group(arg_id) {
                group.conflicts.clone()
            } else {
                debug_assert!(false, "id={:?} is unknown", arg_id);
                Vec::new()
            };
            debug!(
                "Conflicts::gather_direct_conflicts id={:?}, conflicts={:?}",
                arg_id, conf
            );
            conf
        })
    }
}
