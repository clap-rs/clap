// Internal
use crate::builder::StyledStr;
use crate::builder::{Arg, ArgPredicate, Command, PossibleValue};
use crate::error::{Error, Result as ClapResult};
use crate::output::Usage;
use crate::parser::{ArgMatcher, ParseState};
use crate::util::ChildGraph;
use crate::util::FlatMap;
use crate::util::FlatSet;
use crate::util::Id;
use crate::INTERNAL_ERROR_MSG;

pub(crate) struct Validator<'cmd> {
    cmd: &'cmd Command,
    required: ChildGraph<Id>,
}

impl<'cmd> Validator<'cmd> {
    pub(crate) fn new(cmd: &'cmd Command) -> Self {
        let required = cmd.required_graph();
        Validator { cmd, required }
    }

    pub(crate) fn validate(
        &mut self,
        parse_state: ParseState,
        matcher: &mut ArgMatcher,
    ) -> ClapResult<()> {
        debug!("Validator::validate");
        let mut conflicts = Conflicts::new();
        let has_subcmd = matcher.subcommand_name().is_some();

        if let ParseState::Opt(a) = parse_state {
            debug!("Validator::validate: needs_val_of={:?}", a);

            let o = &self.cmd[&a];
            let should_err = if let Some(v) = matcher.args.get(&o.id) {
                v.all_val_groups_empty() && o.get_min_vals() != 0
            } else {
                true
            };
            if should_err {
                return Err(Error::empty_value(
                    self.cmd,
                    &get_possible_values_cli(o)
                        .iter()
                        .filter(|pv| !pv.is_hide_set())
                        .map(|n| n.get_name().to_owned())
                        .collect::<Vec<_>>(),
                    o.to_string(),
                ));
            }
        }

        if !has_subcmd && self.cmd.is_arg_required_else_help_set() {
            let num_user_values = matcher
                .arg_ids()
                .filter(|arg_id| matcher.check_explicit(arg_id, &ArgPredicate::IsPresent))
                .count();
            if num_user_values == 0 {
                let message = self.cmd.write_help_err(false);
                return Err(Error::display_help_error(self.cmd, message));
            }
        }
        if !has_subcmd && self.cmd.is_subcommand_required_set() {
            let bn = self
                .cmd
                .get_bin_name()
                .unwrap_or_else(|| self.cmd.get_name());
            return Err(Error::missing_subcommand(
                self.cmd,
                bn.to_string(),
                Usage::new(self.cmd)
                    .required(&self.required)
                    .create_usage_with_title(&[]),
            ));
        }

        self.validate_conflicts(matcher, &mut conflicts)?;
        if !(self.cmd.is_subcommand_negates_reqs_set() && has_subcmd) {
            self.validate_required(matcher, &mut conflicts)?;
        }

        Ok(())
    }

    fn validate_conflicts(
        &mut self,
        matcher: &ArgMatcher,
        conflicts: &mut Conflicts,
    ) -> ClapResult<()> {
        debug!("Validator::validate_conflicts");

        self.validate_exclusive(matcher)?;

        for arg_id in matcher
            .arg_ids()
            .filter(|arg_id| matcher.check_explicit(arg_id, &ArgPredicate::IsPresent))
            .filter(|arg_id| self.cmd.find(arg_id).is_some())
        {
            debug!("Validator::validate_conflicts::iter: id={:?}", arg_id);
            let conflicts = conflicts.gather_conflicts(self.cmd, matcher, arg_id);
            self.build_conflict_err(arg_id, &conflicts, matcher)?;
        }

        Ok(())
    }

