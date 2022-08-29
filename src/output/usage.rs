// Internal
use crate::builder::StyledStr;
use crate::builder::{Arg, ArgPredicate, Command};
use crate::parser::ArgMatcher;
use crate::util::ChildGraph;
use crate::util::FlatSet;
use crate::util::Id;
use crate::INTERNAL_ERROR_MSG;

static DEFAULT_SUB_VALUE_NAME: &str = "SUBCOMMAND";

pub(crate) struct Usage<'cmd> {
    cmd: &'cmd Command,
    required: Option<&'cmd ChildGraph<Id>>,
}

impl<'cmd> Usage<'cmd> {
    pub(crate) fn new(cmd: &'cmd Command) -> Self {
        Usage {
            cmd,
            required: None,
        }
    }

    pub(crate) fn required(mut self, required: &'cmd ChildGraph<Id>) -> Self {
        self.required = Some(required);
        self
    }

    // Creates a usage string for display. This happens just after all arguments were parsed, but before
    // any subcommands have been parsed (so as to give subcommands their own usage recursively)
    pub(crate) fn create_usage_with_title(&self, used: &[Id]) -> StyledStr {
        debug!("Usage::create_usage_with_title");
        let mut styled = StyledStr::new();
        styled.header("Usage:");
        styled.none("\n    ");
        styled.extend(self.create_usage_no_title(used).into_iter());
        styled
    }

    // Creates a usage string (*without title*) if one was not provided by the user manually.
    pub(crate) fn create_usage_no_title(&self, used: &[Id]) -> StyledStr {
        debug!("Usage::create_usage_no_title");
        if let Some(u) = self.cmd.get_override_usage() {
            u.clone()
        } else if used.is_empty() {
            self.create_help_usage(true)
        } else {
            self.create_smart_usage(used)
        }
    }

    // Creates a usage string for display in help messages (i.e. not for errors)
    fn create_help_usage(&self, incl_reqs: bool) -> StyledStr {
        debug!("Usage::create_help_usage; incl_reqs={:?}", incl_reqs);
        let mut styled = StyledStr::new();
        let name = self
            .cmd
            .get_usage_name()
            .or_else(|| self.cmd.get_bin_name())
            .unwrap_or_else(|| self.cmd.get_name());
        styled.literal(name);

        if self.needs_options_tag() {
            styled.placeholder(" [OPTIONS]");
        }

        let allow_missing_positional = self.cmd.is_allow_missing_positional_set();
        if !allow_missing_positional && incl_reqs {
            self.write_required_usage_from(&[], None, false, &mut styled);
        }

        let has_last = self.cmd.get_positionals().any(|p| p.is_last_set());
        // places a '--' in the usage string if there are args and options
        // supporting multiple values
        if self
            .cmd
            .get_non_positionals()
            .any(|o| o.is_multiple_values_set())
            && self.cmd.get_positionals().any(|p| !p.is_required_set())
            && !(self.cmd.has_visible_subcommands() || self.cmd.is_allow_external_subcommands_set())
            && !has_last
        {
            styled.placeholder(" [--]");
        }
        let not_req_or_hidden =
            |p: &Arg| (!p.is_required_set() || p.is_last_set()) && !p.is_hide_set();
        if self.cmd.get_positionals().any(not_req_or_hidden) {
            if let Some(args_tag) = self.get_optional_args(incl_reqs) {
                styled.placeholder(&*args_tag);
            } else {
                styled.placeholder(" [ARGS]");
            }
            if has_last && incl_reqs {
                let pos = self
                    .cmd
                    .get_positionals()
                    .find(|p| p.is_last_set())
                    .expect(INTERNAL_ERROR_MSG);
                debug!(
                    "Usage::create_help_usage: '{}' has .last(true)",
                    pos.get_id()
                );
                let req = pos.is_required_set();
                if req && self.cmd.get_positionals().any(|p| !p.is_required_set()) {
                    styled.literal(" -- ");
                    styled.placeholder("<");
                } else if req {
                    styled.placeholder(" [--] <");
                } else {
                    styled.placeholder(" [-- <");
                }
                styled.placeholder(&*pos.name_no_brackets());
                styled.placeholder('>');
                styled.placeholder(pos.multiple_str());
                if !req {
                    styled.placeholder(']');
                }
            }
        }

        if allow_missing_positional && incl_reqs {
            self.write_required_usage_from(&[], None, false, &mut styled);
        }

        // incl_reqs is only false when this function is called recursively
        if self.cmd.has_visible_subcommands() && incl_reqs
            || self.cmd.is_allow_external_subcommands_set()
        {
            let placeholder = self
                .cmd
                .get_subcommand_value_name()
                .unwrap_or(DEFAULT_SUB_VALUE_NAME);
            if self.cmd.is_subcommand_negates_reqs_set()
                || self.cmd.is_args_conflicts_with_subcommands_set()
            {
                styled.none("\n    ");
                if self.cmd.is_args_conflicts_with_subcommands_set() {
                    // Short-circuit full usage creation since no args will be relevant
                    styled.literal(name);
                } else {
                    styled.extend(self.create_help_usage(false).into_iter());
                }
                styled.placeholder(" <");
                styled.placeholder(placeholder);
                styled.placeholder(">");
            } else if self.cmd.is_subcommand_required_set() {
                styled.placeholder(" <");
                styled.placeholder(placeholder);
                styled.placeholder(">");
            } else {
                styled.placeholder(" [");
                styled.placeholder(placeholder);
                styled.placeholder("]");
            }
        }
        styled.trim();
        debug!("Usage::create_help_usage: usage={}", styled);
        styled
    }

