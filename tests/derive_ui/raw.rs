// Copyright 2018 Guillaume Pinot (@TeXitoi) <texitoi@texitoi.eu>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use clap::Parser;

#[derive(Parser, Debug)]
struct Opt {
    #[clap(raw(ignore_case = "true"))]
    s: String,
}

#[derive(Parser, Debug)]
struct Opt2 {
    #[clap(raw(requires_if = r#""one", "two""#))]
    s: String,
}
fn main() {
    let opt = Opt::parse();
    println!("{:?}", opt);
}
