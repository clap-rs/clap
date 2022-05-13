use std::any::TypeId;
use std::sync::Arc;

use crate::parser::AnyValue;

/// Parse/validate argument values
#[derive(Clone)]
pub struct ValueParser(pub(crate) ValueParserInner);

#[derive(Clone)]
pub(crate) enum ValueParserInner {
    // Common enough to optimize and for possible values
    Bool,
    // Common enough to optimize
    String,
    // Common enough to optimize
    OsString,
    // Common enough to optimize
    PathBuf,
    Other(Arc<dyn AnyValueParser + Send + Sync + 'static>),
}

impl ValueParser {
    /// Custom parser for argument values
    pub fn new(other: impl AnyValueParser + Send + Sync + 'static) -> Self {
        Self(ValueParserInner::Other(Arc::new(other)))
    }

    /// `Bool` parser for argument values
    ///
    /// See also:
    /// - [`BoolishValueParser`] for different human readable bool representations
    /// - [`FalseyValueParser`] for assuming non-false is true
    pub const fn bool() -> Self {
        Self(ValueParserInner::Bool)
    }

    /// `String` parser for argument values
    pub const fn string() -> Self {
        Self(ValueParserInner::String)
    }

    /// `OsString` parser for argument values
    pub const fn os_string() -> Self {
        Self(ValueParserInner::OsString)
    }

    /// `PathBuf` parser for argument values
    pub const fn path_buf() -> Self {
        Self(ValueParserInner::PathBuf)
    }
}

impl ValueParser {
    /// Parse into a `Arc<Any>`
    ///
    /// When `arg` is `None`, an external subcommand value is being parsed.
    pub fn parse_ref(
        &self,
        cmd: &crate::Command,
        arg: Option<&crate::Arg>,
        value: &std::ffi::OsStr,
    ) -> Result<AnyValue, crate::Error> {
        self.any_value_parser().parse_ref(cmd, arg, value)
    }

    /// Parse into a `Arc<Any>`
    ///
    /// When `arg` is `None`, an external subcommand value is being parsed.
    pub fn parse(
        &self,
        cmd: &crate::Command,
        arg: Option<&crate::Arg>,
        value: std::ffi::OsString,
    ) -> Result<AnyValue, crate::Error> {
        self.any_value_parser().parse(cmd, arg, value)
    }

    /// Describes the content of `Arc<Any>`
    pub fn type_id(&self) -> TypeId {
        self.any_value_parser().type_id()
    }

    /// Describes the content of `Arc<Any>`
    pub fn type_name(&self) -> &'static str {
        self.any_value_parser().type_name()
    }

    /// Reflect on enumerated value properties
    ///
    /// Error checking should not be done with this; it is mostly targeted at user-facing
    /// applications like errors and completion.
    pub fn possible_values(
        &self,
    ) -> Option<Box<dyn Iterator<Item = crate::PossibleValue<'static>> + '_>> {
        self.any_value_parser().possible_values()
    }

    fn any_value_parser(&self) -> &dyn AnyValueParser {
        match &self.0 {
            ValueParserInner::Bool => &BoolValueParser,
            ValueParserInner::String => &StringValueParser,
            ValueParserInner::OsString => &OsStringValueParser,
            ValueParserInner::PathBuf => &PathBufValueParser,
            ValueParserInner::Other(o) => o.as_ref(),
        }
    }
}

impl<P: AnyValueParser + Send + Sync + 'static> From<P> for ValueParser {
    fn from(p: P) -> Self {
        ValueParser(ValueParserInner::Other(Arc::new(p)))
    }
}

impl<'help> std::fmt::Debug for ValueParser {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        match &self.0 {
            ValueParserInner::Bool => f.debug_struct("ValueParser::bool").finish(),
            ValueParserInner::String => f.debug_struct("ValueParser::string").finish(),
            ValueParserInner::OsString => f.debug_struct("ValueParser::os_string").finish(),
            ValueParserInner::PathBuf => f.debug_struct("ValueParser::path_buf").finish(),
            ValueParserInner::Other(o) => write!(f, "ValueParser::other({})", o.type_name()),
        }
    }
}

