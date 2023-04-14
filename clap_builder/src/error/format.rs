#![allow(missing_copy_implementations)]
#![allow(missing_debug_implementations)]
#![cfg_attr(not(feature = "error-context"), allow(dead_code))]
#![cfg_attr(not(feature = "error-context"), allow(unused_imports))]

use crate::builder::Command;
use crate::builder::Style;
use crate::builder::StyledStr;
#[cfg(feature = "error-context")]
use crate::error::ContextKind;
#[cfg(feature = "error-context")]
use crate::error::ContextValue;
use crate::error::ErrorKind;
use crate::output::TAB;

/// Defines how to format an error for displaying to the user
pub trait ErrorFormatter: Sized {
    /// Stylize the error for the terminal
    fn format_error(error: &crate::error::Error<Self>) -> StyledStr;
}

/// Report [`ErrorKind`]
///
/// No context is included.
///
/// **NOTE:** Consider removing the `error-context` default feature if using this to remove all
/// overhead for [`RichFormatter`].
#[non_exhaustive]
pub struct KindFormatter;

impl ErrorFormatter for KindFormatter {
    fn format_error(error: &crate::error::Error<Self>) -> StyledStr {
        use std::fmt::Write as _;

        let mut styled = StyledStr::new();
        start_error(&mut styled);
        if let Some(msg) = error.kind().as_str() {
            styled.push_str(msg);
        } else if let Some(source) = error.inner.source.as_ref() {
            let _ = write!(styled, "{}", source);
        } else {
            styled.push_str("unknown cause");
        }
        styled.push_str("\n");
        styled
    }
}

/// Richly formatted error context
///
/// This follows the [rustc diagnostic style guide](https://rustc-dev-guide.rust-lang.org/diagnostics.html#suggestion-style-guide).
#[non_exhaustive]
#[cfg(feature = "error-context")]
pub struct RichFormatter;

#[cfg(feature = "error-context")]
impl ErrorFormatter for RichFormatter {
    fn format_error(error: &crate::error::Error<Self>) -> StyledStr {
        use std::fmt::Write as _;
        let good = Style::Good.as_style();

        let mut styled = StyledStr::new();
        start_error(&mut styled);

        if !write_dynamic_context(error, &mut styled) {
            if let Some(msg) = error.kind().as_str() {
                styled.push_str(msg);
            } else if let Some(source) = error.inner.source.as_ref() {
                let _ = write!(styled, "{}", source);
            } else {
                styled.push_str("unknown cause");
            }
        }

        let mut suggested = false;
        if let Some(valid) = error.get(ContextKind::SuggestedSubcommand) {
            styled.push_str("\n");
            if !suggested {
                styled.push_str("\n");
                suggested = true;
            }
            did_you_mean(&mut styled, "subcommand", valid);
        }
        if let Some(valid) = error.get(ContextKind::SuggestedArg) {
            styled.push_str("\n");
            if !suggested {
                styled.push_str("\n");
                suggested = true;
            }
            did_you_mean(&mut styled, "argument", valid);
        }
        if let Some(valid) = error.get(ContextKind::SuggestedValue) {
            styled.push_str("\n");
            if !suggested {
                styled.push_str("\n");
                suggested = true;
            }
            did_you_mean(&mut styled, "value", valid);
        }
        let suggestions = error.get(ContextKind::Suggested);
        if let Some(ContextValue::StyledStrs(suggestions)) = suggestions {
            if !suggested {
                styled.push_str("\n");
            }
            for suggestion in suggestions {
                let _ = write!(
                    styled,
                    "\n{TAB}{}tip:{} ",
                    good.render(),
                    good.render_reset()
                );
                styled.push_styled(suggestion);
            }
        }

        let usage = error.get(ContextKind::Usage);
        if let Some(ContextValue::StyledStr(usage)) = usage {
            put_usage(&mut styled, usage);
        }

        try_help(&mut styled, error.inner.help_flag);

        styled
    }
}

fn start_error(styled: &mut StyledStr) {
    use std::fmt::Write as _;
    let error = Style::Error.as_style();
    let _ = write!(styled, "{}error:{} ", error.render(), error.render_reset());
}

