use clap::Clap;
use std::str::FromStr;
use std::sync::atomic::{AtomicU32, Ordering::SeqCst};

static NUM_CALLS: AtomicU32 = AtomicU32::new(0);

#[derive(Debug)]
struct Effectful {}

impl FromStr for Effectful {
    type Err = String;

    fn from_str(_s: &str) -> Result<Self, Self::Err> {
        NUM_CALLS.fetch_add(1, SeqCst);
        Ok(Self {})
    }
}

#[derive(Clap, Debug)]
struct Opt {
    effectful: Effectful,
}

#[test]
fn effectful() {
    let _opt = Opt::parse_from(&["test", "arg"]);
    assert_eq!(NUM_CALLS.load(SeqCst), 1);
}
