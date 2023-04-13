// Std
use std::borrow::Cow;
use std::cmp;
use std::usize;

// Internal
use crate::builder::PossibleValue;
use crate::builder::Str;
use crate::builder::Style;
use crate::builder::StyledStr;
use crate::builder::{Arg, Command};
use crate::output::display_width;
use crate::output::wrap;
use crate::output::Usage;
use crate::output::TAB;
use crate::output::TAB_WIDTH;
use crate::util::FlatSet;

/// `clap` auto-generated help writer
pub(crate) struct AutoHelp<'cmd, 'writer> {
    template: HelpTemplate<'cmd, 'writer>,
}

// Public Functions
impl<'cmd, 'writer> AutoHelp<'cmd, 'writer> {
    /// Create a new `HelpTemplate` instance.
    pub(crate) fn new(
        writer: &'writer mut StyledStr,
        cmd: &'cmd Command,
        usage: &'cmd Usage<'cmd>,
        use_long: bool,
    ) -> Self {
        Self {
            template: HelpTemplate::new(writer, cmd, usage, use_long),
        }
    }

    pub(crate) fn write_help(&mut self) {
        let pos = self
            .template
            .cmd
            .get_positionals()
            .any(|arg| should_show_arg(self.template.use_long, arg));
        let non_pos = self
            .template
            .cmd
            .get_non_positionals()
            .any(|arg| should_show_arg(self.template.use_long, arg));
        let subcmds = self.template.cmd.has_visible_subcommands();

        let template = if non_pos || pos || subcmds {
            DEFAULT_TEMPLATE
        } else {
            DEFAULT_NO_ARGS_TEMPLATE
        };
        self.template.write_templated_help(template);
    }
}

const DEFAULT_TEMPLATE: &str = "\
{before-help}{about-with-newline}
{usage-heading} {usage}

{all-args}{after-help}\
    ";

const DEFAULT_NO_ARGS_TEMPLATE: &str = "\
{before-help}{about-with-newline}
{usage-heading} {usage}{after-help}\
    ";

/// `clap` HelpTemplate Writer.
///
/// Wraps a writer stream providing different methods to generate help for `clap` objects.
pub(crate) struct HelpTemplate<'cmd, 'writer> {
    writer: &'writer mut StyledStr,
    cmd: &'cmd Command,
    usage: &'cmd Usage<'cmd>,
    next_line_help: bool,
    term_w: usize,
    use_long: bool,
}

// Public Functions
impl<'cmd, 'writer> HelpTemplate<'cmd, 'writer> {
    /// Create a new `HelpTemplate` instance.
    pub(crate) fn new(
        writer: &'writer mut StyledStr,
        cmd: &'cmd Command,
        usage: &'cmd Usage<'cmd>,
        use_long: bool,
    ) -> Self {
        debug!(
            "HelpTemplate::new cmd={}, use_long={}",
            cmd.get_name(),
            use_long
        );
        let term_w = match cmd.get_term_width() {
            Some(0) => usize::MAX,
            Some(w) => w,
            None => {
                let (current_width, _h) = dimensions();
                let current_width = current_width.unwrap_or(100);
                let max_width = match cmd.get_max_term_width() {
                    None | Some(0) => usize::MAX,
                    Some(mw) => mw,
                };
                cmp::min(current_width, max_width)
            }
        };
        let next_line_help = cmd.is_next_line_help_set();

        HelpTemplate {
            writer,
            cmd,
            usage,
            next_line_help,
            term_w,
            use_long,
        }
    }

