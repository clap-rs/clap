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
use build::app::Propagation;
use build::AppSettings as AS;
use build::{App, Arg, ArgSettings};
use output::Help;
use output::Usage;
use parse::errors::Error as ClapError;
use parse::errors::ErrorKind;
use parse::errors::Result as ClapResult;
use parse::features::suggestions;
use parse::Validator;
use parse::{ArgMatcher, SubCommand};
use util::{hash, OsStrExt2};
use INTERNAL_ERROR_MSG;
use INVALID_UTF8;
use parse::{SeenArg, Key};

#[derive(Debug, PartialEq, Copy, Clone)]
#[doc(hidden)]
pub enum ParserState {
    Initial,
}

enum HyphenStyle {
    Single,
    Double,
    DoubleOnly,
    None
}

fn assert_highest_index_matches_len(positionals: &[(u64, &Arg)]) {
    // Firt we verify that the highest supplied index, is equal to the number of
    // positional arguments to verify there are no gaps (i.e. supplying an index of 1 and 3
    // but no 2)
    let highest_idx = positionals.iter().max_by(|x, y| x.0.cmp(&y.0)).unwrap();

    let num_p = positionals.len();

    assert!(
        highest_idx.0 == num_p as u64,
        "Found positional argument whose index is {} but there \
            are only {} positional arguments defined",
        highest_idx.0,
        num_p
    );
}

fn assert_low_index_multiples(positionals: &[(u64, &Arg)]) {
    // First we make sure if there is a positional that allows multiple values
    // the one before it (second to last) has one of these:
    //  * a value terminator
    //  * ArgSettings::Last
    //  * The last arg is Required

    let last = positionals.iter().last().unwrap().1;

    let second_to_last = positionals.iter().rev().next().unwrap().1;

    // Either the final positional is required
    // Or the second to last has a terminator or .last(true) set
    let ok = (last.is_set(ArgSettings::Required) || last.is_set(ArgSettings::Last))
        || (second_to_last.terminator.is_some()
            || second_to_last.is_set(ArgSettings::Last));
    assert!(
        ok,
        "When using a positional argument with .multiple(true) that is *not the \
            last* positional argument, the last positional argument (i.e the one \
            with the highest index) *must* have .required(true) or .last(true) set."
    );
}

fn assert_missing_positionals(positionals: &[(u64, &Arg)]) {
    // Check that if a required positional argument is found, all positions with a lower
    // index are also required.
    let mut found = false;
    let mut foundx2 = false;

    for (i, p) in positionals.iter().rev() {
        if foundx2 && !p.is_set(ArgSettings::Required) {
            assert!(
                p.is_set(ArgSettings::Required),
                "Found positional argument which is not required with a lower \
                    index than a required positional argument by two or more: {:?} \
                    index {:?}",
                p.id,
                i
            );
        } else if p.is_set(ArgSettings::Required) && !p.is_set(ArgSettings::Last) {
            // Args that .last(true) don't count since they can be required and have
            // positionals with a lower index that aren't required
            // Imagine: prog <req1> [opt1] -- <req2>
            // Both of these are valid invocations:
            //      $ prog r1 -- r2
            //      $ prog r1 o1 -- r2
            if found {
                foundx2 = true;
                continue;
            }
            found = true;
            continue;
        } else {
            found = false;
        }
    }
}

fn assert_only_one_last(positionals: &[(u64, &Arg)]) {
    assert!(
        positionals.iter().fold(0, |acc, (_, p)| if p.is_set(ArgSettings::Last) {
            acc + 1
        } else {
            acc
        }) < 2,
        "Only one positional argument may have last(true) set. Found two."
    );
}

fn assert_required_last_and_subcommands(positionals: &[(u64, &Arg)], has_subcmds: bool, subs_negate_reqs: bool) {
    assert!(!(positionals.iter()
        .any(|(_, p)| p.is_set(ArgSettings::Last) && p.is_set(ArgSettings::Required))
        && has_subcmds
        && !subs_negate_reqs),
            "Having a required positional argument with .last(true) set *and* child \
                subcommands without setting SubcommandsNegateReqs isn't compatible.");
}

fn get_hyphen_style(arg: &OsStr) -> HyphenStyle {
    if arg.starts_with(b"--") {
        if arg.len() == 2 {
            return HyphenStyle::DoubleOnly;
        } else {
            return HyphenStyle::Double;
        }
    } else if arg.starts_with(b"-") && arg.len() != 1 {
        return HyphenStyle::Single;
    }

    HyphenStyle::None
}

#[doc(hidden)]
pub struct Parser<'help, 'c>
where
    'help: 'c,
{
    pub app: &'c mut App<'help>,
    seen: Vec<SeenArg>,
    cur_idx: Cell<usize>,
}

// Initializing Methods
impl<'help, 'c> Parser<'help, 'c>
where
    'help: 'c,
{
    pub fn new(app: &'c mut App<'help>) -> Self {
        Parser {
            app,
            seen: Vec::new(),
            cur_idx: Cell::new(0),
        }
    }

    fn _setup_positionals(&mut self) -> bool {
        debugln!("Parser::_setup_positionals;");
        // Because you must wait until all arguments have been supplied, this is the first chance
        // to make assertions on positional argument indexes

        let positionals: Vec<(u64, &Arg)> = self
            .app
            .args
            .args
            .iter()
            .filter(|x| x.index.is_some())
            .map(|x| (x.index.unwrap(), x))
            .collect();
        
        if positionals.is_empty() { return true; }

        assert_highest_index_matches_len(&*positionals);

        if positionals.iter().filter(|(_, p)| p.is_set(ArgSettings::MultipleValues)).count() > 1 {
            assert_low_index_multiples(&*positionals);
            self.set(AS::LowIndexMultiplePositional);
        }

        if !self.is_set(AS::AllowMissingPositional) {
            assert_missing_positionals(&*positionals);
        }

        assert_only_one_last(&*positionals);

        assert_required_last_and_subcommands(&*positionals, self.has_subcommands(), self.is_set(AS::SubcommandsNegateReqs));

        true
    }

    // Does all the initializing and prepares the parser
    pub(crate) fn _build(&mut self) {
        debugln!("Parser::_build;");

        self.app.args._build();

        self._setup_positionals();
    }
}

