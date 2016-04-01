
use std::io::{self, Write};
use std::collections::BTreeMap;
use std::fmt::Display;
use std::cmp;
use std::str;

use vec_map::VecMap;

use errors::{Error, Result as ClapResult};

use args::{AnyArg, ArgSettings, DispOrder};
use app::{App, AppSettings};
use app::parser::Parser;

use term;

const TAB: &'static str = "    ";

trait ArgWithDisplay<'b, 'c>: AnyArg<'b, 'c> + Display {}
impl<'b, 'c, T> ArgWithDisplay<'b, 'c> for T where T: AnyArg<'b, 'c> + Display {}

trait ArgWithOrder<'b, 'c>: ArgWithDisplay<'b, 'c> + DispOrder {
    fn as_base(&self) -> &ArgWithDisplay<'b, 'c>;
}
impl<'b, 'c, T> ArgWithOrder<'b, 'c> for T
    where T: ArgWithDisplay<'b, 'c> + DispOrder
{
    fn as_base(&self) -> &ArgWithDisplay<'b, 'c> {
        self
    }
}

impl<'b, 'c> DispOrder for App<'b, 'c> {
    fn disp_ord(&self) -> usize {
        999
    }
}

pub struct Help<'a> {
    writer: &'a mut Write,
    next_line_help: bool,
    hide_pv: bool,
    term_w: Option<usize>,
}

impl<'a> Help<'a> {
    pub fn new(w: &'a mut Write, next_line_help: bool, hide_pv: bool) -> Self {
        Help {
            writer: w,
            next_line_help: next_line_help,
            hide_pv: hide_pv,
            term_w: term::dimensions().map(|(w, _)| w),
        }
    }

    pub fn write_app_help(w: &'a mut Write, app: &App) -> ClapResult<()> {
        let ref parser = app.p;
        let nlh = parser.is_set(AppSettings::NextLineHelp);
        let hide_v = parser.is_set(AppSettings::HidePossibleValuesInHelp);
        Self::new(w, nlh, hide_v).write_help(&parser)
    }
}


fn as_arg_trait<'a, 'b, T: ArgWithOrder<'a, 'b>>(x: &T) -> &ArgWithOrder<'a, 'b> {
    x
}

// AnyArg
impl<'a> Help<'a> {
    fn write_args_unsorted<'b: 'd, 'c: 'd, 'd, I: 'd>(&mut self, args: I) -> io::Result<()>
        where I: Iterator<Item = &'d ArgWithOrder<'b, 'c>>
    {
        let mut longest = 0;
        let mut arg_v = Vec::with_capacity(10);
        for arg in args.filter(|arg| {
            !(arg.is_set(ArgSettings::Hidden)) || arg.is_set(ArgSettings::NextLineHelp)
        }) {
            if arg.longest_filter() {
                longest = cmp::max(longest, arg.to_string().len());
            }
            if !arg.is_set(ArgSettings::Hidden) {
                arg_v.push(arg)
            }
        }
        for arg in arg_v {
            try!(self.write_arg(arg.as_base(), longest));
        }
        Ok(())
    }

    fn write_args<'b: 'd, 'c: 'd, 'd, I: 'd>(&mut self, args: I) -> io::Result<()>
        where I: Iterator<Item = &'d ArgWithOrder<'b, 'c>>
    {
        let mut longest = 0;
        let mut ord_m = VecMap::new();
        for arg in args.filter(|arg| {
            !(arg.is_set(ArgSettings::Hidden)) || arg.is_set(ArgSettings::NextLineHelp)
        }) {
            if arg.longest_filter() {
                longest = cmp::max(longest, arg.to_string().len());
            }
            if !arg.is_set(ArgSettings::Hidden) {
                let btm = ord_m.entry(arg.disp_ord()).or_insert(BTreeMap::new());
                btm.insert(arg.name(), arg);
            }
        }
        for (_, btm) in ord_m.into_iter() {
            for (_, arg) in btm.into_iter() {
                try!(self.write_arg(arg.as_base(), longest));
            }
        }
        Ok(())
    }

