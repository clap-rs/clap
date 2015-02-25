
#[derive(Clone)]
pub struct PosArg {
    pub name: &'static str,
    pub help: Option<&'static str>,
    pub required: bool,
    pub value: Option<String>,
    pub index: i32
}