use proc_macro;

// Unfortunately `proc_macro` and `syn` don't have a good error handling story.
error_chain! {
    errors {
        WrongBodyType(expected: &'static str) {
            description("The wrong type for the derived structure was provided.")
            display("Wrong type for derive structure: {:?} expected", expected)
        }
        ParseError(error: String) {
            description("A parsing failure.")
            display("A parsing failure happened: {:?}", error)
        }
        ProcLexError(error: proc_macro::LexError) {
            description("A proc_macro lex failure.")
            display("A proc_macro lex failure happened: {:?}", error)
        }
    }
}