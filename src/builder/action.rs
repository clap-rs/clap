/// Behavior of arguments when they are encountered while parsing
#[derive(Clone, Debug)]
#[non_exhaustive]
pub(crate) enum ArgAction {
    StoreValue,
    IncOccurrence,
    Help,
    Version,
}