// Require people to implement `TypedValueParser` rather than `AnyValueParser`:
// - Make implementing the user-facing trait easier
// - Enforce in the type-system that a given `AnyValueParser::parse` always returns the same type
//   on each call and that it matches `type_id` / `type_name`
/// Parse/validate argument values into a `Arc<Any>`
pub trait AnyValueParser: private::AnyValueParserSealed {
    /// Parse into a `Arc<Any>`
    ///
    /// When `arg` is `None`, an external subcommand value is being parsed.
    fn parse_ref(
        &self,
        cmd: &crate::Command,
        arg: Option<&crate::Arg>,
        value: &std::ffi::OsStr,
    ) -> Result<AnyValue, crate::Error>;

    /// Parse into a `Arc<Any>`
    ///
    /// When `arg` is `None`, an external subcommand value is being parsed.
    fn parse(
        &self,
        cmd: &crate::Command,
        arg: Option<&crate::Arg>,
        value: std::ffi::OsString,
    ) -> Result<AnyValue, crate::Error>;

    /// Describes the content of `Arc<Any>`
    fn type_id(&self) -> TypeId;

    /// Describes the content of `Arc<Any>`
    fn type_name(&self) -> &'static str;

    /// Reflect on enumerated value properties
    ///
    /// Error checking should not be done with this; it is mostly targeted at user-facing
    /// applications like errors and completion.
    fn possible_values(
        &self,
    ) -> Option<Box<dyn Iterator<Item = crate::PossibleValue<'static>> + '_>> {
        None
    }
}

impl<T, P> AnyValueParser for P
where
    T: std::any::Any + Send + Sync + 'static,
    P: TypedValueParser<Value = T>,
{
    fn parse_ref(
        &self,
        cmd: &crate::Command,
        arg: Option<&crate::Arg>,
        value: &std::ffi::OsStr,
    ) -> Result<AnyValue, crate::Error> {
        let value = TypedValueParser::parse_ref(self, cmd, arg, value)?;
        Ok(Arc::new(value))
    }

    fn parse(
        &self,
        cmd: &crate::Command,
        arg: Option<&crate::Arg>,
        value: std::ffi::OsString,
    ) -> Result<AnyValue, crate::Error> {
        let value = TypedValueParser::parse(self, cmd, arg, value)?;
        Ok(Arc::new(value))
    }

    fn type_id(&self) -> TypeId {
        TypeId::of::<T>()
    }

    fn type_name(&self) -> &'static str {
        std::any::type_name::<T>()
    }

    fn possible_values(
        &self,
    ) -> Option<Box<dyn Iterator<Item = crate::PossibleValue<'static>> + '_>> {
        P::possible_values(self)
    }
}

/// Parse/validate argument values
pub trait TypedValueParser {
    /// Argument's value type
    type Value;

    /// Parse the argument value
    ///
    /// When `arg` is `None`, an external subcommand value is being parsed.
    fn parse_ref(
        &self,
        cmd: &crate::Command,
        arg: Option<&crate::Arg>,
        value: &std::ffi::OsStr,
    ) -> Result<Self::Value, crate::Error>;

    /// Parse the argument value
    ///
    /// When `arg` is `None`, an external subcommand value is being parsed.
    fn parse(
        &self,
        cmd: &crate::Command,
        arg: Option<&crate::Arg>,
        value: std::ffi::OsString,
    ) -> Result<Self::Value, crate::Error> {
        self.parse_ref(cmd, arg, &value)
    }

    /// Reflect on enumerated value properties
    ///
    /// Error checking should not be done with this; it is mostly targeted at user-facing
    /// applications like errors and completion.
    fn possible_values(
        &self,
    ) -> Option<Box<dyn Iterator<Item = crate::PossibleValue<'static>> + '_>> {
        None
    }
}

