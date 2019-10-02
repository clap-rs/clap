// Std
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
use crate::mkeymap::KeyType;
use crate::output::Help;
use crate::output::Usage;
use crate::parse::errors::Error as ClapError;
use crate::parse::errors::ErrorKind;
use crate::parse::errors::Result as ClapResult;
use crate::parse::features::suggestions;
use crate::parse::Validator;
use crate::parse::{ArgMatcher, SubCommand};
use crate::util::{self, ChildGraph, Key, OsStrExt2, EMPTY_HASH};
#[cfg(all(feature = "debug", any(target_os = "windows", target_arch = "wasm32")))]
use crate::util::OsStrExt3;
use crate::INTERNAL_ERROR_MSG;
use crate::INVALID_UTF8;

type Id = u64;

#[derive(Debug, PartialEq, Copy, Clone)]
#[doc(hidden)]
pub enum ParseResult {
    Flag,
    Opt(Id),
    Pos(Id),
    MaybeHyphenValue,
    MaybeNegNum,
    NotFound,
    ValuesDone,
}

#[doc(hidden)]
pub struct Parser<'b, 'c>
where
    'b: 'c,
{
    pub app: &'c mut App<'b>,
    pub required: ChildGraph<Id>,
    pub overriden: Vec<Id>,
    seen: Vec<Id>,
    cur_idx: Cell<usize>,
}

