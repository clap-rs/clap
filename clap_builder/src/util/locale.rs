//! Handles localization (l10n) for the application.
//! This module uses the Fluent localization system when the `i18n` feature is enabled.
#[cfg(feature = "i18n")]
use fluent::{FluentArgs, FluentBundle, FluentResource};
#[cfg(feature = "i18n")]
use std::fmt;
#[cfg(feature = "i18n")]
use std::fs;
#[cfg(feature = "i18n")]
use std::path::{Path, PathBuf};
#[cfg(feature = "i18n")]
use std::str::FromStr;
#[cfg(feature = "i18n")]
use std::sync::OnceLock;
#[cfg(feature = "i18n")]
use unic_langid::LanguageIdentifier;

#[cfg(feature = "i18n")]
/// Represents errors that can occur during localization operations.
#[derive(Debug)]
pub enum LocalizationError {
    /// Represents an I/O error that occurred while loading a localization resource.
    Io {
        /// The underlying I/O error that occurred.
        source: std::io::Error,
        /// The path where the I/O error occurred.
        path: PathBuf,
    },
    /// Represents a parsing error that occurred while processing a localization resource.
    Parse(String),
    /// Represents an error that occurred while adding a resource to a Fluent bundle.
    Bundle(String),
    /// Represents an error when the locales directory cannot be found.
    LocalesDirNotFound(String),
    /// Represents an error that occurred during path resolution.
    PathResolution(String),
}

#[cfg(feature = "i18n")]
impl fmt::Display for LocalizationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LocalizationError::Io { source, path } => {
                write!(f, "I/O error loading '{}': {}", path.display(), source)
            }
            LocalizationError::Parse(msg) => write!(f, "Parse error: {msg}"),
            LocalizationError::Bundle(msg) => write!(f, "Bundle error: {msg}"),
            LocalizationError::LocalesDirNotFound(path) => {
                write!(f, "Locales directory not found: {path}")
            }
            LocalizationError::PathResolution(msg) => {
                write!(f, "Path resolution error: {msg}")
            }
        }
    }
}

#[cfg(feature = "i18n")]
impl std::error::Error for LocalizationError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            LocalizationError::Io { source, .. } => Some(source),
            _ => None,
        }
    }
}

#[cfg(feature = "i18n")]
impl From<std::io::Error> for LocalizationError {
    fn from(error: std::io::Error) -> Self {
        LocalizationError::Io {
            source: error,
            path: PathBuf::from("<unknown>"),
        }
    }
}

/// The default locale used for localization when no other locale is specified.
pub const DEFAULT_LOCALE: &str = "en-US";

#[cfg(not(feature = "i18n"))]
#[macro_export]
/// Macro for retrieving a localized message by its ID. Just return the English string when i18n is disabled
macro_rules! msg {
    ($id:expr, $english:expr) => {
        $english.to_string()
    };
}

#[cfg(feature = "i18n")]
#[macro_export]
/// Macro for retrieving a localized message by its ID, falling back to the provided English string if necessary.
macro_rules! msg {
    ($id:expr, $english:expr) => {{
        $crate::util::locale::get_message_internal($id, $english, None)
    }};
}

#[cfg(not(feature = "i18n"))]
#[macro_export]
/// Macro for formatting localized messages with arguments when i18n is disabled.
macro_rules! msg_args {
    ($id:expr, $english:expr, $($key:expr => $value:expr),*) => {{
        let mut result = $english.to_string();
        $(
            let placeholder = format!("{{{}}}", $key);
            result = result.replace(&placeholder, &$value.to_string());
        )*
        result
    }};
}
#[cfg(feature = "i18n")]
#[macro_export]
/// Macro for formatting localized messages with arguments when i18n is enabled.
macro_rules! msg_args {
    ($id:expr, $english:expr, $($key:expr => $value:expr),*) => {{
        let mut fluent_args = fluent::FluentArgs::new();
        $(
            let val = $value.to_string();
            if let Ok(num) = val.parse::<i64>() {
                fluent_args.set($key, num);
            } else if let Ok(num) = val.parse::<f64>() {
                fluent_args.set($key, num);
            } else {
                fluent_args.set($key, val);
            }
        )*
        $crate::util::locale::get_message_internal($id, $english, Some(fluent_args))
    }};
}

