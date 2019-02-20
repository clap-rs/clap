#[derive(Default)]
pub struct ValidationRules<'help> {
    occurrence: Occurrence,
    conflicts: Vec<Rule<'help>>,
    requirements: Vec<Rule<'help>>,
    self_required: Vec<Rule<'help>>,
    overrides: Vec<Rule<'help>>,
}

impl<'help> ValidationRules<'help> {
    pub fn new() -> Self {
        ValidationRules::default()
    }

    #[inline(always)]
    pub fn requirement_rule(&mut self, r: Rule) {
        self.requirements.push(r);
    }

    #[inline(always)]
    pub fn self_required_rule(&mut self, r: Rule) {
        self.self_required.push(r);
    }

    #[inline(always)]
    pub fn conflicts_rule(&mut self, r: Rule) {
        self.conflicts.push(r);
    }

    #[inline(always)]
    pub fn overrides_rule(&mut self, r: Rule) {
        self.overrides.push(r);
    }

    #[inline(always)]
    pub fn max_occurs(&mut self, num: usize) {
        self.occurrence.max(num);
    }

    #[inline(always)]
    pub fn min_occurs(&mut self, num: usize) {
        self.occurrence.min(num);
    }

    #[inline]
    pub fn exact_occurs(&mut self, num: usize) {
        self.occurrence.exact(num);
    }
}
