use std::str;
use std::io::Cursor;

extern crate clap;
extern crate regex;

use regex::Regex;

use clap::App;

fn compare<S, S2>(l: S, r: S2) -> bool
where
    S: AsRef<str>,
    S2: AsRef<str>,
{
    let re = Regex::new("\x1b[^m]*m").unwrap();
    // Strip out any mismatching \r character on windows that might sneak in on either side
    let ls = l.as_ref().trim().replace("\r", "");
    let rs = r.as_ref().trim().replace("\r", "");
    let left = re.replace_all(&*ls, "");
    let right = re.replace_all(&*rs, "");
    let b = left == right;
    if !b {
        println!("");
        println!("--> left");
        println!("{}", left);
        println!("--> right");
        println!("{}", right);
        println!("--")
    }
    b
}

pub fn compare_output(l: App, args: &str, right: &str, stderr: bool) -> bool {
    let mut buf = Cursor::new(Vec::with_capacity(50));
    let res = l.try_get_matches_from(args.split(' ').collect::<Vec<_>>());
    let err = res.unwrap_err();
    err.write_to(&mut buf).unwrap();
    let content = buf.into_inner();
    let left = String::from_utf8(content).unwrap();
    assert_eq!(
        stderr,
        err.use_stderr(),
        "Should Use STDERR failed. Should be {} but is {}",
        stderr,
        err.use_stderr()
    );
    compare(left, right)
}

pub fn compare_output2(l: App, args: &str, right1: &str, right2: &str, stderr: bool) -> bool {
    let mut buf = Cursor::new(Vec::with_capacity(50));
    let res = l.try_get_matches_from(args.split(' ').collect::<Vec<_>>());
    let err = res.unwrap_err();
    err.write_to(&mut buf).unwrap();
    let content = buf.into_inner();
    let left = String::from_utf8(content).unwrap();
    assert_eq!(stderr, err.use_stderr());
    compare(&*left, right1) || compare(&*left, right2)
}
