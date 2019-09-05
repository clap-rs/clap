use bstr::{BStr, B};

pub struct Alias<'help> {
    pub(crate) name: &'help BStr,
    pub(crate) vis: bool,
}

impl<'help> Alias<'help> {
    pub fn visible<T: ?Sized + AsRef<[u8]>>(n: T) -> Self {
        Alias {
            name: B(n),
            vis: true,
        }
    }
    pub fn hidden(n: &'help str) -> Self {
        Alias {
            name: n,
            vis: false,
        }
    }
}