    // Creates a context aware usage string, or "smart usage" from currently used
    // args, and requirements
    fn create_smart_usage(&self, used: &[Id]) -> StyledStr {
        debug!("Usage::create_smart_usage");
        let mut styled = StyledStr::new();

        styled.literal(
            self.cmd
                .get_usage_name()
                .or_else(|| self.cmd.get_bin_name())
                .unwrap_or_else(|| self.cmd.get_name()),
        );

        self.write_required_usage_from(used, None, true, &mut styled);

        if self.cmd.is_subcommand_required_set() {
            styled.placeholder(" <");
            styled.placeholder(
                self.cmd
                    .get_subcommand_value_name()
                    .unwrap_or(DEFAULT_SUB_VALUE_NAME),
            );
            styled.placeholder(">");
        }
        styled
    }

    // Gets the `[ARGS]` tag for the usage string
    fn get_optional_args(&self, incl_reqs: bool) -> Option<String> {
        debug!("Usage::get_optional_args; incl_reqs = {:?}", incl_reqs);
        let mut count = 0;
        for pos in self
            .cmd
            .get_positionals()
            .filter(|pos| !pos.is_required_set())
            .filter(|pos| !pos.is_hide_set())
            .filter(|pos| !pos.is_last_set())
        {
            debug!("Usage::get_optional_args:iter:{}", pos.get_id());
            let required = self.cmd.groups_for_arg(&pos.id).any(|grp_s| {
                debug!(
                    "Usage::get_optional_args:iter:{:?}:iter:{:?}",
                    pos.get_id(),
                    grp_s
                );
                // if it's part of a required group we don't want to count it
                self.cmd.get_groups().any(|g| g.required && (g.id == grp_s))
            });
            if !required {
                count += 1;
                debug!(
                    "Usage::get_optional_args:iter: {} Args not required or hidden",
                    count
                );
            }
        }

        if !self.cmd.is_dont_collapse_args_in_usage_set() && count > 1 {
            debug!("Usage::get_optional_args:iter: More than one, returning [ARGS]");

            // [ARGS]
            None
        } else if count == 1 && incl_reqs {
            let pos = self
                .cmd
                .get_positionals()
                .find(|pos| {
                    !pos.is_required_set()
                        && !pos.is_hide_set()
                        && !pos.is_last_set()
                        && !self.cmd.groups_for_arg(&pos.id).any(|grp_s| {
                            debug!(
                                "Usage::get_optional_args:iter:{:?}:iter:{:?}",
                                pos.get_id(),
                                grp_s
                            );
                            // if it's part of a required group we don't want to count it
                            self.cmd.get_groups().any(|g| g.required && (g.id == grp_s))
                        })
                })
                .expect(INTERNAL_ERROR_MSG);

            debug!(
                "Usage::get_optional_args:iter: Exactly one, returning '{}'",
                pos.get_id()
            );

            Some(format!(
                " [{}]{}",
                pos.name_no_brackets(),
                pos.multiple_str()
            ))
        } else if self.cmd.is_dont_collapse_args_in_usage_set()
            && self.cmd.has_positionals()
            && incl_reqs
        {
            debug!("Usage::get_optional_args:iter: Don't collapse returning all");
            Some(
                self.cmd
                    .get_positionals()
                    .filter(|pos| !pos.is_required_set())
                    .filter(|pos| !pos.is_hide_set())
                    .filter(|pos| !pos.is_last_set())
                    .map(|pos| format!(" [{}]{}", pos.name_no_brackets(), pos.multiple_str()))
                    .collect::<Vec<_>>()
                    .join(""),
            )
        } else if !incl_reqs {
            debug!(
                "Usage::get_optional_args:iter: incl_reqs=false, building secondary usage string"
            );
            let highest_req_pos = self
                .cmd
                .get_positionals()
                .filter_map(|pos| {
                    if pos.is_required_set() && !pos.is_last_set() {
                        Some(pos.index)
                    } else {
                        None
                    }
                })
                .max()
                .unwrap_or_else(|| Some(self.cmd.get_positionals().count()));
            Some(
                self.cmd
                    .get_positionals()
                    .filter(|pos| pos.index <= highest_req_pos)
                    .filter(|pos| !pos.is_required_set())
                    .filter(|pos| !pos.is_hide_set())
                    .filter(|pos| !pos.is_last_set())
                    .map(|pos| format!(" [{}]{}", pos.name_no_brackets(), pos.multiple_str()))
                    .collect::<Vec<_>>()
                    .join(""),
            )
        } else {
            Some("".into())
        }
    }