    /// Write help to stream for the parser in the format defined by the template.
    ///
    /// For details about the template language see [`Command::help_template`].
    ///
    /// [`Command::help_template`]: Command::help_template()
    pub(crate) fn write_templated_help(&mut self, template: &str) {
        debug!("HelpTemplate::write_templated_help");

        let mut parts = template.split('{');
        if let Some(first) = parts.next() {
            self.none(first);
        }
        for part in parts {
            if let Some((tag, rest)) = part.split_once('}') {
                match tag {
                    "name" => {
                        self.write_display_name();
                    }
                    #[cfg(not(feature = "unstable-v5"))]
                    "bin" => {
                        self.write_bin_name();
                    }
                    "version" => {
                        self.write_version();
                    }
                    "author" => {
                        self.write_author(false, false);
                    }
                    "author-with-newline" => {
                        self.write_author(false, true);
                    }
                    "author-section" => {
                        self.write_author(true, true);
                    }
                    "about" => {
                        self.write_about(false, false);
                    }
                    "about-with-newline" => {
                        self.write_about(false, true);
                    }
                    "about-section" => {
                        self.write_about(true, true);
                    }
                    "usage-heading" => {
                        self.header("Usage:");
                    }
                    "usage" => {
                        self.writer.push_styled(
                            &self.usage.create_usage_no_title(&[]).unwrap_or_default(),
                        );
                    }
                    "all-args" => {
                        self.write_all_args();
                    }
                    "options" => {
                        // Include even those with a heading as we don't have a good way of
                        // handling help_heading in the template.
                        self.write_args(
                            &self.cmd.get_non_positionals().collect::<Vec<_>>(),
                            "options",
                            option_sort_key,
                        );
                    }
                    "positionals" => {
                        self.write_args(
                            &self.cmd.get_positionals().collect::<Vec<_>>(),
                            "positionals",
                            positional_sort_key,
                        );
                    }
                    "subcommands" => {
                        self.write_subcommands(self.cmd);
                    }
                    "tab" => {
                        self.none(TAB);
                    }
                    "after-help" => {
                        self.write_after_help();
                    }
                    "before-help" => {
                        self.write_before_help();
                    }
                    _ => {
                        self.none("{");
                        self.none(tag);
                        self.none("}");
                    }
                }
                self.none(rest);
            }
        }
    }
}

/// Basic template methods
impl<'cmd, 'writer> HelpTemplate<'cmd, 'writer> {
    /// Writes binary name of a Parser Object to the wrapped stream.
    fn write_display_name(&mut self) {
        debug!("HelpTemplate::write_display_name");

        let display_name = wrap(
            &self
                .cmd
                .get_display_name()
                .unwrap_or_else(|| self.cmd.get_name())
                .replace("{n}", "\n"),
            self.term_w,
        );
        self.none(&display_name);
    }

    /// Writes binary name of a Parser Object to the wrapped stream.
    #[cfg(not(feature = "unstable-v5"))]
    fn write_bin_name(&mut self) {
        debug!("HelpTemplate::write_bin_name");

        let bin_name = if let Some(bn) = self.cmd.get_bin_name() {
            if bn.contains(' ') {
                // In case we're dealing with subcommands i.e. git mv is translated to git-mv
                bn.replace(' ', "-")
            } else {
                wrap(&self.cmd.get_name().replace("{n}", "\n"), self.term_w)
            }
        } else {
            wrap(&self.cmd.get_name().replace("{n}", "\n"), self.term_w)
        };
        self.none(&bin_name);
    }

    fn write_version(&mut self) {
        let version = self
            .cmd
            .get_version()
            .or_else(|| self.cmd.get_long_version());
        if let Some(output) = version {
            self.none(wrap(output, self.term_w));
        }
    }

    fn write_author(&mut self, before_new_line: bool, after_new_line: bool) {
        if let Some(author) = self.cmd.get_author() {
            if before_new_line {
                self.none("\n");
            }
            self.none(wrap(author, self.term_w));
            if after_new_line {
                self.none("\n");
            }
        }
    }

    fn write_about(&mut self, before_new_line: bool, after_new_line: bool) {
        let about = if self.use_long {
            self.cmd.get_long_about().or_else(|| self.cmd.get_about())
        } else {
            self.cmd.get_about()
        };
        if let Some(output) = about {
            if before_new_line {
                self.none("\n");
            }
            let mut output = output.clone();
            output.replace_newline_var();
            output.wrap(self.term_w);
            self.writer.push_styled(&output);
            if after_new_line {
                self.none("\n");
            }
        }
    }

