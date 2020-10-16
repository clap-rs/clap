// Std
use std::{
    borrow::Cow,
    cmp,
    collections::BTreeMap,
    io::{self, Write},
    usize,
};

// Internal
use crate::{
    build::{App, AppSettings, Arg, ArgSettings},
    output::{fmt::Colorizer, Usage},
    parse::Parser,
    util::VecMap,
    INTERNAL_ERROR_MSG,
};

// Third party
use indexmap::IndexSet;
use unicode_width::UnicodeWidthStr;

pub(crate) fn dimensions() -> Option<(usize, usize)> {
    #[cfg(not(feature = "wrap_help"))]
    return None;

    #[cfg(feature = "wrap_help")]
    terminal_size::terminal_size().map(|(w, h)| (w.0.into(), h.0.into()))
}

fn str_width(s: &str) -> usize {
    UnicodeWidthStr::width(s)
}

const TAB: &str = "    ";

pub(crate) enum HelpWriter<'writer> {
    Normal(&'writer mut dyn Write),
    Buffer(&'writer mut Colorizer),
}

impl<'writer> HelpWriter<'writer> {
    fn good<T: Into<String> + AsRef<[u8]>>(&mut self, msg: T) -> io::Result<()> {
        match self {
            HelpWriter::Buffer(c) => {
                c.good(msg.into());
                Ok(())
            }
            HelpWriter::Normal(w) => w.write_all(msg.as_ref()),
        }
    }

    fn warning<T: Into<String> + AsRef<[u8]>>(&mut self, msg: T) -> io::Result<()> {
        match self {
            HelpWriter::Buffer(c) => {
                c.warning(msg.into());
                Ok(())
            }
            HelpWriter::Normal(w) => w.write_all(msg.as_ref()),
        }
    }

    fn none<T: Into<String> + AsRef<[u8]>>(&mut self, msg: T) -> io::Result<()> {
        match self {
            HelpWriter::Buffer(c) => {
                c.none(msg.into());
                Ok(())
            }
            HelpWriter::Normal(w) => w.write_all(msg.as_ref()),
        }
    }

    fn spaces(&mut self, n: usize) -> io::Result<()> {
        self.none(" ".repeat(n))
    }
}

impl<'writer, 'help> HelpWriter<'writer> {
    /// Writes argument's short command to the wrapped stream.
    pub fn short(&mut self, arg: &Arg<'help>) -> io::Result<()> {
        debug!("Help::short");

        self.none(TAB)?;

        if let Some(s) = arg.short {
            self.good(&format!("-{}", s))
        } else if arg.has_switch() {
            self.none(TAB)
        } else {
            Ok(())
        }
    }

    /// Writes argument's long command to the wrapped stream.
    pub fn long(&mut self, arg: &Arg<'help>) -> io::Result<()> {
        debug!("Help::long");
        if !arg.has_switch() {
            return Ok(());
        }
        if arg.is_set(ArgSettings::TakesValue) {
            if let Some(l) = arg.long {
                if arg.short.is_some() {
                    self.none(", ")?;
                }
                self.good(&format!("--{}", l))?
            }

            let sep = if arg.is_set(ArgSettings::RequireEquals) {
                "="
            } else {
                " "
            };
            self.none(sep)?;
        } else if let Some(l) = arg.long {
            if arg.short.is_some() {
                self.none(", ")?;
            }
            self.good(&format!("--{}", l))?;
        }
        Ok(())
    }

    /// Writes argument's possible values to the wrapped stream.
    pub fn val(&mut self, arg: &Arg, next_line_help: bool, longest: usize) -> io::Result<()> {
        debug!("Help::val: arg={}", arg.name);
        let mult =
            arg.is_set(ArgSettings::MultipleValues) || arg.is_set(ArgSettings::MultipleOccurrences);
        if arg.is_set(ArgSettings::TakesValue) || arg.index.is_some() {
            let delim = if arg.is_set(ArgSettings::RequireDelimiter) {
                arg.val_delim.expect(INTERNAL_ERROR_MSG)
            } else {
                ' '
            };
            if !arg.val_names.is_empty() {
                let mut it = arg.val_names.iter().peekable();
                while let Some((_, val)) = it.next() {
                    self.good(&format!("<{}>", val))?;
                    if it.peek().is_some() {
                        self.none(&delim.to_string())?;
                    }
                }
                let num = arg.val_names.len();
                if mult && num == 1 {
                    self.good("...")?;
                }
            } else if let Some(num) = arg.num_vals {
                let mut it = (0..num).peekable();
                while let Some(_) = it.next() {
                    self.good(&format!("<{}>", arg.name))?;
                    if it.peek().is_some() {
                        self.none(&delim.to_string())?;
                    }
                }
                if mult && num == 1 {
                    self.good("...")?;
                }
            } else if arg.has_switch() {
                self.good(&format!("<{}>", arg.name))?;
                if mult {
                    self.good("...")?;
                }
            } else {
                self.good(&arg.to_string())?;
            }
        }

        if arg.has_switch() {
            if !next_line_help {
                debug!("No");
                let self_len = str_width(arg.to_string().as_str());
                // subtract ourself
                let mut spcs = longest - self_len;
                // Since we're writing spaces from the tab point we first need to know if we
                // had a long and short, or just short
                if arg.long.is_some() {
                    // Only account 4 after the val
                    spcs += 4;
                } else {
                    // Only account for ', --' + 4 after the val
                    spcs += 8;
                }

                self.spaces(spcs)?;
            } else {
                debug!("Yes");
            }
        } else if !next_line_help {
            debug!("No, and not next_line");
            self.spaces(longest + 4 - str_width(&arg.to_string()))?;
        } else {
            debug!("No");
        }
        Ok(())
    }

    /// Writes argument's help to the wrapped stream.
    pub fn help(
        &mut self,
        arg: &Arg,
        spec_vals: &str,
        term_w: usize,
        longest: usize,
        prevent_nlh: bool,
        use_long: bool,
        next_line_help: bool,
    ) -> io::Result<()> {
        debug!("Help::help");
        let about = if use_long {
            arg.long_about.unwrap_or_else(|| arg.about.unwrap_or(""))
        } else {
            arg.about.unwrap_or_else(|| arg.long_about.unwrap_or(""))
        };
        let mut help = String::from(about) + spec_vals;
        debug!("Help::help: Next Line...{:?}", next_line_help);

        let spaces = if next_line_help {
            12 // "tab" * 3
        } else {
            longest + 12
        };

        let too_long = spaces + str_width(about) + str_width(spec_vals) >= term_w;

        // Is help on next line, if so then indent
        if next_line_help {
            self.none(&format!("\n{}{}{}", TAB, TAB, TAB))?;
        }

        debug!("Help::help: Too long...");
        if too_long && spaces <= term_w {
            debug!("Yes");
            debug!("Help::help: help...{}", help);
            debug!("Help::help: help width...{}", str_width(&*help));
            // Determine how many newlines we need to insert
            let avail_chars = term_w - spaces;
            debug!("Help::help: Usable space...{}", avail_chars);
            help = text_wrapper(&help, avail_chars);
        } else {
            debug!("No");
        }
        if let Some(part) = help.lines().next() {
            self.none(part)?;
        }
        for part in help.lines().skip(1) {
            self.none("\n")?;
            if next_line_help {
                self.none(&format!("{}{}{}", TAB, TAB, TAB))?;
            } else if arg.has_switch() {
                self.spaces(longest + 12)?;
            } else {
                self.spaces(longest + 8)?;
            }
            self.none(part)?;
        }
        if !prevent_nlh && next_line_help {
            self.none("\n")?;
        }
        Ok(())
    }

    pub fn sc_val(&mut self, sc_str: &str, next_line: bool, longest: usize) -> io::Result<()> {
        if !next_line {
            self.spaces(longest + 4 - str_width(sc_str))?;
        }
        Ok(())
    }

    pub fn sc_help(
        &mut self,
        app: &App<'help>,
        spec_vals: &str,
        next_line: bool,
        use_long: bool,
        longest: usize,
        term_w: usize,
    ) -> io::Result<()> {
        debug!("Help::sc_help");
        let about = if use_long {
            app.long_about.unwrap_or_else(|| app.about.unwrap_or(""))
        } else {
            app.about.unwrap_or_else(|| app.long_about.unwrap_or(""))
        };
        let next_line = next_line || use_long;
        debug!("Help::sc_help: Next Line...{:?}", next_line);

        let spaces = if next_line {
            12 // "tab" * 3
        } else {
            longest + 12
        };

        let too_long = spaces + str_width(about) + str_width(&spec_vals) >= term_w;

        // Is help on next line, if so then indent
        if next_line {
            self.none(&format!("\n{}{}{}", TAB, TAB, TAB))?;
        }

        debug!("Help::sc_help: Too long...");
        let mut help = String::from(about) + spec_vals;
        if too_long && spaces <= term_w {
            debug!("Yes");
            debug!("Help::sc_help: help...{}", help);
            debug!("Help::sc_help: help width...{}", str_width(&*help));
            // Determine how many newlines we need to insert
            let avail_chars = term_w - spaces;
            debug!("Help::sc_help: Usable space...{}", avail_chars);
            help = text_wrapper(&help, avail_chars);
        } else {
            debug!("No");
        }
        if let Some(part) = help.lines().next() {
            self.none(part)?;
        }
        for part in help.lines().skip(1) {
            self.none("\n")?;
            if next_line {
                self.none(&format!("{}{}{}", TAB, TAB, TAB))?;
            } else {
                self.spaces(longest + 8)?;
            }
            self.none(part)?;
        }
        if !help.contains('\n') && next_line {
            self.none("\n")?;
        }
        Ok(())
    }

    /// Writes help for an argument to the wrapped stream.
    fn write_arg(
        &mut self,
        arg: &Arg<'help>,
        prevent_nlh: bool,
        next_line_help: bool,
        spec_vals: &str,
        longest: usize,
        term_w: usize,
        use_long: bool,
    ) -> io::Result<()> {
        self.short(arg)?;
        self.long(arg)?;
        self.val(arg, next_line_help, longest)?;
        self.help(
            arg,
            spec_vals,
            term_w,
            longest,
            prevent_nlh,
            use_long,
            next_line_help,
        )?;
        Ok(())
    }

    pub fn write_subcommand(
        &mut self,
        sc_str: &str,
        spec_vals: &str,
        app: &App<'help>,
        next_line_help: bool,
        longest: usize,
        term_w: usize,
        use_long: bool,
    ) -> io::Result<()> {
        debug!("Help::write_subcommand");

        self.none(TAB)?;
        self.good(sc_str)?;
        self.sc_val(sc_str, next_line_help, longest)?;
        self.sc_help(app, spec_vals, next_line_help, use_long, longest, term_w)?;
        Ok(())
    }
}

