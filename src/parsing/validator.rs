// Internal
use INTERNAL_ERROR_MSG;
use INVALID_UTF8;
use {Arg, ArgSettings};
use AppSettings as AS;
use parsing::ArgMatcher;
use matched::MatchedArg;
use output::errors::ErrorKind;
use output::errors::Error as ClapError;
use output::errors::Result as ClapResult;
use parsing::OsStrExt2;
use parsing::{Parser, ParseResult};
use output::fmt::{Colorizer, ColorizerOption};

impl<'a, 'b, 'c> Parser<'a, 'b, 'c> {
    pub fn validate(
        &mut self,
        needs_val_of: ParseResult<'a>,
        subcmd_name: Option<String>,
        matcher: &mut ArgMatcher<'a>,
    ) -> ClapResult<()> {
        debugln!("Validator::validate;");
        let mut reqs_validated = false;
        try!(self.add_defaults(matcher));
        if let ParseResult::Opt(a) = needs_val_of {
            debugln!("Validator::validate: needs_val_of={:?}", a);
            let o = opts!(self.app).find(|o| o.name == a)
                .expect(INTERNAL_ERROR_MSG);
            try!(self.validate_required(matcher));
            reqs_validated = true;
            let should_err = if let Some(v) = matcher.0.args.iter().find(|&(a, _)| a == &&*o.name) {
                v.1.vals.is_empty() && !(o.min_values.is_some() && o.min_values.unwrap() == 0)
            } else {
                true
            };
            if should_err {
                return Err(ClapError::empty_value(
                    o,
                    &*self.create_error_usage(matcher, None),
                    self.color(),
                ));
            }
        }

        if matcher.is_empty() && matcher.subcommand_name().is_none() &&
            self.is_set(AS::ArgRequiredElseHelp)
        {
            let mut out = vec![];
            try!(self.write_help_err(&mut out));
            return Err(ClapError {
                message: String::from_utf8_lossy(&*out).into_owned(),
                kind: ErrorKind::MissingArgumentOrSubcommand,
                info: None,
            });
        }
        try!(self.validate_conflicts(matcher));
        if !(self.is_set(AS::SubcommandsNegateReqs) && subcmd_name.is_some()) && !reqs_validated {
            try!(self.validate_required(matcher));
        }
        try!(self.validate_matched_args(matcher));
        matcher.usage(self.create_usage_with_title(&[]));

        Ok(())
    }

    fn validate_values(
        &self,
        arg: &Arg,
        ma: &MatchedArg,
        matcher: &ArgMatcher<'a>,
    ) -> ClapResult<()>
    {
        debugln!("Validator::validate_values: arg={:?}", arg.name);
        for val in &ma.vals {
            if self.is_set(AS::StrictUtf8) && val.to_str().is_none() {
                debugln!(
                    "Validator::validate_values: invalid UTF-8 found in val {:?}",
                    val
                );
                return Err(ClapError::invalid_utf8(
                    &*self.create_error_usage(matcher, None),
                    self.color(),
                ));
            }
            if let Some(ref p_vals) = arg.possible_values {
                debugln!("Validator::validate_values: possible_vals={:?}", p_vals);
                let val_str = val.to_string_lossy();
                if !p_vals.contains(&&*val_str) {
                    return Err(ClapError::invalid_value(
                        val_str,
                        &*p_vals,
                        arg,
                        &*self.create_error_usage(matcher, None),
                        self.color(),
                    ));
                }
            }
            if !arg.is_set(ArgSettings::EmptyValues) && val.is_empty_() &&
                matcher.contains(&*arg.name)
            {
                debugln!("Validator::validate_values: illegal empty val found");
                return Err(ClapError::empty_value(
                    arg,
                    &*self.create_error_usage(matcher, None),
                    self.color(),
                ));
            }
            if let Some(ref vtor) = arg.validator {
                debug!("Validator::validate_values: checking validator...");
                if let Err(e) = vtor(val.to_string_lossy().into_owned()) {
                    sdebugln!("error");
                    return Err(ClapError::value_validation(Some(arg), e, self.color()));
                } else {
                    sdebugln!("good");
                }
            }
            if let Some(ref vtor) = arg.validator_os {
                debug!("Validator::validate_values: checking validator_os...");
                if let Err(e) = vtor(val) {
                    sdebugln!("error");
                    return Err(ClapError::value_validation(
                        Some(arg),
                        (*e).to_string_lossy().to_string(),
                        self.color(),
                    ));
                } else {
                    sdebugln!("good");
                }
            }
        }
        Ok(())
    }

