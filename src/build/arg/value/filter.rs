pub struct Filter {
    terminator: Terminator,
    #[doc(hidden)]
    validator: Option<Validator>,
    #[doc(hidden)]
    validator_os: Option<ValidatorOs>,
    possible_values: PossibleValues,
    hyphen_values: bool,
}