// Initializing Methods
impl<'b, 'c> Parser<'b, 'c>
where
    'b: 'c,
{
    pub fn new(app: &'c mut App<'b>) -> Self {
        let mut reqs = ChildGraph::with_capacity(5);
        for a in app
            .args
            .args
            .iter()
            .filter(|a| a.settings.is_set(ArgSettings::Required))
            .map(|a| a.id)
        {
            reqs.insert(a);
        }

        Parser {
            app,
            required: reqs,
            overriden: Vec::new(),
            seen: Vec::new(),
            cur_idx: Cell::new(0),
        }
    }

    #[cfg_attr(feature = "lints", allow(block_in_if_condition_stmt))]
    fn _verify_positionals(&mut self) -> bool {
        debugln!("Parser::_verify_positionals;");
        // Because you must wait until all arguments have been supplied, this is the first chance
        // to make assertions on positional argument indexes
        //
        // Firt we verify that the index highest supplied index, is equal to the number of
        // positional arguments to verify there are no gaps (i.e. supplying an index of 1 and 3
        // but no 2)

        // #[cfg(feature = "vec_map")]
        // fn _highest_idx(map: &VecMap<&str>) -> usize { map.keys().last().unwrap_or(0) }

        // #[cfg(not(feature = "vec_map"))]
        // fn _highest_idx(map: &VecMap<&str>) -> usize { *map.keys().last().unwrap_or(&0) }

        let highest_idx = *self
            .app
            .args
            .keys
            .iter()
            .map(|x| &x.key)
            .filter_map(|x| {
                if let KeyType::Position(n) = x {
                    Some(n)
                } else {
                    None
                }
            })
            .max()
            .unwrap_or(&0);

        //_highest_idx(&self.positionals);

        let num_p = self
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

        assert!(
            highest_idx == num_p as u64,
            "Found positional argument whose index is {} but there \
             are only {} positional arguments defined",
            highest_idx,
            num_p
        );

        // Next we verify that only the highest index has a .multiple(true) (if any)
        let only_highest = |a: &Arg| {
            a.is_set(ArgSettings::MultipleValues) && (a.index.unwrap_or(0) != highest_idx)
        };
        if positionals!(self.app).any(only_highest) {
            // First we make sure if there is a positional that allows multiple values
            // the one before it (second to last) has one of these:
            //  * a value terminator
            //  * ArgSettings::Last
            //  * The last arg is Required

            // We can't pass the closure (it.next()) to the macro directly because each call to
            // find() (iterator, not macro) gets called repeatedly.
            let last = self
                .app
                .args
                .get(&KeyType::Position(highest_idx))
                .expect(INTERNAL_ERROR_MSG);

            let second_to_last = self
                .app
                .args
                .get(&KeyType::Position(highest_idx - 1))
                .expect(INTERNAL_ERROR_MSG);

            // Either the final positional is required
            // Or the second to last has a terminator or .last(true) set
            let ok = last.is_set(ArgSettings::Required)
                || (second_to_last.terminator.is_some()
                    || second_to_last.is_set(ArgSettings::Last))
                || last.is_set(ArgSettings::Last);
            assert!(
                ok,
                "When using a positional argument with .multiple(true) that is *not the \
                 last* positional argument, the last positional argument (i.e the one \
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
            let count = positionals!(self.app).fold(0, |acc, p| {
                if p.settings.is_set(ArgSettings::MultipleValues) && p.num_vals.is_none() {
                    acc + 1
                } else {
                    acc
                }
            });
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

            for p in positionals!(self.app) {
                if foundx2 && !p.is_set(ArgSettings::Required) {
                    assert!(
                        p.is_set(ArgSettings::Required),
                        "Found positional argument which is not required with a lower \
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
            for p in (1..=num_p)
                .rev()
                .filter_map(|n| self.app.args.get(&KeyType::Position(n as u64)))
            {
                if found {
                    assert!(
                        p.is_set(ArgSettings::Required),
                        "Found positional argument which is not required with a lower \
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
            positionals!(self.app).fold(0, |acc, p| if p.is_set(ArgSettings::Last) {
                acc + 1
            } else {
                acc
            }) < 2,
            "Only one positional argument may have last(true) set. Found two."
        );
        if positionals!(self.app)
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

    // Does all the initializing and prepares the parser
    pub(crate) fn _build(&mut self) {
        debugln!("Parser::_build;");

        //I wonder whether this part is even needed if we insert all Args using make_entries
        let mut key: Vec<(KeyType, usize)> = Vec::new();
        let mut counter = 0;
        for (i, a) in self.app.args.args.iter_mut().enumerate() {
            if a.index == None && a.short == None && a.long == None {
                counter += 1;
                a.index = Some(counter);
                key.push((KeyType::Position(counter), i));
            }

            // Add args with default requirements
            if a.is_set(ArgSettings::Required) {
                debugln!("Parser::_build: adding {} to default requires", a.name);
                let idx = self.required.insert(a.id);
                // If the arg is required, add all it's requirements to master required list
                if let Some(ref areqs) = a.requires {
                    for name in areqs
                        .iter()
                        .filter(|&&(val, _)| val.is_none())
                        .map(|&(_, name)| name)
                    {
                        self.required.insert_child(idx, name);
                    }
                }
            }
        }
        for (k, i) in key.into_iter() {
            self.app.args.insert_key(k, i);
        }

        debug_assert!(self._verify_positionals());
        // Set the LowIndexMultiple flag if required
        if positionals!(self.app).any(|a| {
            a.is_set(ArgSettings::MultipleValues)
                && (a.index.unwrap_or(0) as usize
                    != self
                        .app
                        .args
                        .keys
                        .iter()
                        .map(|x| &x.key)
                        .filter(|x| x.is_position())
                        .count())
        }) && positionals!(self.app).last().map_or(false, |p_name| {
            !self
                .app
                .find(p_name.id)
                .expect(INTERNAL_ERROR_MSG)
                .is_set(ArgSettings::Last)
        }) {
            self.app.settings.set(AS::LowIndexMultiplePositional);
        }

        for group in &self.app.groups {
            if group.required {
                let idx = self.required.insert(group.id);
                if let Some(ref reqs) = group.requires {
                    for &a in reqs {
                        self.required.insert_child(idx, a);
                    }
                }
            }
        }
    }
}

// Parsing Methods
impl<'b, 'c> Parser<'b, 'c>
where
    'b: 'c,
{
    // The actual parsing function
    #[allow(clippy::cyclomatic_complexity)]
    pub fn get_matches_with<I, T>(
        &mut self,
        matcher: &mut ArgMatcher,
        it: &mut Peekable<I>,
    ) -> ClapResult<()>
    where
        I: Iterator<Item = T>,
        T: Into<OsString> + Clone,
    {
        debugln!("Parser::get_matches_with;");
        // Verify all positional assertions pass
        self._build();

        let has_args = self.has_args();

        let mut subcmd_name: Option<String> = None;
        let mut needs_val_of: ParseResult = ParseResult::NotFound;
        let mut pos_counter = 1;
        while let Some(arg) = it.next() {
            let arg_os = arg.into();
            debugln!(
                "Parser::get_matches_with: Begin parsing '{:?}' ({:?})",
                arg_os,
                &*arg_os.as_bytes()
            );

            self.unset(AS::ValidNegNumFound);
            // Is this a new argument, or values from a previous option?
            let starts_new_arg = self.is_new_arg(&arg_os, needs_val_of);
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
                                if sc_name == "help" && !self.is_set(AS::NoAutoHelp) {
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
                                        None,
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
                            ParseResult::Pos(p.id)
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
                        None,
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
                self.seen.push(p.id);
                let _ = self.add_val_to_arg(p, &arg_os, matcher)?;

                matcher.inc_occurrence_of(p.id);
                for grp in groups_for_arg!(self.app, p.id) {
                    matcher.inc_occurrence_of(grp);
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
                    sc_m.add_val_to(EMPTY_HASH, &a);
                }
                let id = sc_name.key();
                matcher.subcommand(SubCommand {
                    name: sc_name,
                    id,
                    matches: sc_m.into_inner(),
                });
                break;
            } else if !((self.is_set(AS::AllowLeadingHyphen)
                || self.is_set(AS::AllowNegativeNumbers))
                && arg_os.starts_with(b"-"))
                && !self.is_set(AS::InferSubcommands)
            {
                return Err(ClapError::unknown_argument(
                    &*arg_os.to_string_lossy(),
                    None,
                    &*Usage::new(self).create_usage_with_title(&[]),
                    self.app.color(),
                ));
            } else if !has_args || self.is_set(AS::InferSubcommands) && self.has_subcommands() {
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
                    None,
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
    fn possible_subcommand(&self, arg_os: &OsStr) -> (bool, Option<&str>) {
        debugln!("Parser::possible_subcommand: arg={:?}", arg_os);
        fn starts(h: &str, n: &OsStr) -> bool {
            #[cfg(target_os = "windows")]
            use crate::util::OsStrExt3;
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
        let cmds: Vec<OsString> = it.map(Into::into).collect();
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
                if let Some(id) = find_subcmd!(sc, cmd).map(|x| x.id) {
                    sc._propagate(Propagation::To(id));
                }
                if let Some(mut c) = find_subcmd_cloned!(sc, cmd) {
                    c._build();
                    sc = c;
                    if i == cmds.len() - 1 {
                        break;
                    }
                } else if let Some(mut c) = find_subcmd_cloned!(sc, &*cmd.to_string_lossy()) {
                    c._build();
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
            let mut pb = Arg::with_name("subcommand")
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
    #[allow(clippy::wrong_self_convention)]
    fn is_new_arg(&mut self, arg_os: &OsStr, needs_val_of: ParseResult) -> bool {
        debugln!("Parser::is_new_arg:{:?}:{:?}", arg_os, needs_val_of);
        let app_wide_settings = if self.is_set(AS::AllowLeadingHyphen) {
            true
        } else if self.is_set(AS::AllowNegativeNumbers) {
            let a = arg_os.to_string_lossy();
            if a.parse::<i64>().is_ok() || a.parse::<f64>().is_ok() {
                self.set(AS::ValidNegNumFound);
                true
            } else {
                false
            }
        } else {
            false
        };
        let arg_allows_tac = match needs_val_of {
            ParseResult::Opt(name) => {
                let o = self.app.find(name).expect(INTERNAL_ERROR_MSG);
                (o.is_set(ArgSettings::AllowHyphenValues) || app_wide_settings)
            }
            ParseResult::Pos(name) => {
                let p = self.app.find(name).expect(INTERNAL_ERROR_MSG);
                (p.is_set(ArgSettings::AllowHyphenValues) || app_wide_settings)
            }
            ParseResult::ValuesDone => return true,
            _ => false,
        };
        debugln!("Parser::is_new_arg: arg_allows_tac={:?}", arg_allows_tac);

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
        sc_name: &str,
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
        if let Some(id) = find_subcmd!(self.app, sc_name).map(|x| x.id) {
            self.app._propagate(Propagation::To(id));
        }
        if let Some(sc) = subcommands_mut!(self.app).find(|s| s.name == sc_name) {
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
            sc._build();

            debugln!("Parser::parse_subcommand: About to parse sc={}", sc.name);

            {
                let mut p = Parser::new(sc);
                p.get_matches_with(&mut sc_matcher, it)?;
            }
            let name = sc.name.clone();
            let sc_id = name.key();
            matcher.subcommand(SubCommand {
                id: sc_id, // @TODO @maybe: should be sc.id?
                name,
                matches: sc_matcher.into_inner(),
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
        if let Some(help) = self.app.find(util::HELP_HASH) {
            if let Some(h) = help.short {
                if arg == h && !self.app.settings.is_set(AS::NoAutoHelp) {
                    sdebugln!("Help");
                    return Err(self.help_err(false));
                }
            }
        }
        if let Some(version) = self.app.find(util::VERSION_HASH) {
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

    fn parse_long_arg(
        &mut self,
        matcher: &mut ArgMatcher,
        full_arg: &OsStr,
    ) -> ClapResult<ParseResult> {
        // maybe here lifetime should be 'a
        debugln!("Parser::parse_long_arg;");

        // Update the curent index
        self.cur_idx.set(self.cur_idx.get() + 1);

        let mut val = None;
        debug!("Parser::parse_long_arg: Does it contain '='...");
        let arg = if full_arg.contains_byte(b'=') {
            let (p0, p1) = full_arg.trim_left_matches(b'-').split_at_byte(b'=');
            sdebugln!("Yes '{:?}'", p1);
            val = Some(p1);
            p0
        } else {
            sdebugln!("No");
            full_arg.trim_left_matches(b'-')
        };
        if let Some(opt) = self.app.args.get(&KeyType::Long(arg.into())) {
            debugln!(
                "Parser::parse_long_arg: Found valid opt or flag '{}'",
                opt.to_string()
            );
            self.app.settings.set(AS::ValidArgFound);

            self.seen.push(opt.id);

            if opt.is_set(ArgSettings::TakesValue) {
                return Ok(self.parse_opt(val, opt, val.is_some(), matcher)?);
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
                self.seen.push(opt.id);
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
                    None,
                    &*Usage::new(self).create_usage_with_title(&[]),
                    self.app.color(),
                ));
            }
        }
        Ok(ret)
    }

    fn parse_opt(
        &self,
        val: Option<&OsStr>,
        opt: &Arg<'b>,
        had_eq: bool,
        matcher: &mut ArgMatcher,
    ) -> ClapResult<ParseResult> {
        debugln!("Parser::parse_opt; opt={}, val={:?}", opt.name, val);
        debugln!("Parser::parse_opt; opt.settings={:?}", opt.settings);
        let mut has_eq = false;
        let no_val = val.is_none();
        let empty_vals = opt.is_set(ArgSettings::AllowEmptyValues);
        let min_vals_zero = opt.min_vals.unwrap_or(1) == 0;
        let needs_eq = opt.is_set(ArgSettings::RequireEquals);

        debug!("Parser::parse_opt; Checking for val...");
        if let Some(fv) = val {
            has_eq = fv.starts_with(&[b'=']) || had_eq;
            let v = fv.trim_left_matches(b'=');
            if !empty_vals && (v.is_empty() || (needs_eq && !has_eq)) {
                sdebugln!("Found Empty - Error");
                return Err(ClapError::empty_value(
                    opt,
                    &*Usage::new(self).create_usage_with_title(&[]),
                    self.app.color(),
                ));
            }
            sdebugln!("Found - {:?}, len: {}", v, v.len());
            debugln!(
                "Parser::parse_opt: {:?} contains '='...{:?}",
                fv,
                fv.starts_with(&[b'='])
            );
            self.add_val_to_arg(opt, v, matcher)?;
        } else if needs_eq && !(empty_vals || min_vals_zero) {
            sdebugln!("None, but requires equals...Error");
            return Err(ClapError::empty_value(
                opt,
                &*Usage::new(self).create_usage_with_title(&[]),
                self.app.color(),
            ));
        } else {
            sdebugln!("None");
        }

        matcher.inc_occurrence_of(opt.id);
        // Increment or create the group "args"
        for grp in groups_for_arg!(self.app, opt.id) {
            matcher.inc_occurrence_of(grp);
        }

        let needs_delim = opt.is_set(ArgSettings::RequireDelimiter);
        let mult = opt.is_set(ArgSettings::MultipleValues);
        // @TODO @soundness: if doesn't have an equal, but requires equal is ValuesDone?!
        if no_val && min_vals_zero && !has_eq && needs_eq {
            debugln!("Parser::parse_opt: More arg vals not required...");
            return Ok(ParseResult::ValuesDone);
        } else if no_val || (mult && !needs_delim) && !has_eq && matcher.needs_more_vals(opt) {
            debugln!("Parser::parse_opt: More arg vals required...");
            return Ok(ParseResult::Opt(opt.id));
        }
        debugln!("Parser::parse_opt: More arg vals not required...");
        Ok(ParseResult::ValuesDone)
    }

    fn add_val_to_arg(
        &self,
        arg: &Arg<'b>,
        val: &OsStr,
        matcher: &mut ArgMatcher,
    ) -> ClapResult<ParseResult> {
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
        arg: &Arg<'b>,
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

        matcher.add_val_to(arg.id, v);
        matcher.add_index_to(arg.id, self.cur_idx.get());

        // Increment or create the group "args"
        for grp in groups_for_arg!(self.app, arg.id) {
            matcher.add_val_to(grp, v);
        }

        if matcher.needs_more_vals(arg) {
            return Ok(ParseResult::Opt(arg.id));
        }
        Ok(ParseResult::ValuesDone)
    }

    fn parse_flag(&self, flag: &Arg<'b>, matcher: &mut ArgMatcher) -> ClapResult<ParseResult> {
        debugln!("Parser::parse_flag;");

        matcher.inc_occurrence_of(flag.id);
        matcher.add_index_to(flag.id, self.cur_idx.get());
        // Increment or create the group "args"
        for grp in groups_for_arg!(self.app, flag.id) {
            matcher.inc_occurrence_of(grp);
        }

        Ok(ParseResult::Flag)
    }

    fn remove_overrides(&mut self, matcher: &mut ArgMatcher) {
        debugln!("Parser::remove_overrides;");
        let mut to_rem: Vec<Id> = Vec::new();
        let mut self_override: Vec<Id> = Vec::new();
        let mut arg_overrides = Vec::new();
        for &name in matcher.arg_names() {
            debugln!("Parser::remove_overrides:iter:{};", name);
            if let Some(arg) = self.app.find(name) {
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
                    for &o in overrides {
                        if o == arg.id {
                            if handle_self_override(o) {
                                continue;
                            }
                        } else {
                            arg_overrides.push((arg.id, o));
                            arg_overrides.push((o, arg.id));
                        }
                    }
                }
                if self.is_set(AS::AllArgsOverrideSelf) {
                    let _ = handle_self_override(arg.id);
                }
            }
        }

        // remove future overrides in reverse seen order
        for &arg in self.seen.iter().rev() {
            for &(a, overr) in arg_overrides.iter().filter(|&&(a, _)| a == arg) {
                if !to_rem.contains(&a) {
                    to_rem.push(overr);
                }
            }
        }

        // Do self overrides
        for &name in &self_override {
            debugln!("Parser::remove_overrides:iter:self:{}: resetting;", name);
            if let Some(ma) = matcher.get_mut(name) {
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
        for &name in &to_rem {
            debugln!("Parser::remove_overrides:iter:{}: removing;", name);
            matcher.remove(name);
            self.overriden.push(name);
        }
    }

    pub(crate) fn add_defaults(&mut self, matcher: &mut ArgMatcher) -> ClapResult<()> {
        debugln!("Parser::add_defaults;");
        macro_rules! add_val {
            (@default $_self:ident, $a:ident, $m:ident) => {
                if let Some(ref val) = $a.default_val {
                    debugln!("Parser::add_defaults:iter:{}: has default vals", $a.name);
                    if $m
                        .get($a.id)
                        .map(|ma| ma.vals.len())
                        .map(|len| len == 0)
                        .unwrap_or(false)
                    {
                        debugln!(
                            "Parser::add_defaults:iter:{}: has no user defined vals",
                            $a.name
                        );
                        $_self.add_val_to_arg($a, OsStr::new(val), $m)?;
                    } else if $m.get($a.id).is_some() {
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
                    if $m.get($a.id).is_none() {
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
impl<'b, 'c> Parser<'b, 'c>
where
    'b: 'c,
{
    fn did_you_mean_error(&mut self, arg: &str, matcher: &mut ArgMatcher) -> ClapResult<()> {
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
                for g in groups_for_arg!(self.app, opt.id) {
                    matcher.inc_occurrence_of(g);
                }
                matcher.insert(opt.id);
            }
        }

        let used: Vec<Id> = matcher
            .arg_names()
            .filter(|n| {
                if let Some(a) = self.app.find(**n) {
                    !(self.required.contains(a.id) || a.is_set(ArgSettings::Hidden))
                } else {
                    true
                }
            })
            .cloned()
            .collect();

        let did_you_mean_msg = if suffix.0.is_empty() {
            None
        } else {
            Some(suffix.0)
        };

        Err(ClapError::unknown_argument(
            &*format!("--{}", arg),
            did_you_mean_msg,
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
impl<'b, 'c> Parser<'b, 'c>
where
    'b: 'c,
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