impl<T, E> TypedValueParser for fn(&str) -> Result<T, E>
where
    E: Into<Box<dyn std::error::Error + Send + Sync + 'static>>,
{
    type Value = T;

    fn parse_ref(
        &self,
        cmd: &crate::Command,
        arg: Option<&crate::Arg>,
        value: &std::ffi::OsStr,
    ) -> Result<Self::Value, crate::Error> {
        let value = value.to_str().ok_or_else(|| {
            crate::Error::invalid_utf8(
                cmd,
                crate::output::Usage::new(cmd).create_usage_with_title(&[]),
            )
        })?;
        let value = (self)(value).map_err(|e| {
            let arg = arg
                .map(|a| a.to_string())
                .unwrap_or_else(|| "...".to_owned());
            crate::Error::value_validation(arg, value.to_owned(), e.into()).with_cmd(cmd)
        })?;
        Ok(value)
    }
}

impl<T, E> TypedValueParser for fn(&std::ffi::OsStr) -> Result<T, E>
where
    E: Into<Box<dyn std::error::Error + Send + Sync + 'static>>,
{
    type Value = T;

    fn parse_ref(
        &self,
        cmd: &crate::Command,
        arg: Option<&crate::Arg>,
        value: &std::ffi::OsStr,
    ) -> Result<Self::Value, crate::Error> {
        let value = (self)(value).map_err(|e| {
            let arg = arg
                .map(|a| a.to_string())
                .unwrap_or_else(|| "...".to_owned());
            crate::Error::value_validation(arg, value.to_string_lossy().into_owned(), e.into())
                .with_cmd(cmd)
        })?;
        Ok(value)
    }
}

#[derive(Copy, Clone, Debug)]
struct StringValueParser;

impl TypedValueParser for StringValueParser {
    type Value = String;

    fn parse_ref(
        &self,
        cmd: &crate::Command,
        arg: Option<&crate::Arg>,
        value: &std::ffi::OsStr,
    ) -> Result<Self::Value, crate::Error> {
        TypedValueParser::parse(self, cmd, arg, value.to_owned())
    }

    fn parse(
        &self,
        cmd: &crate::Command,
        _arg: Option<&crate::Arg>,
        value: std::ffi::OsString,
    ) -> Result<Self::Value, crate::Error> {
        let value = value.into_string().map_err(|_| {
            crate::Error::invalid_utf8(
                cmd,
                crate::output::Usage::new(cmd).create_usage_with_title(&[]),
            )
        })?;
        Ok(value)
    }
}

#[derive(Copy, Clone, Debug)]
struct OsStringValueParser;

impl TypedValueParser for OsStringValueParser {
    type Value = std::ffi::OsString;

    fn parse_ref(
        &self,
        cmd: &crate::Command,
        arg: Option<&crate::Arg>,
        value: &std::ffi::OsStr,
    ) -> Result<Self::Value, crate::Error> {
        TypedValueParser::parse(self, cmd, arg, value.to_owned())
    }

    fn parse(
        &self,
        _cmd: &crate::Command,
        _arg: Option<&crate::Arg>,
        value: std::ffi::OsString,
    ) -> Result<Self::Value, crate::Error> {
        Ok(value)
    }
}

#[derive(Copy, Clone, Debug)]
struct PathBufValueParser;

impl TypedValueParser for PathBufValueParser {
    type Value = std::path::PathBuf;

    fn parse_ref(
        &self,
        cmd: &crate::Command,
        arg: Option<&crate::Arg>,
        value: &std::ffi::OsStr,
    ) -> Result<Self::Value, crate::Error> {
        TypedValueParser::parse(self, cmd, arg, value.to_owned())
    }

    fn parse(
        &self,
        cmd: &crate::Command,
        arg: Option<&crate::Arg>,
        value: std::ffi::OsString,
    ) -> Result<Self::Value, crate::Error> {
        if value.is_empty() {
            return Err(crate::Error::empty_value(
                cmd,
                &[],
                arg.map(ToString::to_string)
                    .unwrap_or_else(|| "...".to_owned()),
                crate::output::Usage::new(cmd).create_usage_with_title(&[]),
            ));
        }
        Ok(Self::Value::from(value))
    }
}