    fn write_arg<'b, 'c>(&mut self,
                         arg: &ArgWithDisplay<'b, 'c>,
                         longest: usize)
                         -> io::Result<()> {
        debugln!("fn=write_to;");
        try!(self.short(arg));
        try!(self.long(arg, longest));
        try!(self.val(arg, longest));
        try!(self.help(arg, longest));
        try!(self.writer.write(b"\n"));
        Ok(())
    }

    fn short<'b, 'c>(&mut self, arg: &ArgWithDisplay<'b, 'c>) -> io::Result<()> {
        debugln!("fn=short;");
        try!(write!(self.writer, "{}", TAB));
        if let Some(s) = arg.short() {
            write!(self.writer, "-{}", s)
        } else if arg.has_switch() {
            write!(self.writer, "{}", TAB)
        } else {
            Ok(())
        }
    }

    fn long<'b, 'c>(&mut self, arg: &ArgWithDisplay<'b, 'c>, longest: usize) -> io::Result<()> {
        debugln!("fn=long;");
        if !arg.has_switch() {
            return Ok(());
        }
        if arg.takes_value() {
            if let Some(l) = arg.long() {
                try!(write!(self.writer,
                            "{}--{}",
                            if arg.short().is_some() {
                                ", "
                            } else {
                                ""
                            },
                            l));
            }
            try!(write!(self.writer, " "));
        } else {
            // write_spaces! fails when using self.writer
            let ref mut w = self.writer;
            if let Some(l) = arg.long() {
                try!(write!(w,
                            "{}--{}",
                            if arg.short().is_some() {
                                ", "
                            } else {
                                ""
                            },
                            l));
                if !self.next_line_help || !arg.is_set(ArgSettings::NextLineHelp) {
                    write_spaces!((longest + 4) - (l.len() + 2), w);
                }
            } else {
                if !self.next_line_help || !arg.is_set(ArgSettings::NextLineHelp) {
                    // 6 is tab (4) + -- (2)
                    write_spaces!((longest + 6), w);
                }
            }
        }
        Ok(())
    }

    fn val<'b, 'c>(&mut self, arg: &ArgWithDisplay<'b, 'c>, longest: usize) -> io::Result<()> {
        debugln!("fn=val;");
        if !arg.takes_value() {
            return Ok(());
        }
        if let Some(ref vec) = arg.val_names() {
            let mut it = vec.iter().peekable();
            while let Some((_, val)) = it.next() {
                try!(write!(self.writer, "<{}>", val));
                if it.peek().is_some() {
                    try!(write!(self.writer, " "));
                }
            }
            let num = vec.len();
            if arg.is_set(ArgSettings::Multiple) && num == 1 {
                try!(write!(self.writer, "..."));
            }
        } else if let Some(num) = arg.num_vals() {
            let mut it = (0..num).peekable();
            while let Some(_) = it.next() {
                try!(write!(self.writer, "<{}>", arg.name()));
                if it.peek().is_some() {
                    try!(write!(self.writer, " "));
                }
            }
        } else {
            try!(write!(self.writer, "{}", arg));
        }
        let ref mut w = self.writer;
        if arg.has_switch() {
            if !(self.next_line_help || arg.is_set(ArgSettings::NextLineHelp)) {
                let self_len = arg.to_string().len();
                // subtract ourself
                let mut spcs = longest - self_len;
                // Since we're writing spaces from the tab point we first need to know if we
                // had a long and short, or just short
                if arg.long().is_some() {
                    // Only account 4 after the val
                    spcs += 4;
                } else {
                    // Only account for ', --' + 4 after the val
                    spcs += 8;
                }

                write_spaces!(spcs, w);
            }
        } else if !(self.next_line_help || arg.is_set(ArgSettings::NextLineHelp)) {
            write_spaces!(longest + 4 - (arg.to_string().len()), w);
        }
        Ok(())
    }

