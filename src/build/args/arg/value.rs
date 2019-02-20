mod default_values;
mod delimiter;
mod filter;
mod value_name;

use self::filter::Filter;
use self::default_values::DefaultValues;
use super::Occurrence;
use self::delimiter::Delimiter;

pub struct Value<'help> {
    defaults: Option<DefaultValues<'help>>,
    filter: Filter,
    occurrence: Occurrence,
    requires_equals: bool,
    delimiter: Delimiter,
}

impl<'help> Value<'help> {
    fn new() -> Self {
        unimplemented!()
    }
}