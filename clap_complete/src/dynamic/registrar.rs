/// Register shell autocomplete file.
pub trait Registrar {
    /// The recommended file name for the registration code
    fn file_name(&self, name: &str) -> String;
    /// Register for completions
    fn write_registration(
        &self,
        name: &str,
        bin: &str,
        completer: &str,
        buf: &mut dyn std::io::Write,
    ) -> Result<(), std::io::Error>;
}