/// `clap` Help Writer.
///
/// Wraps a writer stream providing different methods to generate help for `clap` objects.
pub(crate) struct Help<'help, 'app, 'parser, 'writer> {
    writer: HelpWriter<'writer>,
    parser: &'parser Parser<'help, 'app>,
    next_line_help: bool,
    hide_pv: bool,
    term_w: usize,
    use_long: bool,
}

// Public Functions
impl<'help, 'app, 'parser, 'writer> Help<'help, 'app, 'parser, 'writer> {
    const DEFAULT_TEMPLATE: &'static str = "\
        {before-help}{bin} {version}\n\
        {author-with-newline}{about-with-newline}\n\
        {usage-heading}\n    {usage}\n\
        \n\
        {all-args}{after-help}\
    ";

    const DEFAULT_NO_ARGS_TEMPLATE: &'static str = "\
        {before-help}{bin} {version}\n\
        {author-with-newline}{about-with-newline}\n\
        {usage-heading}\n    {usage}{after-help}\
    ";

    /// Create a new `Help` instance.
    pub(crate) fn new(
        writer: HelpWriter<'writer>,
        parser: &'parser Parser<'help, 'app>,
        use_long: bool,
    ) -> Self {
        debug!("Help::new");
        let term_w = match parser.app.term_w {
            Some(0) => usize::MAX,
            Some(w) => w,
            None => cmp::min(
                dimensions().map_or(100, |(w, _)| w),
                match parser.app.max_w {
                    None | Some(0) => usize::MAX,
                    Some(mw) => mw,
                },
            ),
        };
        let next_line_help = parser.is_set(AppSettings::NextLineHelp);
        let hide_pv = parser.is_set(AppSettings::HidePossibleValuesInHelp);

        Help {
            writer,
            parser,
            next_line_help,
            hide_pv,
            term_w,
            use_long,
        }
    }