    fn validate_conflicts(&self, matcher: &mut ArgMatcher<'a>) -> ClapResult<()> {
        debugln!(
            "Validator::validate_conflicts: conflicts={:?}",
            self.conflicts
        );
        macro_rules! build_err {
            ($parser:expr, $name:expr, $matcher:ident) => ({
                debugln!("build_err!: name={}", $name);
                let c_with = find_matched_that_contains_arg_in!($parser.app, &$name, conflicts_with, &$matcher)
                    .map_or(None, |aa| Some(format!("{}", aa)));
                debugln!("build_err!: '{:?}' conflicts with '{}'", c_with, &$name);
                $matcher.remove(&$name);
                let usg = $parser.create_error_usage($matcher, None);
                if let Some(p) = args!($parser.app).find(|a| a.name == $name) {
                    debugln!("build_err!: It was a positional...");
                    ClapError::argument_conflict(p, c_with, &*usg, self.color())
                } else {
                    panic!(INTERNAL_ERROR_MSG)
                }
            });
        }

        for name in &self.conflicts {
            debugln!(
                "Validator::validate_conflicts:iter: Checking conflictsed name: {}",
                name
            );
            if self.app.groups.iter().any(|g| &g.name == name) {
                debugln!("Validator::validate_conflicts:iter: groups contains it...");
                for n in self.arg_names_in_group(name) {
                    debugln!(
                        "Validator::validate_conflicts:iter:iter: Checking arg '{}' in group...",
                        n
                    );
                    if matcher.contains(n) {
                        debugln!("Validator::validate_conflicts:iter:iter: matcher contains it...");
                        return Err(build_err!(self, n, matcher));
                    }
                }
            } else if matcher.contains(name) {
                debugln!("Validator::validate_conflicts:iter: matcher contains it...");
                return Err(build_err!(self, *name, matcher));
            }
        }
        Ok(())
    }

    fn validate_matched_args(&self, matcher: &mut ArgMatcher<'a>) -> ClapResult<()> {
        debugln!("Validator::validate_matched_args;");
        for (name, ma) in matcher.iter() {
            debugln!(
                "Validator::validate_matched_args:iter:{}: vals={:#?}",
                name,
                ma.vals
            );
            if let Some(arg) = opts!(self.app).find(|a| a.name == *name).or(positionals!(self.app).find(|a| a.name == *name)).or(None) {
                try!(self.validate_arg_num_vals(arg, ma, matcher));
                try!(self.validate_values(arg, ma, matcher));
                try!(self.validate_arg_requires(arg, ma, matcher));
                try!(self.validate_arg_num_occurs(arg, ma, matcher));
            } else if let Some(flag) = flags!(self.app).find(|a| a.name == *name) {
                try!(self.validate_arg_requires(flag, ma, matcher));
                try!(self.validate_arg_num_occurs(flag, ma, matcher));
            } else {
                let grp = self.app
                    .groups
                    .iter()
                    .find(|g| &g.name == name)
                    .expect(INTERNAL_ERROR_MSG);
                if let Some(ref g_reqs) = grp.requires {
                    if g_reqs.iter().any(|&n| !matcher.contains(n)) {
                        return self.missing_required_error(matcher, None);
                    }
                }
            }
        }
        Ok(())
    }

