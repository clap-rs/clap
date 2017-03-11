macro_rules! remove_overriden {
    (@remove_requires $rem_from:expr, $a:ident.$ov:ident) => {
        if let Some(ora) = $a.$ov() {
            for i in (0 .. $rem_from.len()).rev() {
                let should_remove = ora.iter().any(|&(_, ref name)| name == &$rem_from[i]);
                if should_remove { $rem_from.swap_remove(i); }
            }
        }
    };
    (@remove $rem_from:expr, $a:ident.$ov:ident) => {
        if let Some(ora) = $a.$ov() {
            vec_remove_all!($rem_from, ora.iter());
        }
    };
    (@arg $_self:ident, $arg:ident) => {
        remove_overriden!(@remove_requires $_self.required, $arg.requires);
        remove_overriden!(@remove $_self.blacklist, $arg.blacklist);
        remove_overriden!(@remove $_self.overrides, $arg.overrides);
    };
    ($_self:ident, $name:expr) => {
        debugln!("remove_overriden!;");
        if let Some(o) = $_self.opts.iter() .find(|o| o.b.name == *$name) {
            remove_overriden!(@arg $_self, o);
        } else if let Some(f) = $_self.flags.iter() .find(|f| f.b.name == *$name) {
            remove_overriden!(@arg $_self, f);
        } else {
            let p = $_self.positionals.values()
                                      .find(|p| p.b.name == *$name)
                                      .expect(INTERNAL_ERROR_MSG);
            remove_overriden!(@arg $_self, p);
        }
    };
}

macro_rules! arg_post_processing {
    ($me:ident, $arg:ident, $matcher:ident) => {
        debugln!("arg_post_processing!;");
        // Handle POSIX overrides
        debug!("arg_post_processing!: Is '{}' in overrides...", $arg.to_string());
        if $me.overrides.contains(&$arg.name()) {
            if let Some(ref name) = find_name_from!($me, &$arg.name(), overrides, $matcher) {
                sdebugln!("Yes by {}", name);
                $matcher.remove(name);
                remove_overriden!($me, name);
            }
        } else { sdebugln!("No"); }

        // Add overrides
        debug!("arg_post_processing!: Does '{}' have overrides...", $arg.to_string());
        if let Some(or) = $arg.overrides() {
            sdebugln!("Yes");
            $matcher.remove_all(or);
            for pa in or { remove_overriden!($me, pa); }
            $me.overrides.extend(or);
            vec_remove_all!($me.required, or.iter());
        } else { sdebugln!("No"); }

        // Handle conflicts
        debug!("arg_post_processing!: Does '{}' have conflicts...", $arg.to_string());
        if let Some(bl) = $arg.blacklist() {
            sdebugln!("Yes");

            for c in bl {
                // Inject two-way conflicts
                debug!("arg_post_processing!: Has '{}' already been matched...", c);
                if $matcher.contains(c) {
                    sdebugln!("Yes");
                    // find who blacklisted us...
                    $me.blacklist.push(&$arg.b.name);
                } else {
                    sdebugln!("No");
                }
            }

            $me.blacklist.extend_from_slice(bl);
            vec_remove_all!($me.overrides, bl.iter());
            // vec_remove_all!($me.required, bl.iter());
        } else { sdebugln!("No"); }

        // Add all required args which aren't already found in matcher to the master
        // list
        debug!("arg_post_processing!: Does '{}' have requirements...", $arg.to_string());
        if let Some(reqs) = $arg.requires() {
            for n in reqs.iter()
                .filter(|&&(val, _)| val.is_none())
                .filter(|&&(_, req)| !$matcher.contains(&req))
                .map(|&(_, name)| name) {
                    
                $me.required.push(n);
            }
        } else { sdebugln!("No"); }

        _handle_group_reqs!($me, $arg);
    };
}

macro_rules! _handle_group_reqs{
    ($me:ident, $arg:ident) => ({
        use args::AnyArg;
        debugln!("_handle_group_reqs!;");
        for grp in $me.groups.iter() {
            let found = if grp.args.contains(&$arg.name()) {
                // vec_remove!($me.required, &$arg.name());
                if let Some(ref reqs) = grp.requires {
                    debugln!("_handle_group_reqs!: Adding {:?} to the required list", reqs);
                    $me.required.extend(reqs);
                }
                if let Some(ref bl) = grp.conflicts {
                    $me.blacklist.extend(bl);
                }
                true // What if arg is in more than one group with different reqs?
            } else {
                false
            };
            debugln!("_handle_group_reqs!:iter: grp={}, found={:?}", grp.name, found);
            if found {
                for i in (0 .. $me.required.len()).rev() {
                    let should_remove = grp.args.contains(&$me.required[i]);
                    if should_remove { $me.required.swap_remove(i); }
                }
                debugln!("_handle_group_reqs!:iter: Adding args from group to blacklist...{:?}", grp.args);
                if !grp.multiple {
                    $me.blacklist.extend(&grp.args);
                    vec_remove!($me.blacklist, &$arg.name());
                }
            }
        }
    })
}

macro_rules! parse_positional {
    (
        $_self:ident,
        $p:ident,
        $arg_os:ident,
        $pos_counter:ident,
        $matcher:ident
    ) => {
        debugln!("parse_positional!;");

        if !$_self.is_set(AS::TrailingValues) &&
           ($_self.is_set(AS::TrailingVarArg) &&
            $pos_counter == $_self.positionals.len()) {
            $_self.settings.set(AS::TrailingValues);
        }
        let _ = try!($_self.add_val_to_arg($p, &$arg_os, $matcher));

        $matcher.inc_occurrence_of($p.b.name);
        let _ = $_self.groups_for_arg($p.b.name)
                      .and_then(|vec| Some($matcher.inc_occurrences_of(&*vec)));
        if $_self.cache.map_or(true, |name| name != $p.b.name) {
            arg_post_processing!($_self, $p, $matcher);
            $_self.cache = Some($p.b.name);
        }

        $_self.settings.set(AS::ValidArgFound);
        // Only increment the positional counter if it doesn't allow multiples
        if !$p.b.settings.is_set(ArgSettings::Multiple) {
            $pos_counter += 1;
        }
    };
}
