extern crate clap;
extern crate regex;

use clap::App;

include!("../clap-test.rs");

static EXAMPLE1_TMPL_S: &'static str = include_str!("example1_tmpl_simple.txt");
static EXAMPLE1_TMPS_F: &'static str = include_str!("example1_tmpl_full.txt");

static CUSTOM_TEMPL_HELP: &'static str = "MyApp 1.0
Kevin K. <kbknapp@gmail.com>
Does awesome things

USAGE:
    MyApp [FLAGS] [OPTIONS] <output> [SUBCOMMAND]

FLAGS:
    -d               Turn debugging information on
    -h, --help       Prints help information
    -V, --version    Prints version information
OPTIONS:
    -c, --config <FILE>    Sets a custom config file
ARGS:
    <output>    Sets an optional output file
SUBCOMMANDS:
    help    Prints this message or the help of the given subcommand(s)
    test    does testing things";

static SIMPLE_TEMPLATE: &'static str = "MyApp 1.0
Kevin K. <kbknapp@gmail.com>
Does awesome things

USAGE:
    MyApp [FLAGS] [OPTIONS] <output> [SUBCOMMAND]

ARGS:
    <output>    Sets an optional output file

FLAGS:
    -d               Turn debugging information on
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -c, --config <FILE>    Sets a custom config file

SUBCOMMANDS:
    help    Prints this message or the help of the given subcommand(s)
    test    does testing things";

#[test]
fn with_template() {
    let app = app_example1().help_template(EXAMPLE1_TMPL_S);
    assert!(test::compare_output(
        app,
        "MyApp --help",
        SIMPLE_TEMPLATE,
        false
    ));
}

#[test]
fn custom_template() {
    let app = app_example1().help_template(EXAMPLE1_TMPS_F);
    assert!(test::compare_output(
        app,
        "MyApp --help",
        CUSTOM_TEMPL_HELP,
        false
    ));
}

#[test]
fn template_empty() {
    let app = App::new("MyApp")
        .version("1.0")
        .author("Kevin K. <kbknapp@gmail.com>")
        .about("Does awesome things")
        .help_template("");
    assert!(test::compare_output(app, "MyApp --help", "", false));
}

#[test]
fn template_notag() {
    let app = App::new("MyApp")
        .version("1.0")
        .author("Kevin K. <kbknapp@gmail.com>")
        .about("Does awesome things")
        .help_template("test no tag test");
    assert!(test::compare_output(
        app,
        "MyApp --help",
        "test no tag test",
        false
    ));
}

#[test]
fn template_unknowntag() {
    let app = App::new("MyApp")
        .version("1.0")
        .author("Kevin K. <kbknapp@gmail.com>")
        .about("Does awesome things")
        .help_template("test {unknown_tag} test");
    assert!(test::compare_output(
        app,
        "MyApp --help",
        "test {unknown_tag} test",
        false
    ));
}

#[test]
fn template_author_version() {
    let app = App::new("MyApp")
        .version("1.0")
        .author("Kevin K. <kbknapp@gmail.com>")
        .about("Does awesome things")
        .help_template("{author}\n{version}\n{about}\n{bin}");
    assert!(test::compare_output(
        app,
        "MyApp --help",
        "Kevin K. <kbknapp@gmail.com>\n1.0\nDoes awesome things\nMyApp",
        false
    ));
}

// ----------

fn app_example1<'b, 'c>() -> App<'c> {
    App::new("MyApp")
        .version("1.0")
        .author("Kevin K. <kbknapp@gmail.com>")
        .about("Does awesome things")
        .arg("-c, --config=[FILE] 'Sets a custom config file'")
        .arg("<output>            'Sets an optional output file'")
        .arg("-d...               'Turn debugging information on'")
        .subcommand(
            App::new("test")
                .about("does testing things")
                .arg("-l, --list 'lists test values'"),
        )
}
