// Std
use std::{
    cell::Cell,
    ffi::{OsStr, OsString},
};

// Internal
use crate::{
    build::AppSettings as AS,
    build::{App, Arg, ArgSettings},
    mkeymap::KeyType,
    output::{fmt::Colorizer, Help, HelpWriter, Usage},
    parse::errors::Error as ClapError,
    parse::errors::ErrorKind,
    parse::errors::Result as ClapResult,
    parse::features::suggestions,
    parse::{ArgMatcher, SubCommand},
    parse::{Validator, ValueType},
    util::{termcolor::ColorChoice, ArgStr, ChildGraph, Id},
    INTERNAL_ERROR_MSG, INVALID_UTF8,
};

#[derive(Debug, PartialEq, Clone)]
pub(crate) enum ParseResult {
    Flag,
    FlagSubCommand(String),
    // subcommand name, whether there are more shorts args remaining
    FlagSubCommandShort(String, bool),
    Opt(Id),
    Pos(Id),
    MaybeHyphenValue,
    NotFound,
    ValuesDone,
}

#[derive(Debug)]
pub(crate) struct Input {
    items: Vec<OsString>,
    cursor: usize,
}

impl<I, T> From<I> for Input
where
    I: Iterator<Item = T>,
    T: Into<OsString> + Clone,
{
    fn from(val: I) -> Self {
        Self {
            items: val.map(|x| x.into()).collect(),
            cursor: 0,
        }
    }
}

impl Input {
    pub(crate) fn next(&mut self) -> Option<(&OsStr, &[OsString])> {
        if self.cursor >= self.items.len() {
            None
        } else {
            let current = &self.items[self.cursor];
            self.cursor += 1;
            let remaining = &self.items[self.cursor..];
            Some((current, remaining))
        }
    }

    /// Insert some items to the Input items just after current parsing cursor.
    /// Usually used by replaced items recovering.
    pub(crate) fn insert(&mut self, insert_items: &[&str]) {
        self.items = {
            let mut new_items: Vec<OsString> = insert_items.iter().map(OsString::from).collect();
            for unparsed_item in &self.items[self.cursor..] {
                new_items.push(unparsed_item.clone());
            }
            new_items
        };
        self.cursor = 0;
    }
}

pub(crate) struct Parser<'help, 'app> {
    pub(crate) app: &'app mut App<'help>,
    pub(crate) required: ChildGraph<Id>,
    pub(crate) overridden: Vec<Id>,
    pub(crate) seen: Vec<Id>,
    pub(crate) cur_idx: Cell<usize>,
    pub(crate) skip_idxs: usize,
}

// Initializing Methods
impl<'help, 'app> Parser<'help, 'app> {
    pub(crate) fn new(app: &'app mut App<'help>) -> Self {
        let mut reqs = ChildGraph::with_capacity(5);
        for a in app
            .args
            .args()
            .filter(|a| a.settings.is_set(ArgSettings::Required))
        {
            reqs.insert(a.id.clone());
        }

        Parser {
            app,
            required: reqs,
            overridden: Vec::new(),
            seen: Vec::new(),
            cur_idx: Cell::new(0),
            skip_idxs: 0,
        }
    }

