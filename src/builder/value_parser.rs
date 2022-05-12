use std::any::TypeId;
use std::sync::Arc;

use crate::parser::AnyValue;

/// Parse/validate argument values
#[derive(Clone)]
pub struct ValueParser(pub(crate) ValueParserInner);

#[derive(Clone)]
pub(crate) enum ValueParserInner {
    String,
    OsString,
    PathBuf,
    Other(Arc<dyn AnyValueParser + Send + Sync + 'static>),
}

impl ValueParser {
    /// Custom parser for argument values
    pub fn new(other: impl AnyValueParser + Send + Sync + 'static) -> Self {
        Self(ValueParserInner::Other(Arc::new(other)))
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
            ValueParserInner::PathBuf => Ok(Arc::new(std::path::PathBuf::from(value))),
            ValueParserInner::Other(o) => o.parse_ref(cmd, arg, value),
        }
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
        match &self.0 {
            ValueParserInner::String => {
                let value = value.into_string().map_err(|_| {
                    crate::Error::invalid_utf8(
                        cmd,
                        crate::output::Usage::new(cmd).create_usage_with_title(&[]),
                    )
                })?;
                Ok(Arc::new(value))
            }
            ValueParserInner::OsString => Ok(Arc::new(value)),
            ValueParserInner::PathBuf => Ok(Arc::new(std::path::PathBuf::from(value))),
            ValueParserInner::Other(o) => o.parse(cmd, arg, value),
        }
    }

    /// Describes the content of `Arc<Any>`
    pub fn type_id(&self) -> TypeId {
        match &self.0 {
            ValueParserInner::String => TypeId::of::<String>(),
            ValueParserInner::OsString => TypeId::of::<std::ffi::OsString>(),
            ValueParserInner::PathBuf => TypeId::of::<std::path::PathBuf>(),
            ValueParserInner::Other(o) => o.type_id(),
        }
    }

    /// Describes the content of `Arc<Any>`
    pub fn type_name(&self) -> &'static str {
        match &self.0 {
            ValueParserInner::String => std::any::type_name::<String>(),
            ValueParserInner::OsString => std::any::type_name::<std::ffi::OsString>(),
            ValueParserInner::PathBuf => std::any::type_name::<std::path::PathBuf>(),
            ValueParserInner::Other(o) => o.type_name(),
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
    ) -> Result<T, crate::Error> {
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
    ) -> Result<T, crate::Error> {
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
pub trait ValueParserViaBuiltIn: private::ValueParserViaBuiltInSealed {
    fn value_parser(&self) -> ValueParser;
}
impl ValueParserViaBuiltIn for &AutoValueParser<String> {
    fn value_parser(&self) -> ValueParser {
        ValueParser::string()
    }
}
impl ValueParserViaBuiltIn for &AutoValueParser<std::ffi::OsString> {
    fn value_parser(&self) -> ValueParser {
        ValueParser::os_string()
    }
}
impl ValueParserViaBuiltIn for &AutoValueParser<std::path::PathBuf> {
    fn value_parser(&self) -> ValueParser {
        ValueParser::path_buf()
    }
}

#[doc(hidden)]
pub trait ValueParserViaFromStr: private::ValueParserViaFromStrSealed {
    fn value_parser(&self) -> ValueParser;
}
impl<FromStr> ValueParserViaFromStr for AutoValueParser<FromStr>
where
    FromStr: std::str::FromStr + std::any::Any + Send + Sync + 'static,
    <FromStr as std::str::FromStr>::Err: Into<Box<dyn std::error::Error + Send + Sync + 'static>>,
{
    fn value_parser(&self) -> ValueParser {
        let func: fn(&str) -> Result<FromStr, <FromStr as std::str::FromStr>::Err> =
            FromStr::from_str;
        ValueParser::new(func)
    }
}

/// Parse/validate argument values
///
/// # Example
///
/// ```rust
/// let parser = clap::value_parser!(String);
/// assert_eq!(format!("{:?}", parser), "ValueParser::string");
/// let parser = clap::value_parser!(std::ffi::OsString);
/// assert_eq!(format!("{:?}", parser), "ValueParser::os_string");
/// let parser = clap::value_parser!(std::path::PathBuf);
/// assert_eq!(format!("{:?}", parser), "ValueParser::path_buf");
/// let parser = clap::value_parser!(usize);
/// assert_eq!(format!("{:?}", parser), "ValueParser::other(usize)");
/// ```
#[macro_export]
macro_rules! value_parser {
    ($name:ty) => {{
        use $crate::builder::ValueParserViaBuiltIn;
        use $crate::builder::ValueParserViaFromStr;
        let auto = $crate::builder::AutoValueParser::<$name>::new();
        (&&auto).value_parser()
    }};
}

mod private {
    pub trait AnyValueParserSealed {}
    impl<T, P> AnyValueParserSealed for P
    where
        T: std::any::Any + Send + Sync + 'static,
        P: super::TypedValueParser<Value = T>,
    {
    }

    pub trait ValueParserViaBuiltInSealed {}
    impl ValueParserViaBuiltInSealed for &super::AutoValueParser<String> {}
    impl ValueParserViaBuiltInSealed for &super::AutoValueParser<std::ffi::OsString> {}
    impl ValueParserViaBuiltInSealed for &super::AutoValueParser<std::path::PathBuf> {}

    pub trait ValueParserViaFromStrSealed {}
    impl<FromStr> ValueParserViaFromStrSealed for super::AutoValueParser<FromStr>
    where
        FromStr: std::str::FromStr + std::any::Any + Send + Sync + 'static,
        <FromStr as std::str::FromStr>::Err:
            Into<Box<dyn std::error::Error + Send + Sync + 'static>>,
    {
    }
}
