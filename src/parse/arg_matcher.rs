// Std
use std::{collections::HashMap, ffi::OsString, mem, ops::Deref};

// Internal
use crate::{
    build::{App, Arg, ArgSettings},
    parse::{ArgMatches, MatchedArg, SubCommand, ValueType},
    util::Id,
};

// Third party
use indexmap::map::Entry;

#[derive(Debug, Default)]
pub(crate) struct ArgMatcher(pub(crate) ArgMatches);

impl ArgMatcher {
    pub(crate) fn new(_app: &App) -> Self {
        ArgMatcher(ArgMatches {
            #[cfg(debug_assertions)]
            valid_args: {
                let args = _app.args.args().map(|a| a.id.clone());
                let groups = _app.groups.iter().map(|g| g.id.clone());
                args.chain(groups).collect()
            },
            #[cfg(debug_assertions)]
            valid_subcommands: _app.subcommands.iter().map(|sc| sc.id.clone()).collect(),
            // HACK: Allow an external subcommand's ArgMatches be a stand-in for any ArgMatches
            // since users can't detect it and avoid the asserts.
            //
            // See clap-rs/clap#3263
            #[cfg(debug_assertions)]
            disable_asserts: _app.is_set(crate::AppSettings::AllowExternalSubcommands),
            ..Default::default()
        })
    }

    pub(crate) fn into_inner(self) -> ArgMatches {
        self.0
    }

    pub(crate) fn propagate_globals(&mut self, global_arg_vec: &[Id]) {
        debug!(
            "ArgMatcher::get_global_values: global_arg_vec={:?}",
            global_arg_vec
        );
        let mut vals_map = HashMap::new();
        self.fill_in_global_values(global_arg_vec, &mut vals_map);
    }

    fn fill_in_global_values(
        &mut self,
        global_arg_vec: &[Id],
        vals_map: &mut HashMap<Id, MatchedArg>,
    ) {
        for global_arg in global_arg_vec {
            if let Some(ma) = self.get(global_arg) {
                // We have to check if the parent's global arg wasn't used but still exists
                // such as from a default value.
                //
                // For example, `myprog subcommand --global-arg=value` where --global-arg defines
                // a default value of `other` myprog would have an existing MatchedArg for
                // --global-arg where the value is `other`, however the occurs will be 0.
                let to_update = if let Some(parent_ma) = vals_map.get(global_arg) {
                    if parent_ma.occurs > 0 && ma.occurs == 0 {
                        parent_ma.clone()
                    } else {
                        ma.clone()
                    }
                } else {
                    ma.clone()
                };
                vals_map.insert(global_arg.clone(), to_update);
            }
        }
        if let Some(ref mut sc) = self.0.subcommand {
            let mut am = ArgMatcher(mem::take(&mut sc.matches));
            am.fill_in_global_values(global_arg_vec, vals_map);
            mem::swap(&mut am.0, &mut sc.matches);
        }

        for (name, matched_arg) in vals_map.iter_mut() {
            self.0.args.insert(name.clone(), matched_arg.clone());
        }
    }

    pub(crate) fn get_mut(&mut self, arg: &Id) -> Option<&mut MatchedArg> {
        self.0.args.get_mut(arg)
    }

    pub(crate) fn get(&self, arg: &Id) -> Option<&MatchedArg> {
        self.0.args.get(arg)
    }

    pub(crate) fn remove(&mut self, arg: &Id) {
        self.0.args.swap_remove(arg);
    }

    pub(crate) fn contains(&self, arg: &Id) -> bool {
        self.0.args.contains_key(arg)
    }

    pub(crate) fn contains_explicit(&self, arg: &Id) -> bool {
        self.0
            .args
            .get(arg)
            .map_or(false, |a| a.ty != ValueType::DefaultValue)
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.0.args.is_empty()
    }

    pub(crate) fn arg_names(&self) -> indexmap::map::Keys<Id, MatchedArg> {
        self.0.args.keys()
    }

