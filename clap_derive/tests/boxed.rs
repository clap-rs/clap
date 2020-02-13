use clap::Clap;

#[test]
fn boxed_flatten_subcommand() {
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

    assert_eq!(
        Opt {
            sub: Box::new(Sub::Flame {
                arg: Box::new(Ext { arg: 1 })
            })
        },
        Opt::parse_from(&["test", "flame", "1"])
    );
}