    fn write_before_help(&mut self) {
        debug!("HelpTemplate::write_before_help");
        let before_help = if self.use_long {
            self.cmd
                .get_before_long_help()
                .or_else(|| self.cmd.get_before_help())
        } else {
            self.cmd.get_before_help()
        };
        if let Some(output) = before_help {
            let mut output = output.clone();
            output.replace_newline_var();
            output.wrap(self.term_w);
            self.writer.push_styled(&output);
            self.none("\n\n");
        }
    }

    fn write_after_help(&mut self) {
        debug!("HelpTemplate::write_after_help");
        let after_help = if self.use_long {
            self.cmd
                .get_after_long_help()
                .or_else(|| self.cmd.get_after_help())
        } else {
            self.cmd.get_after_help()
        };
        if let Some(output) = after_help {
            self.none("\n\n");
            let mut output = output.clone();
            output.replace_newline_var();
            output.wrap(self.term_w);
            self.writer.push_styled(&output);
        }
    }
}

/// Arg handling
impl<'cmd, 'writer> HelpTemplate<'cmd, 'writer> {
    /// Writes help for all arguments (options, flags, args, subcommands)
    /// including titles of a Parser Object to the wrapped stream.
    pub(crate) fn write_all_args(&mut self) {
        debug!("HelpTemplate::write_all_args");
        let pos = self
            .cmd
            .get_positionals()
            .filter(|a| a.get_help_heading().is_none())
            .filter(|arg| should_show_arg(self.use_long, arg))
            .collect::<Vec<_>>();
        let non_pos = self
            .cmd
            .get_non_positionals()
            .filter(|a| a.get_help_heading().is_none())
            .filter(|arg| should_show_arg(self.use_long, arg))
            .collect::<Vec<_>>();
        let subcmds = self.cmd.has_visible_subcommands();

        let custom_headings = self
            .cmd
            .get_arguments()
            .filter_map(|arg| arg.get_help_heading())
            .collect::<FlatSet<_>>();

        let mut first = true;

        if subcmds {
            if !first {
                self.none("\n\n");
            }
            first = false;
            let default_help_heading = Str::from("Commands");
            self.header(
                self.cmd
                    .get_subcommand_help_heading()
                    .unwrap_or(&default_help_heading),
            );
            self.header(":");
            self.none("\n");

            self.write_subcommands(self.cmd);
        }

        if !pos.is_empty() {
            if !first {
                self.none("\n\n");
            }
            first = false;
            // Write positional args if any
            self.header("Arguments:");
            self.none("\n");
            self.write_args(&pos, "Arguments", positional_sort_key);
        }

        if !non_pos.is_empty() {
            if !first {
                self.none("\n\n");
            }
            first = false;
            self.header("Options:");
            self.none("\n");
            self.write_args(&non_pos, "Options", option_sort_key);
        }
        if !custom_headings.is_empty() {
            for heading in custom_headings {
                let args = self
                    .cmd
                    .get_arguments()
                    .filter(|a| {
                        if let Some(help_heading) = a.get_help_heading() {
                            return help_heading == heading;
                        }
                        false
                    })
                    .filter(|arg| should_show_arg(self.use_long, arg))
                    .collect::<Vec<_>>();

                if !args.is_empty() {
                    if !first {
                        self.none("\n\n");
                    }
                    first = false;
                    self.header(heading);
                    self.header(":");
                    self.none("\n");
                    self.write_args(&args, heading, option_sort_key);
                }
            }
        }
    }
    /// Sorts arguments by length and display order and write their help to the wrapped stream.
    fn write_args(&mut self, args: &[&Arg], _category: &str, sort_key: ArgSortKey) {
        debug!("HelpTemplate::write_args {}", _category);
        // The shortest an arg can legally be is 2 (i.e. '-x')
        let mut longest = 2;
        let mut ord_v = Vec::new();

        // Determine the longest
        for &arg in args.iter().filter(|arg| {
            // If it's NextLineHelp we don't care to compute how long it is because it may be
            // NextLineHelp on purpose simply *because* it's so long and would throw off all other
            // args alignment
            should_show_arg(self.use_long, arg)
        }) {
            if longest_filter(arg) {
                longest = longest.max(display_width(&arg.to_string()));
                debug!(
                    "HelpTemplate::write_args: arg={:?} longest={}",
                    arg.get_id(),
                    longest
                );
            }

            let key = (sort_key)(arg);
            ord_v.push((key, arg));
        }
        ord_v.sort_by(|a, b| a.0.cmp(&b.0));

        let next_line_help = self.will_args_wrap(args, longest);

        for (i, (_, arg)) in ord_v.iter().enumerate() {
            if i != 0 {
                self.none("\n");
                if next_line_help && self.use_long {
                    self.none("\n");
                }
            }
            self.write_arg(arg, next_line_help, longest);
        }
    }

