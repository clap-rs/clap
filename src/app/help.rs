use std::io::{self, Cursor, Read, Write};
use std::collections::BTreeMap;
use std::fmt::Display;
use std::cmp;

use vec_map::VecMap;

use errors::{Error, Result as ClapResult};

use args::{AnyArg, ArgSettings, DispOrder};
use app::{App, AppSettings};
use app::parser::Parser;
use fmt::Format;

use term;

#[cfg(all(feature = "wrap_help", not(target_os = "windows")))]
use unicode_width::UnicodeWidthStr;

use strext::_StrExt;

#[cfg(any(not(feature = "wrap_help"), target_os = "windows"))]
fn str_width(s: &str) -> usize {
    s.len()
}

#[cfg(all(feature = "wrap_help", not(target_os = "windows")))]
fn str_width(s: &str) -> usize {
    UnicodeWidthStr::width(s)
}

const TAB: &'static str = "    ";

// These are just convenient traits to make the code easier to read.
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

fn as_arg_trait<'a, 'b, T: ArgWithOrder<'a, 'b>>(x: &T) -> &ArgWithOrder<'a, 'b> {
    x
}

impl<'b, 'c> DispOrder for App<'b, 'c> {
    fn disp_ord(&self) -> usize {
        999
    }
}

macro_rules! color {
    ($_self:ident, $nc:expr, $c:ident) => {
        if $_self.color {
            write!($_self.writer, "{}", Format::$c($nc))
        } else {
            write!($_self.writer, "{}", $nc)
        }
    };
    ($_self:ident, $nc:expr, $i:expr, $c:ident) => {
        if $_self.color {
            write!($_self.writer, "{}", Format::$c(format!($nc, $i)))
        } else {
            write!($_self.writer, $nc, $i)
        }
    };
}

/// `clap` Help Writer.
///
/// Wraps a writer stream providing different methods to generate help for `clap` objects.
pub struct Help<'a> {
    writer: &'a mut Write,
    next_line_help: bool,
    hide_pv: bool,
    term_w: Option<usize>,
    color: bool,
}

// Public Functions
impl<'a> Help<'a> {
    /// Create a new `Help` instance.
    pub fn new(w: &'a mut Write, next_line_help: bool, hide_pv: bool, color: bool) -> Self {
        Help {
            writer: w,
            next_line_help: next_line_help,
            hide_pv: hide_pv,
            term_w: term::dimensions().map(|(w, _)| w),
            color: color,
        }
    }

    /// Reads help settings from an App
    /// and write its help to the wrapped stream.
    pub fn write_app_help(w: &'a mut Write, app: &App) -> ClapResult<()> {
        Self::write_parser_help(w, &app.p)
    }

    /// Reads help settings from a Parser
    /// and write its help to the wrapped stream.
    pub fn write_parser_help(w: &'a mut Write, parser: &Parser) -> ClapResult<()> {
        let nlh = parser.is_set(AppSettings::NextLineHelp);
        let hide_v = parser.is_set(AppSettings::HidePossibleValuesInHelp);
        let color = parser.is_set(AppSettings::ColoredHelp);
        Self::new(w, nlh, hide_v, color).write_help(&parser)
    }

    /// Writes the parser help to the wrapped stream.
    pub fn write_help(&mut self, parser: &Parser) -> ClapResult<()> {
        if let Some(h) = parser.meta.help_str {
            try!(writeln!(self.writer, "{}", h).map_err(Error::from));
        } else if let Some(ref tmpl) = parser.meta.template {
            try!(self.write_templated_help(&parser, tmpl));
        } else {
            try!(self.write_default_help(&parser));
        }
        Ok(())
    }
}

// Methods to write AnyArg help.
impl<'a> Help<'a> {
    /// Writes help for each argument in the order they were declared to the wrapped stream.
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

    /// Sorts arguments by length and display order and write their help to the wrapped stream.
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

    /// Writes help for an argument to the wrapped stream.
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

    /// Writes argument's short command to the wrapped stream.
    fn short<'b, 'c>(&mut self, arg: &ArgWithDisplay<'b, 'c>) -> io::Result<()> {
        debugln!("fn=short;");
        try!(write!(self.writer, "{}", TAB));
        if let Some(s) = arg.short() {
            color!(self, "-{}", s, Good)
        } else if arg.has_switch() {
            write!(self.writer, "{}", TAB)
        } else {
            Ok(())
        }
    }

