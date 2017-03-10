use args::{AnyArg, ArgMatcher, PosBuilder};
use args::settings::ArgSettings;
use app::settings::AppSettings as AS;
use app::parser::Parser;

// Creates a usage string for display. This happens just after all arguments were parsed, but before
// any subcommands have been parsed (so as to give subcommands their own usage recursively)
pub fn create_usage_with_title(p: &Parser, used: &[&str]) -> String {
    debugln!("usage::create_usage_with_title;");
    let mut usage = String::with_capacity(75);
    usage.push_str("USAGE:\n    ");
    usage.push_str(&*create_usage_no_title(p, used));
    usage
}

// Creates a usage string to be used in error message (i.e. one with currently used args)
pub fn create_error_usage<'a, 'b>(p: &Parser<'a, 'b>,
                                  matcher: &'b ArgMatcher<'a>,
                                  extra: Option<&str>)
                                  -> String {
    let mut args: Vec<_> = matcher.arg_names()
        .iter()
        .filter(|n| {
            if let Some(o) = find_by_name!(p, *n, opts, iter) {
                !o.b.is_set(ArgSettings::Required) && !o.b.is_set(ArgSettings::Hidden)
            } else if let Some(p) = find_by_name!(p, *n, positionals, values) {
                !p.b.is_set(ArgSettings::Required) && p.b.is_set(ArgSettings::Hidden)
            } else {
                true // flags can't be required, so they're always true
            }
        })
        .map(|&n| n)
        .collect();
    if let Some(r) = extra {
        args.push(r);
    }
    create_usage_with_title(p, &*args)
}

// Creates a usage string (*without title*) if one was not provided by the user manually.
fn create_usage_no_title(p: &Parser, used: &[&str]) -> String {
    debugln!("usage::create_usage_no_title;");
    if let Some(u) = p.meta.usage_str {
        String::from(&*u)
    } else if used.is_empty() {
        create_help_usage(p, true)
    } else {
        create_smart_usage(p, used)
    }
}

// Creates a usage string for display in help messages (i.e. not for errors)
pub fn create_help_usage(p: &Parser, incl_reqs: bool) -> String {
    let mut usage = String::with_capacity(75);
    let name = p.meta
        .usage
        .as_ref()
        .unwrap_or_else(|| {
                            p.meta
                                .bin_name
                                .as_ref()
                                .unwrap_or(&p.meta.name)
                        });
    usage.push_str(&*name);
    let req_string = if incl_reqs {
        let mut reqs: Vec<&str> = p.required().map(|r| &**r).collect();
        reqs.sort();
        reqs.dedup();
        p.get_required_from(&reqs, None, None).iter().fold(String::new(),
                                                           |a, s| a + &format!(" {}", s)[..])
    } else {
        String::new()
    };

    let flags = p.needs_flags_tag();
    if flags && !p.is_set(AS::UnifiedHelpMessage) {
        usage.push_str(" [FLAGS]");
    } else if flags {
        usage.push_str(" [OPTIONS]");
    }
    if !p.is_set(AS::UnifiedHelpMessage) &&
       p.opts.iter().any(|o| !o.is_set(ArgSettings::Required) && !o.is_set(ArgSettings::Hidden)) {
        usage.push_str(" [OPTIONS]");
    }

    usage.push_str(&req_string[..]);

    // places a '--' in the usage string if there are args and options
    // supporting multiple values
    if p.has_positionals() && p.opts.iter().any(|o| o.is_set(ArgSettings::Multiple)) &&
       p.positionals.values().any(|p| !p.is_set(ArgSettings::Required)) &&
       !p.has_visible_subcommands() {
        usage.push_str(" [--]")
    }
    let not_req_or_hidden= |p: &PosBuilder| !p.is_set(ArgSettings::Required) && !p.is_set(ArgSettings::Hidden);
    if p.has_positionals() && p.positionals.values().any(not_req_or_hidden) {
        if let Some(args_tag) = p.get_args_tag() {
            usage.push_str(&*args_tag);
        } else {
            usage.push_str(" [ARGS]");
        }
    }

    // incl_reqs is only false when this function is called recursively
    if p.has_visible_subcommands() && incl_reqs {
        if p.is_set(AS::SubcommandsNegateReqs) || p.is_set(AS::ArgsNegateSubcommands) {
            if !p.is_set(AS::ArgsNegateSubcommands) {
                usage.push_str("\n    ");
                usage.push_str(&*create_help_usage(p, false));
                usage.push_str(" <SUBCOMMAND>");
            } else {
                usage.push_str("\n    ");
                usage.push_str(&*name);
                usage.push_str(" <SUBCOMMAND>");
            }
        } else if p.is_set(AS::SubcommandRequired) || p.is_set(AS::SubcommandRequiredElseHelp) {
            usage.push_str(" <SUBCOMMAND>");
        } else {
            usage.push_str(" [SUBCOMMAND]");
        }
    }
    usage.shrink_to_fit();
    usage
}

// Creates a context aware usage string, or "smart usage" from currently used
// args, and requirements
fn create_smart_usage(p: &Parser, used: &[&str]) -> String {
    debugln!("usage::smart_usage;");
    let mut usage = String::with_capacity(75);
    let mut hs: Vec<&str> = p.required().map(|s| &**s).collect();
    hs.extend_from_slice(used);

    let r_string = p.get_required_from(&hs, None, None).iter().fold(String::new(), |acc, s| {
        acc + &format!(" {}", s)[..]
    });

    usage.push_str(&p.meta
                        .usage
                        .as_ref()
                        .unwrap_or_else(|| {
                                            p.meta
                                                .bin_name
                                                .as_ref()
                                                .unwrap_or(&p.meta.name)
                                        })
                        [..]);
    usage.push_str(&*r_string);
    if p.is_set(AS::SubcommandRequired) {
        usage.push_str(" <SUBCOMMAND>");
    }
    usage.shrink_to_fit();
    usage
}
