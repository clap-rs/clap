// std
use std::collections::{BTreeMap, VecDeque};

// Internal
use crate::{
    build::AppSettings as AS,
    build::{Arg, ArgSettings},
    parse::{ArgMatcher, Parser},
    util::Id,
    INTERNAL_ERROR_MSG,
};

pub(crate) struct Usage<'help, 'app, 'parser> {
    p: &'parser Parser<'help, 'app>,
}

impl<'help, 'app, 'parser> Usage<'help, 'app, 'parser> {
    pub(crate) fn new(p: &'parser Parser<'help, 'app>) -> Self {
        Usage { p }
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
        if let Some(u) = self.p.app.usage_str {
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
            .p
            .app
            .usage
            .as_ref()
            .unwrap_or_else(|| self.p.app.bin_name.as_ref().unwrap_or(&self.p.app.name));
        usage.push_str(&*name);
        let req_string = if incl_reqs {
            self.get_required_usage_from(&[], None, false)
                .iter()
                .fold(String::new(), |a, s| a + &format!(" {}", s)[..])
        } else {
            String::new()
        };

        let flags = self.needs_flags_tag();
        if flags && !self.p.is_set(AS::UnifiedHelpMessage) {
            usage.push_str(" [FLAGS]");
        } else if flags {
            usage.push_str(" [OPTIONS]");
        }
        if !self.p.is_set(AS::UnifiedHelpMessage)
            && self
                .p
                .app
                .get_opts_no_heading()
                .any(|o| !o.is_set(ArgSettings::Required) && !o.is_set(ArgSettings::Hidden))
        {
            usage.push_str(" [OPTIONS]");
        }

        usage.push_str(&req_string[..]);

        let has_last = self
            .p
            .app
            .get_positionals()
            .any(|p| p.is_set(ArgSettings::Last));
        // places a '--' in the usage string if there are args and options
        // supporting multiple values
        if self
            .p
            .app
            .get_opts_no_heading()
            .any(|o| o.is_set(ArgSettings::MultipleValues))
            && self
                .p
                .app
                .get_positionals()
                .any(|p| !p.is_set(ArgSettings::Required))
            && !(self.p.app.has_visible_subcommands()
                || self.p.is_set(AS::AllowExternalSubcommands))
            && !has_last
        {
            usage.push_str(" [--]");
        }
        let not_req_or_hidden = |p: &Arg| {
            (!p.is_set(ArgSettings::Required) || p.is_set(ArgSettings::Last))
                && !p.is_set(ArgSettings::Hidden)
        };
        if self.p.app.get_positionals().any(not_req_or_hidden) {
            if let Some(args_tag) = self.get_args_tag(incl_reqs) {
                usage.push_str(&*args_tag);
            } else {
                usage.push_str(" [ARGS]");
            }
            if has_last && incl_reqs {
                let pos = self
                    .p
                    .app
                    .get_positionals()
                    .find(|p| p.is_set(ArgSettings::Last))
                    .expect(INTERNAL_ERROR_MSG);
                debug!("Usage::create_help_usage: '{}' has .last(true)", pos.name);
                let req = pos.is_set(ArgSettings::Required);
                if req
                    && self
                        .p
                        .app
                        .get_positionals()
                        .any(|p| !p.is_set(ArgSettings::Required))
                {
                    usage.push_str(" -- <");
                } else if req {
                    usage.push_str(" [--] <");
                } else {
                    usage.push_str(" [-- <");
                }
                usage.push_str(&*pos.name_no_brackets());
                usage.push_str(">");
                usage.push_str(pos.multiple_str());
                if !req {
                    usage.push_str("]");
                }
            }
        }

        // incl_reqs is only false when this function is called recursively
        if self.p.app.has_visible_subcommands() && incl_reqs
            || self.p.is_set(AS::AllowExternalSubcommands)
        {
            if self.p.is_set(AS::SubcommandsNegateReqs) || self.p.is_set(AS::ArgsNegateSubcommands)
            {
                if !self.p.is_set(AS::ArgsNegateSubcommands) {
                    usage.push_str("\n    ");
                    usage.push_str(&*self.create_help_usage(false));

                    usage.push_str(" <");
                    usage.push_str(self.p.app.subcommand_placeholder.unwrap_or("SUBCOMMAND"));
                    usage.push_str(">");
                } else {
                    usage.push_str("\n    ");
                    usage.push_str(&*name);

                    usage.push_str(" <");
                    usage.push_str(self.p.app.subcommand_placeholder.unwrap_or("SUBCOMMAND"));
                    usage.push_str(">");
                }
            } else if self.p.is_set(AS::SubcommandRequired)
                || self.p.is_set(AS::SubcommandRequiredElseHelp)
            {
                usage.push_str(" <");
                usage.push_str(self.p.app.subcommand_placeholder.unwrap_or("SUBCOMMAND"));
                usage.push_str(">");
            } else {
                usage.push_str(" [");
                usage.push_str(self.p.app.subcommand_placeholder.unwrap_or("SUBCOMMAND"));
                usage.push_str("]");
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
            .fold(String::new(), |acc, s| acc + &format!(" {}", s)[..]);

        usage.push_str(
            &self
                .p
                .app
                .usage
                .as_ref()
                .unwrap_or_else(|| self.p.app.bin_name.as_ref().unwrap_or(&self.p.app.name))[..],
        );
        usage.push_str(&*r_string);
        if self.p.is_set(AS::SubcommandRequired) {
            usage.push_str(" <");
            usage.push_str(self.p.app.subcommand_placeholder.unwrap_or("SUBCOMMAND"));
            usage.push_str(">");
        }
        usage.shrink_to_fit();
        usage
    }

    // Gets the `[ARGS]` tag for the usage string
    fn get_args_tag(&self, incl_reqs: bool) -> Option<String> {
        debug!("Usage::get_args_tag; incl_reqs = {:?}", incl_reqs);
        let mut count = 0;
        'outer: for pos in self
            .p
            .app
            .get_positionals()
            .filter(|pos| !pos.is_set(ArgSettings::Required))
            .filter(|pos| !pos.is_set(ArgSettings::Hidden))
            .filter(|pos| !pos.is_set(ArgSettings::Last))
        {
            debug!("Usage::get_args_tag:iter:{}", pos.name);
            for grp_s in self.p.app.groups_for_arg(&pos.id) {
                debug!("Usage::get_args_tag:iter:{:?}:iter:{:?}", pos.name, grp_s);
                // if it's part of a required group we don't want to count it
                if self
                    .p
                    .app
                    .groups
                    .iter()
                    .any(|g| g.required && (g.id == grp_s))
                {
                    continue 'outer;
                }
            }
            count += 1;
            debug!(
                "Usage::get_args_tag:iter: {} Args not required or hidden",
                count
            );
        }
        if !self.p.is_set(AS::DontCollapseArgsInUsage) && count > 1 {
            debug!("Usage::get_args_tag:iter: More than one, returning [ARGS]");
            return None; // [ARGS]
        } else if count == 1 && incl_reqs {
            let pos = self
                .p
                .app
                .get_positionals()
                .find(|pos| {
                    !pos.is_set(ArgSettings::Required)
                        && !pos.is_set(ArgSettings::Hidden)
                        && !pos.is_set(ArgSettings::Last)
                })
                .expect(INTERNAL_ERROR_MSG);
            debug!(
                "Usage::get_args_tag:iter: Exactly one, returning '{}'",
                pos.name
            );
            return Some(format!(
                " [{}]{}",
                pos.name_no_brackets(),
                pos.multiple_str()
            ));
        } else if self.p.is_set(AS::DontCollapseArgsInUsage)
            && self.p.has_positionals()
            && incl_reqs
        {
            debug!("Usage::get_args_tag:iter: Don't collapse returning all");
            return Some(
                self.p
                    .app
                    .get_positionals()
                    .filter(|pos| !pos.is_set(ArgSettings::Required))
                    .filter(|pos| !pos.is_set(ArgSettings::Hidden))
                    .filter(|pos| !pos.is_set(ArgSettings::Last))
                    .map(|pos| format!(" [{}]{}", pos.name_no_brackets(), pos.multiple_str()))
                    .collect::<Vec<_>>()
                    .join(""),
            );
        } else if !incl_reqs {
            debug!("Usage::get_args_tag:iter: incl_reqs=false, building secondary usage string");
            let highest_req_pos = self
                .p
                .app
                .get_positionals()
                .filter_map(|pos| {
                    if pos.is_set(ArgSettings::Required) && !pos.is_set(ArgSettings::Last) {
                        Some(pos.index)
                    } else {
                        None
                    }
                })
                .max()
                .unwrap_or_else(|| Some(self.p.app.get_positionals().count() as u64));
            return Some(
                self.p
                    .app
                    .get_positionals()
                    .filter(|pos| pos.index <= highest_req_pos)
                    .filter(|pos| !pos.is_set(ArgSettings::Required))
                    .filter(|pos| !pos.is_set(ArgSettings::Hidden))
                    .filter(|pos| !pos.is_set(ArgSettings::Last))
                    .map(|pos| format!(" [{}]{}", pos.name_no_brackets(), pos.multiple_str()))
                    .collect::<Vec<_>>()
                    .join(""),
            );
        }
        Some("".into())
    }

    // Determines if we need the `[FLAGS]` tag in the usage string
    fn needs_flags_tag(&self) -> bool {
        debug!("Usage::needs_flags_tag");
        'outer: for f in self.p.app.get_flags_no_heading() {
            debug!("Usage::needs_flags_tag:iter: f={}", f.name);
            if let Some(l) = f.long {
                if l == "help" || l == "version" {
                    // Don't print `[FLAGS]` just for help or version
                    continue;
                }
            }
            for grp_s in self.p.app.groups_for_arg(&f.id) {
                debug!("Usage::needs_flags_tag:iter:iter: grp_s={:?}", grp_s);
                if self
                    .p
                    .app
                    .groups
                    .iter()
                    .any(|g| g.id == grp_s && g.required)
                {
                    debug!("Usage::needs_flags_tag:iter:iter: Group is required");
                    continue 'outer;
                }
            }
            if f.is_set(ArgSettings::Hidden) {
                continue;
            }
            debug!("Usage::needs_flags_tag:iter: [FLAGS] required");
            return true;
        }

        debug!("Usage::needs_flags_tag: [FLAGS] not required");
        false
    }

    // Returns the required args in usage string form by fully unrolling all groups
    // `incl_last`: should we incldue args that are Arg::Last? (i.e. `prog [foo] -- [last]). We
    // can't do that for required usages being built for subcommands because it would look like:
    // `prog [foo] -- [last] <subcommand>` which is totally wrong.
    pub(crate) fn get_required_usage_from(
        &self,
        incls: &[Id],
        matcher: Option<&ArgMatcher>,
        incl_last: bool,
    ) -> VecDeque<String> {
        debug!(
            "Usage::get_required_usage_from: incls={:?}, matcher={:?}, incl_last={:?}",
            incls,
            matcher.is_some(),
            incl_last
        );
        let mut ret_val = VecDeque::new();

        let mut unrolled_reqs = vec![];

        for a in self.p.required.iter() {
            if let Some(ref m) = matcher {
                for aa in self.p.app.unroll_requirements_for_arg(a, m) {
                    unrolled_reqs.push(aa);
                }
            }
            // always include the required arg itself. it will not be enumerated
            // by unroll_requirements_for_arg.
            unrolled_reqs.push(a.clone());
        }

        debug!(
            "Usage::get_required_usage_from: unrolled_reqs={:?}",
            unrolled_reqs
        );

        let args_in_groups = self
            .p
            .app
            .groups
            .iter()
            .filter(|gn| self.p.required.contains(&gn.id))
            .flat_map(|g| self.p.app.unroll_args_in_group(&g.id))
            .collect::<Vec<_>>();

        let pmap = unrolled_reqs
            .iter()
            .chain(incls.iter())
            .filter(|a| self.p.app.get_positionals().any(|p| &&p.id == a))
            .filter(|&pos| matcher.map_or(true, |m| !m.contains(pos)))
            .filter_map(|pos| self.p.app.find(pos))
            .filter(|&pos| incl_last || !pos.is_set(ArgSettings::Last))
            .filter(|pos| !args_in_groups.contains(&pos.id))
            .map(|pos| (pos.index.unwrap(), pos))
            .collect::<BTreeMap<u64, &Arg>>(); // sort by index

        for p in pmap.values() {
            debug!("Usage::get_required_usage_from:iter:{:?}", p.id);
            if args_in_groups.is_empty() || !args_in_groups.contains(&p.id) {
                ret_val.push_back(p.to_string());
            }
        }
        for a in unrolled_reqs
            .iter()
            .chain(incls.iter())
            .filter(|name| !self.p.app.get_positionals().any(|p| &&p.id == name))
            .filter(|name| !self.p.app.groups.iter().any(|g| &&g.id == name))
            .filter(|name| !args_in_groups.contains(name))
            .filter(|name| !(matcher.is_some() && matcher.as_ref().unwrap().contains(name)))
        {
            debug!("Usage::get_required_usage_from:iter:{:?}", a);
            let arg = self
                .p
                .app
                .find(&a)
                .map(ToString::to_string)
                .expect(INTERNAL_ERROR_MSG);
            ret_val.push_back(arg);
        }
        let mut g_vec: Vec<String> = vec![];
        for g in unrolled_reqs
            .iter()
            .filter(|n| self.p.app.groups.iter().any(|g| g.id == **n))
        {
            // don't print requirement for required groups that have an arg.
            if let Some(m) = matcher {
                let have_group_entry = self
                    .p
                    .app
                    .unroll_args_in_group(&g)
                    .iter()
                    .any(|arg| m.contains(&arg));
                if have_group_entry {
                    continue;
                }
            }

            let elem = self.p.app.format_group(g);
            if !g_vec.contains(&elem) {
                g_vec.push(elem);
            }
        }
        for g in g_vec {
            ret_val.push_back(g);
        }

        debug!("Usage::get_required_usage_from: ret_val={:?}", ret_val);
        ret_val
    }
}