    fn validate_arg_num_occurs(
        &self,
        a: &Arg<'a, 'b>,
        ma: &MatchedArg,
        matcher: &ArgMatcher<'a>,
    ) -> ClapResult<()>
    {
        debugln!("Validator::validate_arg_num_occurs: a={};", a.name());
        if ma.occurs > 1 && !a.is_set(ArgSettings::Multiple) {
            // Not the first time, and we don't allow multiples
            return Err(ClapError::unexpected_multiple_usage(
                a,
                &*self.create_error_usage(matcher, None),
                self.color(),
            ));
        }
        Ok(())
    }

    fn validate_arg_num_vals(
        &self,
        a: &Arg<'a, 'b>,
        ma: &MatchedArg,
        matcher: &ArgMatcher<'a>,
    ) -> ClapResult<()>
    {
        debugln!("Validator::validate_arg_num_vals;");
        if let Some(num) = a.number_of_values {
            debugln!("Validator::validate_arg_num_vals: num_vals set...{}", num);
            let should_err = if a.is_set(ArgSettings::Multiple) {
                (ma.vals.len() % num) != 0
            } else {
                num != ma.vals.len()
            };
            if should_err {
                debugln!("Validator::validate_arg_num_vals: Sending error WrongNumberOfValues");
                return Err(ClapError::wrong_number_of_values(
                    a,
                    num,
                    if a.is_set(ArgSettings::Multiple) {
                        (ma.vals.len() % num)
                    } else {
                        ma.vals.len()
                    },
                    if ma.vals.len() == 1 ||
                        (a.is_set(ArgSettings::Multiple) &&
                             (ma.vals.len() % num) == 1)
                    {
                        "as"
                    } else {
                        "ere"
                    },
                    &*self.create_error_usage(matcher, None),
                    self.color(),
                ));
            }
        }
        if let Some(num) = a.max_values {
            debugln!("Validator::validate_arg_num_vals: max_vals set...{}", num);
            if ma.vals.len() > num {
                debugln!("Validator::validate_arg_num_vals: Sending error TooManyValues");
                return Err(ClapError::too_many_values(
                    ma.vals
                        .iter()
                        .last()
                        .expect(INTERNAL_ERROR_MSG)
                        .to_str()
                        .expect(INVALID_UTF8),
                    a,
                    &*self.create_error_usage(matcher, None),
                    self.color(),
                ));
            }
        }
        if let Some(num) = a.min_values {
            debugln!("Validator::validate_arg_num_vals: min_vals set: {}", num);
            if ma.vals.len() < num {
                debugln!("Validator::validate_arg_num_vals: Sending error TooFewValues");
                return Err(ClapError::too_few_values(
                    a,
                    num,
                    ma.vals.len(),
                    &*self.create_error_usage(matcher, None),
                    self.color(),
                ));
            }
        }
        // Issue 665 (https://github.com/kbknapp/clap-rs/issues/665)
        if a.is_set(ArgSettings::TakesValue) && !a.is_set(ArgSettings::EmptyValues) && ma.vals.is_empty() {
            return Err(ClapError::empty_value(
                a,
                &*self.create_error_usage(matcher, None),
                self.color(),
            ));
        }
        Ok(())
    }

    // @TOOD-v3-alpha: refactor to include requires_ifs
    fn validate_arg_requires(
        &self,
        a: &Arg<'a, 'b>,
        ma: &MatchedArg,
        matcher: &ArgMatcher<'a>,
    ) -> ClapResult<()>
    {
        debugln!("Validator::validate_arg_requires;");
        if let Some(ref a_reqs) = a.requires {
            for name in a_reqs {
                if !matcher.contains(name) {
                    return self.missing_required_error(matcher, None);
                }
            }
        }
        if let Some(ref a_reqs) = a.requires_ifs {
            for &(name, val) in a_reqs {
                let missing_req =
                    |v| v == val && !matcher.contains(name);
                if ma.vals.iter().any(missing_req) {
                    return self.missing_required_error(matcher, None);
                }
            }
        }
        Ok(())
    }