    fn help<'b, 'c>(&mut self, arg: &ArgWithDisplay<'b, 'c>, longest: usize) -> io::Result<()> {
        debugln!("fn=help;");
        let spec_vals = self.spec_vals(arg);
        let mut help = String::new();
        let h = arg.help().unwrap_or("");
        let spcs = if self.next_line_help || arg.is_set(ArgSettings::NextLineHelp) {
            8 // "tab" + "tab"
        } else {
            longest + 12
        };
        // determine if our help fits or needs to wrap
        let width = self.term_w.unwrap_or(0);
        debugln!("Term width...{}", width);
        let too_long = self.term_w.is_some() && (spcs + h.len() + spec_vals.len() >= width);
        debugln!("Too long...{:?}", too_long);

        // Is help on next line, if so newline + 2x tab
        if self.next_line_help || arg.is_set(ArgSettings::NextLineHelp) {
            try!(write!(self.writer, "\n{}{}", TAB, TAB));
        }

        debug!("Too long...");
        if too_long {
            sdebugln!("Yes");
            help.push_str(h);
            help.push_str(&*spec_vals);
            debugln!("help: {}", help);
            debugln!("help len: {}", help.len());
            // Determine how many newlines we need to insert
            let avail_chars = width - spcs;
            debugln!("Usable space: {}", avail_chars);
            let longest_w = {
                let mut lw = 0;
                for l in help.split(' ').map(|s| s.len()) {
                    if l > lw {
                        lw = l;
                    }
                }
                lw
            };
            debugln!("Longest word...{}", longest_w);
            debug!("Enough space...");
            if longest_w < avail_chars {
                sdebugln!("Yes");
                let mut indices = vec![];
                let mut idx = 0;
                loop {
                    idx += avail_chars - 1;
                    if idx >= help.len() {
                        break;
                    }
                    // 'a' arbitrary non space char
                    if help.chars().nth(idx).unwrap_or('a') != ' ' {
                        idx = find_idx_of_space(&*help, idx);
                    }
                    debugln!("Adding idx: {}", idx);
                    debugln!("At {}: {:?}", idx, help.chars().nth(idx));
                    indices.push(idx);
                    if &help[idx..].len() <= &avail_chars {
                        break;
                    }
                }
                for (i, idx) in indices.iter().enumerate() {
                    debugln!("iter;i={},idx={}", i, idx);
                    let j = idx + (2 * i);
                    debugln!("removing: {}", j);
                    debugln!("at {}: {:?}", j, help.chars().nth(j));
                    help.remove(j);
                    help.insert(j, '{');
                    help.insert(j + 1, 'n');
                    help.insert(j + 2, '}');
                }
            } else {
                sdebugln!("No");
            }
        } else {
            sdebugln!("No");
        }
        let help = if !help.is_empty() {
            &*help
        } else if !spec_vals.is_empty() {
            help.push_str(h);
            help.push_str(&*spec_vals);
            &*help
        } else {
            h
        };
        if help.contains("{n}") {
            if let Some(part) = help.split("{n}").next() {
                try!(write!(self.writer, "{}", part));
            }
            let ref mut w = self.writer;
            for part in help.split("{n}").skip(1) {
                try!(write!(w, "\n"));
                if self.next_line_help || arg.is_set(ArgSettings::NextLineHelp) {
                    try!(write!(w, "{}{}", TAB, TAB));
                } else if arg.has_switch() {
                    write_spaces!(longest + 12, w);
                } else {
                    write_spaces!(longest + 8, w);
                }
                try!(write!(w, "{}", part));
            }
        } else {
            try!(write!(self.writer, "{}", help));
        }
        Ok(())
    }

    fn spec_vals(&self, a: &ArgWithDisplay) -> String {
        debugln!("fn=spec_vals;");
        if let Some(ref pv) = a.default_val() {
            debugln!("Writing defaults");
            return format!(" [default: {}] {}",
                           pv,
                           if !self.hide_pv {
                               if let Some(ref pv) = a.possible_vals() {
                                   format!(" [values: {}]", pv.join(", "))
                               } else {
                                   "".into()
                               }
                           } else {
                               "".into()
                           });
        } else if !self.hide_pv {
            debugln!("Writing values");
            if let Some(ref pv) = a.possible_vals() {
                debugln!("Possible vals...{:?}", pv);
                return format!(" [values: {}]", pv.join(", "));
            }
        }
        String::new()
    }
}


