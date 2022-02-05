//! Error reporting

// Std
use std::{
    borrow::Cow,
    convert::From,
    error,
    fmt::{self, Debug, Display, Formatter},
    io::{self, BufRead},
    result::Result as StdResult,
};

// Internal
use crate::{
    build::Arg,
    output::fmt::{Colorizer, StyleSpec},
    parse::features::suggestions,
    util::{color::ColorChoice, safe_exit, SUCCESS_CODE, USAGE_CODE},
    App, AppSettings,
};

mod kind;

pub use kind::ErrorKind;

/// Short hand for [`Result`] type
///
/// [`Result`]: std::result::Result
pub type Result<T, E = Error> = StdResult<T, E>;

/// Command Line Argument Parser Error
///
/// See [`App::error`] to create an error.
///
/// [`App::error`]: crate::App::error
#[derive(Debug)]
pub struct Error {
    inner: Box<ErrorInner>,
    /// The type of error
    pub kind: ErrorKind,
    /// Additional information depending on the error kind, like values and argument names.
    /// Useful when you want to render an error of your own.
    pub info: Vec<String>,
}

#[derive(Debug)]
struct ErrorInner {
    /// The type of error
    kind: ErrorKind,
    /// Formatted error message, enhancing the cause message with extra information
    message: Message,
    source: Option<Box<dyn error::Error + Send + Sync>>,
    wait_on_exit: bool,
    backtrace: Option<Backtrace>,
}

impl Error {
    /// Create an unformatted error
    ///
    /// This is for you need to pass the error up to
    /// a place that has access to the `App` at which point you can call [`Error::format`].
    ///
    /// Prefer [`App::error`] for generating errors.
    ///
    /// [`App::error`]: crate::App::error
    pub fn raw(kind: ErrorKind, message: impl std::fmt::Display) -> Self {
        Self::new(message.to_string(), kind, false)
    }

    /// Format the existing message with the App's context
    #[must_use]
    pub fn format(mut self, app: &mut App) -> Self {
        app._build();
        let usage = app.render_usage();
        self.inner.message.format(app, usage);
        self.inner.wait_on_exit = app.settings.is_set(AppSettings::WaitOnError);
        self
    }

    /// Type of error for programmatic processing
    pub fn kind(&self) -> ErrorKind {
        self.inner.kind
    }

    /// Should the message be written to `stdout` or not?
    #[inline]
    pub fn use_stderr(&self) -> bool {
        !matches!(
            self.kind(),
            ErrorKind::DisplayHelp | ErrorKind::DisplayVersion
        )
    }

    /// Prints the error and exits.
    ///
    /// Depending on the error kind, this either prints to `stderr` and exits with a status of `2`
    /// or prints to `stdout` and exits with a status of `0`.
    pub fn exit(&self) -> ! {
        if self.use_stderr() {
            // Swallow broken pipe errors
            let _ = self.print();

            if self.inner.wait_on_exit {
                wlnerr!("\nPress [ENTER] / [RETURN] to continue...");
                let mut s = String::new();
                let i = io::stdin();
                i.lock().read_line(&mut s).unwrap();
            }

            safe_exit(USAGE_CODE);
        }

        // Swallow broken pipe errors
        let _ = self.print();
        safe_exit(SUCCESS_CODE)
    }

    /// Prints formatted and colored error to `stdout` or `stderr` according to its error kind
    ///
    /// # Example
    /// ```no_run
    /// use clap::App;
    ///
    /// match App::new("App").try_get_matches() {
    ///     Ok(matches) => {
    ///         // do_something
    ///     },
    ///     Err(err) => {
    ///         err.print().expect("Error writing Error");
    ///         // do_something
    ///     },
    /// };
    /// ```
    pub fn print(&self) -> io::Result<()> {
        self.inner.message.formatted().print()
    }

