extern crate clap;

use std::io::Cursor;

use clap::{App, SubCommand};

static EXAMPLE1_TMPL_S : &'static str = include_str!("example1_tmpl_simple.txt");
static EXAMPLE1_TMPS_F : &'static str = include_str!("example1_tmpl_full.txt");

fn build_new_help(app: &App) -> String {
    let mut buf = Cursor::new(Vec::with_capacity(50));
    app.write_help(&mut buf).unwrap();
    let content = buf.into_inner();
    String::from_utf8(content).unwrap()
}

fn compare2(app1: &App, app2: &App) -> bool {
    let hlp1f = build_new_help(&app1);
    let hlp1 = hlp1f.trim();
    let hlp2f = build_new_help(&app2);
    let hlp2 = hlp2f.trim();
    let b = hlp1 == hlp2;
    if !b {
        println!("");
        println!("--> hlp1");
        println!("{}", hlp1);
        println!("--> hlp2");
        println!("{}", hlp2);
        println!("--")
    }
    b
}

#[test]
fn comparison_with_template() {
    assert!(compare2(&app_example1(), &app_example1().template(EXAMPLE1_TMPL_S)));
    assert!(compare2(&app_example1(), &app_example1().template(EXAMPLE1_TMPS_F)));
}

#[test]
fn template_empty() {
    let app = App::new("MyApp")
                    .version("1.0")
                    .author("Kevin K. <kbknapp@gmail.com>")
                    .about("Does awesome things")
                    .template("");
    assert_eq!(build_new_help(&app), "");
}

#[test]
fn template_notag() {
    let app = App::new("MyApp")
                    .version("1.0")
                    .author("Kevin K. <kbknapp@gmail.com>")
                    .about("Does awesome things")
                    .template(" no tag ");
    assert_eq!(build_new_help(&app), " no tag ");
}

#[test]
fn template_unknowntag() {
    let app = App::new("MyApp")
                    .version("1.0")
                    .author("Kevin K. <kbknapp@gmail.com>")
                    .about("Does awesome things")
                    .template(" {unknown_tag} ");
    assert_eq!(build_new_help(&app), " {unknown_tag} ");
}

#[test]
fn template_author_version() {
    let app = App::new("MyApp")
                    .version("1.0")
                    .author("Kevin K. <kbknapp@gmail.com>")
                    .about("Does awesome things")
                    .template("{author}\n{version}\n{about}\n{bin}");
    assert_eq!(build_new_help(&app), "Kevin K. <kbknapp@gmail.com>\n1.0\nDoes awesome things\nMyApp");
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
