use clap::{Args, Parser, Subcommand};

#[derive(Parser, PartialEq, Debug)]
struct Opt {
    #[command(subcommand)]
    sub: Box<Sub>,
}

#[derive(Subcommand, PartialEq, Debug)]
enum Sub {
    Flame {
        #[command(flatten)]
        arg: Box<Ext>,
    },
}

#[derive(Args, PartialEq, Debug)]
struct Ext {
    arg: u32,
}

#[test]
fn boxed_flatten_subcommand() {
    assert_eq!(
        Opt {
            sub: Box::new(Sub::Flame {
                arg: Box::new(Ext { arg: 1 })
            })
        },
        Opt::try_parse_from(["test", "flame", "1"]).unwrap()
    );
}

#[test]
fn update_boxed_flatten_subcommand() {
    let mut opt = Opt::try_parse_from(["test", "flame", "1"]).unwrap();

    opt.try_update_from(["test", "flame", "42"]).unwrap();

    assert_eq!(
        Opt {
            sub: Box::new(Sub::Flame {
                arg: Box::new(Ext { arg: 42 })
            })
        },
        opt
    );
}
