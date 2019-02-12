pub struct DefaultValue<'help> {
    value: &'help str,
}

pub struct ConditionalDefault<'help> {
    value: DefaultValue<'help>,
    other_arg: Option<u64>,
    other_value: Option<&'help str>,
}

pub struct DefaultValues<'help> {
    defaults: Vec<ConditionalDefault<'help>>,
}