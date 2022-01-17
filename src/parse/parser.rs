// Std
use std::{
    cell::{Cell, RefCell},
    ffi::{OsStr, OsString},
};

// Third Party
use os_str_bytes::RawOsStr;

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
    util::{color::ColorChoice, ChildGraph, Id},
    INTERNAL_ERROR_MSG, INVALID_UTF8,
};

pub(crate) struct Parser<'help, 'app> {
    pub(crate) app: &'app mut App<'help>,
    pub(crate) required: ChildGraph<Id>,
    pub(crate) overridden: RefCell<Vec<Id>>,
    pub(crate) seen: Vec<Id>,
    pub(crate) cur_idx: Cell<usize>,
    /// Index of the previous flag subcommand in a group of flags.
    pub(crate) flag_subcmd_at: Option<usize>,
    /// Counter indicating the number of items to skip
    /// when revisiting the group of flags which includes the flag subcommand.
    pub(crate) flag_subcmd_skip: usize,
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
            overridden: Default::default(),
            seen: Vec::new(),
            cur_idx: Cell::new(0),
            flag_subcmd_at: None,
            flag_subcmd_skip: 0,
        }
    }

    // Does all the initializing and prepares the parser
    pub(crate) fn _build(&mut self) {
        debug!("Parser::_build");

        for group in &self.app.groups {
            if group.required {
                let idx = self.required.insert(group.id.clone());
                for a in &group.requires {
                    self.required.insert_child(idx, a.clone());
                }
            }
        }
    }

    // Should we color the help?
    pub(crate) fn color_help(&self) -> ColorChoice {
        #[cfg(feature = "color")]
        if self.is_set(AS::DisableColoredHelp) {
            return ColorChoice::Never;
        }

        self.app.get_color()
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
        let mut parse_state = ParseState::ValuesDone;
        let mut pos_counter = 1;

        // Already met any valid arg(then we shouldn't expect subcommands after it).
        let mut valid_arg_found = false;
        // If the user already passed '--'. Meaning only positional args follow.
        let mut trailing_values = false;

        // Count of positional args
        let positional_count = self.app.args.keys().filter(|x| x.is_position()).count();
        // If any arg sets .last(true)
        let contains_last = self.app.args.args().any(|x| x.is_set(ArgSettings::Last));

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

            let arg_os = RawOsStr::new(arg_os);
            debug!(
                "Parser::get_matches_with: Begin parsing '{:?}' ({:?})",
                arg_os,
                arg_os.as_raw_bytes()
            );

            // Correct pos_counter.
            pos_counter = {
                let is_second_to_last = pos_counter + 1 == positional_count;

                // The last positional argument, or second to last positional
                // argument may be set to .multiple_values(true) or `.multiple_occurrences(true)`
                let low_index_mults = is_second_to_last
                    && self
                        .app
                        .get_positionals()
                        .any(|a| a.is_multiple() && (positional_count != a.index.unwrap_or(0)))
                    && self
                        .app
                        .get_positionals()
                        .last()
                        .map_or(false, |p_name| !p_name.is_set(ArgSettings::Last));

                let missing_pos = self.is_set(AS::AllowMissingPositional)
                    && is_second_to_last
                    && !trailing_values;

                debug!(
                    "Parser::get_matches_with: Positional counter...{}",
                    pos_counter
                );
                debug!(
                    "Parser::get_matches_with: Low index multiples...{:?}",
                    low_index_mults
                );

                if low_index_mults || missing_pos {
                    let skip_current = if let Some(n) = remaining_args.get(0) {
                        if let Some(p) = self
                            .app
                            .get_positionals()
                            .find(|p| p.index == Some(pos_counter))
                        {
                            // If next value looks like a new_arg or it's a
                            // subcommand, skip positional argument under current
                            // pos_counter(which means current value cannot be a
                            // positional argument with a value next to it), assume
                            // current value matches the next arg.
                            let n = RawOsStr::new(n);
                            self.is_new_arg(&n, p)
                                || self.possible_subcommand(&n, valid_arg_found).is_some()
                        } else {
                            true
                        }
                    } else {
                        true
                    };

                    if skip_current {
                        debug!("Parser::get_matches_with: Bumping the positional counter...");
                        pos_counter + 1
                    } else {
                        pos_counter
                    }
                } else if trailing_values
                    && (self.is_set(AS::AllowMissingPositional) || contains_last)
                {
                    // Came to -- and one positional has .last(true) set, so we go immediately
                    // to the last (highest index) positional
                    debug!("Parser::get_matches_with: .last(true) and --, setting last pos");
                    positional_count
                } else {
                    pos_counter
                }
            };

            // Has the user already passed '--'? Meaning only positional args follow
            if !trailing_values {
                if self.is_set(AS::SubcommandPrecedenceOverArg)
                    || !matches!(parse_state, ParseState::Opt(_) | ParseState::Pos(_))
                {
                    // Does the arg match a subcommand name, or any of its aliases (if defined)
                    let sc_name = self.possible_subcommand(&arg_os, valid_arg_found);
                    debug!("Parser::get_matches_with: sc={:?}", sc_name);
                    if let Some(sc_name) = sc_name {
                        if sc_name == "help"
                            && !self.is_set(AS::NoAutoHelp)
                            && !self.is_set(AS::DisableHelpSubcommand)
                        {
                            self.parse_help_subcommand(remaining_args)?;
                        }
                        subcmd_name = Some(sc_name.to_owned());
                        break;
                    }
                }

                if let Some(long_arg) = arg_os.strip_prefix("--") {
                    let parse_result = self.parse_long_arg(
                        matcher,
                        long_arg,
                        &parse_state,
                        &mut valid_arg_found,
                        trailing_values,
                    );
                    debug!(
                        "Parser::get_matches_with: After parse_long_arg {:?}",
                        parse_result
                    );
                    match parse_result {
                        ParseResult::NoArg => {
                            debug!("Parser::get_matches_with: setting TrailingVals=true");
                            trailing_values = true;
                            continue;
                        }
                        ParseResult::ValuesDone => {
                            parse_state = ParseState::ValuesDone;
                            continue;
                        }
                        ParseResult::Opt(id) => {
                            parse_state = ParseState::Opt(id);
                            continue;
                        }
                        ParseResult::FlagSubCommand(name) => {
                            debug!(
                                "Parser::get_matches_with: FlagSubCommand found in long arg {:?}",
                                &name
                            );
                            subcmd_name = Some(name);
                            break;
                        }
                        ParseResult::EqualsNotProvided { arg } => {
                            return Err(ClapError::no_equals(
                                self.app,
                                arg,
                                Usage::new(self).create_usage_with_title(&[]),
                            ));
                        }
                        ParseResult::NoMatchingArg { arg } => {
                            let remaining_args: Vec<_> = remaining_args
                                .iter()
                                .map(|x| x.to_str().expect(INVALID_UTF8))
                                .collect();
                            return Err(self.did_you_mean_error(&arg, matcher, &remaining_args));
                        }
                        ParseResult::UnneededAttachedValue { rest, used, arg } => {
                            return Err(ClapError::too_many_values(
                                self.app,
                                rest,
                                arg,
                                Usage::new(self).create_usage_no_title(&used),
                            ))
                        }
                        ParseResult::HelpFlag => {
                            return Err(self.help_err(true));
                        }
                        ParseResult::VersionFlag => {
                            return Err(self.version_err(true));
                        }
                        ParseResult::MaybeHyphenValue => {
                            // Maybe a hyphen value, do nothing.
                        }
                        ParseResult::AttachedValueNotConsumed => {
                            unreachable!()
                        }
                    }
                } else if let Some(short_arg) = arg_os.strip_prefix("-") {
                    // Arg looks like a short flag, and not a possible number

                    // Try to parse short args like normal, if AllowHyphenValues or
                    // AllowNegativeNumbers is set, parse_short_arg will *not* throw
                    // an error, and instead return Ok(None)
                    let parse_result = self.parse_short_arg(
                        matcher,
                        short_arg,
                        &parse_state,
                        pos_counter,
                        &mut valid_arg_found,
                        trailing_values,
                    );
                    // If it's None, we then check if one of those two AppSettings was set
                    debug!(
                        "Parser::get_matches_with: After parse_short_arg {:?}",
                        parse_result
                    );
                    match parse_result {
                        ParseResult::NoArg => {
                            // Is a single dash `-`, try positional.
                        }
                        ParseResult::ValuesDone => {
                            parse_state = ParseState::ValuesDone;
                            continue;
                        }
                        ParseResult::Opt(id) => {
                            parse_state = ParseState::Opt(id);
                            continue;
                        }
                        ParseResult::FlagSubCommand(name) => {
                            // If there are more short flags to be processed, we should keep the state, and later
                            // revisit the current group of short flags skipping the subcommand.
                            keep_state = self
                                .flag_subcmd_at
                                .map(|at| {
                                    it.cursor -= 1;
                                    // Since we are now saving the current state, the number of flags to skip during state recovery should
                                    // be the current index (`cur_idx`) minus ONE UNIT TO THE LEFT of the starting position.
                                    self.flag_subcmd_skip = self.cur_idx.get() - at + 1;
                                })
                                .is_some();

                            debug!(
                                "Parser::get_matches_with:FlagSubCommandShort: subcmd_name={}, keep_state={}, flag_subcmd_skip={}",
                                name,
                                keep_state,
                                self.flag_subcmd_skip
                            );

                            subcmd_name = Some(name);
                            break;
                        }
                        ParseResult::EqualsNotProvided { arg } => {
                            return Err(ClapError::no_equals(
                                self.app,
                                arg,
                                Usage::new(self).create_usage_with_title(&[]),
                            ))
                        }
                        ParseResult::NoMatchingArg { arg } => {
                            return Err(ClapError::unknown_argument(
                                self.app,
                                arg,
                                None,
                                Usage::new(self).create_usage_with_title(&[]),
                            ));
                        }
                        ParseResult::HelpFlag => {
                            return Err(self.help_err(false));
                        }
                        ParseResult::VersionFlag => {
                            return Err(self.version_err(false));
                        }
                        ParseResult::MaybeHyphenValue => {
                            // Maybe a hyphen value, do nothing.
                        }
                        ParseResult::UnneededAttachedValue { .. }
                        | ParseResult::AttachedValueNotConsumed => unreachable!(),
                    }
                }

                if let ParseState::Opt(id) = &parse_state {
                    // Assume this is a value of a previous arg.

                    // get the option so we can check the settings
                    let parse_result = self.add_val_to_arg(
                        &self.app[id],
                        &arg_os,
                        matcher,
                        ValueType::CommandLine,
                        true,
                        trailing_values,
                    );
                    parse_state = match parse_result {
                        ParseResult::Opt(id) => ParseState::Opt(id),
                        ParseResult::ValuesDone => ParseState::ValuesDone,
                        _ => unreachable!(),
                    };
                    // get the next value from the iterator
                    continue;
                }
            }

            if let Some(p) = self.app.args.get(&pos_counter) {
                if p.is_set(ArgSettings::Last) && !trailing_values {
                    return Err(ClapError::unknown_argument(
                        self.app,
                        arg_os.to_str_lossy().into_owned(),
                        None,
                        Usage::new(self).create_usage_with_title(&[]),
                    ));
                }

                if self.is_set(AS::TrailingVarArg) && pos_counter == positional_count {
                    trailing_values = true;
                }

                self.seen.push(p.id.clone());
                // Increase occurrence no matter if we are appending, occurrences
                // of positional argument equals to number of values rather than
                // the number of value groups.
                self.inc_occurrence_of_arg(matcher, p);
                // Creating new value group rather than appending when the arg
                // doesn't have any value. This behaviour is right because
                // positional arguments are always present continuously.
                let append = self.has_val_groups(matcher, p);
                self.add_val_to_arg(
                    p,
                    &arg_os,
                    matcher,
                    ValueType::CommandLine,
                    append,
                    trailing_values,
                );

                // Only increment the positional counter if it doesn't allow multiples
                if !p.is_multiple() {
                    pos_counter += 1;
                    parse_state = ParseState::ValuesDone;
                } else {
                    parse_state = ParseState::Pos(p.id.clone());
                }
                valid_arg_found = true;
            } else if self.is_set(AS::AllowExternalSubcommands) {
                // Get external subcommand name
                let sc_name = match arg_os.to_str() {
                    Some(s) => s.to_string(),
                    None => {
                        return Err(ClapError::invalid_utf8(
                            self.app,
                            Usage::new(self).create_usage_with_title(&[]),
                        ));
                    }
                };

                // Collect the external subcommand args
                let mut sc_m = ArgMatcher::new(self.app);

                while let Some((v, _)) = it.next() {
                    let allow_invalid_utf8 =
                        self.is_set(AS::AllowInvalidUtf8ForExternalSubcommands);
                    if !allow_invalid_utf8 && v.to_str().is_none() {
                        return Err(ClapError::invalid_utf8(
                            self.app,
                            Usage::new(self).create_usage_with_title(&[]),
                        ));
                    }
                    sc_m.add_val_to(
                        &Id::empty_hash(),
                        v.to_os_string(),
                        ValueType::CommandLine,
                        false,
                    );
                    sc_m.get_mut(&Id::empty_hash())
                        .expect("just inserted")
                        .invalid_utf8_allowed(allow_invalid_utf8);
                }

                matcher.subcommand(SubCommand {
                    name: sc_name.clone(),
                    id: sc_name.into(),
                    matches: sc_m.into_inner(),
                });

                return Validator::new(self).validate(
                    parse_state,
                    subcmd_name.is_some(),
                    matcher,
                    trailing_values,
                );
            } else {
                // Start error processing
                return Err(self.match_arg_error(&arg_os, valid_arg_found, trailing_values));
            }
        }

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
                self.app,
                bn.to_string(),
                Usage::new(self).create_usage_with_title(&[]),
            ));
        } else if self.is_set(AS::SubcommandRequiredElseHelp) {
            debug!("Parser::get_matches_with: SubcommandRequiredElseHelp=true");
            let message = self.write_help_err()?;
            return Err(ClapError::new(
                message,
                ErrorKind::DisplayHelpOnMissingArgumentOrSubcommand,
                self.app.settings.is_set(AS::WaitOnError),
            ));
        }

        Validator::new(self).validate(parse_state, subcmd_name.is_some(), matcher, trailing_values)
    }

    fn match_arg_error(
        &self,
        arg_os: &RawOsStr,
        valid_arg_found: bool,
        trailing_values: bool,
    ) -> ClapError {
        // If argument follows a `--`
        if trailing_values {
            // If the arg matches a subcommand name, or any of its aliases (if defined)
            if self.possible_subcommand(arg_os, valid_arg_found).is_some() {
                return ClapError::unnecessary_double_dash(
                    self.app,
                    arg_os.to_str_lossy().into_owned(),
                    Usage::new(self).create_usage_with_title(&[]),
                );
            }
        }
        let candidates =
            suggestions::did_you_mean(&arg_os.to_str_lossy(), self.app.all_subcommand_names());
        // If the argument looks like a subcommand.
        if !candidates.is_empty() {
            let candidates: Vec<_> = candidates
                .iter()
                .map(|candidate| format!("'{}'", candidate))
                .collect();
            return ClapError::invalid_subcommand(
                self.app,
                arg_os.to_str_lossy().into_owned(),
                candidates.join(" or "),
                self.app
                    .bin_name
                    .as_ref()
                    .unwrap_or(&self.app.name)
                    .to_string(),
                Usage::new(self).create_usage_with_title(&[]),
            );
        }
        // If the argument must be a subcommand.
        if !self.app.has_args() || self.is_set(AS::InferSubcommands) && self.app.has_subcommands() {
            return ClapError::unrecognized_subcommand(
                self.app,
                arg_os.to_str_lossy().into_owned(),
                self.app
                    .bin_name
                    .as_ref()
                    .unwrap_or(&self.app.name)
                    .to_string(),
            );
        }
        ClapError::unknown_argument(
            self.app,
            arg_os.to_str_lossy().into_owned(),
            None,
            Usage::new(self).create_usage_with_title(&[]),
        )
    }

    // Checks if the arg matches a subcommand name, or any of its aliases (if defined)
    fn possible_subcommand(&self, arg_os: &RawOsStr, valid_arg_found: bool) -> Option<&str> {
        debug!("Parser::possible_subcommand: arg={:?}", arg_os);

        if !(self.is_set(AS::ArgsNegateSubcommands) && valid_arg_found) {
            if self.is_set(AS::InferSubcommands) {
                // For subcommand `test`, we accepts it's prefix: `t`, `te`,
                // `tes` and `test`.
                let v = self
                    .app
                    .all_subcommand_names()
                    .filter(|s| RawOsStr::from_str(s).starts_with_os(arg_os))
                    .collect::<Vec<_>>();

                if v.len() == 1 {
                    return Some(v[0]);
                }

                // If there is any ambiguity, fallback to non-infer subcommand
                // search.
            }
            if let Some(sc) = self.app.find_subcommand(arg_os) {
                return Some(&sc.name);
            }
        }
        None
    }

    // Checks if the arg matches a long flag subcommand name, or any of its aliases (if defined)
    fn possible_long_flag_subcommand(&self, arg_os: &RawOsStr) -> Option<&str> {
        debug!("Parser::possible_long_flag_subcommand: arg={:?}", arg_os);
        if self.is_set(AS::InferSubcommands) {
            let options = self
                .app
                .get_subcommands()
                .fold(Vec::new(), |mut options, sc| {
                    if let Some(long) = sc.long_flag {
                        if RawOsStr::from_str(long).starts_with_os(arg_os) {
                            options.push(long);
                        }
                        options.extend(
                            sc.get_all_aliases()
                                .filter(|alias| RawOsStr::from_str(alias).starts_with_os(arg_os)),
                        )
                    }
                    options
                });
            if options.len() == 1 {
                return Some(options[0]);
            }

            for sc in options {
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

        let mut bin_name = self.app.bin_name.as_ref().unwrap_or(&self.app.name).clone();

        let mut sc = {
            let mut sc = self.app.clone();

            for cmd in cmds.iter() {
                sc = if let Some(c) = sc.find_subcommand(cmd) {
                    c
                } else if let Some(c) = sc.find_subcommand(&cmd.to_string_lossy()) {
                    c
                } else {
                    return Err(ClapError::unrecognized_subcommand(
                        self.app,
                        cmd.to_string_lossy().into_owned(),
                        self.app
                            .bin_name
                            .as_ref()
                            .unwrap_or(&self.app.name)
                            .to_string(),
                    ));
                }
                .clone();

                sc._build();
                bin_name.push(' ');
                bin_name.push_str(&sc.name);
            }

            sc
        };
        sc = sc.bin_name(bin_name);

        let parser = Parser::new(&mut sc);

        Err(parser.help_err(self.app.is_set(AS::UseLongFormatForHelpSubcommand)))
    }

    fn is_new_arg(&self, next: &RawOsStr, current_positional: &Arg) -> bool {
        debug!(
            "Parser::is_new_arg: {:?}:{:?}",
            next, current_positional.name
        );

        if self.is_set(AS::AllowHyphenValues)
            || self.app[&current_positional.id].is_set(ArgSettings::AllowHyphenValues)
            || (self.is_set(AS::AllowNegativeNumbers) && next.to_str_lossy().parse::<f64>().is_ok())
        {
            // If allow hyphen, this isn't a new arg.
            debug!("Parser::is_new_arg: Allow hyphen");
            false
        } else if next.starts_with("--") {
            // If this is a long flag, this is a new arg.
            debug!("Parser::is_new_arg: -- found");
            true
        } else if next.starts_with("-") {
            debug!("Parser::is_new_arg: - found");
            // If this is a short flag, this is a new arg. But a singe '-' by
            // itself is a value and typically means "stdin" on unix systems.
            next.raw_len() != 1
        } else {
            debug!("Parser::is_new_arg: value");
            // Nothing special, this is a value.
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
        debug!("Parser::parse_subcommand");

        let mut mid_string = String::from(" ");

        if !self.is_set(AS::SubcommandsNegateReqs) {
            let reqs = Usage::new(self).get_required_usage_from(&[], None, true); // maybe Some(m)

            for s in &reqs {
                mid_string.push_str(s);
                mid_string.push(' ');
            }
        }

        let partial_parsing_enabled = self.is_set(AS::IgnoreErrors);

        if let Some(sc) = self.app.subcommands.iter_mut().find(|s| s.name == sc_name) {
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

            let mut sc_matcher = ArgMatcher::new(sc);

            debug!("Parser::parse_subcommand: About to parse sc={}", sc.name);

            {
                let mut p = Parser::new(sc);
                // HACK: maintain indexes between parsers
                // FlagSubCommand short arg needs to revisit the current short args, but skip the subcommand itself
                if keep_state {
                    p.cur_idx.set(self.cur_idx.get());
                    p.flag_subcmd_at = self.flag_subcmd_at;
                    p.flag_subcmd_skip = self.flag_subcmd_skip;
                }
                if let Err(error) = p.get_matches_with(&mut sc_matcher, it) {
                    if partial_parsing_enabled {
                        debug!(
                            "Parser::parse_subcommand: ignored error in subcommand {}: {:?}",
                            sc_name, error
                        );
                    } else {
                        return Err(error);
                    }
                }
            }
            matcher.subcommand(SubCommand {
                id: sc.id.clone(),
                name: sc.name.clone(),
                matches: sc_matcher.into_inner(),
            });
        }
        Ok(())
    }

    // Retrieves the names of all args the user has supplied thus far, except required ones
    // because those will be listed in self.required
    fn check_for_help_and_version_str(&self, arg: &RawOsStr) -> Option<ParseResult> {
        debug!("Parser::check_for_help_and_version_str");
        debug!(
            "Parser::check_for_help_and_version_str: Checking if --{:?} is help or version...",
            arg
        );

        if let Some(help) = self.app.find(&Id::help_hash()) {
            if let Some(h) = help.long {
                if arg == h && !self.is_set(AS::NoAutoHelp) && !self.is_set(AS::DisableHelpFlag) {
                    debug!("Help");
                    return Some(ParseResult::HelpFlag);
                }
            }
        }

        if let Some(version) = self.app.find(&Id::version_hash()) {
            if let Some(v) = version.long {
                if arg == v
                    && !self.is_set(AS::NoAutoVersion)
                    && !self.is_set(AS::DisableVersionFlag)
                {
                    debug!("Version");
                    return Some(ParseResult::VersionFlag);
                }
            }
        }

        debug!("Neither");
        None
    }

    fn check_for_help_and_version_char(&self, arg: char) -> Option<ParseResult> {
        debug!("Parser::check_for_help_and_version_char");
        debug!(
            "Parser::check_for_help_and_version_char: Checking if -{} is help or version...",
            arg
        );

        if let Some(help) = self.app.find(&Id::help_hash()) {
            if let Some(h) = help.short {
                if arg == h && !self.is_set(AS::NoAutoHelp) && !self.is_set(AS::DisableHelpFlag) {
                    debug!("Help");
                    return Some(ParseResult::HelpFlag);
                }
            }
        }

        if let Some(version) = self.app.find(&Id::version_hash()) {
            if let Some(v) = version.short {
                if arg == v
                    && !self.is_set(AS::NoAutoVersion)
                    && !self.is_set(AS::DisableVersionFlag)
                {
                    debug!("Version");
                    return Some(ParseResult::VersionFlag);
                }
            }
        }

        debug!("Neither");
        None
    }

    fn use_long_help(&self) -> bool {
        debug!("Parser::use_long_help");
        // In this case, both must be checked. This allows the retention of
        // original formatting, but also ensures that the actual -h or --help
        // specified by the user is sent through. If HiddenShortHelp is not included,
        // then items specified with hidden_short_help will also be hidden.
        let should_long = |v: &Arg| {
            v.long_help.is_some()
                || v.is_set(ArgSettings::HiddenLongHelp)
                || v.is_set(ArgSettings::HiddenShortHelp)
        };

        // Subcommands aren't checked because we prefer short help for them, deferring to
        // `cmd subcmd --help` for more.
        self.app.long_about.is_some()
            || self.app.before_long_help.is_some()
            || self.app.after_long_help.is_some()
            || self.app.args.args().any(should_long)
    }

    fn parse_long_arg(
        &mut self,
        matcher: &mut ArgMatcher,
        long_arg: &RawOsStr,
        parse_state: &ParseState,
        valid_arg_found: &mut bool,
        trailing_values: bool,
    ) -> ParseResult {
        // maybe here lifetime should be 'a
        debug!("Parser::parse_long_arg");

        if matches!(parse_state, ParseState::Opt(opt) | ParseState::Pos(opt) if
            self.app[opt].is_set(ArgSettings::AllowHyphenValues))
        {
            return ParseResult::MaybeHyphenValue;
        }

        // Update the current index
        self.cur_idx.set(self.cur_idx.get() + 1);
        debug!("Parser::parse_long_arg: cur_idx:={}", self.cur_idx.get());

        debug!("Parser::parse_long_arg: Does it contain '='...");
        if long_arg.is_empty() {
            return ParseResult::NoArg;
        }
        let (arg, val) = if let Some(index) = long_arg.find("=") {
            let (p0, p1) = long_arg.split_at(index);
            debug!("Yes '{:?}'", p1);
            (p0, Some(p1))
        } else {
            debug!("No");
            (long_arg, None)
        };

        let opt = if let Some(opt) = self.app.args.get(&*arg.to_os_str()) {
            debug!(
                "Parser::parse_long_arg: Found valid opt or flag '{}'",
                opt.to_string()
            );
            Some(opt)
        } else if self.is_set(AS::InferLongArgs) {
            let arg_str = arg.to_str_lossy();
            self.app.args.args().find(|a| {
                a.long.map_or(false, |long| long.starts_with(&*arg_str))
                    || a.aliases
                        .iter()
                        .any(|(alias, _)| alias.starts_with(&*arg_str))
            })
        } else {
            None
        };

        if let Some(opt) = opt {
            *valid_arg_found = true;
            self.seen.push(opt.id.clone());
            if opt.is_set(ArgSettings::TakesValue) {
                debug!(
                    "Parser::parse_long_arg: Found an opt with value '{:?}'",
                    &val
                );
                self.parse_opt(val, opt, matcher, trailing_values)
            } else if let Some(rest) = val {
                debug!("Parser::parse_long_arg: Got invalid literal `{:?}`", rest);
                let used: Vec<Id> = matcher
                    .arg_names()
                    .filter(|&n| {
                        self.app.find(n).map_or(true, |a| {
                            !(a.is_set(ArgSettings::Hidden) || self.required.contains(&a.id))
                        })
                    })
                    .cloned()
                    .collect();

                ParseResult::UnneededAttachedValue {
                    rest: rest.to_str_lossy().into_owned(),
                    used,
                    arg: opt.to_string(),
                }
            } else if let Some(parse_result) = self.check_for_help_and_version_str(arg) {
                parse_result
            } else {
                debug!("Parser::parse_long_arg: Presence validated");
                self.parse_flag(opt, matcher)
            }
        } else if let Some(sc_name) = self.possible_long_flag_subcommand(arg) {
            ParseResult::FlagSubCommand(sc_name.to_string())
        } else if self.is_set(AS::AllowHyphenValues) {
            ParseResult::MaybeHyphenValue
        } else {
            ParseResult::NoMatchingArg {
                arg: arg.to_str_lossy().into_owned(),
            }
        }
    }

    fn parse_short_arg(
        &mut self,
        matcher: &mut ArgMatcher,
        short_arg: &RawOsStr,
        parse_state: &ParseState,
        // change this to possible pos_arg when removing the usage of &mut Parser.
        pos_counter: usize,
        valid_arg_found: &mut bool,
        trailing_values: bool,
    ) -> ParseResult {
        debug!("Parser::parse_short_arg: short_arg={:?}", short_arg);
        let arg = short_arg.to_str_lossy();

        #[allow(clippy::blocks_in_if_conditions)]
        if self.is_set(AS::AllowNegativeNumbers) && arg.parse::<f64>().is_ok() {
            debug!("Parser::parse_short_arg: negative number");
            return ParseResult::MaybeHyphenValue;
        } else if self.is_set(AS::AllowHyphenValues)
            && arg.chars().any(|c| !self.app.contains_short(c))
        {
            debug!("Parser::parse_short_args: contains non-short flag");
            return ParseResult::MaybeHyphenValue;
        } else if matches!(parse_state, ParseState::Opt(opt) | ParseState::Pos(opt)
                if self.app[opt].is_set(ArgSettings::AllowHyphenValues))
        {
            debug!("Parser::parse_short_args: prior arg accepts hyphenated values",);
            return ParseResult::MaybeHyphenValue;
        } else if self.app.args.get(&pos_counter).map_or(false, |arg| {
            arg.is_set(ArgSettings::AllowHyphenValues) && !arg.is_set(ArgSettings::Last)
        }) {
            debug!(
                "Parser::parse_short_args: positional at {} allows hyphens",
                pos_counter
            );
            return ParseResult::MaybeHyphenValue;
        }

        let mut ret = ParseResult::NoArg;

        let skip = self.flag_subcmd_skip;
        self.flag_subcmd_skip = 0;
        for c in arg.chars().skip(skip) {
            debug!("Parser::parse_short_arg:iter:{}", c);

            // update each index because `-abcd` is four indices to clap
            self.cur_idx.set(self.cur_idx.get() + 1);
            debug!(
                "Parser::parse_short_arg:iter:{}: cur_idx:={}",
                c,
                self.cur_idx.get()
            );

            // Check for matching short options, and return the name if there is no trailing
            // concatenated value: -oval
            // Option: -o
            // Value: val
            if let Some(opt) = self.app.args.get(&c) {
                debug!(
                    "Parser::parse_short_arg:iter:{}: Found valid opt or flag",
                    c
                );
                *valid_arg_found = true;
                self.seen.push(opt.id.clone());
                if !opt.is_set(ArgSettings::TakesValue) {
                    if let Some(parse_result) = self.check_for_help_and_version_char(c) {
                        return parse_result;
                    }
                    ret = self.parse_flag(opt, matcher);
                    continue;
                }

                // Check for trailing concatenated value
                let val = short_arg.split_once(c).expect(INTERNAL_ERROR_MSG).1;
                debug!(
                    "Parser::parse_short_arg:iter:{}: val={:?} (bytes), val={:?} (ascii), short_arg={:?}",
                    c, val, val.as_raw_bytes(), short_arg
                );
                let val = Some(val).filter(|v| !v.is_empty());

                // Default to "we're expecting a value later".
                //
                // If attached value is not consumed, we may have more short
                // flags to parse, continue.
                //
                // e.g. `-xvf`, when RequireEquals && x.min_vals == 0, we don't
                // consume the `vf`, even if it's provided as value.
                match self.parse_opt(val, opt, matcher, trailing_values) {
                    ParseResult::AttachedValueNotConsumed => continue,
                    x => return x,
                }
            }

            return if let Some(sc_name) = self.app.find_short_subcmd(c) {
                debug!("Parser::parse_short_arg:iter:{}: subcommand={}", c, sc_name);
                let name = sc_name.to_string();
                let done_short_args = {
                    let cur_idx = self.cur_idx.get();
                    // Get the index of the previously saved flag subcommand in the group of flags (if exists).
                    // If it is a new flag subcommand, then the formentioned index should be the current one
                    // (ie. `cur_idx`), and should be registered.
                    let at = *self.flag_subcmd_at.get_or_insert(cur_idx);
                    // If we are done, then the difference of indices (cur_idx - at) should be (end - at) which
                    // should equal to (arg.len() - 1),
                    // where `end` is the index of the end of the group.
                    cur_idx - at == arg.len() - 1
                };
                if done_short_args {
                    self.flag_subcmd_at = None;
                }
                ParseResult::FlagSubCommand(name)
            } else {
                ParseResult::NoMatchingArg {
                    arg: format!("-{}", c),
                }
            };
        }
        ret
    }

    fn parse_opt(
        &self,
        attached_value: Option<&RawOsStr>,
        opt: &Arg<'help>,
        matcher: &mut ArgMatcher,
        trailing_values: bool,
    ) -> ParseResult {
        debug!(
            "Parser::parse_opt; opt={}, val={:?}",
            opt.name, attached_value
        );
        debug!("Parser::parse_opt; opt.settings={:?}", opt.settings);
        // has_eq: --flag=value
        let has_eq = matches!(attached_value, Some(fv) if fv.starts_with("="));

        debug!("Parser::parse_opt; Checking for val...");
        // RequireEquals is set, but no '=' is provided, try throwing error.
        if opt.is_set(ArgSettings::RequireEquals) && !has_eq {
            if opt.min_vals == Some(0) {
                debug!("Requires equals, but min_vals == 0");
                self.inc_occurrence_of_arg(matcher, opt);
                // We assume this case is valid: require equals, but min_vals == 0.
                if !opt.default_missing_vals.is_empty() {
                    debug!("Parser::parse_opt: has default_missing_vals");
                    self.add_multiple_vals_to_arg(
                        opt,
                        opt.default_missing_vals.iter().map(OsString::from),
                        matcher,
                        ValueType::CommandLine,
                        false,
                    );
                };
                if attached_value.is_some() {
                    ParseResult::AttachedValueNotConsumed
                } else {
                    ParseResult::ValuesDone
                }
            } else {
                debug!("Requires equals but not provided. Error.");
                ParseResult::EqualsNotProvided {
                    arg: opt.to_string(),
                }
            }
        } else if let Some(fv) = attached_value {
            let v = fv.strip_prefix("=").unwrap_or(fv);
            debug!("Found - {:?}, len: {}", v, v.raw_len());
            debug!(
                "Parser::parse_opt: {:?} contains '='...{:?}",
                fv,
                fv.starts_with("=")
            );
            self.inc_occurrence_of_arg(matcher, opt);
            self.add_val_to_arg(
                opt,
                v,
                matcher,
                ValueType::CommandLine,
                false,
                trailing_values,
            );
            ParseResult::ValuesDone
        } else {
            debug!("Parser::parse_opt: More arg vals required...");
            self.inc_occurrence_of_arg(matcher, opt);
            matcher.new_val_group(&opt.id);
            for group in self.app.groups_for_arg(&opt.id) {
                matcher.new_val_group(&group);
            }
            ParseResult::Opt(opt.id.clone())
        }
    }

    fn add_val_to_arg(
        &self,
        arg: &Arg<'help>,
        val: &RawOsStr,
        matcher: &mut ArgMatcher,
        ty: ValueType,
        append: bool,
        trailing_values: bool,
    ) -> ParseResult {
        debug!("Parser::add_val_to_arg; arg={}, val={:?}", arg.name, val);
        debug!(
            "Parser::add_val_to_arg; trailing_values={:?}, DontDelimTrailingVals={:?}",
            trailing_values,
            self.is_set(AS::DontDelimitTrailingValues)
        );
        if !(trailing_values && self.is_set(AS::DontDelimitTrailingValues)) {
            if let Some(delim) = arg.val_delim {
                let arg_split = val.split(delim);
                let vals = if let Some(t) = arg.terminator {
                    let mut vals = vec![];
                    for val in arg_split {
                        if t == val {
                            break;
                        }
                        vals.push(val);
                    }
                    vals
                } else {
                    arg_split.collect()
                };
                self.add_multiple_vals_to_arg(
                    arg,
                    vals.into_iter().map(|x| x.to_os_str().into_owned()),
                    matcher,
                    ty,
                    append,
                );
                // If there was a delimiter used or we must use the delimiter to
                // separate the values or no more vals is needed, we're not
                // looking for more values.
                return if val.contains(delim)
                    || arg.is_set(ArgSettings::RequireDelimiter)
                    || !matcher.needs_more_vals(arg)
                {
                    ParseResult::ValuesDone
                } else {
                    ParseResult::Opt(arg.id.clone())
                };
            }
        }
        if let Some(t) = arg.terminator {
            if t == val {
                return ParseResult::ValuesDone;
            }
        }
        self.add_single_val_to_arg(arg, val.to_os_str().into_owned(), matcher, ty, append);
        if matcher.needs_more_vals(arg) {
            ParseResult::Opt(arg.id.clone())
        } else {
            ParseResult::ValuesDone
        }
    }

    fn add_multiple_vals_to_arg(
        &self,
        arg: &Arg<'help>,
        vals: impl Iterator<Item = OsString>,
        matcher: &mut ArgMatcher,
        ty: ValueType,
        append: bool,
    ) {
        // If not appending, create a new val group and then append vals in.
        if !append {
            matcher.new_val_group(&arg.id);
            for group in self.app.groups_for_arg(&arg.id) {
                matcher.new_val_group(&group);
            }
        }
        for val in vals {
            self.add_single_val_to_arg(arg, val, matcher, ty, true);
        }
    }

    fn add_single_val_to_arg(
        &self,
        arg: &Arg<'help>,
        val: OsString,
        matcher: &mut ArgMatcher,
        ty: ValueType,
        append: bool,
    ) {
        debug!("Parser::add_single_val_to_arg: adding val...{:?}", val);

        // update the current index because each value is a distinct index to clap
        self.cur_idx.set(self.cur_idx.get() + 1);
        debug!(
            "Parser::add_single_val_to_arg: cur_idx:={}",
            self.cur_idx.get()
        );

        // Increment or create the group "args"
        for group in self.app.groups_for_arg(&arg.id) {
            matcher.add_val_to(&group, val.clone(), ty, append);
        }

        matcher.add_val_to(&arg.id, val, ty, append);
        matcher.add_index_to(&arg.id, self.cur_idx.get(), ty);
    }

    fn has_val_groups(&self, matcher: &mut ArgMatcher, arg: &Arg<'help>) -> bool {
        matcher.has_val_groups(&arg.id)
    }

    fn parse_flag(&self, flag: &Arg<'help>, matcher: &mut ArgMatcher) -> ParseResult {
        debug!("Parser::parse_flag");

        self.inc_occurrence_of_arg(matcher, flag);
        matcher.add_index_to(&flag.id, self.cur_idx.get(), ValueType::CommandLine);

        ParseResult::ValuesDone
    }

    fn remove_overrides(&self, arg: &Arg<'help>, matcher: &mut ArgMatcher) {
        debug!("Parser::remove_overrides: id={:?}", arg.id);
        for override_id in &arg.overrides {
            debug!("Parser::remove_overrides:iter:{:?}: removing", override_id);
            matcher.remove(override_id);
            self.overridden.borrow_mut().push(override_id.clone());
        }

        // Override anything that can override us
        let mut transitive = Vec::new();
        for arg_id in matcher.arg_names() {
            if let Some(overrider) = self.app.find(arg_id) {
                if overrider.overrides.contains(&arg.id) {
                    transitive.push(&overrider.id);
                }
            }
        }
        for overrider_id in transitive {
            debug!("Parser::remove_overrides:iter:{:?}: removing", overrider_id);
            matcher.remove(overrider_id);
            self.overridden.borrow_mut().push(overrider_id.clone());
        }
    }

    pub(crate) fn add_defaults(&mut self, matcher: &mut ArgMatcher, trailing_values: bool) {
        debug!("Parser::add_defaults");

        for o in self.app.get_opts() {
            debug!("Parser::add_defaults:iter:{}:", o.name);
            self.add_value(o, matcher, ValueType::DefaultValue, trailing_values);
        }

        for p in self.app.get_positionals() {
            debug!("Parser::add_defaults:iter:{}:", p.name);
            self.add_value(p, matcher, ValueType::DefaultValue, trailing_values);
        }
    }

    fn add_value(
        &self,
        arg: &Arg<'help>,
        matcher: &mut ArgMatcher,
        ty: ValueType,
        trailing_values: bool,
    ) {
        if !arg.default_vals_ifs.is_empty() {
            debug!("Parser::add_value: has conditional defaults");
            if matcher.get(&arg.id).is_none() {
                for (id, val, default) in arg.default_vals_ifs.iter() {
                    let add = if let Some(a) = matcher.get(id) {
                        if let Some(v) = val {
                            a.vals_flatten().any(|value| v == value)
                        } else {
                            true
                        }
                    } else {
                        false
                    };

                    if add {
                        if let Some(default) = default {
                            self.add_val_to_arg(
                                arg,
                                &RawOsStr::new(default),
                                matcher,
                                ty,
                                false,
                                trailing_values,
                            );
                        }
                        return;
                    }
                }
            }
        } else {
            debug!("Parser::add_value: doesn't have conditional defaults");
        }

        fn process_default_vals(arg: &Arg<'_>, default_vals: &[&OsStr]) -> Vec<OsString> {
            if let Some(delim) = arg.val_delim {
                let mut vals = vec![];
                for val in default_vals {
                    let val = RawOsStr::new(val);
                    for val in val.split(delim) {
                        vals.push(val.to_os_str().into_owned());
                    }
                }
                vals
            } else {
                default_vals.iter().map(OsString::from).collect()
            }
        }

        if !arg.default_vals.is_empty() {
            debug!("Parser::add_value:iter:{}: has default vals", arg.name);
            if matcher.get(&arg.id).is_some() {
                debug!("Parser::add_value:iter:{}: was used", arg.name);
            // do nothing
            } else {
                debug!("Parser::add_value:iter:{}: wasn't used", arg.name);

                self.add_multiple_vals_to_arg(
                    arg,
                    process_default_vals(arg, &arg.default_vals).into_iter(),
                    matcher,
                    ty,
                    false,
                );
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
                Some(ma) if ma.all_val_groups_empty() => {
                    debug!(
                        "Parser::add_value:iter:{}: has no user defined vals",
                        arg.name
                    );
                    self.add_multiple_vals_to_arg(
                        arg,
                        process_default_vals(arg, &arg.default_missing_vals).into_iter(),
                        matcher,
                        ty,
                        false,
                    );
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
    }

    #[cfg(feature = "env")]
    pub(crate) fn add_env(
        &mut self,
        matcher: &mut ArgMatcher,
        trailing_values: bool,
    ) -> ClapResult<()> {
        use crate::util::str_to_bool;

        self.app.args.args().try_for_each(|a| {
            // Use env only if the arg was absent among command line args,
            // early return if this is not the case.
            if matcher.get(&a.id).map_or(false, |a| a.occurs != 0) {
                debug!("Parser::add_env: Skipping existing arg `{}`", a);
                return Ok(());
            }

            debug!("Parser::add_env: Checking arg `{}`", a);
            if let Some((_, Some(ref val))) = a.env {
                let val = RawOsStr::new(val);

                if a.is_set(ArgSettings::TakesValue) {
                    debug!(
                        "Parser::add_env: Found an opt with value={:?}, trailing={:?}",
                        val, trailing_values
                    );
                    self.add_val_to_arg(
                        a,
                        &val,
                        matcher,
                        ValueType::EnvVariable,
                        false,
                        trailing_values,
                    );
                    return Ok(());
                }

                debug!("Parser::add_env: Checking for help and version");
                // Early return on `HelpFlag` or `VersionFlag`.
                match self.check_for_help_and_version_str(&val) {
                    Some(ParseResult::HelpFlag) => {
                        return Err(self.help_err(true));
                    }
                    Some(ParseResult::VersionFlag) => {
                        return Err(self.version_err(true));
                    }
                    _ => (),
                }

                debug!("Parser::add_env: Found a flag with value `{:?}`", val);
                let predicate = str_to_bool(val.to_str_lossy());
                debug!("Parser::add_env: Found boolean literal `{}`", predicate);
                if predicate {
                    matcher.add_index_to(&a.id, self.cur_idx.get(), ValueType::EnvVariable);
                }
            }

            Ok(())
        })
    }

    /// Increase occurrence of specific argument and the grouped arg it's in.
    fn inc_occurrence_of_arg(&self, matcher: &mut ArgMatcher, arg: &Arg<'help>) {
        // With each new occurrence, remove overrides from prior occurrences
        self.remove_overrides(arg, matcher);

        matcher.inc_occurrence_of_arg(arg);
        // Increment or create the group "args"
        for group in self.app.groups_for_arg(&arg.id) {
            matcher.inc_occurrence_of_group(&group);
        }
    }
}

// Error, Help, and Version Methods
impl<'help, 'app> Parser<'help, 'app> {
    /// Is only used for the long flag(which is the only one needs fuzzy searching)
    fn did_you_mean_error(
        &mut self,
        arg: &str,
        matcher: &mut ArgMatcher,
        remaining_args: &[&str],
    ) -> ClapError {
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
            longs.iter().map(|x| &x[..]),
            self.app.subcommands.as_mut_slice(),
        );

        // Add the arg to the matches to build a proper usage string
        if let Some((name, _)) = did_you_mean.as_ref() {
            if let Some(opt) = self.app.args.get(&name.as_ref()) {
                self.inc_occurrence_of_arg(matcher, opt);
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

        ClapError::unknown_argument(
            self.app,
            format!("--{}", arg),
            did_you_mean,
            Usage::new(self).create_usage_with_title(&*used),
        )
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
            _ => ClapError::new(
                c,
                ErrorKind::DisplayHelp,
                self.app.settings.is_set(AS::WaitOnError),
            ),
        }
    }

    fn version_err(&self, use_long: bool) -> ClapError {
        debug!("Parser::version_err");

        let msg = self.app._render_version(use_long);
        let mut c = Colorizer::new(false, self.color_help());
        c.none(msg);
        ClapError::new(
            c,
            ErrorKind::DisplayVersion,
            self.app.settings.is_set(AS::WaitOnError),
        )
    }
}

// Query Methods
impl<'help, 'app> Parser<'help, 'app> {
    pub(crate) fn is_set(&self, s: AS) -> bool {
        self.app.is_set(s)
    }
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
        self.items = insert_items
            .iter()
            .map(OsString::from)
            .chain(self.items.drain(self.cursor..))
            .collect();
        self.cursor = 0;
    }
}

#[derive(Debug)]
pub(crate) enum ParseState {
    ValuesDone,
    Opt(Id),
    Pos(Id),
}

/// Recoverable Parsing results.
#[derive(Debug, PartialEq, Clone)]
enum ParseResult {
    FlagSubCommand(String),
    Opt(Id),
    ValuesDone,
    /// Value attached to the short flag is not consumed(e.g. 'u' for `-cu` is
    /// not consumed).
    AttachedValueNotConsumed,
    /// This long flag doesn't need a value but is provided one.
    UnneededAttachedValue {
        rest: String,
        used: Vec<Id>,
        arg: String,
    },
    /// This flag might be an hyphen Value.
    MaybeHyphenValue,
    /// Equals required but not provided.
    EqualsNotProvided {
        arg: String,
    },
    /// Failed to match a Arg.
    NoMatchingArg {
        arg: String,
    },
    /// No argument found e.g. parser is given `-` when parsing a flag.
    NoArg,
    /// This is a Help flag.
    HelpFlag,
    /// This is a version flag.
    VersionFlag,
}
