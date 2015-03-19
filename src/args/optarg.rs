/// `OptArg` represents a option argument for command line applications, which is one that
/// takes an additional value. Option arguments are always preceded by either a `-` 
/// (single character) or `--` (single word, no spaces) then followed by a space and the
/// value.  `OptArg` isn't directly used by the end application
/// writer, only internally to the `clap` library.
///
/// # Example
///
/// ```sh
/// $ myprog -a some --test other --third=file
/// ```
///
/// **NOTE:** The long version may also use the `--argument=value` version too
pub struct OptArg {
    /// The unique name of the argument, required
    pub name: &'static str,
    /// The short version (i.e. single character) of the argument, no preceding `-`
    pub short: Option<char>,
    /// The long version of the flag (i.e. word) without the preceding `--`
    pub long: Option<&'static str>,
    /// The string of text that will displayed to the user when the application's
    /// `help` text is displayed
    pub help: Option<&'static str>,
    /// If this is a required by default when using the command line program
    /// i.e. a configuration file that's required for the program to function
    /// **NOTE:** required by default means, it is required *until* mutually
    /// exclusive arguments are evaluated.
    pub required: bool,
    /// A list of names of other arguments that are *required* to be used when 
    /// this flag is used
    pub requires: Option<Vec<&'static str>>,
    /// A list of names for other arguments that *may not* be used with this flag
    pub blacklist: Option<Vec<&'static str>>,
    /// Allow multiple occurrences of an option argument such as "-c some -c other"
    pub multiple: bool,
    /// How many occurences of this option have been found when parsing
    pub occurrences: u8,
    /// The value provided to the argument by the user
    pub values: Vec<String>
}