/// Parse an [`ArgEnum`][crate::ArgEnum] value.
///
/// # Example
///
/// ```rust
/// # use std::ffi::OsStr;
/// # use clap::builder::TypedValueParser;
/// # let cmd = clap::Command::new("test");
/// # let arg = None;
///
/// #[derive(Copy, Clone, Debug, PartialEq, Eq)]
/// enum ColorChoice {
///     Always,
///     Auto,
///     Never,
/// }
///
/// impl clap::ArgEnum for ColorChoice {
///     fn value_variants<'a>() -> &'a [Self] {
///         &[Self::Always, Self::Auto, Self::Never]
///     }
///
///     fn to_possible_value<'a>(&self) -> Option<clap::PossibleValue<'a>> {
///         match self {
///             Self::Always => Some(clap::PossibleValue::new("always")),
///             Self::Auto => Some(clap::PossibleValue::new("auto")),
///             Self::Never => Some(clap::PossibleValue::new("never")),
///         }
///     }
/// }
///
/// let value_parser = clap::builder::ArgEnumValueParser::<ColorChoice>::new();
/// // or
/// let value_parser = clap::value_parser!(ColorChoice);
///
/// assert!(value_parser.parse_ref(&cmd, arg, OsStr::new("random")).is_err());
/// assert!(value_parser.parse_ref(&cmd, arg, OsStr::new("")).is_err());
/// assert_eq!(value_parser.parse_ref(&cmd, arg, OsStr::new("always")).unwrap(), ColorChoice::Always);
/// assert_eq!(value_parser.parse_ref(&cmd, arg, OsStr::new("auto")).unwrap(), ColorChoice::Auto);
/// assert_eq!(value_parser.parse_ref(&cmd, arg, OsStr::new("never")).unwrap(), ColorChoice::Never);
/// ```
#[derive(Clone, Debug)]
pub struct ArgEnumValueParser<E: crate::ArgEnum + Clone + Send + Sync + 'static>(
    std::marker::PhantomData<E>,
);

impl<E: crate::ArgEnum + Clone + Send + Sync + 'static> ArgEnumValueParser<E> {
    /// Parse an [`ArgEnum`][crate::ArgEnum]
    pub fn new() -> Self {
        let phantom: std::marker::PhantomData<E> = Default::default();
        Self(phantom)
    }
}

impl<E: crate::ArgEnum + Clone + Send + Sync + 'static> TypedValueParser for ArgEnumValueParser<E> {
    type Value = E;

    fn parse_ref(
        &self,
        cmd: &crate::Command,
        arg: Option<&crate::Arg>,
        value: &std::ffi::OsStr,
    ) -> Result<Self::Value, crate::Error> {
        let ignore_case = arg.map(|a| a.is_ignore_case_set()).unwrap_or(false);
        let possible_vals = || {
            E::value_variants()
                .iter()
                .filter_map(|v| v.to_possible_value())
                .filter(|v| !v.is_hide_set())
                .map(|v| v.get_name())
                .collect::<Vec<_>>()
        };

        let value = value.to_str().ok_or_else(|| {
            crate::Error::invalid_value(
                cmd,
                value.to_string_lossy().into_owned(),
                &possible_vals(),
                arg.map(ToString::to_string)
                    .unwrap_or_else(|| "...".to_owned()),
                crate::output::Usage::new(cmd).create_usage_with_title(&[]),
            )
        })?;
        let value = E::value_variants()
            .iter()
            .find(|v| {
                v.to_possible_value()
                    .expect("ArgEnum::value_variants contains only values with a corresponding ArgEnum::to_possible_value")
                    .matches(value, ignore_case)
            })
            .ok_or_else(|| {
            crate::Error::invalid_value(
                cmd,
                value.to_owned(),
                &possible_vals(),
                arg.map(ToString::to_string)
                    .unwrap_or_else(|| "...".to_owned()),
                crate::output::Usage::new(cmd).create_usage_with_title(&[]),
            )
            })?
            .clone();
        Ok(value)
    }

    fn possible_values(
        &self,
    ) -> Option<Box<dyn Iterator<Item = crate::PossibleValue<'static>> + '_>> {
        Some(Box::new(
            E::value_variants()
                .iter()
                .filter_map(|v| v.to_possible_value()),
        ))
    }
}

