use args::{ArgSettings, Arg, ArgFlags, ArgKind};

#[derive(Debug, Clone)]
pub struct Base<'a, 'b>
    where 'a: 'b
{
    pub name: &'a str,
    pub id: usize,
    pub kind: ArgKind,
    pub help: Option<&'b str>,
    pub blacklist: Option<Vec<&'a str>>,
    pub settings: ArgFlags,
    pub r_unless: Option<Vec<&'a str>>,
    pub overrides: Option<Vec<&'a str>>,
    pub groups: Option<Vec<&'a str>>,
    pub requires: Option<Vec<(Option<&'b str>, &'a str)>>,
}

impl<'n, 'e> Default for Base<'n, 'e> {
    fn default() -> Self {
        Base {
            name: "",
            id: 0,
            kind: ArgKind::Pos,
            help: None,
            blacklist: None,
            settings: ArgFlags::new(),
            r_unless: None,
            overrides: None,
            requires: None,
            groups: None,
        }
    }
}

impl<'n, 'e> Base<'n, 'e> {
    pub fn new(name: &'n str) -> Self { Base { name: name, ..Default::default() } }

    pub fn set(&mut self, s: ArgSettings) { self.settings.set(s); }
}

impl<'n, 'e, 'z> From<&'z Arg<'n, 'e>> for Base<'n, 'e> {
    fn from(a: &'z Arg<'n, 'e>) -> Self {
        Base {
            name: a.name,
            help: a.help,
            id: 0,
            kind: ArgKind::Pos,
            blacklist: a.blacklist.clone(),
            settings: a.settings,
            r_unless: a.r_unless.clone(),
            overrides: a.overrides.clone(),
            requires: a.requires.clone(),
            groups: a.groups.clone(),
        }
    }
}
