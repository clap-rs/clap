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

macro_rules! arg_post_processing(
    ($me:ident, $arg:ident, $matcher:ident) => ({
        use args::AnyArg;
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
            for pa in or {
                sdebugln!("\tYes '{}'", pa);
                $matcher.remove(&*pa);
                remove_overriden!($me, pa);
                $me.overrides.push(pa);
                vec_remove!($me.required, pa);
            }
        } else { sdebugln!("No"); }

        // Handle conflicts
        debug!("Does '{}' have conflicts...", $arg.to_string());
        if let Some(bl) = $arg.blacklist() {
            for name in bl {
                sdebugln!("\n\tYes '{}'", name);
                $me.blacklist.push(name);
                vec_remove!($me.overrides, name);
                vec_remove!($me.required, name);
            }
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
    })
);

macro_rules! _handle_group_reqs{
    ($me:ident, $arg:ident) => ({
        use args::AnyArg;
        debugln!("macro=_handle_group_reqs!;");
        for grp in $me.groups.values() {
            let mut found = false;
            for name in &grp.args {
                if name == &$arg.name() {
                    vec_remove!($me.required, name);
                    if let Some(ref reqs) = grp.requires {
                        for r in reqs {
                            $me.required.push(r);
                        }
                    }
                    if let Some(ref bl) = grp.conflicts {
                        for &b in bl {
                            $me.blacklist.push(b);
                        }
                    }
                    found = true;
                    break;
                }
            }
            if found {
                for name in &grp.args {
                    if name == &$arg.name() { continue }
                    vec_remove!($me.required, name);

                    $me.blacklist.push(name);
                }
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
