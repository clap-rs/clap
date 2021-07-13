// Copyright 2018 Guillaume Pinot (@TeXitoi) <texitoi@texitoi.eu>,
// Kevin Knapp (@kbknapp) <kbknapp@gmail.com>, and
// Andrew Hobden (@hoverbear) <andrew@hoverbear.org>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
//
// This work was derived from Structopt (https://github.com/TeXitoi/structopt)
// commit#ea76fa1b1b273e65e3b0b1046643715b49bec51f which is licensed under the
// MIT/Apache 2.0 license.

mod utils;

use clap::Parser;
use utils::*;

#[derive(Parser, PartialEq, Debug)]
enum Opt {
    /// Fetch stuff from GitHub
    Fetch {
        #[clap(long)]
        all: bool,
        #[clap(short, long)]
        /// Overwrite local branches.
        force: bool,
        repo: String,
    },

    Add {
        #[clap(short, long)]
        interactive: bool,
        #[clap(short, long)]
        verbose: bool,
    },
}

#[test]
fn test_fetch() {
    assert_eq!(
        Opt::Fetch {
            all: true,
            force: false,
            repo: "origin".to_string()
        },
        Opt::parse_from(&["test", "fetch", "--all", "origin"])
    );
    assert_eq!(
        Opt::Fetch {
            all: false,
            force: true,
            repo: "origin".to_string()
        },
        Opt::parse_from(&["test", "fetch", "-f", "origin"])
    );
}

#[test]
fn test_add() {
    assert_eq!(
        Opt::Add {
            interactive: false,
            verbose: false
        },
        Opt::parse_from(&["test", "add"])
    );
    assert_eq!(
        Opt::Add {
            interactive: true,
            verbose: true
        },
        Opt::parse_from(&["test", "add", "-i", "-v"])
    );
}

#[test]
fn test_no_parse() {
    let result = Opt::try_parse_from(&["test", "badcmd", "-i", "-v"]);
    assert!(result.is_err());

    let result = Opt::try_parse_from(&["test", "add", "--badoption"]);
    assert!(result.is_err());

    let result = Opt::try_parse_from(&["test"]);
    assert!(result.is_err());
}

#[derive(Parser, PartialEq, Debug)]
enum Opt2 {
    DoSomething { arg: String },
}

#[test]
/// This test is specifically to make sure that hyphenated subcommands get
/// processed correctly.
fn test_hyphenated_subcommands() {
    assert_eq!(
        Opt2::DoSomething {
            arg: "blah".to_string()
        },
        Opt2::parse_from(&["test", "do-something", "blah"])
    );
}

#[derive(Parser, PartialEq, Debug)]
enum Opt3 {
    Add,
    Init,
    Fetch,
}

#[test]
fn test_null_commands() {
    assert_eq!(Opt3::Add, Opt3::parse_from(&["test", "add"]));
    assert_eq!(Opt3::Init, Opt3::parse_from(&["test", "init"]));
    assert_eq!(Opt3::Fetch, Opt3::parse_from(&["test", "fetch"]));
}

#[derive(Parser, PartialEq, Debug)]
#[clap(about = "Not shown")]
struct Add {
    file: String,
}
/// Not shown
#[derive(Parser, PartialEq, Debug)]
struct Fetch {
    remote: String,
}
#[derive(Parser, PartialEq, Debug)]
enum Opt4 {
    // Not shown
    /// Add a file
    Add(Add),
    Init,
    /// download history from remote
    Fetch(Fetch),
}

#[test]
fn test_tuple_commands() {
    assert_eq!(
        Opt4::Add(Add {
            file: "f".to_string()
        }),
        Opt4::parse_from(&["test", "add", "f"])
    );
    assert_eq!(Opt4::Init, Opt4::parse_from(&["test", "init"]));
    assert_eq!(
        Opt4::Fetch(Fetch {
            remote: "origin".to_string()
        }),
        Opt4::parse_from(&["test", "fetch", "origin"])
    );

    let output = get_long_help::<Opt4>();

    assert!(output.contains("download history from remote"));
    assert!(output.contains("Add a file"));
    assert!(!output.contains("Not shown"));
}

