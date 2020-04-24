// Copyright 2018 Guillaume Pinot (@TeXitoi) <texitoi@texitoi.eu>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed

#[test]
fn ui() {
    if std::env::var("CLAP_TRYBUILD_RUN") != Ok("yes".into()) {
        // This is not the latest rustc. Skipping trybuild tests
        // because they likely won't match.
        return;
    }

    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/*.rs");
}
