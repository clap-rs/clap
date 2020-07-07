use clap::Clap;

#[derive(Clap, PartialEq, Debug)]
struct Opt {
    #[clap(subcommand)]
    sub: Box<Sub>,
}

#[derive(Clap, PartialEq, Debug)]
enum Sub {
    Flame {
        #[clap(flatten)]
        arg: Box<Ext>,
    },
}

#[derive(Clap, PartialEq, Debug)]
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
        Opt::parse_from(&["test", "flame", "1"])
    );
}

#[test]
fn update_boxed_flatten_subcommand() {
    let mut opt = Opt::parse_from(&["test", "flame", "1"]);

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
