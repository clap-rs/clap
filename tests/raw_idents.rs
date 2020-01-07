use clap::Clap;

#[test]
fn raw_idents() {
    #[derive(Clap, Debug, PartialEq)]
    struct Opt {
        #[clap(short, long)]
        r#type: Vec<String>,
    }

    assert_eq!(
        Opt {
            r#type: vec!["long".into(), "short".into()]
        },
        Opt::parse_from(&["test", "--type", "long", "-t", "short"])
    );
}
