extern crate clap;

use std::io::Cursor;

use clap::{App, SubCommand};

static EXAMPLE1_TMPL_S : &'static str = include_str!("example1_tmpl_simple.txt");
static EXAMPLE1_TMPS_F : &'static str = include_str!("example1_tmpl_full.txt");

static CUSTOM_TEMPL_HELP: &'static str = "MyApp 1.0
Kevin K. <kbknapp@gmail.com>
Does awesome things

USAGE:
    MyApp [FLAGS] [OPTIONS] <output> [SUBCOMMAND]

FLAGS:
    -d      Turn debugging information on
OPTIONS:
    -c, --config <FILE>    Sets a custom config file
ARGS:
    <output>    Sets an optional output file
SUBCOMMANDS:
    test    does testing things";

static SIMPLE_TEMPLATE: &'static str = "MyApp 1.0
Kevin K. <kbknapp@gmail.com>
Does awesome things

USAGE:
    MyApp [FLAGS] [OPTIONS] <output> [SUBCOMMAND]

FLAGS:
    -d      Turn debugging information on

OPTIONS:
    -c, --config <FILE>    Sets a custom config file

ARGS:
    <output>    Sets an optional output file

SUBCOMMANDS:
    test    does testing things";

fn build_new_help(app: &App) -> String {
    let mut buf = Cursor::new(Vec::with_capacity(50));
    app.write_help(&mut buf).unwrap();
    let content = buf.into_inner();
    String::from_utf8(content).unwrap()
}

fn compare_app_str(l: &App, right: &str) -> bool {
    let left = build_new_help(&l);
    let b = left.trim() == right;
    if !b {
        println!("");
        println!("--> left");
        println!("{}", left);
        println!("--> right");
        println!("{}", right);
        println!("--")
    }
    b
}

#[test]
fn with_template() {
    assert!(compare_app_str(&app_example1().template(EXAMPLE1_TMPL_S), SIMPLE_TEMPLATE));
}

#[test]
fn custom_template() {
    let app = app_example1().template(EXAMPLE1_TMPS_F);
    assert!(compare_app_str(&app, CUSTOM_TEMPL_HELP));
}

#[test]
fn template_empty() {
    let app = App::new("MyApp")
                    .version("1.0")
                    .author("Kevin K. <kbknapp@gmail.com>")
                    .about("Does awesome things")
                    .template("");
    assert!(compare_app_str(&app, ""));
}

#[test]
fn template_notag() {
    let app = App::new("MyApp")
                    .version("1.0")
                    .author("Kevin K. <kbknapp@gmail.com>")
                    .about("Does awesome things")
                    .template("test no tag test");
    assert!(compare_app_str(&app, "test no tag test"));
}

#[test]
fn template_unknowntag() {
    let app = App::new("MyApp")
                    .version("1.0")
                    .author("Kevin K. <kbknapp@gmail.com>")
                    .about("Does awesome things")
                    .template("test {unknown_tag} test");
    assert!(compare_app_str(&app, "test {unknown_tag} test"));
}

#[test]
fn template_author_version() {
    let app = App::new("MyApp")
                    .version("1.0")
                    .author("Kevin K. <kbknapp@gmail.com>")
                    .about("Does awesome things")
                    .template("{author}\n{version}\n{about}\n{bin}");
    assert!(compare_app_str(&app, "Kevin K. <kbknapp@gmail.com>\n1.0\nDoes awesome things\nMyApp"));
}

fn app_example1<'b, 'c>() -> App<'b, 'c> {
    App::new("MyApp")
        .version("1.0")
        .author("Kevin K. <kbknapp@gmail.com>")
        .about("Does awesome things")
        .args_from_usage("-c, --config=[FILE] 'Sets a custom config file'
                          <output>            'Sets an optional output file'
                          -d...               'Turn debugging information on'")
        .subcommand(SubCommand::with_name("test")
                        .about("does testing things")
                        .arg_from_usage("-l, --list 'lists test values'"))
}
