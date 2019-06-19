// Used to simulate a fairly large number of options/flags and parsing with thousands of positional
// args
//
// CLI used is adapted from ripgrep 48a8a3a691220f9e5b2b08f4051abe8655ea7e8a

#![feature(test)]

extern crate clap;
extern crate test;
#[macro_use]
extern crate lazy_static;

use clap::{App, AppSettings, Arg, ArgSettings};

use std::collections::HashMap;
use std::io::Cursor;
use test::Bencher;

#[bench]
fn build_app_short(b: &mut Bencher) {
    b.iter(|| app_short());
}

#[bench]
fn build_app_long(b: &mut Bencher) {
    b.iter(|| app_long());
}

#[bench]
fn build_help_short(b: &mut Bencher) {
    let app = app_short();
    b.iter(|| build_help(&app));
}

#[bench]
fn build_help_long(b: &mut Bencher) {
    let app = app_long();
    b.iter(|| build_help(&app));
}

#[bench]
fn parse_clean(b: &mut Bencher) {
    b.iter(|| app_short().get_matches_from(vec!["rg", "pat"]));
}

#[bench]
fn parse_complex(b: &mut Bencher) {
    b.iter(|| {
        app_short().get_matches_from(vec![
            "rg",
            "pat",
            "-cFlN",
            "-pqr=some",
            "--null",
            "--no-filename",
            "--no-messages",
            "-SH",
            "-C5",
            "--follow",
            "-e some",
        ])
    });
}

#[bench]
fn parse_lots(b: &mut Bencher) {
    b.iter(|| {
        app_short().get_matches_from(vec![
            "rg", "pat", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some", "some", "some", "some", "some",
            "some", "some", "some", "some", "some", "some", "some",
        ])
    });
}

const ABOUT: &'static str = "
ripgrep (rg) recursively searches your current directory for a regex pattern.

ripgrep's regex engine uses finite automata and guarantees linear time
searching. Because of this, features like backreferences and arbitrary
lookaround are not supported.

Project home page: https://github.com/BurntSushi/ripgrep

Use -h for short descriptions and --help for more details.";

const USAGE: &'static str = "
    rg [OPTIONS] <pattern> [<path> ...]
    rg [OPTIONS] [-e PATTERN | -f FILE ]... [<path> ...]
    rg [OPTIONS] --files [<path> ...]
    rg [OPTIONS] --type-list";

const TEMPLATE: &'static str = "\
{bin} {version}
{author}
{about}

USAGE:{usage}

ARGS:
{positionals}

OPTIONS:
{unified}";

/// Build a clap application with short help strings.
pub fn app_short() -> App<'static, 'static> {
    app(false, |k| USAGES[k].short)
}

/// Build a clap application with long help strings.
pub fn app_long() -> App<'static, 'static> {
    app(true, |k| USAGES[k].long)
}

/// Build the help text of an application.
fn build_help(app: &App) -> String {
    let mut buf = Cursor::new(Vec::with_capacity(50));
    app.write_help(&mut buf).unwrap();
    let content = buf.into_inner();
    String::from_utf8(content).unwrap()
}

