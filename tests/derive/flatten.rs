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

use clap::{Args, Parser, Subcommand};
use snapbox::assert_data_eq;
use snapbox::prelude::*;
use snapbox::str;

use crate::utils;

#[test]
fn flatten() {
    #[derive(Args, PartialEq, Debug)]
    struct Common {
        arg: i32,
    }

    #[derive(Parser, PartialEq, Debug)]
    struct Opt {
        #[command(flatten)]
        common: Common,
    }
    assert_eq!(
        Opt {
            common: Common { arg: 42 }
        },
        Opt::try_parse_from(["test", "42"]).unwrap()
    );
    assert!(Opt::try_parse_from(["test"]).is_err());
    assert!(Opt::try_parse_from(["test", "42", "24"]).is_err());
}

#[cfg(debug_assertions)]
#[test]
#[should_panic]
fn flatten_twice() {
    #[derive(Args, PartialEq, Debug)]
    struct Common {
        arg: i32,
    }

    #[derive(Parser, PartialEq, Debug)]
    struct Opt {
        #[command(flatten)]
        c1: Common,
        // Defines "arg" twice, so this should not work.
        #[command(flatten)]
        c2: Common,
    }
    Opt::try_parse_from(["test", "42", "43"]).unwrap();
}

#[test]
fn flatten_in_subcommand() {
    #[derive(Args, PartialEq, Debug)]
    struct Common {
        arg: i32,
    }

    #[derive(Args, PartialEq, Debug)]
    struct Add {
        #[arg(short)]
        interactive: bool,
        #[command(flatten)]
        common: Common,
    }

    #[derive(Parser, PartialEq, Debug)]
    enum Opt {
        Fetch {
            #[arg(short)]
            all: bool,
            #[command(flatten)]
            common: Common,
        },

        Add(Add),
    }

    assert_eq!(
        Opt::Fetch {
            all: false,
            common: Common { arg: 42 }
        },
        Opt::try_parse_from(["test", "fetch", "42"]).unwrap()
    );
    assert_eq!(
        Opt::Add(Add {
            interactive: true,
            common: Common { arg: 43 }
        }),
        Opt::try_parse_from(["test", "add", "-i", "43"]).unwrap()
    );
}

#[test]
fn update_args_with_flatten() {
    #[derive(Args, PartialEq, Debug)]
    struct Common {
        arg: i32,
    }

    #[derive(Parser, PartialEq, Debug)]
    struct Opt {
        #[command(flatten)]
        common: Common,
    }

    let mut opt = Opt {
        common: Common { arg: 42 },
    };
    opt.try_update_from(["test"]).unwrap();
    assert_eq!(Opt::try_parse_from(["test", "42"]).unwrap(), opt);

    let mut opt = Opt {
        common: Common { arg: 42 },
    };
    opt.try_update_from(["test", "52"]).unwrap();
    assert_eq!(Opt::try_parse_from(["test", "52"]).unwrap(), opt);
}

#[derive(Subcommand, PartialEq, Debug)]
enum BaseCli {
    Command1(Command1),
}

#[derive(Args, PartialEq, Debug)]
struct Command1 {
    arg1: i32,

    arg2: i32,
}

#[derive(Args, PartialEq, Debug)]
struct Command2 {
    arg2: i32,
}

#[derive(Parser, PartialEq, Debug)]
enum Opt {
    #[command(flatten)]
    BaseCli(BaseCli),
    Command2(Command2),
}

#[test]
fn merge_subcommands_with_flatten() {
    assert_eq!(
        Opt::BaseCli(BaseCli::Command1(Command1 { arg1: 42, arg2: 44 })),
        Opt::try_parse_from(["test", "command1", "42", "44"]).unwrap()
    );
    assert_eq!(
        Opt::Command2(Command2 { arg2: 43 }),
        Opt::try_parse_from(["test", "command2", "43"]).unwrap()
    );
}

#[test]
fn update_subcommands_with_flatten() {
    let mut opt = Opt::BaseCli(BaseCli::Command1(Command1 { arg1: 12, arg2: 14 }));
    opt.try_update_from(["test", "command1", "42", "44"])
        .unwrap();
    assert_eq!(
        Opt::try_parse_from(["test", "command1", "42", "44"]).unwrap(),
        opt
    );

    let mut opt = Opt::BaseCli(BaseCli::Command1(Command1 { arg1: 12, arg2: 14 }));
    opt.try_update_from(["test", "command1", "42"]).unwrap();
    assert_eq!(
        Opt::try_parse_from(["test", "command1", "42", "14"]).unwrap(),
        opt
    );

    let mut opt = Opt::BaseCli(BaseCli::Command1(Command1 { arg1: 12, arg2: 14 }));
    opt.try_update_from(["test", "command2", "43"]).unwrap();
    assert_eq!(
        Opt::try_parse_from(["test", "command2", "43"]).unwrap(),
        opt
    );
}

