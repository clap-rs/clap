// Std
#[cfg(all(feature = "debug", any(target_os = "windows", target_arch = "wasm32")))]
use osstringext::OsStrExt3;
use std::cell::Cell;
use std::ffi::{OsStr, OsString};
use std::io::{self, BufWriter, Write};
use std::iter::Peekable;
use std::mem;
#[cfg(all(
    feature = "debug",
    not(any(target_os = "windows", target_arch = "wasm32"))
))]
use std::os::unix::ffi::OsStrExt;

// Internal
use crate::build::app::Propagation;
use crate::build::AppSettings as AS;
use crate::build::{App, Arg, ArgSettings};
use crate::output::{usage, Help};
use crate::parse::errors::Error as ClapError;
use crate::parse::errors::ErrorKind;
use crate::parse::errors::Result as ClapResult;
use crate::parse::features::suggestions;
use crate::parse::Validator;
use crate::parse::{KeyType, HyphenStyle, RawValue, RawArg, RawLong, RawOpt, SeenArg, ArgMatcher, SubCommand, ArgPrediction, ValueState};
use crate::util::{hash, OsStrExt2};
use crate::INTERNAL_ERROR_MSG;
use crate::INVALID_UTF8;

const HELP_HASH: u64 = hash("help");

#[derive(Debug, PartialEq, Copy, Clone)]
#[doc(hidden)]
pub enum ParseCtx<'help> {
    Initial,
    ArgAcceptsVals(u64),
    PosByIndexAcceptsVals(usize),
    SubCmd(u64),
    NextArg,
    MaybeNegNum,
    MaybeHyphenValue,
    UnknownShort,
    UnknownPositional,
    TrailingValues,
    LowIndexMultsOrMissingPos,
}

impl Default for ParseCtx {
    fn default() -> Self {
        ParseCtx::Initial
    }
}


#[derive(Default)]
#[doc(hidden)]
pub struct Parser<'help> {
    seen: Vec<SeenArg>,
    cur_idx: Cell<usize>,
    num_pos: usize,
    cur_pos: usize,
    cache: u64,
    trailing_vals: bool,
}

