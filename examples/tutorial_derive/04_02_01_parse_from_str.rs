use std::fmt::{Display, Formatter};
use std::marker::PhantomData;

#[derive(Clone, Debug)]
struct Foo {
    v: u32
}

impl std::str::FromStr for Foo {
    type Err = NotSendSyncError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s=="error" {
            Err(NotSendSyncError{_pd: PhantomData })
        } else {
            Ok(Foo{v: 42})
        }
    }
}

#[derive(Debug)]
struct NotSendSyncError {
    _pd: PhantomData<std::cell::RefCell<()>>,
}

impl Display for NotSendSyncError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{:?}", self)
    }
}

impl std::error::Error for NotSendSyncError {

}

#[derive(clap::Parser, Debug, Clone)]
#[structopt(name = "use FromStr")]
struct Config {
    //#[arg(long, value_parser=clap::value_parser!(Foo))]
    #[arg(long)]
    foo: Foo
}

fn main() {
    let config = <Config as clap::Parser>::parse();
    println!("foo.v={}", config.foo.v);
}
