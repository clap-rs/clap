pub struct PosArg {
    pub name: &'static str,
    pub help: Option<&'static str>,
    pub required: bool,
    pub requires: Option<Vec<&'static str>>,
    pub blacklist: Option<Vec<&'static str>>,
    pub value: Option<String>,
    pub index: u8 
}