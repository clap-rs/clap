// Std
use std::cmp;
use std::collections::BTreeMap;
use std::io::{self, Cursor, Read, Write};
use std::usize;

// Internal
use {Arg, App, AppSettings, ArgSettings};
use parsing::{DispOrder, Parser};
use errors::{Error as ClapError, Result as ClapResult};
use output::fmt::{Format, Colorizer, ColorizerOption};

// Third Party
use unicode_width::UnicodeWidthStr;
#[cfg(feature = "wrap_help")]
use term_size;
#[cfg(feature = "wrap_help")]
use textwrap;
use vec_map::VecMap;

#[cfg(not(feature = "wrap_help"))]
mod term_size {
    pub fn dimensions() -> Option<(usize, usize)> { None }
}

fn str_width(s: &str) -> usize { UnicodeWidthStr::width(s) }

const TAB: &'static str = "    ";

impl<'b, 'c> DispOrder for App<'b, 'c> {
    fn disp_ord(&self) -> usize { 999 }
}

macro_rules! color {
    ($_self:ident, $w:expr, $s:expr, $c:ident) => {
        if $_self.color {
            write!($w, "{}", $_self.cizer.$c($s))
        } else {
            write!($w, "{}", $s)
        }
    };
    ($_self:ident, $w:expr, $fmt_s:expr, $v:expr, $c:ident) => {
        if $_self.color {
            write!($w, "{}", $_self.cizer.$c(format!($fmt_s, $v)))
        } else {
            write!($w, $fmt_s, $v)
        }
    };
}

/// `clap` Help Writer.
///
/// Wraps a writer stream providing different methods to generate help for `clap` objects.
pub struct HelpWriter<'a, 'b, 'c, 'd> where 'a: 'b, 'b: 'c, 'c: 'd {
    parser: &'d Parser<'a, 'b, 'c>,
    next_line_help: bool,
    hide_pv: bool,
    term_width: usize,
    color: bool,
    cizer: Colorizer,
    longest: usize,
    force_next_line: bool,
    use_long: bool,
}

// Public Functions
impl<'a, 'b, 'c, 'd> HelpWriter<'a, 'b, 'c, 'd> {
    /// Create a new `Help` instance.
    pub fn new(p: &'d Parser<'a, 'b, 'c>, use_stderr: bool) -> Self {
        debugln!("HelpWriter::new;");
        // @DESIGN @TODO-v3-beta: shouldn't use_stderr be determined by the Write object passed in 
        // later??
        let nlh = p.is_set(AppSettings::NextLineHelp);
        let hide_v = p.is_set(AppSettings::HidePossibleValuesInHelp);
        let color = p.is_set(AppSettings::ColoredHelp);
        let cizer = Colorizer::new(ColorizerOption {
            use_stderr: use_stderr,
            when: p.color(),
        });
        HelpWriter {
            parser: p,
            next_line_help: nlh,
            hide_pv: hide_v,
            term_width: match p.app.term_width {
                Some(width) => if width == 0 { usize::MAX } else { width },
                None => {
                    cmp::min(
                        term_size::dimensions().map_or(120, |(w, _)| w),
                        match p.app.max_term_width {
                            None | Some(0) => usize::MAX,
                            Some(mw) => mw,
                        },
                    )
                }
            },
            color: color,
            cizer: cizer,
            longest: 0,
            force_next_line: false,
            use_long: false,
        }
    }

    /// Reads help settings from an App
    /// and write its help to the wrapped stream.
    pub fn write_help<W: Write>(&mut self, w: &mut W) -> ClapResult<()> {
        self._write_help(w, false)
    }

    /// Reads help settings from an App
    /// and write its help to the wrapped stream.
    pub fn write_long_help<W: Write>(&mut self, w: &mut W) -> ClapResult<()> {
        self._write_help(w, true)
    }

    #[doc(hidden)]
    pub fn _write_help<W: Write>(&mut self, w: &mut W, use_long: bool) -> ClapResult<()> {
        debugln!("HelpWriter::write_app_help;");
        // @TODO-v3-alpha: Derive Display Order
        self.use_long = use_long;

        debugln!("HelpWriter::write_help;");
        if let Some(h) = self.parser.app.override_help {
            try!(write!(w, "{}", h).map_err(ClapError::from));
        } else if let Some(tmpl) = self.parser.app.help_template {
            try!(self.write_templated_help(w, tmpl));
        } else {
            try!(self.write_default_help(w));
        }
        Ok(())
    }