    // Determines if we need the `[OPTIONS]` tag in the usage string
    fn needs_options_tag(&self) -> bool {
        debug!("Usage::needs_options_tag");
        'outer: for f in self.cmd.get_non_positionals() {
            debug!("Usage::needs_options_tag:iter: f={}", f.get_id());

            // Don't print `[OPTIONS]` just for help or version
            if f.get_long() == Some("help") || f.get_long() == Some("version") {
                debug!("Usage::needs_options_tag:iter Option is built-in");
                continue;
            }

            if f.is_hide_set() {
                debug!("Usage::needs_options_tag:iter Option is hidden");
                continue;
            }
            if f.is_required_set() {
                debug!("Usage::needs_options_tag:iter Option is required");
                continue;
            }
            for grp_s in self.cmd.groups_for_arg(&f.id) {
                debug!("Usage::needs_options_tag:iter:iter: grp_s={:?}", grp_s);
                if self.cmd.get_groups().any(|g| g.id == grp_s && g.required) {
                    debug!("Usage::needs_options_tag:iter:iter: Group is required");
                    continue 'outer;
                }
            }

            debug!("Usage::needs_options_tag:iter: [OPTIONS] required");
            return true;
        }

        debug!("Usage::needs_options_tag: [OPTIONS] not required");
        false
    }

    // Returns the required args in usage string form by fully unrolling all groups
    // `incl_last`: should we include args that are Arg::Last? (i.e. `prog [foo] -- [last]). We
    // can't do that for required usages being built for subcommands because it would look like:
    // `prog [foo] -- [last] <subcommand>` which is totally wrong.
    pub(crate) fn write_required_usage_from(
        &self,
        incls: &[Id],
        matcher: Option<&ArgMatcher>,
        incl_last: bool,
        styled: &mut StyledStr,
    ) {
        for required in self.get_required_usage_from(incls, matcher, incl_last) {
            styled.none(" ");
            styled.extend(required.into_iter());
        }
    }

    pub(crate) fn get_required_usage_from(
        &self,
        incls: &[Id],
        matcher: Option<&ArgMatcher>,
        incl_last: bool,
    ) -> Vec<StyledStr> {
        debug!(
            "Usage::get_required_usage_from: incls={:?}, matcher={:?}, incl_last={:?}",
            incls,
            matcher.is_some(),
            incl_last
        );

        let required_owned;
        let required = if let Some(required) = self.required {
            required
        } else {
            required_owned = self.cmd.required_graph();
            &required_owned
        };

        let mut unrolled_reqs = FlatSet::new();
        for a in required.iter() {
            let is_relevant = |(val, req_arg): &(ArgPredicate, Id)| -> Option<Id> {
                let required = match val {
                    ArgPredicate::Equals(_) => {
                        if let Some(matcher) = matcher {
                            matcher.check_explicit(a, val)
                        } else {
                            false
                        }
                    }
                    ArgPredicate::IsPresent => true,
                };
                required.then(|| req_arg.clone())
            };

            for aa in self.cmd.unroll_arg_requires(is_relevant, a) {
                // if we don't check for duplicates here this causes duplicate error messages
                // see https://github.com/clap-rs/clap/issues/2770
                unrolled_reqs.insert(aa);
            }
            // always include the required arg itself. it will not be enumerated
            // by unroll_requirements_for_arg.
            unrolled_reqs.insert(a.clone());
        }
        debug!(
            "Usage::get_required_usage_from: unrolled_reqs={:?}",
            unrolled_reqs
        );

        let mut required_groups_members = FlatSet::new();
        let mut required_opts = FlatSet::new();
        let mut required_groups = FlatSet::new();
        let mut required_positionals = FlatSet::new();
        for req in unrolled_reqs.iter().chain(incls.iter()) {
            if let Some(arg) = self.cmd.find(req) {
                let is_present = matcher
                    .map(|m| m.check_explicit(req, &ArgPredicate::IsPresent))
                    .unwrap_or(false);
                debug!(
                    "Usage::get_required_usage_from:iter:{:?} arg is_present={}",
                    req, is_present
                );
                if !is_present {
                    if arg.is_positional() {
                        if incl_last || !arg.is_last_set() {
                            required_positionals.insert((arg.index.unwrap(), arg.stylized()));
                        }
                    } else {
                        required_opts.insert(arg.stylized());
                    }
                }
            } else {
                debug_assert!(self.cmd.find_group(req).is_some());
                let group_members = self.cmd.unroll_args_in_group(req);
                let is_present = matcher
                    .map(|m| {
                        group_members
                            .iter()
                            .any(|arg| m.check_explicit(arg, &ArgPredicate::IsPresent))
                    })
                    .unwrap_or(false);
                debug!(
                    "Usage::get_required_usage_from:iter:{:?} group is_present={}",
                    req, is_present
                );
                if !is_present {
                    let elem = self.cmd.format_group(req);
                    required_groups.insert(elem);
                    required_groups_members.extend(
                        group_members
                            .iter()
                            .flat_map(|id| self.cmd.find(id))
                            .map(|arg| arg.stylized()),
                    );
                }
            }
        }

        let mut ret_val = Vec::new();

        required_opts.retain(|arg| !required_groups_members.contains(arg));
        ret_val.extend(required_opts);

        ret_val.extend(required_groups);

        required_positionals.sort_by_key(|(ind, _)| *ind); // sort by index
        for (_, p) in required_positionals {
            if !required_groups_members.contains(&p) {
                ret_val.push(p);
            }
        }

        debug!("Usage::get_required_usage_from: ret_val={:?}", ret_val);
        ret_val
    }
}
