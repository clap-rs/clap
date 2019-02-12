pub struct Value<'help> {
    defaults: Option<DefaultValues>,
    filter: Filter,
    occurrence: Occurrence,
    requires_equals: bool,
    delimiter: Delimiter,
}