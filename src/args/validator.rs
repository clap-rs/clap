/// Trait which must be implemented for objects that validate arguments.
pub trait Validator : Clone {
    /// Function to validate an Argument
    fn validate(&self, &String) -> Result<(), String>;
}
