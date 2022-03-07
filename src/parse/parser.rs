// Std
use std::{
    cell::{Cell, RefCell},
    ffi::{OsStr, OsString},
};

// Third Party
use os_str_bytes::RawOsStr;

// Internal
use crate::build::{Arg, Command};
use crate::error::Error as ClapError;
use crate::error::Result as ClapResult;
use crate::mkeymap::KeyType;
use crate::output::{fmt::Colorizer, Help, HelpWriter, Usage};
use crate::parse::features::suggestions;
use crate::parse::{ArgMatcher, SubCommand};
use crate::parse::{Validator, ValueSource};
use crate::util::{color::ColorChoice, Id};
use crate::{build::AppSettings as AS, PossibleValue};
use crate::{INTERNAL_ERROR_MSG, INVALID_UTF8};

pub(crate) struct Parser<'help, 'cmd> {
    pub(crate) cmd: &'cmd mut Command<'help>,
    pub(crate) overridden: RefCell<Vec<Id>>,
    seen: Vec<Id>,
    cur_idx: Cell<usize>,
    /// Index of the previous flag subcommand in a group of flags.
    flag_subcmd_at: Option<usize>,
    /// Counter indicating the number of items to skip
    /// when revisiting the group of flags which includes the flag subcommand.
    flag_subcmd_skip: usize,
}

// Initializing Methods
impl<'help, 'cmd> Parser<'help, 'cmd> {
    pub(crate) fn new(cmd: &'cmd mut Command<'help>) -> Self {
        Parser {
            cmd,
            overridden: Default::default(),
            seen: Vec::new(),
            cur_idx: Cell::new(0),
            flag_subcmd_at: None,
            flag_subcmd_skip: 0,
        }
    }

    // Should we color the help?
    pub(crate) fn color_help(&self) -> ColorChoice {
        #[cfg(feature = "color")]
        if self.cmd.is_disable_colored_help_set() {
            return ColorChoice::Never;
        }

        self.cmd.get_color()
    }
}