    /// Writes argument's long command to the wrapped stream.
    fn long<'b, 'c>(&mut self, arg: &ArgWithDisplay<'b, 'c>, longest: usize) -> io::Result<()> {
        debugln!("fn=long;");
        if !arg.has_switch() {
            return Ok(());
        }
        if arg.takes_value() {
            if let Some(l) = arg.long() {
                if arg.short().is_some() {
                    try!(write!(self.writer, ", "));
                }
                try!(color!(self, "--{}", l, Good))
            }
            try!(write!(self.writer, " "));
        } else {
            if let Some(l) = arg.long() {
                if arg.short().is_some() {
                    try!(write!(self.writer, ", "));
                }
                try!(color!(self, "--{}", l, Good));
                if !self.next_line_help || !arg.is_set(ArgSettings::NextLineHelp) {
                    write_nspaces!(self.writer, (longest + 4) - (l.len() + 2));
                }
            } else {
                if !self.next_line_help || !arg.is_set(ArgSettings::NextLineHelp) {
                    // 6 is tab (4) + -- (2)
                    write_nspaces!(self.writer, (longest + 6));
                }
            }
        }
        Ok(())
    }

    /// Writes argument's possible values to the wrapped stream.
    fn val<'b, 'c>(&mut self, arg: &ArgWithDisplay<'b, 'c>, longest: usize) -> io::Result<()> {
        debugln!("fn=val;");
        if !arg.takes_value() {
            return Ok(());
        }
        if let Some(ref vec) = arg.val_names() {
            let mut it = vec.iter().peekable();
            while let Some((_, val)) = it.next() {
                try!(color!(self, "<{}>", val, Good));
                if it.peek().is_some() {
                    try!(write!(self.writer, " "));
                }
            }
            let num = vec.len();
            if arg.is_set(ArgSettings::Multiple) && num == 1 {
                try!(color!(self, "...", Good));
            }
        } else if let Some(num) = arg.num_vals() {
            let mut it = (0..num).peekable();
            while let Some(_) = it.next() {
                try!(color!(self, "<{}>", arg.name(), Good));
                if it.peek().is_some() {
                    try!(write!(self.writer, " "));
                }
            }
        } else if arg.has_switch() {
            try!(color!(self, "<{}>", arg.name(), Good));
        } else {
            try!(color!(self, "{}", arg, Good));
        }
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

                write_nspaces!(self.writer, spcs);
            }
        } else if !(self.next_line_help || arg.is_set(ArgSettings::NextLineHelp)) {
            write_nspaces!(self.writer, longest + 4 - (arg.to_string().len()));
        }
        Ok(())
    }

    /// Writes argument's help to the wrapped stream.
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
        let too_long = self.term_w.is_some() &&
                       (spcs + str_width(h) + str_width(&*spec_vals) >= width);
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
            debugln!("help width: {}", str_width(&*help));
            // Determine how many newlines we need to insert
            let avail_chars = width - spcs;
            debugln!("Usable space: {}", avail_chars);
            let longest_w = {
                let mut lw = 0;
                for l in help.split(' ').map(|s| str_width(s)) {
                    if l > lw {
                        lw = l;
                    }
                }
                lw
            };
            debugln!("Longest word...{}", longest_w);
            debug!("Enough space to wrap...");
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
                    if str_width(&help[idx..]) <= avail_chars {
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
        } else if spec_vals.is_empty() {
            h
        } else {
            help.push_str(h);
            help.push_str(&*spec_vals);
            &*help
        };
        if help.contains("{n}") {
            if let Some(part) = help.split("{n}").next() {
                try!(write!(self.writer, "{}", part));
            }
            for part in help.split("{n}").skip(1) {
                try!(write!(self.writer, "\n"));
                if self.next_line_help || arg.is_set(ArgSettings::NextLineHelp) {
                    try!(write!(self.writer, "{}{}", TAB, TAB));
                } else if arg.has_switch() {
                    write_nspaces!(self.writer, longest + 12);
                } else {
                    write_nspaces!(self.writer, longest + 8);
                }
                try!(write!(self.writer, "{}", part));
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
                           if self.color {
                               format!("{}", Format::Good(pv))
                           } else {
                               pv.to_string()
                           },
                           if self.hide_pv {
                               "".into()
                           } else {
                               if let Some(ref pv) = a.possible_vals() {
                                   if self.color {
                                       format!(" [values: {}]",
                                               pv.iter()
                                                 .map(|v| format!("{}", Format::Good(v)))
                                                 .collect::<Vec<_>>()
                                                 .join(", "))
                                   } else {
                                       format!(" [values: {}]", pv.join(", "))
                                   }
                               } else {
                                   "".into()
                               }
                           });
        } else if !self.hide_pv {
            debugln!("Writing values");
            if let Some(ref pv) = a.possible_vals() {
                debugln!("Possible vals...{:?}", pv);
                return if self.color {
                    format!(" [values: {}]",
                            pv.iter()
                              .map(|v| format!("{}", Format::Good(v)))
                              .collect::<Vec<_>>()
                              .join(", "))
                } else {
                    format!(" [values: {}]", pv.join(", "))
                };
            }
        }
        String::new()
    }
}