    pub(crate) fn entry(&mut self, arg: &Id) -> indexmap::map::Entry<Id, MatchedArg> {
        self.0.args.entry(arg.clone())
    }

    pub(crate) fn subcommand(&mut self, sc: SubCommand) {
        self.0.subcommand = Some(Box::new(sc));
    }

    pub(crate) fn subcommand_name(&self) -> Option<&str> {
        self.0.subcommand_name()
    }

    pub(crate) fn iter(&self) -> indexmap::map::Iter<Id, MatchedArg> {
        self.0.args.iter()
    }

    pub(crate) fn inc_occurrence_of_arg(&mut self, arg: &Arg) {
        let id = &arg.id;
        debug!("ArgMatcher::inc_occurrence_of_arg: id={:?}", id);
        let ma = self.entry(id).or_insert(MatchedArg::new());
        ma.set_ty(ValueType::CommandLine);
        ma.set_ignore_case(arg.is_set(ArgSettings::IgnoreCase));
        ma.invalid_utf8_allowed(arg.is_set(ArgSettings::AllowInvalidUtf8));
        ma.occurs += 1;
    }

    pub(crate) fn inc_occurrence_of_group(&mut self, id: &Id) {
        debug!("ArgMatcher::inc_occurrence_of_group: id={:?}", id);
        let ma = self.entry(id).or_insert(MatchedArg::new());
        ma.set_ty(ValueType::CommandLine);
        ma.occurs += 1;
    }

    pub(crate) fn add_val_to(&mut self, arg: &Id, val: OsString, ty: ValueType, append: bool) {
        if append {
            self.append_val_to(arg, val, ty);
        } else {
            self.push_val_to(arg, val, ty);
        }
    }

    fn push_val_to(&mut self, arg: &Id, val: OsString, ty: ValueType) {
        // We will manually inc occurrences later(for flexibility under
        // specific circumstances, like only add one occurrence for flag
        // when we met: `--flag=one,two`).
        let ma = self.entry(arg).or_default();
        ma.set_ty(ty);
        ma.push_val(val);
    }

    fn append_val_to(&mut self, arg: &Id, val: OsString, ty: ValueType) {
        let ma = self.entry(arg).or_default();
        ma.set_ty(ty);
        ma.append_val(val);
    }

    pub(crate) fn new_val_group(&mut self, arg: &Id) {
        let ma = self.entry(arg).or_default();
        ma.new_val_group();
    }

    pub(crate) fn add_index_to(&mut self, arg: &Id, idx: usize, ty: ValueType) {
        let ma = self.entry(arg).or_default();
        ma.set_ty(ty);
        ma.push_index(idx);
    }

    pub(crate) fn has_val_groups(&mut self, arg: &Id) -> bool {
        match self.entry(arg) {
            Entry::Occupied(e) => e.get().has_val_groups(),
            Entry::Vacant(_) => false,
        }
    }

    pub(crate) fn needs_more_vals(&self, o: &Arg) -> bool {
        debug!("ArgMatcher::needs_more_vals: o={}", o.name);
        if let Some(ma) = self.get(&o.id) {
            let current_num = ma.num_vals();
            if let Some(num) = o.num_vals {
                debug!("ArgMatcher::needs_more_vals: num_vals...{}", num);
                return if o.is_set(ArgSettings::MultipleOccurrences) {
                    (current_num % num) != 0
                } else {
                    num != current_num
                };
            } else if let Some(num) = o.max_vals {
                debug!("ArgMatcher::needs_more_vals: max_vals...{}", num);
                return current_num < num;
            } else if o.min_vals.is_some() {
                debug!("ArgMatcher::needs_more_vals: min_vals...true");
                return true;
            }
            return o.is_set(ArgSettings::MultipleValues);
        }
        true
    }
}

impl Deref for ArgMatcher {
    type Target = ArgMatches;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
