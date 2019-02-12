use build::Aliases;

pub struct Long<'help> {
    aliases: Aliases<'help>
}

impl<'help> Long<'help> {
    pub fn long(&mut self, l: &'help str) {
        self.aliases.add_visible(l.trim_left_matches(|c| c == '-'));
    }
    pub fn hidden_long(&mut self, l: &'help str) {
        self.aliases.add_hidden(l.trim_left_matches(|c| c == '-'));
    }
}