#[test]
fn global_passed_down() {
    #[derive(Debug, PartialEq, Parser)]
    struct Opt {
        #[clap(global = true, long)]
        other: bool,
        #[clap(subcommand)]
        sub: Subcommands,
    }

    #[derive(Debug, PartialEq, Parser)]
    enum Subcommands {
        Add,
        Global(GlobalCmd),
    }

    #[derive(Debug, PartialEq, Parser)]
    struct GlobalCmd {
        #[clap(from_global)]
        other: bool,
    }

    assert_eq!(
        Opt::parse_from(&["test", "global"]),
        Opt {
            other: false,
            sub: Subcommands::Global(GlobalCmd { other: false })
        }
    );

    assert_eq!(
        Opt::parse_from(&["test", "global", "--other"]),
        Opt {
            other: true,
            sub: Subcommands::Global(GlobalCmd { other: true })
        }
    );
}

#[test]
fn external_subcommand() {
    #[derive(Debug, PartialEq, Parser)]
    struct Opt {
        #[clap(subcommand)]
        sub: Subcommands,
    }

    #[derive(Debug, PartialEq, Parser)]
    enum Subcommands {
        Add,
        Remove,
        #[clap(external_subcommand)]
        Other(Vec<String>),
    }

    assert_eq!(
        Opt::parse_from(&["test", "add"]),
        Opt {
            sub: Subcommands::Add
        }
    );

    assert_eq!(
        Opt::parse_from(&["test", "remove"]),
        Opt {
            sub: Subcommands::Remove
        }
    );

    assert!(Opt::try_parse_from(&["test"]).is_err());

    assert_eq!(
        Opt::try_parse_from(&["test", "git", "status"]).unwrap(),
        Opt {
            sub: Subcommands::Other(vec!["git".into(), "status".into()])
        }
    );
}

#[test]
fn external_subcommand_os_string() {
    use std::ffi::OsString;

    #[derive(Debug, PartialEq, Parser)]
    struct Opt {
        #[clap(subcommand)]
        sub: Subcommands,
    }

    #[derive(Debug, PartialEq, Parser)]
    enum Subcommands {
        #[clap(external_subcommand)]
        Other(Vec<OsString>),
    }

    assert_eq!(
        Opt::try_parse_from(&["test", "git", "status"]).unwrap(),
        Opt {
            sub: Subcommands::Other(vec!["git".into(), "status".into()])
        }
    );

    assert!(Opt::try_parse_from(&["test"]).is_err());
}

#[test]
fn external_subcommand_optional() {
    #[derive(Debug, PartialEq, Parser)]
    struct Opt {
        #[clap(subcommand)]
        sub: Option<Subcommands>,
    }

    #[derive(Debug, PartialEq, Parser)]
    enum Subcommands {
        #[clap(external_subcommand)]
        Other(Vec<String>),
    }

    assert_eq!(
        Opt::try_parse_from(&["test", "git", "status"]).unwrap(),
        Opt {
            sub: Some(Subcommands::Other(vec!["git".into(), "status".into()]))
        }
    );

    assert_eq!(Opt::try_parse_from(&["test"]).unwrap(), Opt { sub: None });
}

// #[test]
// #[ignore] // FIXME (@CreepySkeleton)
// fn enum_in_enum_subsubcommand() {
//     #[derive(Parser, Debug, PartialEq)]
//     pub enum Opt {
//         Daemon(DaemonCommand),
//     }

//     #[derive(Parser, Debug, PartialEq)]
//     pub enum DaemonCommand {
//         Start,
//         Stop,
//     }

//     let result = Opt::try_parse_from(&["test"]);
//     assert!(result.is_err());

//     let result = Opt::try_parse_from(&["test", "daemon"]);
//     assert!(result.is_err());

//     let result = Opt::parse_from(&["test", "daemon", "start"]);
//     assert_eq!(Opt::Daemon(DaemonCommand::Start), result);
// }

// WTF??? It tests that `flatten` is essentially a synonym
// for `subcommand`. It's not documented and I doubt it was supposed to
// work this way.
// #[test]
// #[ignore]
// fn flatten_enum() {
//     #[derive(Parser, Debug, PartialEq)]
//     struct Opt {
//         #[clap(flatten)]
//         sub_cmd: SubCmd,
//     }
//     #[derive(Parser, Debug, PartialEq)]
//     enum SubCmd {
//         Foo,
//         Bar,
//     }

//     assert!(Opt::try_parse_from(&["test"]).is_err());
//     assert_eq!(
//         Opt::parse_from(&["test", "foo"]),
//         Opt {
//             sub_cmd: SubCmd::Foo
//         }
//     );
// }
