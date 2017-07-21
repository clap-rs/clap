macro_rules! remove_overriden {
    (@remove_requires $rem_from:expr, $a:ident.$ov:ident) => {
        if let Some(ref ora) = $a.$ov {
            for i in (0 .. $rem_from.len()).rev() {
                let should_remove = ora.iter().any(|name| &name == &&$rem_from[i]);
                if should_remove { $rem_from.swap_remove(i); }
            }
        }
    };
    (@remove $rem_from:expr, $a:ident.$ov:ident) => {
        if let Some(ref ora) = $a.$ov {
            vec_remove_all!($rem_from, ora.iter());
        }
    };
    (@arg $parser:ident, $arg:ident) => {
        remove_overriden!(@remove_requires $parser.required, $arg.requires);
        remove_overriden!(@remove $parser.conflicts, $arg.conflicts_with);
        remove_overriden!(@remove $parser.overrides, $arg.overrides_with);
    };
    ($parser:ident, $name:expr) => {
        debugln!("remove_overriden!;");
        if let Some(a) = args!($parser.app).find(|a| &a.name == $name) {
            remove_overriden!(@arg $parser, a);
        }
    };
}

macro_rules! arg_post_processing {
    ($parser:ident, $arg:ident, $matcher:ident) => {
        debugln!("arg_post_processing!;");
        // Handle POSIX overrides
        debug!("arg_post_processing!: Is '{}' in overrides...", $arg.name);
        if $parser.overrides.contains(&$arg.name) {
            if let Some(ref name) = find_name_from!($parser.app, &$arg.name, overrides_with, $matcher) {
                sdebugln!("Yes by {}", name);
                $matcher.remove(name);
                remove_overriden!($parser, name);
            }
        } else { sdebugln!("No"); }

        // Add overrides
        debug!("arg_post_processing!: Does '{}' have overrides...", $arg.name);
        if let Some(ref or) = $arg.overrides_with {
            sdebugln!("Yes");
            $matcher.remove_all(&*or);
            for pa in or { remove_overriden!($parser, pa); }
            $parser.overrides.extend(or);
            vec_remove_all!($parser.required, or.iter());
        } else { sdebugln!("No"); }

        // Handle conflicts
        debug!("arg_post_processing!: Does '{}' have conflicts...", $arg.to_string());
        if let Some(ref bl) = $arg.conflicts_with {
            sdebugln!("Yes");

            for c in bl {
                // Inject two-way conflicts
                debug!("arg_post_processing!: Has '{}' already been matched...", c);
                if $matcher.contains(c) {
                    sdebugln!("Yes");
                    // find who conflictsed us...
                    $parser.conflicts.push(&$arg.name);
                } else {
                    sdebugln!("No");
                }
            }

            $parser.conflicts.extend_from_slice(&*bl);
            vec_remove_all!($parser.overrides, bl.iter());
            // vec_remove_all!($me.required, bl.iter());
        } else { sdebugln!("No"); }

        // Add all required args which aren't already found in matcher to the master
        // list
        debug!("arg_post_processing!: Does '{}' have requirements...", $arg.to_string());
        if let Some(ref reqs) = $arg.requires {
            sdebugln!("yes");
            for n in reqs.iter()
                .filter(|req| !$matcher.contains(&req))
                .map(|&name| name) {
                    
                $parser.required.push(n);
            }
        } else { sdebugln!("no"); }
        debug!("arg_post_processing!: Does '{}' have conditional requirements...", $arg.to_string());
        if let Some(ref reqs) = $arg.requires {
            sdebugln!("yes");
            for n in reqs.iter()
                .filter(|req| !$matcher.contains(&req))
                .map(|&name| name) {
                    
                $parser.required.push(n);
            }
        } else { sdebugln!("no"); }

        handle_group_reqs!($parser, $arg);
    };
}

macro_rules! handle_group_reqs {
    ($parser:ident, $arg:ident) => ({
        debugln!("handle_group_reqs!;");
        for grp in &$parser.app.groups {
            let found = if grp.args.contains(&$arg.name) {
                if let Some(ref reqs) = grp.requires {
                    debugln!("handle_group_reqs!: Adding {:?} to the required list", reqs);
                    $parser.required.extend(reqs);
                }
                if let Some(ref bl) = grp.conflicts {
                    $parser.conflicts.extend(bl);
                }
                true // What if arg is in more than one group with different reqs?
            } else {
                false
            };
            debugln!("handle_group_reqs!:iter: grp={}, found={:?}", grp.name, found);
            if found {
                for i in (0 .. $parser.required.len()).rev() {
                    let should_remove = grp.args.contains(&$parser.required[i]);
                    if should_remove { $parser.required.swap_remove(i); }
                }
                debugln!("handle_group_reqs!:iter: Adding args from group to conflicts...{:?}", grp.args);
                if !grp.multiple {
                    $parser.conflicts.extend(&grp.args);
                    debugln!("handle_group_reqs!: removing {:?} from conflicts", $arg.name);
                    for i in (0 .. $parser.conflicts.len()).rev() {
                        let should_remove = $parser.conflicts[i] == $arg.name;
                        if should_remove { $parser.conflicts.swap_remove(i); }
                    }
                }
            }
        }
    })
}

macro_rules! parse_positional {
    (
        $parser:ident, 
        $p:ident,
        $arg_os:ident,
        $pos_counter:ident,
        $matcher:ident
    ) => {
        debugln!("parse_positional!;");

        if !$parser.is_set(AS::TrailingValues) &&
           ($parser.is_set(AS::TrailingVarArg) &&
            $pos_counter == positionals!($parser.app).count()) {
            $parser.app._settings.set(AS::TrailingValues);
        }
        let _ = try!($parser.add_val_to_arg($p, &$arg_os, $matcher));

        $matcher.inc_occurrence_of($p.name);
        let _ = $parser.groups_for_arg($p.name)
                      .and_then(|vec| Some($matcher.inc_occurrences_of(&*vec)));
        if $parser.cache.map_or(true, |name| name != $p.name) {
            arg_post_processing!($parser, $p, $matcher);
            $parser.cache = Some($p.name);
        }

        $parser.app._settings.set(AS::ValidArgFound);
        // Only increment the positional counter if it doesn't allow multiples
        if !$p.is_set(ArgSettings::Multiple) {
            $pos_counter += 1;
        }
    };
}
