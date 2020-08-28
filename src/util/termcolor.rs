#[derive(Debug, Copy, Clone, PartialEq)]
pub(crate) enum ColorChoice {
    Auto,
    Always,
    Never,
}

#[derive(Debug)]
pub(crate) enum Color {
    Green,
    Yellow,
    Red,
}
