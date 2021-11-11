pub(crate) enum ArgPredicate<'a> {
    IsPresent,
    Equals(&'a str),
}