    /// Deprecated, replaced with [`App::error`]
    ///
    /// [`App::error`]: crate::App::error
    #[deprecated(since = "3.0.0", note = "Replaced with `App::error`")]
    pub fn with_description(description: String, kind: ErrorKind) -> Self {
        Error::raw(kind, description)
    }

    pub(crate) fn new(message: impl Into<Message>, kind: ErrorKind, wait_on_exit: bool) -> Self {
        Self {
            inner: Box::new(ErrorInner {
                kind,
                message: message.into(),
                source: None,
                wait_on_exit,
                backtrace: Backtrace::new(),
            }),
            kind,
            info: vec![],
        }
    }

    #[inline(never)]
    pub(crate) fn for_app(
        app: &App,
        colorizer: Colorizer,
        kind: ErrorKind,
        info: Vec<String>,
    ) -> Self {
        Self::new(
            colorizer,
            kind,
            app.settings.is_set(AppSettings::WaitOnError),
        )
        .set_info(info)
    }

    pub(crate) fn set_info(mut self, info: Vec<String>) -> Self {
        self.info = info;
        self
    }

    pub(crate) fn set_source(mut self, source: Box<dyn error::Error + Send + Sync>) -> Self {
        self.inner.source = Some(source);
        self
    }

    pub(crate) fn argument_conflict(
        app: &App,
        arg: &Arg,
        others: Vec<String>,
        usage: String,
    ) -> Self {
        let mut c = Colorizer::new(true, app.get_color(), app.get_style_spec());
        let arg = arg.to_string();

        start_error(&mut c, "The argument '");
        c.warning(arg);
        c.none("' cannot be used with");

        match others.len() {
            0 => {
                c.none(" one or more of the other specified arguments");
            }
            1 => {
                c.none(" '");
                c.warning(&*others[0]);
                c.none("'");
            }
            _ => {
                c.none(":");
                for v in &others {
                    c.none("\n    ");
                    c.warning(&**v);
                }
            }
        }

        put_usage(&mut c, usage);
        try_help(app, &mut c);

        Self::for_app(app, c, ErrorKind::ArgumentConflict, others)
    }

    pub(crate) fn empty_value(app: &App, good_vals: &[&str], arg: &Arg, usage: String) -> Self {
        let mut c = Colorizer::new(true, app.get_color(), app.get_style_spec());
        let arg = arg.to_string();

        start_error(&mut c, "The argument '");
        c.warning(&*arg);
        c.none("' requires a value but none was supplied");
        if !good_vals.is_empty() {
            let good_vals: Vec<String> = good_vals
                .iter()
                .map(|&v| {
                    if v.contains(char::is_whitespace) {
                        format!("{:?}", v)
                    } else {
                        v.to_owned()
                    }
                })
                .collect();
            c.none("\n\t[possible values: ");

            if let Some((last, elements)) = good_vals.split_last() {
                for v in elements {
                    c.good(v);
                    c.none(", ");
                }

                c.good(last);
            }

            c.none("]");
        }
        put_usage(&mut c, usage);
        try_help(app, &mut c);

        Self::for_app(app, c, ErrorKind::EmptyValue, vec![arg])
    }

    pub(crate) fn no_equals(app: &App, arg: String, usage: String) -> Self {
        let mut c = Colorizer::new(true, app.get_color(), app.get_style_spec());

        start_error(&mut c, "Equal sign is needed when assigning values to '");
        c.warning(&*arg);
        c.none("'.");

        put_usage(&mut c, usage);
        try_help(app, &mut c);

        Self::for_app(app, c, ErrorKind::NoEquals, vec![arg])
    }

