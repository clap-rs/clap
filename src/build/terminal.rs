#[derive(Copy, Clone, Default, Debug)]
pub struct Terminal {
    // The terminal width as determined at runtime, or overridden by the consumer
    #[doc(hidden)]
    pub width: Option<usize>,
    // The maximum allowed terminal width as set by the consumer
    #[doc(hidden)]
    pub max_width: Option<usize>,
}
