#[derive(Clone)]
pub struct FlagArg {
    pub name: &'static str,
    pub short: Option<char>,
    pub long: Option<&'static str>,
    pub help: Option<&'static str>,
    pub multiple: bool,
    pub occurrences: u8,
    pub requires: Option<Vec<&'static str>>
}