    /// Writes the parser help to the wrapped stream.
    pub(crate) fn write_help(&mut self) -> io::Result<()> {
        debug!("Help::write_help");

        if let Some(h) = self.parser.app.help_str {
            self.writer.none(h)?;
        } else if let Some(tmpl) = self.parser.app.template {
            self.write_templated_help(tmpl)?;
        } else {
            let flags = self.parser.has_flags();
            let pos = self.parser.has_positionals();
            let opts = self.parser.has_opts();
            let subcmds = self.parser.has_subcommands();

            if flags || opts || pos || subcmds {
                self.write_templated_help(Self::DEFAULT_TEMPLATE)?;
            } else {
                self.write_templated_help(Self::DEFAULT_NO_ARGS_TEMPLATE)?;
            }
        }

        self.writer.none("\n")?;

        Ok(())
    }
}

// Methods to write Arg help.
impl<'help, 'app, 'parser, 'writer> Help<'help, 'app, 'parser, 'writer> {
    fn none<T: Into<String> + AsRef<[u8]>>(&mut self, msg: T) -> io::Result<()> {
        self.writer.none(msg)
    }

    /// Writes help for each argument in the order they were declared to the wrapped stream.
    fn write_args_unsorted(&mut self, args: &[&Arg<'help>]) -> io::Result<()> {
        debug!("Help::write_args_unsorted");
        // The shortest an arg can legally be is 2 (i.e. '-x')
        let mut longest = 2;
        let mut arg_v = Vec::with_capacity(10);
        let use_long = self.use_long;
        for arg in args.iter().filter(|arg| should_show_arg(use_long, *arg)) {
            if arg.longest_filter() {
                longest = longest.max(str_width(arg.to_string().as_str()));
            }
            arg_v.push(arg)
        }
        let mut first = true;
        let arg_c = arg_v.len();
        for (i, arg) in arg_v.iter().enumerate() {
            if first {
                first = false;
            } else {
                self.writer.none("\n")?;
            }
            self.write_arg(arg, i < arg_c, longest)?;
        }
        Ok(())
    }

    /// Will use next line help on writing args.
    fn will_args_wrapping(&self, args: &[&Arg<'help>], use_long: bool, longest: usize) -> bool {
        args.iter()
            .filter(|arg| should_show_arg(use_long, *arg))
            .position(|arg| {
                let spec_vals = &self.spec_vals(arg);
                self.arg_next_line_help(arg, spec_vals, longest)
            })
            .is_some()
    }

    /// Sorts arguments by length and display order and write their help to the wrapped stream.
    fn write_args(&mut self, args: &[&Arg<'help>]) -> io::Result<()> {
        debug!("Help::write_args");
        // The shortest an arg can legally be is 2 (i.e. '-x')
        let mut longest = 2;
        let mut ord_m = VecMap::new();
        let use_long = self.use_long;

        // Determine the longest
        for arg in args.iter().filter(|arg| {
            // If it's NextLineHelp we don't care to compute how long it is because it may be
            // NextLineHelp on purpose simply *because* it's so long and would throw off all other
            // args alignment
            should_show_arg(use_long, *arg)
        }) {
            if arg.longest_filter() {
                debug!("Help::write_args: Current Longest...{}", longest);
                longest = longest.max(str_width(arg.to_string().as_str()));
                debug!("Help::write_args: New Longest...{}", longest);
            }
            let btm = ord_m.entry(arg.disp_ord).or_insert(BTreeMap::new());

            // Formatting key like this to ensure that:
            // 1. Argument has long flags are printed just after short flags.
            // 2. For two args both have short flags like `-c` and `-C`, the
            //    `-C` arg is printed just after the `-c` arg
            // 3. For args without short or long flag, print them at last(sorted
            //    by arg name).
            // Example order: -a, -b, -B, -s, --select-file, --select-folder, -x

            let key = if let Some(x) = arg.short {
                let mut s = x.to_ascii_lowercase().to_string();
                s.push(if x.is_ascii_lowercase() { '0' } else { '1' });
                s
            } else if let Some(x) = arg.long {
                x.to_string()
            } else {
                let mut s = '{'.to_string();
                s.push_str(arg.name);
                s
            };
            btm.insert(key, arg);
        }

        if self.will_args_wrapping(args, use_long, longest) {
            self.next_line_help = true;
        }

        let mut first = true;
        for btm in ord_m.values() {
            for arg in btm.values() {
                if first {
                    first = false;
                } else {
                    self.none("\n")?;
                }
                self.write_arg(arg, false, longest)?;
            }
        }
        Ok(())
    }

    fn arg_next_line_help(&self, arg: &Arg<'help>, spec_vals: &str, longest: usize) -> bool {
        let setting_next_line =
            self.next_line_help || arg.is_set(ArgSettings::NextLineHelp) || self.use_long;

        let h = arg.about.unwrap_or("");
        let h_w = str_width(h) + str_width(spec_vals);
        let taken = longest + 12;

        let force_next_line = !setting_next_line
            && self.term_w >= taken
            && (taken as f32 / self.term_w as f32) > 0.40
            && h_w > (self.term_w - taken);

        setting_next_line || force_next_line
    }

    /// Writes help for an argument to the wrapped stream.
    fn write_arg(&mut self, arg: &Arg<'help>, prevent_nlh: bool, longest: usize) -> io::Result<()> {
        debug!("Help::write_arg");
        let spec_vals = &self.spec_vals(arg);

        let next_line_help = self.arg_next_line_help(arg, spec_vals, longest);

        self.writer.write_arg(
            arg,
            prevent_nlh,
            next_line_help,
            spec_vals,
            longest,
            self.term_w,
            self.use_long,
        )
    }

    fn spec_vals(&self, a: &Arg) -> String {
        debug!("Help::spec_vals: a={}", a);
        let mut spec_vals = vec![];
        if let Some(ref env) = a.env {
            debug!(
                "Help::spec_vals: Found environment variable...[{:?}:{:?}]",
                env.0, env.1
            );
            let env_val = if !a.is_set(ArgSettings::HideEnvValues) {
                format!(
                    "={}",
                    env.1
                        .as_ref()
                        .map_or(Cow::Borrowed(""), |val| val.to_string_lossy())
                )
            } else {
                String::new()
            };
            let env_info = format!("[env: {}{}]", env.0.to_string_lossy(), env_val);
            spec_vals.push(env_info);
        }
        if !a.is_set(ArgSettings::HideDefaultValue) && !a.default_vals.is_empty() {
            debug!(
                "Help::spec_vals: Found default value...[{:?}]",
                a.default_vals
            );

            let pvs = a
                .default_vals
                .iter()
                .map(|&pvs| pvs.to_string_lossy())
                .map(|pvs| {
                    if pvs.contains(char::is_whitespace) {
                        Cow::from(format!("{:?}", pvs))
                    } else {
                        pvs
                    }
                })
                .collect::<Vec<_>>()
                .join(" ");

            spec_vals.push(format!("[default: {}]", pvs));
        }
        if !a.aliases.is_empty() {
            debug!("Help::spec_vals: Found aliases...{:?}", a.aliases);

            let als = a
                .aliases
                .iter()
                .filter(|&als| als.1) // visible
                .map(|&als| als.0) // name
                .collect::<Vec<_>>()
                .join(", ");

            if !als.is_empty() {
                spec_vals.push(format!("[aliases: {}]", als));
            }
        }

        if !a.short_aliases.is_empty() {
            debug!(
                "Help::spec_vals: Found short aliases...{:?}",
                a.short_aliases
            );

            let als = a
                .short_aliases
                .iter()
                .filter(|&als| als.1) // visible
                .map(|&als| als.0.to_string()) // name
                .collect::<Vec<_>>()
                .join(", ");

            if !als.is_empty() {
                spec_vals.push(format!("[short aliases: {}]", als));
            }
        }

        if !self.hide_pv
            && !a.is_set(ArgSettings::HidePossibleValues)
            && !a.possible_vals.is_empty()
        {
            debug!(
                "Help::spec_vals: Found possible vals...{:?}",
                a.possible_vals
            );

            let pvs = a
                .possible_vals
                .iter()
                .map(|&pv| {
                    if pv.contains(char::is_whitespace) {
                        format!("{:?}", pv)
                    } else {
                        pv.to_string()
                    }
                })
                .collect::<Vec<_>>()
                .join(", ");

            spec_vals.push(format!("[possible values: {}]", pvs));
        }
        let prefix = if !spec_vals.is_empty() && !a.get_about().unwrap_or("").is_empty() {
            " "
        } else {
            ""
        };
        prefix.to_string() + &spec_vals.join(" ")
    }

    fn write_before_help(&mut self) -> io::Result<()> {
        let before_help = if self.use_long {
            self.parser
                .app
                .before_long_help
                .or(self.parser.app.before_help)
        } else {
            self.parser.app.before_help
        };
        if let Some(output) = before_help {
            self.none(text_wrapper(output, self.term_w))?;
            self.none("\n\n")?;
        }
        Ok(())
    }

    fn write_after_help(&mut self) -> io::Result<()> {
        let after_help = if self.use_long {
            self.parser
                .app
                .after_long_help
                .or(self.parser.app.after_help)
        } else {
            self.parser.app.after_help
        };
        if let Some(output) = after_help {
            self.none("\n\n")?;
            self.none(text_wrapper(output, self.term_w))?;
        }
        Ok(())
    }

    fn write_about(&mut self, new_line: bool) -> io::Result<()> {
        let about = if self.use_long {
            self.parser.app.long_about.or(self.parser.app.about)
        } else {
            self.parser.app.about
        };
        if let Some(output) = about {
            self.writer.none(text_wrapper(output, self.term_w))?;
            if new_line {
                self.writer.none("\n")?;
            }
        }
        Ok(())
    }
}

/// Methods to write a single subcommand
impl<'help, 'app, 'parser, 'writer> Help<'help, 'app, 'parser, 'writer> {
    fn subcommand_next_line_help(&self, app: &App<'help>, spec_vals: &str, longest: usize) -> bool {
        let setting_next_line = self.next_line_help;

        let h = app.about.unwrap_or("");
        let h_w = str_width(h) + str_width(spec_vals);

        let taken = longest + 12;
        let force_next_line = !setting_next_line
            && self.term_w >= taken
            && (taken as f32 / self.term_w as f32) > 0.40
            && h_w > (self.term_w - taken);

        setting_next_line | force_next_line
    }

    fn write_subcommand(
        &mut self,
        sc_str: &str,
        app: &App<'help>,
        longest: usize,
    ) -> io::Result<()> {
        debug!("Help::write_subcommand");

        let spec_vals = &self.sc_spec_vals(app);

        let next_line_help = self.subcommand_next_line_help(app, spec_vals, longest);

        self.writer.write_subcommand(
            sc_str,
            spec_vals,
            app,
            next_line_help,
            longest,
            self.term_w,
            self.use_long,
        )
    }

    fn sc_spec_vals(&self, a: &App) -> String {
        debug!("Help::sc_spec_vals: a={}", a.name);
        let mut spec_vals = vec![];
        if !a.aliases.is_empty() || !a.short_flag_aliases.is_empty() {
            debug!("Help::spec_vals: Found aliases...{:?}", a.aliases);
            debug!(
                "Help::spec_vals: Found short flag aliases...{:?}",
                a.short_flag_aliases
            );

            let mut short_als = a
                .get_visible_short_flag_aliases()
                .map(|a| format!("-{}", a))
                .collect::<Vec<_>>();

            let als = a.get_visible_aliases().map(|s| s.to_string());

            short_als.extend(als);

            let all_als = short_als.join(", ");

            if !all_als.is_empty() {
                spec_vals.push(format!(" [aliases: {}]", all_als));
            }
        }
        spec_vals.join(" ")
    }
}

// Methods to write Parser help.
impl<'help, 'app, 'parser, 'writer> Help<'help, 'app, 'parser, 'writer> {
    /// Writes help for all arguments (options, flags, args, subcommands)
    /// including titles of a Parser Object to the wrapped stream.
    pub(crate) fn write_all_args(&mut self) -> io::Result<()> {
        debug!("Help::write_all_args");
        let flags = self.parser.has_flags();
        let pos = self
            .parser
            .app
            .get_positionals()
            .filter(|arg| should_show_arg(self.use_long, arg))
            .any(|_| true);
        let opts = self
            .parser
            .app
            .get_opts_with_no_heading()
            .filter(|arg| should_show_arg(self.use_long, arg))
            .collect::<Vec<_>>();
        let subcmds = self.parser.has_visible_subcommands();

        let custom_headings = self
            .parser
            .app
            .args
            .args
            .iter()
            .filter_map(|arg| arg.help_heading)
            .collect::<IndexSet<_>>();

        let mut first = if pos {
            // Write positional args if any
            self.writer.warning("ARGS:\n")?;
            self.write_args_unsorted(&self.parser.app.get_positionals().collect::<Vec<_>>())?;
            false
        } else {
            true
        };

        let unified_help = self.parser.is_set(AppSettings::UnifiedHelpMessage);

        if unified_help && (flags || !opts.is_empty()) {
            let opts_flags = self
                .parser
                .app
                .args
                .args
                .iter()
                .filter(|a| a.has_switch())
                .collect::<Vec<_>>();
            if !first {
                self.none("\n\n")?;
            }
            self.writer.warning("OPTIONS:\n")?;
            self.write_args(&*opts_flags)?;
            first = false;
        } else {
            if flags {
                if !first {
                    self.none("\n\n")?;
                }
                self.writer.warning("FLAGS:\n")?;
                let flags_v: Vec<_> = self.parser.app.get_flags_with_no_heading().collect();
                self.write_args(&flags_v)?;
                first = false;
            }
            if !opts.is_empty() {
                if !first {
                    self.none("\n\n")?;
                }
                self.writer.warning("OPTIONS:\n")?;
                self.write_args(&opts)?;
                first = false;
            }
            if !custom_headings.is_empty() {
                for heading in custom_headings {
                    if !first {
                        self.none("\n\n")?;
                    }
                    self.writer.warning(&*format!("{}:\n", heading))?;
                    let args = self
                        .parser
                        .app
                        .args
                        .args
                        .iter()
                        .filter(|a| {
                            if let Some(help_heading) = a.help_heading {
                                return help_heading == heading;
                            }
                            false
                        })
                        .collect::<Vec<_>>();
                    self.write_args(&*args)?;
                    first = false
                }
            }
        }

        if subcmds {
            if !first {
                self.none("\n\n")?;
            }

            self.writer
                .warning(self.parser.app.subcommand_header.unwrap_or("SUBCOMMANDS"))?;
            self.writer.warning(":\n")?;

            self.write_subcommands(&self.parser.app)?;
        }

        Ok(())
    }

    /// Will use next line help on writing subcommands.
    fn will_subcommands_wrapping(&self, subcommands: &[App<'help>], longest: usize) -> bool {
        subcommands
            .iter()
            .filter(|&subcommand| should_show_subcommand(subcommand))
            .position(|subcommand| {
                let spec_vals = &self.sc_spec_vals(subcommand);
                self.subcommand_next_line_help(subcommand, spec_vals, longest)
            })
            .is_some()
    }

    /// Writes help for subcommands of a Parser Object to the wrapped stream.
    fn write_subcommands(&mut self, app: &App<'help>) -> io::Result<()> {
        debug!("Help::write_subcommands");
        // The shortest an arg can legally be is 2 (i.e. '-x')
        let mut longest = 2;
        let mut ord_m = VecMap::new();
        for subcommand in app
            .subcommands
            .iter()
            .filter(|subcommand| should_show_subcommand(subcommand))
        {
            let btm = ord_m.entry(subcommand.disp_ord).or_insert(BTreeMap::new());
            let mut sc_str = String::new();
            sc_str.push_str(
                &subcommand
                    .short_flag
                    .map_or(String::new(), |c| format!("-{}, ", c)),
            );
            sc_str.push_str(
                &subcommand
                    .long_flag
                    .map_or(String::new(), |c| format!("--{}, ", c)),
            );
            sc_str.push_str(&subcommand.name);
            longest = longest.max(str_width(&sc_str));
            btm.insert(sc_str, subcommand.clone());
        }

        debug!("Help::write_subcommands longest = {}", longest);

        if self.will_subcommands_wrapping(&app.subcommands, longest) {
            self.next_line_help = true;
        }

        let mut first = true;
        for btm in ord_m.values() {
            for (sc_str, sc) in btm {
                if first {
                    first = false;
                } else {
                    self.none("\n")?;
                }
                self.write_subcommand(sc_str, sc, longest)?;
            }
        }
        Ok(())
    }

    /// Writes binary name of a Parser Object to the wrapped stream.
    fn write_bin_name(&mut self) -> io::Result<()> {
        debug!("Help::write_bin_name");
        let term_w = self.term_w;
        let bin_name = if let Some(bn) = self.parser.app.bin_name.as_ref() {
            if bn.contains(' ') {
                // In case we're dealing with subcommands i.e. git mv is translated to git-mv
                bn.replace(" ", "-")
            } else {
                text_wrapper(&self.parser.app.name, term_w)
            }
        } else {
            text_wrapper(&self.parser.app.name, term_w)
        };
        self.writer.good(&bin_name)?;
        Ok(())
    }
}

// Methods to write Parser help using templates.
impl<'help, 'app, 'parser, 'writer> Help<'help, 'app, 'parser, 'writer> {
    /// Write help to stream for the parser in the format defined by the template.
    ///
    /// For details about the template language see [`App::help_template`].
    ///
    /// [`App::help_template`]: ./struct.App.html#method.help_template
    fn write_templated_help(&mut self, template: &str) -> io::Result<()> {
        debug!("Help::write_templated_help");

        // The strategy is to copy the template from the reader to wrapped stream
        // until a tag is found. Depending on its value, the appropriate content is copied
        // to the wrapped stream.
        // The copy from template is then resumed, repeating this sequence until reading
        // the complete template.

        macro_rules! tags {
            (
                match $part:ident {
                    $( $tag:expr => $action:stmt )*
                }
            ) => {
                match $part {
                    $(
                        part if part.starts_with(concat!($tag, "}")) => {
                            $action
                            let rest = &part[$tag.len()+1..];
                            self.none(rest)?;
                        }
                    )*

                    // Unknown tag, write it back.
                    part => {
                        self.none("{")?;
                        self.none(part)?;
                    }
                }
            };
        }

        let mut parts = template.split('{');
        if let Some(first) = parts.next() {
            self.none(first)?;
        }

        for part in parts {
            tags! {
                match part {
                    "bin" => {
                        self.write_bin_name()?;
                    }
                    "version" => {
                        if let Some(s) = self.parser.app.version {
                            self.none(s)?;
                        }
                    }
                    "author" => {
                        if let Some(s) = self.parser.app.author {
                            self.none(&text_wrapper(s, self.term_w))?;
                        }
                    }
                    "author-with-newline" => {
                        if let Some(s) = self.parser.app.author {
                            self.none(&text_wrapper(s, self.term_w))?;
                            self.none("\n")?;
                        }
                    }
                    "about" => {
                        self.write_about(false)?;
                    }
                    "about-with-newline" => {
                        self.write_about(true)?;
                    }
                    "usage-heading" => {
                        self.writer.warning("USAGE:")?;
                    }
                    "usage" => {
                        self.none(Usage::new(self.parser).create_usage_no_title(&[]))?;
                    }
                    "all-args" => {
                        self.write_all_args()?;
                    }
                    "unified" => {
                        let opts_flags = self
                            .parser
                            .app
                            .args
                            .args
                            .iter()
                            .filter(|a| a.has_switch())
                            .collect::<Vec<_>>();
                        self.write_args(&opts_flags)?;
                    }
                    "flags" => {
                        self.write_args(&self.parser.app.get_flags_with_no_heading().collect::<Vec<_>>())?;
                    }
                    "options" => {
                        self.write_args(&self.parser.app.get_opts_with_no_heading().collect::<Vec<_>>())?;
                    }
                    "positionals" => {
                        self.write_args(&self.parser.app.get_positionals().collect::<Vec<_>>())?;
                    }
                    "subcommands" => {
                        self.write_subcommands(self.parser.app)?;
                    }
                    "after-help" => {
                        self.write_after_help()?;
                    }
                    "before-help" => {
                        self.write_before_help()?;
                    }
                }
            }
        }

        Ok(())
    }
}

fn should_show_arg(use_long: bool, arg: &Arg) -> bool {
    debug!("should_show_arg: use_long={:?}, arg={}", use_long, arg.name);
    if arg.is_set(ArgSettings::Hidden) {
        return false;
    }
    (!arg.is_set(ArgSettings::HiddenLongHelp) && use_long)
        || (!arg.is_set(ArgSettings::HiddenShortHelp) && !use_long)
        || arg.is_set(ArgSettings::NextLineHelp)
}

fn should_show_subcommand(subcommand: &App) -> bool {
    !subcommand.is_set(AppSettings::Hidden)
}

fn text_wrapper(help: &str, width: usize) -> String {
    let wrapper = textwrap::Wrapper::new(width).break_words(false);
    help.lines()
        .map(|line| wrapper.fill(line))
        .collect::<Vec<String>>()
        .join("\n")
}

#[cfg(test)]
mod test {
    use super::text_wrapper;

    #[test]
    fn wrap_help_last_word() {
        let help = String::from("foo bar baz");
        assert_eq!(text_wrapper(&help, 5), "foo\nbar\nbaz");
    }
}
