use args::settings::ArgSettings;

pub trait AnyArg<'n> {
    fn name(&self) -> &'n str;
    fn overrides(&self) -> Option<&[&'n str]>;
    fn is_set(&self, &ArgSettings) -> bool;
    fn set(&mut self, &ArgSettings);
}