#[cfg(feature = "i18n")]
struct Localizer {
    primary_bundle: FluentBundle<FluentResource>,
    fallback_bundle: Option<FluentBundle<FluentResource>>,
}

#[cfg(feature = "i18n")]
impl Localizer {
    fn new(primary_bundle: FluentBundle<FluentResource>) -> Self {
        Self {
            primary_bundle,
            fallback_bundle: None,
        }
    }

    fn with_fallback(mut self, fallback_bundle: FluentBundle<FluentResource>) -> Self {
        self.fallback_bundle = Some(fallback_bundle);
        self
    }

    fn format(&self, id: &str, english: &str, args: Option<&FluentArgs<'_>>) -> String {
        if let Some(message) = self.primary_bundle.get_message(id).and_then(|m| m.value()) {
            let mut errs = Vec::new();
            return self
                .primary_bundle
                .format_pattern(message, args, &mut errs)
                .to_string();
        }

        if let Some(ref fallback) = self.fallback_bundle {
            if let Some(message) = fallback.get_message(id).and_then(|m| m.value()) {
                let mut errs = Vec::new();
                return fallback
                    .format_pattern(message, args, &mut errs)
                    .to_string();
            }
        }

        english.to_string()
    }
}

#[cfg(feature = "i18n")]
thread_local! {
    static LOCALIZER: OnceLock<Localizer> = const { OnceLock::new() };
}

#[cfg(feature = "i18n")]
/// Retrieves a localized message by its ID, falling back to the provided English string if necessary.
///
/// # Arguments
///
/// * `id` - The message identifier.
/// * `english` - The English fallback string.
/// * `args` - Optional Fluent arguments for message formatting.
///
/// # Returns
///
/// The localized message as a `String`.
pub fn get_message_internal(id: &str, english: &str, args: Option<FluentArgs<'_>>) -> String {
    LOCALIZER.with(|lock| {
        lock.get()
            .map(|loc| loc.format(id, english, args.as_ref()))
            .unwrap_or_else(|| english.to_string())
    })
}

#[cfg(feature = "i18n")]
fn init_localization(
    locale: &LanguageIdentifier,
    locales_dir: &Path,
) -> Result<(), LocalizationError> {
    // Check if already initialized first
    let already_initialized = LOCALIZER.with(|lock| lock.get().is_some());
    if already_initialized {
        return Ok(());
    }

    let en_locale = LanguageIdentifier::from_str(DEFAULT_LOCALE)
        .expect("Default locale should always be valid");
    let english_bundle = create_bundle(&en_locale, locales_dir)?;
    let loc = if locale == &en_locale {
        Localizer::new(english_bundle)
    } else {
        if let Ok(primary_bundle) = create_bundle(locale, locales_dir) {
            Localizer::new(primary_bundle).with_fallback(english_bundle)
        } else {
            Localizer::new(english_bundle)
        }
    };

    LOCALIZER.with(|lock| {
        // Use set() but ignore the error if it's already set
        // This handles race conditions where another thread might have initialized it
        // between our check above and this point
        let _ = lock.set(loc);
    });
    Ok(())
}