/// Verify the value is from an enumerated set pf [`PossibleValue`][crate::PossibleValue].
///
/// # Example
///
/// ```rust
/// # use std::ffi::OsStr;
/// # use clap::builder::TypedValueParser;
/// # let cmd = clap::Command::new("test");
/// # let arg = None;
/// let value_parser = clap::builder::PossibleValuesParser::new(["always", "auto", "never"]);
/// assert!(value_parser.parse_ref(&cmd, arg, OsStr::new("random")).is_err());
/// assert!(value_parser.parse_ref(&cmd, arg, OsStr::new("")).is_err());
/// assert_eq!(value_parser.parse_ref(&cmd, arg, OsStr::new("always")).unwrap(), "always");
/// assert_eq!(value_parser.parse_ref(&cmd, arg, OsStr::new("auto")).unwrap(), "auto");
/// assert_eq!(value_parser.parse_ref(&cmd, arg, OsStr::new("never")).unwrap(), "never");
/// ```
#[derive(Clone, Debug)]
pub struct PossibleValuesParser(Vec<super::PossibleValue<'static>>);

impl PossibleValuesParser {
    /// Verify the value is from an enumerated set pf [`PossibleValue`][crate::PossibleValue].
    pub fn new(values: impl Into<PossibleValuesParser>) -> Self {
        values.into()
    }
}

impl TypedValueParser for PossibleValuesParser {
    type Value = String;

    fn parse_ref(
        &self,
        cmd: &crate::Command,
        arg: Option<&crate::Arg>,
        value: &std::ffi::OsStr,
    ) -> Result<Self::Value, crate::Error> {
        TypedValueParser::parse(self, cmd, arg, value.to_owned())
    }

    fn parse(
        &self,
        cmd: &crate::Command,
        arg: Option<&crate::Arg>,
        value: std::ffi::OsString,
    ) -> Result<String, crate::Error> {
        let value = value.into_string().map_err(|_| {
            crate::Error::invalid_utf8(
                cmd,
                crate::output::Usage::new(cmd).create_usage_with_title(&[]),
            )
        })?;

        let ignore_case = arg.map(|a| a.is_ignore_case_set()).unwrap_or(false);
        if self.0.iter().any(|v| v.matches(&value, ignore_case)) {
            Ok(value)
        } else {
            let possible_vals = self
                .0
                .iter()
                .filter(|v| !v.is_hide_set())
                .map(crate::builder::PossibleValue::get_name)
                .collect::<Vec<_>>();

            Err(crate::Error::invalid_value(
                cmd,
                value.to_owned(),
                &possible_vals,
                arg.map(ToString::to_string)
                    .unwrap_or_else(|| "...".to_owned()),
                crate::output::Usage::new(cmd).create_usage_with_title(&[]),
            ))
        }
    }

    fn possible_values(
        &self,
    ) -> Option<Box<dyn Iterator<Item = crate::PossibleValue<'static>> + '_>> {
        Some(Box::new(self.0.iter().cloned()))
    }
}

impl<I, T> From<I> for PossibleValuesParser
where
    I: IntoIterator<Item = T>,
    T: Into<super::PossibleValue<'static>>,
{
    fn from(values: I) -> Self {
        Self(values.into_iter().map(|t| t.into()).collect())
    }
}

#[derive(Copy, Clone, Debug)]
struct BoolValueParser;

impl BoolValueParser {
    fn possible_values() -> impl Iterator<Item = crate::PossibleValue<'static>> {
        ["true", "false"]
            .iter()
            .copied()
            .map(|l| crate::PossibleValue::new(l).hide(true))
    }
}

impl TypedValueParser for BoolValueParser {
    type Value = bool;

    fn parse_ref(
        &self,
        cmd: &crate::Command,
        arg: Option<&crate::Arg>,
        value: &std::ffi::OsStr,
    ) -> Result<Self::Value, crate::Error> {
        let value = if value == std::ffi::OsStr::new("true") {
            true
        } else if value == std::ffi::OsStr::new("false") {
            false
        } else {
            // Intentionally showing hidden as we hide all of them
            let possible_vals = Self::possible_values()
                .map(|v| v.get_name())
                .collect::<Vec<_>>();

            return Err(crate::Error::invalid_value(
                cmd,
                value.to_string_lossy().into_owned(),
                &possible_vals,
                arg.map(ToString::to_string)
                    .unwrap_or_else(|| "...".to_owned()),
                crate::output::Usage::new(cmd).create_usage_with_title(&[]),
            ));
        };
        Ok(value)
    }

