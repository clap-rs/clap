pub struct Rule<'help> {
    other_arg: u64,
    self_value: Option<&'help str>,
    other_value: Option<&'help str>,
}

pub struct Rules<'help> {
    rules: Vec<Rule<'help>>,
}

pub struct ValidationRules<'help> {
    occurrence: Occurrence,
    conflicts: Rules<'help>,
    requirements: Rules<'help>,
    overrides: Rules<'help>,
    requirements_unless: Rules<'help>,
}
