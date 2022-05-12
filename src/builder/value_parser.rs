use std::sync::Arc;

use crate::parser::AnyValue;

/// Parse/validate argument values
#[derive(Clone)]
pub struct ValueParser(pub(crate) ValueParserInner);

#[derive(Clone)]
pub(crate) enum ValueParserInner {
    String,
    OsString,
}

impl ValueParser {
    /// `String` parser for argument values
    pub const fn string() -> Self {
        Self(ValueParserInner::String)
    }

    /// `OsString` parser for argument values
    pub const fn os_string() -> Self {
        Self(ValueParserInner::OsString)
    }
}

impl ValueParser {
    /// Parse into a `Arc<Any>`
    ///
    /// When `arg` is `None`, an external subcommand value is being parsed.
    pub fn parse_ref(
        &self,
        cmd: &crate::Command,
        _arg: Option<&crate::Arg>,
        value: &std::ffi::OsStr,
    ) -> Result<AnyValue, crate::Error> {
        match &self.0 {
            ValueParserInner::String => {
                let value = value.to_str().ok_or_else(|| {
                    crate::Error::invalid_utf8(
                        cmd,
                        crate::output::Usage::new(cmd).create_usage_with_title(&[]),
                    )
                })?;
                Ok(Arc::new(value.to_owned()))
            }
            ValueParserInner::OsString => Ok(Arc::new(value.to_owned())),
        }
    }
}

impl<'help> std::fmt::Debug for ValueParser {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        match &self.0 {
            ValueParserInner::String => f.debug_struct("ValueParser::string").finish(),
            ValueParserInner::OsString => f.debug_struct("ValueParser::os_string").finish(),
        }
    }
}
