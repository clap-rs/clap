struct RelationArg<'a, T> {
    args: Vec<T>,
    value: Option<&'a [u8]>,
    modifier: RelationModifier
}

enum RelationModifier {
    All,
    Any,
    None
}

enum RelationKind<'a, T> {
    Present(RelationArg<'a, T>),
    NotPresent(RelationArg<'a, T>),
    None
}

struct Relations<'a, T>(Vec<RelationKind<'a, T>>);

impl<'a, T> Relation<'a, T> where T: Eq {
    fn is_sat(&self, &[T]) -> bool {
        for r in self.
    }
}