    pub(crate) fn invalid_value(
        app: &App,
        bad_val: String,
        good_vals: &[&str],
        arg: &Arg,
        usage: String,
    ) -> Self {
        let mut c = Colorizer::new(true, app.get_color(), app.get_style_spec());
        let suffix = suggestions::did_you_mean(&bad_val, good_vals.iter()).pop();
        let arg = arg.to_string();

        let good_vals: Vec<String> = good_vals
            .iter()
            .map(|&v| {
                if v.contains(char::is_whitespace) {
                    format!("{:?}", v)
                } else {
                    v.to_owned()
                }
            })
            .collect();

        start_error(&mut c, "");
        c.warning(format!("{:?}", bad_val));
        c.none(" isn't a valid value for '");
        c.warning(&*arg);
        c.none("'\n\t[possible values: ");

        if let Some((last, elements)) = good_vals.split_last() {
            for v in elements {
                c.good(v);
                c.none(", ");
            }

            c.good(last);
        }

        c.none("]");

        if let Some(val) = suffix {
            c.none("\n\n\tDid you mean ");
            c.good(format!("{:?}", val));
            c.none("?");
        }

        put_usage(&mut c, usage);
        try_help(app, &mut c);

        let mut info = vec![arg, bad_val];
        info.extend(good_vals);

        Self::for_app(app, c, ErrorKind::InvalidValue, info)
    }

    pub(crate) fn invalid_subcommand(
        app: &App,
        subcmd: String,
        did_you_mean: String,
        name: String,
        usage: String,
    ) -> Self {
        let mut c = Colorizer::new(true, app.get_color(), app.get_style_spec());

        start_error(&mut c, "The subcommand '");
        c.warning(&*subcmd);
        c.none("' wasn't recognized\n\n\tDid you mean ");
        c.good(did_you_mean);
        c.none("");
        c.none(format!(
            "?\n\nIf you believe you received this message in error, try re-running with '{} ",
            name
        ));
        c.good("--");
        c.none(format!(" {}'", subcmd));
        put_usage(&mut c, usage);
        try_help(app, &mut c);

        Self::for_app(app, c, ErrorKind::InvalidSubcommand, vec![subcmd])
    }

    pub(crate) fn unrecognized_subcommand(app: &App, subcmd: String, name: String) -> Self {
        let mut c = Colorizer::new(true, app.get_color(), app.get_style_spec());

        start_error(&mut c, " The subcommand '");
        c.warning(&*subcmd);
        c.none("' wasn't recognized\n\n");
        c.warning("USAGE:");
        c.none(format!("\n    {} <subcommands>", name));
        try_help(app, &mut c);

        Self::for_app(app, c, ErrorKind::UnrecognizedSubcommand, vec![subcmd])
    }

    pub(crate) fn missing_required_argument(
        app: &App,
        required: Vec<String>,
        usage: String,
    ) -> Self {
        let mut c = Colorizer::new(true, app.get_color(), app.get_style_spec());

        start_error(
            &mut c,
            "The following required arguments were not provided:",
        );

        for v in &required {
            c.none("\n    ");
            c.good(&**v);
        }

        put_usage(&mut c, usage);
        try_help(app, &mut c);

        Self::for_app(app, c, ErrorKind::MissingRequiredArgument, required)
    }

    pub(crate) fn missing_subcommand(app: &App, name: String, usage: String) -> Self {
        let mut c = Colorizer::new(true, app.get_color(), app.get_style_spec());

        start_error(&mut c, "'");
        c.warning(name);
        c.none("' requires a subcommand, but one was not provided");
        put_usage(&mut c, usage);
        try_help(app, &mut c);

        Self::for_app(app, c, ErrorKind::MissingSubcommand, vec![])
    }

    pub(crate) fn invalid_utf8(app: &App, usage: String) -> Self {
        let mut c = Colorizer::new(true, app.get_color(), app.get_style_spec());

        start_error(
            &mut c,
            "Invalid UTF-8 was detected in one or more arguments",
        );
        put_usage(&mut c, usage);
        try_help(app, &mut c);

        Self::for_app(app, c, ErrorKind::InvalidUtf8, vec![])
    }

