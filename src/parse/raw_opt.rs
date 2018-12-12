struct RawOpt<'a> {
    raw_key: &'a OsStr,
    key: Key,
    value: RawValue<'a>,
}