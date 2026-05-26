#![cfg(feature = "markdown")]

use crate::common;

fn markdown_command() -> clap::Command {
    clap::Command::new("my-app")
        .version("1.0")
        .about("A **bold** and *italic* app with `code`")
        .long_about(
            "This is a tool for testing **markdown** rendering.\n\
             \n\
             ## Features\n\
             \n\
             - Supports **bold** text\n\
             - Supports *italic* text\n\
             - Supports `code` spans\n\
             \n\
             ## Examples\n\
             \n\
             ```\n\
             my-app --flag value\n\
             ```\n\
             \n\
             For more info see [the docs](https://example.com).",
        )
        .arg(
            clap::Arg::new("input")
                .help("The *input* file to process (use `stdin` for **standard input**)"),
        )
        .arg(
            clap::Arg::new("format")
                .long("format")
                .short('f')
                .help("Output **format** to use")
                .action(clap::ArgAction::Set),
        )
        .after_help(
            "## Notes\n\
             \n\
             > This is a blockquote with **bold** text.\n\
             \n\
             1. First step\n\
             2. Second step\n\
             3. Third step",
        )
}

#[test]
fn markdown_formatting() {
    let cmd = markdown_command();
    common::assert_matches(snapbox::file!["../snapshots/markdown_formatting.roff"], cmd);
}