    pub(crate) fn too_many_occurrences(
        app: &App,
        arg: &Arg,
        max_occurs: usize,
        curr_occurs: usize,
        usage: String,
    ) -> Self {
        let mut c = Colorizer::new(true, app.get_color(), app.get_style_spec());
        let were_provided = Error::singular_or_plural(curr_occurs);
        let arg = arg.to_string();
        let max_occurs = max_occurs.to_string();
        let curr_occurs = curr_occurs.to_string();

        start_error(&mut c, "The argument '");
        c.warning(&*arg);
        c.none("' allows at most ");
        c.warning(&*max_occurs);
        c.none(" occurrences, but ");
        c.warning(&*curr_occurs);
        c.none(were_provided);
        put_usage(&mut c, usage);
        try_help(app, &mut c);

        Self::for_app(
            app,
            c,
            ErrorKind::TooManyOccurrences,
            vec![arg, curr_occurs, max_occurs],
        )
    }

    pub(crate) fn too_many_values(app: &App, val: String, arg: String, usage: String) -> Self {
        let mut c = Colorizer::new(true, app.get_color(), app.get_style_spec());

        start_error(&mut c, "The value '");
        c.warning(&*val);
        c.none("' was provided to '");
        c.warning(&*arg);
        c.none("' but it wasn't expecting any more values");
        put_usage(&mut c, usage);
        try_help(app, &mut c);

        Self::for_app(app, c, ErrorKind::TooManyValues, vec![arg, val])
    }

    pub(crate) fn too_few_values(
        app: &App,
        arg: &Arg,
        min_vals: usize,
        curr_vals: usize,
        usage: String,
    ) -> Self {
        let mut c = Colorizer::new(true, app.get_color(), app.get_style_spec());
        let were_provided = Error::singular_or_plural(curr_vals);
        let arg = arg.to_string();
        let min_vals = min_vals.to_string();
        let curr_vals = curr_vals.to_string();

        start_error(&mut c, "The argument '");
        c.warning(&*arg);
        c.none("' requires at least ");
        c.warning(&*min_vals);
        c.none(" values, but only ");
        c.warning(&*curr_vals);
        c.none(were_provided);
        put_usage(&mut c, usage);
        try_help(app, &mut c);

        Self::for_app(
            app,
            c,
            ErrorKind::TooFewValues,
            vec![arg, curr_vals, min_vals],
        )
    }

    pub(crate) fn value_validation(
        app: &App,
        arg: String,
        val: String,
        err: Box<dyn error::Error + Send + Sync>,
    ) -> Self {
        let mut err = Self::value_validation_with_color(
            arg,
            val,
            err,
            app.get_color(),
            app.get_style_spec(),
            app.settings.is_set(AppSettings::WaitOnError),
        );
        match &mut err.inner.message {
            Message::Raw(_) => {
                unreachable!("`value_validation_with_color` only deals in formatted errors")
            }
            Message::Formatted(c) => try_help(app, c),
        }
        err
    }

    pub(crate) fn value_validation_without_app(
        arg: String,
        val: String,
        err: Box<dyn error::Error + Send + Sync>,
    ) -> Self {
        let mut err = Self::value_validation_with_color(
            arg,
            val,
            err,
            ColorChoice::Never,
            StyleSpec::empty(),
            false,
        );
        match &mut err.inner.message {
            Message::Raw(_) => {
                unreachable!("`value_validation_with_color` only deals in formatted errors")
            }
            Message::Formatted(c) => {
                c.none("\n");
            }
        }
        err
    }

    fn value_validation_with_color(
        arg: String,
        val: String,
        err: Box<dyn error::Error + Send + Sync>,
        color: ColorChoice,
        style_spec: StyleSpec,
        wait_on_exit: bool,
    ) -> Self {
        let mut c = Colorizer::new(true, color, style_spec);

        start_error(&mut c, "Invalid value");

        c.none(" for '");
        c.warning(&*arg);
        c.none("'");

        c.none(format!(": {}", err));

        Self::new(c, ErrorKind::ValueValidation, wait_on_exit)
            .set_info(vec![arg, val, err.to_string()])
            .set_source(err)
    }