// Methods to write Parser help.
impl<'a> Help<'a> {
    /// Writes help for all arguments (options, flags, args, subcommands)
    /// including titles of a Parser Object to the wrapped stream.
    pub fn write_all_args(&mut self, parser: &Parser) -> ClapResult<()> {

        let flags = parser.has_flags();
        let pos = parser.has_positionals();
        let opts = parser.has_opts();
        let subcmds = parser.has_subcommands();

        let unified_help = parser.is_set(AppSettings::UnifiedHelpMessage);

        let mut first = true;

        if unified_help && (flags || opts) {
            let opts_flags = parser.iter_flags()
                                   .map(as_arg_trait)
                                   .chain(parser.iter_opts().map(as_arg_trait));
            try!(color!(self, "OPTIONS:\n", Warning));
            try!(self.write_args(opts_flags));
            first = false;
        } else {
            if flags {
                try!(color!(self, "FLAGS:\n", Warning));
                try!(self.write_args(parser.iter_flags()
                                           .map(as_arg_trait)));
                first = false;
            }
            if opts {
                if !first {
                    try!(self.writer.write(b"\n"));
                }
                try!(color!(self, "OPTIONS:\n", Warning));
                try!(self.write_args(parser.iter_opts().map(as_arg_trait)));
                first = false;
            }
        }

        if pos {
            if !first {
                try!(self.writer.write(b"\n"));
            }
            try!(color!(self, "ARGS:\n", Warning));
            try!(self.write_args_unsorted(parser.iter_positionals().map(as_arg_trait)));
            first = false;
        }

        if subcmds {
            if !first {
                try!(self.writer.write(b"\n"));
            }
            try!(color!(self, "SUBCOMMANDS:\n", Warning));
            try!(self.write_subcommands(&parser));
        }

        Ok(())
    }

    /// Writes help for subcommands of a Parser Object to the wrapped stream.
    fn write_subcommands(&mut self, parser: &Parser) -> io::Result<()> {
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
        Ok(())
    }

    /// Writes version of a Parser Object to the wrapped stream.
    fn write_version(&mut self, parser: &Parser) -> io::Result<()> {
        try!(write!(self.writer, "{}", parser.meta.version.unwrap_or("".into())));
        Ok(())
    }

    /// Writes binary name of a Parser Object to the wrapped stream.
    fn write_bin_name(&mut self, parser: &Parser) -> io::Result<()> {
        if let Some(bn) = parser.meta.bin_name.as_ref() {
            if bn.contains(' ') {
                // Incase we're dealing with subcommands i.e. git mv is translated to git-mv
                try!(color!(self, bn.replace(" ", "-"), Good))
            } else {
                try!(color!(self, &parser.meta.name[..], Good))
            }
        } else {
            try!(color!(self, &parser.meta.name[..], Good))
        }
        Ok(())
    }

    /// Writes default help for a Parser Object to the wrapped stream.
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

