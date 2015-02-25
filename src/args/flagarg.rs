
#[derive(Clone)]
pub struct FlagArg {
    pub name: &'static str,
    pub short: Option<&'static str>,
    pub long: Option<&'static str>,
    pub help: Option<&'static str>,
}