    /// Writes the version to the wrapped stream
    pub fn write_version<W: Write>(&mut self, w: &mut W) -> ClapResult<()> {
        try!(self._write_version(w, false));
        Ok(())
    }

    /// Writes the long version to the wrapped stream
    pub fn write_long_version<W: Write>(&mut self, w: &mut W) -> ClapResult<()> {
        try!(self._write_version(w, true));
        Ok(())
    }

    #[doc(hidden)]
    fn _write_version<W: Write>(&mut self, w: &mut W, use_long: bool) -> io::Result<()> {
        let ver = if use_long {
            self.parser
                .app
                .long_version
                .unwrap_or_else(|| self.parser.app.version.unwrap_or(""))
        } else {
            self.parser
                .app
                .version
                .unwrap_or_else(|| self.parser.app.long_version.unwrap_or(""))
        };
        if let Some(bn) = self.parser.app.bin_name.as_ref() {
            if bn.contains(' ') {
                // Incase we're dealing with subcommands i.e. git mv is translated to git-mv
                write!(w, "{} {}", bn.replace(" ", "-"), ver)
            } else {
                write!(w, "{} {}", &self.parser.app.name[..], ver)
            }
        } else {
            write!(w, "{} {}", &self.parser.app.name[..], ver)
        }
    }
}

impl<'a, 'b, 'c, 'd> HelpWriter<'a, 'b, 'c, 'd> {
    /// Writes help for each argument in the order they were declared to the wrapped stream.
    fn write_args_unsorted<'z, W: Write, I: 'z>(&mut self, w: &mut W, args: I) -> io::Result<()>
    where
        I: Iterator<Item = &'z Arg<'a, 'b>>,
        'b: 'z,
    {
        debugln!("HelpWriter::write_args_unsorted;");
        // The shortest an arg can legally be is 2 (i.e. '-x')
        self.longest = 2;
        let mut arg_v = Vec::with_capacity(10);
        for arg in args.filter(|arg| {
            !(arg.is_set(ArgSettings::Hidden)) || arg.is_set(ArgSettings::NextLineHelp)
        })
        {
            if arg._longest_filter() {
                self.longest = cmp::max(self.longest, str_width(arg.to_string().as_str()));
            }
            arg_v.push(arg)
        }
        let mut first = true;
        for arg in arg_v {
            if first {
                first = false;
            } else {
                try!(w.write_all(b"\n"));
            }
            try!(self.write_arg(w, arg));
        }
        Ok(())
    }

    /// Sorts arguments by length and display order and write their help to the wrapped stream.
    fn write_args<'z, W: Write, I: 'z>(&mut self, w: &mut W, args: I) -> io::Result<()>
    where
        I: Iterator<Item = &'z Arg<'a, 'b>>,
        'b: 'z
    {
        debugln!("HelpWriter::write_args;");
        // The shortest an arg can legally be is 2 (i.e. '-x')
        self.longest = 2;
        let mut ord_m = VecMap::new();
        // Determine the longest
        for arg in args.filter(|arg| {
            // If it's NextLineHelp, but we don't care to compute how long because it may be
            // NextLineHelp on purpose *because* it's so long and would throw off all other
            // args alignment
            !arg.is_set(ArgSettings::Hidden) || arg.is_set(ArgSettings::NextLineHelp)
        })
        {
            if arg._longest_filter() {
                debugln!("HelpWriter::write_args: Current Longest...{}", self.longest);
                self.longest = cmp::max(self.longest, str_width(arg.to_string().as_str()));
                debugln!("HelpWriter::write_args: New Longest...{}", self.longest);
            }
            let btm = ord_m.entry(arg.disp_ord()).or_insert(BTreeMap::new());
            btm.insert(arg.name, arg);
        }
        let mut first = true;
        for btm in ord_m.values() {
            for arg in btm.values() {
                if first {
                    first = false;
                } else {
                    try!(w.write_all(b"\n"));
                }
                try!(self.write_arg(w, arg));
            }
        }
        Ok(())
    }

    /// Writes help for an subcommand to the wrapped stream.
    fn write_subcommand_as_arg<W: Write>(&mut self, w: &mut W, sc: &App<'a, 'b>) -> io::Result<()> {
        debugln!("HelpWriter::write_subcommand_as_arg;");
        let spec_vals = self.subcommand_spec_vals(sc);
        try!(self.subcommand_help(w, sc, &*spec_vals));
        Ok(())
    }