    pub(crate) fn wrong_number_of_values(
        app: &App,
        arg: &Arg,
        num_vals: usize,
        curr_vals: usize,
        usage: String,
    ) -> Self {
        let mut c = Colorizer::new(true, app.get_color(), app.get_style_spec());
        let were_provided = Error::singular_or_plural(curr_vals);
        let arg = arg.to_string();
        let num_vals = num_vals.to_string();
        let curr_vals = curr_vals.to_string();

        start_error(&mut c, "The argument '");
        c.warning(&*arg);
        c.none("' requires ");
        c.warning(&*num_vals);
        c.none(" values, but ");
        c.warning(&*curr_vals);
        c.none(were_provided);
        put_usage(&mut c, usage);
        try_help(app, &mut c);

        Self::for_app(
            app,
            c,
            ErrorKind::WrongNumberOfValues,
            vec![arg, curr_vals, num_vals],
        )
    }

    pub(crate) fn unexpected_multiple_usage(app: &App, arg: &Arg, usage: String) -> Self {
        let mut c = Colorizer::new(true, app.get_color(), app.get_style_spec());
        let arg = arg.to_string();

        start_error(&mut c, "The argument '");
        c.warning(&*arg);
        c.none("' was provided more than once, but cannot be used multiple times");
        put_usage(&mut c, usage);
        try_help(app, &mut c);

        Self::for_app(app, c, ErrorKind::UnexpectedMultipleUsage, vec![arg])
    }

    pub(crate) fn unknown_argument(
        app: &App,
        arg: String,
        did_you_mean: Option<(String, Option<String>)>,
        usage: String,
    ) -> Self {
        let mut c = Colorizer::new(true, app.get_color(), app.get_style_spec());

        start_error(&mut c, "Found argument '");
        c.warning(&*arg);
        c.none("' which wasn't expected, or isn't valid in this context");

        if let Some((flag, subcmd)) = did_you_mean {
            let flag = format!("--{}", flag);
            c.none("\n\n\tDid you mean ");

            if let Some(subcmd) = subcmd {
                c.none("to put '");
                c.good(flag);
                c.none("' after the subcommand '");
                c.good(subcmd);
                c.none("'?");
            } else {
                c.none("'");
                c.good(flag);
                c.none("'?");
            }
        }

        // If the user wants to supply things like `--a-flag` or `-b` as a value,
        // suggest `--` for disambiguation.
        if arg.starts_with('-') {
            c.none(format!(
                "\n\n\tIf you tried to supply `{}` as a value rather than a flag, use `-- {}`",
                arg, arg
            ));
        }

        put_usage(&mut c, usage);
        try_help(app, &mut c);

        Self::for_app(app, c, ErrorKind::UnknownArgument, vec![arg])
    }

    pub(crate) fn unnecessary_double_dash(app: &App, arg: String, usage: String) -> Self {
        let mut c = Colorizer::new(true, app.get_color(), app.get_style_spec());

        start_error(&mut c, "Found argument '");
        c.warning(&*arg);
        c.none("' which wasn't expected, or isn't valid in this context");

        c.none(format!(
            "\n\n\tIf you tried to supply `{}` as a subcommand, remove the '--' before it.",
            arg
        ));
        put_usage(&mut c, usage);
        try_help(app, &mut c);

        Self::for_app(app, c, ErrorKind::UnknownArgument, vec![arg])
    }

    pub(crate) fn argument_not_found_auto(arg: String) -> Self {
        let mut c = Colorizer::new(true, ColorChoice::Never, StyleSpec::empty());

        start_error(&mut c, "The argument '");
        c.warning(&*arg);
        c.none("' wasn't found\n");

        Self::new(c, ErrorKind::ArgumentNotFound, false).set_info(vec![arg])
    }

