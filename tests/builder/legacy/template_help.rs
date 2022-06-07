use super::utils;

use clap::{arg, Command};

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
    let cmd = get_app().help_template(EXAMPLE1_TMPL_S);
    utils::assert_output(cmd, "MyApp --help", SIMPLE_TEMPLATE, false);
}

#[test]
fn custom_template() {
    let cmd = get_app().help_template(EXAMPLE1_TMPS_F);
    utils::assert_output(cmd, "MyApp --help", CUSTOM_TEMPL_HELP, false);
}

#[test]
fn template_empty() {
    let cmd = Command::new("MyApp")
        .version("1.0")
        .author("Kevin K. <kbknapp@gmail.com>")
        .about("Does awesome things")
        .help_template("");
    utils::assert_output(cmd, "MyApp --help", "\n", false);
}

#[test]
fn template_notag() {
    let cmd = Command::new("MyApp")
        .version("1.0")
        .author("Kevin K. <kbknapp@gmail.com>")
        .about("Does awesome things")
        .help_template("test no tag test");
    utils::assert_output(cmd, "MyApp --help", "test no tag test\n", false);
}

#[test]
fn template_unknowntag() {
    let cmd = Command::new("MyApp")
        .version("1.0")
        .author("Kevin K. <kbknapp@gmail.com>")
        .about("Does awesome things")
        .help_template("test {unknown_tag} test");
    utils::assert_output(cmd, "MyApp --help", "test {unknown_tag} test\n", false);
}

#[test]
fn template_author_version() {
    let cmd = Command::new("MyApp")
        .version("1.0")
        .author("Kevin K. <kbknapp@gmail.com>")
        .about("Does awesome things")
        .help_template("{author}\n{version}\n{about}\n{bin}");
    utils::assert_output(
        cmd,
        "MyApp --help",
        "Kevin K. <kbknapp@gmail.com>\n1.0\nDoes awesome things\nMyApp\n",
        false,
    );
}

// ----------

fn get_app() -> Command<'static> {
    Command::new("MyApp")
        .version("1.0")
        .author("Kevin K. <kbknapp@gmail.com>")
        .about("Does awesome things")
        .arg(
            arg!(
                -c --config <FILE> "Sets a custom config file"
            )
            .required(false),
        )
        .arg(arg!(
            <output>            "Sets an optional output file"
        ))
        .arg(arg!(
            d: -d ...           "Turn debugging information on"
        ))
        .subcommand(
            Command::new("test")
                .about("does testing things")
                .arg(arg!(-l --list "lists test values")),
        )
}
