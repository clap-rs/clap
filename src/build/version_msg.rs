#[derive(Copy, Clone, Default, Debug)]
pub struct VersionMsg<'help> {
    // Version string to be displayed after the `name` when `-V` used, or `--version` is used if
    // `long_version` isn't defined.
    #[doc(hidden)]
    pub version: Option<&'help str>,
    // Version string to be displayed after the `name` when `--version` is used
    #[doc(hidden)]
    pub long_version: Option<&'help str>,
}