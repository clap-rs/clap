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
        .args_from_usage("-c, --config=[FILE] 'Sets a custom config file'
                                 <output> 'Sets an optional output file'
                                 -d... 'Turn debugging information on'")
        .subcommand(SubCommand::with_name("test")
                        .about("does testing things")
                        .arg_from_usage("-l, --list 'lists test values'"))
}

fn app_example2<'b, 'c>() -> App<'b, 'c> {
    App::new("MyApp")
        .version("1.0")
        .author("Kevin K. <kbknapp@gmail.com>")
        .about("Does awesome things")
}

fn app_example3<'b, 'c>() -> App<'b, 'c> {
    App::new("MyApp")
        // All application settings go here...

        // A simple "Flag" argument example (i.e. "-d") using the builder pattern
        .arg(Arg::with_name("debug")
                    .help("turn on debugging information")
                    .short("d"))

        // Two arguments, one "Option" argument (i.e. one that takes a value) such
        // as "-c some", and one positional argument (i.e. "myapp some_file")
        .args(&[
            Arg::with_name("config")
                    .help("sets the config file to use")
                    .takes_value(true)
                    .short("c")
                    .long("config"),
            Arg::with_name("input")
                    .help("the input file to use")
                    .index(1)
                    .required(true)
        ])

        // *Note* the following two examples are convienience methods, if you wish
        // to still get the full configurability of Arg::with_name() and the readability
        // of arg_from_usage(), you can instantiate a new Arg with Arg::from_usage() and
        // still be able to set all the additional properties, just like Arg::with_name()
        //
        //
        // One "Flag" using a usage string
        .arg_from_usage("--license 'display the license file'")

        // Two args, one "Positional", and one "Option" using a usage string
        .args_from_usage("[output] 'Supply an output file to use'
                          -i, --int=[IFACE] 'Set an interface to use'")
}

fn app_example4<'b, 'c>() -> App<'b, 'c> {
    App::new("MyApp")
        .about("Parses an input file to do awesome things")
        .version("1.0")
        .author("Kevin K. <kbknapp@gmail.com>")
        .arg(Arg::with_name("debug")
                 .help("turn on debugging information")
                 .short("d")
                 .long("debug"))
        .arg(Arg::with_name("config")
                 .help("sets the config file to use")
                 .short("c")
                 .long("config"))
        .arg(Arg::with_name("input")
                 .help("the input file to use")
                 .index(1)
                 .required(true))
}

fn app_example5<'b, 'c>() -> App<'b, 'c> {
    App::new("MyApp")
        // Regular App configuration goes here...

        // We'll add a flag that represents an awesome meter...
        //
        // I'll explain each possible setting that "flags" accept. Keep in mind
        // that you DO NOT need to set each of these for every flag, only the ones
        // you want for your individual case.
        .arg(Arg::with_name("awesome")
                    .help("turns up the awesome") // Displayed when showing help info
                    .short("a")                   // Trigger this arg with "-a"
                    .long("awesome")              // Trigger this arg with "--awesome"
                    .multiple(true)               // This flag should allow multiple
                                                  // occurrences such as "-aaa" or "-a -a"
                    .requires("config")           // Says, "If the user uses -a, they MUST
                                                  // also use this other 'config' arg too"
                                                  // Can also specifiy a list using
                                                  // requires_all(Vec<&str>)
                    .conflicts_with("output")     // Opposite of requires(), says "if the
                                                  // user uses -a, they CANNOT use 'output'"
                                                  // also has a mutually_excludes_all(Vec<&str>)
        )
}

fn app_example6<'b, 'c>() -> App<'b, 'c> {
    App::new("MyApp")
        // Regular App configuration goes here...

        // We'll add two positional arguments, a input file, and a config file.
        //
        // I'll explain each possible setting that "positionals" accept. Keep in
        // mind that you DO NOT need to set each of these for every flag, only the
        // ones that apply to your individual case.
        .arg(Arg::with_name("input")
                    .help("the input file to use") // Displayed when showing help info
                    .index(1)                      // Set the order in which the user must
                                                   // specify this argument (Starts at 1)
                    .requires("config")            // Says, "If the user uses "input", they MUST
                                                   // also use this other 'config' arg too"
                                                   // Can also specifiy a list using
                                                   // requires_all(Vec<&str>)
                    .conflicts_with("output")      // Opposite of requires(), says "if the
                                                   // user uses -a, they CANNOT use 'output'"
                                                   // also has a mutually_excludes_all(Vec<&str>)
                    .required(true)                // By default this argument MUST be present
                                                   // NOTE: mutual exclusions take precedence over
                                                   // required arguments
        )
        .arg(Arg::with_name("config")
                    .help("the config file to use")
                    .index(2))                     // Note, we do not need to specify required(true)
                                                   // if we don't want to, because "input" already
                                                   // requires "config"
                                                   // Note, we also do not need to specify requires("input")
                                                   // because requires lists are automatically two-way
}

fn app_example7<'b, 'c>() -> App<'b, 'c> {
    App::new("MyApp")
        // Regular App configuration goes here...

        // Assume we an application that accepts an input file via the "-i file"
        // or the "--input file" (as wel as "--input=file").
        // Below every setting supported by option arguments is discussed.
        // NOTE: You DO NOT need to specify each setting, only those which apply
        // to your particular case.
        .arg(Arg::with_name("input")
                    .help("the input file to use") // Displayed when showing help info
                    .takes_value(true)             // MUST be set to true in order to be an "option" argument
                    .short("i")                    // This argument is triggered with "-i"
                    .long("input")                 // This argument is triggered with "--input"
                    .multiple(true)                // Set to true if you wish to allow multiple occurrences
                                                   // such as "-i file -i other_file -i third_file"
                    .required(true)                // By default this argument MUST be present
                                                   // NOTE: mutual exclusions take precedence over
                                                   // required arguments
                    .requires("config")            // Says, "If the user uses "input", they MUST
                                                   // also use this other 'config' arg too"
                                                   // Can also specifiy a list using
                                                   // requires_all(Vec<&str>)
                    .conflicts_with("output")      // Opposite of requires(), says "if the
                                                   // user uses -a, they CANNOT use 'output'"
                                                   // also has a conflicts_with_all(Vec<&str>)
        )
}

fn app_example8<'b, 'c>() -> App<'b, 'c> {
    App::new("MyApp")
                    // Regular App configuration goes here...

                    // Assume we an application that accepts an input file via the "-i file"
                    // or the "--input file" (as wel as "--input=file").
                    // Below every setting supported by option arguments is discussed.
                    // NOTE: You DO NOT need to specify each setting, only those which apply
                    // to your particular case.
        .arg(Arg::with_name("input")
                    .help("the input file to use") // Displayed when showing help info
                    .takes_value(true)             // MUST be set to true in order to be an "option" argument
                    .short("i")                    // This argument is triggered with "-i"
                    .long("input")                 // This argument is triggered with "--input"
                    .multiple(true)                // Set to true if you wish to allow multiple occurrences
                                                   // such as "-i file -i other_file -i third_file"
                    .required(true)                // By default this argument MUST be present
                                                   // NOTE: mutual exclusions take precedence over
                                                   // required arguments
                    .requires("config")            // Says, "If the user uses "input", they MUST
                                                   // also use this other 'config' arg too"
                                                   // Can also specifiy a list using
                                                   // requires_all(Vec<&str>)
                    .conflicts_with("output")      // Opposite of requires(), says "if the
                                                   // user uses -a, they CANNOT use 'output'"
                                                   // also has a conflicts_with_all(Vec<&str>)
            )
}

fn app_example10<'b, 'c>() -> App<'b, 'c> {
    App::new("myapp")
        .about("does awesome things")
        .arg(Arg::with_name("CONFIG")
                 .help("The config file to use (default is \"config.json\")")
                 .short("c")
                 .takes_value(true))
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
/*
MyApp 1.0
Kevin K. <kbknapp@gmail.com>
Parses an input file to do awesome things

USAGE:
    test [FLAGS] <input>

FLAGS:
    -c, --config     sets the config file to use
    -d, --debug      turn on debugging information
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <input>    the input file to use
*/

    let app = app_example4().template("{bin} {version}\n{author}\n{about}\n\nUSAGE:\n    {usage}\n\nFLAGS:\n{flags}\n\nARGS:\n{args}\n");
    b.iter(|| build_help(&app));
}