    /// Writes help for an argument to the wrapped stream.
    fn write_arg<W: Write>(&mut self, w: &mut W, arg: &Arg<'a, 'b>) -> io::Result<()> {
        debugln!("HelpWriter::write_arg;");
        try!(self.short(w, arg));
        try!(self.long(w, arg));
        let spec_vals = try!(self.val(w, arg));
        try!(self.help(w, arg, &*spec_vals));
        Ok(())
    }

    /// Writes argument's short command to the wrapped stream.
    fn short<W: Write>(&mut self, w: &mut W, arg: &Arg<'a, 'b>) -> io::Result<()> {
        debugln!("HelpWriter::short;");
        try!(write!(w, "{}", TAB));
        if let Some(s) = arg.short {
            color!(self, w, "-{}", s, good)
        } else if arg._has_switch() {
            write!(w, "{}", TAB)
        } else {
            Ok(())
        }
    }

    /// Writes argument's long command to the wrapped stream.
    fn long<W: Write>(&mut self, w: &mut W, arg: &Arg<'a, 'b>) -> io::Result<()> {
        debugln!("HelpWriter::long;");
        if !arg._has_switch() {
            return Ok(());
        }
        if arg.is_set(ArgSettings::TakesValue) {
            if let Some(l) = arg.long {
                if arg.short.is_some() {
                    try!(write!(w, ", "));
                }
                try!(color!(self, w, "--{}", l, good))
            }

            let sep = if arg.is_set(ArgSettings::RequireEquals) {
                "="
            } else {
                " "
            };
            try!(write!(w, "{}", sep));
        } else if let Some(l) = arg.long {
            if arg.short.is_some() {
                try!(write!(w, ", "));
            }
            try!(color!(self, w, "--{}", l, good));
        }
        Ok(())
    }

    /// Writes argument's possible values to the wrapped stream.
    fn val<W: Write>(&mut self, w: &mut W, arg: &Arg<'a, 'b>) -> Result<String, io::Error> {
        debugln!("HelpWriter::val: arg={}", arg);
        if arg.is_set(ArgSettings::TakesValue) {
            if let Some(ref vec) = arg.value_names {
                let mut it = vec.iter().peekable();
                while let Some((_, val)) = it.next() {
                    try!(color!(self, w, "<{}>", val, good));
                    if it.peek().is_some() {
                        try!(write!(w, " "));
                    }
                }
                let num = vec.len();
                if arg.is_set(ArgSettings::Multiple) && num == 1 {
                    try!(color!(self, w, "...", good));
                }
            } else if let Some(num) = arg.number_of_values {
                let mut it = (0..num).peekable();
                while let Some(_) = it.next() {
                    try!(color!(self, w, "<{}>", arg.name, good));
                    if it.peek().is_some() {
                        try!(write!(w, " "));
                    }
                }
                if arg.is_set(ArgSettings::Multiple) && num == 1 {
                    try!(color!(self, w, "...", good));
                }
            } else if arg._has_switch() {
                try!(color!(self, w, "<{}>", arg.name, good));
                if arg.is_set(ArgSettings::Multiple) {
                    try!(color!(self, w, "...", good));
                }
            } else {
                try!(color!(self, w, "{}", arg, good));
            }
        }

        let spec_vals = self.spec_vals(arg);
        let h = arg.help.unwrap_or("");
        let h_w = str_width(h) + str_width(&*spec_vals);
        let nlh = self.next_line_help || arg.is_set(ArgSettings::NextLineHelp);
        let taken = self.longest + 12;
        self.force_next_line = !nlh && self.term_width >= taken &&
            (taken as f32 / self.term_width as f32) > 0.40 &&
            h_w > (self.term_width - taken);

        debug!("HelpWriter::val: Has switch...");
        if arg._has_switch() {
            sdebugln!("Yes");
            debugln!("HelpWriter::val: force_next_line...{:?}", self.force_next_line);
            debugln!("HelpWriter::val: nlh...{:?}", nlh);
            debugln!("HelpWriter::val: taken...{}", taken);
            debugln!(
                "HelpWriter::val: help_width > (width - taken)...{} > ({} - {})",
                h_w,
                self.term_width,
                taken
            );
            debugln!("HelpWriter::val: longest...{}", self.longest);
            debug!("HelpWriter::val: next_line...");
            if !(nlh || self.force_next_line) {
                sdebugln!("No");
                let self_len = str_width(arg.to_string().as_str());
                // subtract ourself
                let mut spcs = self.longest - self_len;
                // Since we're writing spaces from the tab point we first need to know if we
                // had a long and short, or just short
                if arg.long.is_some() {
                    // Only account 4 after the val
                    spcs += 4;
                } else {
                    // Only account for ', --' + 4 after the val
                    spcs += 8;
                }

                write_nspaces!(w, spcs);
            } else {
                sdebugln!("Yes");
            }
        } else if !(nlh || self.force_next_line) {
            sdebugln!("No, and not next_line");
            write_nspaces!(
                w,
                self.longest + 4 - (str_width(arg.to_string().as_str()))
            );
        } else {
            sdebugln!("No");
        }
        Ok(spec_vals)
    }

