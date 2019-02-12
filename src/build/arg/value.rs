pub struct Value<'help> {
    defaults: Option<DefaultValues>,
    name: Option<&'help str>,
    filter: Filter,
    occurrence: Occurrence,
    requires_equals: bool,
    delimiter: Delimiter,
}