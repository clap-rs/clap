use crate::build::arg::Alias;

pub(crate) struct Long<'help> {
    aliases: Vec<Alias<'help>>,
}

impl<'help> Long<'help> {}