    fn write_before_after_help<W: Write>(&mut self, w: &mut W, h: &str) -> io::Result<()> {
        debugln!("HelpWriter::write_before_after_help;");
        let mut help = String::from(h);
        // determine if our help fits or needs to wrap
        debugln!(
            "HelpWriter::write_before_after_help: Term width...{}",
            self.term_width
        );
        let too_long = str_width(h) >= self.term_width;

        debug!("HelpWriter::write_before_after_help: Too long...");
        if too_long || h.contains("{n}") {
            sdebugln!("Yes");
            debugln!("HelpWriter::write_before_after_help: help: {}", help);
            debugln!(
                "HelpWriter::write_before_after_help: help width: {}",
                str_width(&*help)
            );
            // Determine how many newlines we need to insert
            debugln!(
                "HelpWriter::write_before_after_help: Usable space: {}",
                self.term_width
            );
            help = wrap_help(&help.replace("{n}", "\n"), self.term_width);
        } else {
            sdebugln!("No");
        }
        try!(write!(w, "{}", help));
        Ok(())
    }

    /// Writes argument's help to the wrapped stream.
    fn help<W: Write>(&mut self, w: &mut W, arg: &Arg<'a, 'b>, spec_vals: &str) -> io::Result<()> {
        debugln!("HelpWriter::help;");
        let h = if self.use_long {
            arg.long_help.unwrap_or_else(|| arg.help.unwrap_or(""))
        } else {
            arg.help.unwrap_or_else(|| arg.long_help.unwrap_or(""))
        };
        let mut help = String::from(h) + spec_vals;
        let nlh = self.next_line_help || arg.is_set(ArgSettings::NextLineHelp) || self.use_long;
        debugln!("HelpWriter::help: Next Line...{:?}", nlh);

        let spcs = if nlh || self.force_next_line {
            12 // "tab" * 3
        } else {
            self.longest + 12
        };

        let too_long = spcs + str_width(h) + str_width(&*spec_vals) >= self.term_width;

        // Is help on next line, if so then indent
        if nlh || self.force_next_line {
            try!(write!(w, "\n{}{}{}", TAB, TAB, TAB));
        }

        debug!("HelpWriter::help: Too long...");
        if too_long && spcs <= self.term_width || h.contains("{n}") {
            sdebugln!("Yes");
            debugln!("HelpWriter::help: help...{}", help);
            debugln!("HelpWriter::help: help width...{}", str_width(&*help));
            // Determine how many newlines we need to insert
            let avail_chars = self.term_width - spcs;
            debugln!("HelpWriter::help: Usable space...{}", avail_chars);
            help = wrap_help(&help.replace("{n}", "\n"), avail_chars);
        } else {
            sdebugln!("No");
        }
        if let Some(part) = help.lines().next() {
            try!(write!(w, "{}", part));
        }
        for part in help.lines().skip(1) {
            try!(write!(w, "\n"));
            if nlh || self.force_next_line {
                try!(write!(w, "{}{}{}", TAB, TAB, TAB));
            } else if arg._has_switch() {
                write_nspaces!(w, self.longest + 12);
            } else {
                write_nspaces!(w, self.longest + 8);
            }
            try!(write!(w, "{}", part));
        }
        if !help.contains('\n') && (nlh || self.force_next_line) {
            try!(write!(w, "\n"));
        }
        Ok(())
    }