    /// Writes help for an argument to the wrapped stream.
    fn write_arg(&mut self, arg: &Arg, next_line_help: bool, longest: usize) {
        let spec_vals = &self.spec_vals(arg);

        self.none(TAB);
        self.short(arg);
        self.long(arg);
        self.writer.push_styled(&arg.stylize_arg_suffix(None));
        self.align_to_about(arg, next_line_help, longest);

        let about = if self.use_long {
            arg.get_long_help()
                .or_else(|| arg.get_help())
                .unwrap_or_default()
        } else {
            arg.get_help()
                .or_else(|| arg.get_long_help())
                .unwrap_or_default()
        };

        self.help(Some(arg), about, spec_vals, next_line_help, longest);
    }

    /// Writes argument's short command to the wrapped stream.
    fn short(&mut self, arg: &Arg) {
        debug!("HelpTemplate::short");

        if let Some(s) = arg.get_short() {
            self.literal(format!("-{s}"));
        } else if arg.get_long().is_some() {
            self.none("    ");
        }
    }

    /// Writes argument's long command to the wrapped stream.
    fn long(&mut self, arg: &Arg) {
        debug!("HelpTemplate::long");
        if let Some(long) = arg.get_long() {
            if arg.get_short().is_some() {
                self.none(", ");
            }
            self.literal(format!("--{long}"));
        }
    }

    /// Write alignment padding between arg's switches/values and its about message.
    fn align_to_about(&mut self, arg: &Arg, next_line_help: bool, longest: usize) {
        debug!(
            "HelpTemplate::align_to_about: arg={}, next_line_help={}, longest={}",
            arg.get_id(),
            next_line_help,
            longest
        );
        if self.use_long || next_line_help {
            // long help prints messages on the next line so it doesn't need to align text
            debug!("HelpTemplate::align_to_about: printing long help so skip alignment");
        } else if !arg.is_positional() {
            let self_len = display_width(&arg.to_string());
            // Since we're writing spaces from the tab point we first need to know if we
            // had a long and short, or just short
            let padding = if arg.get_long().is_some() {
                // Only account 4 after the val
                TAB_WIDTH
            } else {
                // Only account for ', --' + 4 after the val
                TAB_WIDTH + 4
            };
            let spcs = longest + padding - self_len;
            debug!(
                "HelpTemplate::align_to_about: positional=false arg_len={}, spaces={}",
                self_len, spcs
            );

            self.spaces(spcs);
        } else {
            let self_len = display_width(&arg.to_string());
            let padding = TAB_WIDTH;
            let spcs = longest + padding - self_len;
            debug!(
                "HelpTemplate::align_to_about: positional=true arg_len={}, spaces={}",
                self_len, spcs
            );

            self.spaces(spcs);
        }
    }

