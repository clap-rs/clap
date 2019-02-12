pub struct HelpMessage<'help> {
    short_message: &'help str,
    long_message: &'help str,
    value_names: VecMap<ValueName>,
    display_order: DisplayOrder,
    unified_order: DisplayOrder, // @TODO remove?
    heading: &'help str, // @TODO multiple?
}