    /// Writes argument's help to the wrapped stream.
    fn subcommand_help<W: Write>(&mut self, w: &mut W, arg: &App<'a, 'b>, spec_vals: &str) -> io::Result<()> {
        debugln!("HelpWriter::subcommand_help;");
        let h = if self.use_long {
            arg.long_about.unwrap_or_else(|| arg.about.unwrap_or(""))
        } else {
            arg.about.unwrap_or_else(|| arg.long_about.unwrap_or(""))
        };
        let mut help = String::from(h) + spec_vals;
        let nlh = self.next_line_help || self.force_next_line || self.use_long;
        debugln!("HelpWriter::subcommand_help: Next Line...{:?}", nlh);

        let spcs = if nlh || self.force_next_line {
            12 // "tab" * 3
        } else {
            self.longest + 12
        };

        let too_long = spcs + str_width(h) + str_width(&*spec_vals) >= self.term_width;

        // Is help on next line, if so then indent
        if nlh || self.force_next_line {
            try!(write!(w, "\n{}{}{}", TAB, TAB, TAB));
        }

        debug!("HelpWriter::subcommand_help: Too long...");
        if too_long && spcs <= self.term_width || h.contains("{n}") {
            sdebugln!("Yes");
            debugln!("HelpWriter::subcommand_help: help...{}", help);
            debugln!("HelpWriter::subcommand_help: help width...{}", str_width(&*help));
            // Determine how many newlines we need to insert
            let avail_chars = self.term_width - spcs;
            debugln!("HelpWriter::subcommand_help: Usable space...{}", avail_chars);
            help = wrap_help(&help.replace("{n}", "\n"), avail_chars);
        } else {
            sdebugln!("No");
        }
        if let Some(part) = help.lines().next() {
            try!(write!(w, "{}", part));
        }
        for part in help.lines().skip(1) {
            try!(write!(w, "\n"));
            if nlh || self.force_next_line {
                try!(write!(w, "{}{}{}", TAB, TAB, TAB));
            } else {
                write_nspaces!(w, self.longest + 8);
            }
            try!(write!(w, "{}", part));
        }
        if !help.contains('\n') && (nlh || self.force_next_line) {
            try!(write!(w, "\n"));
        }
        Ok(())
    }

    fn spec_vals(&self, a: &Arg) -> String {
        debugln!("HelpWriter::spec_vals: a={}", a);
        let mut spec_vals = vec![];
        if !a.is_set(ArgSettings::HideDefaultValue) {
            if let Some(pv) = a.default_value {
                debugln!("HelpWriter::spec_vals: Found default value...[{:?}]", pv);
                spec_vals.push(format!(
                    " [default: {}]",
                    if self.color {
                        self.cizer.good(pv.to_string_lossy())
                    } else {
                        Format::None(pv.to_string_lossy())
                    }
                ));
            }
        }
        // @TODO-v3-alpha: consider visible aliases
        if let Some(ref aliases) = a.aliases {
            debugln!("HelpWriter::spec_vals: Found aliases...{:?}", aliases);
            spec_vals.push(format!(
                " [aliases: {}]",
                if self.color {
                    aliases
                        .iter()
                        .map(|v| format!("{}", self.cizer.good(v)))
                        .collect::<Vec<_>>()
                        .join(", ")
                } else {
                    aliases.join(", ")
                }
            ));
        }
        if !self.hide_pv && !a.is_set(ArgSettings::HidePossibleValues) {
            if let Some(ref pv) = a.possible_values {
                debugln!("HelpWriter::spec_vals: Found possible vals...{:?}", pv);
                spec_vals.push(if self.color {
                    format!(
                        " [values: {}]",
                        pv.iter()
                            .map(|v| format!("{}", self.cizer.good(v)))
                            .collect::<Vec<_>>()
                            .join(", ")
                    )
                } else {
                    format!(" [values: {}]", pv.join(", "))
                });
            }
        }
        spec_vals.join(" ")
    }

    fn subcommand_spec_vals(&self, a: &App) -> String {
        debugln!("HelpWriter::spec_vals: a={}", a);
        let mut spec_vals = vec![];
        // @TODO-v3-alpha: consider visible aliases
        if let Some(ref aliases) = a.aliases {
            debugln!("HelpWriter::spec_vals: Found aliases...{:?}", aliases);
            spec_vals.push(format!(
                " [aliases: {}]",
                if self.color {
                    aliases
                        .iter()
                        .map(|v| format!("{}", self.cizer.good(v)))
                        .collect::<Vec<_>>()
                        .join(", ")
                } else {
                    aliases.join(", ")
                }
            ));
        }
        spec_vals.join(" ")
    }
}


