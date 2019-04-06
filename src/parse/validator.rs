// Internal
use crate::build::app::AppSettings as AS;
use crate::build::{Arg, ArgSettings};
use crate::output::fmt::{Colorizer, ColorizerOption};
use crate::output::Usage;
use crate::parse::errors::Result as ClapResult;
use crate::parse::errors::{Error, ErrorKind};
use crate::parse::{ArgMatcher, MatchedArg, ParseResult, Parser};
use crate::util::ChildGraph;
use crate::INTERNAL_ERROR_MSG;
use crate::INVALID_UTF8;

type Id = u64;

pub struct Validator<'b, 'c, 'z>
where
    'b: 'c,
    'c: 'z,
{
    p: &'z mut Parser<'b, 'c>,
    c: ChildGraph<Id>,
}

impl<'b, 'c, 'z> Validator<'b, 'c, 'z> {
    pub fn new(p: &'z mut Parser<'b, 'c>) -> Self {
        Validator {
            p,
            c: ChildGraph::with_capacity(5),
        }
    }

    pub fn validate(
        &mut self,
        needs_val_of: ParseResult,
        subcmd_name: &Option<String>,
        matcher: &mut ArgMatcher,
    ) -> ClapResult<()> {
        debugln!("Validator::validate;");
        let mut reqs_validated = false;
        self.p.add_env(matcher)?;
        self.p.add_defaults(matcher)?;
        if let ParseResult::Opt(a) = needs_val_of {
            debugln!("Validator::validate: needs_val_of={:?}", a);
            {
                self.validate_required(matcher)?;
            }
            let o = self.p.app.find(a).expect(INTERNAL_ERROR_MSG);
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
                ));
            }
        }

        if matcher.is_empty()
            && matcher.subcommand_name().is_none()
            && self.p.is_set(AS::ArgRequiredElseHelp)
        {
            let mut out = vec![];
            self.p.write_help_err(&mut out)?;
            return Err(Error {
                message: String::from_utf8_lossy(&*out).into_owned(),
                kind: ErrorKind::MissingArgumentOrSubcommand,
                info: None,
            });
        }
        self.validate_conflicts(matcher)?;
        if !(self.p.is_set(AS::SubcommandsNegateReqs) && subcmd_name.is_some() || reqs_validated) {
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
        debugln!("Validator::validate_arg_values: arg={:?}", arg.name);
        for val in &ma.vals {
            if self.p.is_set(AS::StrictUtf8) && val.to_str().is_none() {
                debugln!(
                    "Validator::validate_arg_values: invalid UTF-8 found in val {:?}",
                    val
                );
                return Err(Error::invalid_utf8(
                    &*Usage::new(self.p).create_usage_with_title(&[]),
                    self.p.app.color(),
                ));
            }
            if let Some(ref p_vals) = arg.possible_vals {
                debugln!("Validator::validate_arg_values: possible_vals={:?}", p_vals);
                let val_str = val.to_string_lossy();
                let ok = if arg.is_set(ArgSettings::IgnoreCase) {
                    p_vals.iter().any(|pv| pv.eq_ignore_ascii_case(&*val_str))
                } else {
                    p_vals.contains(&&*val_str)
                };
                if !ok {
                    let used: Vec<Id> = matcher
                        .arg_names()
                        .filter(|&&n| {
                            if let Some(a) = self.p.app.find(n) {
                                !(self.p.required.contains(a.id) || a.is_set(ArgSettings::Hidden))
                            } else {
                                true
                            }
                        })
                        .cloned()
                        .collect();
                    return Err(Error::invalid_value(
                        val_str,
                        p_vals,
                        arg,
                        &*Usage::new(self.p).create_usage_with_title(&*used),
                        self.p.app.color(),
                    ));
                }
            }
            if !arg.is_set(ArgSettings::AllowEmptyValues)
                && val.is_empty()
                && matcher.contains(arg.id)
            {
                debugln!("Validator::validate_arg_values: illegal empty val found");
                return Err(Error::empty_value(
                    arg,
                    &*Usage::new(self.p).create_usage_with_title(&[]),
                    self.p.app.color(),
                ));
            }
            if let Some(ref vtor) = arg.validator {
                debug!("Validator::validate_arg_values: checking validator...");
                if let Err(e) = vtor(val.to_string_lossy().into_owned()) {
                    sdebugln!("error");
                    return Err(Error::value_validation(Some(arg), &e, self.p.app.color()));
                } else {
                    sdebugln!("good");
                }
            }
            if let Some(ref vtor) = arg.validator_os {
                debug!("Validator::validate_arg_values: checking validator_os...");
                if let Err(e) = vtor(val) {
                    sdebugln!("error");
                    return Err(Error::value_validation(
                        Some(arg),
                        &(*e).to_string(),
                        self.p.app.color(),
                    ));
                } else {
                    sdebugln!("good");
                }
            }
        }
        Ok(())
    }

    fn build_conflict_err(&self, name: Id, matcher: &ArgMatcher) -> ClapResult<()> {
        debugln!("build_err!: name={}", name);
        let usg = Usage::new(self.p).create_usage_with_title(&[]);
        if self.p.app.find(name).is_some() {
            for &k in matcher.arg_names() {
                if let Some(a) = self.p.app.find(k) {
                    if let Some(ref v) = a.blacklist {
                        if v.contains(&name) {
                            return Err(Error::argument_conflict(
                                a,
                                Some(a.to_string()),
                                &*usg,
                                self.p.app.color(),
                            ));
                        }
                    }
                }
            }
        } else if let Some(g) = self.p.app.groups.iter().find(|x| x.id == name) {
            let args_in_group = self.p.app.unroll_args_in_group(g.id);
            let first = matcher
                .arg_names()
                .find(|x| args_in_group.contains(x))
                .expect(INTERNAL_ERROR_MSG);
            let c_with = matcher
                .arg_names()
                .find(|x| x != &first && args_in_group.contains(x))
                .map(|&x| self.p.app.find(x).expect(INTERNAL_ERROR_MSG).to_string());
            debugln!("build_err!:c_with={:?}:group", c_with);
            return Err(Error::argument_conflict(
                self.p.app.find(*first).expect(INTERNAL_ERROR_MSG),
                c_with,
                &*usg,
                self.p.app.color(),
            ));
        }

        panic!(INTERNAL_ERROR_MSG);
    }

    fn validate_conflicts(&mut self, matcher: &mut ArgMatcher) -> ClapResult<()> {
        debugln!("Validator::validate_conflicts;");
        self.gather_conflicts(matcher);
        for name in self.c.iter() {
            debugln!("Validator::validate_conflicts:iter:{};", name);
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
                    .unroll_args_in_group(g.id)
                    .iter()
                    .filter(|&&a| matcher.contains(a))
                    .count()
                    > 1;

                let conf_with_arg = if let Some(ref c) = g.conflicts {
                    c.iter().any(|&x| matcher.contains(x))
                } else {
                    false
                };

                let arg_conf_with_gr = matcher
                    .arg_names()
                    .filter_map(|&x| self.p.app.find(x))
                    .filter_map(|x| x.blacklist.as_ref())
                    .any(|c| c.iter().any(|&c| c == g.id));

                should_err = conf_with_self || conf_with_arg || arg_conf_with_gr;
            } else if let Some(ma) = matcher.get(*name) {
                debugln!(
                    "Validator::validate_conflicts:iter:{}: matcher contains it...",
                    name
                );
                should_err = ma.occurs > 0;
            }
            if should_err {
                return self.build_conflict_err(*name, matcher);
            }
        }
        Ok(())
    }

    // Gathers potential conflicts based on used argument, but without considering requirements
    // and such
    fn gather_conflicts(&mut self, matcher: &mut ArgMatcher) {
        debugln!("Validator::gather_conflicts;");
        for &name in matcher.arg_names() {
            debugln!("Validator::gather_conflicts:iter:{};", name);
            if let Some(arg) = self.p.app.find(name) {
                // Since an arg was used, every arg it conflicts with is added to the conflicts
                if let Some(ref bl) = arg.blacklist {
                    for &conf in bl {
                        if self.p.app.find(conf).is_some() {
                            if conf != name {
                                self.c.insert(conf);
                            }
                        } else {
                            // for g_arg in self.p.app.unroll_args_in_group(conf) {
                            //     if &g_arg != name {
                            self.c.insert(conf); // TODO ERROR is here - groups allow one arg but this line disallows all group args
                                                 //     }
                                                 // }
                        }
                    }
                }
                // Now we need to know which groups this arg was a memeber of, to add all other
                // args in that group to the conflicts, as well as any args those args conflict
                // with
                for grp in groups_for_arg!(self.p.app, name) {
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
                        self.c.insert(g.id);
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
                .find(|grp| grp.id == name)
            {
                debugln!("Validator::gather_conflicts:iter:{}:group;", name);
                self.c.insert(g.id);
            }
        }
    }

    fn gather_requirements(&mut self, matcher: &ArgMatcher) {
        debugln!("Validator::gather_requirements;");
        for &name in matcher.arg_names() {
            debugln!("Validator::gather_requirements:iter:{};", name);
            if let Some(arg) = self.p.app.find(name) {
                for req in self.p.app.unroll_requirements_for_arg(arg.id, matcher) {
                    self.p.required.insert(req);
                }
            } else if let Some(g) = self.p.app.groups.iter().find(|grp| grp.id == name) {
                debugln!("Validator::gather_conflicts:iter:{}:group;", name);
                if let Some(ref reqs) = g.requires {
                    for &r in reqs {
                        self.p.required.insert(r);
                    }
                }
            }
        }
    }

    fn validate_matched_args(&self, matcher: &mut ArgMatcher) -> ClapResult<()> {
        debugln!("Validator::validate_matched_args;");
        for (&name, ma) in matcher.iter() {
            debugln!(
                "Validator::validate_matched_args:iter:{}: vals={:#?}",
                name,
                ma.vals
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
                    .find(|g| g.id == name)
                    .expect(INTERNAL_ERROR_MSG);
                if let Some(ref g_reqs) = grp.requires {
                    if g_reqs.iter().any(|&n| !matcher.contains(n)) {
                        return self.missing_required_error(matcher, Some(name));
                    }
                }
            }
        }
        Ok(())
    }

    fn validate_arg_num_occurs(&self, a: &Arg, ma: &MatchedArg) -> ClapResult<()> {
        debugln!(
            "Validator::validate_arg_num_occurs: {}={};",
            a.name,
            ma.occurs
        );
        if ma.occurs > 1 && !a.is_set(ArgSettings::MultipleOccurrences) {
            // Not the first time, and we don't allow multiples
            return Err(Error::unexpected_multiple_usage(
                a,
                &*Usage::new(self.p).create_usage_with_title(&[]),
                self.p.app.color(),
            ));
        }
        Ok(())
    }

    fn validate_arg_num_vals(&self, a: &Arg, ma: &MatchedArg) -> ClapResult<()> {
        debugln!("Validator::validate_arg_num_vals;");
        if let Some(num) = a.num_vals {
            debugln!("Validator::validate_arg_num_vals: num_vals set...{}", num);
            let should_err = if a.is_set(ArgSettings::MultipleValues) {
                ((ma.vals.len() as u64) % num) != 0
            } else {
                num != (ma.vals.len() as u64)
            };
            if should_err {
                debugln!("Validator::validate_arg_num_vals: Sending error WrongNumberOfValues");
                return Err(Error::wrong_number_of_values(
                    a,
                    num,
                    if a.is_set(ArgSettings::MultipleValues) {
                        (ma.vals.len() % num as usize)
                    } else {
                        ma.vals.len()
                    },
                    if ma.vals.len() == 1
                        || (a.is_set(ArgSettings::MultipleValues)
                            && (ma.vals.len() % num as usize) == 1)
                    {
                        "as"
                    } else {
                        "ere"
                    },
                    &*Usage::new(self.p).create_usage_with_title(&[]),
                    self.p.app.color(),
                ));
            }
        }
        if let Some(num) = a.max_vals {
            debugln!("Validator::validate_arg_num_vals: max_vals set...{}", num);
            if (ma.vals.len() as u64) > num {
                debugln!("Validator::validate_arg_num_vals: Sending error TooManyValues");
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
                ));
            }
        }
        let min_vals_zero = if let Some(num) = a.min_vals {
            debugln!("Validator::validate_arg_num_vals: min_vals set: {}", num);
            if (ma.vals.len() as u64) < num && num != 0 {
                debugln!("Validator::validate_arg_num_vals: Sending error TooFewValues");
                return Err(Error::too_few_values(
                    a,
                    num,
                    ma.vals.len(),
                    &*Usage::new(self.p).create_usage_with_title(&[]),
                    self.p.app.color(),
                ));
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
            ));
        }
        Ok(())
    }

    fn validate_arg_requires(
        &self,
        a: &Arg<'b>,
        ma: &MatchedArg,
        matcher: &ArgMatcher,
    ) -> ClapResult<()> {
        debugln!("Validator::validate_arg_requires:{};", a.name);
        if let Some(ref a_reqs) = a.requires {
            for &(val, name) in a_reqs.iter().filter(|&&(val, _)| val.is_some()) {
                let missing_req =
                    |v| v == val.expect(INTERNAL_ERROR_MSG) && !matcher.contains(name);
                if ma.vals.iter().any(missing_req) {
                    return self.missing_required_error(matcher, Some(a.id));
                }
            }
            for &(_, name) in a_reqs.iter().filter(|&&(val, _)| val.is_none()) {
                if !matcher.contains(name) {
                    return self.missing_required_error(matcher, Some(name));
                }
            }
        }
        Ok(())
    }

    fn validate_required(&mut self, matcher: &ArgMatcher) -> ClapResult<()> {
        debugln!(
            "Validator::validate_required: required={:?};",
            self.p.required
        );
        self.gather_requirements(matcher);

        for &arg_or_group in self.p.required.iter().filter(|&&r| !matcher.contains(r)) {
            debugln!("Validator::validate_required:iter:aog={:?};", arg_or_group);
            if let Some(arg) = self.p.app.find(arg_or_group) {
                if !self.is_missing_required_ok(arg, matcher) {
                    return self.missing_required_error(matcher, None);
                }
            } else if let Some(group) = self.p.app.groups.iter().find(|g| g.id == arg_or_group) {
                if !self
                    .p
                    .app
                    .unroll_args_in_group(group.id)
                    .iter()
                    .any(|&a| matcher.contains(a))
                {
                    return self.missing_required_error(matcher, None);
                }
            }
        }

        // Validate the conditionally required args
        for (a, r_ifs) in self
            .p
            .app
            .args
            .args
            .iter()
            .filter(|a| a.r_ifs.is_some())
            .map(|a| (a, a.r_ifs.as_ref().unwrap()))
        {
            for (other, val) in r_ifs.iter() {
                if let Some(ma) = matcher.get(*other) {
                    if ma.contains_val(val) && !matcher.contains(a.id) {
                        return self.missing_required_error(matcher, Some(a.id));
                    }
                }
            }
        }
        Ok(())
    }

    fn is_missing_required_ok(&self, a: &Arg<'b>, matcher: &ArgMatcher) -> bool {
        debugln!("Validator::is_missing_required_ok: {}", a.name);
        self.validate_arg_conflicts(a, matcher) || self.p.overriden.contains(&a.id)
    }

    fn validate_arg_conflicts(&self, a: &Arg<'b>, matcher: &ArgMatcher) -> bool {
        debugln!("Validator::validate_arg_conflicts: a={:?};", a.name);
        a.blacklist
            .as_ref()
            .map(|bl| {
                bl.iter().any(|&conf| {
                    matcher.contains(conf)
                        || self
                            .p
                            .app
                            .groups
                            .iter()
                            .find(|g| g.id == conf)
                            .map_or(false, |g| g.args.iter().any(|&arg| matcher.contains(arg)))
                })
            })
            .unwrap_or(false)
    }

    fn validate_required_unless(&self, matcher: &ArgMatcher) -> ClapResult<()> {
        debugln!("Validator::validate_required_unless;");
        for a in self
            .p
            .app
            .args
            .args
            .iter()
            .filter(|a| a.r_unless.is_some())
            .filter(|a| !matcher.contains(a.id))
        {
            debugln!("Validator::validate_required_unless:iter:{};", a.name);
            if self.fails_arg_required_unless(a, matcher) {
                return self.missing_required_error(matcher, Some(a.id));
            }
        }

        Ok(())
    }

    // Failing a required unless means, the arg's "unless" wasn't present, and neither were they
    fn fails_arg_required_unless(&self, a: &Arg<'b>, matcher: &ArgMatcher) -> bool {
        debugln!("Validator::fails_arg_required_unless: a={:?};", a.name);
        macro_rules! check {
            ($how:ident, $_self:expr, $a:ident, $m:ident) => {{
                $a.r_unless
                    .as_ref()
                    .map(|ru| !ru.iter().$how(|&n| $m.contains(n)))
                    .unwrap_or(false)
            }};
        }
        if a.is_set(ArgSettings::RequiredUnlessAll) {
            debugln!("Validator::fails_arg_required_unless:{}:All;", a.name);
            check!(all, self.p, a, matcher)
        } else {
            debugln!("Validator::fails_arg_required_unless:{}:Any;", a.name);
            check!(any, self.p, a, matcher)
        }
    }

    // `incl`: an arg to include in the error even if not used
    fn missing_required_error(&self, matcher: &ArgMatcher, incl: Option<Id>) -> ClapResult<()> {
        debugln!("Validator::missing_required_error; incl={:?}", incl);
        let c = Colorizer::new(&ColorizerOption {
            use_stderr: true,
            when: self.p.app.color(),
        });
        debugln!(
            "Validator::missing_required_error: reqs={:?}",
            self.p.required
        );
        let usg = Usage::new(self.p);
        let req_args = if let Some(x) = incl {
            usg.get_required_usage_from(&[x], Some(matcher), true)
                .iter()
                .fold(String::new(), |acc, s| {
                    acc + &format!("\n    {}", c.error(s))[..]
                })
        } else {
            usg.get_required_usage_from(&[], None, true)
                .iter()
                .fold(String::new(), |acc, s| {
                    acc + &format!("\n    {}", c.error(s))[..]
                })
        };
        debugln!(
            "Validator::missing_required_error: req_args={:#?}",
            req_args
        );
        let used: Vec<Id> = matcher
            .arg_names()
            .filter(|&&n| {
                if let Some(a) = self.p.app.find(n) {
                    !(self.p.required.contains(a.id) || a.is_set(ArgSettings::Hidden))
                } else {
                    true
                }
            })
            .cloned()
            .chain(incl)
            .collect();
        Err(Error::missing_required_argument(
            &*req_args,
            &*usg.create_usage_with_title(&*used),
            self.p.app.color(),
        ))
    }
}