    /// Writes argument's help to the wrapped stream.
    fn help(
        &mut self,
        arg: Option<&Arg>,
        about: &StyledStr,
        spec_vals: &str,
        next_line_help: bool,
        longest: usize,
    ) {
        debug!("HelpTemplate::help");

        // Is help on next line, if so then indent
        if next_line_help {
            debug!("HelpTemplate::help: Next Line...{:?}", next_line_help);
            self.none("\n");
            self.none(TAB);
            self.none(NEXT_LINE_INDENT);
        }

        let spaces = if next_line_help {
            TAB.len() + NEXT_LINE_INDENT.len()
        } else if let Some(true) = arg.map(|a| a.is_positional()) {
            longest + TAB_WIDTH * 2
        } else {
            longest + TAB_WIDTH * 2 + 4 // See `fn short` for the 4
        };
        let trailing_indent = spaces; // Don't indent any further than the first line is indented
        let trailing_indent = self.get_spaces(trailing_indent);

        let mut help = about.clone();
        help.replace_newline_var();
        if !spec_vals.is_empty() {
            if !help.is_empty() {
                let sep = if self.use_long && arg.is_some() {
                    "\n\n"
                } else {
                    " "
                };
                help.none(sep);
            }
            help.none(spec_vals);
        }
        let avail_chars = self.term_w.saturating_sub(spaces);
        debug!(
            "HelpTemplate::help: help_width={}, spaces={}, avail={}",
            spaces,
            help.display_width(),
            avail_chars
        );
        help.wrap(avail_chars);
        help.indent("", &trailing_indent);
        let help_is_empty = help.is_empty();
        self.writer.push_styled(&help);
        if let Some(arg) = arg {
            const DASH_SPACE: usize = "- ".len();
            const COLON_SPACE: usize = ": ".len();
            let possible_vals = arg.get_possible_values();
            if self.use_long
                && !arg.is_hide_possible_values_set()
                && possible_vals.iter().any(PossibleValue::should_show_help)
            {
                debug!(
                    "HelpTemplate::help: Found possible vals...{:?}",
                    possible_vals
                );
                if !help_is_empty {
                    self.none("\n\n");
                    self.spaces(spaces);
                }
                self.none("Possible values:");
                let longest = possible_vals
                    .iter()
                    .filter_map(|f| f.get_visible_quoted_name().map(|name| display_width(&name)))
                    .max()
                    .expect("Only called with possible value");
                let help_longest = possible_vals
                    .iter()
                    .filter_map(|f| f.get_visible_help().map(|h| h.display_width()))
                    .max()
                    .expect("Only called with possible value with help");
                // should new line
                let taken = longest + spaces + DASH_SPACE;

                let possible_value_new_line =
                    self.term_w >= taken && self.term_w < taken + COLON_SPACE + help_longest;

                let spaces = spaces + TAB_WIDTH - DASH_SPACE;
                let trailing_indent = if possible_value_new_line {
                    spaces + DASH_SPACE
                } else {
                    spaces + longest + DASH_SPACE + COLON_SPACE
                };
                let trailing_indent = self.get_spaces(trailing_indent);

                for pv in possible_vals.iter().filter(|pv| !pv.is_hide_set()) {
                    self.none("\n");
                    self.spaces(spaces);
                    self.none("- ");
                    self.literal(pv.get_name());
                    if let Some(help) = pv.get_help() {
                        debug!("HelpTemplate::help: Possible Value help");

                        if possible_value_new_line {
                            self.none(":\n");
                            self.spaces(trailing_indent.len());
                        } else {
                            self.none(": ");
                            // To align help messages
                            self.spaces(longest - display_width(pv.get_name()));
                        }

                        let avail_chars = if self.term_w > trailing_indent.len() {
                            self.term_w - trailing_indent.len()
                        } else {
                            usize::MAX
                        };

                        let mut help = help.clone();
                        help.replace_newline_var();
                        help.wrap(avail_chars);
                        help.indent("", &trailing_indent);
                        self.writer.push_styled(&help);
                    }
                }
            }
        }
    }

    /// Will use next line help on writing args.
    fn will_args_wrap(&self, args: &[&Arg], longest: usize) -> bool {
        args.iter()
            .filter(|arg| should_show_arg(self.use_long, arg))
            .any(|arg| {
                let spec_vals = &self.spec_vals(arg);
                self.arg_next_line_help(arg, spec_vals, longest)
            })
    }

