use args::ArgMatches;

pub enum ArgNames<'ar, 'a> {
    Matches(&'ar ArgMatches<'ar, 'ar>),
    Opt(&'a str),
    None
}