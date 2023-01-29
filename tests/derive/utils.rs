// Hi, future me (or whoever you are)!
//
// Yes, we do need this attr.
// No, the warnings cannot be fixed otherwise.
// Accept and endure. Do not touch.
#![allow(unused)]

use clap::CommandFactory;

pub const FULL_TEMPLATE: &str = "\
{before-help}{name} {version}
{author-with-newline}{about-with-newline}
{usage-heading} {usage}

{all-args}{after-help}";

pub fn get_help<T: CommandFactory>() -> String {
    let output = <T as CommandFactory>::command().render_help().to_string();

    eprintln!("\n%%% HELP %%%:=====\n{output}\n=====\n");
    eprintln!("\n%%% HELP (DEBUG) %%%:=====\n{output:?}\n=====\n");

    output
}

pub fn get_long_help<T: CommandFactory>() -> String {
    let output = <T as CommandFactory>::command()
        .render_long_help()
        .to_string();

    eprintln!("\n%%% LONG_HELP %%%:=====\n{output}\n=====\n");
    eprintln!("\n%%% LONG_HELP (DEBUG) %%%:=====\n{output:?}\n=====\n");

    output
}

pub fn get_subcommand_long_help<T: CommandFactory>(subcmd: &str) -> String {
    let output = <T as CommandFactory>::command()
        .get_subcommands_mut()
        .find(|s| s.get_name() == subcmd)
        .unwrap()
        .render_long_help()
        .to_string();

    eprintln!("\n%%% SUBCOMMAND `{subcmd}` HELP %%%:=====\n{output}\n=====\n",);
    eprintln!("\n%%% SUBCOMMAND `{subcmd}` HELP (DEBUG) %%%:=====\n{output:?}\n=====\n",);

    output
}

#[track_caller]
pub fn assert_output<P: clap::Parser + std::fmt::Debug>(args: &str, expected: &str, stderr: bool) {
    let res = P::try_parse_from(args.split(' ').collect::<Vec<_>>());
    let err = res.unwrap_err();
    let actual = err.render().to_string();
    assert_eq!(
        stderr,
        err.use_stderr(),
        "Should Use STDERR failed. Should be {} but is {}",
        stderr,
        err.use_stderr()
    );
    snapbox::assert_eq(expected, actual)
}
