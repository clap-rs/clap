use indexmap::IndexSet;

// Internal
use crate::{
    build::AppSettings as AS,
    build::{App, Arg, ArgPredicate},
    parse::ArgMatcher,
    util::{ChildGraph, Id},
    INTERNAL_ERROR_MSG,
};

pub(crate) struct Usage<'help, 'app> {
    app: &'app App<'help>,
    required: &'app ChildGraph<Id>,
}

impl<'help, 'app> Usage<'help, 'app> {
    pub(crate) fn new(app: &'app App<'help>, required: &'app ChildGraph<Id>) -> Self {
        Usage { app, required }
    }

    // Creates a usage string for display. This happens just after all arguments were parsed, but before
    // any subcommands have been parsed (so as to give subcommands their own usage recursively)
    pub(crate) fn create_usage_with_title(&self, used: &[Id]) -> String {
        debug!("Usage::create_usage_with_title");
        let mut usage = String::with_capacity(75);
        usage.push_str("USAGE:\n    ");
        usage.push_str(&*self.create_usage_no_title(used));
        usage
    }

    // Creates a usage string (*without title*) if one was not provided by the user manually.
    pub(crate) fn create_usage_no_title(&self, used: &[Id]) -> String {
        debug!("Usage::create_usage_no_title");
        if let Some(u) = self.app.usage_str {
            String::from(&*u)
        } else if used.is_empty() {
            self.create_help_usage(true)
        } else {
            self.create_smart_usage(used)
        }
    }

    // Creates a usage string for display in help messages (i.e. not for errors)
    pub(crate) fn create_help_usage(&self, incl_reqs: bool) -> String {
        debug!("Usage::create_help_usage; incl_reqs={:?}", incl_reqs);
        let mut usage = String::with_capacity(75);
        let name = self
            .app
            .usage
            .as_ref()
            .or_else(|| self.app.bin_name.as_ref())
            .unwrap_or(&self.app.name);
        usage.push_str(&*name);
        let req_string = if incl_reqs {
            self.get_required_usage_from(&[], None, false)
                .iter()
                .fold(String::new(), |a, s| a + " " + s)
        } else {
            String::new()
        };

        if self.needs_options_tag() {
            usage.push_str(" [OPTIONS]");
        }

        let allow_missing_positional = self.app.is_allow_missing_positional_set();
        if !allow_missing_positional {
            usage.push_str(&req_string);
        }

        let has_last = self.app.get_positionals().any(|p| p.is_last_set());
        // places a '--' in the usage string if there are args and options
        // supporting multiple values
        if self
            .app
            .get_non_positionals()
            .any(|o| o.is_multiple_values_set())
            && self.app.get_positionals().any(|p| !p.is_required_set())
            && !(self.app.has_visible_subcommands() || self.app.is_allow_external_subcommands_set())
            && !has_last
        {
            usage.push_str(" [--]");
        }
        let not_req_or_hidden =
            |p: &Arg| (!p.is_required_set() || p.is_last_set()) && !p.is_hide_set();
        if self.app.get_positionals().any(not_req_or_hidden) {
            if let Some(args_tag) = self.get_args_tag(incl_reqs) {
                usage.push_str(&*args_tag);
            } else {
                usage.push_str(" [ARGS]");
            }
            if has_last && incl_reqs {
                let pos = self
                    .app
                    .get_positionals()
                    .find(|p| p.is_last_set())
                    .expect(INTERNAL_ERROR_MSG);
                debug!("Usage::create_help_usage: '{}' has .last(true)", pos.name);
                let req = pos.is_required_set();
                if req && self.app.get_positionals().any(|p| !p.is_required_set()) {
                    usage.push_str(" -- <");
                } else if req {
                    usage.push_str(" [--] <");
                } else {
                    usage.push_str(" [-- <");
                }
                usage.push_str(&*pos.name_no_brackets());
                usage.push('>');
                usage.push_str(pos.multiple_str());
                if !req {
                    usage.push(']');
                }
            }
        }

        if allow_missing_positional {
            usage.push_str(&req_string);
        }

        // incl_reqs is only false when this function is called recursively
        if self.app.has_visible_subcommands() && incl_reqs
            || self.app.is_allow_external_subcommands_set()
        {
            let placeholder = self.app.subcommand_value_name.unwrap_or("SUBCOMMAND");
            #[allow(deprecated)]
            if self.app.is_subcommand_negates_reqs_set()
                || self.app.is_args_conflicts_with_subcommands_set()
            {
                usage.push_str("\n    ");
                if !self.app.is_args_conflicts_with_subcommands_set() {
                    usage.push_str(&*self.create_help_usage(false));
                } else {
                    usage.push_str(&*name);
                }
                usage.push_str(" <");
                usage.push_str(placeholder);
                usage.push('>');
            } else if self.app.is_subcommand_required_set()
                || self.app.is_set(AS::SubcommandRequiredElseHelp)
            {
                usage.push_str(" <");
                usage.push_str(placeholder);
                usage.push('>');
            } else {
                usage.push_str(" [");
                usage.push_str(placeholder);
                usage.push(']');
            }
        }
        usage.shrink_to_fit();
        debug!("Usage::create_help_usage: usage={}", usage);
        usage
    }

