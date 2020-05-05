// Std
use std::{collections::HashMap, ffi::OsString, mem, ops::Deref};

// Internal
use crate::{
    build::{Arg, ArgSettings},
    parse::{ArgMatches, MatchedArg, SubCommand, ValueType},
    util::Id,
};

#[derive(Debug)]
pub(crate) struct ArgMatcher(pub(crate) ArgMatches);

impl Default for ArgMatcher {
    fn default() -> Self {
        ArgMatcher(ArgMatches::default())
    }
}

impl Deref for ArgMatcher {
    type Target = ArgMatches;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl ArgMatcher {
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

    pub(crate) fn insert(&mut self, name: &Id) {
        self.0.args.insert(name.clone(), MatchedArg::new());
    }

    pub(crate) fn contains(&self, arg: &Id) -> bool {
        self.0.args.contains_key(arg)
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

    pub(crate) fn inc_occurrence_of(&mut self, arg: &Id) {
        debug!("ArgMatcher::inc_occurrence_of: arg={:?}", arg);
        if let Some(a) = self.get_mut(arg) {
            a.occurs += 1;
            return;
        }
        debug!("ArgMatcher::inc_occurrence_of: first instance");
        self.insert(arg);
    }

    pub(crate) fn add_val_to(&mut self, arg: &Id, val: OsString, ty: ValueType) {
        let ma = self.entry(arg).or_insert(MatchedArg {
            occurs: 0, // @TODO @question Shouldn't this be 1 if we're already adding a value to this arg?
            ty,
            indices: Vec::with_capacity(1),
            vals: Vec::with_capacity(1),
        });
        ma.vals.push(val);
    }

    pub(crate) fn add_index_to(&mut self, arg: &Id, idx: usize, ty: ValueType) {
        let ma = self.entry(arg).or_insert(MatchedArg {
            occurs: 0,
            indices: Vec::with_capacity(1),
            vals: Vec::new(),
            ty,
        });
        ma.indices.push(idx);
    }

    pub(crate) fn needs_more_vals(&self, o: &Arg) -> bool {
        debug!("ArgMatcher::needs_more_vals: o={}", o.name);
        if let Some(ma) = self.get(&o.id) {
            if let Some(num) = o.num_vals {
                debug!("ArgMatcher::needs_more_vals: num_vals...{}", num);
                return if o.is_set(ArgSettings::MultipleValues) {
                    ((ma.vals.len() as u64) % num) != 0
                } else {
                    num != (ma.vals.len() as u64)
                };
            } else if let Some(num) = o.max_vals {
                debug!("ArgMatcher::needs_more_vals: max_vals...{}", num);
                return (ma.vals.len() as u64) < num;
            } else if o.min_vals.is_some() {
                debug!("ArgMatcher::needs_more_vals: min_vals...true");
                return true;
            }
            return o.is_set(ArgSettings::MultipleValues);
        }
        true
    }
}
