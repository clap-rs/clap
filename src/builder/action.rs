/// Behavior of arguments when they are encountered while parsing
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum Action {
    StoreValue,
    Flag,
    Help,
    Version,
}
