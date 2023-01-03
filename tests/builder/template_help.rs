use super::utils;

use clap::{arg, Command};

#[cfg(not(feature = "unstable-v5"))]
static EXAMPLE1_TMPL_S: &str = "{bin} {version}
{author}
{about}

Usage: {usage}

{all-args}";

#[cfg(feature = "unstable-v5")]
static EXAMPLE1_TMPL_S: &str = "{name} {version}
{author}
{about}

Usage: {usage}

{all-args}";

#[cfg(not(feature = "unstable-v5"))]
static EXAMPLE1_TMPS_F: &str = "{bin} {version}
{author}
{about}

Usage: {usage}

Options:
{options}
Arguments:
{positionals}
Commands:
{subcommands}";

#[cfg(feature = "unstable-v5")]
static EXAMPLE1_TMPS_F: &str = "{name} {version}
{author}
{about}

Usage: {usage}

Options:
{options}
Arguments:
{positionals}
Commands:
{subcommands}";

static CUSTOM_TEMPL_HELP: &str = "MyApp 1.0
Kevin K. <kbknapp@gmail.com>
Does awesome things

Usage: MyApp [OPTIONS] <output> [COMMAND]

Options:
  -c, --config <FILE>  Sets a custom config file
  -d...                Turn debugging information on
  -h, --help           Print help
  -V, --version        Print version
Arguments:
  <output>  Sets an optional output file
Commands:
  test  does testing things
  help  Print this message or the help of the given subcommand(s)
";

static SIMPLE_TEMPLATE: &str = "MyApp 1.0
Kevin K. <kbknapp@gmail.com>
Does awesome things

Usage: MyApp [OPTIONS] <output> [COMMAND]

Commands:
  test  does testing things
  help  Print this message or the help of the given subcommand(s)

Arguments:
  <output>  Sets an optional output file

Options:
  -c, --config <FILE>  Sets a custom config file
  -d...                Turn debugging information on
  -h, --help           Print help
  -V, --version        Print version
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
    #[cfg(not(feature = "unstable-v5"))]
    let cmd = Command::new("MyApp")
        .version("1.0")
        .author("Kevin K. <kbknapp@gmail.com>")
        .about("Does awesome things")
        .help_template("{author}\n{version}\n{about}\n{bin}");

    #[cfg(feature = "unstable-v5")]
    let cmd = Command::new("MyApp")
        .version("1.0")
        .author("Kevin K. <kbknapp@gmail.com>")
        .about("Does awesome things")
        .help_template("{author}\n{version}\n{about}\n{name}");

    utils::assert_output(
        cmd,
        "MyApp --help",
        "Kevin K. <kbknapp@gmail.com>\n1.0\nDoes awesome things\nMyApp\n",
        false,
    );
}

// ----------

fn get_app() -> Command {
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
