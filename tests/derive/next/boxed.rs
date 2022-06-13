use clap::{Args, Parser, Subcommand};

#[derive(Parser, PartialEq, Debug)]
struct Opt {
    #[clap(subcommand)]
    sub: Box<Sub>,
}

#[derive(Subcommand, PartialEq, Debug)]
enum Sub {
    Flame {
        #[clap(flatten)]
        arg: Box<Ext>,
    },
}

#[derive(Args, PartialEq, Debug)]
struct Ext {
    #[clap(value_parser)]
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
        Opt::try_parse_from(&["test", "flame", "1"]).unwrap()
    );
}

#[test]
fn update_boxed_flatten_subcommand() {
    let mut opt = Opt::try_parse_from(&["test", "flame", "1"]).unwrap();

    opt.update_from(&["test", "flame", "42"]);

    assert_eq!(
        Opt {
            sub: Box::new(Sub::Flame {
                arg: Box::new(Ext { arg: 42 })
            })
        },
        opt
    );
}
