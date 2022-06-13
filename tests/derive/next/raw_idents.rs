use clap::Parser;

#[test]
fn raw_idents() {
    #[derive(Parser, Debug, PartialEq)]
    struct Opt {
        #[clap(short, long, value_parser)]
        r#type: String,
    }

    assert_eq!(
        Opt {
            r#type: "long".into()
        },
        Opt::try_parse_from(&["test", "--type", "long"]).unwrap()
    );

    assert_eq!(
        Opt {
            r#type: "short".into()
        },
        Opt::try_parse_from(&["test", "-t", "short"]).unwrap()
    );
}