    #[cfg(debug_assertions)]
    fn _verify_positionals(&self) -> bool {
        debug!("Parser::_verify_positionals");
        // Because you must wait until all arguments have been supplied, this is the first chance
        // to make assertions on positional argument indexes
        //
        // First we verify that the index highest supplied index, is equal to the number of
        // positional arguments to verify there are no gaps (i.e. supplying an index of 1 and 3
        // but no 2)

        let highest_idx = self
            .app
            .args
            .keys()
            .filter_map(|x| {
                if let KeyType::Position(n) = x {
                    Some(*n)
                } else {
                    None
                }
            })
            .max()
            .unwrap_or(0);

        //_highest_idx(&self.positionals);

        let num_p = self.app.args.keys().filter(|x| x.is_position()).count() as u64;

        assert!(
            highest_idx == num_p,
            "Found positional argument whose index is {} but there \
             are only {} positional arguments defined",
            highest_idx,
            num_p
        );

        // Next we verify that only the highest index has a .multiple(true) (if any)
        let only_highest = |a: &Arg| {
            a.is_set(ArgSettings::MultipleValues) && (a.index.unwrap_or(0) != highest_idx)
        };
        if self.app.get_positionals().any(only_highest) {
            // First we make sure if there is a positional that allows multiple values
            // the one before it (second to last) has one of these:
            //  * a value terminator
            //  * ArgSettings::Last
            //  * The last arg is Required

            // We can't pass the closure (it.next()) to the macro directly because each call to
            // find() (iterator, not macro) gets called repeatedly.
            let last = &self.app.args[&KeyType::Position(highest_idx)];
            let second_to_last = &self.app.args[&KeyType::Position(highest_idx - 1)];

            // Either the final positional is required
            // Or the second to last has a terminator or .last(true) set
            let ok = last.is_set(ArgSettings::Required)
                || (second_to_last.terminator.is_some()
                    || second_to_last.is_set(ArgSettings::Last))
                || last.is_set(ArgSettings::Last);
            assert!(
                ok,
                "When using a positional argument with .multiple(true) that is *not the \
                 last* positional argument, the last positional argument (i.e. the one \
                 with the highest index) *must* have .required(true) or .last(true) set."
            );

            // We make sure if the second to last is Multiple the last is ArgSettings::Last
            let ok = second_to_last.is_set(ArgSettings::MultipleValues)
                || last.is_set(ArgSettings::Last);
            assert!(
                ok,
                "Only the last positional argument, or second to last positional \
                 argument may be set to .multiple(true)"
            );

            // Next we check how many have both Multiple and not a specific number of values set
            let count = self
                .app
                .get_positionals()
                .filter(|p| p.settings.is_set(ArgSettings::MultipleValues) && p.num_vals.is_none())
                .count();
            let ok = count <= 1
                || (last.is_set(ArgSettings::Last)
                    && last.is_set(ArgSettings::MultipleValues)
                    && second_to_last.is_set(ArgSettings::MultipleValues)
                    && count == 2);
            assert!(
                ok,
                "Only one positional argument with .multiple(true) set is allowed per \
                 command, unless the second one also has .last(true) set"
            );
        }

        if self.is_set(AS::AllowMissingPositional) {
            // Check that if a required positional argument is found, all positions with a lower
            // index are also required.
            let mut found = false;
            let mut foundx2 = false;

            for p in self.app.get_positionals() {
                if foundx2 && !p.is_set(ArgSettings::Required) {
                    assert!(
                        p.is_set(ArgSettings::Required),
                        "Found non-required positional argument with a lower \
                         index than a required positional argument by two or more: {:?} \
                         index {:?}",
                        p.name,
                        p.index
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
        } else {
            // Check that if a required positional argument is found, all positions with a lower
            // index are also required
            let mut found = false;
            for p in (1..=num_p).rev().filter_map(|n| self.app.args.get(&n)) {
                if found {
                    assert!(
                        p.is_set(ArgSettings::Required),
                        "Found non-required positional argument with a lower \
                         index than a required positional argument: {:?} index {:?}",
                        p.name,
                        p.index
                    );
                } else if p.is_set(ArgSettings::Required) && !p.is_set(ArgSettings::Last) {
                    // Args that .last(true) don't count since they can be required and have
                    // positionals with a lower index that aren't required
                    // Imagine: prog <req1> [opt1] -- <req2>
                    // Both of these are valid invocations:
                    //      $ prog r1 -- r2
                    //      $ prog r1 o1 -- r2
                    found = true;
                    continue;
                }
            }
        }
        assert!(
            self.app
                .get_positionals()
                .filter(|p| p.is_set(ArgSettings::Last))
                .count()
                < 2,
            "Only one positional argument may have last(true) set. Found two."
        );
        if self
            .app
            .get_positionals()
            .any(|p| p.is_set(ArgSettings::Last) && p.is_set(ArgSettings::Required))
            && self.has_subcommands()
            && !self.is_set(AS::SubcommandsNegateReqs)
        {
            panic!(
                "Having a required positional argument with .last(true) set *and* child \
                 subcommands without setting SubcommandsNegateReqs isn't compatible."
            );
        }

        true
    }

    #[allow(clippy::blocks_in_if_conditions)]
    // Does all the initializing and prepares the parser
    pub(crate) fn _build(&mut self) {
        debug!("Parser::_build");

        #[cfg(debug_assertions)]
        self._verify_positionals();

        for group in &self.app.groups {
            if group.required {
                let idx = self.required.insert(group.id.clone());
                for a in &group.requires {
                    self.required.insert_child(idx, a.clone());
                }
            }
        }
    }
}

// Parsing Methods
impl<'help, 'app> Parser<'help, 'app> {
    // The actual parsing function
    #[allow(clippy::cognitive_complexity)]
    pub(crate) fn get_matches_with(
        &mut self,
        matcher: &mut ArgMatcher,
        it: &mut Input,
    ) -> ClapResult<()> {
        debug!("Parser::get_matches_with");
        // Verify all positional assertions pass
        self._build();

        let mut subcmd_name: Option<String> = None;
        let mut keep_state = false;
        let mut external_subcommand = false;
        let mut needs_val_of = ParseResult::NotFound;
        let mut pos_counter = 1;
        let positional_count = self.app.args.keys().filter(|x| x.is_position()).count();

        while let Some((arg_os, remaining_args)) = it.next() {
            // Recover the replaced items if any.
            if let Some((_replacer, replaced_items)) = self
                .app
                .replacers
                .iter()
                .find(|(key, _)| OsStr::new(key) == arg_os)
            {
                it.insert(replaced_items);
                debug!(
                    "Parser::get_matches_with: found replacer: {:?}, target: {:?}",
                    _replacer, replaced_items
                );
                continue;
            }

            let arg_os = ArgStr::new(arg_os);
            debug!(
                "Parser::get_matches_with: Begin parsing '{:?}' ({:?})",
                arg_os,
                arg_os.as_raw_bytes()
            );

            // Is this a new argument, or a value for previous option?
            let starts_new_arg = self.is_new_arg(&arg_os, &needs_val_of);

            if !self.is_set(AS::TrailingValues) && arg_os == "--" && starts_new_arg {
                debug!("Parser::get_matches_with: setting TrailingVals=true");
                self.set(AS::TrailingValues);
                continue;
            }

            // Has the user already passed '--'? Meaning only positional args follow
            if !self.is_set(AS::TrailingValues) {
                // Does the arg match a subcommand name, or any of its aliases (if defined)
                match needs_val_of {
                    ParseResult::Opt(_) | ParseResult::Pos(_)
                        if !self.is_set(AS::SubcommandPrecedenceOverArg) => {}
                    _ => {
                        let sc_name = self.possible_subcommand(&arg_os);
                        debug!(
                            "Parser::get_matches_with: possible_sc={:?}, sc={:?}",
                            sc_name.is_some(),
                            sc_name
                        );
                        if let Some(sc_name) = sc_name {
                            if sc_name == "help" && !self.is_set(AS::NoAutoHelp) {
                                self.parse_help_subcommand(remaining_args)?;
                            }

                            subcmd_name = Some(sc_name.to_owned());
                            break;
                        }
                    }
                }

                if starts_new_arg {
                    if arg_os.starts_with("--") {
                        needs_val_of = self.parse_long_arg(matcher, &arg_os, remaining_args)?;
                        debug!(
                            "Parser::get_matches_with: After parse_long_arg {:?}",
                            needs_val_of
                        );
                        match needs_val_of {
                            ParseResult::Flag | ParseResult::Opt(..) | ParseResult::ValuesDone => {
                                continue;
                            }
                            ParseResult::FlagSubCommand(ref name) => {
                                debug!("Parser::get_matches_with: FlagSubCommand found in long arg {:?}", name);
                                subcmd_name = Some(name.to_owned());
                                break;
                            }
                            ParseResult::FlagSubCommandShort(_, _) => unreachable!(),
                            _ => (),
                        }
                    } else if arg_os.starts_with("-")
                        && arg_os.len() != 1
                        && !(self.is_set(AS::AllowNegativeNumbers)
                            && arg_os.to_string_lossy().parse::<f64>().is_ok())
                    {
                        // Arg looks like a short flag, and not a possible number

                        // Try to parse short args like normal, if AllowLeadingHyphen or
                        // AllowNegativeNumbers is set, parse_short_arg will *not* throw
                        // an error, and instead return Ok(None)
                        needs_val_of = self.parse_short_arg(matcher, &arg_os)?;
                        // If it's None, we then check if one of those two AppSettings was set
                        debug!(
                            "Parser::get_matches_with: After parse_short_arg {:?}",
                            needs_val_of
                        );
                        match needs_val_of {
                            ParseResult::Opt(..) | ParseResult::Flag | ParseResult::ValuesDone => {
                                continue;
                            }
                            ParseResult::FlagSubCommandShort(ref name, done) => {
                                // There are more short args, revist the current short args skipping the subcommand
                                keep_state = !done;
                                if keep_state {
                                    it.cursor -= 1;
                                    self.skip_idxs = self.cur_idx.get();
                                }

                                subcmd_name = Some(name.to_owned());
                                break;
                            }
                            ParseResult::FlagSubCommand(_) => unreachable!(),
                            _ => (),
                        }
                    }
                } else if let ParseResult::Opt(id) = needs_val_of {
                    // Check to see if parsing a value from a previous arg

                    // get the option so we can check the settings
                    needs_val_of = self.add_val_to_arg(
                        &self.app[&id],
                        &arg_os,
                        matcher,
                        ValueType::CommandLine,
                    )?;
                    // get the next value from the iterator
                    continue;
                }
            }

            let is_second_to_last = pos_counter + 1 == positional_count;

            // The last positional argument, or second to last positional
            // argument may be set to .multiple(true)
            let low_index_mults = is_second_to_last
                && self.app.get_positionals().any(|a| {
                    a.is_set(ArgSettings::MultipleValues)
                        && (positional_count != a.index.unwrap_or(0) as usize)
                })
                && self
                    .app
                    .get_positionals()
                    .last()
                    .map_or(false, |p_name| !p_name.is_set(ArgSettings::Last));

            let missing_pos = self.is_set(AS::AllowMissingPositional)
                && is_second_to_last
                && !self.is_set(AS::TrailingValues);

            debug!(
                "Parser::get_matches_with: Positional counter...{}",
                pos_counter
            );
            debug!(
                "Parser::get_matches_with: Low index multiples...{:?}",
                low_index_mults
            );

            if low_index_mults || missing_pos {
                if let Some(n) = remaining_args.get(0) {
                    needs_val_of = match needs_val_of {
                        ParseResult::ValuesDone => ParseResult::ValuesDone,
                        _ => {
                            if let Some(p) = self
                                .app
                                .get_positionals()
                                .find(|p| p.index == Some(pos_counter as u64))
                            {
                                ParseResult::Pos(p.id.clone())
                            } else {
                                ParseResult::ValuesDone
                            }
                        }
                    };

                    let n = ArgStr::new(n);
                    let sc_match = { self.possible_subcommand(&n).is_some() };

                    if self.is_new_arg(&n, &needs_val_of)
                        || sc_match
                        || !suggestions::did_you_mean(
                            &n.to_string_lossy(),
                            self.app.all_subcommand_names(),
                        )
                        .is_empty()
                    {
                        debug!("Parser::get_matches_with: Bumping the positional counter...");
                        pos_counter += 1;
                    }
                } else {
                    debug!("Parser::get_matches_with: Bumping the positional counter...");
                    pos_counter += 1;
                }
            } else if self.is_set(AS::TrailingValues)
                && (self.is_set(AS::AllowMissingPositional) || self.is_set(AS::ContainsLast))
            {
                // Came to -- and one positional has .last(true) set, so we go immediately
                // to the last (highest index) positional
                debug!("Parser::get_matches_with: .last(true) and --, setting last pos");
                pos_counter = positional_count;
            }

            if let Some(p) = self
                .app
                .args
                .args()
                .find(|a| a.is_positional() && a.index == Some(pos_counter as u64))
            {
                if p.is_set(ArgSettings::Last) && !self.is_set(AS::TrailingValues) {
                    return Err(ClapError::unknown_argument(
                        arg_os.to_string_lossy().to_string(),
                        None,
                        Usage::new(self).create_usage_with_title(&[]),
                        self.app.color(),
                    ));
                }

                if !self.is_set(AS::TrailingValues)
                    && (self.is_set(AS::TrailingVarArg) && pos_counter == positional_count)
                {
                    self.app.settings.set(AS::TrailingValues);
                }

                self.seen.push(p.id.clone());
                self.add_val_to_arg(p, &arg_os, matcher, ValueType::CommandLine)?;

                matcher.inc_occurrence_of(&p.id);
                for grp in self.app.groups_for_arg(&p.id) {
                    matcher.inc_occurrence_of(&grp);
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
                        if self.is_set(AS::StrictUtf8) {
                            return Err(ClapError::invalid_utf8(
                                Usage::new(self).create_usage_with_title(&[]),
                                self.app.color(),
                            ));
                        }
                        arg_os.to_string_lossy().into_owned()
                    }
                };

                // Collect the external subcommand args
                let mut sc_m = ArgMatcher::default();

                while let Some((v, _)) = it.next() {
                    if self.is_set(AS::StrictUtf8) && v.to_str().is_none() {
                        return Err(ClapError::invalid_utf8(
                            Usage::new(self).create_usage_with_title(&[]),
                            self.app.color(),
                        ));
                    }
                    sc_m.add_val_to(&Id::empty_hash(), v.to_os_string(), ValueType::CommandLine);
                }

                matcher.subcommand(SubCommand {
                    name: sc_name.clone(),
                    id: sc_name.into(),
                    matches: sc_m.into_inner(),
                });

                external_subcommand = true;

                break;
            } else {
                // Start error processing

                // If argument follows a `--`
                if self.is_set(AS::TrailingValues) {
                    // If the arg matches a subcommand name, or any of its aliases (if defined)
                    if self.possible_subcommand(&arg_os).is_some() {
                        return Err(ClapError::unnecessary_double_dash(
                            arg_os.to_string_lossy().to_string(),
                            Usage::new(self).create_usage_with_title(&[]),
                            self.app.color(),
                        ));
                    }
                } else {
                    let cands = suggestions::did_you_mean(
                        &arg_os.to_string_lossy(),
                        self.app.all_subcommand_names(),
                    );
                    // If the argument looks like a subcommand.
                    if !cands.is_empty() {
                        let cands: Vec<_> =
                            cands.iter().map(|cand| format!("'{}'", cand)).collect();
                        return Err(ClapError::invalid_subcommand(
                            arg_os.to_string_lossy().to_string(),
                            cands.join(" or "),
                            self.app
                                .bin_name
                                .as_ref()
                                .unwrap_or(&self.app.name)
                                .to_string(),
                            Usage::new(self).create_usage_with_title(&[]),
                            self.app.color(),
                        ));
                    }
                    // If the argument must be a subcommand.
                    if !self.has_args()
                        || self.is_set(AS::InferSubcommands) && self.has_subcommands()
                    {
                        return Err(ClapError::unrecognized_subcommand(
                            arg_os.to_string_lossy().to_string(),
                            self.app
                                .bin_name
                                .as_ref()
                                .unwrap_or(&self.app.name)
                                .to_string(),
                            self.app.color(),
                        ));
                    }
                }
                return Err(ClapError::unknown_argument(
                    arg_os.to_string_lossy().to_string(),
                    None,
                    Usage::new(self).create_usage_with_title(&[]),
                    self.app.color(),
                ));
            }
        }

        if !external_subcommand {
            if let Some(ref pos_sc_name) = subcmd_name {
                let sc_name = self
                    .app
                    .find_subcommand(pos_sc_name)
                    .expect(INTERNAL_ERROR_MSG)
                    .name
                    .clone();
                self.parse_subcommand(&sc_name, matcher, it, keep_state)?;
            } else if self.is_set(AS::SubcommandRequired) {
                let bn = self.app.bin_name.as_ref().unwrap_or(&self.app.name);
                return Err(ClapError::missing_subcommand(
                    bn.to_string(),
                    Usage::new(self).create_usage_with_title(&[]),
                    self.app.color(),
                ));
            } else if self.is_set(AS::SubcommandRequiredElseHelp) {
                debug!("Parser::get_matches_with: SubcommandRequiredElseHelp=true");
                let message = self.write_help_err()?;
                return Err(ClapError {
                    message,
                    kind: ErrorKind::DisplayHelpOnMissingArgumentOrSubcommand,
                    info: vec![],
                    source: None,
                });
            }
        }

        self.remove_overrides(matcher);

        Validator::new(self).validate(needs_val_of, subcmd_name.is_some(), matcher)
    }

    // Should we color the help?
    pub(crate) fn color_help(&self) -> ColorChoice {
        debug!("Parser::color_help");

        if self.is_set(AS::ColoredHelp) {
            self.app.color()
        } else {
            ColorChoice::Never
        }
    }

    // Checks if the arg matches a subcommand name, or any of its aliases (if defined)
    fn possible_subcommand(&self, arg_os: &ArgStr) -> Option<&str> {
        debug!("Parser::possible_subcommand: arg={:?}", arg_os);

        if self.is_set(AS::ArgsNegateSubcommands) && self.is_set(AS::ValidArgFound) {
            return None;
        }

        if self.is_set(AS::InferSubcommands) {
            let v = self
                .app
                .all_subcommand_names()
                .filter(|s| arg_os.is_prefix_of(s))
                .collect::<Vec<_>>();

            if v.len() == 1 {
                return Some(v[0]);
            }

            for sc in &v {
                if sc == arg_os {
                    return Some(sc);
                }
            }
        } else if let Some(sc) = self.app.find_subcommand(arg_os) {
            return Some(&sc.name);
        }

        None
    }

    // Checks if the arg matches a long flag subcommand name, or any of its aliases (if defined)
    fn possible_long_flag_subcommand(&self, arg_os: &ArgStr) -> Option<&str> {
        debug!("Parser::possible_long_flag_subcommand: arg={:?}", arg_os);
        if self.is_set(AS::InferSubcommands) {
            let options = self
                .app
                .get_subcommands()
                .fold(Vec::new(), |mut options, sc| {
                    if let Some(long) = sc.long_flag {
                        if arg_os.is_prefix_of(long) {
                            options.push(long);
                        }
                        options.extend(
                            sc.get_all_aliases()
                                .filter(|alias| arg_os.is_prefix_of(alias)),
                        )
                    }
                    options
                });
            if options.len() == 1 {
                return Some(options[0]);
            }

            for sc in &options {
                if sc == arg_os {
                    return Some(sc);
                }
            }
        } else if let Some(sc_name) = self.app.find_long_subcmd(arg_os) {
            return Some(sc_name);
        }
        None
    }

    fn parse_help_subcommand(&self, cmds: &[OsString]) -> ClapResult<ParseResult> {
        debug!("Parser::parse_help_subcommand");

        let mut help_help = false;
        let mut bin_name = self.app.bin_name.as_ref().unwrap_or(&self.app.name).clone();

        let mut sc = {
            // @TODO @perf: cloning all these Apps isn't great, but since it's just displaying the
            // help message there are bigger fish to fry
            let mut sc = self.app.clone();

            for (i, cmd) in cmds.iter().enumerate() {
                if cmd == OsStr::new("help") {
                    // cmd help help
                    help_help = true;
                    break; // Maybe?
                }

                if let Some(mut c) = sc.find_subcommand(cmd).cloned() {
                    c._build();
                    sc = c;

                    if i == cmds.len() - 1 {
                        break;
                    }
                } else if let Some(mut c) = sc.find_subcommand(&cmd.to_string_lossy()).cloned() {
                    c._build();
                    sc = c;

                    if i == cmds.len() - 1 {
                        break;
                    }
                } else {
                    return Err(ClapError::unrecognized_subcommand(
                        cmd.to_string_lossy().to_string(),
                        self.app
                            .bin_name
                            .as_ref()
                            .unwrap_or(&self.app.name)
                            .to_string(),
                        self.app.color(),
                    ));
                }

                bin_name = format!("{} {}", bin_name, &sc.name);
            }

            sc
        };

        let parser = Parser::new(&mut sc);

        if help_help {
            let mut pb = Arg::new("subcommand")
                .index(1)
                .setting(ArgSettings::MultipleValues)
                .about("The subcommand whose help message to display");

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

    fn is_new_arg(&mut self, arg_os: &ArgStr, last_result: &ParseResult) -> bool {
        debug!("Parser::is_new_arg: {:?}:{:?}", arg_os, last_result);

        let want_value = match last_result {
            ParseResult::Opt(name) | ParseResult::Pos(name) => {
                self.is_set(AS::AllowLeadingHyphen)
                    || self.app[name].is_set(ArgSettings::AllowHyphenValues)
                    || (self.is_set(AS::AllowNegativeNumbers)
                        && arg_os.to_string_lossy().parse::<f64>().is_ok())
            }
            ParseResult::ValuesDone => return true,
            _ => false,
        };

        debug!("Parser::is_new_arg: want_value={:?}", want_value);

        // Is this a new argument, or values from a previous option?
        if want_value {
            false
        } else if arg_os.starts_with("--") {
            debug!("Parser::is_new_arg: -- found");
            true
        } else if arg_os.starts_with("-") {
            debug!("Parser::is_new_arg: - found");
            // a singe '-' by itself is a value and typically means "stdin" on unix systems
            arg_os.len() != 1
        } else {
            debug!("Parser::is_new_arg: value");
            false
        }
    }

    fn parse_subcommand(
        &mut self,
        sc_name: &str,
        matcher: &mut ArgMatcher,
        it: &mut Input,
        keep_state: bool,
    ) -> ClapResult<()> {
        use std::fmt::Write;

        debug!("Parser::parse_subcommand");

        let mut mid_string = String::from(" ");

        if !self.is_set(AS::SubcommandsNegateReqs) {
            let reqs = Usage::new(self).get_required_usage_from(&[], None, true); // maybe Some(m)

            for s in &reqs {
                write!(&mut mid_string, "{} ", s).expect(INTERNAL_ERROR_MSG);
            }
        }

        if let Some(sc) = self.app.subcommands.iter_mut().find(|s| s.name == sc_name) {
            let mut sc_matcher = ArgMatcher::default();
            // Display subcommand name, short and long in usage
            let mut sc_names = sc.name.clone();
            let mut flag_subcmd = false;
            if let Some(l) = sc.long_flag {
                sc_names.push_str(&format!(", --{}", l));
                flag_subcmd = true;
            }
            if let Some(s) = sc.short_flag {
                sc_names.push_str(&format!(", -{}", s));
                flag_subcmd = true;
            }

            if flag_subcmd {
                sc_names = format!("{{{}}}", sc_names);
            }

            sc.usage = Some(
                self.app
                    .bin_name
                    .as_ref()
                    .map(|bin_name| format!("{}{}{}", bin_name, mid_string, sc_names))
                    .unwrap_or(sc_names),
            );

            // bin_name should be parent's bin_name + [<reqs>] + the sc's name separated by
            // a space
            sc.bin_name = Some(format!(
                "{}{}{}",
                self.app.bin_name.as_ref().unwrap_or(&String::new()),
                if self.app.bin_name.is_some() { " " } else { "" },
                &*sc.name
            ));

            // Ensure all args are built and ready to parse
            sc._build();

            debug!("Parser::parse_subcommand: About to parse sc={}", sc.name);

            {
                let mut p = Parser::new(sc);
                // HACK: maintain indexes between parsers
                // FlagSubCommand short arg needs to revist the current short args, but skip the subcommand itself
                if keep_state {
                    p.cur_idx.set(self.cur_idx.get());
                    p.skip_idxs = self.skip_idxs;
                }
                p.get_matches_with(&mut sc_matcher, it)?;
            }
            let name = sc.name.clone();
            matcher.subcommand(SubCommand {
                id: Id::from_ref(&name), // @TODO @maybe: should be sc.id?
                name,
                matches: sc_matcher.into_inner(),
            });
        }
        Ok(())
    }

    // Retrieves the names of all args the user has supplied thus far, except required ones
    // because those will be listed in self.required
    fn check_for_help_and_version_str(&self, arg: &ArgStr) -> ClapResult<()> {
        debug!("Parser::check_for_help_and_version_str");
        debug!(
            "Parser::check_for_help_and_version_str: Checking if --{:?} is help or version...",
            arg
        );

        if let Some(help) = self.app.find(&Id::help_hash()) {
            if let Some(h) = help.long {
                if arg == h && !self.is_set(AS::NoAutoHelp) {
                    debug!("Help");
                    return Err(self.help_err(true));
                }
            }
        }

        if let Some(version) = self.app.find(&Id::version_hash()) {
            if let Some(v) = version.long {
                if arg == v && !self.is_set(AS::NoAutoVersion) {
                    debug!("Version");
                    return Err(self.version_err(true));
                }
            }
        }

        debug!("Neither");
        Ok(())
    }

    fn check_for_help_and_version_char(&self, arg: char) -> ClapResult<()> {
        debug!("Parser::check_for_help_and_version_char");
        debug!(
            "Parser::check_for_help_and_version_char: Checking if -{} is help or version...",
            arg
        );

        if let Some(help) = self.app.find(&Id::help_hash()) {
            if let Some(h) = help.short {
                if arg == h && !self.is_set(AS::NoAutoHelp) {
                    debug!("Help");
                    return Err(self.help_err(false));
                }
            }
        }

        if let Some(version) = self.app.find(&Id::version_hash()) {
            if let Some(v) = version.short {
                if arg == v && !self.is_set(AS::NoAutoVersion) {
                    debug!("Version");
                    return Err(self.version_err(false));
                }
            }
        }

        debug!("Neither");
        Ok(())
    }

    fn use_long_help(&self) -> bool {
        debug!("Parser::use_long_help");
        // In this case, both must be checked. This allows the retention of
        // original formatting, but also ensures that the actual -h or --help
        // specified by the user is sent through. If HiddenShortHelp is not included,
        // then items specified with hidden_short_help will also be hidden.
        let should_long = |v: &Arg| {
            v.long_about.is_some()
                || v.is_set(ArgSettings::HiddenLongHelp)
                || v.is_set(ArgSettings::HiddenShortHelp)
        };

        self.app.long_about.is_some()
            || self.app.before_long_help.is_some()
            || self.app.after_long_help.is_some()
            || self.app.args.args().any(should_long)
            || self.app.subcommands.iter().any(|s| s.long_about.is_some())
    }

    fn parse_long_arg(
        &mut self,
        matcher: &mut ArgMatcher,
        full_arg: &ArgStr,
        remaining_args: &[OsString],
    ) -> ClapResult<ParseResult> {
        // maybe here lifetime should be 'a
        debug!("Parser::parse_long_arg");

        // Update the curent index
        self.cur_idx.set(self.cur_idx.get() + 1);

        debug!("Parser::parse_long_arg: Does it contain '='...");
        let matches;
        let (arg, val) = if full_arg.contains_byte(b'=') {
            matches = full_arg.trim_start_matches(b'-');
            let (p0, p1) = matches.split_at_byte(b'=');
            debug!("Yes '{:?}'", p1);
            (p0, Some(p1))
        } else {
            debug!("No");
            (full_arg.trim_start_matches(b'-'), None)
        };
        if let Some(opt) = self.app.args.get(&arg.to_os_string()) {
            debug!(
                "Parser::parse_long_arg: Found valid opt or flag '{}'",
                opt.to_string()
            );
            self.app.settings.set(AS::ValidArgFound);

            self.seen.push(opt.id.clone());

            if opt.is_set(ArgSettings::TakesValue) {
                return Ok(self.parse_opt(&val, opt, val.is_some(), matcher)?);
            }
            self.check_for_help_and_version_str(&arg)?;
            self.parse_flag(opt, matcher)?;

            return Ok(ParseResult::Flag);
        } else if let Some(sc_name) = self.possible_long_flag_subcommand(&arg) {
            return Ok(ParseResult::FlagSubCommand(sc_name.to_string()));
        } else if self.is_set(AS::AllowLeadingHyphen) {
            return Ok(ParseResult::MaybeHyphenValue);
        }

        debug!("Parser::parse_long_arg: Didn't match anything");
        let remaining_args: Vec<_> = remaining_args
            .iter()
            .map(|x| x.to_str().expect(INVALID_UTF8))
            .collect();
        self.did_you_mean_error(arg.to_str().expect(INVALID_UTF8), matcher, &remaining_args)
            .map(|_| ParseResult::NotFound)
    }

    fn parse_short_arg(
        &mut self,
        matcher: &mut ArgMatcher,
        full_arg: &ArgStr,
    ) -> ClapResult<ParseResult> {
        debug!("Parser::parse_short_arg: full_arg={:?}", full_arg);
        let arg_os = full_arg.trim_start_matches(b'-');
        let arg = arg_os.to_string_lossy();

        // If AllowLeadingHyphen is set, we want to ensure `-val` gets parsed as `-val` and not
        // `-v` `-a` `-l` assuming `v` `a` and `l` are all, or mostly, valid shorts.
        if self.is_set(AS::AllowLeadingHyphen) && arg.chars().any(|c| !self.contains_short(c)) {
            debug!(
                "Parser::parse_short_arg: LeadingHyphenAllowed yet -{} isn't valid",
                arg
            );
            return Ok(ParseResult::MaybeHyphenValue);
        }

        let mut ret = ParseResult::NotFound;
        let skip = self.skip_idxs;
        self.skip_idxs = 0;
        for c in arg.chars().skip(skip) {
            debug!("Parser::parse_short_arg:iter:{}", c);

            // update each index because `-abcd` is four indices to clap
            self.cur_idx.set(self.cur_idx.get() + 1);

            // Check for matching short options, and return the name if there is no trailing
            // concatenated value: -oval
            // Option: -o
            // Value: val
            if let Some(opt) = self.app.args.get(&c) {
                debug!(
                    "Parser::parse_short_arg:iter:{}: Found valid opt or flag",
                    c
                );
                self.app.settings.set(AS::ValidArgFound);
                self.seen.push(opt.id.clone());
                if !opt.is_set(ArgSettings::TakesValue) {
                    self.check_for_help_and_version_char(c)?;
                    ret = self.parse_flag(opt, matcher)?;
                    continue;
                }

                // Check for trailing concatenated value
                let i = arg_os.split(c).next().unwrap().len() + c.len_utf8();
                debug!(
                    "Parser::parse_short_arg:iter:{}: i={}, arg_os={:?}",
                    c, i, arg_os
                );
                let val = if i != arg_os.len() {
                    // This is always a valid place to split, because the separator is UTF-8.
                    let val = arg_os.split_at_unchecked(i).1;
                    debug!(
                        "Parser::parse_short_arg:iter:{}: val={:?} (bytes), val={:?} (ascii)",
                        c,
                        val.as_raw_bytes(),
                        val
                    );
                    Some(val)
                } else {
                    None
                };

                // Default to "we're expecting a value later"
                return self.parse_opt(&val, opt, false, matcher);
            } else if let Some(sc_name) = self.app.find_short_subcmd(c) {
                debug!("Parser::parse_short_arg:iter:{}: subcommand={}", c, sc_name);
                let name = sc_name.to_string();
                let done_short_args = self.cur_idx.get() == arg.len();
                return Ok(ParseResult::FlagSubCommandShort(name, done_short_args));
            } else {
                let arg = format!("-{}", c);

                return Err(ClapError::unknown_argument(
                    arg,
                    None,
                    Usage::new(self).create_usage_with_title(&[]),
                    self.app.color(),
                ));
            }
        }
        Ok(ret)
    }

    fn parse_opt(
        &self,
        val: &Option<ArgStr>,
        opt: &Arg<'help>,
        had_eq: bool,
        matcher: &mut ArgMatcher,
    ) -> ClapResult<ParseResult> {
        debug!("Parser::parse_opt; opt={}, val={:?}", opt.name, val);
        debug!("Parser::parse_opt; opt.settings={:?}", opt.settings);
        let mut has_eq = false;
        let no_val = val.is_none();
        let empty_vals = opt.is_set(ArgSettings::AllowEmptyValues);
        let min_vals_zero = opt.min_vals.unwrap_or(1) == 0;
        let needs_eq = opt.is_set(ArgSettings::RequireEquals);

        debug!("Parser::parse_opt; Checking for val...");
        if let Some(fv) = val {
            has_eq = fv.starts_with("=") || had_eq;
            let v = fv.trim_start_n_matches(1, b'=');
            if !empty_vals && (v.is_empty() || (needs_eq && !has_eq)) {
                debug!("Found Empty - Error");
                return Err(ClapError::empty_value(
                    opt,
                    Usage::new(self).create_usage_with_title(&[]),
                    self.app.color(),
                ));
            }
            debug!("Found - {:?}, len: {}", v, v.len());
            debug!(
                "Parser::parse_opt: {:?} contains '='...{:?}",
                fv,
                fv.starts_with("=")
            );
            self.add_val_to_arg(opt, &v, matcher, ValueType::CommandLine)?;
        } else if needs_eq && !(empty_vals || min_vals_zero) {
            debug!("None, but requires equals...Error");
            return Err(ClapError::empty_value(
                opt,
                Usage::new(self).create_usage_with_title(&[]),
                self.app.color(),
            ));
        } else if needs_eq && min_vals_zero {
            debug!("None and requires equals, but min_vals == 0");
            if !opt.default_missing_vals.is_empty() {
                debug!("Parser::parse_opt: has default_missing_vals");
                for val in &opt.default_missing_vals {
                    debug!(
                        "Parser::parse_opt: adding value from default_missing_values; val = {:?}",
                        val
                    );
                    self.add_val_to_arg(opt, &ArgStr::new(val), matcher, ValueType::CommandLine)?;
                }
            };
        } else {
            debug!("None");
        }

        matcher.inc_occurrence_of(&opt.id);
        // Increment or create the group "args"
        for grp in self.app.groups_for_arg(&opt.id) {
            matcher.inc_occurrence_of(&grp);
        }

        let needs_delim = opt.is_set(ArgSettings::RequireDelimiter);
        let mult = opt.is_set(ArgSettings::MultipleValues);
        // @TODO @soundness: if doesn't have an equal, but requires equal is ValuesDone?!
        if no_val && min_vals_zero && !has_eq && needs_eq {
            debug!("Parser::parse_opt: More arg vals not required...");
            return Ok(ParseResult::ValuesDone);
        } else if no_val || (mult && !needs_delim) && !has_eq && matcher.needs_more_vals(opt) {
            debug!("Parser::parse_opt: More arg vals required...");
            return Ok(ParseResult::Opt(opt.id.clone()));
        }
        debug!("Parser::parse_opt: More arg vals not required...");
        Ok(ParseResult::ValuesDone)
    }

    fn add_val_to_arg(
        &self,
        arg: &Arg<'help>,
        val: &ArgStr,
        matcher: &mut ArgMatcher,
        ty: ValueType,
    ) -> ClapResult<ParseResult> {
        debug!("Parser::add_val_to_arg; arg={}, val={:?}", arg.name, val);
        debug!(
            "Parser::add_val_to_arg; trailing_vals={:?}, DontDelimTrailingVals={:?}",
            self.is_set(AS::TrailingValues),
            self.is_set(AS::DontDelimitTrailingValues)
        );
        if !(self.is_set(AS::TrailingValues) && self.is_set(AS::DontDelimitTrailingValues)) {
            if let Some(delim) = arg.val_delim {
                if val.is_empty() {
                    Ok(self.add_single_val_to_arg(arg, val, matcher, ty)?)
                } else {
                    let mut iret = ParseResult::ValuesDone;
                    for v in val.split(delim) {
                        iret = self.add_single_val_to_arg(arg, &v, matcher, ty)?;
                    }
                    // If there was a delimiter used, we're not looking for more values
                    if val.contains_char(delim) || arg.is_set(ArgSettings::RequireDelimiter) {
                        iret = ParseResult::ValuesDone;
                    }
                    Ok(iret)
                }
            } else {
                self.add_single_val_to_arg(arg, val, matcher, ty)
            }
        } else {
            self.add_single_val_to_arg(arg, val, matcher, ty)
        }
    }

    fn add_single_val_to_arg(
        &self,
        arg: &Arg<'help>,
        v: &ArgStr,
        matcher: &mut ArgMatcher,
        ty: ValueType,
    ) -> ClapResult<ParseResult> {
        debug!("Parser::add_single_val_to_arg: adding val...{:?}", v);

        // update the current index because each value is a distinct index to clap
        self.cur_idx.set(self.cur_idx.get() + 1);

        // @TODO @docs @p4 docs should probably note that terminator doesn't get an index
        if let Some(t) = arg.terminator {
            if t == v {
                return Ok(ParseResult::ValuesDone);
            }
        }

        matcher.add_val_to(&arg.id, v.to_os_string(), ty);
        matcher.add_index_to(&arg.id, self.cur_idx.get(), ty);

        // Increment or create the group "args"
        for grp in self.app.groups_for_arg(&arg.id) {
            matcher.add_val_to(&grp, v.to_os_string(), ty);
        }

        if matcher.needs_more_vals(arg) {
            return Ok(ParseResult::Opt(arg.id.clone()));
        }
        Ok(ParseResult::ValuesDone)
    }

    fn parse_flag(&self, flag: &Arg<'help>, matcher: &mut ArgMatcher) -> ClapResult<ParseResult> {
        debug!("Parser::parse_flag");

        matcher.inc_occurrence_of(&flag.id);
        matcher.add_index_to(&flag.id, self.cur_idx.get(), ValueType::CommandLine);
        // Increment or create the group "args"
        for grp in self.app.groups_for_arg(&flag.id) {
            matcher.inc_occurrence_of(&grp);
        }

        Ok(ParseResult::Flag)
    }

    fn remove_overrides(&mut self, matcher: &mut ArgMatcher) {
        debug!("Parser::remove_overrides");
        let mut to_rem: Vec<Id> = Vec::new();
        let mut self_override: Vec<Id> = Vec::new();
        let mut arg_overrides = Vec::new();
        for name in matcher.arg_names() {
            debug!("Parser::remove_overrides:iter:{:?}", name);
            if let Some(arg) = self.app.find(name) {
                let mut handle_self_override = |o: &Id| {
                    if (arg.is_set(ArgSettings::MultipleValues)
                        || arg.is_set(ArgSettings::MultipleOccurrences))
                        || !arg.has_switch()
                    {
                        return true;
                    }
                    debug!(
                        "Parser::remove_overrides:iter:{:?}:iter:{:?}: self override",
                        name, o
                    );
                    self_override.push(o.clone());
                    false
                };
                for o in &arg.overrides {
                    debug!("Parser::remove_overrides:iter:{:?} => {:?}", name, o);
                    if *o == arg.id {
                        if handle_self_override(o) {
                            continue;
                        }
                    } else {
                        arg_overrides.push((arg.id.clone(), o));
                        arg_overrides.push((o.clone(), &arg.id));
                    }
                }
                if self.is_set(AS::AllArgsOverrideSelf) {
                    let _ = handle_self_override(&arg.id);
                }
            }
        }

        // remove future overrides in reverse seen order
        for arg in self.seen.iter().rev() {
            for (a, overr) in arg_overrides.iter().filter(|(a, _)| a == arg) {
                if !to_rem.contains(&a) {
                    to_rem.push((*overr).clone());
                }
            }
        }

        // Do self overrides
        for name in &self_override {
            debug!("Parser::remove_overrides:iter:self:{:?}: resetting", name);
            if let Some(ma) = matcher.get_mut(name) {
                if ma.occurs < 2 {
                    continue;
                }
                ma.occurs = 1;

                let len = ma.vals.len().saturating_sub(1);
                ma.vals.drain(0..len);
            }
        }

        // Finally remove conflicts
        for name in &to_rem {
            debug!("Parser::remove_overrides:iter:{:?}: removing", name);
            matcher.remove(name);
            self.overridden.push(name.clone());
        }
    }

    pub(crate) fn add_defaults(&mut self, matcher: &mut ArgMatcher) -> ClapResult<()> {
        debug!("Parser::add_defaults");

        for o in self.app.get_opts() {
            debug!("Parser::add_defaults:iter:{}:", o.name);
            self.add_value(o, matcher, ValueType::DefaultValue)?;
        }

        for p in self.app.get_positionals() {
            debug!("Parser::add_defaults:iter:{}:", p.name);
            self.add_value(p, matcher, ValueType::DefaultValue)?;
        }

        Ok(())
    }

    fn add_value(
        &self,
        arg: &Arg<'help>,
        matcher: &mut ArgMatcher,
        ty: ValueType,
    ) -> ClapResult<()> {
        if !arg.default_vals_ifs.is_empty() {
            debug!("Parser::add_value: has conditional defaults");

            let mut done = false;
            if matcher.get(&arg.id).is_none() {
                for (id, val, default) in arg.default_vals_ifs.values() {
                    let add = if let Some(a) = matcher.get(&id) {
                        if let Some(v) = val {
                            a.vals.iter().any(|value| v == value)
                        } else {
                            true
                        }
                    } else {
                        false
                    };

                    if add {
                        self.add_val_to_arg(arg, &ArgStr::new(default), matcher, ty)?;
                        done = true;
                        break;
                    }
                }
            }

            if done {
                return Ok(());
            }
        } else {
            debug!("Parser::add_value: doesn't have conditional defaults");
        }

        if !arg.default_vals.is_empty() {
            debug!("Parser::add_value:iter:{}: has default vals", arg.name);
            if matcher.get(&arg.id).is_some() {
                debug!("Parser::add_value:iter:{}: was used", arg.name);
            // do nothing
            } else {
                debug!("Parser::add_value:iter:{}: wasn't used", arg.name);

                for val in &arg.default_vals {
                    self.add_val_to_arg(arg, &ArgStr::new(val), matcher, ty)?;
                }
            }
        } else {
            debug!(
                "Parser::add_value:iter:{}: doesn't have default vals",
                arg.name
            );

            // do nothing
        }

        if !arg.default_missing_vals.is_empty() {
            debug!(
                "Parser::add_value:iter:{}: has default missing vals",
                arg.name
            );
            match matcher.get(&arg.id) {
                Some(ma) if ma.vals.is_empty() => {
                    debug!(
                        "Parser::add_value:iter:{}: has no user defined vals",
                        arg.name
                    );
                    for val in &arg.default_missing_vals {
                        self.add_val_to_arg(arg, &ArgStr::new(val), matcher, ty)?;
                    }
                }
                None => {
                    debug!("Parser::add_value:iter:{}: wasn't used", arg.name);
                    // do nothing
                }
                _ => {
                    debug!("Parser::add_value:iter:{}: has user defined vals", arg.name);
                    // do nothing
                }
            }
        } else {
            debug!(
                "Parser::add_value:iter:{}: doesn't have default missing vals",
                arg.name
            );

            // do nothing
        }

        Ok(())
    }

    pub(crate) fn add_env(&mut self, matcher: &mut ArgMatcher) -> ClapResult<()> {
        for a in self.app.args.args() {
            // Use env only if the arg was not present among command line args
            if matcher.get(&a.id).map_or(true, |a| a.occurs == 0) {
                if let Some((_, Some(ref val))) = a.env {
                    let val = &ArgStr::new(val);
                    if a.is_set(ArgSettings::TakesValue) {
                        self.add_val_to_arg(a, val, matcher, ValueType::EnvVariable)?;
                    } else {
                        self.check_for_help_and_version_str(val)?;
                        matcher.add_index_to(&a.id, self.cur_idx.get(), ValueType::EnvVariable);
                    }
                }
            }
        }
        Ok(())
    }
}

// Error, Help, and Version Methods
impl<'help, 'app> Parser<'help, 'app> {
    fn did_you_mean_error(
        &mut self,
        arg: &str,
        matcher: &mut ArgMatcher,
        remaining_args: &[&str],
    ) -> ClapResult<()> {
        debug!("Parser::did_you_mean_error: arg={}", arg);
        // Didn't match a flag or option
        let longs = self
            .app
            .args
            .keys()
            .filter_map(|x| match x {
                KeyType::Long(l) => Some(l.to_string_lossy().into_owned()),
                _ => None,
            })
            .collect::<Vec<_>>();
        debug!("Parser::did_you_mean_error: longs={:?}", longs);

        let did_you_mean = suggestions::did_you_mean_flag(
            arg,
            remaining_args,
            longs.iter().map(|ref x| &x[..]),
            self.app.subcommands.as_mut_slice(),
        );

        // Add the arg to the matches to build a proper usage string
        if let Some((name, _)) = did_you_mean.as_ref() {
            if let Some(opt) = self.app.args.get(&name.as_ref()) {
                for g in self.app.groups_for_arg(&opt.id) {
                    matcher.inc_occurrence_of(&g);
                }
                matcher.insert(&opt.id);
            }
        }

        let used: Vec<Id> = matcher
            .arg_names()
            .filter(|n| {
                self.app.find(n).map_or(true, |a| {
                    !(self.required.contains(&a.id) || a.is_set(ArgSettings::Hidden))
                })
            })
            .cloned()
            .collect();

        Err(ClapError::unknown_argument(
            format!("--{}", arg),
            did_you_mean,
            Usage::new(self).create_usage_with_title(&*used),
            self.app.color(),
        ))
    }

    pub(crate) fn write_help_err(&self) -> ClapResult<Colorizer> {
        let mut c = Colorizer::new(true, self.color_help());
        Help::new(HelpWriter::Buffer(&mut c), self, false).write_help()?;
        Ok(c)
    }

    fn help_err(&self, mut use_long: bool) -> ClapError {
        debug!(
            "Parser::help_err: use_long={:?}",
            use_long && self.use_long_help()
        );

        use_long = use_long && self.use_long_help();
        let mut c = Colorizer::new(false, self.color_help());

        match Help::new(HelpWriter::Buffer(&mut c), self, use_long).write_help() {
            Err(e) => e.into(),
            _ => ClapError {
                message: c,
                kind: ErrorKind::DisplayHelp,
                info: vec![],
                source: None,
            },
        }
    }

    fn version_err(&self, use_long: bool) -> ClapError {
        debug!("Parser::version_err");

        let msg = self.app._render_version(use_long);
        let mut c = Colorizer::new(false, self.color_help());
        c.none(msg);
        ClapError {
            message: c,
            kind: ErrorKind::DisplayVersion,
            info: vec![],
            source: None,
        }
    }
}

// Query Methods
impl<'help, 'app> Parser<'help, 'app> {
    fn contains_short(&self, s: char) -> bool {
        self.app.contains_short(s)
    }

    pub(crate) fn has_args(&self) -> bool {
        self.app.has_args()
    }

    pub(crate) fn has_opts(&self) -> bool {
        self.app.has_opts()
    }

    pub(crate) fn has_flags(&self) -> bool {
        self.app.has_flags()
    }

    pub(crate) fn has_positionals(&self) -> bool {
        self.app.args.keys().any(|x| x.is_position())
    }

    pub(crate) fn has_subcommands(&self) -> bool {
        self.app.has_subcommands()
    }

    pub(crate) fn has_visible_subcommands(&self) -> bool {
        self.app.has_visible_subcommands()
    }

    pub(crate) fn is_set(&self, s: AS) -> bool {
        self.app.is_set(s)
    }

    pub(crate) fn set(&mut self, s: AS) {
        self.app.set(s)
    }
}
