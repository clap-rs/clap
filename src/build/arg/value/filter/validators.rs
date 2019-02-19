pub type Validator = Rc<Fn(String) -> Result<(), String>>;
pub type ValidatorOs = Rc<Fn(&OsStr) -> Result<(), String>>;


