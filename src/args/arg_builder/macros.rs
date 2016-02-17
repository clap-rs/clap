macro_rules! write_arg_help {
    (@opt $_self:ident, $w:ident, $tab:ident, $longest:ident, $skip_pv:ident, $nlh:ident) => {
        write_arg_help!(@short $_self, $w, $tab);
        write_arg_help!(@opt_long $_self, $w, $nlh, $longest);
        write_arg_help!(@val $_self, $w);
        if !($nlh || $_self.settings.is_set(ArgSettings::NextLineHelp)) {
            write_spaces!(if $_self.long.is_some() { $longest + 4 } else { $longest + 8 } - ($_self.to_string().len()), $w);
        }
        if let Some(h) = $_self.help {
            write_arg_help!(@help $_self, $w, h, $tab, $longest, $nlh);
            write_arg_help!(@spec_vals $_self, $w, $skip_pv);
        }
    };
    (@flag $_self:ident, $w:ident, $tab:ident, $longest:ident, $nlh:ident) => {
        write_arg_help!(@short $_self, $w, $tab);
        write_arg_help!(@flag_long $_self, $w, $longest, $nlh);
        if let Some(h) = $_self.help {
            write_arg_help!(@help $_self, $w, h, $tab, $longest, $nlh);
        }
    };
    (@pos $_self:ident, $w:ident, $tab:ident, $longest:ident, $skip_pv:ident, $nlh:ident) => {
        try!(write!($w, "{}", $tab));
        write_arg_help!(@val $_self, $w);
        if !($nlh || $_self.settings.is_set(ArgSettings::NextLineHelp)) {
            write_spaces!($longest + 4 - ($_self.to_string().len()), $w);
        }
        if let Some(h) = $_self.help {
            write_arg_help!(@help $_self, $w, h, $tab, $longest, $nlh);
            write_arg_help!(@spec_vals $_self, $w, $skip_pv);
        }
    };
    (@short $_self:ident, $w:ident, $tab:ident) => {
        try!(write!($w, "{}", $tab));
        if let Some(s) = $_self.short {
            try!(write!($w, "-{}", s));
        } else {
            try!(write!($w, "{}", $tab));
        }
    };
    (@flag_long $_self:ident, $w:ident, $longest:ident, $nlh:ident) => {
        if let Some(l) = $_self.long {
            write_arg_help!(@long $_self, $w, l);
            if !$nlh || !$_self.settings.is_set(ArgSettings::NextLineHelp) {
                write_spaces!(($longest + 4) - (l.len() + 2), $w);
            }
        } else {
            if !$nlh || !$_self.settings.is_set(ArgSettings::NextLineHelp) {
                // 6 is tab (4) + -- (2)
                write_spaces!(($longest + 6), $w);
            }
        }
    };
    (@opt_long $_self:ident, $w:ident, $nlh:ident, $longest:ident) => {
        if let Some(l) = $_self.long {
            write_arg_help!(@long $_self, $w, l);
        }
        try!(write!($w, " "));
    };
    (@long $_self:ident, $w:ident, $l:ident) => {
        try!(write!($w,
                    "{}--{}",
                    if $_self.short.is_some() {
                        ", "
                    } else {
                        ""
                    },
                    $l));
    };
    (@val $_self:ident, $w:ident) => {
        if let Some(ref vec) = $_self.val_names {
            let mut it = vec.iter().peekable();
            while let Some((_, val)) = it.next() {
                try!(write!($w, "<{}>", val));
                if it.peek().is_some() { try!(write!($w, " ")); }
            }
            let num = vec.len();
            if $_self.settings.is_set(ArgSettings::Multiple) && num == 1 {
                try!(write!($w, "..."));
            }
        } else if let Some(num) = $_self.num_vals {
            for _ in 0..num {
                try!(write!($w, "<{}>", $_self.name));
            }
        } else {
            try!(write!($w,
                        "<{}>{}",
                        $_self.name,
                        if $_self.settings.is_set(ArgSettings::Multiple) {
                            "..."
                        } else {
                            ""
                        }));
        }
    };
    (@spec_vals $_self:ident, $w:ident, $skip_pv:ident) => {
        if let Some(ref pv) = $_self.default_val {
            try!(write!($w, " [default: {}]", pv));
        }
        if !$skip_pv {
            if let Some(ref pv) = $_self.possible_vals {
                try!(write!($w, " [values: {}]", pv.join(", ")));
            }
        }
    };
    (@help $_self:ident, $w:ident, $h:ident, $tab:ident, $longest:expr, $nlh:ident) => {
        if $nlh || $_self.settings.is_set(ArgSettings::NextLineHelp) {
            try!(write!($w, "\n{}{}", $tab, $tab));
        }
        if $h.contains("{n}") {
            if let Some(part) = $h.split("{n}").next() {
                try!(write!($w, "{}", part));
            }
            for part in $h.split("{n}").skip(1) {
                try!(write!($w, "\n"));
                if $nlh || $_self.settings.is_set(ArgSettings::NextLineHelp) {
                    try!(write!($w, "{}{}", $tab, $tab));
                } else {
                    write_spaces!($longest + 12, $w);
                }
                try!(write!($w, "{}", part));
            }
        } else {
            try!(write!($w, "{}", $h));
        }
    };
}
