// Copyright 2018 Guillaume Pinot (@TeXitoi) <texitoi@texitoi.eu>,
// Kevin Knapp (@kbknapp) <kbknapp@gmail.com>, and
// Ana Hobden (@hoverbear) <operator@hoverbear.org>
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

use crate::utils;

use clap::{Args, Parser, Subcommand};

#[derive(Parser, PartialEq, Eq, Debug)]
enum Opt {
    /// Fetch stuff from GitHub
    Fetch {
        #[arg(long)]
        all: bool,
        /// Overwrite local branches.
        #[arg(short, long)]
        force: bool,

        repo: String,
    },

    Add {
        #[arg(short, long)]
        interactive: bool,
        #[arg(short, long)]
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
        Opt::try_parse_from(["test", "fetch", "--all", "origin"]).unwrap()
    );
    assert_eq!(
        Opt::Fetch {
            all: false,
            force: true,
            repo: "origin".to_string()
        },
        Opt::try_parse_from(["test", "fetch", "-f", "origin"]).unwrap()
    );
}

#[test]
fn test_add() {
    assert_eq!(
        Opt::Add {
            interactive: false,
            verbose: false
        },
        Opt::try_parse_from(["test", "add"]).unwrap()
    );
    assert_eq!(
        Opt::Add {
            interactive: true,
            verbose: true
        },
        Opt::try_parse_from(["test", "add", "-i", "-v"]).unwrap()
    );
}

#[test]
fn test_no_parse() {
    let result = Opt::try_parse_from(["test", "badcmd", "-i", "-v"]);
    assert!(result.is_err());

    let result = Opt::try_parse_from(["test", "add", "--badoption"]);
    assert!(result.is_err());

    let result = Opt::try_parse_from(["test"]);
    assert!(result.is_err());
}

#[derive(Parser, PartialEq, Eq, Debug)]
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
        Opt2::try_parse_from(["test", "do-something", "blah"]).unwrap()
    );
}

#[derive(Parser, PartialEq, Eq, Debug)]
enum Opt3 {
    Add,
    Init,
    Fetch,
}

#[test]
fn test_null_commands() {
    assert_eq!(Opt3::Add, Opt3::try_parse_from(["test", "add"]).unwrap());
    assert_eq!(Opt3::Init, Opt3::try_parse_from(["test", "init"]).unwrap());
    assert_eq!(
        Opt3::Fetch,
        Opt3::try_parse_from(["test", "fetch"]).unwrap()
    );
}

#[derive(Parser, PartialEq, Eq, Debug)]
#[command(about = "Not shown")]
struct Add {
    file: String,
}
/// Not shown
#[derive(Parser, PartialEq, Eq, Debug)]
struct Fetch {
    remote: String,
}
#[derive(Parser, PartialEq, Eq, Debug)]
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
        Opt4::try_parse_from(["test", "add", "f"]).unwrap()
    );
    assert_eq!(Opt4::Init, Opt4::try_parse_from(["test", "init"]).unwrap());
    assert_eq!(
        Opt4::Fetch(Fetch {
            remote: "origin".to_string()
        }),
        Opt4::try_parse_from(["test", "fetch", "origin"]).unwrap()
    );

    let output = utils::get_long_help::<Opt4>();

    assert!(output.contains("download history from remote"));
    assert!(output.contains("Add a file"));
    assert!(!output.contains("Not shown"));
}

#[test]
fn global_passed_down() {
    #[derive(Debug, PartialEq, Eq, Parser)]
    struct Opt {
        #[arg(global = true, long)]
        other: bool,
        #[command(subcommand)]
        sub: Subcommands,
    }

    #[derive(Debug, PartialEq, Eq, Subcommand)]
    enum Subcommands {
        Add,
        Global(GlobalCmd),
    }

    #[derive(Debug, PartialEq, Eq, Args)]
    struct GlobalCmd {
        #[arg(from_global)]
        other: bool,
    }

    assert_eq!(
        Opt::try_parse_from(["test", "global"]).unwrap(),
        Opt {
            other: false,
            sub: Subcommands::Global(GlobalCmd { other: false })
        }
    );

    assert_eq!(
        Opt::try_parse_from(["test", "global", "--other"]).unwrap(),
        Opt {
            other: true,
            sub: Subcommands::Global(GlobalCmd { other: true })
        }
    );
}

