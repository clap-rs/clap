#[macro_use]
extern crate clap;

use clap::{App, Arg};
use std::vec::Vec;

arg_enum!{
    #[derive(Debug)]
    enum Val1 {
        ValOne,
        ValTwo
    }
}
arg_enum!{
    #[derive(Debug)]
    pub enum Val2 {
        ValOne,
        ValTwo
    }
}
arg_enum!{
    enum Val3 {
        ValOne,
        ValTwo
    }
}
arg_enum!{
    pub enum Val4 {
        ValOne,
        ValTwo
    }
}

#[test]
#[cfg_attr(feature = "lints", allow(single_match))]
fn test_enums() {
    let v1_lower = "valone";
    let v1_camel = "ValOne";

    let v1_lp = v1_lower.parse::<Val1>().unwrap();
    let v1_cp = v1_camel.parse::<Val1>().unwrap();
    match v1_lp {
        Val1::ValOne => (),
        _ => panic!("Val1 didn't parse correctly"),
    }
    match v1_cp {
        Val1::ValOne => (),
        _ => panic!("Val1 didn't parse correctly"),
    }
    let v1_lp = v1_lower.parse::<Val2>().unwrap();
    let v1_cp = v1_camel.parse::<Val2>().unwrap();
    match v1_lp {
        Val2::ValOne => (),
        _ => panic!("Val1 didn't parse correctly"),
    }
    match v1_cp {
        Val2::ValOne => (),
        _ => panic!("Val1 didn't parse correctly"),
    }
    let v1_lp = v1_lower.parse::<Val3>().unwrap();
    let v1_cp = v1_camel.parse::<Val3>().unwrap();
    match v1_lp {
        Val3::ValOne => (),
        _ => panic!("Val1 didn't parse correctly"),
    }
    match v1_cp {
        Val3::ValOne => (),
        _ => panic!("Val1 didn't parse correctly"),
    }
    let v1_lp = v1_lower.parse::<Val4>().unwrap();
    let v1_cp = v1_camel.parse::<Val4>().unwrap();
    match v1_lp {
        Val4::ValOne => (),
        _ => panic!("Val1 didn't parse correctly"),
    }
    match v1_cp {
        Val4::ValOne => (),
        _ => panic!("Val1 didn't parse correctly"),
    }
}

#[test]
fn create_app() {
    let _ =
        App::new("test").version("1.0").author("kevin").about("does awesome things").get_matches();
}

#[test]
fn add_multiple_arg() {
    let _ = App::new("test")
                .args(&mut [
                    Arg::with_name("test").short("s"),
                    Arg::with_name("test2").short("l")])
                .get_matches();
}
