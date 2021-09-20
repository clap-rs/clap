// Because its evaluated before the tests compilation here I can set env
// variables and the macros later will expand with them.
pub fn setup() {
    std::env::set_var("EXAMPLE_NAME", "AppName");
}