    fn validate_required(&self, matcher: &ArgMatcher<'a>) -> ClapResult<()> {
        debugln!(
            "Validator::validate_required: required={:?};",
            self.required
        );
        'outer: for name in &self.required {
            debugln!("Validator::validate_required:iter:{}:", name);
            if matcher.contains(name) {
                continue 'outer;
            }
            if let Some(a) = args!(self.app).find(|a| a.name == *name) {
                if self.is_missing_required_ok(a, matcher) {
                    continue 'outer;
                }
            }
            return self.missing_required_error(matcher, None);
        }

        // @DESIGN @TODO-v3-alpha: go through all args?
        // Validate the conditionally required args
        for &(a, v, r) in &self.req_ifs {
            if let Some(ma) = matcher.get(a) {
                if matcher.get(r).is_none() && ma.vals.iter().any(|val| val == v) {
                    return self.missing_required_error(matcher, Some(r));
                }
            }
        }
        Ok(())
    }

    fn validate_arg_conflicts(&self, a: &Arg<'a, 'b>, matcher: &ArgMatcher<'a>) -> Option<bool>
    {
        debugln!("Validator::validate_arg_conflicts: a={:?};", a.name);
        a.conflicts_with.as_ref().map(|bl| {
            bl.iter().any(|conf| {
                matcher.contains(conf) ||
                    self.app.groups.iter().find(|g| &g.name == conf).map_or(
                        false,
                        |g| {
                            g.args.iter().any(|arg| matcher.contains(arg))
                        },
                    )
            })
        })
    }

    fn validate_required_unless(&self, a: &Arg, matcher: &ArgMatcher) -> Option<bool>
    {
        debugln!("Validator::validate_required_unless: a={:?};", a.name());
        macro_rules! check {
            ($how:ident, $_self:expr, $a:ident, $m:ident) => {{
                $a.required_unless.as_ref().map(|ru| {
                    ru.iter().$how(|n| {
                        $m.contains(n) || {
                            if let Some(grp) = $_self.app.groups.iter().find(|g| &g.name == n) {
                                     grp.args.iter().any(|arg| $m.contains(arg))
                            } else {
                                false
                            }
                        }
                    })
                })
            }}; 
        }
        if a.is_set(ArgSettings::RequiredUnlessAll) {
            check!(all, self, a, matcher)
        } else {
            check!(any, self, a, matcher)
        }
    }

    fn missing_required_error(&self, matcher: &ArgMatcher<'a>, extra: Option<&'a str>) -> ClapResult<()> {
        debugln!("Validator::missing_required_error: extra={:?}", extra);
        let c = Colorizer::new(ColorizerOption {
            use_stderr: true,
            when: self.color(),
        });
        let mut reqs = self.required.iter().map(|&r| &*r).collect::<Vec<_>>();
        if let Some(r) = extra {
            reqs.push(r);
        }
        reqs.retain(|n| !matcher.contains(n));
        reqs.dedup();
        debugln!("Validator::missing_required_error: reqs={:#?}", reqs);
        let req_args =
            self.get_required_usage_from(&reqs[..], Some(matcher), extra, true)
                .iter()
                .fold(String::new(), |acc, s| {
                    acc + &format!("\n    {}", c.error(s))[..]
                });
        debugln!(
            "Validator::missing_required_error: req_args={:#?}",
            req_args
        );
        Err(ClapError::missing_required_argument(
            &*req_args,
            &*self.create_error_usage(matcher, extra),
            self.color(),
        ))
    }

    #[inline]
    fn is_missing_required_ok(&self, a: &Arg<'a, 'b>, matcher: &ArgMatcher<'a>) -> bool
    {
        debugln!("Validator::is_missing_required_ok: a={}", a.name());
        self.validate_arg_conflicts(a, matcher).unwrap_or(false) ||
            self.validate_required_unless(a, matcher).unwrap_or(false)
    }
}
