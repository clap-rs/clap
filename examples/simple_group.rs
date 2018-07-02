#[macro_use]
extern crate clap;

use clap::Clap;

#[derive(Clap, Debug)]
struct Opt {
    /// Set a custom HTTP verb
    #[clap(long = "method", group = "verb")]
    method: Option<String>,
    /// HTTP GET; default if no other HTTP verb is selected
    #[clap(long = "get", group = "verb")]
    get: bool,
    /// HTTP HEAD
    #[clap(long = "head", group = "verb")]
    head: bool,
    /// HTTP POST
    #[clap(long = "post", group = "verb")]
    post: bool,
    /// HTTP PUT
    #[clap(long = "put", group = "verb")]
    put: bool,
    /// HTTP DELETE
    #[clap(long = "delete", group = "verb")]
    delete: bool,
}

fn main() {
    let opt = Opt::parse();
    println!("{:?}", opt);
}