/// Build a clap application parameterized by usage strings.
///
/// The function given should take a clap argument name and return a help
/// string. `app` will panic if a usage string is not defined.
///
/// This is an intentionally stand-alone module so that it can be used easily
/// in a `build.rs` script to build shell completion files.
fn app<F>(next_line_help: bool, doc: F) -> App<'static, 'static>
where
    F: Fn(&'static str) -> &'static str,
{
    let arg = |name| {
        Arg::with_name(name)
            .help(doc(name))
            .next_line_help(next_line_help)
    };
    let flag = |name| arg(name).long(name);

    App::new("ripgrep")
        .author("BurntSushi") // simulating since it's only a bench
        .version("0.4.0") // Simulating
        .about(ABOUT)
        .max_term_width(100)
        .setting(AppSettings::UnifiedHelpMessage)
        .usage(USAGE)
        .template(TEMPLATE)
        // Handle help/version manually to make their output formatting
        // consistent with short/long views.
        .arg(arg("help-short").short("h"))
        .arg(flag("help"))
        .arg(flag("version").short("V"))
        // First, set up primary positional/flag arguments.
        .arg(arg("pattern").required_unless_one(&[
            "file",
            "files",
            "help-short",
            "help",
            "regexp",
            "type-list",
            "version",
        ]))
        .arg(arg("path").multiple(true))
        .arg(
            flag("regexp")
                .short("e")
                .takes_value(true)
                .multiple(true)
                .number_of_values(1)
                .set(ArgSettings::AllowLeadingHyphen)
                .value_name("pattern"),
        )
        .arg(
            flag("files")
                // This should also conflict with `pattern`, but the first file
                // path will actually be in `pattern`.
                .conflicts_with_all(&["file", "regexp", "type-list"]),
        )
        .arg(flag("type-list").conflicts_with_all(&["file", "files", "pattern", "regexp"]))
        // Second, set up common flags.
        .arg(flag("text").short("a"))
        .arg(flag("count").short("c"))
        .arg(
            flag("color")
                .value_name("WHEN")
                .takes_value(true)
                .hide_possible_values(true)
                .possible_values(&["never", "auto", "always", "ansi"]),
        )
        .arg(
            flag("colors")
                .value_name("SPEC")
                .takes_value(true)
                .multiple(true)
                .number_of_values(1),
        )
        .arg(flag("fixed-strings").short("F"))
        .arg(
            flag("glob")
                .short("g")
                .takes_value(true)
                .multiple(true)
                .number_of_values(1)
                .value_name("GLOB"),
        )
        .arg(flag("ignore-case").short("i"))
        .arg(flag("line-number").short("n"))
        .arg(flag("no-line-number").short("N"))
        .arg(flag("quiet").short("q"))
        .arg(
            flag("type")
                .short("t")
                .takes_value(true)
                .multiple(true)
                .number_of_values(1)
                .value_name("TYPE"),
        )
        .arg(
            flag("type-not")
                .short("T")
                .takes_value(true)
                .multiple(true)
                .number_of_values(1)
                .value_name("TYPE"),
        )
        .arg(flag("unrestricted").short("u").multiple(true))
        .arg(flag("invert-match").short("v"))
        .arg(flag("word-regexp").short("w"))
        // Third, set up less common flags.
        .arg(
            flag("after-context")
                .short("A")
                .value_name("NUM")
                .takes_value(true)
                .validator(validate_number),
        )
        .arg(
            flag("before-context")
                .short("B")
                .value_name("NUM")
                .takes_value(true)
                .validator(validate_number),
        )
        .arg(
            flag("context")
                .short("C")
                .value_name("NUM")
                .takes_value(true)
                .validator(validate_number),
        )
        .arg(flag("column"))
        .arg(
            flag("context-separator")
                .value_name("SEPARATOR")
                .takes_value(true),
        )
        .arg(flag("debug"))
        .arg(
            flag("file")
                .short("f")
                .value_name("FILE")
                .takes_value(true)
                .multiple(true)
                .number_of_values(1),
        )
        .arg(flag("files-with-matches").short("l"))
        .arg(flag("files-without-match"))
        .arg(flag("with-filename").short("H"))
        .arg(flag("no-filename"))
        .arg(flag("heading").overrides_with("no-heading"))
        .arg(flag("no-heading").overrides_with("heading"))
        .arg(flag("hidden"))
        .arg(
            flag("ignore-file")
                .value_name("FILE")
                .takes_value(true)
                .multiple(true)
                .number_of_values(1),
        )
        .arg(flag("follow").short("L"))
        .arg(
            flag("max-count")
                .short("m")
                .value_name("NUM")
                .takes_value(true)
                .validator(validate_number),
        )
        .arg(
            flag("maxdepth")
                .value_name("NUM")
                .takes_value(true)
                .validator(validate_number),
        )
        .arg(flag("mmap"))
        .arg(flag("no-messages"))
        .arg(flag("no-mmap"))
        .arg(flag("no-ignore"))
        .arg(flag("no-ignore-parent"))
        .arg(flag("no-ignore-vcs"))
        .arg(flag("null"))
        .arg(
            flag("path-separator")
                .value_name("SEPARATOR")
                .takes_value(true),
        )
        .arg(flag("pretty").short("p"))
        .arg(
            flag("replace")
                .short("r")
                .value_name("ARG")
                .takes_value(true),
        )
        .arg(flag("case-sensitive").short("s"))
        .arg(flag("smart-case").short("S"))
        .arg(flag("sort-files"))
        .arg(
            flag("threads")
                .short("j")
                .value_name("ARG")
                .takes_value(true)
                .validator(validate_number),
        )
        .arg(flag("vimgrep"))
        .arg(
            flag("type-add")
                .value_name("TYPE")
                .takes_value(true)
                .multiple(true)
                .number_of_values(1),
        )
        .arg(
            flag("type-clear")
                .value_name("TYPE")
                .takes_value(true)
                .multiple(true)
                .number_of_values(1),
        )
}

struct Usage {
    short: &'static str,
    long: &'static str,
}

macro_rules! doc {
    ($map:expr, $name:expr, $short:expr) => {
        doc!($map, $name, $short, $short)
    };
    ($map:expr, $name:expr, $short:expr, $long:expr) => {
        $map.insert(
            $name,
            Usage {
                short: $short,
                long: concat!($long, "\n "),
            },
        );
    };
}

lazy_static! {
    static ref USAGES: HashMap<&'static str, Usage> = {
        let mut h = HashMap::new();
        doc!(
            h,
            "help-short",
            "Show short help output.",
            "Show short help output. Use --help to show more details."
        );
        doc!(
            h,
            "help",
            "Show verbose help output.",
            "When given, more details about flags are provided."
        );
        doc!(h, "version", "Prints version information.");

        doc!(
            h,
            "pattern",
            "A regular expression used for searching.",
            "A regular expression used for searching. Multiple patterns \
             may be given. To match a pattern beginning with a -, use [-]."
        );
        doc!(
            h,
            "regexp",
            "A regular expression used for searching.",
            "A regular expression used for searching. Multiple patterns \
             may be given. To match a pattern beginning with a -, use [-]."
        );
        doc!(
            h,
            "path",
            "A file or directory to search.",
            "A file or directory to search. Directories are searched \
             recursively."
        );
        doc!(
            h,
            "files",
            "Print each file that would be searched.",
            "Print each file that would be searched without actually \
             performing the search. This is useful to determine whether a \
             particular file is being searched or not."
        );
        doc!(
            h,
            "type-list",
            "Show all supported file types.",
            "Show all supported file types and their corresponding globs."
        );

        doc!(h, "text", "Search binary files as if they were text.");
        doc!(h, "count", "Only show count of matches for each file.");
        doc!(
            h,
            "color",
            "When to use color. [default: auto]",
            "When to use color in the output. The possible values are \
             never, auto, always or ansi. The default is auto. When always \
             is used, coloring is attempted based on your environment. When \
             ansi used, coloring is forcefully done using ANSI escape color \
             codes."
        );
        doc!(
            h,
            "colors",
            "Configure color settings and styles.",
            "This flag specifies color settings for use in the output. \
             This flag may be provided multiple times. Settings are applied \
             iteratively. Colors are limited to one of eight choices: \
             red, blue, green, cyan, magenta, yellow, white and black. \
             Styles are limited to nobold, bold, nointense or intense.\n\n\
             The format of the flag is {type}:{attribute}:{value}. {type} \
             should be one of path, line or match. {attribute} can be fg, bg \
             or style. {value} is either a color (for fg and bg) or a text \
             style. A special format, {type}:none, will clear all color \
             settings for {type}.\n\nFor example, the following command will \
             change the match color to magenta and the background color for \
             line numbers to yellow:\n\n\
             rg --colors 'match:fg:magenta' --colors 'line:bg:yellow' foo."
        );
        doc!(
            h,
            "fixed-strings",
            "Treat the pattern as a literal string.",
            "Treat the pattern as a literal string instead of a regular \
             expression. When this flag is used, special regular expression \
             meta characters such as (){}*+. do not need to be escaped."
        );
        doc!(
            h,
            "glob",
            "Include or exclude files/directories.",
            "Include or exclude files/directories for searching that \
             match the given glob. This always overrides any other \
             ignore logic. Multiple glob flags may be used. Globbing \
             rules match .gitignore globs. Precede a glob with a ! \
             to exclude it."
        );
        doc!(
            h,
            "ignore-case",
            "Case insensitive search.",
            "Case insensitive search. This is overridden by \
             --case-sensitive."
        );
        doc!(
            h,
            "line-number",
            "Show line numbers.",
            "Show line numbers (1-based). This is enabled by default when \
             searching in a tty."
        );
        doc!(
            h,
            "no-line-number",
            "Suppress line numbers.",
            "Suppress line numbers. This is enabled by default when NOT \
             searching in a tty."
        );
        doc!(
            h,
            "quiet",
            "Do not print anything to stdout.",
            "Do not print anything to stdout. If a match is found in a file, \
             stop searching. This is useful when ripgrep is used only for \
             its exit code."
        );
        doc!(
            h,
            "type",
            "Only search files matching TYPE.",
            "Only search files matching TYPE. Multiple type flags may be \
             provided. Use the --type-list flag to list all available \
             types."
        );
        doc!(
            h,
            "type-not",
            "Do not search files matching TYPE.",
            "Do not search files matching TYPE. Multiple type-not flags may \
             be provided. Use the --type-list flag to list all available \
             types."
        );
        doc!(
            h,
            "unrestricted",
            "Reduce the level of \"smart\" searching.",
            "Reduce the level of \"smart\" searching. A single -u \
             won't respect .gitignore (etc.) files. Two -u flags will \
             additionally search hidden files and directories. Three \
             -u flags will additionally search binary files. -uu is \
             roughly equivalent to grep -r and -uuu is roughly \
             equivalent to grep -a -r."
        );
        doc!(
            h,
            "invert-match",
            "Invert matching.",
            "Invert matching. Show lines that don't match given patterns."
        );
        doc!(
            h,
            "word-regexp",
            "Only show matches surrounded by word boundaries.",
            "Only show matches surrounded by word boundaries. This is \
             equivalent to putting \\b before and after all of the search \
             patterns."
        );

        doc!(h, "after-context", "Show NUM lines after each match.");
        doc!(h, "before-context", "Show NUM lines before each match.");
        doc!(h, "context", "Show NUM lines before and after each match.");
        doc!(
            h,
            "column",
            "Show column numbers",
            "Show column numbers (1-based). This only shows the column \
             numbers for the first match on each line. This does not try \
             to account for Unicode. One byte is equal to one column. This \
             implies --line-number."
        );
        doc!(
            h,
            "context-separator",
            "Set the context separator string. [default: --]",
            "The string used to separate non-contiguous context lines in the \
             output. Escape sequences like \\x7F or \\t may be used. The \
             default value is --."
        );
        doc!(
            h,
            "debug",
            "Show debug messages.",
            "Show debug messages. Please use this when filing a bug report."
        );
        doc!(
            h,
            "file",
            "Search for patterns from the given file.",
            "Search for patterns from the given file, with one pattern per \
             line. When this flag is used or multiple times or in \
             combination with the -e/--regexp flag, then all patterns \
             provided are searched. Empty pattern lines will match all input \
             lines, and the newline is not counted as part of the pattern."
        );
        doc!(
            h,
            "files-with-matches",
            "Only show the path of each file with at least one match."
        );
        doc!(
            h,
            "files-without-match",
            "Only show the path of each file that contains zero matches."
        );
        doc!(
            h,
            "with-filename",
            "Show file name for each match.",
            "Prefix each match with the file name that contains it. This is \
             the default when more than one file is searched."
        );
        doc!(
            h,
            "no-filename",
            "Never show the file name for a match.",
            "Never show the file name for a match. This is the default when \
             one file is searched."
        );
        doc!(
            h,
            "heading",
            "Show matches grouped by each file.",
            "This shows the file name above clusters of matches from each \
             file instead of showing the file name for every match. This is \
             the default mode at a tty."
        );
        doc!(
            h,
            "no-heading",
            "Don't group matches by each file.",
            "Don't group matches by each file. If -H/--with-filename is \
             enabled, then file names will be shown for every line matched. \
             This is the default mode when not at a tty."
        );
        doc!(
            h,
            "hidden",
            "Search hidden files and directories.",
            "Search hidden files and directories. By default, hidden files \
             and directories are skipped."
        );
        doc!(
            h,
            "ignore-file",
            "Specify additional ignore files.",
            "Specify additional ignore files for filtering file paths. \
             Ignore files should be in the gitignore format and are matched \
             relative to the current working directory. These ignore files \
             have lower precedence than all other ignore files. When \
             specifying multiple ignore files, earlier files have lower \
             precedence than later files."
        );
        doc!(h, "follow", "Follow symbolic links.");
        doc!(
            h,
            "max-count",
            "Limit the number of matches.",
            "Limit the number of matching lines per file searched to NUM."
        );
        doc!(
            h,
            "maxdepth",
            "Descend at most NUM directories.",
            "Limit the depth of directory traversal to NUM levels beyond \
             the paths given. A value of zero only searches the \
             starting-points themselves.\n\nFor example, \
             'rg --maxdepth 0 dir/' is a no-op because dir/ will not be \
             descended into. 'rg --maxdepth 1 dir/' will search only the \
             direct children of dir/."
        );
        doc!(
            h,
            "mmap",
            "Searching using memory maps when possible.",
            "Search using memory maps when possible. This is enabled by \
             default when ripgrep thinks it will be faster. Note that memory \
             map searching doesn't currently support all options, so if an \
             incompatible option (e.g., --context) is given with --mmap, \
             then memory maps will not be used."
        );
        doc!(
            h,
            "no-messages",
            "Suppress all error messages.",
            "Suppress all error messages. This is equivalent to redirecting \
             stderr to /dev/null."
        );
        doc!(
            h,
            "no-mmap",
            "Never use memory maps.",
            "Never use memory maps, even when they might be faster."
        );
        doc!(
            h,
            "no-ignore",
            "Don't respect ignore files.",
            "Don't respect ignore files (.gitignore, .ignore, etc.). This \
             implies --no-ignore-parent and --no-ignore-vcs."
        );
        doc!(
            h,
            "no-ignore-parent",
            "Don't respect ignore files in parent directories.",
            "Don't respect ignore files (.gitignore, .ignore, etc.) in \
             parent directories."
        );
        doc!(
            h,
            "no-ignore-vcs",
            "Don't respect VCS ignore files",
            "Don't respect version control ignore files (.gitignore, etc.). \
             This implies --no-ignore-parent. Note that .ignore files will \
             continue to be respected."
        );
        doc!(
            h,
            "null",
            "Print NUL byte after file names",
            "Whenever a file name is printed, follow it with a NUL byte. \
             This includes printing file names before matches, and when \
             printing a list of matching files such as with --count, \
             --files-with-matches and --files. This option is useful for use \
             with xargs."
        );
        doc!(
            h,
            "path-separator",
            "Path separator to use when printing file paths.",
            "The path separator to use when printing file paths. This \
             defaults to your platform's path separator, which is / on Unix \
             and \\ on Windows. This flag is intended for overriding the \
             default when the environment demands it (e.g., cygwin). A path \
             separator is limited to a single byte."
        );
        doc!(h, "pretty", "Alias for --color always --heading -n.");
        doc!(
            h,
            "replace",
            "Replace matches with string given.",
            "Replace every match with the string given when printing \
             results. Neither this flag nor any other flag will modify your \
             files.\n\nCapture group indices (e.g., $5) and names \
             (e.g., $foo) are supported in the replacement string.\n\n\
             Note that the replacement by default replaces each match, and \
             NOT the entire line. To replace the entire line, you should \
             match the entire line."
        );
        doc!(
            h,
            "case-sensitive",
            "Search case sensitively.",
            "Search case sensitively. This overrides -i/--ignore-case and \
             -S/--smart-case."
        );
        doc!(
            h,
            "smart-case",
            "Smart case search.",
            "Searches case insensitively if the pattern is all lowercase. \
             Search case sensitively otherwise. This is overridden by \
             either -s/--case-sensitive or -i/--ignore-case."
        );
        doc!(
            h,
            "sort-files",
            "Sort results by file path. Implies --threads=1.",
            "Sort results by file path. Note that this currently \
             disables all parallelism and runs search in a single thread."
        );
        doc!(
            h,
            "threads",
            "The approximate number of threads to use.",
            "The approximate number of threads to use. A value of 0 (which \
             is the default) causes ripgrep to choose the thread count \
             using heuristics."
        );
        doc!(
            h,
            "vimgrep",
            "Show results in vim compatible format.",
            "Show results with every match on its own line, including \
             line numbers and column numbers. With this option, a line with \
             more than one match will be printed more than once."
        );

        doc!(
            h,
            "type-add",
            "Add a new glob for a file type.",
            "Add a new glob for a particular file type. Only one glob can be \
             added at a time. Multiple --type-add flags can be provided. \
             Unless --type-clear is used, globs are added to any existing \
             globs defined inside of ripgrep.\n\nNote that this MUST be \
             passed to every invocation of ripgrep. Type settings are NOT \
             persisted.\n\nExample: \
             rg --type-add 'foo:*.foo' -tfoo PATTERN.\n\n\
             --type-add can also be used to include rules from other types \
             with the special include directive. The include directive \
             permits specifying one or more other type names (separated by a \
             comma) that have been defined and its rules will automatically \
             be imported into the type specified. For example, to create a \
             type called src that matches C++, Python and Markdown files, one \
             can use:\n\n\
             --type-add 'src:include:cpp,py,md'\n\n\
             Additional glob rules can still be added to the src type by \
             using the --type-add flag again:\n\n\
             --type-add 'src:include:cpp,py,md' --type-add 'src:*.foo'\n\n\
             Note that type names must consist only of Unicode letters or \
             numbers. Punctuation characters are not allowed."
        );
        doc!(
            h,
            "type-clear",
            "Clear globs for given file type.",
            "Clear the file type globs previously defined for TYPE. This \
             only clears the default type definitions that are found inside \
             of ripgrep.\n\nNote that this MUST be passed to every \
             invocation of ripgrep. Type settings are NOT persisted."
        );

        h
    };
}

fn validate_number(s: String) -> Result<(), String> {
    s.parse::<usize>()
        .map(|_| ())
        .map_err(|err| err.to_string())
}