#[test]
fn external_subcommand() {
    #[derive(Debug, PartialEq, Eq, Parser)]
    struct Opt {
        #[command(subcommand)]
        sub: Subcommands,
    }

    #[derive(Debug, PartialEq, Eq, Subcommand)]
    enum Subcommands {
        Add,
        Remove,
        #[command(external_subcommand)]
        Other(Vec<String>),
    }

    assert_eq!(
        Opt::try_parse_from(["test", "add"]).unwrap(),
        Opt {
            sub: Subcommands::Add
        }
    );

    assert_eq!(
        Opt::try_parse_from(["test", "remove"]).unwrap(),
        Opt {
            sub: Subcommands::Remove
        }
    );

    assert!(Opt::try_parse_from(["test"]).is_err());

    assert_eq!(
        Opt::try_parse_from(["test", "git", "status"]).unwrap(),
        Opt {
            sub: Subcommands::Other(vec!["git".into(), "status".into()])
        }
    );
}

#[test]
fn external_subcommand_os_string() {
    use std::ffi::OsString;

    #[derive(Debug, PartialEq, Eq, Parser)]
    struct Opt {
        #[command(subcommand)]
        sub: Subcommands,
    }

    #[derive(Debug, PartialEq, Eq, Subcommand)]
    enum Subcommands {
        #[command(external_subcommand)]
        Other(Vec<OsString>),
    }

    assert_eq!(
        Opt::try_parse_from(["test", "git", "status"]).unwrap(),
        Opt {
            sub: Subcommands::Other(vec!["git".into(), "status".into()])
        }
    );

    assert!(Opt::try_parse_from(["test"]).is_err());
}

#[test]
fn external_subcommand_optional() {
    #[derive(Debug, PartialEq, Eq, Parser)]
    struct Opt {
        #[command(subcommand)]
        sub: Option<Subcommands>,
    }

    #[derive(Debug, PartialEq, Eq, Subcommand)]
    enum Subcommands {
        #[command(external_subcommand)]
        Other(Vec<String>),
    }

    assert_eq!(
        Opt::try_parse_from(["test", "git", "status"]).unwrap(),
        Opt {
            sub: Some(Subcommands::Other(vec!["git".into(), "status".into()]))
        }
    );

    assert_eq!(Opt::try_parse_from(["test"]).unwrap(), Opt { sub: None });
}

#[test]
fn enum_in_enum_subsubcommand() {
    #[derive(Parser, Debug, PartialEq, Eq)]
    pub enum Opt {
        #[command(alias = "l")]
        List,
        #[command(subcommand, alias = "d")]
        Daemon(DaemonCommand),
    }

    #[derive(Subcommand, Debug, PartialEq, Eq)]
    pub enum DaemonCommand {
        Start,
        Stop,
    }

    let result = Opt::try_parse_from(["test"]);
    assert!(result.is_err());

    let result = Opt::try_parse_from(["test", "list"]).unwrap();
    assert_eq!(Opt::List, result);

    let result = Opt::try_parse_from(["test", "l"]).unwrap();
    assert_eq!(Opt::List, result);

    let result = Opt::try_parse_from(["test", "daemon"]);
    assert!(result.is_err());

    let result = Opt::try_parse_from(["test", "daemon", "start"]).unwrap();
    assert_eq!(Opt::Daemon(DaemonCommand::Start), result);

    let result = Opt::try_parse_from(["test", "d", "start"]).unwrap();
    assert_eq!(Opt::Daemon(DaemonCommand::Start), result);
}