// Parsing Methods
impl<'help> Parser<'help> {
    // The actual parsing function
    fn parse<I, T>(
        &mut self,
        it: &mut Peekable<I>,
        app: &mut App,
    ) -> ClapResult<()>
    where
        I: Iterator<Item = T>,
        T: AsRef<OsStr>,
    {
        debugln!("Parser::get_matches_with;");
        let mut matcher = ArgMatcher::new();
        let mut ctx = ParseCtx::Initial;

        'outer: for arg in it {
            debugln!("Parser::get_matches_with: Begin parsing '{:?}'", arg_os);
            let arg_os = arg.as_ref();
            let raw = arg_os.into();

            // First make sure this isn't coming after a `--` only
            ctx = if self.trailing_vals {
                self.handle_low_index_multiples(&mut ctx, is_second_to_last)
            } else {
                self.try_parse_arg(&mut matcher, ctx, &raw)?
            };

            'inner: loop {
                match ctx {
                    ParseCtx::LowIndexMultsOrMissingPos => {
                        if it.peek().is_some() && (self.cur_pos < self.num_pos) {
                            ctx = ParseCtx::PosAcceptsVal;
                        } else {
                            continue 'outer;
                        }
                    },
                    ParseCtx::PosAcceptsVals => {
                        if let Some(p) = app.args.get_by_index(self.cur_pos) {
                            ctx = self.parse_positional(app, p, &mut matcher, raw.into())?;
                            // Only increment the positional counter if it doesn't allow multiples
                            if ctx == ParseCtx::NextArg {
                                self.cur_pos += 1;
                            }
                        } else {
                            // Unknown Positional Argument
                            ctx = ParseCtx::ExternalSubCmd;
                        }
                    }
                    ParseCtx::ExternalSubCmd => {
                        if !self.is_set(AS::AllowExternalSubcommands) {
                            return Err(self.find_unknown_arg_error(app, raw.0));
                        }
                        // Get external subcommand name
                        // @TODO @perf @p3 probably don't need to convert to a String anymore
                        // unless checking strict UTF-8
                        let sc_name = match raw.0.to_str() {
                            Some(s) => s.to_string(),
                            None => {
                                if !self.is_set(AS::StrictUtf8) {
                                    ctx = ParseCtx::InvalidUtf8;
                                    continue;
                                }
                                arg_os.to_string_lossy().into_owned()
                            }
                        };

                        // Collect the external subcommand args
                        let mut sc_m = ArgMatcher::new();
                        while let Some(v) = it.next() {
                            let a = v.into();
                            if a.to_str().is_none() && !self.is_set(AS::StrictUtf8) {
                                ctx = ParseCtx::InvalidUtf8;
                            }
                            sc_m.add_val_to(0, &a);
                        }

                        matcher.subcommand(SubCommand {
                            id: hash(&*sc_name),
                            matches: sc_m.into(),
                        });
                        break 'outer;
                    },
                    ParseCtx::UnknownLong => {
                        if self.is_set(AS::AllowLeadingHyphen) {
                            ctx = ParseCtx::MaybeHyphenValue;
                        } else {
                            return self.did_you_mean_error(raw.0.to_str().expect(INVALID_UTF8), &mut matcher);
                        }
                    }
                    ParseCtx::UnknownShort => {
                        if self.is_set(AS::AllowLeadingHyphen) {
                            ctx = ParseCtx::MaybeHyphenValue;
                            continue;
                        } else if self.is_set(AS::AllowNegativeNumbers) {
                            ctx = ParseCtx::MaybeNegNum;
                            continue;
                        } else {
                            ctx = ParseCtx::UnknownArgError;
                        }
                    },
                    ParseCtx::MaybeNegNum => {
                        if raw.0.to_string_lossy().parse::<i64>().is_ok()
                            || raw.0.to_string_lossy().parse::<f64>().is_ok() {
                            ctx = ParseCtx::MaybeHyphenValue;
                        }
                        ctx = ParseCtx::UnknownArgError;
                    },
                    ParseCtx::MaybeHyphenValue => {
                        if app.find(self.cache).map(|x| x.accepts_value()).unwrap_or(false) {
                            ctx = ParseCtx::ArgAcceptsVals(self.cache);
                            continue;
                        }
                        ctx = ParseCtx::UnknownArgError;
                    },
                    ParseCtx::ArgAcceptsVals(id) => {
                        let opt = app.args.get_by_id(id).unwrap();
                        ctx = self.add_val_to_arg(opt, raw.0.into(), &mut matcher)?;
                        if let ParseCtx::ArgAcceptsVals(id) = ctx {
                            continue 'outer;
                        }
                    },
                    ParseCtx::SubCmd(id) => {
                        if id == HELP_HASH && !self.is_set(AS::NoAutoHelp) {
                            self.parse_help_subcommand(it)?;
                        }
                        break;
                    },
                    ParseCtx::UknownArgError => {
                        return Err(ClapError::unknown_argument(
                            &*raw.0.to_string_lossy(),
                            "",
                            &*usage::with_title(app, &[]),
                            app.color(),
                        ));
                    },
                    ParseCtx::InvalidUtf8 => {
                        return Err(ClapError::invalid_utf8(
                            &*Usage::new(app).create_usage_with_title(&[]),
                            app.color(),
                        ));
                    }
                }
            }

