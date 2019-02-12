pub struct EnvDefaultValue<'help> {
    key: &'help OsStr,
    value: Option<OsString>
}
pub struct ConditionalDefault<'help> {
    value: &'help str,
    other_arg: Option<u64>,
    other_value: Option<&'help str>,
}

pub struct DefaultValues<'help> {
    defaults: Vec<ConditionalDefault<'help>>,
    env: Option<EnvDefaultValue<'help>>,
}