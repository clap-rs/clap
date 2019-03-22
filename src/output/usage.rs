// std
use std::collections::{BTreeMap, VecDeque};

// Internal
use crate::build::AppSettings as AS;
use crate::build::{App, Arg, ArgSettings};
use crate::parse::{ArgMatcher, Parser};
use crate::INTERNAL_ERROR_MSG;

// Creates a usage string for display. This happens just after all arguments were parsed, but before
// any subcommands have been parsed (so as to give subcommands their own usage recursively)
pub fn with_title(app: &App, used: &[u64]) -> String {
    debugln!("usage::with_title;");
    let mut usage = String::with_capacity(75);
    usage.push_str("USAGE:\n    ");
    usage.push_str(&*without_title(app, used));
    usage
}

// Creates a usage string (*without title*) if one was not provided by the user manually.
pub fn without_title(app: &App, used: &[u64]) -> String {
    debugln!("usage::create_usage_no_title;");
    if let Some(u) = app.usage_str {
        String::from(&*u)
    } else if used.is_empty() {
        help_usage(app, true)
    } else {
        smart_usage(app, used)
    }
}

// Creates a usage string for display in help messages (i.e. not for errors)
pub fn help_usage(app: &App, incl_reqs: bool) -> String {
    debugln!("Usage::create_help_usage; incl_reqs={:?}", incl_reqs);
    let mut usage = String::with_capacity(75);
    let name = app
        .usage
        .as_ref()
        .unwrap_or_else(|| app.bin_name.as_ref().unwrap_or(&app.name));
    usage.push_str(&*name);
    let req_string = if incl_reqs {
        required_usage_from(app, &[], None, false)
            .iter()
            .fold(String::new(), |a, s| a + &format!(" {}", s)[..])
    } else {
        String::new()
    };

    let flags = needs_flags_tag(app);
    if flags && !app.is_set(AS::UnifiedHelpMessage) {
        usage.push_str(" [FLAGS]");
    } else if flags {
        usage.push_str(" [OPTIONS]");
    }
    if !app.is_set(AS::UnifiedHelpMessage)
        && opts!(app).any(|o| !o.is_set(ArgSettings::Required) && !o.is_set(ArgSettings::Hidden))
    {
        usage.push_str(" [OPTIONS]");
    }

    usage.push_str(&req_string[..]);

    let has_last = app.args.positionals().any(|p| p.is_set(ArgSettings::Last));
    // places a '--' in the usage string if there are args and options
    // supporting multiple values
    if app
        .args
        .options()
        .any(|o| o.is_set(ArgSettings::MultipleValues))
        && app
            .args
            .positionals()
            .any(|p| !p.is_set(ArgSettings::Required))
        && !(app.has_visible_subcommands() || app.is_set(AS::AllowExternalSubcommands))
        && !has_last
    {
        usage.push_str(" [--]");
    }
    let not_req_or_hidden = |p: &Arg| {
        (!p.is_set(ArgSettings::Required) || p.is_set(ArgSettings::Last))
            && !p.is_set(ArgSettings::Hidden)
    };
    if app.args.positionals().any(not_req_or_hidden) {
        if let Some(args_tag) = get_args_tag(app, incl_reqs) {
            usage.push_str(&*args_tag);
        } else {
            usage.push_str(" [ARGS]");
        }
        if has_last && incl_reqs {
            let pos = app
                .args
                .positionals()
                .find(|p| p.is_set(ArgSettings::Last))
                .expect(INTERNAL_ERROR_MSG);
            debugln!("Usage::create_help_usage: '{}' has .last(true)", pos.name);
            let req = pos.is_set(ArgSettings::Required);
            if req
                && app
                    .args
                    .positionals()
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
    if app.has_visible_subcommands() && incl_reqs || app.is_set(AS::AllowExternalSubcommands) {
        if app.is_set(AS::SubcommandsNegateReqs) || app.is_set(AS::ArgsNegateSubcommands) {
            if !app.is_set(AS::ArgsNegateSubcommands) {
                usage.push_str("\n    ");
                usage.push_str(&*help_usage(app, false));
                usage.push_str(" <SUBCOMMAND>");
            } else {
                usage.push_str("\n    ");
                usage.push_str(&*name);
                usage.push_str(" <SUBCOMMAND>");
            }
        } else if app.is_set(AS::SubcommandRequired) || app.is_set(AS::SubcommandRequiredElseHelp) {
            usage.push_str(" <SUBCOMMAND>");
        } else {
            usage.push_str(" [SUBCOMMAND]");
        }
    }
    usage.shrink_to_fit();
    debugln!("usage::create_help_usage: usage={}", usage);
    usage
}

// Creates a context aware usage string, or "smart usage" from currently used
// args, and requirements
fn smart_usage(app: &App, used: &[u64]) -> String {
    debugln!("usage::smart_usage;");
    let mut usage = String::with_capacity(75);

    let r_string = required_usage_from(app, used, None, true)
        .iter()
        .fold(String::new(), |acc, s| acc + &format!(" {}", s)[..]);

    usage.push_str(
        &app.app
            .usage
            .as_ref()
            .unwrap_or_else(|| app.bin_name.as_ref().unwrap_or(&app.name))[..],
    );
    usage.push_str(&*r_string);
    if app.is_set(AS::SubcommandRequired) {
        usage.push_str(" <SUBCOMMAND>");
    }
    usage.shrink_to_fit();
    usage
}

// Gets the `[ARGS]` tag for the usage string
fn args_tag(app: &App, incl_reqs: bool) -> Option<String> {
    debugln!("usage::get_args_tag; incl_reqs = {:?}", incl_reqs);
    let mut count = 0;
    'outer: for pos in app
        .args
        .positionals()
        .filter(|pos| !pos.is_set(ArgSettings::Required))
        .filter(|pos| !pos.is_set(ArgSettings::Hidden))
        .filter(|pos| !pos.is_set(ArgSettings::Last))
    {
        debugln!("usage::get_args_tag:iter:{}:", pos.name);
        for grp_s in groups_for_arg!(app, &pos.id) {
            debugln!("usage::get_args_tag:iter:{}:iter:{};", pos.name, grp_s);
            // if it's part of a required group we don't want to count it
            if app.app.groups.iter().any(|g| g.required && (g.id == grp_s)) {
                continue 'outer;
            }
        }
        count += 1;
        debugln!(
            "usage::get_args_tag:iter: {} Args not required or hidden",
            count
        );
    }
    if !app.is_set(AS::DontCollapseArgsInUsage) && count > 1 {
        debugln!("usage::get_args_tag:iter: More than one, returning [ARGS]");
        return None; // [ARGS]
    } else if count == 1 && incl_reqs {
        let pos = app
            .args
            .positionals()
            .find(|pos| {
                !pos.is_set(ArgSettings::Required)
                    && !pos.is_set(ArgSettings::Hidden)
                    && !pos.is_set(ArgSettings::Last)
            })
            .expect(INTERNAL_ERROR_MSG);
        debugln!(
            "usage::get_args_tag:iter: Exactly one, returning '{}'",
            pos.name
        );
        return Some(format!(
            " [{}]{}",
            pos.name_no_brackets(),
            pos.multiple_str()
        ));
    } else if app.is_set(AS::DontCollapseArgsInUsage) && app.has_positionals() && incl_reqs {
        debugln!("usage::get_args_tag:iter: Don't collapse returning all");
        return Some(
            app.args
                .positionals()
                .filter(|pos| !pos.is_set(ArgSettings::Required))
                .filter(|pos| !pos.is_set(ArgSettings::Hidden))
                .filter(|pos| !pos.is_set(ArgSettings::Last))
                .map(|pos| format!(" [{}]{}", pos.name_no_brackets(), pos.multiple_str()))
                .collect::<Vec<_>>()
                .join(""),
        );
    } else if !incl_reqs {
        debugln!("usage::get_args_tag:iter: incl_reqs=false, building secondary usage string");
        let highest_req_pos = app
            .args
            .positionals()
            .filter_map(|pos| {
                if pos.is_set(ArgSettings::Required) && !pos.is_set(ArgSettings::Last) {
                    Some(pos.index)
                } else {
                    None
                }
            })
            .max()
            .unwrap_or_else(|| Some(app.args.positionals().count() as u64));
        return Some(
            app.args
                .positionals()
                .filter_map(|pos| {
                    if pos.index <= highest_req_pos {
                        Some(pos)
                    } else {
                        None
                    }
                })
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
fn needs_flags_tag(app: &App) -> bool {
    debugln!("usage::needs_flags_tag;");
    'outer: for f in flags!(app) {
        debugln!("usage::needs_flags_tag:iter: f={};", f.name);
        if let Some(l) = f.long {
            if l == "help" || l == "version" {
                // Don't print `[FLAGS]` just for help or version
                continue;
            }
        }
        for grp_s in groups_for_arg!(app, &f.id) {
            debugln!("usage::needs_flags_tag:iter:iter: grp_s={};", grp_s);
            if app.groups.iter().any(|g| g.id == grp_s && g.required) {
                debugln!("usage::needs_flags_tag:iter:iter: Group is required");
                continue 'outer;
            }
        }
        if f.is_set(ArgSettings::Hidden) {
            continue;
        }
        debugln!("usage::needs_flags_tag:iter: [FLAGS] required");
        return true;
    }

    debugln!("usage::needs_flags_tag: [FLAGS] not required");
    false
}

// Returns the required args in usage string form by fully unrolling all groups
// `incl_last`: should we incldue args that are Arg::Last? (i.e. `prog [foo] -- [last]). We
// can't do that for required usages being built for subcommands because it would look like:
// `prog [foo] -- [last] <subcommand>` which is totally wrong.
pub fn required_usage_from(
    app: &App,
    incls: &[u64],
    matcher: Option<&ArgMatcher>,
    incl_last: bool,
) -> VecDeque<String> {
    debugln!(
        "Usage::get_required_usage_from: incls={:?}, matcher={:?}, incl_last={:?}",
        incls,
        matcher.is_some(),
        incl_last
    );
    let mut ret_val = VecDeque::new();

    let mut unrolled_reqs = vec![];

    for a in app.required.iter() {
        if let Some(ref m) = matcher {
            for aa in app.unroll_requirements_for_arg(*a, m) {
                unrolled_reqs.push(aa);
            }
        } else {
            unrolled_reqs.push(*a);
        }
    }

    let args_in_groups = app
        .groups
        .iter()
        // @TODO @p1 figure out required instead of parser
        .filter(|gn| app.required.contains(gn.id))
        .flat_map(|g| app.unroll_args_in_group(g.id))
        .collect::<Vec<_>>();

    let pmap = if let Some(m) = matcher {
        unrolled_reqs
            .iter()
            .chain(incls.iter())
            .filter(|a| app.args.positionals().any(|p| &&p.id == a))
            .filter(|&pos| !m.contains(*pos))
            .filter_map(|pos| app.find(*pos))
            .filter(|&pos| incl_last || !pos.is_set(ArgSettings::Last))
            .filter(|pos| !args_in_groups.contains(&pos.id))
            .map(|pos| (pos.index.unwrap(), pos))
            .collect::<BTreeMap<u64, &Arg>>() // sort by index
    } else {
        unrolled_reqs
            .iter()
            .chain(incls.iter())
            .filter(|a| app.args.positionals().any(|p| &&p.id == a))
            .filter_map(|pos| app.find(*pos))
            .filter(|&pos| incl_last || !pos.is_set(ArgSettings::Last))
            .filter(|pos| !args_in_groups.contains(&pos.id))
            .map(|pos| (pos.index.unwrap(), pos))
            .collect::<BTreeMap<u64, &Arg>>() // sort by index
    };
    for &p in pmap.values() {
        debugln!("Usage::get_required_usage_from:iter:{}", p);
        if args_in_groups.is_empty() || !args_in_groups.contains(&p.id) {
            ret_val.push_back(p.to_string());
        }
    }
    for a in unrolled_reqs
        .iter()
        .chain(incls.iter())
        .filter(|name| !app.args.positionals().any(|p| &&p.id == name))
        .filter(|name| !app.groups.iter().any(|g| &&g.id == name))
        .filter(|name| !args_in_groups.contains(name))
        .filter(|name| !(matcher.is_some() && matcher.unwrap().contains(**name)))
    {
        debugln!("Usage::get_required_usage_from:iter:{}:", a);
        let arg = app
            .find(*a)
            .map(|f| f.to_string())
            .expect(INTERNAL_ERROR_MSG);
        ret_val.push_back(arg);
    }
    let mut g_vec: Vec<String> = vec![];
    for g in unrolled_reqs
        .iter()
        .filter(|n| app.groups.iter().any(|g| &&g.id == n))
    {
        let elem = app.format_group(*g);
        if !g_vec.contains(&elem) {
            g_vec.push(elem);
        }
    }
    for g in g_vec {
        ret_val.push_back(g);
    }

    debugln!("Usage::get_required_usage_from: ret_val={:?}", ret_val);
    ret_val
}
