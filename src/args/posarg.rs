pub struct PosArg {
    pub name: &'static str,
    pub help: Option<&'static str>,
    pub required: bool,
    pub value: Option<&'static str>,
    pub index: u8 
}