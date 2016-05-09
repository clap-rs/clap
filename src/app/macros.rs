macro_rules! remove_overriden {
    (@remove $_self:ident, $v:ident, $a:ident.$ov:ident) => {
        if let Some(ref ora) = $a.$ov {
            vec_remove_all!($_self.$v, ora);
        }
    };
    (@arg $_self:ident, $arg:ident) => {
        remove_overriden!(@remove $_self, required, $arg.requires);
        remove_overriden!(@remove $_self, blacklist, $arg.blacklist);
        remove_overriden!(@remove $_self, overrides, $arg.overrides);
    };
    ($_self:ident, $name:expr) => {
        debugln!("macro=remove_overriden!;");
        if let Some(ref o) = $_self.opts.iter().filter(|o| o.name == *$name).next() {
            remove_overriden!(@arg $_self, o);
        } else if let Some(ref f) = $_self.flags.iter().filter(|f| f.name == *$name).next() {
            remove_overriden!(@arg $_self, f);
        } else if let Some(p) = $_self.positionals.values().filter(|p| p.name == *$name).next() {
            remove_overriden!(@arg $_self, p);
        }
    };
}

macro_rules! arg_post_processing {
    ($me:ident, $arg:ident, $matcher:ident) => {
        debugln!("macro=arg_post_processing!;");
        // Handle POSIX overrides
        debug!("Is '{}' in overrides...", $arg.to_string());
        if $me.overrides.contains(&$arg.name()) {
            if let Some(ref name) = $me.overriden_from($arg.name(), $matcher) {
                sdebugln!("Yes by {}", name);
                $matcher.remove(name);
                remove_overriden!($me, name);
            }
        } else { sdebugln!("No"); }

        // Add overrides
        debug!("Does '{}' have overrides...", $arg.to_string());
        if let Some(or) = $arg.overrides() {
            sdebugln!("Yes");
            $matcher.remove_all(or);
            for pa in or { remove_overriden!($me, pa); }
            $me.overrides.extend(or);
            vec_remove_all!($me.required, or);
        } else { sdebugln!("No"); }

        // Handle conflicts
        debug!("Does '{}' have conflicts...", $arg.to_string());
        if let Some(bl) = $arg.blacklist() {
            sdebugln!("Yes");
            $me.blacklist.extend(bl);
            vec_remove_all!($me.overrides, bl);
            vec_remove_all!($me.required, bl);
        } else { sdebugln!("No"); }

        // Add all required args which aren't already found in matcher to the master
        // list
        debug!("Does '{}' have requirements...", $arg.to_string());
        if let Some(reqs) = $arg.requires() {
            for n in reqs {
                if $matcher.contains(&n) {
                    sdebugln!("\tYes '{}' but it's already met", n);
                    continue;
                } else { sdebugln!("\tYes '{}'", n); }

                $me.required.push(n);
            }
        } else { sdebugln!("No"); }

        _handle_group_reqs!($me, $arg);
    };
}

macro_rules! _handle_group_reqs{
    ($me:ident, $arg:ident) => ({
        use args::AnyArg;
        debugln!("macro=_handle_group_reqs!;");
        for grp in $me.groups.values() {
            let mut found = false;
            if grp.args.contains(&$arg.name()) {
                vec_remove!($me.required, &$arg.name());
                if let Some(ref reqs) = grp.requires {
                    $me.required.extend(reqs);
                }
                if let Some(ref bl) = grp.conflicts {
                    $me.blacklist.extend(bl);
                }
                found = true; // What if arg is in more than one group with different reqs?
            }
            if found {
                vec_remove_all!($me.required, &grp.args);
                debugln!("Adding args from group to blacklist...{:?}", grp.args);
                $me.blacklist.extend(&grp.args);
                vec_remove!($me.blacklist, &$arg.name());
            }
        }
    })
}

macro_rules! validate_multiples {
    ($_self:ident, $a:ident, $m:ident) => {
        debugln!("macro=validate_multiples!;");
        if $m.contains(&$a.name) && !$a.settings.is_set(ArgSettings::Multiple) {
            // Not the first time, and we don't allow multiples
            return Err(Error::unexpected_multiple_usage($a, &*$_self.create_current_usage($m)))
        }
    };
}

macro_rules! parse_positional {
    (
        $_self:ident,
        $p:ident,
        $arg_os:ident,
        $pos_only:ident,
        $pos_counter:ident,
        $matcher:ident
    ) => {
        debugln!("macro=parse_positional!;");
        validate_multiples!($_self, $p, $matcher);

        if let Err(e) = $_self.add_val_to_arg($p, &$arg_os, $matcher) {
            return Err(e);
        }
        if !$pos_only &&
           ($_self.settings.is_set(AppSettings::TrailingVarArg) &&
            $pos_counter == $_self.positionals.len()) {
            $pos_only = true;
        }

        $matcher.inc_occurrence_of($p.name);
        let _ = $_self.groups_for_arg($p.name)
                      .and_then(|vec| Some($matcher.inc_occurrences_of(&*vec)));
        arg_post_processing!($_self, $p, $matcher);
// Only increment the positional counter if it doesn't allow multiples
        if !$p.settings.is_set(ArgSettings::Multiple) {
            $pos_counter += 1;
        }
    };
}