    fn arg_next_line_help(&self, arg: &Arg, spec_vals: &str, longest: usize) -> bool {
        if self.next_line_help || arg.is_next_line_help_set() || self.use_long {
            // setting_next_line
            true
        } else {
            // force_next_line
            let h = arg.get_help().unwrap_or_default();
            let h_w = h.display_width() + display_width(spec_vals);
            let taken = if arg.is_positional() {
                longest + TAB_WIDTH * 2
            } else {
                longest + TAB_WIDTH * 2 + 4 // See `fn short` for the 4
            };
            self.term_w >= taken
                && (taken as f32 / self.term_w as f32) > 0.40
                && h_w > (self.term_w - taken)
        }
    }

    fn spec_vals(&self, a: &Arg) -> String {
        debug!("HelpTemplate::spec_vals: a={}", a);
        let mut spec_vals = Vec::new();
        #[cfg(feature = "env")]
        if let Some(ref env) = a.env {
            if !a.is_hide_env_set() {
                debug!(
                    "HelpTemplate::spec_vals: Found environment variable...[{:?}:{:?}]",
                    env.0, env.1
                );
                let env_val = if !a.is_hide_env_values_set() {
                    format!(
                        "={}",
                        env.1
                            .as_ref()
                            .map(|s| s.to_string_lossy())
                            .unwrap_or_default()
                    )
                } else {
                    Default::default()
                };
                let env_info = format!("[env: {}{}]", env.0.to_string_lossy(), env_val);
                spec_vals.push(env_info);
            }
        }
        if a.is_takes_value_set() && !a.is_hide_default_value_set() && !a.default_vals.is_empty() {
            debug!(
                "HelpTemplate::spec_vals: Found default value...[{:?}]",
                a.default_vals
            );

            let pvs = a
                .default_vals
                .iter()
                .map(|pvs| pvs.to_string_lossy())
                .map(|pvs| {
                    if pvs.contains(char::is_whitespace) {
                        Cow::from(format!("{pvs:?}"))
                    } else {
                        pvs
                    }
                })
                .collect::<Vec<_>>()
                .join(" ");

            spec_vals.push(format!("[default: {pvs}]"));
        }

        let als = a
            .aliases
            .iter()
            .filter(|&als| als.1) // visible
            .map(|als| als.0.as_str()) // name
            .collect::<Vec<_>>()
            .join(", ");
        if !als.is_empty() {
            debug!("HelpTemplate::spec_vals: Found aliases...{:?}", a.aliases);
            spec_vals.push(format!("[aliases: {als}]"));
        }

        let als = a
            .short_aliases
            .iter()
            .filter(|&als| als.1) // visible
            .map(|&als| als.0.to_string()) // name
            .collect::<Vec<_>>()
            .join(", ");
        if !als.is_empty() {
            debug!(
                "HelpTemplate::spec_vals: Found short aliases...{:?}",
                a.short_aliases
            );
            spec_vals.push(format!("[short aliases: {als}]"));
        }

        let possible_vals = a.get_possible_values();
        if !(a.is_hide_possible_values_set()
            || possible_vals.is_empty()
            || self.use_long && possible_vals.iter().any(PossibleValue::should_show_help))
        {
            debug!(
                "HelpTemplate::spec_vals: Found possible vals...{:?}",
                possible_vals
            );

            let pvs = possible_vals
                .iter()
                .filter_map(PossibleValue::get_visible_quoted_name)
                .collect::<Vec<_>>()
                .join(", ");

            spec_vals.push(format!("[possible values: {pvs}]"));
        }
        let connector = if self.use_long { "\n" } else { " " };
        spec_vals.join(connector)
    }

    fn header<T: AsRef<str>>(&mut self, msg: T) {
        self.writer.stylize(Style::Header, msg.as_ref());
    }

    fn literal<T: AsRef<str>>(&mut self, msg: T) {
        self.writer.stylize(Style::Literal, msg.as_ref());
    }

    fn none<T: Into<String>>(&mut self, msg: T) {
        self.writer.none(msg);
    }

    fn get_spaces(&self, n: usize) -> String {
        " ".repeat(n)
    }

