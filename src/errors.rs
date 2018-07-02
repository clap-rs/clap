// Copyright 2018 Guillaume Pinot (@TeXitoi) <texitoi@texitoi.eu>,
// Kevin Knapp (@kbknapp) <kbknapp@gmail.com>, and
// Andrew Hobden (@hoverbear) <andrew@hoverbear.org>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
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
