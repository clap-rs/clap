pub enum ConditionsModifier {
    All,
    Any,
    None,
}

impl Default for ConditionsModifer {
    fn default() -> Self {
        ConditionsModifier::Any
    }
}

pub struct Condition<'help> {
    arg: u64,
    arg_value: Option<&'help str>,
    other_value: Option<&'help str>,
}

impl<'help> Condition<'help> {
    pub fn new(id: ArgId) -> Self {
        Condition {
            arg: id,
            arg_value: None,
            other_value: None,
        }

    }

    pub fn arg(mut self, id: ArgId) -> Self {
        self.arg = id;
        self
    }

    pub fn arg_value(mut self, val: &'help str) -> Self {
        self.arg_value = Some(val);
        self
    }

    pub fn other_value(mut self, val: &'help str) -> Self {
        self.other_value = Some(val);
        self
    }
}