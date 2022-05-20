// Copyright 2018 Guillaume Pinot (@TeXitoi) <texitoi@texitoi.eu>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed

#[cfg(feature = "derive")]
#[rustversion::attr(any(not(stable), before(1.56), since(1.57)), ignore)] // MSRV
#[test]
fn ui() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/derive_ui/*.rs");
    #[cfg(feature = "unstable-v4")]
    t.compile_fail("tests/derive_ui/next/*.rs");
    #[cfg(not(feature = "unstable-v4"))]
    t.compile_fail("tests/derive_ui/stable/*.rs");
}
