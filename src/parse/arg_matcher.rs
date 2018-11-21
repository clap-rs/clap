// Std
use std::collections::HashMap;
use std::ffi::OsStr;
use std::mem;

// Third Party
use indexmap;

// Internal
use build::{Arg, ArgSettings};
use parse::{ArgMatches, MatchedArg, SubCommand};

#[doc(hidden)]
pub struct ArgMatcher(pub ArgMatches);

impl Default for ArgMatcher {
    fn default() -> Self { ArgMatcher(ArgMatches::default()) }
}

impl ArgMatcher {
    pub fn new() -> Self { ArgMatcher::default() }

    #[allow(dead_code)]
    pub fn is_present(&self, name: &str) -> bool { self.0.is_present(name) }

    pub fn propagate_globals(&mut self, global_arg_vec: &[u64]) {
        debugln!(
            "ArgMatcher::get_global_values: global_arg_vec={:?}",
            global_arg_vec
        );
        let mut vals_map = HashMap::new();
        self.fill_in_global_values(global_arg_vec, &mut vals_map);
    }

    fn fill_in_global_values(
        &mut self,
        global_arg_vec: &[u64],
        vals_map: &mut HashMap<u64, MatchedArg>,
    ) {
        for global_arg in global_arg_vec {
            if let Some(ma) = self.get(*global_arg) {
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
                vals_map.insert(*global_arg, to_update);
            }
        }
        if let Some(ref mut sc) = self.0.subcommand {
            let mut am = ArgMatcher(mem::replace(&mut sc.matches, ArgMatches::new()));
            am.fill_in_global_values(global_arg_vec, vals_map);
            mem::swap(&mut am.0, &mut sc.matches);
        }

        for (name, matched_arg) in vals_map.iter_mut() {
            self.0.args.insert(*name, matched_arg.clone());
        }
    }

    pub fn get_mut(&mut self, arg: u64) -> Option<&mut MatchedArg> { self.0.args.get_mut(&arg) }

    pub fn get(&self, arg: u64) -> Option<&MatchedArg> { self.0.args.get(&arg) }

    pub fn remove(&mut self, arg: u64) { self.0.args.remove(&arg); }

    #[allow(dead_code)]
    pub fn remove_all(&mut self, args: &[u64]) {
        for arg in args {
            self.0.args.remove(arg);
        }
    }

    pub fn insert(&mut self, name: u64) { self.0.args.insert(name, MatchedArg::new()); }

    pub fn contains(&self, arg: u64) -> bool { self.0.args.contains_key(&arg) }

    pub fn is_empty(&self) -> bool { self.0.args.is_empty() }

    pub fn arg_names(&self) -> indexmap::map::Keys<u64, MatchedArg> { self.0.args.keys() }

    pub fn entry(&mut self, arg: u64) -> indexmap::map::Entry<u64, MatchedArg> {
        self.0.args.entry(arg)
    }

    pub fn subcommand(&mut self, sc: SubCommand) { self.0.subcommand = Some(Box::new(sc)); }

    pub fn subcommand_name(&self) -> Option<&str> { self.0.subcommand_name() }

    pub fn iter(&self) -> indexmap::map::Iter<u64, MatchedArg> { self.0.args.iter() }

    pub fn inc_occurrence_of(&mut self, arg: u64) {
        debugln!("ArgMatcher::inc_occurrence_of: arg={}", arg);
        if let Some(a) = self.get_mut(arg) {
            a.occurs += 1;
            return;
        }
        debugln!("ArgMatcher::inc_occurrence_of: first instance");
        self.insert(arg);
    }

    pub fn add_val_to(&mut self, arg: u64, val: &OsStr) {
        let ma = self.entry(arg).or_insert(MatchedArg {
            occurs: 0,
            indices: Vec::with_capacity(1),
            vals: Vec::with_capacity(1),
        });
        ma.vals.push(val.to_owned());
    }

    pub fn add_index_to(&mut self, arg: u64, idx: usize) {
        let ma = self.entry(arg).or_insert(MatchedArg {
            occurs: 0,
            indices: Vec::with_capacity(1),
            vals: Vec::new(),
        });
        ma.indices.push(idx);
    }

    pub fn needs_more_vals<T>(&self, o: &Arg) -> bool where T: Eq + Copy {
        debugln!("ArgMatcher::needs_more_vals: o={}", o.name);
        if let Some(ma) = self.get(o.id) {
            if let Some(num) = o.num_vals {
                debugln!("ArgMatcher::needs_more_vals: num_vals...{}", num);
                return if o.is_set(ArgSettings::MultipleValues) {
                    ((ma.vals.len() as u64) % num) != 0
                } else {
                    num != (ma.vals.len() as u64)
                };
            } else if let Some(num) = o.max_vals {
                debugln!("ArgMatcher::needs_more_vals: max_vals...{}", num);
                return (ma.vals.len() as u64) <= num;
            } else if o.min_vals.is_some() {
                debugln!("ArgMatcher::needs_more_vals: min_vals...true");
                return true;
            }
            return o.is_set(ArgSettings::MultipleValues);
        }
        true
    }
}

impl Into<ArgMatches> for ArgMatcher {
    fn into(self) -> ArgMatches { self.0 }
}