    fn possible_values(
        &self,
    ) -> Option<Box<dyn Iterator<Item = crate::PossibleValue<'static>> + '_>> {
        Some(Box::new(Self::possible_values()))
    }
}

/// Parse false-like string values, everything else is `true`
///
/// See also:
/// - [`ValueParser::bool`] for assuming non-false is true
/// - [`BoolishValueParser`] for different human readable bool representations
///
/// # Example
///
/// ```rust
/// # use std::ffi::OsStr;
/// # use clap::builder::TypedValueParser;
/// # let cmd = clap::Command::new("test");
/// # let arg = None;
/// let value_parser = clap::builder::FalseyValueParser;
/// assert_eq!(value_parser.parse_ref(&cmd, arg, OsStr::new("random")).unwrap(), true);
/// assert_eq!(value_parser.parse_ref(&cmd, arg, OsStr::new("100")).unwrap(), true);
/// assert_eq!(value_parser.parse_ref(&cmd, arg, OsStr::new("")).unwrap(), false);
/// assert_eq!(value_parser.parse_ref(&cmd, arg, OsStr::new("false")).unwrap(), false);
/// assert_eq!(value_parser.parse_ref(&cmd, arg, OsStr::new("No")).unwrap(), false);
/// assert_eq!(value_parser.parse_ref(&cmd, arg, OsStr::new("oFF")).unwrap(), false);
/// assert_eq!(value_parser.parse_ref(&cmd, arg, OsStr::new("0")).unwrap(), false);
/// ```
#[derive(Copy, Clone, Debug)]
pub struct FalseyValueParser;

impl FalseyValueParser {
    fn possible_values() -> impl Iterator<Item = crate::PossibleValue<'static>> {
        crate::util::TRUE_LITERALS
            .iter()
            .chain(crate::util::FALSE_LITERALS.iter())
            .copied()
            .map(|l| crate::PossibleValue::new(l).hide(true))
    }
}

impl TypedValueParser for FalseyValueParser {
    type Value = bool;

    fn parse_ref(
        &self,
        cmd: &crate::Command,
        _arg: Option<&crate::Arg>,
        value: &std::ffi::OsStr,
    ) -> Result<Self::Value, crate::Error> {
        let value = value.to_str().ok_or_else(|| {
            crate::Error::invalid_utf8(
                cmd,
                crate::output::Usage::new(cmd).create_usage_with_title(&[]),
            )
        })?;
        let value = if value.is_empty() {
            false
        } else {
            crate::util::str_to_bool(value).unwrap_or(true)
        };
        Ok(value)
    }

    fn possible_values(
        &self,
    ) -> Option<Box<dyn Iterator<Item = crate::PossibleValue<'static>> + '_>> {
        Some(Box::new(Self::possible_values()))
    }
}

/// Parse bool-like string values, everything else is `true`
///
/// See also:
/// - [`ValueParser::bool`] for different human readable bool representations
/// - [`FalseyValueParser`] for assuming non-false is true
///
/// # Example
///
/// ```rust
/// # use std::ffi::OsStr;
/// # use clap::builder::TypedValueParser;
/// # let cmd = clap::Command::new("test");
/// # let arg = None;
/// let value_parser = clap::builder::BoolishValueParser;
/// assert!(value_parser.parse_ref(&cmd, arg, OsStr::new("random")).is_err());
/// assert!(value_parser.parse_ref(&cmd, arg, OsStr::new("")).is_err());
/// assert!(value_parser.parse_ref(&cmd, arg, OsStr::new("100")).is_err());
/// assert_eq!(value_parser.parse_ref(&cmd, arg, OsStr::new("true")).unwrap(), true);
/// assert_eq!(value_parser.parse_ref(&cmd, arg, OsStr::new("Yes")).unwrap(), true);
/// assert_eq!(value_parser.parse_ref(&cmd, arg, OsStr::new("oN")).unwrap(), true);
/// assert_eq!(value_parser.parse_ref(&cmd, arg, OsStr::new("1")).unwrap(), true);
/// assert_eq!(value_parser.parse_ref(&cmd, arg, OsStr::new("false")).unwrap(), false);
/// assert_eq!(value_parser.parse_ref(&cmd, arg, OsStr::new("No")).unwrap(), false);
/// assert_eq!(value_parser.parse_ref(&cmd, arg, OsStr::new("oFF")).unwrap(), false);
/// assert_eq!(value_parser.parse_ref(&cmd, arg, OsStr::new("0")).unwrap(), false);
/// ```
#[derive(Copy, Clone, Debug)]
pub struct BoolishValueParser;