// Parsing Methods
impl<'help, 'c> Parser<'help, 'c>
where
    'help: 'c,
{
    // The actual parsing function
    pub fn get_matches_with<I, T>(
        &mut self,
        matcher: &mut ArgMatcher,
        it: &mut Peekable<I>,
    ) -> ClapResult<()>
    where
        I: Iterator<Item = T>,
        T: AsRef<OsStr>, // + Clone?
    {
        debugln!("Parser::get_matches_with;");
        // Verify all positional assertions pass
        self._build();

        let mut subcmd_name: Option<u64> = None;
        let mut state: ParserState = ParserState::Initial;
        let mut pos_counter = 1;

        for arg in it {
            let arg_os = arg.as_ref();
            debugln!(
                "Parser::get_matches_with: Begin parsing '{:?}' ({:?})",
                arg_os,
                &*arg_os.as_bytes()
            );

            match state {
                ParserState::Initial => match get_hyphen_style(&*arg_os) {
                    HyphenStyle::DoubleOnly => {
                        self.set(AS::TrailingValues);
                        continue;
                    },
                    HyphenStyle::Double => state = self.parse_long(matcher, arg_os.into()),
                    HyphenStyle::Single =>,
                    HyphenStyle::None =>,
                },
            }

            // Is this a new argument, or values from a previous option?
            let starts_new_arg = self.is_new_arg(&arg_os, state);
            if !self.is_set(AS::TrailingValues)
                && arg_os.starts_with(b"--")
                && arg_os.len() == 2
                && starts_new_arg
            {
                debugln!("Parser::get_matches_with: setting TrailingVals=true");
                self.set(AS::TrailingValues);
                continue;
            }

            // Has the user already passed '--'? Meaning only positional args follow
            if !self.is_set(AS::TrailingValues) {
                // Does the arg match a subcommand name, or any of it's aliases (if defined)
                {
                    match needs_val_of {
                        ParseResult::Opt(_) | ParseResult::Pos(_) => (),
                        _ => {
                            let (is_match, sc_name) = self.possible_subcommand(&arg_os);
                            debugln!(
                                "Parser::get_matches_with: possible_sc={:?}, sc={:?}",
                                is_match,
                                sc_name
                            );
                            if is_match {
                                let sc_name = sc_name.expect(INTERNAL_ERROR_MSG);
                                if sc_name == hash("help") && !self.is_set(AS::NoAutoHelp) {
                                    self.parse_help_subcommand(it)?;
                                }
                                subcmd_name = Some(sc_name.to_owned());
                                break;
                            }
                        }
                    }
                }

                if starts_new_arg {
                    if arg_os.starts_with(b"--") {
                        needs_val_of = self.parse_long_arg(matcher, &arg_os)?;
                        debugln!(
                            "Parser:get_matches_with: After parse_long_arg {:?}",
                            needs_val_of
                        );
                        match needs_val_of {
                            ParseResult::Flag | ParseResult::Opt(..) | ParseResult::ValuesDone => {
                                continue
                            }
                            _ => (),
                        }
                    } else if arg_os.starts_with(b"-") && arg_os.len() != 1 {
                        // Try to parse short args like normal, if AllowLeadingHyphen or
                        // AllowNegativeNumbers is set, parse_short_arg will *not* throw
                        // an error, and instead return Ok(None)
                        needs_val_of = self.parse_short_arg(matcher, &arg_os)?;
                        // If it's None, we then check if one of those two AppSettings was set
                        debugln!(
                            "Parser:get_matches_with: After parse_short_arg {:?}",
                            needs_val_of
                        );
                        match needs_val_of {
                            ParseResult::MaybeNegNum => {
                                if !(arg_os.to_string_lossy().parse::<i64>().is_ok()
                                    || arg_os.to_string_lossy().parse::<f64>().is_ok())
                                {
                                    return Err(ClapError::unknown_argument(
                                        &*arg_os.to_string_lossy(),
                                        "",
                                        &*Usage::new(self).create_usage_with_title(&[]),
                                        self.app.color(),
                                    ));
                                }
                            }
                            ParseResult::Opt(..) | ParseResult::Flag | ParseResult::ValuesDone => {
                                continue
                            }
                            _ => (),
                        }
                    }
                } else if let ParseResult::Opt(name) = needs_val_of {
                    // Check to see if parsing a value from a previous arg
                    let arg = self.app.find(name).expect(INTERNAL_ERROR_MSG);
                    // get the option so we can check the settings
                    needs_val_of = self.add_val_to_arg(arg, &arg_os, matcher)?;
                    // get the next value from the iterator
                    continue;
                }
            }

            if !(self.is_set(AS::ArgsNegateSubcommands) && self.is_set(AS::ValidArgFound)
                || self.is_set(AS::AllowExternalSubcommands)
                || self.is_set(AS::InferSubcommands))
            {
                if let Some(cdate) =
                    suggestions::did_you_mean(&*arg_os.to_string_lossy(), sc_names!(self.app))
                {
                    return Err(ClapError::invalid_subcommand(
                        arg_os.to_string_lossy().into_owned(),
                        cdate,
                        self.app.bin_name.as_ref().unwrap_or(&self.app.name),
                        &*Usage::new(self).create_usage_with_title(&[]),
                        self.app.color(),
                    ));
                }
            }

            let positional_count = self
                .app
                .args
                .keys
                .iter()
                .map(|x| &x.key)
                .filter(|x| {
                    if let KeyType::Position(_) = x {
                        true
                    } else {
                        false
                    }
                })
                .count();
            let is_second_to_last = positional_count > 1 && (pos_counter == (positional_count - 1));

            let low_index_mults = self.is_set(AS::LowIndexMultiplePositional) && is_second_to_last;
            let missing_pos = self.is_set(AS::AllowMissingPositional)
                && is_second_to_last
                && !self.is_set(AS::TrailingValues);
            debugln!(
                "Parser::get_matches_with: Positional counter...{}",
                pos_counter
            );
            debugln!(
                "Parser::get_matches_with: Low index multiples...{:?}",
                low_index_mults
            );
            if low_index_mults || missing_pos {
                if let Some(na) = it.peek() {
                    let n = (*na).clone().into();
                    needs_val_of = if needs_val_of != ParseResult::ValuesDone {
                        if let Some(p) =
                            positionals!(self.app).find(|p| p.index == Some(pos_counter as u64))
                        {
                            ParseResult::Pos(p.name)
                        } else {
                            ParseResult::ValuesDone
                        }
                    } else {
                        ParseResult::ValuesDone
                    };
                    let sc_match = { self.possible_subcommand(&n).0 };
                    if self.is_new_arg(&n, needs_val_of)
                        || sc_match
                        || suggestions::did_you_mean(&n.to_string_lossy(), sc_names!(self.app))
                            .is_some()
                    {
                        debugln!("Parser::get_matches_with: Bumping the positional counter...");
                        pos_counter += 1;
                    }
                } else {
                    debugln!("Parser::get_matches_with: Bumping the positional counter...");
                    pos_counter += 1;
                }
            } else if (self.is_set(AS::AllowMissingPositional) && self.is_set(AS::TrailingValues))
                || (self.is_set(AS::ContainsLast) && self.is_set(AS::TrailingValues))
            {
                // Came to -- and one postional has .last(true) set, so we go immediately
                // to the last (highest index) positional
                debugln!("Parser::get_matches_with: .last(true) and --, setting last pos");
                pos_counter = self
                    .app
                    .args
                    .keys
                    .iter()
                    .map(|x| &x.key)
                    .filter(|x| {
                        if let KeyType::Position(_) = x {
                            true
                        } else {
                            false
                        }
                    })
                    .count();
            }
            if let Some(p) = positionals!(self.app).find(|p| p.index == Some(pos_counter as u64)) {
                if p.is_set(ArgSettings::Last) && !self.is_set(AS::TrailingValues) {
                    return Err(ClapError::unknown_argument(
                        &*arg_os.to_string_lossy(),
                        "",
                        &*Usage::new(self).create_usage_with_title(&[]),
                        self.app.color(),
                    ));
                }
                if !self.is_set(AS::TrailingValues)
                    && (self.is_set(AS::TrailingVarArg)
                        && pos_counter
                            == self
                                .app
                                .args
                                .keys
                                .iter()
                                .map(|x| &x.key)
                                .filter(|x| x.is_position())
                                .count())
                {
                    self.app.settings.set(AS::TrailingValues);
                }
                self.seen.push(p.name);
                let _ = self.add_val_to_arg(p, &arg_os, matcher)?;

                matcher.inc_occurrence_of(p.name);
                for grp in groups_for_arg!(self.app, &p.name) {
                    matcher.inc_occurrence_of(&*grp);
                }

                self.app.settings.set(AS::ValidArgFound);
                // Only increment the positional counter if it doesn't allow multiples
                if !p.settings.is_set(ArgSettings::MultipleValues) {
                    pos_counter += 1;
                }
                self.app.settings.set(AS::ValidArgFound);
            } else if self.is_set(AS::AllowExternalSubcommands) {
                // Get external subcommand name
                let sc_name = match arg_os.to_str() {
                    Some(s) => s.to_string(),
                    None => {
                        if !self.is_set(AS::StrictUtf8) {
                            return Err(ClapError::invalid_utf8(
                                &*Usage::new(self).create_usage_with_title(&[]),
                                self.app.color(),
                            ));
                        }
                        arg_os.to_string_lossy().into_owned()
                    }
                };

                // Collect the external subcommand args
                let mut sc_m = ArgMatcher::new();
                while let Some(v) = it.next() {
                    let a = v.into();
                    if a.to_str().is_none() && !self.is_set(AS::StrictUtf8) {
                        return Err(ClapError::invalid_utf8(
                            &*Usage::new(self).create_usage_with_title(&[]),
                            self.app.color(),
                        ));
                    }
                    sc_m.add_val_to(0, &a);
                }

                matcher.subcommand(SubCommand {
                    name: sc_name,
                    matches: sc_m.into(),
                });
            } else if !((self.is_set(AS::AllowLeadingHyphen)
                || self.is_set(AS::AllowNegativeNumbers))
                && arg_os.starts_with(b"-"))
                && !self.is_set(AS::InferSubcommands)
            {
                return Err(ClapError::unknown_argument(
                    &*arg_os.to_string_lossy(),
                    "",
                    &*Usage::new(self).create_usage_with_title(&[]),
                    self.app.color(),
                ));
            } else if !self.has_args() || self.is_set(AS::InferSubcommands) && self.has_subcommands() {
                if let Some(cdate) =
                    suggestions::did_you_mean(&*arg_os.to_string_lossy(), sc_names!(self.app))
                {
                    return Err(ClapError::invalid_subcommand(
                        arg_os.to_string_lossy().into_owned(),
                        cdate,
                        self.app.bin_name.as_ref().unwrap_or(&self.app.name),
                        &*Usage::new(self).create_usage_with_title(&[]),
                        self.app.color(),
                    ));
                } else {
                    return Err(ClapError::unrecognized_subcommand(
                        arg_os.to_string_lossy().into_owned(),
                        self.app.bin_name.as_ref().unwrap_or(&self.app.name),
                        self.app.color(),
                    ));
                }
            } else {
                return Err(ClapError::unknown_argument(
                    &*arg_os.to_string_lossy(),
                    "",
                    &*Usage::new(self).create_usage_with_title(&[]),
                    self.app.color(),
                ));
            }
        }

        if let Some(ref pos_sc_name) = subcmd_name {
            let sc_name = {
                find_subcmd!(self.app, *pos_sc_name)
                    .expect(INTERNAL_ERROR_MSG)
                    .name
                    .clone()
            };
            self.parse_subcommand(&*sc_name, matcher, it)?;
        } else if self.is_set(AS::SubcommandRequired) {
            let bn = self.app.bin_name.as_ref().unwrap_or(&self.app.name);
            return Err(ClapError::missing_subcommand(
                bn,
                &Usage::new(self).create_usage_with_title(&[]),
                self.app.color(),
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

        self.remove_overrides(matcher);

        Validator::new(self).validate(needs_val_of, &subcmd_name, matcher)
    }

    // Checks if the arg matches a subcommand name, or any of it's aliases (if defined)
    fn possible_subcommand(&self, arg_os: &OsStr) -> (bool, Option<u64>) {
        debugln!("Parser::possible_subcommand: arg={:?}", arg_os);
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
            return (false, None);
        }
        if !self.is_set(AS::InferSubcommands) {
            if let Some(sc) = find_subcmd!(self.app, arg_os) {
                return (true, Some(&sc.name));
            }
        } else {
            let v = sc_names!(self.app)
                .filter(|s| starts(s, &*arg_os))
                .collect::<Vec<_>>();

            if v.len() == 1 {
                return (true, Some(v[0]));
            }
        }
        (false, None)
    }

    fn parse_help_subcommand<I, T>(&self, it: &mut I) -> ClapResult<ParseResult>
    where
        I: Iterator<Item = T>,
        T: Into<OsString>,
    {
        debugln!("Parser::parse_help_subcommand;");
        let cmds: Vec<OsString> = it.map(|c| c.into()).collect();
        let mut help_help = false;
        let mut bin_name = self.app.bin_name.as_ref().unwrap_or(&self.app.name).clone();
        let mut sc = {
            // @TODO @perf: cloning all these Apps ins't great, but since it's just displaying the
            // help message there are bigger fish to fry
            let mut sc = self.app.clone();
            for (i, cmd) in cmds.iter().enumerate() {
                if &*cmd.to_string_lossy() == "help" {
                    // cmd help help
                    help_help = true;
                    break; // Maybe?
                }
                if let Some(mut c) = find_subcmd_cloned!(sc, cmd) {
                    c._build(Propagation::NextLevel);
                    sc = c;
                    if i == cmds.len() - 1 {
                        break;
                    }
                } else if let Some(mut c) = find_subcmd_cloned!(sc, &*cmd.to_string_lossy()) {
                    c._build(Propagation::NextLevel);
                    sc = c;
                    if i == cmds.len() - 1 {
                        break;
                    }
                } else {
                    return Err(ClapError::unrecognized_subcommand(
                        cmd.to_string_lossy().into_owned(),
                        self.app.bin_name.as_ref().unwrap_or(&self.app.name),
                        self.app.color(),
                    ));
                }
                bin_name = format!("{} {}", bin_name, &*sc.name);
            }
            sc
        };
        let parser = Parser::new(&mut sc);
        if help_help {
            let mut pb = Arg::new("subcommand")
                .index(1)
                .setting(ArgSettings::MultipleValues)
                .help("The subcommand whose help message to display");
            pb._build();
            //parser.positionals.insert(1, pb.name);
            parser.app.settings = parser.app.settings | self.app.g_settings;
            parser.app.g_settings = self.app.g_settings;
        }
        if parser.app.bin_name != self.app.bin_name {
            parser.app.bin_name = Some(format!("{} {}", bin_name, parser.app.name));
        }
        Err(parser.help_err(false))
    }

    // allow wrong self convention due to self.valid_neg_num = true and it's a private method
    fn is_new_arg(&mut self, arg_os: &OsStr, curr_state: ParserState) -> ParserState {
        debugln!("Parser::is_new_arg:{:?}:{:?}", arg_os, state);
        let new_state = ParserState::Initial;
        if self.is_set(AS::AllowLeadingHyphen) {
            new_state = ParserState::GlobalHyphenValuesAllowed;
        } else if self.is_set(AS::AllowNegativeNumbers) {
            let a = arg_os.to_string_lossy();
            if let Ok(i) = a.parse::<i64>() {
                new_state = ParserState::ValidNegIntFound(i);
            } else if let Ok(f) = a.parse::<f64>() {
                new_state = ParserState::ValidNegFloatFound(f);
            }
        }

        match curr_state {
            ParserState::Opt(id) | ParserStatePos(id) => {
                let a = self.app.find(name).expect(INTERNAL_ERROR_MSG);
                if a.is_set(ArgSettings::AllowHyphenValues) {
                    new_state = ParserState::MaybeValueOf(id);
                }
            }
            ParserState::ValuesDone => new_state = ParserState::ValuesDone,
            _ => (),
        }

        // Is this a new argument, or values from a previous option?
        let mut ret = if arg_os.starts_with(b"--") {
            debugln!("Parser::is_new_arg: -- found");
            if arg_os.len() == 2 && !arg_allows_tac {
                return true; // We have to return true so override everything else
            } else if arg_allows_tac {
                return false;
            }
            true
        } else if arg_os.starts_with(b"-") {
            debugln!("Parser::is_new_arg: - found");
            // a singe '-' by itself is a value and typically means "stdin" on unix systems
            arg_os.len() != 1
        } else {
            debugln!("Parser::is_new_arg: probably value");
            false
        };

        ret = ret && !arg_allows_tac;

        debugln!("Parser::is_new_arg: starts_new_arg={:?}", ret);
        ret
    }

    fn parse_subcommand<I, T>(
        &mut self,
        sc_name: u64,
        matcher: &mut ArgMatcher,
        it: &mut Peekable<I>,
    ) -> ClapResult<()>
    where
        I: Iterator<Item = T>,
        T: Into<OsString> + Clone,
    {
        use std::fmt::Write;
        debugln!("Parser::parse_subcommand;");
        let mut mid_string = String::new();
        if !self.is_set(AS::SubcommandsNegateReqs) {
            let reqs = Usage::new(self).get_required_usage_from(&[], None, true); // maybe Some(m)

            for s in &reqs {
                write!(&mut mid_string, " {}", s).expect(INTERNAL_ERROR_MSG);
            }
        }
        mid_string.push_str(" ");
        if let Some(ref mut sc) = subcommands_mut!(self.app).find(|s| s.name == sc_name) {
            let mut sc_matcher = ArgMatcher::new();
            // bin_name should be parent's bin_name + [<reqs>] + the sc's name separated by
            // a space
            sc.usage = Some(format!(
                "{}{}{}",
                self.app.bin_name.as_ref().unwrap_or(&String::new()),
                if self.app.bin_name.is_some() {
                    &*mid_string
                } else {
                    ""
                },
                &*sc.name
            ));
            sc.bin_name = Some(format!(
                "{}{}{}",
                self.app.bin_name.as_ref().unwrap_or(&String::new()),
                if self.app.bin_name.is_some() { " " } else { "" },
                &*sc.name
            ));

            // Ensure all args are built and ready to parse
            sc._build(Propagation::NextLevel);

            debugln!("Parser::parse_subcommand: About to parse sc={}", sc.name);

            let name = sc.name.clone();
            let mut p = Parser::new(sc);
            p.get_matches_with(&mut sc_matcher, it)?;
            matcher.subcommand(SubCommand {
                name,
                matches: sc_matcher.into(),
            });
        }
        Ok(())
    }

    // Retrieves the names of all args the user has supplied thus far, except required ones
    // because those will be listed in self.required
    fn check_for_help_and_version_str(&self, arg: &OsStr) -> ClapResult<()> {
        debugln!("Parser::check_for_help_and_version_str;");
        debug!(
            "Parser::check_for_help_and_version_str: Checking if --{} is help or version...",
            arg.to_str().unwrap()
        );

        // Needs to use app.settings.is_set instead of just is_set() because is_set() checks
        // both global and local settings, we only want to check local
        if arg == "help" && !self.app.settings.is_set(AS::NoAutoHelp) {
            sdebugln!("Help");
            return Err(self.help_err(true));
        }
        if arg == "version" && !self.app.settings.is_set(AS::NoAutoVersion) {
            sdebugln!("Version");
            return Err(self.version_err(true));
        }
        sdebugln!("Neither");

        Ok(())
    }

    fn check_for_help_and_version_char(&self, arg: char) -> ClapResult<()> {
        debugln!("Parser::check_for_help_and_version_char;");
        debug!(
            "Parser::check_for_help_and_version_char: Checking if -{} is help or version...",
            arg
        );
        // Needs to use app.settings.is_set instead of just is_set() because is_set() checks
        // both global and local settings, we only want to check local
        if let Some(help) = self.app.find(hash("help")) {
            if let Some(h) = help.short {
                if arg == h && !self.app.settings.is_set(AS::NoAutoHelp) {
                    sdebugln!("Help");
                    return Err(self.help_err(false));
                }
            }
        }
        if let Some(version) = self.app.find(hash("version")) {
            if let Some(v) = version.short {
                if arg == v && !self.app.settings.is_set(AS::NoAutoVersion) {
                    sdebugln!("Version");
                    return Err(self.version_err(false));
                }
            }
        }
        sdebugln!("Neither");
        Ok(())
    }

    fn use_long_help(&self) -> bool {
        debugln!("Parser::use_long_help;");
        // In this case, both must be checked. This allows the retention of
        // original formatting, but also ensures that the actual -h or --help
        // specified by the user is sent through. If HiddenShortHelp is not included,
        // then items specified with hidden_short_help will also be hidden.
        let should_long = |v: &Arg| {
            v.long_help.is_some()
                || v.is_set(ArgSettings::HiddenLongHelp)
                || v.is_set(ArgSettings::HiddenShortHelp)
        };

        self.app.long_about.is_some()
            || self.app.args.args.iter().any(|f| should_long(&f))
            || subcommands!(self.app).any(|s| s.long_about.is_some())
    }

    fn parse_long(
        &mut self,
        matcher: &mut ArgMatcher,
        full_arg: RawArg,
    ) -> ClapResult<ParseState> {
        // maybe here lifetime should be 'a
        debugln!("Parser::parse_long;");

        // Update the curent index
        self.cur_idx.set(self.cur_idx.get() + 1);

        let raw_long: RawLong = full_arg.into();
        if let Some(arg) = self.app.args.get_by_long_with_hyphen(raw_long.long.as_bytes()) {
            debugln!("Parser::parse_long: Found valid arg '{}'", arg.to_string());
            self.app.settings.set(AS::ValidArgFound);

            self.seen.push(SeenArg::new(arg.id, Key::Long));

            if opt.is_set(ArgSettings::TakesValue) {
                return Ok(self.parse_opt(raw_long, matcher)?);
            }
            self.check_for_help_and_version_str(arg)?;
            self.parse_flag(opt, matcher)?;

            return Ok(ParseResult::Flag);
        } else if self.is_set(AS::AllowLeadingHyphen) {
            return Ok(ParseResult::MaybeHyphenValue);
        } else if self.is_set(AS::ValidNegNumFound) {
            return Ok(ParseResult::MaybeNegNum);
        }

        debugln!("Parser::parse_long_arg: Didn't match anything");
        self.did_you_mean_error(arg.to_str().expect(INVALID_UTF8), matcher)
            .map(|_| ParseResult::NotFound)
    }

    #[cfg_attr(feature = "lints", allow(len_zero))]
    fn parse_short_arg(
        &mut self,
        matcher: &mut ArgMatcher,
        full_arg: &OsStr,
    ) -> ClapResult<ParseResult> {
        debugln!("Parser::parse_short_arg: full_arg={:?}", full_arg);
        let arg_os = full_arg.trim_left_matches(b'-');
        let arg = arg_os.to_string_lossy();

        // If AllowLeadingHyphen is set, we want to ensure `-val` gets parsed as `-val` and not
        // `-v` `-a` `-l` assuming `v` `a` and `l` are all, or mostly, valid shorts.
        if self.is_set(AS::AllowLeadingHyphen) {
            if arg.chars().any(|c| !self.contains_short(c)) {
                debugln!(
                    "Parser::parse_short_arg: LeadingHyphenAllowed yet -{} isn't valid",
                    arg
                );
                return Ok(ParseResult::MaybeHyphenValue);
            }
        } else if self.is_set(AS::ValidNegNumFound) {
            // TODO: Add docs about having AllowNegativeNumbers and `-2` as a valid short
            // May be better to move this to *after* not finding a valid flag/opt?
            debugln!("Parser::parse_short_arg: Valid negative num...");
            return Ok(ParseResult::MaybeNegNum);
        }

        let mut ret = ParseResult::NotFound;
        for c in arg.chars() {
            debugln!("Parser::parse_short_arg:iter:{}", c);

            // update each index because `-abcd` is four indices to clap
            self.cur_idx.set(self.cur_idx.get() + 1);

            // Check for matching short options, and return the name if there is no trailing
            // concatenated value: -oval
            // Option: -o
            // Value: val
            if let Some(opt) = self.app.args.get(&KeyType::Short(c)) {
                debugln!(
                    "Parser::parse_short_arg:iter:{}: Found valid opt or flag",
                    c
                );
                self.app.settings.set(AS::ValidArgFound);
                self.seen.push(opt.name);
                if !opt.is_set(ArgSettings::TakesValue) {
                    self.check_for_help_and_version_char(c)?;
                    ret = self.parse_flag(opt, matcher)?;
                    continue;
                }

                // Check for trailing concatenated value
                let p: Vec<_> = arg.splitn(2, c).collect();
                debugln!(
                    "Parser::parse_short_arg:iter:{}: p[0]={:?}, p[1]={:?}",
                    c,
                    p[0].as_bytes(),
                    p[1].as_bytes()
                );
                let i = p[0].as_bytes().len() + 1;
                let val = if !p[1].as_bytes().is_empty() {
                    debugln!(
                        "Parser::parse_short_arg:iter:{}: val={:?} (bytes), val={:?} (ascii)",
                        c,
                        arg_os.split_at(i).1.as_bytes(),
                        arg_os.split_at(i).1
                    );
                    Some(arg_os.split_at(i).1)
                } else {
                    None
                };

                // Default to "we're expecting a value later"
                let ret = self.parse_opt(val, opt, false, matcher)?;

                return Ok(ret);
            } else {
                let arg = format!("-{}", c);
                return Err(ClapError::unknown_argument(
                    &*arg,
                    "",
                    &*Usage::new(self).create_usage_with_title(&[]),
                    self.app.color(),
                ));
            }
        }
        Ok(ret)
    }

    fn parse_opt(
        &self,
        raw: RawOpt,
        opt: &Arg<'help>,
        matcher: &mut ArgMatcher,
    ) -> ClapResult<ParseState> {
        debugln!("Parser::parse_opt; opt={}, val={:?}", opt.id, raw_long.full_value);
        debugln!("Parser::parse_opt; opt.settings={:?}", opt.settings);
        let needs_eq = opt.is_set(ArgSettings::RequireEquals);
        let had_eq = raw.value.map_or(false, |v| v.had_eq);

        if raw.value.is_some() {
            self.add_val_to_arg(opt, raw, matcher)?;
        } else if needs_eq {
            return Err(ClapError::empty_value(
                opt,
                &*Usage::new(self).create_usage_with_title(&[]),
                self.app.color(),
            ));
        }

        matcher.inc_occurrence_of(opt.id);
        // Increment or create the group "args"
        for grp in groups_for_arg!(self.app, &opt.id) {
            matcher.inc_occurrence_of(&*grp);
        }

        let needs_delim = opt.is_set(ArgSettings::RequireDelimiter);
        let mult = opt.is_set(ArgSettings::MultipleValues);
        if raw.value.is_none() || (mult && !needs_delim) && !had_eq && matcher.needs_more_vals(opt) {
            debugln!("Parser::parse_opt: More arg vals required...");
            return Ok(ParseState::OptAcceptsValues(opt.id));
        }
        debugln!("Parser::parse_opt: More arg vals not required...");
        Ok(ParseState::Initial)
    }

    fn add_val_to_arg(
        &self,
        arg: &Arg<'help>,
        raw: RawValue,
        matcher: &mut ArgMatcher,
    ) -> ClapResult<ParseState> {
        debugln!("Parser::add_val_to_arg; arg={}, val={:?}", arg.name, val);
        debugln!(
            "Parser::add_val_to_arg; trailing_vals={:?}, DontDelimTrailingVals={:?}",
            self.is_set(AS::TrailingValues),
            self.is_set(AS::DontDelimitTrailingValues)
        );
        if !(self.is_set(AS::TrailingValues) && self.is_set(AS::DontDelimitTrailingValues)) {
            if let Some(delim) = arg.val_delim {
                if val.is_empty() {
                    Ok(self.add_single_val_to_arg(arg, val, matcher)?)
                } else {
                    let mut iret = ParseResult::ValuesDone;
                    for v in val.split(delim as u32 as u8) {
                        iret = self.add_single_val_to_arg(arg, v, matcher)?;
                    }
                    // If there was a delimiter used, we're not looking for more values
                    if val.contains_byte(delim as u32 as u8)
                        || arg.is_set(ArgSettings::RequireDelimiter)
                    {
                        iret = ParseResult::ValuesDone;
                    }
                    Ok(iret)
                }
            } else {
                self.add_single_val_to_arg(arg, val, matcher)
            }
        } else {
            self.add_single_val_to_arg(arg, val, matcher)
        }
    }

    fn add_single_val_to_arg(
        &self,
        arg: &Arg<'help>,
        v: &OsStr,
        matcher: &mut ArgMatcher,
    ) -> ClapResult<ParseResult> {
        debugln!("Parser::add_single_val_to_arg;");
        debugln!("Parser::add_single_val_to_arg: adding val...{:?}", v);

        // update the current index because each value is a distinct index to clap
        self.cur_idx.set(self.cur_idx.get() + 1);

        // @TODO @docs @p4 docs should probably note that terminator doesn't get an index
        if let Some(t) = arg.terminator {
            if t == v {
                return Ok(ParseResult::ValuesDone);
            }
        }

        matcher.add_val_to(arg.name, v);
        matcher.add_index_to(arg.name, self.cur_idx.get());

        // Increment or create the group "args"
        for grp in groups_for_arg!(self.app, &arg.name) {
            matcher.add_val_to(&*grp, v);
        }

        if matcher.needs_more_vals(arg) {
            return Ok(ParseResult::Opt(arg.name));
        }
        Ok(ParseResult::ValuesDone)
    }

    fn parse_flag(
        &self,
        flag: &Arg<'help>,
        matcher: &mut ArgMatcher,
    ) -> ClapResult<ParseResult> {
        debugln!("Parser::parse_flag;");

        matcher.inc_occurrence_of(flag.name);
        matcher.add_index_to(flag.name, self.cur_idx.get());
        // Increment or create the group "args"
        for grp in groups_for_arg!(self.app, &flag.name) {
            matcher.inc_occurrence_of(grp);
        }

        Ok(ParseResult::Flag)
    }

    fn remove_overrides(&mut self, matcher: &mut ArgMatcher) {
        debugln!("Parser::remove_overrides;");
        let mut to_rem: Vec<u64> = Vec::new();
        let mut self_override: Vec<u64> = Vec::new();
        let mut arg_overrides = Vec::new();
        for name in matcher.arg_names() {
            debugln!("Parser::remove_overrides:iter:{};", name);
            if let Some(arg) = self.app.find(*name) {
                let mut handle_self_override = |o| {
                    if (arg.is_set(ArgSettings::MultipleValues)
                        || arg.is_set(ArgSettings::MultipleOccurrences))
                        || !arg.has_switch()
                    {
                        return true;
                    }
                    debugln!(
                        "Parser::remove_overrides:iter:{}:iter:{}: self override;",
                        name,
                        o
                    );
                    self_override.push(o);
                    false
                };
                if let Some(ref overrides) = arg.overrides {
                    debugln!("Parser::remove_overrides:iter:{}:{:?};", name, overrides);
                    for o in overrides {
                        if o == &arg.name {
                            if handle_self_override(o) {
                                continue;
                            }
                        } else {
                            arg_overrides.push((&arg.name, o));
                            arg_overrides.push((o, &arg.name));
                        }
                    }
                }
                if self.is_set(AS::AllArgsOverrideSelf) {
                    let _ = handle_self_override(arg.name);
                }
            }
        }

        // remove future overrides in reverse seen order
        for arg in self.seen.iter().rev() {
            for &(a, overr) in arg_overrides.iter().filter(|&&(a, _)| a == arg) {
                if !to_rem.contains(a) {
                    to_rem.push(*overr);
                }
            }
        }

        // Do self overrides
        for name in &self_override {
            debugln!("Parser::remove_overrides:iter:self:{}: resetting;", name);
            if let Some(ma) = matcher.get_mut(*name) {
                if ma.occurs < 2 {
                    continue;
                }
                ma.occurs = 1;
                if !ma.vals.is_empty() {
                    // This avoids a clone
                    let mut v = vec![ma.vals.pop().expect(INTERNAL_ERROR_MSG)];
                    mem::swap(&mut v, &mut ma.vals);
                }
            }
        }

        // Finally remove conflicts
        for name in &to_rem {
            debugln!("Parser::remove_overrides:iter:{}: removing;", name);
            matcher.remove(*name);
            self.overriden.push(*name);
        }
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

        for o in opts!(self.app) {
            debug!("Parser::add_defaults:iter:{}:", o.name);
            add_val!(self, o, matcher);
        }
        for p in positionals!(self.app) {
            debug!("Parser::add_defaults:iter:{}:", p.name);
            add_val!(self, p, matcher);
        }
        Ok(())
    }

    pub(crate) fn add_env(&mut self, matcher: &mut ArgMatcher) -> ClapResult<()> {
        for a in self.app.args.args.iter() {
            if let Some(ref val) = a.env {
                if let Some(ref val) = val.1 {
                    self.add_val_to_arg(a, OsStr::new(val), matcher)?;
                }
            }
        }
        Ok(())
    }
}

// Error, Help, and Version Methods
impl<'help, 'c> Parser<'help, 'c>
where
    'help: 'c,
{
    fn did_you_mean_error(&mut self, arg: u64, matcher: &mut ArgMatcher) -> ClapResult<()> {
        debugln!("Parser::did_you_mean_error: arg={}", arg);
        // Didn't match a flag or option
        let longs = self
            .app
            .args
            .keys
            .iter()
            .map(|x| &x.key)
            .filter_map(|x| match x {
                KeyType::Long(l) => Some(l.to_string_lossy().into_owned()),
                _ => None,
            })
            .collect::<Vec<_>>();
        debugln!("Parser::did_you_mean_error: longs={:?}", longs);

        let suffix = suggestions::did_you_mean_flag_suffix(
            arg,
            longs.iter().map(|ref x| &x[..]),
            self.app.subcommands.as_mut_slice(),
        );

        // Add the arg to the matches to build a proper usage string
        if let Some(ref name) = suffix.1 {
            if let Some(opt) = self.app.args.get(&KeyType::Long(OsString::from(name))) {
                for g in groups_for_arg!(self.app, &opt.name) {
                    matcher.inc_occurrence_of(g);
                }
                matcher.insert(&*opt.name);
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
            &*Usage::new(self).create_usage_with_title(&*used),
            self.app.color(),
        ))
    }

    // Prints the version to the user and exits if quit=true
    fn print_version<W: Write>(&self, w: &mut W, use_long: bool) -> ClapResult<()> {
        self.app._write_version(w, use_long)?;
        w.flush().map_err(ClapError::from)
    }

    pub(crate) fn write_help_err<W: Write>(&self, w: &mut W) -> ClapResult<()> {
        Help::new(w, self, false, true).write_help()
    }

    fn help_err(&self, mut use_long: bool) -> ClapError {
        debugln!(
            "Parser::help_err: use_long={:?}",
            use_long && self.use_long_help()
        );
        use_long = use_long && self.use_long_help();
        let mut buf = vec![];
        match Help::new(&mut buf, self, use_long, false).write_help() {
            Err(e) => e,
            _ => ClapError {
                message: String::from_utf8(buf).unwrap_or_default(),
                kind: ErrorKind::HelpDisplayed,
                info: None,
            },
        }
    }

    fn version_err(&self, use_long: bool) -> ClapError {
        debugln!("Parser::version_err: ");
        let out = io::stdout();
        let mut buf_w = BufWriter::new(out.lock());
        match self.print_version(&mut buf_w, use_long) {
            Err(e) => e,
            _ => ClapError {
                message: String::new(),
                kind: ErrorKind::VersionDisplayed,
                info: None,
            },
        }
    }
}

// Query Methods
impl<'help, 'c> Parser<'help, 'c>
where
    'help: 'c,
{
    fn contains_short(&self, s: char) -> bool { self.app.contains_short(s) }

    #[cfg_attr(feature = "lints", allow(needless_borrow))]
    pub(crate) fn has_args(&self) -> bool { self.app.has_args() }

    pub(crate) fn has_opts(&self) -> bool { self.app.has_opts() }

    pub(crate) fn has_flags(&self) -> bool { self.app.has_flags() }

    pub(crate) fn has_positionals(&self) -> bool {
        self.app.args.keys.iter().any(|x| x.key.is_position())
    }

    pub(crate) fn has_subcommands(&self) -> bool { self.app.has_subcommands() }

    pub(crate) fn has_visible_subcommands(&self) -> bool { self.app.has_visible_subcommands() }

    pub(crate) fn is_set(&self, s: AS) -> bool { self.app.is_set(s) }

    pub(crate) fn set(&mut self, s: AS) { self.app.set(s) }

    pub(crate) fn unset(&mut self, s: AS) { self.app.unset(s) }
}
