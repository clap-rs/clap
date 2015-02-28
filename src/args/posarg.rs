/// PosArg represents a positional argument, i.e. one that isn't preceded 
/// by a `-` or `--`. 
/// Example: 
/// ```sh
/// $ myprog some_file
/// ```
/// where `some_file` is the first positional argument to `myprog`
/// **NOTE:** The index starts at `1` **NOT** `0`
pub struct PosArg {
    pub name: &'static str,
    pub help: Option<&'static str>,
    pub required: bool,
    pub requires: Option<Vec<&'static str>>,
    pub blacklist: Option<Vec<&'static str>>,
    pub value: Option<String>,
    pub index: u8 
}