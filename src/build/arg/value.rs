mod default_values;
mod delimiter;
mod filter;
mod possible_values;
mod terminator;
mod value_name;

pub struct Value<'help> {
    defaults: Option<DefaultValues>,
    filter: Filter,
    occurrence: Occurrence,
    requires_equals: bool,
    delimiter: Delimiter,
}

impl<'help> Value<'help> {
    fn new() -> Self {

    }
}