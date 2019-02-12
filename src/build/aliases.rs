use util::hash;

#[derive(Copy, Clone, Debug)]
pub struct Alias<'help> {
    id: u64,
    name: &'help str,
    visible: bool
}

impl<'help> Alias<'help> {
    fn new<S: AsRef<str> + 'help>(name: S) -> Self {
        let name = name.as_ref();
        Alias {
            id: hash(name),
            name,
            visible: false
        }
    }

    fn visible(mut self) -> Self {
        self.visible = true;
        self
    }

    fn hidden(mut self) -> Self {
        self.visible = false;
        self
    }

    fn set_visible(mut self, visible: bool) -> Self {
        self.visible = visible;
        self
    }
}

impl<'help> Default for Alias<'help> {
    fn default() -> Self {
        Alias {
            id: 0,
            name: "",
            visible: false
        }
    }
}

#[derive(Default)]
pub struct Aliases<'help>(Vec<Alias<'help>>);

impl<'help> Aliases<'help> {
    pub fn add_visible<S: AsRef<&'help str>>(&mut self, name: S) {
        self.0.push(Alias::new(name));
    }
    pub fn add_hidden<S: AsRef<&'help str>>(&mut self, name: S) {
        self.0.push(Alias::new(name).hidden());
    }
}