impl BoolishValueParser {
    fn possible_values() -> impl Iterator<Item = crate::PossibleValue<'static>> {
        crate::util::TRUE_LITERALS
            .iter()
            .chain(crate::util::FALSE_LITERALS.iter())
            .copied()
            .map(|l| crate::PossibleValue::new(l).hide(true))
    }
}

impl TypedValueParser for BoolishValueParser {
    type Value = bool;

    fn parse_ref(
        &self,
        cmd: &crate::Command,
        arg: Option<&crate::Arg>,
        value: &std::ffi::OsStr,
    ) -> Result<Self::Value, crate::Error> {
        let value = value.to_str().ok_or_else(|| {
            crate::Error::invalid_utf8(
                cmd,
                crate::output::Usage::new(cmd).create_usage_with_title(&[]),
            )
        })?;
        let value = crate::util::str_to_bool(value).ok_or_else(|| {
            let arg = arg
                .map(|a| a.to_string())
                .unwrap_or_else(|| "...".to_owned());
            crate::Error::value_validation(arg, value.to_owned(), "value was not a boolean".into())
                .with_cmd(cmd)
        })?;
        Ok(value)
    }

    fn possible_values(
        &self,
    ) -> Option<Box<dyn Iterator<Item = crate::PossibleValue<'static>> + '_>> {
        Some(Box::new(Self::possible_values()))
    }
}

/// Parse non-empty string values
///
/// # Example
///
/// ```rust
/// # use std::ffi::OsStr;
/// # use clap::builder::TypedValueParser;
/// # let cmd = clap::Command::new("test");
/// # let arg = None;
/// let value_parser = clap::builder::NonEmptyStringValueParser;
/// assert_eq!(value_parser.parse_ref(&cmd, arg, OsStr::new("random")).unwrap(), "random");
/// assert!(value_parser.parse_ref(&cmd, arg, OsStr::new("")).is_err());
/// ```
#[derive(Copy, Clone, Debug)]
pub struct NonEmptyStringValueParser;

impl TypedValueParser for NonEmptyStringValueParser {
    type Value = String;

    fn parse_ref(
        &self,
        cmd: &crate::Command,
        arg: Option<&crate::Arg>,
        value: &std::ffi::OsStr,
    ) -> Result<Self::Value, crate::Error> {
        if value.is_empty() {
            return Err(crate::Error::empty_value(
                cmd,
                &[],
                arg.map(ToString::to_string)
                    .unwrap_or_else(|| "...".to_owned()),
                crate::output::Usage::new(cmd).create_usage_with_title(&[]),
            ));
        }
        let value = value.to_str().ok_or_else(|| {
            crate::Error::invalid_utf8(
                cmd,
                crate::output::Usage::new(cmd).create_usage_with_title(&[]),
            )
        })?;
        Ok(value.to_owned())
    }
}

#[doc(hidden)]
#[derive(Debug)]
pub struct AutoValueParser<T>(std::marker::PhantomData<T>);

impl<T> AutoValueParser<T> {
    #[doc(hidden)]
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self(Default::default())
    }
}

#[doc(hidden)]
pub mod via_prelude {
    use super::*;