#[must_use]
#[cfg(feature = "error-context")]
fn write_dynamic_context(error: &crate::error::Error, styled: &mut StyledStr) -> bool {
    use std::fmt::Write as _;
    let good = Style::Good.as_style();
    let warning = Style::Warning.as_style();
    let literal = Style::Literal.as_style();

    match error.kind() {
        ErrorKind::ArgumentConflict => {
            let invalid_arg = error.get(ContextKind::InvalidArg);
            let prior_arg = error.get(ContextKind::PriorArg);
            if let (Some(ContextValue::String(invalid_arg)), Some(prior_arg)) =
                (invalid_arg, prior_arg)
            {
                if ContextValue::String(invalid_arg.clone()) == *prior_arg {
                    let _ = write!(
                        styled,
                        "the argument '{}{invalid_arg}{}' cannot be used multiple times",
                        warning.render(),
                        warning.render_reset()
                    );
                } else {
                    let _ = write!(
                        styled,
                        "the argument '{}{invalid_arg}{}' cannot be used with",
                        warning.render(),
                        warning.render_reset()
                    );

                    match prior_arg {
                        ContextValue::Strings(values) => {
                            styled.push_str(":");
                            for v in values {
                                let _ = write!(
                                    styled,
                                    "\n{TAB}{}{v}{}",
                                    warning.render(),
                                    warning.render_reset()
                                );
                            }
                        }
                        ContextValue::String(value) => {
                            let _ = write!(
                                styled,
                                " '{}{value}{}'",
                                warning.render(),
                                warning.render_reset()
                            );
                        }
                        _ => {
                            styled.push_str(" one or more of the other specified arguments");
                        }
                    }
                }
                true
            } else {
                false
            }
        }
        ErrorKind::NoEquals => {
            let invalid_arg = error.get(ContextKind::InvalidArg);
            if let Some(ContextValue::String(invalid_arg)) = invalid_arg {
                let _ = write!(
                    styled,
                    "equal sign is needed when assigning values to '{}{invalid_arg}{}'",
                    warning.render(),
                    warning.render_reset()
                );
                true
            } else {
                false
            }
        }
        ErrorKind::InvalidValue => {
            let invalid_arg = error.get(ContextKind::InvalidArg);
            let invalid_value = error.get(ContextKind::InvalidValue);
            if let (
                Some(ContextValue::String(invalid_arg)),
                Some(ContextValue::String(invalid_value)),
            ) = (invalid_arg, invalid_value)
            {
                if invalid_value.is_empty() {
                    let _ = write!(
                        styled,
                        "a value is required for '{}{invalid_arg}{}' but none was supplied",
                        warning.render(),
                        warning.render_reset()
                    );
                } else {
                    let _ = write!(
                        styled,
                        "invalid value '{}{invalid_value}{}' for '{}{invalid_arg}{}'",
                        warning.render(),
                        warning.render_reset(),
                        literal.render(),
                        literal.render_reset()
                    );
                }

                let possible_values = error.get(ContextKind::ValidValue);
                if let Some(ContextValue::Strings(possible_values)) = possible_values {
                    if !possible_values.is_empty() {
                        let _ = write!(styled, "\n{TAB}[possible values: ");
                        if let Some((last, elements)) = possible_values.split_last() {
                            for v in elements {
                                let _ = write!(
                                    styled,
                                    "{}{}{}, ",
                                    good.render(),
                                    Escape(v),
                                    good.render_reset()
                                );
                            }
                            let _ = write!(
                                styled,
                                "{}{}{}",
                                good.render(),
                                Escape(last),
                                good.render_reset()
                            );
                        }
                        styled.push_str("]");
                    }
                }
                true
            } else {
                false
            }
        }
        ErrorKind::InvalidSubcommand => {
            let invalid_sub = error.get(ContextKind::InvalidSubcommand);
            if let Some(ContextValue::String(invalid_sub)) = invalid_sub {
                let _ = write!(
                    styled,
                    "unrecognized subcommand '{}{invalid_sub}{}'",
                    warning.render(),
                    warning.render_reset()
                );
                true
            } else {
                false
            }
        }
        ErrorKind::MissingRequiredArgument => {
            let invalid_arg = error.get(ContextKind::InvalidArg);
            if let Some(ContextValue::Strings(invalid_arg)) = invalid_arg {
                styled.push_str("the following required arguments were not provided:");
                for v in invalid_arg {
                    let _ = write!(styled, "\n{TAB}{}{v}{}", good.render(), good.render_reset());
                }
                true
            } else {
                false
            }
        }
        ErrorKind::MissingSubcommand => {
            let invalid_sub = error.get(ContextKind::InvalidSubcommand);
            if let Some(ContextValue::String(invalid_sub)) = invalid_sub {
                let _ = write!(
                    styled,
                    "'{}{invalid_sub}{}' requires a subcommand but one was not provided",
                    warning.render(),
                    warning.render_reset()
                );

                let possible_values = error.get(ContextKind::ValidSubcommand);
                if let Some(ContextValue::Strings(possible_values)) = possible_values {
                    if !possible_values.is_empty() {
                        let _ = write!(styled, "\n{TAB}[subcommands: ");
                        if let Some((last, elements)) = possible_values.split_last() {
                            for v in elements {
                                let _ = write!(
                                    styled,
                                    "{}{}{}, ",
                                    good.render(),
                                    Escape(v),
                                    good.render_reset()
                                );
                            }
                            let _ = write!(
                                styled,
                                "{}{}{}",
                                good.render(),
                                Escape(last),
                                good.render_reset()
                            );
                        }
                        styled.push_str("]");
                    }
                }

                true
            } else {
                false
            }
        }
        ErrorKind::InvalidUtf8 => false,
        ErrorKind::TooManyValues => {
            let invalid_arg = error.get(ContextKind::InvalidArg);
            let invalid_value = error.get(ContextKind::InvalidValue);
            if let (
                Some(ContextValue::String(invalid_arg)),
                Some(ContextValue::String(invalid_value)),
            ) = (invalid_arg, invalid_value)
            {
                let _ = write!(
                    styled,
                    "unexpected value '{}{invalid_value}{}' for '{}{invalid_arg}{}' found; no more were expected",
                    warning.render(),
                    warning.render_reset(),
                    literal.render(),
                    literal.render_reset(),
                );
                true
            } else {
                false
            }
        }
        ErrorKind::TooFewValues => {
            let invalid_arg = error.get(ContextKind::InvalidArg);
            let actual_num_values = error.get(ContextKind::ActualNumValues);
            let min_values = error.get(ContextKind::MinValues);
            if let (
                Some(ContextValue::String(invalid_arg)),
                Some(ContextValue::Number(actual_num_values)),
                Some(ContextValue::Number(min_values)),
            ) = (invalid_arg, actual_num_values, min_values)
            {
                let were_provided = singular_or_plural(*actual_num_values as usize);
                let _ = write!(
                    styled,
                    "{}{min_values}{} more values required by '{}{invalid_arg}{}'; only {}{actual_num_values}{}{were_provided}",
                    good.render(),
                    good.render_reset(),
                    literal.render(),
                    literal.render_reset(),
                    warning.render(),
                    warning.render_reset(),
                );
                true
            } else {
                false
            }
        }
        ErrorKind::ValueValidation => {
            let invalid_arg = error.get(ContextKind::InvalidArg);
            let invalid_value = error.get(ContextKind::InvalidValue);
            if let (
                Some(ContextValue::String(invalid_arg)),
                Some(ContextValue::String(invalid_value)),
            ) = (invalid_arg, invalid_value)
            {
                let _ = write!(
                    styled,
                    "invalid value '{}{invalid_value}{}' for '{}{invalid_arg}{}'",
                    warning.render(),
                    warning.render_reset(),
                    literal.render(),
                    literal.render_reset(),
                );
                if let Some(source) = error.inner.source.as_deref() {
                    let _ = write!(styled, ": {}", source);
                }
                true
            } else {
                false
            }
        }
        ErrorKind::WrongNumberOfValues => {
            let invalid_arg = error.get(ContextKind::InvalidArg);
            let actual_num_values = error.get(ContextKind::ActualNumValues);
            let num_values = error.get(ContextKind::ExpectedNumValues);
            if let (
                Some(ContextValue::String(invalid_arg)),
                Some(ContextValue::Number(actual_num_values)),
                Some(ContextValue::Number(num_values)),
            ) = (invalid_arg, actual_num_values, num_values)
            {
                let were_provided = singular_or_plural(*actual_num_values as usize);
                let _ = write!(
                    styled,
                    "{}{num_values}{} values required for '{}{invalid_arg}{}' but {}{actual_num_values}{}{were_provided}",
                    good.render(),
                    good.render_reset(),
                    literal.render(),
                    literal.render_reset(),
                    warning.render(),
                    warning.render_reset(),
                );
                true
            } else {
                false
            }
        }
        ErrorKind::UnknownArgument => {
            let invalid_arg = error.get(ContextKind::InvalidArg);
            if let Some(ContextValue::String(invalid_arg)) = invalid_arg {
                let _ = write!(
                    styled,
                    "unexpected argument '{}{invalid_arg}{}' found",
                    warning.render(),
                    warning.render_reset(),
                );
                true
            } else {
                false
            }
        }
        ErrorKind::DisplayHelp
        | ErrorKind::DisplayHelpOnMissingArgumentOrSubcommand
        | ErrorKind::DisplayVersion
        | ErrorKind::Io
        | ErrorKind::Format => false,
    }
}