#[test]
fn update_subcommands() {
    #[derive(Parser, PartialEq, Eq, Debug)]
    enum Opt {
        Command1(Command1),
        Command2(Command2),
    }

    #[derive(Parser, PartialEq, Eq, Debug)]
    struct Command1 {
        arg1: i32,

        arg2: i32,
    }

    #[derive(Parser, PartialEq, Eq, Debug)]
    struct Command2 {
        arg2: i32,
    }

    // Full subcommand update
    let mut opt = Opt::Command1(Command1 { arg1: 12, arg2: 14 });
    opt.try_update_from(["test", "command1", "42", "44"])
        .unwrap();
    assert_eq!(
        Opt::try_parse_from(["test", "command1", "42", "44"]).unwrap(),
        opt
    );

    // Partial subcommand update
    let mut opt = Opt::Command1(Command1 { arg1: 12, arg2: 14 });
    opt.try_update_from(["test", "command1", "42"]).unwrap();
    assert_eq!(
        Opt::try_parse_from(["test", "command1", "42", "14"]).unwrap(),
        opt
    );

    // Change subcommand
    let mut opt = Opt::Command1(Command1 { arg1: 12, arg2: 14 });
    opt.try_update_from(["test", "command2", "43"]).unwrap();
    assert_eq!(
        Opt::try_parse_from(["test", "command2", "43"]).unwrap(),
        opt
    );
}

#[test]
fn update_subcommands_explicit_required() {
    #[derive(Parser, PartialEq, Eq, Debug)]
    #[command(subcommand_required = true)]
    enum Opt {
        Command1(Command1),
        Command2(Command2),
    }

    #[derive(Parser, PartialEq, Eq, Debug)]
    struct Command1 {
        arg1: i32,

        arg2: i32,
    }

    #[derive(Parser, PartialEq, Eq, Debug)]
    struct Command2 {
        arg2: i32,
    }

    // Full subcommand update
    let mut opt = Opt::Command1(Command1 { arg1: 12, arg2: 14 });
    opt.try_update_from(["test"]).unwrap();
    assert_eq!(Opt::Command1(Command1 { arg1: 12, arg2: 14 }), opt);
}

#[test]
fn update_sub_subcommands() {
    #[derive(Parser, PartialEq, Eq, Debug)]
    enum Opt {
        #[command(subcommand)]
        Child1(Child1),
        #[command(subcommand)]
        Child2(Child2),
    }

    #[derive(Subcommand, PartialEq, Eq, Debug)]
    enum Child1 {
        Command1(Command1),
        Command2(Command2),
    }

    #[derive(Subcommand, PartialEq, Eq, Debug)]
    enum Child2 {
        Command1(Command1),
        Command2(Command2),
    }

    #[derive(Args, PartialEq, Eq, Debug)]
    struct Command1 {
        arg1: i32,

        arg2: i32,
    }

    #[derive(Args, PartialEq, Eq, Debug)]
    struct Command2 {
        arg2: i32,
    }

    // Full subcommand update
    let mut opt = Opt::Child1(Child1::Command1(Command1 { arg1: 12, arg2: 14 }));
    opt.try_update_from(["test", "child1", "command1", "42", "44"])
        .unwrap();
    assert_eq!(
        Opt::try_parse_from(["test", "child1", "command1", "42", "44"]).unwrap(),
        opt
    );

    // Partial subcommand update
    let mut opt = Opt::Child1(Child1::Command1(Command1 { arg1: 12, arg2: 14 }));
    opt.try_update_from(["test", "child1", "command1", "42"])
        .unwrap();
    assert_eq!(
        Opt::try_parse_from(["test", "child1", "command1", "42", "14"]).unwrap(),
        opt
    );

    // Partial subcommand update
    let mut opt = Opt::Child1(Child1::Command1(Command1 { arg1: 12, arg2: 14 }));
    opt.try_update_from(["test", "child1", "command2", "43"])
        .unwrap();
    assert_eq!(
        Opt::try_parse_from(["test", "child1", "command2", "43"]).unwrap(),
        opt
    );

    // Change subcommand
    let mut opt = Opt::Child1(Child1::Command1(Command1 { arg1: 12, arg2: 14 }));
    opt.try_update_from(["test", "child2", "command2", "43"])
        .unwrap();
    assert_eq!(
        Opt::try_parse_from(["test", "child2", "command2", "43"]).unwrap(),
        opt
    );
}

