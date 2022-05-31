/// Behavior of arguments when they are encountered while parsing
#[derive(Clone, Debug)]
#[non_exhaustive]
pub(crate) enum ArgAction {
    StoreValue,
    IncOccurrence,
    Help,
    Version,
}

impl ArgAction {
    pub(crate) fn takes_value(&self) -> bool {
        match self {
            Self::StoreValue => true,
            Self::IncOccurrence => false,
            Self::Help => false,
            Self::Version => false,
        }
    }
}
