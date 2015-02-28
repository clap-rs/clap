/// `PosArg` represents a positional argument, i.e. one that isn't preceded 
/// by a `-` or `--`. `PosArg` isn't directly used by the end application
/// writer, only internally to the `clap` library.
///
/// Example: 
///
/// ```sh
/// $ myprog some_file
/// ```
///
/// where `some_file` is the first positional argument to `myprog`
///
/// **NOTE:** The index starts at `1` **NOT** `0`
pub struct PosArg {
	/// The unique name of the argument, required
    pub name: &'static str,
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
    /// The value provided to the argument by the user
    pub value: Option<String>,
    /// The index of the argument
    pub index: u8 
}