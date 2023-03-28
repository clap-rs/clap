#![allow(missing_copy_implementations)]
#![allow(missing_debug_implementations)]
#![cfg_attr(not(feature = "error-context"), allow(dead_code))]
#![cfg_attr(not(feature = "error-context"), allow(unused_imports))]

use crate::builder::Command;
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
        let mut styled = StyledStr::new();
        start_error(&mut styled);
        if let Some(msg) = error.kind().as_str() {
            styled.none(msg.to_owned());
        } else if let Some(source) = error.inner.source.as_ref() {
            styled.none(source.to_string());
        } else {
            styled.none("unknown cause");
        }
        styled.none("\n");
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
        let mut styled = StyledStr::new();
        start_error(&mut styled);

        if !write_dynamic_context(error, &mut styled) {
            if let Some(msg) = error.kind().as_str() {
                styled.none(msg.to_owned());
            } else if let Some(source) = error.inner.source.as_ref() {
                styled.none(source.to_string());
            } else {
                styled.none("unknown cause");
            }
        }

        let mut suggested = false;
        if let Some(valid) = error.get(ContextKind::SuggestedSubcommand) {
            styled.none("\n");
            if !suggested {
                styled.none("\n");
                suggested = true;
            }
            did_you_mean(&mut styled, "subcommand", valid);
        }
        if let Some(valid) = error.get(ContextKind::SuggestedArg) {
            styled.none("\n");
            if !suggested {
                styled.none("\n");
                suggested = true;
            }
            did_you_mean(&mut styled, "argument", valid);
        }
        if let Some(valid) = error.get(ContextKind::SuggestedValue) {
            styled.none("\n");
            if !suggested {
                styled.none("\n");
                suggested = true;
            }
            did_you_mean(&mut styled, "value", valid);
        }
        let suggestions = error.get(ContextKind::Suggested);
        if let Some(ContextValue::StyledStrs(suggestions)) = suggestions {
            if !suggested {
                styled.none("\n");
            }
            for suggestion in suggestions {
                styled.none("\n");
                styled.none(TAB);
                styled.good("tip: ");
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
    styled.error("error:");
    styled.none(" ");
}

#[must_use]
#[cfg(feature = "error-context")]
fn write_dynamic_context(error: &crate::error::Error, styled: &mut StyledStr) -> bool {
    match error.kind() {
        ErrorKind::ArgumentConflict => {
            let invalid_arg = error.get(ContextKind::InvalidArg);
            let prior_arg = error.get(ContextKind::PriorArg);
            if let (Some(ContextValue::String(invalid_arg)), Some(prior_arg)) =
                (invalid_arg, prior_arg)
            {
                if ContextValue::String(invalid_arg.clone()) == *prior_arg {
                    styled.none("the argument '");
                    styled.warning(invalid_arg);
                    styled.none("' cannot be used multiple times");
                } else {
                    styled.none("the argument '");
                    styled.warning(invalid_arg);
                    styled.none("' cannot be used with");

                    match prior_arg {
                        ContextValue::Strings(values) => {
                            styled.none(":");
                            for v in values {
                                styled.none("\n");
                                styled.none(TAB);
                                styled.warning(&**v);
                            }
                        }
                        ContextValue::String(value) => {
                            styled.none(" '");
                            styled.warning(value);
                            styled.none("'");
                        }
                        _ => {
                            styled.none(" one or more of the other specified arguments");
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
                styled.none("equal sign is needed when assigning values to '");
                styled.warning(invalid_arg);
                styled.none("'");
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
                    styled.none("a value is required for '");
                    styled.warning(invalid_arg);
                    styled.none("' but none was supplied");
                } else {
                    styled.none("invalid value '");
                    styled.none(invalid_value);
                    styled.none("' for '");
                    styled.warning(invalid_arg);
                    styled.none("'");
                }

                let possible_values = error.get(ContextKind::ValidValue);
                if let Some(ContextValue::Strings(possible_values)) = possible_values {
                    if !possible_values.is_empty() {
                        styled.none("\n");
                        styled.none(TAB);
                        styled.none("[possible values: ");
                        if let Some((last, elements)) = possible_values.split_last() {
                            for v in elements {
                                styled.good(escape(v));
                                styled.none(", ");
                            }
                            styled.good(escape(last));
                        }
                        styled.none("]");
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
                styled.none("unrecognized subcommand '");
                styled.warning(invalid_sub);
                styled.none("'");
                true
            } else {
                false
            }
        }
        ErrorKind::MissingRequiredArgument => {
            let invalid_arg = error.get(ContextKind::InvalidArg);
            if let Some(ContextValue::Strings(invalid_arg)) = invalid_arg {
                styled.none("the following required arguments were not provided:");
                for v in invalid_arg {
                    styled.none("\n");
                    styled.none(TAB);
                    styled.good(&**v);
                }
                true
            } else {
                false
            }
        }
        ErrorKind::MissingSubcommand => {
            let invalid_sub = error.get(ContextKind::InvalidSubcommand);
            if let Some(ContextValue::String(invalid_sub)) = invalid_sub {
                styled.none("'");
                styled.warning(invalid_sub);
                styled.none("' requires a subcommand but one was not provided");

                let possible_values = error.get(ContextKind::ValidSubcommand);
                if let Some(ContextValue::Strings(possible_values)) = possible_values {
                    if !possible_values.is_empty() {
                        styled.none("\n");
                        styled.none(TAB);
                        styled.none("[subcommands: ");
                        if let Some((last, elements)) = possible_values.split_last() {
                            for v in elements {
                                styled.good(escape(v));
                                styled.none(", ");
                            }
                            styled.good(escape(last));
                        }
                        styled.none("]");
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
                styled.none("unexpected value '");
                styled.warning(invalid_value);
                styled.none("' for '");
                styled.warning(invalid_arg);
                styled.none("' found; no more were expected");
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
                styled.warning(min_values.to_string());
                styled.none(" more values required by '");
                styled.warning(invalid_arg);
                styled.none("'; only ");
                styled.warning(actual_num_values.to_string());
                styled.none(were_provided);
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
                styled.none("invalid value '");
                styled.warning(invalid_value);
                styled.none("' for '");
                styled.warning(invalid_arg);
                if let Some(source) = error.inner.source.as_deref() {
                    styled.none("': ");
                    styled.none(source.to_string());
                } else {
                    styled.none("'");
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
                styled.warning(num_values.to_string());
                styled.none(" values required for '");
                styled.warning(invalid_arg);
                styled.none("' but ");
                styled.warning(actual_num_values.to_string());
                styled.none(were_provided);
                true
            } else {
                false
            }
        }
        ErrorKind::UnknownArgument => {
            let invalid_arg = error.get(ContextKind::InvalidArg);
            if let Some(ContextValue::String(invalid_arg)) = invalid_arg {
                styled.none("unexpected argument '");
                styled.warning(invalid_arg.to_string());
                styled.none("' found");
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
    styled.none(message);
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
    styled.none("\n\n");
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
        styled.none("\n\nFor more information, try '");
        styled.literal(help.to_owned());
        styled.none("'.\n");
    } else {
        styled.none("\n");
    }
}

#[cfg(feature = "error-context")]
fn did_you_mean(styled: &mut StyledStr, context: &str, valid: &ContextValue) {
    if let ContextValue::String(valid) = valid {
        styled.none(TAB);
        styled.good("tip: a similar ");
        styled.none(context);
        styled.none(" exists: '");
        styled.good(valid);
        styled.none("'");
    } else if let ContextValue::Strings(valid) = valid {
        styled.none(TAB);
        if valid.len() == 1 {
            styled.good("tip: a similar ");
            styled.none(context);
            styled.none(" exists: ");
        } else {
            styled.good("tip: some similar ");
            styled.none(context);
            styled.none("s exist: ");
        }
        for (i, valid) in valid.iter().enumerate() {
            if i != 0 {
                styled.none(", ");
            }
            styled.none("'");
            styled.good(valid);
            styled.none("'");
        }
    }
}

fn escape(s: impl AsRef<str>) -> String {
    let s = s.as_ref();
    if s.contains(char::is_whitespace) {
        format!("{s:?}")
    } else {
        s.to_owned()
    }
}