            self.maybe_misspelled_subcmd(app, raw.0)?;
        }

        self.check_subcommand(it, app, &mut matcher, pos_sc)?;

        let overridden = matcher.remove_overrides(self, &*self.seen);

        Validator::new(self, overridden).validate(&subcmd_name, &mut matcher)
    }

    fn check_subcommand(&mut self, it: &mut Peekable<I>, app: &mut App, mut matcher: &mut ArgMatcher, subcmd_name: Option<u64>) -> Clapresult<()> {
        if let Some(pos_sc_name) = subcmd_name {
            let sc= app.subcommands
                .iter_mut()
                .find(|x| x.id == *pos_sc_name)
                .expect(INTERNAL_ERROR_MSG);
            self.parse_subcommand(sc, matcher, it)?;
        } else if self.is_set(AS::SubcommandRequired) {
            let bn = app.bin_name.as_ref().unwrap_or(&app.name.into());
            return Err(ClapError::missing_subcommand(
                bn,
                &Usage::new(self).create_usage_with_title(&[]),
                app.color(),
            ));
        } else if self.is_set(AS::SubcommandRequiredElseHelp) {
            debugln!("Parser::get_matches_with: SubcommandRequiredElseHelp=true");
            let mut out = vec![];
            self.write_help_err(&mut out)?;
            return Err(ClapError {
                message: String::from_utf8_lossy(&*out).into_owned(),
                kind: ErrorKind::MissingArgumentOrSubcommand,
                info: None,
            });
        }

        Ok(())
    }

    fn handle_low_index_multiples(&mut self, ctx: &mut ParseCtx, is_second_to_last: bool) -> ParseCtx {
        let is_second_to_last = self.pos_counter == (self.num_pos - 1);

        let low_index_mults = self.is_set(AS::LowIndexMultiplePositional)
            && is_second_to_last;
        let missing_pos = self.is_set(AS::AllowMissingPositional)
            && is_second_to_last
            && !self.is_set(AS::TrailingValues);

        if low_index_mults || missing_pos {
            ParseCtx::LowIndexMultsOrMissingPos
        } else if self.skip_to_last_positional() {
            // Came to -- and one postional has .last(true) set, so we go immediately
            // to the last (highest index) positional
            debugln!("Parser::parse: .last(true) and --, setting last pos");
            self.cur_pos = self.num_pos;
        }
        ParseCtx::PosAcceptsVal
    }

    fn try_parse_arg(&mut self, mut matcher: &mut ArgMatcher, mut ctx: ParseCtx, raw: &RawArg) -> ClapResult<ParseCtx> {
        match raw.make_prediction(ctx) {
            ArgPrediction::ShortKey(raw) => {
                self.parse_short(&mut matcher, raw.into())
            },
            ArgPrediction::LongKey(raw) => {
                self.parse_long(&mut matcher, raw.into())
            },
            ArgPrediction::PossibleValue(raw) => {
                self.possible_subcommand(&raw)
            },
            ArgPrediction::TrailingValueSignal => {
                self.trailing_vals = true;
                Ok(ParseCtx::NextArg)
            },
            ArgPrediction::Value => {unimplemented!()},
        }
    }

    fn find_unknown_arg_error(&self, app: &App, raw: &OsStr) -> ClapError {
        if !((app.is_set(AS::AllowLeadingHyphen) || app.is_set(AS::AllowNegativeNumbers)) && raw.starts_with(b"-")) && !self.is_set(AS::InferSubcommands) {
            return ClapError::unknown_argument(
                &*raw.to_string_lossy(),
                "",
                &*usage::with_title(app, &[]),
                app.color(),
            );
        } else if !app.has_args() || self.is_set(AS::InferSubcommands) && app.has_subcommands() {
            if let Some(cdate) = suggestions::did_you_mean(&*raw.to_string_lossy(), sc_names!(app)) {
                return ClapError::invalid_subcommand(
                    raw.to_string_lossy().into_owned(),
                    cdate,
                    app.bin_name.as_ref().unwrap_or(&app.name.into()),
                    &*usage::with_title(app, &[]),
                    app.color(),
                );
            } else {
                return ClapError::unrecognized_subcommand(
                    raw.to_string_lossy().into_owned(),
                    app.bin_name.as_ref().unwrap_or(&app.name.into()),
                    app.color(),
                );
            }
        } else {
            return ClapError::unknown_argument(
                &*raw.to_string_lossy(),
                "",
                &*usage::with_title(app, &[]),
                app.color(),
            );
        }
    }

    fn maybe_misspelled_subcmd(&self, app: &App, arg_os: &OsStr) -> ClapResult<()> {
        // @TODO @p2 Use Flags directly for BitOr instead of this...
        if !(app.is_set(AS::ArgsNegateSubcommands) && app.is_set(AS::ValidArgFound)
            || app.is_set(AS::AllowExternalSubcommands)
            || app.is_set(AS::InferSubcommands))
            {
                if let Some(cdate) =
                suggestions::did_you_mean(arg_os.to_string_lossy(), sc_names!(app))
                    {
                        return Err(ClapError::invalid_subcommand(
                            arg_os.to_string_lossy().into_owned(),
                            cdate,
                            app.bin_name.as_ref().unwrap_or(&app.name.into()),
                            &*usage::with_title(app, &[]),
                            app.color(),
                        ));
                    }
            }

        Ok(())
    }

    fn parse_positional(&mut self, app: &mut App, p: &Arg, matcher: &mut ArgMatcher, raw: RawValue) -> ClapResult<ParseCtx> {
        let no_trailing_vals = !self.is_set(AS::TrailingValues);
        if p.is_set(ArgSettings::Last) && no_trailing_vals {
            return Err(ClapError::unknown_argument(
                &*raw.raw.to_string_lossy(),
                "",
                &*usage::with_title(app, &[]),
                self.app.color(),
            ));
        }

        if no_trailing_vals && (self.is_set(AS::TrailingVarArg) && self.cur_pos == self.cur_pos) {
            self.app.settings.set(AS::TrailingValues);
        }

        self.seen.push(SeenArg { id: p.id, key: KeyType::Index});
        let ret = self.add_val_to_arg(p, &raw.raw.into(), matcher)?;

        matcher.inc_occurrence_of(p.name);
        for grp in groups_for_arg!(self.app, &p.name) {
            matcher.inc_occurrence_of(&*grp);
        }

        self.app.settings.set(AS::ValidArgFound);
        Ok(ret)
    }

    // Checks if the arg matches a subcommand name, or any of it's aliases (if defined)
    fn possible_subcommand(&self, raw: &RawArg) -> ParseCtx {
        debugln!("Parser::possible_subcommand: arg={:?}", raw);
        fn starts(h: &str, n: &OsStr) -> bool {
            #[cfg(target_os = "windows")]
            use osstringext::OsStrExt3;
            #[cfg(not(target_os = "windows"))]
            use std::os::unix::ffi::OsStrExt;

            let n_bytes = n.as_bytes();
            let h_bytes = OsStr::new(h).as_bytes();

            h_bytes.starts_with(n_bytes)
        }

        if self.is_set(AS::ArgsNegateSubcommands) && self.is_set(AS::ValidArgFound) {
            return None;
        }
        if !self.is_set(AS::InferSubcommands) {
            let pos_id = hash(raw.0);
            if let Some(sc) = self.app.subcommands.iter().find(|x| x.id == pos_id) {
                return ParseCtx::SubCmd(&sc.id);
            }
        } else {
            let v = sc_names!(self.app)
                .filter(|s| starts(s, &*raw.0))
                .collect::<Vec<_>>();

            if v.len() == 1 {
                return ParseResult::SubCmd(hash(v[0]));
            }
        }
        ParseResult::PosByIndex(self.cur_pos)
    }

    fn parse_subcommand<I, T>(
        &mut self,
        s: &mut App,
        matcher: &mut ArgMatcher,
        it: &mut Peekable<I>,
    ) -> ClapResult<()>
    where
        I: Iterator<Item = T>,
        T: Into<OsString> + Clone,
    {
        use std::fmt::Write;
        debugln!("Parser::parse_subcommand;");

        // Ensure all args are built and ready to parse
        sc._build(Propagation::NextLevel);

        debugln!("Parser::parse_subcommand: About to parse sc={}", sc.name);

        let mut p = Parser::new(sc);
        let mut sc_matcher = ArgMatcher::new();
        p.get_matches_with(&mut sc_matcher, it)?;
        matcher.subcommand(SubCommand {
            id: sc.id,
            matches: sc_matcher.into(),
        });
        Ok(())
    }

    // Retrieves the names of all args the user has supplied thus far, except required ones
    // because those will be listed in self.required
    fn check_for_help_and_version_long(&self, arg: &OsStr) -> ClapResult<()> {
        debugln!("Parser::check_for_help_and_version_long;");

        // Needs to use app.settings.is_set instead of just is_set() because is_set() checks
        // both global and local settings, we only want to check local
        if arg == "help" && !self.app.settings.is_set(AS::NoAutoHelp) {
            return Err(self.help_err(true));
        }
        if arg == "version" && !self.app.settings.is_set(AS::NoAutoVersion) {
            return Err(self.version_err(true));
        }

        Ok(())
    }

    fn check_for_help_and_version_char(&self, arg: char) -> ClapResult<()> {
        debugln!("Parser::check_for_help_and_version_char;");
        // Needs to use app.settings.is_set instead of just is_set() because is_set() checks
        // both global and local settings, we only want to check local
        if let Some(help) = self.app.find(hash("help")) {
            if let Some(h) = help.short {
                if arg == h && !self.app.settings.is_set(AS::NoAutoHelp) {
                    return Err(self.help_err(false));
                }
            }
        }
        if let Some(version) = self.app.find(hash("version")) {
            if let Some(v) = version.short {
                if arg == v && !self.app.settings.is_set(AS::NoAutoVersion) {
                    return Err(self.version_err(false));
                }
            }
        }
        Ok(())
    }

    fn parse_long(
        &mut self,
        matcher: &mut ArgMatcher,
        raw_long: RawLong,
    ) -> ClapResult<ParseCtx> {
        // maybe here lifetime should be 'a
        debugln!("Parser::parse_long;");

        // Update the curent index
        self.cur_idx.set(self.cur_idx.get() + 1);

        if let Some(arg) = self.app.args.get_by_long_with_hyphen(raw_long.key_as_bytes()) {
            self.app.settings.set(AS::ValidArgFound);

            self.seen.push(SeenArg::new(arg.id, KeyType::Long));

            if arg.is_set(ArgSettings::TakesValue) {
                return self.parse_opt(app, raw_long.into(), arg, matcher);
            }

            // Check for help/version *after* opt so we avoid needlessly checking every time
            self.check_for_help_and_version_long(raw_long.key())?;
            self.parse_flag(arg.id, matcher)?;

            return Ok(ParseResult::NextArg);
        }

        Ok(ParseResult::MaybeHyphenValue)
    }

    fn parse_short(
        &mut self,
        matcher: &mut ArgMatcher,
        full_arg: RawArg,
    ) -> ClapResult<ParseCtx> {
        debugln!("Parser::parse_short_arg: full_arg={:?}", full_arg);
        let arg_os = full_arg.trim_left_matches(b'-');
        let arg = arg_os.to_string_lossy();

        let mut ret = ParseResult::UnkownShort;
        for c in arg.chars() {
            debugln!("Parser::parse_short_arg:iter:{}", c);

            // update each index because `-abcd` is four indices to clap
            self.cur_idx.set(self.cur_idx.get() + 1);

            // Check for matching short options, and return the name if there is no trailing
            // concatenated value: -oval
            // Option: -o
            // Value: val
            if let Some(arg) = self.app.args.get_by_short(c) {
                debugln!( "Parser::parse_short_arg:iter:{}: Found valid opt or flag", c );
                self.app.settings.set(AS::ValidArgFound);
                self.seen.push(arg.name);
                if !arg.is_set(ArgSettings::TakesValue) {
                    self.check_for_help_and_version_char(c)?;
                    ret = self.parse_flag(arg.id, matcher)?;
                    continue;
                }

                // Check for trailing concatenated value such as -oval where 'o' is the short and
                // 'val' is the value
                let p: Vec<_> = arg.splitn(2, c).collect();
                let ro = RawOpt {
                    raw_key: p[0],
                    key: KeyType::Short,
                    value: RawValue::from_maybe_empty_osstr(p[1]),
                };
                return self.parse_opt(app, ro, arg, matcher);
            } else {
                return Ok(ParseResult::UnknownShort);
            }
        }
        Ok(ret)
    }

    fn parse_opt(
        &self,
        app: &mut App,
        raw: RawOpt,
        opt: &Arg<'help>,
        matcher: &mut ArgMatcher,
    ) -> ClapResult<ParseCtx> {
        debugln!("Parser::parse_opt; opt={}, val={:?}", opt.id, raw.value);
        let had_eq = raw.had_eq();
        let mut ret = ParseResult::Initial; // @TODO: valid args found state?

        if raw.has_value() {
            ret = self.add_val_to_arg(opt, raw.value_unchecked(), matcher)?;
        } else if opt.is_set(ArgSettings::RequireEquals) {
            return Err(ClapError::empty_value(
                opt,
                &*usage::with_title(app, &[]),
                app.color(),
            ));
        }

        matcher.inc_occurrence_of(opt.id);
        // Increment or create the group
        for grp in groups_for_arg!(self.app, &opt.id) {
            matcher.inc_occurrence_of(&*grp);
        }

        Ok(ParseResult::NextArg)
    }

    fn add_val_to_arg(
        &self,
        arg: &Arg<'help>,
        mut raw: RawValue,
        matcher: &mut ArgMatcher,
    ) -> ClapResult<ParseCtx> {
        debugln!("Parser::add_val_to_arg; arg={}, val={:?}", arg.id, val);
        let honor_delims = !(self.is_set(AS::TrailingValues)
            && self.is_set(AS::DontDelimitTrailingValues));
        if honor_delims {
            raw.sep = arg.val_delim;
        }
        let mut ret = ParseResult::Initial; // @TODO: valid args found state?
        for v in raw.values() {
            ret = self.add_single_val_to_arg(arg, v, matcher)?;
        }
        // If there was a delimiter used, we're not looking for more values because
        // --foo=bar,baz qux isn't three values. Same with --foo bar,baz qux
        if honor_delims && raw.used_sep() { //|| arg.is_set(ArgSettings::RequireDelimiter)) {
            ret = ParseResult::NextArg;
        } else {
            ret = match matcher.value_state_after_val(arg) {
                ValueState::Done => {unimplemented!()}
                ValueState::RequiresValue(id) => {unimplemented!()}
                ValueState::AcceptsValue(id) => {unimplemented!()}
            };
        }
        Ok(ret)
    }

    fn add_single_val_to_arg(
        &self,
        arg: &Arg<'help>,
        v: &OsStr,
        matcher: &mut ArgMatcher,
    ) -> ClapResult<ParseCtx> {
        debugln!("Parser::add_single_val_to_arg: adding val...{:?}", v);

        // update the current index because each value is a distinct index to clap
        self.cur_idx.set(self.cur_idx.get() + 1);

        // @TODO @docs @p4 docs should probably note that terminator doesn't get an index
        if let Some(t) = arg.terminator {
            if t == v {
                return Ok(ParseResult::NextArg); // @TODO maybe add, ValueDone state?
            }
        }

        matcher.add_val_to(arg.id, v);
        matcher.add_index_to(arg.id, self.cur_idx.get());

        // Increment or create the group "args"
        for grp in groups_for_arg!(self.app, &arg.id) {
            matcher.add_val_to(&*grp, v);
        }

        if matcher.needs_more_vals(arg) {
            return Ok(ParseResult::ArgAcceptsVals(arg.id));
        }
        Ok(ParseResult::NextArg)
    }

    fn parse_flag(
        &self,
        flag_id: u64,
        matcher: &mut ArgMatcher,
    ) -> ClapResult<ParseCtx> {
        debugln!("Parser::parse_flag;");

        matcher.inc_occurrence_of(flag_id);
        matcher.add_index_to(flag_id, self.cur_idx.get());
        // Increment or create the group "args"
        for grp in groups_for_arg!(self.app, &flag_id) {
            matcher.inc_occurrence_of(grp);
        }

        Ok(ParseResult::NextArg)
    }


    pub(crate) fn add_defaults(&mut self, matcher: &mut ArgMatcher) -> ClapResult<()> {
        debugln!("Parser::add_defaults;");
        macro_rules! add_val {
            (@default $_self:ident, $a:ident, $m:ident) => {
                if let Some(ref val) = $a.default_val {
                    debugln!("Parser::add_defaults:iter:{}: has default vals", $a.name);
                    if $m
                        .get($a.name)
                        .map(|ma| ma.vals.len())
                        .map(|len| len == 0)
                        .unwrap_or(false)
                    {
                        debugln!(
                            "Parser::add_defaults:iter:{}: has no user defined vals",
                            $a.name
                        );
                        $_self.add_val_to_arg($a, OsStr::new(val), $m)?;
                    } else if $m.get($a.name).is_some() {
                        debugln!(
                            "Parser::add_defaults:iter:{}: has user defined vals",
                            $a.name
                        );
                    } else {
                        debugln!("Parser::add_defaults:iter:{}: wasn't used", $a.name);

                        $_self.add_val_to_arg($a, OsStr::new(val), $m)?;
                    }
                } else {
                    debugln!(
                        "Parser::add_defaults:iter:{}: doesn't have default vals",
                        $a.name
                    );
                }
            };
            ($_self:ident, $a:ident, $m:ident) => {
                if let Some(ref vm) = $a.default_vals_ifs {
                    sdebugln!(" has conditional defaults");
                    let mut done = false;
                    if $m.get($a.name).is_none() {
                        for &(arg, val, default) in vm.values() {
                            let add = if let Some(a) = $m.get(arg) {
                                if let Some(v) = val {
                                    a.vals.iter().any(|value| v == value)
                                } else {
                                    true
                                }
                            } else {
                                false
                            };
                            if add {
                                $_self.add_val_to_arg($a, OsStr::new(default), $m)?;
                                done = true;
                                break;
                            }
                        }
                    }

                    if done {
                        continue; // outer loop (outside macro)
                    }
                } else {
                    sdebugln!(" doesn't have conditional defaults");
                }
                add_val!(@default $_self, $a, $m)
            };
        }

        for o in self.app.args.opts() {
            debug!("Parser::add_defaults:iter:{}:", o.name);
            add_val!(self, o, matcher);
        }
        for p in self.app.args.positionals() {
            debug!("Parser::add_defaults:iter:{}:", p.name);
            add_val!(self, p, matcher);
        }
        Ok(())
    }

    pub(crate) fn add_env(&mut self, matcher: &mut ArgMatcher) -> ClapResult<()> {
        for a in self.app.args.args.iter() {
            if let Some(ref val) = a.env {
                if let Some(ref val) = val.1 {
                    self.add_val_to_arg(a, OsStr::new(val).into(), matcher)?;
                }
            }
        }
        Ok(())
    }
}

