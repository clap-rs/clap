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
            self.none(h)?;
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

        self.none("\n")?;

        Ok(())
    }
}

macro_rules! write_method {
    ($_self:ident, $msg:ident, $meth:ident) => {
        match &mut $_self.writer {
            HelpWriter::Buffer(c) => {
                c.$meth(($msg).into());
                Ok(())
            }
            HelpWriter::Normal(w) => w.write_all($msg.as_ref()),
        }
    };
}

// Methods to write Arg help.
impl<'help, 'app, 'parser, 'writer> Help<'help, 'app, 'parser, 'writer> {
    fn good<T: Into<String> + AsRef<[u8]>>(&mut self, msg: T) -> io::Result<()> {
        write_method!(self, msg, good)
    }

    fn warning<T: Into<String> + AsRef<[u8]>>(&mut self, msg: T) -> io::Result<()> {
        write_method!(self, msg, warning)
    }

    fn none<T: Into<String> + AsRef<[u8]>>(&mut self, msg: T) -> io::Result<()> {
        write_method!(self, msg, none)
    }

    fn spaces(&mut self, n: usize) -> io::Result<()> {
        self.none(" ".repeat(n))
    }

    /// Writes help for each argument in the order they were declared to the wrapped stream.
    fn write_args_unsorted(&mut self, args: &[&Arg<'help>]) -> io::Result<()> {
        debug!("Help::write_args_unsorted");
        // The shortest an arg can legally be is 2 (i.e. '-x')
        let mut longest = 2;
        let mut arg_v = Vec::with_capacity(10);

        for arg in args
            .iter()
            .filter(|arg| should_show_arg(self.use_long, *arg))
        {
            if arg.longest_filter() {
                longest = longest.max(str_width(arg.to_string().as_str()));
            }
            arg_v.push(arg)
        }

        let next_line_help = self.will_args_wrap(args, longest);

        let argc = arg_v.len();
        for (i, arg) in arg_v.iter().enumerate() {
            self.write_arg(arg, i + 1 == argc, next_line_help, longest)?;
        }
        Ok(())
    }

    /// Sorts arguments by length and display order and write their help to the wrapped stream.
    fn write_args(&mut self, args: &[&Arg<'help>]) -> io::Result<()> {
        debug!("Help::write_args");
        // The shortest an arg can legally be is 2 (i.e. '-x')
        let mut longest = 2;
        let mut ord_m = VecMap::new();

        // Determine the longest
        for arg in args.iter().filter(|arg| {
            // If it's NextLineHelp we don't care to compute how long it is because it may be
            // NextLineHelp on purpose simply *because* it's so long and would throw off all other
            // args alignment
            should_show_arg(self.use_long, *arg)
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

        let next_line_help = self.will_args_wrap(args, longest);

        let num_ord_m = ord_m.len();
        for (i, btm) in ord_m.values().enumerate() {
            let last_btm = i + 1 == num_ord_m;
            let num_args = btm.len();
            for (i, arg) in btm.values().enumerate() {
                let last_arg = last_btm && i + 1 == num_args;
                self.write_arg(arg, last_arg, next_line_help, longest)?;
            }
        }
        Ok(())
    }

    /// Writes help for an argument to the wrapped stream.
    fn write_arg(
        &mut self,
        arg: &Arg<'help>,
        last_arg: bool,
        next_line_help: bool,
        longest: usize,
    ) -> io::Result<()> {
        let spec_vals = &self.spec_vals(arg);

        self.write_arg_inner(arg, spec_vals, next_line_help, longest)?;

        if !last_arg {
            self.none("\n")?;
            if next_line_help {
                self.none("\n")?;
            }
        }
        Ok(())
    }

    /// Writes argument's short command to the wrapped stream.
    fn short(&mut self, arg: &Arg<'help>) -> io::Result<()> {
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
    fn long(&mut self, arg: &Arg<'help>) -> io::Result<()> {
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
    fn val(&mut self, arg: &Arg<'help>, next_line_help: bool, longest: usize) -> io::Result<()> {
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

        debug!("Help::val: Has switch...");
        if self.use_long {
            // long help prints messages on the next line so it don't need to align text
            debug!("Help::val: printing long help so skip alignment");
        } else if arg.has_switch() {
            debug!("Yes");
            debug!("Help::val: nlh...{:?}", next_line_help);
            if !next_line_help {
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
            }
        } else if !next_line_help {
            debug!("No, and not next_line");
            self.spaces(longest + 4 - str_width(&arg.to_string()))?;
        } else {
            debug!("No");
        }
        Ok(())
    }

    fn write_before_help(&mut self) -> io::Result<()> {
        debug!("Help::write_before_help");
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
        debug!("Help::write_after_help");
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

    /// Writes argument's help to the wrapped stream.
    fn help(
        &mut self,
        has_switch: bool,
        about: &str,
        spec_vals: &str,
        next_line_help: bool,
        longest: usize,
    ) -> io::Result<()> {
        debug!("Help::help");
        let mut help = String::from(about) + spec_vals;
        debug!("Help::help: Next Line...{:?}", next_line_help);

        let spaces = if next_line_help {
            12 // "tab" * 3
        } else {
            longest + 12
        };

        let too_long = spaces + str_width(about) + str_width(spec_vals) >= self.term_w;

        // Is help on next line, if so then indent
        if next_line_help {
            self.none(&format!("\n{}{}{}", TAB, TAB, TAB))?;
        }

        debug!("Help::help: Too long...");
        if too_long && spaces <= self.term_w {
            debug!("Yes");
            debug!("Help::help: help...{}", help);
            debug!("Help::help: help width...{}", str_width(&help));
            // Determine how many newlines we need to insert
            let avail_chars = self.term_w - spaces;
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
            } else if has_switch {
                self.spaces(longest + 12)?;
            } else {
                self.spaces(longest + 8)?;
            }
            self.none(part)?;
        }
        Ok(())
    }

    /// Writes help for an argument to the wrapped stream.
    fn write_arg_inner(
        &mut self,
        arg: &Arg<'help>,
        spec_vals: &str,
        next_line_help: bool,
        longest: usize,
    ) -> io::Result<()> {
        self.short(arg)?;
        self.long(arg)?;
        self.val(arg, next_line_help, longest)?;

        let about = if self.use_long {
            arg.long_about.unwrap_or_else(|| arg.about.unwrap_or(""))
        } else {
            arg.about.unwrap_or_else(|| arg.long_about.unwrap_or(""))
        };

        self.help(arg.has_switch(), about, spec_vals, next_line_help, longest)?;
        Ok(())
    }

    /// Will use next line help on writing args.
    fn will_args_wrap(&self, args: &[&Arg<'help>], longest: usize) -> bool {
        args.iter()
            .filter(|arg| should_show_arg(self.use_long, *arg))
            .any(|arg| {
                let spec_vals = &self.spec_vals(arg);
                self.arg_next_line_help(arg, spec_vals, longest)
            })
    }

    fn arg_next_line_help(&self, arg: &Arg<'help>, spec_vals: &str, longest: usize) -> bool {
        if self.next_line_help || arg.is_set(ArgSettings::NextLineHelp) || self.use_long {
            // setting_next_line
            true
        } else {
            // force_next_line
            let h = arg.about.unwrap_or("");
            let h_w = str_width(h) + str_width(spec_vals);
            let taken = longest + 12;
            self.term_w >= taken
                && (taken as f32 / self.term_w as f32) > 0.40
                && h_w > (self.term_w - taken)
        }
    }

    fn spec_vals(&self, a: &Arg) -> String {
        debug!("Help::spec_vals: a={}", a);
        let mut spec_vals = vec![];
        if let Some(ref env) = a.env {
            if !a.is_set(ArgSettings::HideEnv) {
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

    fn write_about(&mut self, new_line: bool) -> io::Result<()> {
        let about = if self.use_long {
            self.parser.app.long_about.or(self.parser.app.about)
        } else {
            self.parser.app.about
        };
        if let Some(output) = about {
            self.none(text_wrapper(output, self.term_w))?;
            if new_line {
                self.none("\n")?;
            }
        }
        Ok(())
    }

    fn write_author(&mut self, new_line: bool) -> io::Result<()> {
        if let Some(author) = self.parser.app.author {
            self.none(text_wrapper(author, self.term_w))?;
            if new_line {
                self.none("\n")?;
            }
        }
        Ok(())
    }
}

/// Methods to write a single subcommand
impl<'help, 'app, 'parser, 'writer> Help<'help, 'app, 'parser, 'writer> {
    fn write_subcommand(
        &mut self,
        sc_str: &str,
        app: &App<'help>,
        next_line_help: bool,
        longest: usize,
    ) -> io::Result<()> {
        debug!("Help::write_subcommand");

        let spec_vals = &self.sc_spec_vals(app);

        let about = if self.use_long {
            app.long_about.unwrap_or_else(|| app.about.unwrap_or(""))
        } else {
            app.about.unwrap_or_else(|| app.long_about.unwrap_or(""))
        };

        self.subcmd(sc_str, next_line_help, longest)?;
        self.help(false, about, spec_vals, next_line_help, longest)
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

    fn subcommand_next_line_help(&self, app: &App<'help>, spec_vals: &str, longest: usize) -> bool {
        if self.next_line_help | self.use_long {
            // setting_next_line
            true
        } else {
            // force_next_line
            let h = app.about.unwrap_or("");
            let h_w = str_width(h) + str_width(spec_vals);
            let taken = longest + 12;
            self.term_w >= taken
                && (taken as f32 / self.term_w as f32) > 0.40
                && h_w > (self.term_w - taken)
        }
    }

    /// Writes subcommand to the wrapped stream.
    fn subcmd(&mut self, sc_str: &str, next_line_help: bool, longest: usize) -> io::Result<()> {
        self.none(TAB)?;
        self.good(sc_str)?;
        if !next_line_help {
            let width = str_width(sc_str);
            self.spaces(width.max(longest + 4) - width)?;
        }
        Ok(())
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
            .args()
            .filter_map(|arg| arg.help_heading)
            .collect::<IndexSet<_>>();

        let mut first = if pos {
            // Write positional args if any
            self.warning("ARGS:\n")?;
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
                .args()
                .filter(|a| a.has_switch())
                .collect::<Vec<_>>();
            if !first {
                self.none("\n\n")?;
            }
            self.warning("OPTIONS:\n")?;
            self.write_args(&*opts_flags)?;
            first = false;
        } else {
            if flags {
                if !first {
                    self.none("\n\n")?;
                }
                self.warning("FLAGS:\n")?;
                let flags_v: Vec<_> = self.parser.app.get_flags_with_no_heading().collect();
                self.write_args(&flags_v)?;
                first = false;
            }
            if !opts.is_empty() {
                if !first {
                    self.none("\n\n")?;
                }
                self.warning("OPTIONS:\n")?;
                self.write_args(&opts)?;
                first = false;
            }
            if !custom_headings.is_empty() {
                for heading in custom_headings {
                    if !first {
                        self.none("\n\n")?;
                    }
                    self.warning(&*format!("{}:\n", heading))?;
                    let args = self
                        .parser
                        .app
                        .args
                        .args()
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

            self.warning(self.parser.app.subcommand_header.unwrap_or("SUBCOMMANDS"))?;
            self.warning(":\n")?;

            self.write_subcommands(&self.parser.app)?;
        }

        Ok(())
    }

    /// Will use next line help on writing subcommands.
    fn will_subcommands_wrap(&self, subcommands: &[App<'help>], longest: usize) -> bool {
        subcommands
            .iter()
            .filter(|&subcommand| should_show_subcommand(subcommand))
            .any(|subcommand| {
                let spec_vals = &self.sc_spec_vals(subcommand);
                self.subcommand_next_line_help(subcommand, spec_vals, longest)
            })
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

        let next_line_help = self.will_subcommands_wrap(&app.subcommands, longest);

        let mut first = true;
        for btm in ord_m.values() {
            for (sc_str, sc) in btm {
                if first {
                    first = false;
                } else {
                    self.none("\n")?;
                }
                self.write_subcommand(sc_str, sc, next_line_help, longest)?;
            }
        }
        Ok(())
    }

    /// Writes binary name of a Parser Object to the wrapped stream.
    fn write_bin_name(&mut self) -> io::Result<()> {
        debug!("Help::write_bin_name");

        let bin_name = if let Some(bn) = self.parser.app.bin_name.as_ref() {
            if bn.contains(' ') {
                // In case we're dealing with subcommands i.e. git mv is translated to git-mv
                bn.replace(" ", "-")
            } else {
                text_wrapper(&self.parser.app.name, self.term_w)
            }
        } else {
            text_wrapper(&self.parser.app.name, self.term_w)
        };
        self.good(&bin_name)?;
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
                        self.write_author(false)?;
                    }
                    "author-with-newline" => {
                        self.write_author(true)?;
                    }
                    "about" => {
                        self.write_about(false)?;
                    }
                    "about-with-newline" => {
                        self.write_about(true)?;
                    }
                    "usage-heading" => {
                        self.warning("USAGE:")?;
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
                            .args()
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
