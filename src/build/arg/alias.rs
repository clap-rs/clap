pub(crate) struct Alias<'help> {
    name: &'help str,
    vis: bool,
}

impl<'help> Alias<'help> {
    fn visible(n: &'help str) -> Self { Alias { name: n, vis: true } }
    fn hidden(n: &'help str) -> Self {
        Alias {
            name: n,
            vis: false,
        }
    }
}
