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

use clap::{CommandFactory, Parser, Subcommand, ValueEnum};
use snapbox::assert_data_eq;
use snapbox::prelude::*;
use snapbox::str;

#[test]
fn doc_comments() {
    /// Lorem ipsum
    #[derive(Parser, PartialEq, Debug)]
    struct LoremIpsum {
        /// Fooify a bar
        /// and a baz
        #[arg(short, long)]
        foo: bool,
    }

    let help = utils::get_long_help::<LoremIpsum>();
    assert_data_eq!(help, str![[r#"
Lorem ipsum

Usage: clap [OPTIONS]

Options:
  -f, --foo
          Fooify a bar and a baz

  -h, --help
          Print help

"#]].raw());
}

#[test]
fn help_is_better_than_comments() {
    /// Lorem ipsum
    #[derive(Parser, PartialEq, Debug)]
    #[command(name = "lorem-ipsum", about = "Dolor sit amet")]
    struct LoremIpsum {
        /// Fooify a bar
        #[arg(short, long, help = "DO NOT PASS A BAR UNDER ANY CIRCUMSTANCES")]
        foo: bool,
    }

    let help = utils::get_long_help::<LoremIpsum>();
    assert_data_eq!(help, str![[r#"
Dolor sit amet

Usage: lorem-ipsum [OPTIONS]

Options:
  -f, --foo
          DO NOT PASS A BAR UNDER ANY CIRCUMSTANCES

  -h, --help
          Print help

"#]].raw());
}

#[test]
fn empty_line_in_doc_comment_is_double_linefeed() {
    /// Foo.
    ///
    /// Bar
    #[derive(Parser, PartialEq, Debug)]
    #[command(name = "lorem-ipsum")]
    struct LoremIpsum {}

    let help = utils::get_long_help::<LoremIpsum>();
    assert!(help.starts_with(
        "\
Foo.

Bar

Usage:"
    ));
}

#[test]
fn field_long_doc_comment_both_help_long_help() {
    /// Lorem ipsumclap
    #[derive(Parser, PartialEq, Debug)]
    #[command(name = "lorem-ipsum", about = "Dolor sit amet")]
    struct LoremIpsum {
        /// Dot is removed from multiline comments.
        ///
        /// Long help
        #[arg(long)]
        foo: bool,

        /// Dot is removed from one short comment.
        #[arg(long)]
        bar: bool,
    }

    let short_help = utils::get_help::<LoremIpsum>();
    let long_help = utils::get_long_help::<LoremIpsum>();

    assert_data_eq!(short_help, str![[r#"
Dolor sit amet

Usage: lorem-ipsum [OPTIONS]

Options:
      --foo   Dot is removed from multiline comments
      --bar   Dot is removed from one short comment
  -h, --help  Print help (see more with '--help')

"#]].raw());
    assert_data_eq!(long_help, str![[r#"
Dolor sit amet

Usage: lorem-ipsum [OPTIONS]

Options:
      --foo
          Dot is removed from multiline comments.
          
          Long help

      --bar
          Dot is removed from one short comment

  -h, --help
          Print help (see a summary with '-h')

"#]].raw());
}

#[test]
fn top_long_doc_comment_both_help_long_help() {
    /// Lorem ipsumclap
    #[derive(Parser, Debug)]
    #[command(name = "lorem-ipsum", about = "Dolor sit amet")]
    struct LoremIpsum {
        #[command(subcommand)]
        foo: SubCommand,
    }

    #[derive(Parser, Debug)]
    pub(crate) enum SubCommand {
        /// DO NOT PASS A BAR UNDER ANY CIRCUMSTANCES
        ///
        /// Or something else
        Foo {
            #[arg(help = "foo")]
            bars: String,
        },
    }

    let short_help = utils::get_help::<LoremIpsum>();
    let long_help = utils::get_subcommand_long_help::<LoremIpsum>("foo");

    assert_data_eq!(short_help, str![[r#"
Dolor sit amet

Usage: lorem-ipsum <COMMAND>

Commands:
  foo   DO NOT PASS A BAR UNDER ANY CIRCUMSTANCES
  help  Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help

"#]].raw());
    assert_data_eq!(long_help, str![[r#"
DO NOT PASS A BAR UNDER ANY CIRCUMSTANCES

Or something else

Usage: foo <BARS>

Arguments:
  <BARS>
          foo

Options:
  -h, --help
          Print help (see a summary with '-h')

"#]].raw());
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
    #[command(verbatim_doc_comment)]
    struct SeeFigure1 {
        #[arg(long)]
        foo: bool,
    }

    let help = utils::get_long_help::<SeeFigure1>();
    assert_data_eq!(help, str![[r#"
DANCE!

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
      (      () ) )

Usage: clap [OPTIONS]

Options:
      --foo
          

  -h, --help
          Print help (see a summary with '-h')

"#]].raw());
}

#[test]
fn verbatim_doc_comment_field() {
    #[derive(Parser, Debug)]
    struct Command {
        /// This help ends in a period.
        #[arg(long, verbatim_doc_comment)]
        foo: bool,
        /// This help does not end in a period.
        #[arg(long)]
        bar: bool,
    }

    let help = utils::get_long_help::<Command>();

    assert_data_eq!(help, str![[r#"
Usage: clap [OPTIONS]

Options:
      --foo
          This help ends in a period.

      --bar
          This help does not end in a period

  -h, --help
          Print help

"#]].raw());
}

#[test]
fn multiline_separates_default() {
    #[derive(Parser, Debug)]
    struct Command {
        /// Multiline
        ///
        /// Doc comment
        #[arg(long, default_value = "x")]
        x: String,
    }

    let help = utils::get_long_help::<Command>();
    assert_data_eq!(help, str![[r#"
Usage: clap [OPTIONS]

Options:
      --x <X>
          Multiline
          
          Doc comment
          
          [default: x]

  -h, --help
          Print help (see a summary with '-h')

"#]].raw());

    // The short help should still have the default on the same line
    let help = utils::get_help::<Command>();
    assert_data_eq!(help, str![[r#"
Usage: clap [OPTIONS]

Options:
      --x <X>  Multiline [default: x]
  -h, --help   Print help (see more with '--help')

"#]].raw());
}

#[test]
fn value_enum_multiline_doc_comment() {
    #[derive(Parser, Debug)]
    struct Command {
        x: LoremIpsum,
    }

    #[derive(ValueEnum, Clone, PartialEq, Debug)]
    enum LoremIpsum {
        /// Doc comment summary
        ///
        /// The doc comment body is ignored
        Bar,
    }

    let help = utils::get_long_help::<Command>();

    // There is no long help text for possible values. The long help only contains the summary.
    assert_data_eq!(help, str![[r#"
Usage: clap <X>

Arguments:
  <X>
          Possible values:
          - bar: Doc comment summary

Options:
  -h, --help
          Print help (see a summary with '-h')

"#]].raw());
}

#[test]
fn doc_comment_about_handles_both_abouts() {
    /// Opts doc comment summary
    #[derive(Parser, Debug)]
    pub(crate) struct Opts {
        #[command(subcommand)]
        pub(crate) cmd: Sub,
    }

    /// Sub doc comment summary
    ///
    /// Sub doc comment body
    #[derive(Parser, PartialEq, Eq, Debug)]
    pub(crate) enum Sub {
        Compress { output: String },
    }

    let cmd = Opts::command();
    assert_eq!(
        cmd.get_about().map(|s| s.to_string()),
        Some("Opts doc comment summary".to_owned())
    );
    // clap will fallback to `about` on `None`.  The main care about is not providing a `Sub` doc
    // comment.
    assert_eq!(cmd.get_long_about(), None);
}

#[test]
fn respect_subcommand_doc_comment() {
    #[derive(Parser, Debug)]
    pub(crate) enum Cmd {
        /// For child
        #[command(subcommand)]
        Child(Child),
    }

    #[derive(Subcommand, Debug)]
    pub(crate) enum Child {
        One,
        Twp,
    }

    let output = str![[r#"
Usage: cmd <COMMAND>

Commands:
  child  For child
  help   Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help

"#]];
    utils::assert_output::<Cmd>("cmd --help", output, false);
}

#[test]
fn force_long_help() {
    /// Lorem ipsum
    #[derive(Parser, PartialEq, Debug)]
    struct LoremIpsum {
        /// Fooify a bar
        /// and a baz.
        #[arg(short, long, long_help)]
        foo: bool,
    }

    let help = utils::get_long_help::<LoremIpsum>();
    assert_data_eq!(help, str![[r#"
Lorem ipsum

Usage: clap [OPTIONS]

Options:
  -f, --foo
          Fooify a bar and a baz.

  -h, --help
          Print help (see a summary with '-h')

"#]].raw());
}
