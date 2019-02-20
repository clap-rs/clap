#[derive(Default)]
pub struct HelpMessage<'help> {
    short: Option<&'help str>,
    long: Option<&'help str>,
    value_names: VecMap<ValueName>,
    display_order: DisplayOrder,
    unified_order: DisplayOrder, // @TODO remove?
    heading: &'help str, // @TODO multiple?
}

impl<'help> HelpMessage<'help> {
    pub fn new() -> Self {
        HelpMessage::default()
    }

    #[inline(always)]
    pub fn short_message(&mut self, m: &'help str) {
        self.short = Some(m);
    }

    #[inline(always)]
    pub fn long_message(&mut self, m: &'help str) {
        self.long = Some(m);
    }
}