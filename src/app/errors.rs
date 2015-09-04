pub enum ClapErrorType {
    Matches,
    Opt,
    None
}

pub struct ClapError {
	pub error: String,
	pub error_type: ClapErrorType,
}