    /// Returns the singular or plural form on the verb to be based on the argument's value.
    fn singular_or_plural(n: usize) -> &'static str {
        if n > 1 {
            " were provided"
        } else {
            " was provided"
        }
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Error::raw(ErrorKind::Io, e)
    }
}

impl From<fmt::Error> for Error {
    fn from(e: fmt::Error) -> Self {
        Error::raw(ErrorKind::Format, e)
    }
}

impl error::Error for Error {
    #[allow(trivial_casts)]
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        self.inner.source.as_ref().map(|e| e.as_ref() as _)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        // Assuming `self.message` already has a trailing newline, from `try_help` or similar
        write!(f, "{}", self.inner.message.formatted())?;
        if let Some(backtrace) = self.inner.backtrace.as_ref() {
            writeln!(f)?;
            writeln!(f, "Backtrace:")?;
            writeln!(f, "{}", backtrace)?;
        }
        Ok(())
    }
}

fn start_error(c: &mut Colorizer, msg: impl Into<String>) {
    c.error("error:");
    c.none(" ");
    c.none(msg);
}

fn put_usage(c: &mut Colorizer, usage: impl Into<String>) {
    c.none("\n\n");
    c.none(usage);
}

fn try_help(app: &App, c: &mut Colorizer) {
    if !app.settings.is_set(AppSettings::DisableHelpFlag) {
        c.none("\n\nFor more information try ");
        c.good("--help");
        c.none("\n");
    } else if app.has_subcommands() && !app.settings.is_set(AppSettings::DisableHelpSubcommand) {
        c.none("\n\nFor more information try ");
        c.good("help");
        c.none("\n");
    } else {
        c.none("\n");
    }
}

#[derive(Clone, Debug)]
pub(crate) enum Message {
    Raw(String),
    Formatted(Colorizer),
}

impl Message {
    fn format(&mut self, app: &App, usage: String) {
        match self {
            Message::Raw(s) => {
                let mut c = Colorizer::new(true, app.get_color(), app.get_style_spec());

                let mut message = String::new();
                std::mem::swap(s, &mut message);
                start_error(&mut c, message);
                put_usage(&mut c, usage);
                try_help(app, &mut c);
                *self = Self::Formatted(c);
            }
            Message::Formatted(_) => {}
        }
    }

    fn formatted(&self) -> Cow<Colorizer> {
        match self {
            Message::Raw(s) => {
                let mut c = Colorizer::new(true, ColorChoice::Never, StyleSpec::empty());
                start_error(&mut c, s);
                Cow::Owned(c)
            }
            Message::Formatted(c) => Cow::Borrowed(c),
        }
    }
}

impl From<String> for Message {
    fn from(inner: String) -> Self {
        Self::Raw(inner)
    }
}

impl From<Colorizer> for Message {
    fn from(inner: Colorizer) -> Self {
        Self::Formatted(inner)
    }
}

#[cfg(feature = "debug")]
#[derive(Debug)]
struct Backtrace(backtrace::Backtrace);

#[cfg(feature = "debug")]
impl Backtrace {
    fn new() -> Option<Self> {
        Some(Self(backtrace::Backtrace::new()))
    }
}

#[cfg(feature = "debug")]
impl Display for Backtrace {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        // `backtrace::Backtrace` uses `Debug` instead of `Display`
        write!(f, "{:?}", self.0)
    }
}

#[cfg(not(feature = "debug"))]
#[derive(Debug)]
struct Backtrace;

#[cfg(not(feature = "debug"))]
impl Backtrace {
    fn new() -> Option<Self> {
        None
    }
}

#[cfg(not(feature = "debug"))]
impl Display for Backtrace {
    fn fmt(&self, _: &mut Formatter) -> fmt::Result {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    /// Check `clap::Error` impls Send and Sync.
    mod clap_error_impl_send_sync {
        use crate::Error;
        trait Foo: std::error::Error + Send + Sync + 'static {}
        impl Foo for Error {}
    }
}
