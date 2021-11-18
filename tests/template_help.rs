mod utils;

use clap::{App, Arg};

static EXAMPLE1_TMPL_S: &str = "{bin} {version}
{author}
{about}

USAGE:
    {usage}

{all-args}";

static EXAMPLE1_TMPS_F: &str = "{bin} {version}
{author}
{about}

USAGE:
    {usage}

OPTIONS:
{options}
ARGS:
{positionals}
SUBCOMMANDS:
{subcommands}";

static CUSTOM_TEMPL_HELP: &str = "MyApp 1.0
Kevin K. <kbknapp@gmail.com>
Does awesome things

USAGE:
    MyApp [OPTIONS] <output> [SUBCOMMAND]

OPTIONS:
    -c, --config <FILE>    Sets a custom config file
    -d                     Turn debugging information on
    -h, --help             Print help information
    -V, --version          Print version information
ARGS:
    <output>    Sets an optional output file
SUBCOMMANDS:
    help    Print this message or the help of the given subcommand(s)
    test    does testing things
";

static SIMPLE_TEMPLATE: &str = "MyApp 1.0
Kevin K. <kbknapp@gmail.com>
Does awesome things

USAGE:
    MyApp [OPTIONS] <output> [SUBCOMMAND]

ARGS:
    <output>    Sets an optional output file

OPTIONS:
    -c, --config <FILE>    Sets a custom config file
    -d                     Turn debugging information on
    -h, --help             Print help information
    -V, --version          Print version information

SUBCOMMANDS:
    help    Print this message or the help of the given subcommand(s)
    test    does testing things
";

#[test]
fn with_template() {
    let app = get_app().help_template(EXAMPLE1_TMPL_S);
    assert!(utils::compare_output(
        app,
        "MyApp --help",
        SIMPLE_TEMPLATE,
        false
    ));
}

#[test]
fn custom_template() {
    let app = get_app().help_template(EXAMPLE1_TMPS_F);
    assert!(utils::compare_output(
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
    assert!(utils::compare_output(app, "MyApp --help", "\n", false));
}

#[test]
fn template_notag() {
    let app = App::new("MyApp")
        .version("1.0")
        .author("Kevin K. <kbknapp@gmail.com>")
        .about("Does awesome things")
        .help_template("test no tag test");
    assert!(utils::compare_output(
        app,
        "MyApp --help",
        "test no tag test\n",
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
    assert!(utils::compare_output(
        app,
        "MyApp --help",
        "test {unknown_tag} test\n",
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
    assert!(utils::compare_output(
        app,
        "MyApp --help",
        "Kevin K. <kbknapp@gmail.com>\n1.0\nDoes awesome things\nMyApp\n",
        false
    ));
}

// ----------

fn get_app() -> App<'static> {
    App::new("MyApp")
        .version("1.0")
        .author("Kevin K. <kbknapp@gmail.com>")
        .about("Does awesome things")
        .arg(Arg::from_usage(
            "-c, --config=[FILE] 'Sets a custom config file'",
        ))
        .arg(Arg::from_usage(
            "<output>            'Sets an optional output file'",
        ))
        .arg(Arg::from_usage(
            "-d...               'Turn debugging information on'",
        ))
        .subcommand(
            App::new("test")
                .about("does testing things")
                .arg(Arg::from_usage("-l, --list 'lists test values'")),
        )
}
