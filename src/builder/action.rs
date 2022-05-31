/// Behavior of arguments when they are encountered while parsing
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub(crate) enum ArgAction {
    StoreValue,
    Flag,
    Help,
    Version,
}
