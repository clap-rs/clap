//! Utilities for loading help texts from external sources.

use std::collections::HashMap;
use lazy_static::lazy_static;
use crate::INTERNAL_ERROR_MSG;

/// Implement this trait for any type that can provide texts to render into the CLI output.
/// Useful for internationalizing applications.
pub trait TextProvider {

    /// Provided a key referring to a text to render in the application, retrieve the text. For
    /// internationalized applications, the internal state of the implementing type should be
    /// aware of the current locale of the application.
    fn get(&self, key: &str) -> impl AsRef<str>;
}

/// A simple [`TextProvider`] implementation which loads Clap's default English texts.
pub struct DefaultTextProvider(HashMap<String, String>);

impl DefaultTextProvider {
    /// Initialize the default [`TextProvider`], which includes the default English texts of Clap.
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for DefaultTextProvider {
    fn default() -> Self {
        let text_raw = include_str!("../texts/en.yaml");
        let parsed: HashMap<String, String> =
            serde_yml::from_str(text_raw).expect(INTERNAL_ERROR_MSG);

        Self(parsed)
    }
}

impl TextProvider for DefaultTextProvider {
    fn get(&self, key: &str) -> impl AsRef<str> {
        self.0.get(key).expect(INTERNAL_ERROR_MSG)
    }
}

lazy_static! {
    /// Initializes the [`DefaultTextProvider`] statically to avoid initializing multiple times.
    pub static ref DEFAULT_TEXT_PROVIDER: DefaultTextProvider = {
        DefaultTextProvider::new()
    };
}