    // Creates a context aware usage string, or "smart usage" from currently used
    // args, and requirements
    fn create_smart_usage(&self, used: &[Id]) -> String {
        debug!("Usage::create_smart_usage");
        let mut usage = String::with_capacity(75);

        let r_string = self
            .get_required_usage_from(used, None, true)
            .iter()
            .fold(String::new(), |acc, s| acc + " " + s);

        usage.push_str(
            &self
                .app
                .usage
                .as_ref()
                .or_else(|| self.app.bin_name.as_ref())
                .unwrap_or(&self.app.name)[..],
        );
        usage.push_str(&*r_string);
        if self.app.is_subcommand_required_set() {
            usage.push_str(" <");
            usage.push_str(self.app.subcommand_value_name.unwrap_or("SUBCOMMAND"));
            usage.push('>');
        }
        usage.shrink_to_fit();
        usage
    }

    // Gets the `[ARGS]` tag for the usage string
    fn get_args_tag(&self, incl_reqs: bool) -> Option<String> {
        debug!("Usage::get_args_tag; incl_reqs = {:?}", incl_reqs);
        let mut count = 0;
        for pos in self
            .app
            .get_positionals()
            .filter(|pos| !pos.is_required_set())
            .filter(|pos| !pos.is_hide_set())
            .filter(|pos| !pos.is_last_set())
        {
            debug!("Usage::get_args_tag:iter:{}", pos.name);
            let required = self.app.groups_for_arg(&pos.id).any(|grp_s| {
                debug!("Usage::get_args_tag:iter:{:?}:iter:{:?}", pos.name, grp_s);
                // if it's part of a required group we don't want to count it
                self.app
                    .groups
                    .iter()
                    .any(|g| g.required && (g.id == grp_s))
            });
            if !required {
                count += 1;
                debug!(
                    "Usage::get_args_tag:iter: {} Args not required or hidden",
                    count
                );
            }
        }

        if !self.app.is_dont_collapse_args_in_usage_set() && count > 1 {
            debug!("Usage::get_args_tag:iter: More than one, returning [ARGS]");

            // [ARGS]
            None
        } else if count == 1 && incl_reqs {
            let pos = self
                .app
                .get_positionals()
                .find(|pos| {
                    !pos.is_required_set()
                        && !pos.is_hide_set()
                        && !pos.is_last_set()
                        && !self.app.groups_for_arg(&pos.id).any(|grp_s| {
                            debug!("Usage::get_args_tag:iter:{:?}:iter:{:?}", pos.name, grp_s);
                            // if it's part of a required group we don't want to count it
                            self.app
                                .groups
                                .iter()
                                .any(|g| g.required && (g.id == grp_s))
                        })
                })
                .expect(INTERNAL_ERROR_MSG);

            debug!(
                "Usage::get_args_tag:iter: Exactly one, returning '{}'",
                pos.name
            );

            Some(format!(
                " [{}]{}",
                pos.name_no_brackets(),
                pos.multiple_str()
            ))
        } else if self.app.is_dont_collapse_args_in_usage_set()
            && self.app.has_positionals()
            && incl_reqs
        {
            debug!("Usage::get_args_tag:iter: Don't collapse returning all");
            Some(
                self.app
                    .get_positionals()
                    .filter(|pos| !pos.is_required_set())
                    .filter(|pos| !pos.is_hide_set())
                    .filter(|pos| !pos.is_last_set())
                    .map(|pos| format!(" [{}]{}", pos.name_no_brackets(), pos.multiple_str()))
                    .collect::<Vec<_>>()
                    .join(""),
            )
        } else if !incl_reqs {
            debug!("Usage::get_args_tag:iter: incl_reqs=false, building secondary usage string");
            let highest_req_pos = self
                .app
                .get_positionals()
                .filter_map(|pos| {
                    if pos.is_required_set() && !pos.is_last_set() {
                        Some(pos.index)
                    } else {
                        None
                    }
                })
                .max()
                .unwrap_or_else(|| Some(self.app.get_positionals().count()));
            Some(
                self.app
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
        'outer: for f in self.app.get_non_positionals() {
            debug!("Usage::needs_options_tag:iter: f={}", f.name);

            // Don't print `[OPTIONS]` just for help or version
            if f.long == Some("help") || f.long == Some("version") {
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
            for grp_s in self.app.groups_for_arg(&f.id) {
                debug!("Usage::needs_options_tag:iter:iter: grp_s={:?}", grp_s);
                if self.app.groups.iter().any(|g| g.id == grp_s && g.required) {
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
    pub(crate) fn get_required_usage_from(
        &self,
        incls: &[Id],
        matcher: Option<&ArgMatcher>,
        incl_last: bool,
    ) -> Vec<String> {
        debug!(
            "Usage::get_required_usage_from: incls={:?}, matcher={:?}, incl_last={:?}",
            incls,
            matcher.is_some(),
            incl_last
        );
        let mut ret_val = Vec::new();

        let mut unrolled_reqs = IndexSet::new();

        for a in self.required.iter() {
            let is_relevant = |(val, req_arg): &(ArgPredicate<'_>, Id)| -> Option<Id> {
                let required = match val {
                    ArgPredicate::Equals(_) => {
                        if let Some(matcher) = matcher {
                            matcher.check_explicit(a, *val)
                        } else {
                            false
                        }
                    }
                    ArgPredicate::IsPresent => true,
                };
                required.then(|| req_arg.clone())
            };

            for aa in self.app.unroll_arg_requires(is_relevant, a) {
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

        let args_in_groups = self
            .app
            .groups
            .iter()
            .filter(|gn| self.required.contains(&gn.id))
            .flat_map(|g| self.app.unroll_args_in_group(&g.id))
            .collect::<Vec<_>>();

        for a in unrolled_reqs
            .iter()
            .chain(incls.iter())
            .filter(|name| !self.app.get_positionals().any(|p| &&p.id == name))
            .filter(|name| !self.app.groups.iter().any(|g| &&g.id == name))
            .filter(|name| !args_in_groups.contains(name))
            .filter(|name| !(matcher.is_some() && matcher.as_ref().unwrap().contains(name)))
        {
            debug!("Usage::get_required_usage_from:iter:{:?}", a);
            let arg = self.app.find(a).expect(INTERNAL_ERROR_MSG).to_string();
            ret_val.push(arg);
        }
        let mut g_vec: Vec<String> = vec![];
        for g in unrolled_reqs
            .iter()
            .filter(|n| self.app.groups.iter().any(|g| g.id == **n))
        {
            // don't print requirement for required groups that have an arg.
            if let Some(m) = matcher {
                let have_group_entry = self
                    .app
                    .unroll_args_in_group(g)
                    .iter()
                    .any(|arg| m.contains(arg));
                if have_group_entry {
                    continue;
                }
            }

            let elem = self.app.format_group(g);
            if !g_vec.contains(&elem) {
                g_vec.push(elem);
            }
        }
        ret_val.extend_from_slice(&g_vec);

        let mut pvec = unrolled_reqs
            .iter()
            .chain(incls.iter())
            .filter(|a| self.app.get_positionals().any(|p| &&p.id == a))
            .filter(|&pos| matcher.map_or(true, |m| !m.contains(pos)))
            .filter_map(|pos| self.app.find(pos))
            .filter(|&pos| incl_last || !pos.is_last_set())
            .filter(|pos| !args_in_groups.contains(&pos.id))
            .map(|pos| (pos.index.unwrap(), pos))
            .collect::<Vec<(usize, &Arg)>>();
        pvec.sort_by_key(|(ind, _)| *ind); // sort by index

        for (_, p) in pvec {
            debug!("Usage::get_required_usage_from:iter:{:?}", p.id);
            if !args_in_groups.contains(&p.id) {
                ret_val.push(p.to_string());
            }
        }

        debug!("Usage::get_required_usage_from: ret_val={:?}", ret_val);
        ret_val
    }
}
