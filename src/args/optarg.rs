pub struct OptArg {
    pub name: &'static str,
    pub short: Option<char>,
    pub long: Option<&'static str>,
    pub help: Option<&'static str>,
    pub required: bool,
    pub requires: Option<Vec<&'static str>>,
    pub value: Option<String>
}