    fn spaces(&mut self, n: usize) {
        self.none(self.get_spaces(n));
    }
}

/// Subcommand handling
impl<'cmd, 'writer> HelpTemplate<'cmd, 'writer> {
    /// Writes help for subcommands of a Parser Object to the wrapped stream.
    fn write_subcommands(&mut self, cmd: &Command) {
        debug!("HelpTemplate::write_subcommands");
        // The shortest an arg can legally be is 2 (i.e. '-x')
        let mut longest = 2;
        let mut ord_v = Vec::new();
        for subcommand in cmd
            .get_subcommands()
            .filter(|subcommand| should_show_subcommand(subcommand))
        {
            let mut styled = StyledStr::new();
            styled.stylize(Style::Literal, subcommand.get_name());
            if let Some(short) = subcommand.get_short_flag() {
                styled.none(", ");
                styled.stylize(Style::Literal, &format!("-{short}"));
            }
            if let Some(long) = subcommand.get_long_flag() {
                styled.none(", ");
                styled.stylize(Style::Literal, &format!("--{long}"));
            }
            longest = longest.max(styled.display_width());
            ord_v.push((subcommand.get_display_order(), styled, subcommand));
        }
        ord_v.sort_by(|a, b| (a.0, &a.1).cmp(&(b.0, &b.1)));

        debug!("HelpTemplate::write_subcommands longest = {}", longest);

        let next_line_help = self.will_subcommands_wrap(cmd.get_subcommands(), longest);

        let mut first = true;
        for (_, sc_str, sc) in ord_v {
            if first {
                first = false;
            } else {
                self.none("\n");
            }
            self.write_subcommand(sc_str, sc, next_line_help, longest);
        }
    }

    /// Will use next line help on writing subcommands.
    fn will_subcommands_wrap<'a>(
        &self,
        subcommands: impl IntoIterator<Item = &'a Command>,
        longest: usize,
    ) -> bool {
        subcommands
            .into_iter()
            .filter(|&subcommand| should_show_subcommand(subcommand))
            .any(|subcommand| {
                let spec_vals = &self.sc_spec_vals(subcommand);
                self.subcommand_next_line_help(subcommand, spec_vals, longest)
            })
    }

    fn write_subcommand(
        &mut self,
        sc_str: StyledStr,
        cmd: &Command,
        next_line_help: bool,
        longest: usize,
    ) {
        debug!("HelpTemplate::write_subcommand");

        let spec_vals = &self.sc_spec_vals(cmd);

        let about = cmd
            .get_about()
            .or_else(|| cmd.get_long_about())
            .unwrap_or_default();

        self.subcmd(sc_str, next_line_help, longest);
        self.help(None, about, spec_vals, next_line_help, longest)
    }

    fn sc_spec_vals(&self, a: &Command) -> String {
        debug!("HelpTemplate::sc_spec_vals: a={}", a.get_name());
        let mut spec_vals = vec![];

        let mut short_als = a
            .get_visible_short_flag_aliases()
            .map(|a| format!("-{a}"))
            .collect::<Vec<_>>();
        let als = a.get_visible_aliases().map(|s| s.to_string());
        short_als.extend(als);
        let all_als = short_als.join(", ");
        if !all_als.is_empty() {
            debug!(
                "HelpTemplate::spec_vals: Found aliases...{:?}",
                a.get_all_aliases().collect::<Vec<_>>()
            );
            debug!(
                "HelpTemplate::spec_vals: Found short flag aliases...{:?}",
                a.get_all_short_flag_aliases().collect::<Vec<_>>()
            );
            spec_vals.push(format!("[aliases: {all_als}]"));
        }

        spec_vals.join(" ")
    }

    fn subcommand_next_line_help(&self, cmd: &Command, spec_vals: &str, longest: usize) -> bool {
        if self.next_line_help | self.use_long {
            // setting_next_line
            true
        } else {
            // force_next_line
            let h = cmd.get_about().unwrap_or_default();
            let h_w = h.display_width() + display_width(spec_vals);
            let taken = longest + TAB_WIDTH * 2;
            self.term_w >= taken
                && (taken as f32 / self.term_w as f32) > 0.40
                && h_w > (self.term_w - taken)
        }
    }

    /// Writes subcommand to the wrapped stream.
    fn subcmd(&mut self, sc_str: StyledStr, next_line_help: bool, longest: usize) {
        let width = sc_str.display_width();

        self.none(TAB);
        self.writer.push_styled(&sc_str);
        if !next_line_help {
            self.spaces(longest + TAB_WIDTH - width);
        }
    }
}

