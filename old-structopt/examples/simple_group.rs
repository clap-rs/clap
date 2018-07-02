#[macro_use]
extern crate structopt;

use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct Opt {
    /// Set a custom HTTP verb
    #[structopt(long = "method", group = "verb")]
    method: Option<String>,
    /// HTTP GET; default if no other HTTP verb is selected
    #[structopt(long = "get", group = "verb")]
    get: bool,
    /// HTTP HEAD
    #[structopt(long = "head", group = "verb")]
    head: bool,
    /// HTTP POST
    #[structopt(long = "post", group = "verb")]
    post: bool,
    /// HTTP PUT
    #[structopt(long = "put", group = "verb")]
    put: bool,
    /// HTTP DELETE
    #[structopt(long = "delete", group = "verb")]
    delete: bool,
}

fn main() {
    let opt = Opt::from_args();
    println!("{:?}", opt);
}