#[test]
fn flatten_with_doc_comment() {
    #[derive(Args, PartialEq, Debug)]
    struct Common {
        /// This is an arg. Arg means "argument". Command line argument.
        arg: i32,
    }

    #[derive(Parser, PartialEq, Debug)]
    struct Opt {
        /// The very important comment that clippy had me put here.
        /// It knows better.
        #[command(flatten)]
        common: Common,
    }
    assert_eq!(
        Opt {
            common: Common { arg: 42 }
        },
        Opt::try_parse_from(["test", "42"]).unwrap()
    );

    let help = utils::get_help::<Opt>();
    assert_data_eq!(help, str![[r#"
Usage: clap <ARG>

Arguments:
  <ARG>  This is an arg. Arg means "argument". Command line argument

Options:
  -h, --help  Print help

"#]].raw());
}

#[test]
fn docstrings_ordering_with_multiple_command() {
    /// This is the docstring for Flattened
    #[derive(Args)]
    struct Flattened {
        #[arg(long)]
        foo: bool,
    }

    /// This is the docstring for Command
    #[derive(Parser)]
    struct Command {
        #[command(flatten)]
        flattened: Flattened,
    }

    let short_help = utils::get_help::<Command>();

    assert_data_eq!(short_help, str![[r#"
This is the docstring for Command

Usage: clap [OPTIONS]

Options:
      --foo   
  -h, --help  Print help

"#]].raw());
}

#[test]
fn docstrings_ordering_with_multiple_clap_partial() {
    /// This is the docstring for Flattened
    #[derive(Args)]
    struct Flattened {
        #[arg(long)]
        foo: bool,
    }

    #[derive(Parser)]
    struct Command {
        #[command(flatten)]
        flattened: Flattened,
    }

    let short_help = utils::get_help::<Command>();

    assert_data_eq!(short_help, str![[r#"
This is the docstring for Flattened

Usage: clap [OPTIONS]

Options:
      --foo   
  -h, --help  Print help

"#]].raw());
}

#[test]
#[should_panic = "cannot `#[flatten]` an `Option<Args>` with `#[group(skip)]`"]
fn flatten_skipped_group() {
    #[derive(clap::Parser, Debug)]
    struct Cli {
        #[clap(flatten)]
        args: Option<Args>,
    }

    #[derive(clap::Args, Debug)]
    #[group(skip)]
    struct Args {
        #[clap(short)]
        param: bool,
    }

    Cli::try_parse_from(["test"]).unwrap();
}

#[cfg(feature = "string")]
mod flatten_prefix {
    use super::*;

    #[test]
    fn basic_prefix() {
        #[derive(Args, PartialEq, Debug)]
        struct StorageOptions {
            #[arg(long)]
            host: String,
            #[arg(long)]
            username: String,
        }

        #[derive(Parser, PartialEq, Debug)]
        struct Cli {
            #[command(flatten = "source-")]
            source: StorageOptions,
        }

        assert_eq!(
            Cli {
                source: StorageOptions {
                    host: "localhost".into(),
                    username: "admin".into(),
                }
            },
            Cli::try_parse_from([
                "test",
                "--source-host",
                "localhost",
                "--source-username",
                "admin"
            ])
            .unwrap()
        );
    }

    #[test]
    fn duplicate_flatten_with_different_prefixes() {
        #[derive(Args, PartialEq, Debug)]
        struct StorageOptions {
            #[arg(long)]
            host: String,
            #[arg(long)]
            username: String,
        }

        #[derive(Parser, PartialEq, Debug)]
        struct Cli {
            #[command(flatten = "source-")]
            source: StorageOptions,
            #[command(flatten = "dest-")]
            dest: StorageOptions,
        }

        assert_eq!(
            Cli {
                source: StorageOptions {
                    host: "src.example.com".into(),
                    username: "reader".into(),
                },
                dest: StorageOptions {
                    host: "dst.example.com".into(),
                    username: "writer".into(),
                },
            },
            Cli::try_parse_from([
                "test",
                "--source-host",
                "src.example.com",
                "--source-username",
                "reader",
                "--dest-host",
                "dst.example.com",
                "--dest-username",
                "writer",
            ])
            .unwrap()
        );
    }

    #[test]
    #[should_panic(expected = "short flags cannot be prefixed")]
    fn prefix_with_short_flag_panics() {
        #[derive(Args, PartialEq, Debug)]
        struct Opts {
            #[arg(short, long)]
            verbose: bool,
        }

        #[derive(Parser, PartialEq, Debug)]
        struct Cli {
            #[command(flatten = "src-")]
            opts: Opts,
        }

        // The panic happens during command construction (augment_args)
        let _ = Cli::try_parse_from(["test"]);
    }

    #[test]
    fn prefix_help_shows_prefixed_flags() {
        #[derive(Args, PartialEq, Debug)]
        struct StorageOptions {
            #[arg(long)]
            host: String,
        }

        #[derive(Parser, PartialEq, Debug)]
        struct Cli {
            #[command(flatten = "source-")]
            source: StorageOptions,
        }

        let help = utils::get_help::<Cli>();
        assert!(
            help.contains("--source-host"),
            "Help should contain --source-host, got:\n{help}"
        );
    }

    #[test]
    fn prefix_with_optional_flatten() {
        #[derive(Args, PartialEq, Debug)]
        struct StorageOptions {
            #[arg(long)]
            host: Option<String>,
        }

        #[derive(Parser, PartialEq, Debug)]
        struct Cli {
            #[command(flatten = "src-")]
            source: Option<StorageOptions>,
        }

        // Without args, the optional should be None
        let cli = Cli::try_parse_from(["test"]).unwrap();
        assert_eq!(cli.source, None);

        // With args, the optional should be Some
        let cli = Cli::try_parse_from(["test", "--src-host", "localhost"]).unwrap();
        assert_eq!(
            cli.source,
            Some(StorageOptions {
                host: Some("localhost".into())
            })
        );
    }
}