const NEXT_LINE_INDENT: &str = "        ";

type ArgSortKey = fn(arg: &Arg) -> (usize, String);

fn positional_sort_key(arg: &Arg) -> (usize, String) {
    (arg.get_index().unwrap_or(0), String::new())
}

fn option_sort_key(arg: &Arg) -> (usize, String) {
    // Formatting key like this to ensure that:
    // 1. Argument has long flags are printed just after short flags.
    // 2. For two args both have short flags like `-c` and `-C`, the
    //    `-C` arg is printed just after the `-c` arg
    // 3. For args without short or long flag, print them at last(sorted
    //    by arg name).
    // Example order: -a, -b, -B, -s, --select-file, --select-folder, -x

    let key = if let Some(x) = arg.get_short() {
        let mut s = x.to_ascii_lowercase().to_string();
        s.push(if x.is_ascii_lowercase() { '0' } else { '1' });
        s
    } else if let Some(x) = arg.get_long() {
        x.to_string()
    } else {
        let mut s = '{'.to_string();
        s.push_str(arg.get_id().as_str());
        s
    };
    (arg.get_display_order(), key)
}

pub(crate) fn dimensions() -> (Option<usize>, Option<usize>) {
    #[cfg(not(feature = "wrap_help"))]
    return (None, None);

    #[cfg(feature = "wrap_help")]
    terminal_size::terminal_size()
        .map(|(w, h)| (Some(w.0.into()), Some(h.0.into())))
        .unwrap_or_else(|| (parse_env("COLUMNS"), parse_env("LINES")))
}

#[cfg(feature = "wrap_help")]
fn parse_env(var: &str) -> Option<usize> {
    some!(some!(std::env::var_os(var)).to_str())
        .parse::<usize>()
        .ok()
}

fn should_show_arg(use_long: bool, arg: &Arg) -> bool {
    debug!(
        "should_show_arg: use_long={:?}, arg={}",
        use_long,
        arg.get_id()
    );
    if arg.is_hide_set() {
        return false;
    }
    (!arg.is_hide_long_help_set() && use_long)
        || (!arg.is_hide_short_help_set() && !use_long)
        || arg.is_next_line_help_set()
}

fn should_show_subcommand(subcommand: &Command) -> bool {
    !subcommand.is_hide_set()
}

fn longest_filter(arg: &Arg) -> bool {
    arg.is_takes_value_set() || arg.get_long().is_some() || arg.get_short().is_none()
}

#[cfg(test)]
mod test {
    #[test]
    #[cfg(feature = "wrap_help")]
    fn wrap_help_last_word() {
        use super::*;

        let help = String::from("foo bar baz");
        assert_eq!(wrap(&help, 5), "foo\nbar\nbaz");
    }

    #[test]
    #[cfg(feature = "unicode")]
    fn display_width_handles_non_ascii() {
        use super::*;

        // Popular Danish tongue-twister, the name of a fruit dessert.
        let text = "rødgrød med fløde";
        assert_eq!(display_width(text), 17);
        // Note that the string width is smaller than the string
        // length. This is due to the precomposed non-ASCII letters:
        assert_eq!(text.len(), 20);
    }

    #[test]
    #[cfg(feature = "unicode")]
    fn display_width_handles_emojis() {
        use super::*;

        let text = "😂";
        // There is a single `char`...
        assert_eq!(text.chars().count(), 1);
        // but it is double-width:
        assert_eq!(display_width(text), 2);
        // This is much less than the byte length:
        assert_eq!(text.len(), 4);
    }
}