// Methods to write Parser help.
impl<'a, 'b, 'c, 'd> HelpWriter<'a, 'b, 'c, 'd> {
    /// Writes help for all arguments (options, flags, args, subcommands)
    /// including titles of a Parser Object to the wrapped stream.
    #[cfg_attr(feature = "lints", allow(useless_let_if_seq))]
    #[cfg_attr(feature = "cargo-clippy", allow(useless_let_if_seq))]
    pub fn write_all_args<W: Write>(&mut self, w: &mut W) -> ClapResult<()> {
        debugln!("HelpWriter::write_all_args;");
        let flags = self.parser.has_flags();
        let pos = positionals!(self.parser.app)
            .filter(|arg| !arg.is_set(ArgSettings::Hidden))
            .count() > 0;
        let opts = self.parser.has_opts();
        let subcmds = self.parser.has_subcommands();

        let unified_help = self.parser.is_set(AppSettings::UnifiedHelpMessage);

        let mut first = true;

        if unified_help && (flags || opts) {
            let opts_flags = flags!(self.parser.app).chain(
                opts!(self.parser.app)
            );
            try!(color!(self, w, "OPTIONS:\n", warning));
            try!(self.write_args(w, opts_flags));
            first = false;
        } else {
            if flags {
                try!(color!(self, w, "FLAGS:\n", warning));
                try!(self.write_args(w, flags!(self.parser.app)));
                first = false;
            }
            if opts {
                if !first {
                    try!(w.write_all(b"\n\n"));
                }
                try!(color!(self, w, "OPTIONS:\n", warning));
                try!(self.write_args(w, opts!(self.parser.app)));
                first = false;
            }
        }

        if pos {
            if !first {
                try!(w.write_all(b"\n\n"));
            }
            try!(color!(self, w, "ARGS:\n", warning));
            try!(self.write_args_unsorted(
                w,
                positionals!(self.parser.app)
            ));
            first = false;
        }

        if subcmds {
            if !first {
                try!(w.write_all(b"\n\n"));
            }
            try!(color!(self, w, "SUBCOMMANDS:\n", warning));
            try!(self.write_subcommands(w));
        }

        Ok(())
    }

    /// Writes help for subcommands of a Parser Object to the wrapped stream.
    fn write_subcommands<W: Write>(&mut self, w: &mut W) -> io::Result<()> {
        debugln!("HelpWriter::write_subcommands;");
        // The shortest an arg can legally be is 2 (i.e. '-x')
        self.longest = 2;
        let mut ord_m = VecMap::new();
        for sc in subcommands!(self.parser.app)
            .filter(|s| !s.is_set(AppSettings::Hidden))
        {
            let btm = ord_m.entry(sc.display_order).or_insert(BTreeMap::new());
            self.longest = cmp::max(self.longest, str_width(sc.name.as_str()));
            //self.longest = cmp::max(self.longest, sc.p.app.name.len());
            btm.insert(sc.name.clone(), sc.clone());
        }

        let mut first = true;
        for btm in ord_m.values() {
            for sc in btm.values() {
                if first {
                    first = false;
                } else {
                    try!(w.write_all(b"\n"));
                }
                try!(self.write_subcommand_as_arg(w, sc));
            }
        }
        Ok(())
    }

    /// Writes version of a Parser Object to the wrapped stream.
    fn write_only_version<W: Write>(&mut self, w: &mut W) -> io::Result<()> {
        debugln!("HelpWriter::write_only_version;");
        try!(write!(w, "{}", self.parser.app.version.unwrap_or("")));
        Ok(())
    }

    /// Writes binary name of a Parser Object to the wrapped stream.
    fn write_bin_name<W: Write>(&mut self, w: &mut W) -> io::Result<()> {
        debugln!("HelpWriter::write_bin_name;");
        macro_rules! write_name {
            () => {{
                let mut name = self.parser.app.name.clone();
                name = name.replace("{n}", "\n");
                try!(color!(self, w, wrap_help(&name, self.term_width), good));
            }};
        }
        if let Some(bn) = self.parser.app.bin_name.as_ref() {
            if bn.contains(' ') {
                // Incase we're dealing with subcommands i.e. git mv is translated to git-mv
                try!(color!(self, w, bn.replace(" ", "-"), good))
            } else {
                write_name!();
            }
        } else {
            write_name!();
        }
        Ok(())
    }

    /// Writes default help for a Parser Object to the wrapped stream.
    pub fn write_default_help<W: Write>(&mut self, w: &mut W) -> ClapResult<()> {
        debugln!("HelpWriter::write_default_help;");
        if let Some(h) = self.parser.app.before_help {
            try!(self.write_before_after_help(w, h));
            try!(w.write_all(b"\n\n"));
        }

        macro_rules! write_thing {
            ($thing:expr) => {{
                let mut owned_thing = $thing.to_owned();
                owned_thing = owned_thing.replace("{n}", "\n");
                try!(write!(w, "{}\n",
                            wrap_help(&owned_thing, self.term_width)))
            }};
        }
        // Print the version
        try!(self.write_bin_name(w));
        try!(w.write_all(b" "));
        try!(self.write_only_version(w));
        try!(w.write_all(b"\n"));
        if let Some(author) = self.parser.app.author {
            write_thing!(author)
        }
        if let Some(about) = self.parser.app.about {
            write_thing!(about)
        }

        try!(color!(self, w, "\nUSAGE:", warning));
        try!(write!(
            w,
            "\n{}{}\n\n",
            TAB,
            self.parser.create_usage_no_title(&[])
        ));

        let flags = self.parser.has_flags();
        let pos = self.parser.has_positionals();
        let opts = self.parser.has_opts();
        let subcmds = self.parser.has_subcommands();

        if flags || opts || pos || subcmds {
            try!(self.write_all_args(w));
        }

        if let Some(h) = self.parser.app.after_help {
            if flags || opts || pos || subcmds {
                try!(w.write_all(b"\n\n"));
            }
            try!(self.write_before_after_help(w, h));
        }

        w.flush().map_err(ClapError::from)
    }
}

