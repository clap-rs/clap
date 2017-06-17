#![feature(test)]

extern crate clap;
extern crate test;

use test::Bencher;

use std::io::Cursor;

use clap::App;
use clap::{Arg, SubCommand};

fn build_help(app: &App) -> String {
    let mut buf = Cursor::new(Vec::with_capacity(50));
    app.write_help(&mut buf).unwrap();
    let content = buf.into_inner();
    String::from_utf8(content).unwrap()
}

fn app_example1<'b, 'c>() -> App<'b, 'c> {
    App::new("MyApp")
        .version("1.0")
        .author("Kevin K. <kbknapp@gmail.com>")
        .about("Does awesome things")
        .args_from_usage(
            "-c, --config=[FILE] 'Sets a custom config file'
                          <output> 'Sets an optional output file'
                          -d... 'Turn debugging information on'",
        )
        .subcommand(
            SubCommand::with_name("test")
                .about("does testing things")
                .arg_from_usage("-l, --list 'lists test values'"),
        )
}

fn app_example2<'b, 'c>() -> App<'b, 'c> {
    App::new("MyApp")
        .version("1.0")
        .author("Kevin K. <kbknapp@gmail.com>")
        .about("Does awesome things")
}

fn app_example3<'b, 'c>() -> App<'b, 'c> {
    App::new("MyApp")
        .arg(
            Arg::with_name("debug")
                .help("turn on debugging information")
                .short("d"),
        )
        .args(
            &[
                Arg::with_name("config")
                    .help("sets the config file to use")
                    .takes_value(true)
                    .short("c")
                    .long("config"),
                Arg::with_name("input")
                    .help("the input file to use")
                    .index(1)
                    .required(true),
            ],
        )
        .arg_from_usage("--license 'display the license file'")
        .args_from_usage(
            "[output] 'Supply an output file to use'
                          -i, --int=[IFACE] 'Set an interface to use'",
        )
}

fn app_example4<'b, 'c>() -> App<'b, 'c> {
    App::new("MyApp")
        .about("Parses an input file to do awesome things")
        .version("1.0")
        .author("Kevin K. <kbknapp@gmail.com>")
        .arg(
            Arg::with_name("debug")
                .help("turn on debugging information")
                .short("d")
                .long("debug"),
        )
        .arg(
            Arg::with_name("config")
                .help("sets the config file to use")
                .short("c")
                .long("config"),
        )
        .arg(
            Arg::with_name("input")
                .help("the input file to use")
                .index(1)
                .required(true),
        )
}

fn app_example5<'b, 'c>() -> App<'b, 'c> {
    App::new("MyApp").arg(
        Arg::with_name("awesome")
            .help("turns up the awesome")
            .short("a")
            .long("awesome")
            .multiple(true)
            .requires("config")
            .conflicts_with("output"),
    )
}

fn app_example6<'b, 'c>() -> App<'b, 'c> {
    App::new("MyApp")
        .arg(
            Arg::with_name("input")
                .help("the input file to use")
                .index(1)
                .requires("config")
                .conflicts_with("output")
                .required(true),
        )
        .arg(
            Arg::with_name("config")
                .help("the config file to use")
                .index(2),
        )
}

fn app_example7<'b, 'c>() -> App<'b, 'c> {
    App::new("MyApp")
        .arg(Arg::with_name("config"))
        .arg(Arg::with_name("output"))
        .arg(
            Arg::with_name("input")
                .help("the input file to use")
                .takes_value(true)
                .short("i")
                .long("input")
                .multiple(true)
                .required(true)
                .requires("config")
                .conflicts_with("output"),
        )
}

fn app_example8<'b, 'c>() -> App<'b, 'c> {
    App::new("MyApp")
        .arg(Arg::with_name("config"))
        .arg(Arg::with_name("output"))
        .arg(
            Arg::with_name("input")
                .help("the input file to use")
                .takes_value(true)
                .short("i")
                .long("input")
                .multiple(true)
                .required(true)
                .requires("config")
                .conflicts_with("output"),
        )
}

fn app_example10<'b, 'c>() -> App<'b, 'c> {
    App::new("myapp").about("does awesome things").arg(
        Arg::with_name("CONFIG")
            .help("The config file to use (default is \"config.json\")")
            .short("c")
            .takes_value(true),
    )
}

#[bench]
fn example1(b: &mut Bencher) {
    let app = app_example1();
    b.iter(|| build_help(&app));
}

#[bench]
fn example2(b: &mut Bencher) {
    let app = app_example2();
    b.iter(|| build_help(&app));
}

#[bench]
fn example3(b: &mut Bencher) {
    let app = app_example3();
    b.iter(|| build_help(&app));
}

#[bench]
fn example4(b: &mut Bencher) {
    let app = app_example4();
    b.iter(|| build_help(&app));
}

#[bench]
fn example5(b: &mut Bencher) {
    let app = app_example5();
    b.iter(|| build_help(&app));
}

#[bench]
fn example6(b: &mut Bencher) {
    let app = app_example6();
    b.iter(|| build_help(&app));
}

#[bench]
fn example7(b: &mut Bencher) {
    let app = app_example7();
    b.iter(|| build_help(&app));
}

#[bench]
fn example8(b: &mut Bencher) {
    let app = app_example8();
    b.iter(|| build_help(&app));
}

#[bench]
fn example10(b: &mut Bencher) {
    let app = app_example10();
    b.iter(|| build_help(&app));
}

#[bench]
fn example4_template(b: &mut Bencher) {
    let app = app_example4().template("{bin} {version}\n{author}\n{about}\n\nUSAGE:\n    {usage}\n\nFLAGS:\n{flags}\n\nARGS:\n{args}\n");
    b.iter(|| build_help(&app));
}