    fn validate_exclusive(&self, matcher: &ArgMatcher) -> ClapResult<()> {
        debug!("Validator::validate_exclusive");
        let args_count = matcher
            .arg_ids()
            .filter(|arg_id| {
                matcher.check_explicit(arg_id, &crate::builder::ArgPredicate::IsPresent)
            })
            .count();
        if args_count <= 1 {
            // Nothing present to conflict with
            return Ok(());
        }

        matcher
            .arg_ids()
            .filter(|arg_id| {
                matcher.check_explicit(arg_id, &crate::builder::ArgPredicate::IsPresent)
            })
            .filter_map(|name| {
                debug!("Validator::validate_exclusive:iter:{:?}", name);
                self.cmd
                    .find(name)
                    // Find `arg`s which are exclusive but also appear with other args.
                    .filter(|&arg| arg.is_exclusive_set() && args_count > 1)
            })
            // Throw an error for the first conflict found.
            .try_for_each(|arg| {
                Err(Error::argument_conflict(
                    self.cmd,
                    arg.to_string(),
                    Vec::new(),
                    Usage::new(self.cmd)
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
        let mut seen = FlatSet::new();
        let conflicts = conflict_ids
            .iter()
            .flat_map(|c_id| {
                if self.cmd.find_group(c_id).is_some() {
                    self.cmd.unroll_args_in_group(c_id)
                } else {
                    vec![c_id.clone()]
                }
            })
            .filter_map(|c_id| {
                seen.insert(c_id.clone()).then(|| {
                    let c_arg = self.cmd.find(&c_id).expect(INTERNAL_ERROR_MSG);
                    c_arg.to_string()
                })
            })
            .collect();

        let former_arg = self.cmd.find(name).expect(INTERNAL_ERROR_MSG);
        let usg = self.build_conflict_err_usage(matcher, conflict_ids);
        Err(Error::argument_conflict(
            self.cmd,
            former_arg.to_string(),
            conflicts,
            usg,
        ))
    }

    fn build_conflict_err_usage(&self, matcher: &ArgMatcher, conflicting_keys: &[Id]) -> StyledStr {
        let used_filtered: Vec<Id> = matcher
            .arg_ids()
            .filter(|arg_id| matcher.check_explicit(arg_id, &ArgPredicate::IsPresent))
            .filter(|n| {
                // Filter out the args we don't want to specify.
                self.cmd.find(n).map_or(true, |a| !a.is_hide_set())
            })
            .filter(|key| !conflicting_keys.contains(key))
            .cloned()
            .collect();
        let required: Vec<Id> = used_filtered
            .iter()
            .filter_map(|key| self.cmd.find(key))
            .flat_map(|arg| arg.requires.iter().map(|item| &item.1))
            .filter(|key| !used_filtered.contains(key) && !conflicting_keys.contains(key))
            .chain(used_filtered.iter())
            .cloned()
            .collect();
        Usage::new(self.cmd)
            .required(&self.required)
            .create_usage_with_title(&required)
    }

    fn gather_requires(&mut self, matcher: &ArgMatcher) {
        debug!("Validator::gather_requires");
        for name in matcher
            .arg_ids()
            .filter(|arg_id| matcher.check_explicit(arg_id, &ArgPredicate::IsPresent))
        {
            debug!("Validator::gather_requires:iter:{:?}", name);
            if let Some(arg) = self.cmd.find(name) {
                let is_relevant = |(val, req_arg): &(ArgPredicate, Id)| -> Option<Id> {
                    let required = matcher.check_explicit(&arg.id, val);
                    required.then(|| req_arg.clone())
                };

                for req in self.cmd.unroll_arg_requires(is_relevant, &arg.id) {
                    self.required.insert(req);
                }
            } else if let Some(g) = self.cmd.find_group(name) {
                debug!("Validator::gather_requires:iter:{:?}:group", name);
                for r in &g.requires {
                    self.required.insert(r.clone());
                }
            }
        }
    }

    fn validate_required(
        &mut self,
        matcher: &ArgMatcher,
        conflicts: &mut Conflicts,
    ) -> ClapResult<()> {
        debug!("Validator::validate_required: required={:?}", self.required);
        self.gather_requires(matcher);

        let is_exclusive_present = matcher
            .arg_ids()
            .filter(|arg_id| matcher.check_explicit(arg_id, &ArgPredicate::IsPresent))
            .any(|id| {
                self.cmd
                    .find(id)
                    .map(|arg| arg.is_exclusive_set())
                    .unwrap_or_default()
            });
        debug!(
            "Validator::validate_required: is_exclusive_present={}",
            is_exclusive_present
        );

        for arg_or_group in self
            .required
            .iter()
            .filter(|r| !matcher.check_explicit(r, &ArgPredicate::IsPresent))
        {
            debug!("Validator::validate_required:iter:aog={:?}", arg_or_group);
            if let Some(arg) = self.cmd.find(arg_or_group) {
                debug!("Validator::validate_required:iter: This is an arg");
                if !is_exclusive_present && !self.is_missing_required_ok(arg, matcher, conflicts) {
                    return self.missing_required_error(matcher, vec![]);
                }
            } else if let Some(group) = self.cmd.find_group(arg_or_group) {
                debug!("Validator::validate_required:iter: This is a group");
                if !self
                    .cmd
                    .unroll_args_in_group(&group.id)
                    .iter()
                    .any(|a| matcher.check_explicit(a, &ArgPredicate::IsPresent))
                {
                    return self.missing_required_error(matcher, vec![]);
                }
            }
        }

        // Validate the conditionally required args
        for a in self.cmd.get_arguments() {
            if matcher.check_explicit(&a.id, &ArgPredicate::IsPresent) {
                continue;
            }

            for (other, val) in &a.r_ifs {
                if matcher.check_explicit(other, &ArgPredicate::Equals(val.into())) {
                    return self.missing_required_error(matcher, vec![a.id.clone()]);
                }
            }

            let match_all = a.r_ifs_all.iter().all(|(other, val)| {
                matcher.check_explicit(other, &ArgPredicate::Equals(val.into()))
            });
            if match_all && !a.r_ifs_all.is_empty() {
                return self.missing_required_error(matcher, vec![a.id.clone()]);
            }
        }

        debug!("Validator::validate_required_unless");
        let mut failed_args = Vec::new();
        for a in self.cmd.get_arguments() {
            if (!a.r_unless.is_empty() || !a.r_unless_all.is_empty())
                && !matcher.check_explicit(&a.id, &ArgPredicate::IsPresent)
                && self.fails_arg_required_unless(a, matcher)
            {
                failed_args.push(a.id.clone());
            }
        }
        if !failed_args.is_empty() {
            self.missing_required_error(matcher, failed_args)?;
        }

        Ok(())
    }

    fn is_missing_required_ok(
        &self,
        a: &Arg,
        matcher: &ArgMatcher,
        conflicts: &mut Conflicts,
    ) -> bool {
        debug!("Validator::is_missing_required_ok: {}", a.get_id());
        let conflicts = conflicts.gather_conflicts(self.cmd, matcher, &a.id);
        !conflicts.is_empty()
    }

    // Failing a required unless means, the arg's "unless" wasn't present, and neither were they
    fn fails_arg_required_unless(&self, a: &Arg, matcher: &ArgMatcher) -> bool {
        debug!("Validator::fails_arg_required_unless: a={:?}", a.get_id());
        let exists = |id| matcher.check_explicit(id, &ArgPredicate::IsPresent);

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

        let usg = Usage::new(self.cmd).required(&self.required);

        let req_args = usg
            .get_required_usage_from(&incl, Some(matcher), true)
            .into_iter()
            .map(|s| s.to_string())
            .collect::<Vec<_>>();

        debug!(
            "Validator::missing_required_error: req_args={:#?}",
            req_args
        );

        let used: Vec<Id> = matcher
            .arg_ids()
            .filter(|arg_id| matcher.check_explicit(arg_id, &ArgPredicate::IsPresent))
            .filter(|n| {
                // Filter out the args we don't want to specify.
                self.cmd.find(n).map_or(true, |a| !a.is_hide_set())
            })
            .cloned()
            .chain(incl)
            .collect();

        Err(Error::missing_required_argument(
            self.cmd,
            req_args,
            usg.create_usage_with_title(&used),
        ))
    }
}

#[derive(Default, Clone, Debug)]
struct Conflicts {
    potential: FlatMap<Id, Vec<Id>>,
}

impl Conflicts {
    fn new() -> Self {
        Self::default()
    }

    fn gather_conflicts(&mut self, cmd: &Command, matcher: &ArgMatcher, arg_id: &Id) -> Vec<Id> {
        debug!("Conflicts::gather_conflicts: arg={:?}", arg_id);
        let mut conflicts = Vec::new();
        for other_arg_id in matcher
            .arg_ids()
            .filter(|arg_id| matcher.check_explicit(arg_id, &ArgPredicate::IsPresent))
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
        debug!("Conflicts::gather_conflicts: conflicts={:?}", conflicts);
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

                // Overrides are implicitly conflicts
                conf.extend(arg.overrides.iter().cloned());

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

pub(crate) fn get_possible_values_cli(a: &Arg) -> Vec<PossibleValue> {
    if !a.is_takes_value_set() {
        vec![]
    } else {
        a.get_value_parser()
            .possible_values()
            .map(|pvs| pvs.collect())
            .unwrap_or_default()
    }
}