        try!(color!(self, "\nUSAGE:", Warning));
        try!(write!(self.writer,
                    "\n{}{}\n\n",
                    TAB,
                    parser.create_usage_no_title(&[])));

        let flags = parser.has_flags();
        let pos = parser.has_positionals();
        let opts = parser.has_opts();
        let subcmds = parser.has_subcommands();

        if flags || opts || pos || subcmds {
            try!(self.write_all_args(&parser));
        }

        if let Some(h) = parser.meta.more_help {
            try!(write!(self.writer, "{}\n", h));
        }

        self.writer.flush().map_err(Error::from)
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
/// In addition to Errors, there are three possible outputs:
///   - None: The reader was consumed.
///   - Some(Ok(0)): No tag was captured but the reader still contains data.
///   - Some(Ok(length>0)): a tag with `length` was captured to the tag_buffer.
fn copy_and_capture<R: Read, W: Write>(r: &mut R,
                                       w: &mut W,
                                       tag_buffer: &mut Cursor<Vec<u8>>)
                                       -> Option<io::Result<usize>> {
    use self::CopyUntilResult::*;

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
impl<'a> Help<'a> {
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
    fn write_templated_help(&mut self, parser: &Parser, template: &str) -> ClapResult<()> {
        let mut tmplr = Cursor::new(&template);
        let mut tag_buf = Cursor::new(vec![0u8; 15]);

        // The strategy is to copy the template from the the reader to wrapped stream
        // until a tag is found. Depending on its value, the appropriate content is copied
        // to the wrapped stream.
        // The copy from template is then resumed, repeating this sequence until reading
        // the complete template.

        loop {
            let tag_length = match copy_and_capture(&mut tmplr, &mut self.writer, &mut tag_buf) {
                None => return Ok(()),
                Some(Err(e)) => return Err(Error::from(e)),
                Some(Ok(val)) if val > 0 => val,
                _ => continue,
            };

            match &tag_buf.get_ref()[0..tag_length] {
                b"?" => {
                    try!(self.writer.write(b"Could not decode tag name"));
                }
                b"bin" => {
                    try!(self.write_bin_name(&parser));
                }
                b"version" => {
                    try!(write!(self.writer,
                                "{}",
                                parser.meta.version.unwrap_or("unknown version")));
                }
                b"author" => {
                    try!(write!(self.writer,
                                "{}",
                                parser.meta.author.unwrap_or("unknown author")));
                }
                b"about" => {
                    try!(write!(self.writer,
                                "{}",
                                parser.meta.about.unwrap_or("unknown about")));
                }
                b"usage" => {
                    try!(write!(self.writer, "{}", parser.create_usage_no_title(&[])));
                }
                b"all-args" => {
                    try!(self.write_all_args(&parser));
                }
                b"unified" => {
                    let opts_flags = parser.iter_flags()
                                           .map(as_arg_trait)
                                           .chain(parser.iter_opts().map(as_arg_trait));
                    try!(self.write_args(opts_flags));
                }
                b"flags" => {
                    try!(self.write_args(parser.iter_flags()
                                               .map(as_arg_trait)));
                }
                b"options" => {
                    try!(self.write_args(parser.iter_opts()
                                               .map(as_arg_trait)));
                }
                b"positionals" => {
                    try!(self.write_args(parser.iter_positionals()
                                               .map(as_arg_trait)));
                }
                b"subcommands" => {
                    try!(self.write_subcommands(&parser));
                }
                b"after-help" => {
                    try!(write!(self.writer,
                                "{}",
                                parser.meta.more_help.unwrap_or("unknown after-help")));
                }
                b"before-help" => {
                    try!(write!(self.writer,
                                "{}",
                                parser.meta.pre_help.unwrap_or("unknown before-help")));
                }
                // Unknown tag, write it back.
                ref r => {
                    try!(self.writer.write(b"{"));
                    try!(self.writer.write(r));
                    try!(self.writer.write(b"}"));
                }
            }

        }
    }
}


fn find_idx_of_space(full: &str, mut start: usize) -> usize {
    debugln!("fn=find_idx_of_space;");
    let haystack = if full._is_char_boundary(start) {
        &full[..start]
    } else {
        while !full._is_char_boundary(start) {
            start -= 1;
        }
        &full[..start]
    };
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