#[cfg(feature = "i18n")]
fn create_bundle(
    locale: &LanguageIdentifier,
    locales_dir: &Path,
) -> Result<FluentBundle<FluentResource>, LocalizationError> {
    let locale_path = locales_dir.join(format!("{locale}.ftl"));

    let ftl_file = fs::read_to_string(&locale_path).map_err(|e| LocalizationError::Io {
        source: e,
        path: locale_path.clone(),
    })?;

    let resource = FluentResource::try_new(ftl_file).map_err(|_| {
        LocalizationError::Parse(format!(
            "Failed to parse localization resource for {}: {}",
            locale,
            locale_path.display()
        ))
    })?;

    let mut bundle = FluentBundle::new(vec![locale.clone()]);
    bundle.set_use_isolating(false);

    bundle.add_resource(resource).map_err(|errs| {
        LocalizationError::Bundle(format!(
            "Failed to add resource to bundle for {locale}: {errs:?}"
        ))
    })?;

    Ok(bundle)
}

#[cfg(feature = "i18n")]
fn detect_system_locale() -> Result<LanguageIdentifier, LocalizationError> {
    let locale_str = std::env::var("LANG")
        .unwrap_or_else(|_| DEFAULT_LOCALE.to_string())
        .split('.')
        .next()
        .unwrap_or(DEFAULT_LOCALE)
        .to_string();

    LanguageIdentifier::from_str(&locale_str)
        .map_err(|_| LocalizationError::Parse(format!("Failed to parse locale: {locale_str}")))
}

#[cfg(not(feature = "i18n"))]
#[inline]
/// Sets up localization for the application (no-op when `i18n` feature is disabled).
///
/// # Arguments
///
/// * `_app_name` - The name of the application (unused when `i18n` is disabled).
///
/// # Returns
///
/// Always returns `Ok(())`.
pub fn setup_localization(_app_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}

#[cfg(feature = "i18n")]
/// Sets up localization for the application by detecting the system locale and initializing the localizer.
/// This function is idempotent - calling it multiple times will not cause errors.
///
/// # Arguments
///
/// * `app_name` - The name of the application, used to locate the appropriate locales directory.
///
/// # Errors
///
/// Returns a `LocalizationError` if the localization setup fails.
pub fn setup_localization(app_name: &str) -> Result<(), LocalizationError> {
    let locale = detect_system_locale().unwrap_or_else(|_| {
        LanguageIdentifier::from_str(DEFAULT_LOCALE).expect("Default locale should always be valid")
    });

    let locales_dir = get_locales_dir(app_name)?;
    init_localization(&locale, &locales_dir)
}

#[cfg(feature = "i18n")]
fn get_locales_dir(app_name: &str) -> Result<PathBuf, LocalizationError> {
    #[cfg(debug_assertions)]
    {
        let manifest_dir = env!("CARGO_MANIFEST_DIR");
        let dev_path = PathBuf::from(manifest_dir)
            .join("../uu")
            .join(app_name)
            .join("locales");
        if dev_path.exists() {
            return Ok(dev_path);
        }
        let fallback_dev_path = PathBuf::from(manifest_dir).join("locales");
        if fallback_dev_path.exists() {
            return Ok(fallback_dev_path);
        }
        Err(LocalizationError::LocalesDirNotFound(format!(
            "Development locales directory not found at {} or {}",
            dev_path.display(),
            fallback_dev_path.display()
        )))
    }

    #[cfg(not(debug_assertions))]
    {
        use std::env;
        let exe_path = env::current_exe().map_err(|e| {
            LocalizationError::PathResolution(format!("Failed to get executable path: {}", e))
        })?;
        let exe_dir = exe_path.parent().ok_or_else(|| {
            LocalizationError::PathResolution("Failed to get executable directory".to_string())
        })?;
        let resource_path = exe_dir.join("locales").join(app_name);
        if resource_path.exists() {
            return Ok(resource_path);
        }
        let fallback_path = exe_dir.join("locales");
        if fallback_path.exists() {
            return Ok(fallback_path);
        }
        Err(LocalizationError::LocalesDirNotFound(format!(
            "Release locales directory not found at {} or {}",
            resource_path.display(),
            fallback_path.display()
        )))
    }
}