pub(crate) fn format_error_message(
    message: &str,
    cmd: Option<&Command>,
    usage: Option<&StyledStr>,
) -> StyledStr {
    let mut styled = StyledStr::new();
    start_error(&mut styled);
    styled.push_str(message);
    if let Some(usage) = usage {
        put_usage(&mut styled, usage);
    }
    if let Some(cmd) = cmd {
        try_help(&mut styled, get_help_flag(cmd));
    }
    styled
}

/// Returns the singular or plural form on the verb to be based on the argument's value.
fn singular_or_plural(n: usize) -> &'static str {
    if n > 1 {
        " were provided"
    } else {
        " was provided"
    }
}

fn put_usage(styled: &mut StyledStr, usage: &StyledStr) {
    styled.push_str("\n\n");
    styled.push_styled(usage);
}

pub(crate) fn get_help_flag(cmd: &Command) -> Option<&'static str> {
    if !cmd.is_disable_help_flag_set() {
        Some("--help")
    } else if cmd.has_subcommands() && !cmd.is_disable_help_subcommand_set() {
        Some("help")
    } else {
        None
    }
}

fn try_help(styled: &mut StyledStr, help: Option<&str>) {
    if let Some(help) = help {
        use std::fmt::Write as _;
        let literal = Style::Literal.as_style();
        let _ = write!(
            styled,
            "\n\nFor more information, try '{}{help}{}'.\n",
            literal.render(),
            literal.render_reset()
        );
    } else {
        styled.push_str("\n");
    }
}

#[cfg(feature = "error-context")]
fn did_you_mean(styled: &mut StyledStr, context: &str, valid: &ContextValue) {
    use std::fmt::Write as _;
    let good = Style::Good.as_style();

    let _ = write!(styled, "{TAB}{}tip:{}", good.render(), good.render_reset());
    if let ContextValue::String(valid) = valid {
        let _ = write!(
            styled,
            " a similar {context} exists: '{}{valid}{}'",
            good.render(),
            good.render_reset()
        );
    } else if let ContextValue::Strings(valid) = valid {
        if valid.len() == 1 {
            let _ = write!(styled, " a similar {context} exists: ",);
        } else {
            let _ = write!(styled, " some similar {context}s exist: ",);
        }
        for (i, valid) in valid.iter().enumerate() {
            if i != 0 {
                styled.push_str(", ");
            }
            let _ = write!(styled, "'{}{valid}{}'", good.render(), good.render_reset());
        }
    }
}

struct Escape<'s>(&'s str);

impl<'s> std::fmt::Display for Escape<'s> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.0.contains(char::is_whitespace) {
            std::fmt::Debug::fmt(self.0, f)
        } else {
            self.0.fmt(f)
        }
    }
}