// Parsing Methods
impl<'help, 'cmd> Parser<'help, 'cmd> {
    // The actual parsing function
    #[allow(clippy::cognitive_complexity)]
    pub(crate) fn get_matches_with(
        &mut self,
        matcher: &mut ArgMatcher,
        it: &mut Input,
    ) -> ClapResult<()> {
        debug!("Parser::get_matches_with");
        // Verify all positional assertions pass

        let mut subcmd_name: Option<String> = None;
        let mut keep_state = false;
        let mut parse_state = ParseState::ValuesDone;
        let mut pos_counter = 1;

        // Already met any valid arg(then we shouldn't expect subcommands after it).
        let mut valid_arg_found = false;
        // If the user already passed '--'. Meaning only positional args follow.
        let mut trailing_values = false;

        // Count of positional args
        let positional_count = self
            .cmd
            .get_keymap()
            .keys()
            .filter(|x| x.is_position())
            .count();
        // If any arg sets .last(true)
        let contains_last = self.cmd.get_arguments().any(|x| x.is_last_set());

        while let Some((arg_os, remaining_args)) = it.next() {
            // Recover the replaced items if any.
            if let Some(replaced_items) = self.cmd.get_replacement(arg_os) {
                debug!(
                    "Parser::get_matches_with: found replacer: {:?}, target: {:?}",
                    arg_os, replaced_items
                );
                it.insert(replaced_items);
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
                        .cmd
                        .get_positionals()
                        .any(|a| a.is_multiple() && (positional_count != a.index.unwrap_or(0)))
                    && self
                        .cmd
                        .get_positionals()
                        .last()
                        .map_or(false, |p_name| !p_name.is_last_set());

                let missing_pos = self.cmd.is_allow_missing_positional_set()
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
                            .cmd
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
                    && (self.cmd.is_allow_missing_positional_set() || contains_last)
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
                if self.cmd.is_subcommand_precedence_over_arg_set()
                    || !matches!(parse_state, ParseState::Opt(_) | ParseState::Pos(_))
                {
                    // Does the arg match a subcommand name, or any of its aliases (if defined)
                    let sc_name = self.possible_subcommand(&arg_os, valid_arg_found);
                    debug!("Parser::get_matches_with: sc={:?}", sc_name);
                    if let Some(sc_name) = sc_name {
                        if sc_name == "help"
                            && !self.is_set(AS::NoAutoHelp)
                            && !self.cmd.is_disable_help_subcommand_set()
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
                                self.cmd,
                                arg,
                                Usage::new(self.cmd).create_usage_with_title(&[]),
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
                                self.cmd,
                                rest,
                                arg,
                                Usage::new(self.cmd).create_usage_no_title(&used),
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

                    // Try to parse short args like normal, if allow_hyphen_values or
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
                                self.cmd,
                                arg,
                                Usage::new(self.cmd).create_usage_with_title(&[]),
                            ))
                        }
                        ParseResult::NoMatchingArg { arg } => {
                            return Err(ClapError::unknown_argument(
                                self.cmd,
                                arg,
                                None,
                                Usage::new(self.cmd).create_usage_with_title(&[]),
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
                        &self.cmd[id],
                        &arg_os,
                        matcher,
                        ValueSource::CommandLine,
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

            if let Some(p) = self.cmd.get_keymap().get(&pos_counter) {
                if p.is_last_set() && !trailing_values {
                    return Err(ClapError::unknown_argument(
                        self.cmd,
                        arg_os.to_str_lossy().into_owned(),
                        None,
                        Usage::new(self.cmd).create_usage_with_title(&[]),
                    ));
                }

                if self.cmd.is_trailing_var_arg_set() && pos_counter == positional_count {
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
                    ValueSource::CommandLine,
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
            } else if self.cmd.is_allow_external_subcommands_set() {
                // Get external subcommand name
                let sc_name = match arg_os.to_str() {
                    Some(s) => s.to_string(),
                    None => {
                        return Err(ClapError::invalid_utf8(
                            self.cmd,
                            Usage::new(self.cmd).create_usage_with_title(&[]),
                        ));
                    }
                };

                // Collect the external subcommand args
                let mut sc_m = ArgMatcher::new(self.cmd);

                while let Some((v, _)) = it.next() {
                    let allow_invalid_utf8 = self
                        .cmd
                        .is_allow_invalid_utf8_for_external_subcommands_set();
                    if !allow_invalid_utf8 && v.to_str().is_none() {
                        return Err(ClapError::invalid_utf8(
                            self.cmd,
                            Usage::new(self.cmd).create_usage_with_title(&[]),
                        ));
                    }
                    sc_m.add_val_to(
                        &Id::empty_hash(),
                        v.to_os_string(),
                        ValueSource::CommandLine,
                        false,
                    );
                    sc_m.get_mut(&Id::empty_hash())
                        .expect("just inserted")
                        .invalid_utf8_allowed(allow_invalid_utf8);
                }

                matcher.subcommand(SubCommand {
                    id: Id::from(&*sc_name),
                    name: sc_name,
                    matches: sc_m.into_inner(),
                });

                return Validator::new(self).validate(parse_state, matcher, trailing_values);
            } else {
                // Start error processing
                return Err(self.match_arg_error(&arg_os, valid_arg_found, trailing_values));
            }
        }

        if let Some(ref pos_sc_name) = subcmd_name {
            let sc_name = self
                .cmd
                .find_subcommand(pos_sc_name)
                .expect(INTERNAL_ERROR_MSG)
                .get_name()
                .to_owned();
            self.parse_subcommand(&sc_name, matcher, it, keep_state)?;
        }

        Validator::new(self).validate(parse_state, matcher, trailing_values)
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
                    self.cmd,
                    arg_os.to_str_lossy().into_owned(),
                    Usage::new(self.cmd).create_usage_with_title(&[]),
                );
            }
        }
        let candidates =
            suggestions::did_you_mean(&arg_os.to_str_lossy(), self.cmd.all_subcommand_names());
        // If the argument looks like a subcommand.
        if !candidates.is_empty() {
            let candidates: Vec<_> = candidates
                .iter()
                .map(|candidate| format!("'{}'", candidate))
                .collect();
            return ClapError::invalid_subcommand(
                self.cmd,
                arg_os.to_str_lossy().into_owned(),
                candidates.join(" or "),
                self.cmd
                    .get_bin_name()
                    .unwrap_or_else(|| self.cmd.get_name())
                    .to_owned(),
                Usage::new(self.cmd).create_usage_with_title(&[]),
            );
        }
        // If the argument must be a subcommand.
        if !self.cmd.has_args() || self.cmd.is_infer_subcommands_set() && self.cmd.has_subcommands()
        {
            return ClapError::unrecognized_subcommand(
                self.cmd,
                arg_os.to_str_lossy().into_owned(),
                self.cmd
                    .get_bin_name()
                    .unwrap_or_else(|| self.cmd.get_name())
                    .to_owned(),
            );
        }
        ClapError::unknown_argument(
            self.cmd,
            arg_os.to_str_lossy().into_owned(),
            None,
            Usage::new(self.cmd).create_usage_with_title(&[]),
        )
    }

    // Checks if the arg matches a subcommand name, or any of its aliases (if defined)
    fn possible_subcommand(&self, arg_os: &RawOsStr, valid_arg_found: bool) -> Option<&str> {
        debug!("Parser::possible_subcommand: arg={:?}", arg_os);

        if !(self.cmd.is_args_conflicts_with_subcommands_set() && valid_arg_found) {
            if self.cmd.is_infer_subcommands_set() {
                // For subcommand `test`, we accepts it's prefix: `t`, `te`,
                // `tes` and `test`.
                let v = self
                    .cmd
                    .all_subcommand_names()
                    .filter(|s| RawOsStr::from_str(s).starts_with_os(arg_os))
                    .collect::<Vec<_>>();

                if v.len() == 1 {
                    return Some(v[0]);
                }

                // If there is any ambiguity, fallback to non-infer subcommand
                // search.
            }
            if let Some(sc) = self.cmd.find_subcommand(arg_os) {
                return Some(sc.get_name());
            }
        }
        None
    }

    // Checks if the arg matches a long flag subcommand name, or any of its aliases (if defined)
    fn possible_long_flag_subcommand(&self, arg_os: &RawOsStr) -> Option<&str> {
        debug!("Parser::possible_long_flag_subcommand: arg={:?}", arg_os);
        if self.cmd.is_infer_subcommands_set() {
            let options = self
                .cmd
                .get_subcommands()
                .fold(Vec::new(), |mut options, sc| {
                    if let Some(long) = sc.get_long_flag() {
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
        } else if let Some(sc_name) = self.cmd.find_long_subcmd(arg_os) {
            return Some(sc_name);
        }
        None
    }

    fn parse_help_subcommand(&self, cmds: &[OsString]) -> ClapResult<ParseResult> {
        debug!("Parser::parse_help_subcommand");

        let mut bin_name = self
            .cmd
            .get_bin_name()
            .unwrap_or_else(|| self.cmd.get_name())
            .to_owned();

        let mut sc = {
            let mut sc = self.cmd.clone();

            for cmd in cmds.iter() {
                sc = if let Some(c) = sc.find_subcommand(cmd) {
                    c
                } else if let Some(c) = sc.find_subcommand(&cmd.to_string_lossy()) {
                    c
                } else {
                    return Err(ClapError::unrecognized_subcommand(
                        self.cmd,
                        cmd.to_string_lossy().into_owned(),
                        self.cmd
                            .get_bin_name()
                            .unwrap_or_else(|| self.cmd.get_name())
                            .to_owned(),
                    ));
                }
                .clone();

                sc._build();
                bin_name.push(' ');
                bin_name.push_str(sc.get_name());
            }

            sc
        };
        sc = sc.bin_name(bin_name);

        let parser = Parser::new(&mut sc);

        Err(parser.help_err(true))
    }

    fn is_new_arg(&self, next: &RawOsStr, current_positional: &Arg) -> bool {
        debug!(
            "Parser::is_new_arg: {:?}:{:?}",
            next, current_positional.name
        );

        if self.cmd.is_allow_hyphen_values_set()
            || self.cmd[&current_positional.id].is_allow_hyphen_values_set()
            || (self.cmd.is_allow_negative_numbers_set()
                && next.to_str_lossy().parse::<f64>().is_ok())
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

        let partial_parsing_enabled = self.cmd.is_ignore_errors_set();

        if let Some(sc) = self.cmd._build_subcommand(sc_name) {
            let mut sc_matcher = ArgMatcher::new(sc);

            debug!(
                "Parser::parse_subcommand: About to parse sc={}",
                sc.get_name()
            );

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
                id: sc.get_id(),
                name: sc.get_name().to_owned(),
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

        if let Some(help) = self.cmd.find(&Id::help_hash()) {
            if let Some(h) = help.long {
                if arg == h && !self.is_set(AS::NoAutoHelp) && !self.cmd.is_disable_help_flag_set()
                {
                    debug!("Help");
                    return Some(ParseResult::HelpFlag);
                }
            }
        }

        if let Some(version) = self.cmd.find(&Id::version_hash()) {
            if let Some(v) = version.long {
                if arg == v
                    && !self.is_set(AS::NoAutoVersion)
                    && !self.cmd.is_disable_version_flag_set()
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

        if let Some(help) = self.cmd.find(&Id::help_hash()) {
            if let Some(h) = help.short {
                if arg == h && !self.is_set(AS::NoAutoHelp) && !self.cmd.is_disable_help_flag_set()
                {
                    debug!("Help");
                    return Some(ParseResult::HelpFlag);
                }
            }
        }

        if let Some(version) = self.cmd.find(&Id::version_hash()) {
            if let Some(v) = version.short {
                if arg == v
                    && !self.is_set(AS::NoAutoVersion)
                    && !self.cmd.is_disable_version_flag_set()
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
        // specified by the user is sent through. If hide_short_help is not included,
        // then items specified with hidden_short_help will also be hidden.
        let should_long = |v: &Arg| {
            v.long_help.is_some()
                || v.is_hide_long_help_set()
                || v.is_hide_short_help_set()
                || cfg!(feature = "unstable-v4")
                    && v.possible_vals.iter().any(PossibleValue::should_show_help)
        };

        // Subcommands aren't checked because we prefer short help for them, deferring to
        // `cmd subcmd --help` for more.
        self.cmd.get_long_about().is_some()
            || self.cmd.get_before_long_help().is_some()
            || self.cmd.get_after_long_help().is_some()
            || self.cmd.get_arguments().any(should_long)
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
            self.cmd[opt].is_allow_hyphen_values_set())
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

        let opt = if let Some(opt) = self.cmd.get_keymap().get(&*arg.to_os_str()) {
            debug!(
                "Parser::parse_long_arg: Found valid opt or flag '{}'",
                opt.to_string()
            );
            Some(opt)
        } else if self.cmd.is_infer_long_args_set() {
            let arg_str = arg.to_str_lossy();
            self.cmd.get_arguments().find(|a| {
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
            if opt.is_takes_value_set() {
                debug!(
                    "Parser::parse_long_arg: Found an opt with value '{:?}'",
                    &val
                );
                self.parse_opt(val, opt, matcher, trailing_values)
            } else if let Some(rest) = val {
                let required = self.cmd.required_graph();
                debug!("Parser::parse_long_arg: Got invalid literal `{:?}`", rest);
                let used: Vec<Id> = matcher
                    .arg_names()
                    .filter(|&n| {
                        self.cmd
                            .find(n)
                            .map_or(true, |a| !(a.is_hide_set() || required.contains(&a.id)))
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
        } else if self.cmd.is_allow_hyphen_values_set() {
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
        if self.cmd.is_allow_negative_numbers_set() && arg.parse::<f64>().is_ok() {
            debug!("Parser::parse_short_arg: negative number");
            return ParseResult::MaybeHyphenValue;
        } else if self.cmd.is_allow_hyphen_values_set()
            && arg.chars().any(|c| !self.cmd.contains_short(c))
        {
            debug!("Parser::parse_short_args: contains non-short flag");
            return ParseResult::MaybeHyphenValue;
        } else if matches!(parse_state, ParseState::Opt(opt) | ParseState::Pos(opt)
                if self.cmd[opt].is_allow_hyphen_values_set())
        {
            debug!("Parser::parse_short_args: prior arg accepts hyphenated values",);
            return ParseResult::MaybeHyphenValue;
        } else if self
            .cmd
            .get_keymap()
            .get(&pos_counter)
            .map_or(false, |arg| {
                arg.is_allow_hyphen_values_set() && !arg.is_last_set()
            })
        {
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
            if let Some(opt) = self.cmd.get_keymap().get(&c) {
                debug!(
                    "Parser::parse_short_arg:iter:{}: Found valid opt or flag",
                    c
                );
                *valid_arg_found = true;
                self.seen.push(opt.id.clone());
                if !opt.is_takes_value_set() {
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
                // e.g. `-xvf`, when require_equals && x.min_vals == 0, we don't
                // consume the `vf`, even if it's provided as value.
                match self.parse_opt(val, opt, matcher, trailing_values) {
                    ParseResult::AttachedValueNotConsumed => continue,
                    x => return x,
                }
            }

            return if let Some(sc_name) = self.cmd.find_short_subcmd(c) {
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
        // require_equals is set, but no '=' is provided, try throwing error.
        if opt.is_require_equals_set() && !has_eq {
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
                        ValueSource::CommandLine,
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
                ValueSource::CommandLine,
                false,
                trailing_values,
            );
            ParseResult::ValuesDone
        } else {
            debug!("Parser::parse_opt: More arg vals required...");
            self.inc_occurrence_of_arg(matcher, opt);
            matcher.new_val_group(&opt.id);
            for group in self.cmd.groups_for_arg(&opt.id) {
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
        ty: ValueSource,
        append: bool,
        trailing_values: bool,
    ) -> ParseResult {
        debug!("Parser::add_val_to_arg; arg={}, val={:?}", arg.name, val);
        debug!(
            "Parser::add_val_to_arg; trailing_values={:?}, DontDelimTrailingVals={:?}",
            trailing_values,
            self.cmd.is_dont_delimit_trailing_values_set()
        );
        if !(trailing_values && self.cmd.is_dont_delimit_trailing_values_set()) {
            if let Some(delim) = arg.val_delim {
                let terminator = arg.terminator.map(OsStr::new);
                let vals = val
                    .split(delim)
                    .map(|x| x.to_os_str().into_owned())
                    .take_while(|val| Some(val.as_os_str()) != terminator);
                self.add_multiple_vals_to_arg(arg, vals, matcher, ty, append);
                // If there was a delimiter used or we must use the delimiter to
                // separate the values or no more vals is needed, we're not
                // looking for more values.
                return if val.contains(delim)
                    || arg.is_require_value_delimiter_set()
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
        ty: ValueSource,
        append: bool,
    ) {
        // If not appending, create a new val group and then append vals in.
        if !append {
            matcher.new_val_group(&arg.id);
            for group in self.cmd.groups_for_arg(&arg.id) {
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
        ty: ValueSource,
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
        for group in self.cmd.groups_for_arg(&arg.id) {
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
        matcher.add_index_to(&flag.id, self.cur_idx.get(), ValueSource::CommandLine);

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
            if let Some(overrider) = self.cmd.find(arg_id) {
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

        for o in self.cmd.get_opts() {
            debug!("Parser::add_defaults:iter:{}:", o.name);
            self.add_value(o, matcher, ValueSource::DefaultValue, trailing_values);
        }

        for p in self.cmd.get_positionals() {
            debug!("Parser::add_defaults:iter:{}:", p.name);
            self.add_value(p, matcher, ValueSource::DefaultValue, trailing_values);
        }
    }

    fn add_value(
        &self,
        arg: &Arg<'help>,
        matcher: &mut ArgMatcher,
        ty: ValueSource,
        trailing_values: bool,
    ) {
        if !arg.default_vals_ifs.is_empty() {
            debug!("Parser::add_value: has conditional defaults");
            if matcher.get(&arg.id).is_none() {
                for (id, val, default) in arg.default_vals_ifs.iter() {
                    let add = if let Some(a) = matcher.get(id) {
                        match val {
                            crate::build::ArgPredicate::Equals(v) => {
                                a.vals_flatten().any(|value| v == value)
                            }
                            crate::build::ArgPredicate::IsPresent => true,
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

        self.cmd.get_arguments().try_for_each(|a| {
            // Use env only if the arg was absent among command line args,
            // early return if this is not the case.
            if matcher
                .get(&a.id)
                .map_or(false, |a| a.get_occurrences() != 0)
            {
                debug!("Parser::add_env: Skipping existing arg `{}`", a);
                return Ok(());
            }

            debug!("Parser::add_env: Checking arg `{}`", a);
            if let Some((_, Some(ref val))) = a.env {
                let val = RawOsStr::new(val);

                if a.is_takes_value_set() {
                    debug!(
                        "Parser::add_env: Found an opt with value={:?}, trailing={:?}",
                        val, trailing_values
                    );
                    self.add_val_to_arg(
                        a,
                        &val,
                        matcher,
                        ValueSource::EnvVariable,
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
                    matcher.add_index_to(&a.id, self.cur_idx.get(), ValueSource::EnvVariable);
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
        for group in self.cmd.groups_for_arg(&arg.id) {
            matcher.inc_occurrence_of_group(&group);
        }
    }
}

// Error, Help, and Version Methods
impl<'help, 'cmd> Parser<'help, 'cmd> {
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
            .cmd
            .get_keymap()
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
            self.cmd.get_subcommands_mut(),
        );

        // Add the arg to the matches to build a proper usage string
        if let Some((name, _)) = did_you_mean.as_ref() {
            if let Some(opt) = self.cmd.get_keymap().get(&name.as_ref()) {
                self.inc_occurrence_of_arg(matcher, opt);
            }
        }

        let required = self.cmd.required_graph();
        let used: Vec<Id> = matcher
            .arg_names()
            .filter(|n| {
                self.cmd
                    .find(n)
                    .map_or(true, |a| !(required.contains(&a.id) || a.is_hide_set()))
            })
            .cloned()
            .collect();

        ClapError::unknown_argument(
            self.cmd,
            format!("--{}", arg),
            did_you_mean,
            Usage::new(self.cmd)
                .required(&required)
                .create_usage_with_title(&*used),
        )
    }

    pub(crate) fn write_help_err(&self) -> ClapResult<Colorizer> {
        let usage = Usage::new(self.cmd);
        let mut c = Colorizer::new(true, self.color_help());
        Help::new(HelpWriter::Buffer(&mut c), self.cmd, &usage, false).write_help()?;
        Ok(c)
    }

    fn help_err(&self, mut use_long: bool) -> ClapError {
        debug!(
            "Parser::help_err: use_long={:?}",
            use_long && self.use_long_help()
        );

        use_long = use_long && self.use_long_help();
        let usage = Usage::new(self.cmd);
        let mut c = Colorizer::new(false, self.color_help());

        match Help::new(HelpWriter::Buffer(&mut c), self.cmd, &usage, use_long).write_help() {
            Err(e) => e.into(),
            _ => ClapError::display_help(self.cmd, c),
        }
    }

    fn version_err(&self, use_long: bool) -> ClapError {
        debug!("Parser::version_err");

        let msg = self.cmd._render_version(use_long);
        let mut c = Colorizer::new(false, self.color_help());
        c.none(msg);
        ClapError::display_version(self.cmd, c)
    }
}

// Query Methods
impl<'help, 'cmd> Parser<'help, 'cmd> {
    pub(crate) fn is_set(&self, s: AS) -> bool {
        self.cmd.is_set(s)
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
