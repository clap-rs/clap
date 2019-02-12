pub struct PossibleValue<'help> {
    value: &'help str,
}

pub struct PossibleValues<'help> {
    values: Vec<PossibleValue<'help>>
}