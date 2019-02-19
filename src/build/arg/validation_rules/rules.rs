use super::ConditionsModifier;

pub enum RuleModifer {
    Unless,
    With,
}

impl Default for RuleModifer {
    fn default() -> Self {
        RuleModifer::With
    }
}

#[derive(Default)]
pub struct Rule<'help> {
    rule_mod: RuleModifer,
    conditions_mod: ConditionsModifier,
    conditions: Vec<Condition<'help>>,
}

impl<'help> Rule<'help> {
    pub fn new() -> Self {
        Rule::default()
    }

    pub fn rule_modifier(mut self, rm: RuleModifer) -> Self {
        self.rule_mod = rm;
        self
    }

    pub fn conditions_mod(mut self, cm: ConditionsModifier) -> Self {
        self.conditions_mod = cm;
        self
    }

    pub fn condition(mut self, c: Condition) -> Self {
        self.conditions.push(c);
        self
    }

    pub fn conditions(mut self, conds: &dyn Iterator<Item=Condition>) -> Self {
        for c in conds {
            self.conditions.push(cond);
        }
        self
    }

    pub fn clear_conditions(&mut self) {
        self.conditions.clear()
    }
}