// Error, Help, and Version Methods
impl Parser {
    fn did_you_mean_error(&mut self, arg: &str, matcher: &mut ArgMatcher) -> ClapResult<()> {
        debugln!("Parser::did_you_mean_error: arg={}", arg);

        // Get all longs
        let longs = self
            .app
            .args
            .args
            .iter()
            .filter_map(|x| x.long)
            .collect::<Vec<_>>();
        debugln!("Parser::did_you_mean_error: longs={:?}", longs);

        let suffix = suggestions::did_you_mean_flag_suffix(
            arg,
            longs.iter().map(|ref x| &x[..]),
            self.app.subcommands.as_mut_slice(),
        );

        // Add the arg to the matches to build a proper usage string
        if let Some(ref name) = suffix.1 {
            if let Some(opt) = self.app.args.get_by_long(&*name) {
                for g in groups_for_arg!(self.app, &opt.id) {
                    matcher.inc_occurrence_of(g);
                }
                matcher.insert(opt.id);
            }
        }

        let used: Vec<u64> = matcher
            .arg_names()
            .filter(|n| {
                if let Some(a) = self.app.find(**n) {
                    !(self.required.contains(a.name) || a.is_set(ArgSettings::Hidden))
                } else {
                    true
                }
            })
            .cloned()
            .collect();

        Err(ClapError::unknown_argument(
            &*format!("--{}", arg),
            &*suffix.0,
            &*usage::with_title(app, &*used),
            app.color(),
        ))
    }

}

// Query Methods
impl Parser {
    #[inline]
    fn skip_to_last_positional(&self) -> bool {
        self.is_set(AS::TrailingValues) && (self.is_set(AS::AllowMissingPositional) || self.is_set(AS::ContainsLast))
    }

    fn contains_short(&self, s: char) -> bool { self.app.contains_short(s) }

    #[cfg_attr(feature = "lints", allow(needless_borrow))]
    pub(crate) fn has_args(&self) -> bool { self.app.has_args() }

    pub(crate) fn has_opts(&self) -> bool { self.app.has_opts() }

    pub(crate) fn has_flags(&self) -> bool { self.app.has_flags() }

    pub(crate) fn has_positionals(&self) -> bool {
        self.app.args.args.iter().any(|x| x.index.is_some())
    }

    pub(crate) fn has_subcommands(&self) -> bool { self.app.has_subcommands() }

    pub(crate) fn has_visible_subcommands(&self) -> bool { self.app.has_visible_subcommands() }

    pub(crate) fn is_set(&self, s: AS) -> bool { self.app.is_set(s) }

    pub(crate) fn set(&mut self, s: AS) { self.app.set(s) }

    pub(crate) fn unset(&mut self, s: AS) { self.app.unset(s) }
}
