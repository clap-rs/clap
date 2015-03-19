#![crate_type= "lib"]

#![feature(libc, exit_status)]

// DOCS

pub use args::{Arg, SubCommand, ArgMatches};
pub use app::App;

mod app;
mod args;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn unique_arg_names(){
        App::new("some").args(vec![
            Arg::new("arg").short("a"),
            Arg::new("arg").short("b")
        ]);
    }
    #[test]
    #[should_panic]
    fn unique_arg_shorts(){
        App::new("some").args(vec![
            Arg::new("arg1").short("a"),
            Arg::new("arg2").short("a")
        ]);
    }
    #[test]
    #[should_panic]
    fn unique_arg_longs(){
        App::new("some").args(vec![
            Arg::new("arg1").long("long"),
            Arg::new("arg2").long("long")
        ]);
    }
    #[test]
    fn create_app(){
        App::new("some").about("about").author("author").version("1.0");
    }
    #[test]
    fn create_arg_flag(){
        Arg::new("some").short("a").long("long").help("help with some arg").multiple(true);
    }
    #[test]
    fn create_arg_pos(){
        Arg::new("some").index(1).help("help with some arg").required(true);
    }
    #[test]
    fn create_arg_opt(){
        Arg::new("some").short("s").long("some").takes_value(true).help("help with some arg").required(true);
    }
}
