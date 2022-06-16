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

use clap::{CommandFactory, Parser, ValueEnum};

#[test]
fn doc_comments() {
    /// Lorem ipsum
    #[derive(Parser, PartialEq, Debug)]
    struct LoremIpsum {
        /// Fooify a bar
        /// and a baz
        #[clap(short, long)]
        foo: bool,
    }

    let help = utils::get_long_help::<LoremIpsum>();
    assert!(help.contains("Lorem ipsum"));
    assert!(help.contains("Fooify a bar and a baz"));
}

#[test]
fn help_is_better_than_comments() {
    /// Lorem ipsum
    #[derive(Parser, PartialEq, Debug)]
    #[clap(name = "lorem-ipsum", about = "Dolor sit amet")]
    struct LoremIpsum {
        /// Fooify a bar
        #[clap(short, long, help = "DO NOT PASS A BAR UNDER ANY CIRCUMSTANCES")]
        foo: bool,
    }

    let help = utils::get_long_help::<LoremIpsum>();
    assert!(help.contains("Dolor sit amet"));
    assert!(!help.contains("Lorem ipsum"));
    assert!(help.contains("DO NOT PASS A BAR"));
}

#[test]
fn empty_line_in_doc_comment_is_double_linefeed() {
    /// Foo.
    ///
    /// Bar
    #[derive(Parser, PartialEq, Debug)]
    #[clap(name = "lorem-ipsum")]
    struct LoremIpsum {}

    let help = utils::get_long_help::<LoremIpsum>();
    assert!(help.starts_with("lorem-ipsum \nFoo.\n\nBar\n\nUSAGE:"));
}

#[test]
fn field_long_doc_comment_both_help_long_help() {
    /// Lorem ipsumclap
    #[derive(Parser, PartialEq, Debug)]
    #[clap(name = "lorem-ipsum", about = "Dolor sit amet")]
    struct LoremIpsum {
        /// Dot is removed from multiline comments.
        ///
        /// Long help
        #[clap(long)]
        foo: bool,

        /// Dot is removed from one short comment.
        #[clap(long)]
        bar: bool,
    }

    let short_help = utils::get_help::<LoremIpsum>();
    let long_help = utils::get_long_help::<LoremIpsum>();

    assert!(short_help.contains("Dot is removed from one short comment"));
    assert!(!short_help.contains("Dot is removed from one short comment."));
    assert!(short_help.contains("Dot is removed from multiline comments"));
    assert!(!short_help.contains("Dot is removed from multiline comments."));
    assert!(long_help.contains("Long help"));
    assert!(!short_help.contains("Long help"));
}

#[test]
fn top_long_doc_comment_both_help_long_help() {
    /// Lorem ipsumclap
    #[derive(Parser, Debug)]
    #[clap(name = "lorem-ipsum", about = "Dolor sit amet")]
    struct LoremIpsum {
        #[clap(subcommand)]
        foo: SubCommand,
    }

    #[derive(Parser, Debug)]
    pub enum SubCommand {
        /// DO NOT PASS A BAR UNDER ANY CIRCUMSTANCES
        ///
        /// Or something else
        Foo {
            #[clap(help = "foo")]
            bars: String,
        },
    }

    let short_help = utils::get_help::<LoremIpsum>();
    let long_help = utils::get_subcommand_long_help::<LoremIpsum>("foo");

    assert!(!short_help.contains("Or something else"));
    assert!(long_help.contains("DO NOT PASS A BAR UNDER ANY CIRCUMSTANCES"));
    assert!(long_help.contains("Or something else"));
}

#[test]
fn verbatim_doc_comment() {
    /// DANCE!
    ///
    ///                    ()
    ///                    |
    ///               (   ()   )
    ///     ) ________    //  )
    ///  ()  |\       \  //
    /// ( \\__ \ ______\//
    ///    \__) |       |
    ///      |  |       |
    ///       \ |       |
    ///        \|_______|
    ///        //    \\
    ///       ((     ||
    ///        \\    ||
    ///      ( ()    ||
    ///       (      () ) )
    #[derive(Parser, Debug)]
    #[clap(verbatim_doc_comment)]
    struct SeeFigure1 {
        #[clap(long)]
        foo: bool,
    }

    let help = utils::get_long_help::<SeeFigure1>();
    let sample = r#"
                   ()
                   |
              (   ()   )
    ) ________    //  )
 ()  |\       \  //
( \\__ \ ______\//
   \__) |       |
     |  |       |
      \ |       |
       \|_______|
       //    \\
      ((     ||
       \\    ||
     ( ()    ||
      (      () ) )"#;

    assert!(help.contains(sample))
}

#[test]
fn verbatim_doc_comment_field() {
    #[derive(Parser, Debug)]
    struct Command {
        /// This help ends in a period.
        #[clap(long, verbatim_doc_comment)]
        foo: bool,
        /// This help does not end in a period.
        #[clap(long)]
        bar: bool,
    }

    let help = utils::get_long_help::<Command>();

    assert!(help.contains("This help ends in a period."));
    assert!(help.contains("This help does not end in a period"));
}

#[test]
fn multiline_separates_default() {
    #[derive(Parser, Debug)]
    struct Command {
        /// Multiline
        ///
        /// Doc comment
        #[clap(long, default_value = "x")]
        x: String,
    }

    let help = utils::get_long_help::<Command>();
    assert!(!help.contains("Doc comment [default"));
    assert!(help.lines().any(|s| s.trim().starts_with("[default")));

    // The short help should still have the default on the same line
    let help = utils::get_help::<Command>();
    assert!(help.contains("Multiline [default"));
}

#[test]
fn argenum_multiline_doc_comment() {
    #[derive(ValueEnum, Clone)]
    enum LoremIpsum {
        /// Multiline
        ///
        /// Doc comment
        Bar,
    }
}

#[test]
fn doc_comment_about_handles_both_abouts() {
    /// Opts doc comment summary
    #[derive(Parser, Debug)]
    pub struct Opts {
        #[clap(subcommand)]
        pub cmd: Sub,
    }

    /// Sub doc comment summary
    ///
    /// Sub doc comment body
    #[derive(Parser, PartialEq, Eq, Debug)]
    pub enum Sub {
        Compress { output: String },
    }

    let cmd = Opts::command();
    assert_eq!(cmd.get_about(), Some("Opts doc comment summary"));
    // clap will fallback to `about` on `None`.  The main care about is not providing a `Sub` doc
    // comment.
    assert_eq!(cmd.get_long_about(), None);
}