#[test]
fn update_ext_subcommand() {
    #[derive(Parser, PartialEq, Eq, Debug)]
    enum Opt {
        Command1(Command1),
        Command2(Command2),
        #[command(external_subcommand)]
        Ext(Vec<String>),
    }

    #[derive(Args, PartialEq, Eq, Debug)]
    struct Command1 {
        arg1: i32,

        arg2: i32,
    }

    #[derive(Args, PartialEq, Eq, Debug)]
    struct Command2 {
        arg2: i32,
    }

    // Full subcommand update
    let mut opt = Opt::Ext(vec!["12".into(), "14".into()]);
    opt.try_update_from(["test", "ext", "42", "44"]).unwrap();
    assert_eq!(
        Opt::try_parse_from(["test", "ext", "42", "44"]).unwrap(),
        opt
    );

    // No partial subcommand update
    let mut opt = Opt::Ext(vec!["12".into(), "14".into()]);
    opt.try_update_from(["test", "ext", "42"]).unwrap();
    assert_eq!(Opt::try_parse_from(["test", "ext", "42"]).unwrap(), opt);

    // Change subcommand
    let mut opt = Opt::Ext(vec!["12".into(), "14".into()]);
    opt.try_update_from(["test", "command2", "43"]).unwrap();
    assert_eq!(
        Opt::try_parse_from(["test", "command2", "43"]).unwrap(),
        opt
    );

    let mut opt = Opt::Command1(Command1 { arg1: 12, arg2: 14 });
    opt.try_update_from(["test", "ext", "42", "44"]).unwrap();
    assert_eq!(
        Opt::try_parse_from(["test", "ext", "42", "44"]).unwrap(),
        opt
    );
}
#[test]
fn subcommand_name_not_literal() {
    fn get_name() -> &'static str {
        "renamed"
    }

    #[derive(Parser, PartialEq, Eq, Debug)]
    struct Opt {
        #[command(subcommand)]
        subcmd: SubCmd,
    }

    #[derive(Subcommand, PartialEq, Eq, Debug)]
    enum SubCmd {
        #[command(name = get_name())]
        SubCmd1,
    }

    assert!(Opt::try_parse_from(["test", "renamed"]).is_ok());
}

#[test]
fn skip_subcommand() {
    #[derive(Debug, PartialEq, Eq, Parser)]
    struct Opt {
        #[command(subcommand)]
        sub: Subcommands,
    }

    #[derive(Debug, PartialEq, Eq, Subcommand)]
    enum Subcommands {
        Add,
        Remove,

        #[allow(dead_code)]
        #[command(skip)]
        Skip,

        #[allow(dead_code)]
        #[command(skip)]
        Other(Other),
    }

    #[allow(dead_code)]
    #[derive(Debug, PartialEq, Eq)]
    enum Other {
        One,
        Twp,
    }

    assert!(Subcommands::has_subcommand("add"));
    assert!(Subcommands::has_subcommand("remove"));
    assert!(!Subcommands::has_subcommand("skip"));
    assert!(!Subcommands::has_subcommand("other"));

    assert_eq!(
        Opt::try_parse_from(["test", "add"]).unwrap(),
        Opt {
            sub: Subcommands::Add
        }
    );

    assert_eq!(
        Opt::try_parse_from(["test", "remove"]).unwrap(),
        Opt {
            sub: Subcommands::Remove
        }
    );

    let res = Opt::try_parse_from(["test", "skip"]);
    assert_eq!(
        res.unwrap_err().kind(),
        clap::error::ErrorKind::InvalidSubcommand,
    );

    let res = Opt::try_parse_from(["test", "other"]);
    assert_eq!(
        res.unwrap_err().kind(),
        clap::error::ErrorKind::InvalidSubcommand,
    );
}

#[test]
fn built_in_subcommand_escaped() {
    #[derive(Debug, PartialEq, Eq, Parser)]
    enum Command {
        Install {
            arg: Option<String>,
        },
        #[command(external_subcommand)]
        Custom(Vec<String>),
    }

    assert_eq!(
        Command::try_parse_from(["test", "install", "arg"]).unwrap(),
        Command::Install {
            arg: Some(String::from("arg"))
        }
    );
    assert_eq!(
        Command::try_parse_from(["test", "--", "install"]).unwrap(),
        Command::Custom(vec![String::from("install")])
    );
    assert_eq!(
        Command::try_parse_from(["test", "--", "install", "arg"]).unwrap(),
        Command::Custom(vec![String::from("install"), String::from("arg")])
    );
}