/// Possible results for a copying function that stops when a given
/// byte was found.
enum CopyUntilResult {
    DelimiterFound(usize),
    DelimiterNotFound(usize),
    ReaderEmpty,
    ReadError(io::Error),
    WriteError(io::Error),
}

/// Copies the contents of a reader into a writer until a delimiter byte is found.
/// On success, the total number of bytes that were
/// copied from reader to writer is returned.
fn copy_until<R: Read, W: Write>(r: &mut R, w: &mut W, delimiter_byte: u8) -> CopyUntilResult {
    debugln!("copy_until;");

    let mut count = 0;
    for wb in r.bytes() {
        match wb {
            Ok(b) => {
                if b == delimiter_byte {
                    return CopyUntilResult::DelimiterFound(count);
                }
                match w.write(&[b]) {
                    Ok(c) => count += c,
                    Err(e) => return CopyUntilResult::WriteError(e),
                }
            }
            Err(e) => return CopyUntilResult::ReadError(e),
        }
    }
    if count > 0 {
        CopyUntilResult::DelimiterNotFound(count)
    } else {
        CopyUntilResult::ReaderEmpty
    }
}

/// Copies the contents of a reader into a writer until a {tag} is found,
/// copying the tag content to a buffer and returning its size.
/// In addition to errors, there are three possible outputs:
///   - `None`: The reader was consumed.
///   - `Some(Ok(0))`: No tag was captured but the reader still contains data.
///   - `Some(Ok(length>0))`: a tag with `length` was captured to the `tag_buffer`.
fn copy_and_capture<R: Read, W: Write>(
    r: &mut R,
    w: &mut W,
    tag_buffer: &mut Cursor<Vec<u8>>,
) -> Option<io::Result<usize>> {
    use self::CopyUntilResult::*;
    debugln!("copy_and_capture;");

    // Find the opening byte.
    match copy_until(r, w, b'{') {

        // The end of the reader was reached without finding the opening tag.
        // (either with or without having copied data to the writer)
        // Return None indicating that we are done.
        ReaderEmpty |
        DelimiterNotFound(_) => None,

        // Something went wrong.
        ReadError(e) | WriteError(e) => Some(Err(e)),

        // The opening byte was found.
        // (either with or without having copied data to the writer)
        DelimiterFound(_) => {

            // Lets reset the buffer first and find out how long it is.
            tag_buffer.set_position(0);
            let buffer_size = tag_buffer.get_ref().len();

            // Find the closing byte,limiting the reader to the length of the buffer.
            let mut rb = r.take(buffer_size as u64);
            match copy_until(&mut rb, tag_buffer, b'}') {

                // We were already at the end of the reader.
                // Return None indicating that we are done.
                ReaderEmpty => None,

                // The closing tag was found.
                // Return the tag_length.
                DelimiterFound(tag_length) => Some(Ok(tag_length)),

                // The end of the reader was found without finding the closing tag.
                // Write the opening byte and captured text to the writer.
                // Return 0 indicating that nothing was caputred but the reader still contains data.
                DelimiterNotFound(not_tag_length) => {
                    match w.write(b"{") {
                        Err(e) => Some(Err(e)),
                        _ => {
                            match w.write(&tag_buffer.get_ref()[0..not_tag_length]) {
                                Err(e) => Some(Err(e)),
                                _ => Some(Ok(0)),
                            }
                        }
                    }
                }

                ReadError(e) | WriteError(e) => Some(Err(e)),
            }
        }
    }
}