    #[doc(hidden)]
    pub trait ValueParserViaBuiltIn: private::ValueParserViaBuiltInSealed {
        fn value_parser(&self) -> ValueParser;
    }
    impl ValueParserViaBuiltIn for &&AutoValueParser<String> {
        fn value_parser(&self) -> ValueParser {
            ValueParser::string()
        }
    }
    impl ValueParserViaBuiltIn for &&AutoValueParser<std::ffi::OsString> {
        fn value_parser(&self) -> ValueParser {
            ValueParser::os_string()
        }
    }
    impl ValueParserViaBuiltIn for &&AutoValueParser<std::path::PathBuf> {
        fn value_parser(&self) -> ValueParser {
            ValueParser::path_buf()
        }
    }

    #[doc(hidden)]
    pub trait ValueParserViaArgEnum: private::ValueParserViaArgEnumSealed {
        type Output;

        fn value_parser(&self) -> Self::Output;
    }
    impl<E: crate::ArgEnum + Clone + Send + Sync + 'static> ValueParserViaArgEnum
        for &AutoValueParser<E>
    {
        type Output = ArgEnumValueParser<E>;

        fn value_parser(&self) -> Self::Output {
            ArgEnumValueParser::<E>::new().into()
        }
    }

    #[doc(hidden)]
    pub trait ValueParserViaFromStr: private::ValueParserViaFromStrSealed {
        fn value_parser(&self) -> ValueParser;
    }
    impl<FromStr> ValueParserViaFromStr for AutoValueParser<FromStr>
    where
        FromStr: std::str::FromStr + std::any::Any + Send + Sync + 'static,
        <FromStr as std::str::FromStr>::Err:
            Into<Box<dyn std::error::Error + Send + Sync + 'static>>,
    {
        fn value_parser(&self) -> ValueParser {
            let func: fn(&str) -> Result<FromStr, <FromStr as std::str::FromStr>::Err> =
                FromStr::from_str;
            ValueParser::new(func)
        }
    }
}

/// Parse/validate argument values
///
/// # Example
///
/// ```rust
/// // Built-in types
/// let parser = clap::value_parser!(String);
/// assert_eq!(format!("{:?}", parser), "ValueParser::string");
/// let parser = clap::value_parser!(std::ffi::OsString);
/// assert_eq!(format!("{:?}", parser), "ValueParser::os_string");
/// let parser = clap::value_parser!(std::path::PathBuf);
/// assert_eq!(format!("{:?}", parser), "ValueParser::path_buf");
///
/// // FromStr types
/// let parser = clap::value_parser!(usize);
/// assert_eq!(format!("{:?}", parser), "ValueParser::other(usize)");
/// ```
#[macro_export]
macro_rules! value_parser {
    ($name:ty) => {{
        use $crate::builder::via_prelude::*;
        let auto = $crate::builder::AutoValueParser::<$name>::new();
        (&&&auto).value_parser()
    }};
}

mod private {
    use super::*;

    pub trait AnyValueParserSealed {}
    impl<T, P> AnyValueParserSealed for P
    where
        T: std::any::Any + Send + Sync + 'static,
        P: TypedValueParser<Value = T>,
    {
    }

    pub trait ValueParserViaSelfSealed {}
    impl<P: Into<ValueParser>> ValueParserViaSelfSealed for &&&AutoValueParser<P> {}

    pub trait ValueParserViaBuiltInSealed {}
    impl ValueParserViaBuiltInSealed for &&AutoValueParser<String> {}
    impl ValueParserViaBuiltInSealed for &&AutoValueParser<std::ffi::OsString> {}
    impl ValueParserViaBuiltInSealed for &&AutoValueParser<std::path::PathBuf> {}

    pub trait ValueParserViaArgEnumSealed {}
    impl<E: crate::ArgEnum> ValueParserViaArgEnumSealed for &AutoValueParser<E> {}

    pub trait ValueParserViaFromStrSealed {}
    impl<FromStr> ValueParserViaFromStrSealed for AutoValueParser<FromStr>
    where
        FromStr: std::str::FromStr + std::any::Any + Send + Sync + 'static,
        <FromStr as std::str::FromStr>::Err:
            Into<Box<dyn std::error::Error + Send + Sync + 'static>>,
    {
    }
}
