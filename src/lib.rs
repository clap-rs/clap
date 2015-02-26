#![feature(collections, core)]

pub use argmatches::ArgMatches;
pub use arg::Arg;
pub use app::App;

mod app;
mod argmatches;
mod arg;
mod args;

#[test]
fn it_works() {
}