// Methods to write Parser help using templates.
impl<'a, 'b, 'c, 'd> HelpWriter<'a, 'b, 'c, 'd> {
    /// Write help to stream for the parser in the format defined by the template.
    ///
    /// Tags arg given inside curly brackets:
    /// Valid tags are:
    ///     * `{bin}`         - Binary name.
    ///     * `{version}`     - Version number.
    ///     * `{author}`      - Author information.
    ///     * `{usage}`       - Automatically generated or given usage string.
    ///     * `{all-args}`    - Help for all arguments (options, flags, positionals arguments,
    ///                         and subcommands) including titles.
    ///     * `{unified}`     - Unified help for options and flags.
    ///     * `{flags}`       - Help for flags.
    ///     * `{options}`     - Help for options.
    ///     * `{positionals}` - Help for positionals arguments.
    ///     * `{subcommands}` - Help for subcommands.
    ///     * `{after-help}`  - Info to be displayed after the help message.
    ///     * `{before-help}` - Info to be displayed before the help message.
    ///
    /// The template system is, on purpose, very simple. Therefore the tags have to writen
    /// in the lowercase and without spacing.
    fn write_templated_help<W: Write>(&mut self, w: &mut W, template: &str) -> ClapResult<()> {
        debugln!("HelpWriter::write_templated_help;");
        let mut tmplr = Cursor::new(&template);
        let mut tag_buf = Cursor::new(vec![0u8; 15]);

        // The strategy is to copy the template from the the reader to wrapped stream
        // until a tag is found. Depending on its value, the appropriate content is copied
        // to the wrapped stream.
        // The copy from template is then resumed, repeating this sequence until reading
        // the complete template.

        loop {
            let tag_length = match copy_and_capture(&mut tmplr, w, &mut tag_buf) {
                None => return Ok(()),
                Some(Err(e)) => return Err(ClapError::from(e)),
                Some(Ok(val)) if val > 0 => val,
                _ => continue,
            };

            debugln!("HelpWriter::write_template_help:iter: tag_buf={};", unsafe {
                String::from_utf8_unchecked(
                    tag_buf.get_ref()[0..tag_length]
                        .iter()
                        .map(|&i| i)
                        .collect::<Vec<_>>(),
                )
            });
            match &tag_buf.get_ref()[0..tag_length] {
                b"?" => {
                    try!(w.write_all(b"Could not decode tag name"));
                }
                b"bin" => {
                    try!(self.write_bin_name(w));
                }
                b"version" => {
                    try!(write!(
                        w,
                        "{}",
                        self.parser.app.version.unwrap_or("unknown version")
                    ));
                }
                b"author" => {
                    try!(write!(
                        w,
                        "{}",
                        self.parser.app.author.unwrap_or("unknown author")
                    ));
                }
                b"about" => {
                    try!(write!(
                        w,
                        "{}",
                        self.parser.app.about.unwrap_or("unknown about")
                    ));
                }
                b"usage" => {
                    try!(write!(
                        w,
                        "{}",
                        self.parser.create_usage_no_title(&[])
                    ));
                }
                b"all-args" => {
                    try!(self.write_all_args(w));
                }
                b"unified" => {
                    let opts_flags = flags!(self.parser.app).chain(
                        opts!(self.parser.app)
                    );
                    try!(self.write_args(w, opts_flags));
                }
                b"flags" => {
                    try!(self.write_args(w, flags!(self.parser.app)));
                }
                b"options" => {
                    try!(self.write_args(w, opts!(self.parser.app)));
                }
                b"positionals" => {
                    try!(self.write_args(w, positionals!(self.parser.app)));
                }
                b"subcommands" => {
                    try!(self.write_subcommands(w));
                }
                b"after-help" => {
                    try!(write!(
                        w,
                        "{}",
                        self.parser.app.after_help.unwrap_or("unknown after-help")
                    ));
                }
                b"before-help" => {
                    try!(write!(
                        w,
                        "{}",
                        self.parser.app.before_help.unwrap_or("unknown before-help")
                    ));
                }
                // Unknown tag, write it back.
                r => {
                    try!(w.write_all(b"{"));
                    try!(w.write_all(r));
                    try!(w.write_all(b"}"));
                }
            }
        }
    }
}

fn wrap_help(help: &str, avail_chars: usize) -> String {
    let wrapper = textwrap::Wrapper::new(avail_chars).break_words(false);
    help.lines()
        .map(|line| wrapper.fill(line))
        .collect::<Vec<String>>()
        .join("\n")
}

#[cfg(test)]
mod test {
    use super::wrap_help;

    #[test]
    fn wrap_help_last_word() {
        let help = String::from("foo bar baz");
        assert_eq!(wrap_help(&help, 5), "foo\nbar\nbaz");
    }
}