// Parser
impl<'a> Help<'a> {
    pub fn write_all_args(&mut self, parser: &Parser) -> ClapResult<()> {

        let flags = !parser.has_flags();
        let pos = !parser.has_positionals();
        let opts = !parser.has_opts();
        let subcmds = !parser.has_subcommands();

        let unified_help = parser.is_set(AppSettings::UnifiedHelpMessage);

        if unified_help && (flags || opts) {
            let opts_flags = parser.iter_flags()
                                   .map(as_arg_trait)
                                   .chain(parser.iter_opts().map(as_arg_trait));
            try!(write!(self.writer, "\nOPTIONS:\n"));
            try!(self.write_args(opts_flags));
        } else {
            if flags {
                try!(write!(self.writer, "\nFLAGS:\n"));
                try!(self.write_args(parser.iter_flags()
                                           .map(as_arg_trait)));
            }
            if opts {
                try!(write!(self.writer, "\nOPTIONS:\n"));
                try!(self.write_args(parser.iter_opts().map(as_arg_trait)));
            }
        }

        if pos {
            try!(write!(self.writer, "\nARGS:\n"));
            try!(self.write_args_unsorted(parser.iter_positionals().map(as_arg_trait)));
        }

        if subcmds {
            try!(write!(self.writer, "\nSUBCOMMANDS:\n"));

            let mut longest = 0;

            let mut ord_m = VecMap::new();
            for sc in parser.subcommands.iter().filter(|s| !s.p.is_set(AppSettings::Hidden)) {
                let btm = ord_m.entry(sc.p.meta.disp_ord).or_insert(BTreeMap::new());
                btm.insert(sc.p.meta.name.clone(), sc);
                longest = cmp::max(longest, sc.p.meta.name.len());
            }

            for (_, btm) in ord_m.into_iter() {
                for (_, sc) in btm.into_iter() {
                    try!(self.write_arg(sc, longest));
                }
            }
        }
        //


        Ok(())
    }

    fn write_version(&mut self, parser: &Parser) -> io::Result<()> {
        try!(write!(self.writer, "{}", parser.meta.version.unwrap_or("".into())));
        Ok(())
    }

    fn write_bin_name(&mut self, parser: &Parser) -> io::Result<()> {
        if let Some(bn) = parser.meta.bin_name.as_ref() {
            if bn.contains(' ') {
                // Incase we're dealing with subcommands i.e. git mv is translated to git-mv
                try!(write!(self.writer, "{}", bn.replace(" ", "-")))
            } else {
                try!(write!(self.writer, "{}", &parser.meta.name[..]))
            }
        } else {
            try!(write!(self.writer, "{}", &parser.meta.name[..]))
        }
        Ok(())
    }

    pub fn write_help(&mut self, parser: &Parser) -> ClapResult<()> {
        if let Some(h) = parser.meta.help_str {
            try!(writeln!(self.writer, "{}", h).map_err(Error::from));
            Ok(())
        } else {
            self.write_default_help(&parser)
        }
    }

    #[cfg_attr(feature = "lints", allow(for_kv_map))]
    pub fn write_default_help(&mut self, parser: &Parser) -> ClapResult<()> {

        // Print the version
        try!(self.write_bin_name(&parser));
        try!(self.writer.write(b" "));
        try!(self.write_version(&parser));
        try!(self.writer.write(b"\n"));
        if let Some(author) = parser.meta.author {
            try!(write!(self.writer, "{}\n", author));
        }
        if let Some(about) = parser.meta.about {
            try!(write!(self.writer, "{}\n", about));
        }

        try!(write!(self.writer, "\n{}", parser.create_usage(&[])));

        let flags = !parser.has_flags();
        let pos = !parser.has_positionals();
        let opts = !parser.has_opts();
        let subcmds = !parser.has_subcommands();

        if flags || opts || pos || subcmds {
            try!(write!(self.writer, "\n"));
            try!(self.write_all_args(&parser));
        }

        if let Some(h) = parser.meta.more_help {
            try!(write!(self.writer, "\n{}", h));
        }

        self.writer.flush().map_err(Error::from)
    }
}


fn find_idx_of_space(full: &str, start: usize) -> usize {
    debugln!("fn=find_idx_of_space;");
    let haystack = &full[..start];
    debugln!("haystack: {}", haystack);
    for (i, c) in haystack.chars().rev().enumerate() {
        debugln!("iter;c={},i={}", c, i);
        if c == ' ' {
            debugln!("Found space returning start-i...{}", start - (i + 1));
            return start - (i + 1);
        }
    }
    0
}
