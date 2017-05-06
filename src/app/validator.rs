// std
use std::fmt::Display;

// Internal
use INTERNAL_ERROR_MSG;
use INVALID_UTF8;
use args::{AnyArg, ArgMatcher, MatchedArg};
use args::settings::ArgSettings;
use errors::{Error, ErrorKind};
use errors::Result as ClapResult;
use osstringext::OsStrExt2;
use app::settings::AppSettings as AS;
use app::parser::{Parser, ParseResult};
use fmt::Colorizer;
use app::usage;

pub struct Validator<'a, 'b, 'z>(&'z mut Parser<'a, 'b>)
    where 'a: 'b,
          'b: 'z;

impl<'a, 'b, 'z> Validator<'a, 'b, 'z> {
    pub fn new(p: &'z mut Parser<'a, 'b>) -> Self { Validator(p) }

    pub fn validate(&mut self,
                    needs_val_of: ParseResult<'a>,
                    subcmd_name: Option<String>,
                    matcher: &mut ArgMatcher<'a>)
                    -> ClapResult<()> {
        debugln!("Validator::validate;");
        let mut reqs_validated = false;
        try!(self.0.add_defaults(matcher));
        if let ParseResult::Opt(a) = needs_val_of {
            debugln!("Validator::validate: needs_val_of={:?}", a);
            let o = self.0
                .opts
                .iter()
                .find(|o| o.b.name == a)
                .expect(INTERNAL_ERROR_MSG);
            try!(self.validate_required(matcher));
            reqs_validated = true;
            let should_err = if let Some(v) = matcher.0.args.get(&*o.b.name) {
                v.vals.is_empty() && !(o.v.min_vals.is_some() && o.v.min_vals.unwrap() == 0)
            } else {
                true
            };
            if should_err {
                return Err(Error::empty_value(o,
                                              &*usage::create_error_usage(self.0, matcher, None),
                                              self.0.color()));
            }
        }

        if matcher.is_empty() && matcher.subcommand_name().is_none() &&
           self.0.is_set(AS::ArgRequiredElseHelp) {
            let mut out = vec![];
            try!(self.0.write_help_err(&mut out));
            return Err(Error {
                           message: String::from_utf8_lossy(&*out).into_owned(),
                           kind: ErrorKind::MissingArgumentOrSubcommand,
                           info: None,
                       });
        }
        try!(self.validate_blacklist(matcher));
        if !(self.0.is_set(AS::SubcommandsNegateReqs) && subcmd_name.is_some()) && !reqs_validated {
            try!(self.validate_required(matcher));
        }
        try!(self.validate_matched_args(matcher));
        matcher.usage(usage::create_usage_with_title(self.0, &[]));

        Ok(())
    }

    fn validate_values<A>(&self,
                          arg: &A,
                          ma: &MatchedArg,
                          matcher: &ArgMatcher<'a>)
                          -> ClapResult<()>
        where A: AnyArg<'a, 'b> + Display
    {
        debugln!("Validator::validate_values: arg={:?}", arg.name());
        for val in &ma.vals {
            if self.0.is_set(AS::StrictUtf8) && val.to_str().is_none() {
                debugln!("Validator::validate_values: invalid UTF-8 found in val {:?}",
                         val);
                return Err(Error::invalid_utf8(&*usage::create_error_usage(self.0, matcher, None),
                                               self.0.color()));
            }
            if let Some(p_vals) = arg.possible_vals() {
                debugln!("Validator::validate_values: possible_vals={:?}", p_vals);
                let val_str = val.to_string_lossy();
                if !p_vals.contains(&&*val_str) {
                    return Err(Error::invalid_value(val_str,
                                                    p_vals,
                                                    arg,
                                                    &*usage::create_error_usage(self.0,
                                                                                matcher,
                                                                                None),
                                                    self.0.color()));
                }
            }
            if !arg.is_set(ArgSettings::EmptyValues) && val.is_empty_() &&
               matcher.contains(&*arg.name()) {
                debugln!("Validator::validate_values: illegal empty val found");
                return Err(Error::empty_value(arg,
                                              &*usage::create_error_usage(self.0, matcher, None),
                                              self.0.color()));
            }
            if let Some(vtor) = arg.validator() {
                debug!("Validator::validate_values: checking validator...");
                if let Err(e) = vtor(val.to_string_lossy().into_owned()) {
                    sdebugln!("error");
                    return Err(Error::value_validation(Some(arg), e, self.0.color()));
                } else {
                    sdebugln!("good");
                }
            }
            if let Some(vtor) = arg.validator_os() {
                debug!("Validator::validate_values: checking validator_os...");
                if let Err(e) = vtor(val) {
                    sdebugln!("error");
                    return Err(Error::value_validation(Some(arg),
                                                       (*e).to_string_lossy().to_string(),
                                                       self.0.color()));
                } else {
                    sdebugln!("good");
                }
            }
        }
        Ok(())
    }

    fn validate_blacklist(&self, matcher: &mut ArgMatcher) -> ClapResult<()> {
        debugln!("Validator::validate_blacklist: blacklist={:?}",
                 self.0.blacklist);
        macro_rules! build_err {
            ($p:expr, $name:expr, $matcher:ident) => ({
                debugln!("build_err!: name={}", $name);
                let mut c_with = find_from!($p, $name, blacklist, &$matcher);
                c_with = c_with.or(
                    $p.find_any_arg($name).map_or(None, |aa| aa.blacklist())
                                           .map_or(None, 
                                                |bl| bl.iter().find(|arg| $matcher.contains(arg)))
                                           .map_or(None, |an| $p.find_any_arg(an))
                                           .map_or(None, |aa| Some(format!("{}", aa)))
                );
                debugln!("build_err!: '{:?}' conflicts with '{}'", c_with, $name);
                $matcher.remove($name);
                let usg = usage::create_error_usage($p, $matcher, None);
                if let Some(f) = find_by_name!($p, $name, flags, iter) {
                    debugln!("build_err!: It was a flag...");
                    Error::argument_conflict(f, c_with, &*usg, self.0.color())
                } else if let Some(o) = find_by_name!($p, $name, opts, iter) {
                   debugln!("build_err!: It was an option...");
                    Error::argument_conflict(o, c_with, &*usg, self.0.color())
                } else {
                    match find_by_name!($p, $name, positionals, values) {
                        Some(p) => {
                            debugln!("build_err!: It was a positional...");
                            Error::argument_conflict(p, c_with, &*usg, self.0.color())
                        },
                        None    => panic!(INTERNAL_ERROR_MSG)
                    }
                }
            });
        }

        for name in &self.0.blacklist {
            debugln!("Validator::validate_blacklist:iter: Checking blacklisted name: {}",
                     name);
            if self.0.groups.iter().any(|g| &g.name == name) {
                debugln!("Validator::validate_blacklist:iter: groups contains it...");
                for n in self.0.arg_names_in_group(name) {
                    debugln!("Validator::validate_blacklist:iter:iter: Checking arg '{}' in group...",
                             n);
                    if matcher.contains(n) {
                        debugln!("Validator::validate_blacklist:iter:iter: matcher contains it...");
                        return Err(build_err!(self.0, &n, matcher));
                    }
                }
            } else if matcher.contains(name) {
                debugln!("Validator::validate_blacklist:iter: matcher contains it...");
                return Err(build_err!(self.0, name, matcher));
            }
        }
        Ok(())
    }

    fn validate_matched_args(&self, matcher: &mut ArgMatcher<'a>) -> ClapResult<()> {
        debugln!("Validator::validate_matched_args;");
        for (name, ma) in matcher.iter() {
            debugln!("Validator::validate_matched_args:iter:{}: vals={:#?}",
                     name,
                     ma.vals);
            if let Some(opt) = find_by_name!(self.0, name, opts, iter) {
                try!(self.validate_arg_num_vals(opt, ma, matcher));
                try!(self.validate_values(opt, ma, matcher));
                try!(self.validate_arg_requires(opt, ma, matcher));
                try!(self.validate_arg_num_occurs(opt, ma, matcher));
            } else if let Some(flag) = find_by_name!(self.0, name, flags, iter) {
                try!(self.validate_arg_requires(flag, ma, matcher));
                try!(self.validate_arg_num_occurs(flag, ma, matcher));
            } else if let Some(pos) = find_by_name!(self.0, name, positionals, values) {
                try!(self.validate_arg_num_vals(pos, ma, matcher));
                try!(self.validate_arg_num_occurs(pos, ma, matcher));
                try!(self.validate_values(pos, ma, matcher));
                try!(self.validate_arg_requires(pos, ma, matcher));
            } else {
                let grp = self.0
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

    fn validate_arg_num_occurs<A>(&self,
                                  a: &A,
                                  ma: &MatchedArg,
                                  matcher: &ArgMatcher)
                                  -> ClapResult<()>
        where A: AnyArg<'a, 'b> + Display
    {
        debugln!("Validator::validate_arg_num_occurs: a={};", a.name());
        if ma.occurs > 1 && !a.is_set(ArgSettings::Multiple) {
            // Not the first time, and we don't allow multiples
            return Err(Error::unexpected_multiple_usage(a,
                                                        &*usage::create_error_usage(self.0,
                                                                                    matcher,
                                                                                    None),
                                                        self.0.color()));
        }
        Ok(())
    }

    fn validate_arg_num_vals<A>(&self,
                                a: &A,
                                ma: &MatchedArg,
                                matcher: &ArgMatcher)
                                -> ClapResult<()>
        where A: AnyArg<'a, 'b> + Display
    {
        debugln!("Validator::validate_arg_num_vals;");
        if let Some(num) = a.num_vals() {
            debugln!("Validator::validate_arg_num_vals: num_vals set...{}", num);
            let should_err = if a.is_set(ArgSettings::Multiple) {
                ((ma.vals.len() as u64) % num) != 0
            } else {
                num != (ma.vals.len() as u64)
            };
            if should_err {
                debugln!("Validator::validate_arg_num_vals: Sending error WrongNumberOfValues");
                return Err(Error::wrong_number_of_values(a,
                                                         num,
                                                         if a.is_set(ArgSettings::Multiple) {
                                                             (ma.vals.len() % num as usize)
                                                         } else {
                                                             ma.vals.len()
                                                         },
                                                         if ma.vals.len() == 1 ||
                                                            (a.is_set(ArgSettings::Multiple) &&
                                                             (ma.vals.len() % num as usize) ==
                                                             1) {
                                                             "as"
                                                         } else {
                                                             "ere"
                                                         },
                                                         &*usage::create_error_usage(self.0,
                                                                                     matcher,
                                                                                     None),
                                                         self.0.color()));
            }
        }
        if let Some(num) = a.max_vals() {
            debugln!("Validator::validate_arg_num_vals: max_vals set...{}", num);
            if (ma.vals.len() as u64) > num {
                debugln!("Validator::validate_arg_num_vals: Sending error TooManyValues");
                return Err(Error::too_many_values(ma.vals
                                                      .iter()
                                                      .last()
                                                      .expect(INTERNAL_ERROR_MSG)
                                                      .to_str()
                                                      .expect(INVALID_UTF8),
                                                  a,
                                                  &*usage::create_error_usage(self.0,
                                                                              matcher,
                                                                              None),
                                                  self.0.color()));
            }
        }
        if let Some(num) = a.min_vals() {
            debugln!("Validator::validate_arg_num_vals: min_vals set: {}", num);
            if (ma.vals.len() as u64) < num {
                debugln!("Validator::validate_arg_num_vals: Sending error TooFewValues");
                return Err(Error::too_few_values(a,
                                                 num,
                                                 ma.vals.len(),
                                                 &*usage::create_error_usage(self.0,
                                                                             matcher,
                                                                             None),
                                                 self.0.color()));
            }
        }
        // Issue 665 (https://github.com/kbknapp/clap-rs/issues/665)
        if a.takes_value() && !a.is_set(ArgSettings::EmptyValues) && ma.vals.is_empty() {
            return Err(Error::empty_value(a,
                                          &*usage::create_error_usage(self.0, matcher, None),
                                          self.0.color()));
        }
        Ok(())
    }

    fn validate_arg_requires<A>(&self,
                                a: &A,
                                ma: &MatchedArg,
                                matcher: &ArgMatcher)
                                -> ClapResult<()>
        where A: AnyArg<'a, 'b> + Display
    {
        debugln!("Validator::validate_arg_requires;");
        if let Some(a_reqs) = a.requires() {
            for &(val, name) in a_reqs.iter().filter(|&&(val, _)| val.is_some()) {
                let missing_req =
                    |v| v == val.expect(INTERNAL_ERROR_MSG) && !matcher.contains(name);
                if ma.vals.iter().any(missing_req) {
                    return self.missing_required_error(matcher, None);
                }
            }
        }
        Ok(())
    }

    fn validate_required(&self, matcher: &ArgMatcher) -> ClapResult<()> {
        debugln!("Validator::validate_required: required={:?};",
                 self.0.required);
        'outer: for name in &self.0.required {
            debugln!("Validator::validate_required:iter:{}:", name);
            if matcher.contains(name) {
                continue 'outer;
            }
            if let Some(a) = find_by_name!(self.0, name, flags, iter) {
                if self.is_missing_required_ok(a, matcher) {
                    continue 'outer;
                }
            } else if let Some(a) = find_by_name!(self.0, name, opts, iter) {
                if self.is_missing_required_ok(a, matcher) {
                    continue 'outer;
                }
            } else if let Some(a) = find_by_name!(self.0, name, positionals, values) {
                if self.is_missing_required_ok(a, matcher) {
                    continue 'outer;
                }
            }
            return self.missing_required_error(matcher, None);
        }

        // Validate the conditionally required args
        for &(a, v, r) in &self.0.r_ifs {
            if let Some(ma) = matcher.get(a) {
                if matcher.get(r).is_none() && ma.vals.iter().any(|val| val == v) {
                    return self.missing_required_error(matcher, Some(r));
                }
            }
        }
        Ok(())
    }

    fn validate_conflicts<A>(&self, a: &A, matcher: &ArgMatcher) -> Option<bool>
        where A: AnyArg<'a, 'b>
    {
        debugln!("Validator::validate_conflicts: a={:?};", a.name());
        a.blacklist()
            .map(|bl| {
                bl.iter()
                    .any(|conf| {
                        matcher.contains(conf) ||
                        self.0
                            .groups
                            .iter()
                            .find(|g| &g.name == conf)
                            .map_or(false, |g| g.args.iter().any(|arg| matcher.contains(arg)))
                    })
            })
    }

    fn validate_required_unless<A>(&self, a: &A, matcher: &ArgMatcher) -> Option<bool>
        where A: AnyArg<'a, 'b>
    {
        debugln!("Validator::validate_required_unless: a={:?};", a.name());
        macro_rules! check {
            ($how:ident, $_self:expr, $a:ident, $m:ident) => {{
                $a.required_unless().map(|ru| {
                    ru.iter().$how(|n| {
                        $m.contains(n) || {
                            if let Some(grp) = $_self.groups.iter().find(|g| &g.name == n) {
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
            check!(all, self.0, a, matcher)
        } else {
            check!(any, self.0, a, matcher)
        }
    }

    fn missing_required_error(&self, matcher: &ArgMatcher, extra: Option<&str>) -> ClapResult<()> {
        debugln!("Validator::missing_required_error: extra={:?}", extra);
        let c = Colorizer {
            use_stderr: true,
            when: self.0.color(),
        };
        let mut reqs = self.0
            .required
            .iter()
            .map(|&r| &*r)
            .collect::<Vec<_>>();
        if let Some(r) = extra {
            reqs.push(r);
        }
        reqs.retain(|n| !matcher.contains(n));
        reqs.dedup();
        debugln!("Validator::missing_required_error: reqs={:#?}", reqs);
        let req_args =
            usage::get_required_usage_from(self.0, &reqs[..], Some(matcher), extra, true)
                .iter()
                .fold(String::new(),
                      |acc, s| acc + &format!("\n    {}", c.error(s))[..]);
        debugln!("Validator::missing_required_error: req_args={:#?}",
                 req_args);
        Err(Error::missing_required_argument(&*req_args,
                                             &*usage::create_error_usage(self.0, matcher, extra),
                                             self.0.color()))
    }

    #[inline]
    fn is_missing_required_ok<A>(&self, a: &A, matcher: &ArgMatcher) -> bool
        where A: AnyArg<'a, 'b>
    {
        debugln!("Validator::is_missing_required_ok: a={}", a.name());
        self.validate_conflicts(a, matcher).unwrap_or(false) ||
        self.validate_required_unless(a, matcher)
            .unwrap_or(false)
    }
}