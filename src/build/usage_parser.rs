use nom::IResult;
use nom::branch::alt;
use nom::bytes::complete::{tag, take, take_till, take_until};
use nom::character::complete::{multispace0, multispace1};
use nom::combinator::{map, opt, rest, verify};
use nom::multi::many0;
use nom::sequence::{delimited, preceded, separated_pair, terminated, tuple};

// Internal
use crate::build::{Arg, ArgSettings};
use crate::util::{Key, VecMap};
use crate::INTERNAL_ERROR_MSG;

#[doc(hidden)]
#[derive(Debug)]
pub struct UsageParser<'a> {
    usage: &'a str,
}

type ArgAst<'a> = (
    Option<((bool, &'a str), bool)>, // name
    Option<&'a str>, // default
    Vec<(((bool, &'a str), bool), Option<&'a str>, Vec<(bool, &'a str)>, bool)>, // params
    Option<&'a str>, // default
    Option<(&'a str, Option<&'a str>)> // help&default
);

fn arg(i: &str) -> IResult<&str, ArgAst> {
    let multi = &map(opt(tag("...")), |o| o.is_some());

    let reqname = map(delimited(
        tag("<"),
        take_until(">"),
        tag(">")
    ), |n| (true, n));
    let optname = map(delimited(
        tag("["),
        take_until("]"),
        tag("]"),
    ), |n| (false, n));
    let name = &alt((reqname, optname));

    let v2default = &preceded(
        tag("@"),
        take_till(|c: char| c.is_whitespace()),
    );

    let short = map(delimited(
        tag("-"),
        verify(take(1usize), |c: &str| c != "-"),
        take_till(|c: char| c.is_whitespace() || c == '.'), // adapt cavalier handling of syntax: tests/flags.rs:short_flag_misspel
    ), |s| (false, s));
    let long = &map(preceded(
        tag("--"),
        take_till(|c: char| c.is_whitespace() || c == '=' || c == '.'),
    ), |s| (true, s));
    let param = &tuple((
        tuple((alt((long, short)), multi)),
        opt(preceded(multispace0, v2default)),
        many0(preceded(alt((tag("="), multispace1)), name)),
        multi,
    ));

    let take_until_consume = |p| terminated(take_until(p), tag(p));
    let helpv3_with_default = map(
        separated_pair(
            take_until_consume("[default:"), // help
            multispace0,
            take_until_consume("]'"), // default
        ),
        |(help, def): (&str, &str)| (help.trim_end(), Some(def))
    );
    let helpv3_no_default = map(
        rest, // help
        |h: &str| (&h[..h.len()-1] /* chop closing ' */, None)
    );
    let help = &preceded(
        tag("'"),
        alt((helpv3_with_default, /* ordering of alternatives is important! */ helpv3_no_default)),
    );

    let arg = tuple((
        opt(tuple((name, multi))),
        opt(preceded(multispace0, v2default)),
        many0(preceded(multispace0, param)),
        opt(preceded(multispace0, v2default)),
        opt(preceded(multispace0, help)),
    ));

    let res = arg(i);
    res
}

fn conv<'a>((name, default1, params, default2, help): ArgAst<'a>) -> Arg<'a> {
    let mut arg = Arg::default();
    let mut explicit_name_set = false;



    // set name if specified

    if let Some(((req, name), multi)) = name {
        if req {
            arg.setb(ArgSettings::Required);
        }
        if multi {
            arg.setb(ArgSettings::MultipleOccurrences);
        }
        arg.id = name.key();
        arg.name = name;
        explicit_name_set = true;
    }


    // set default

    let default = default1.or(default2).or(help.and_then(|t| t.1));
    if default.is_some() {
        arg.setb(ArgSettings::TakesValue);
#[cfg(any(target_os = "windows", target_arch = "wasm32"))]
use osstringext::OsStrExt3;
#[cfg(not(any(target_os = "windows", target_arch = "wasm32")))]
use std::os::unix::ffi::OsStrExt;
        arg.default_val = default.map(|v| std::ffi::OsStr::from_bytes(v.as_bytes()));
    }


    // set help

    arg.help = help.map(|t| t.0);


    // set args

    for param in params {
        let (((long, name), multi_occ), default, values, multi_val) = param;
        if long {
            arg.long = Some(name);
        } else {
            arg.short = Some(name.chars().next().expect("empty short"));
        }
        if multi_occ || multi_val {
            arg.setb(ArgSettings::MultipleOccurrences);
        }
        if default.is_some() {
            arg.setb(ArgSettings::TakesValue);
#[cfg(any(target_os = "windows", target_arch = "wasm32"))]
use osstringext::OsStrExt3;
#[cfg(not(any(target_os = "windows", target_arch = "wasm32")))]
use std::os::unix::ffi::OsStrExt;
            arg.default_val = default.map(|v| std::ffi::OsStr::from_bytes(v.as_bytes()));
        }

        if !values.is_empty() {
            arg.num_vals = if values.len() >= 2 {
                Some(values.len() as u64)
            } else {
                None
            };
            arg.setb(ArgSettings::TakesValue);
            if multi_val {
                arg.setb(ArgSettings::MultipleValues);
            }
            let mut v = VecMap::new();
            for (idx, (req, name)) in values.into_iter().enumerate() {
                if req && !explicit_name_set {
                    arg.setb(ArgSettings::Required);
                }
                v.insert(idx, name);
            }
            arg.val_names = Some(v);
        }
        if !name.is_empty() &&
            (arg.name.is_empty() ||
             (long && !explicit_name_set)) {
                arg.id = name.key();
                arg.name = name;
        }
    }

    arg
}

impl<'a> UsageParser<'a> {
    fn new(usage: &'a str) -> Self {
        debugln!("UsageParser::new: usage={:?}", usage);
        UsageParser {
            usage,
        }
    }

    pub fn from_usage(usage: &'a str) -> Self {
        debugln!("UsageParser::from;");
        UsageParser::new(usage)
    }

    pub fn parse(self) -> Arg<'a> {
        debugln!("UsageParser::parse;");
        let (leftover, ast) = arg(self.usage).expect(INTERNAL_ERROR_MSG);
        if leftover != "" {
            panic!("leftover after parsing: {}", leftover);
        }
        let mut arg = conv(ast);

        arg.disp_ord = 999;
        arg.unified_ord = 999;

         if !arg.has_switch() && arg.is_set(ArgSettings::MultipleOccurrences) {
             // We had a positional and need to set mult vals too
             arg.setb(ArgSettings::MultipleValues);
         }

        debugln!("UsageParser::parse: vals...{:?}", arg.val_names);

        arg
    }
}


#[cfg(test)]
mod test {
    use crate::build::{Arg, ArgSettings};

    #[test]
    fn create_flag_usage() {
        let a = Arg::from("[flag] -f 'some help info'");
        assert_eq!(a.name, "flag");
        assert_eq!(a.short.unwrap(), 'f');
        assert!(a.long.is_none());
        assert_eq!(a.help.unwrap(), "some help info");
        assert!(!a.is_set(ArgSettings::MultipleOccurrences));
        assert!(a.val_names.is_none());
        assert!(a.num_vals.is_none());

        let a = Arg::from("[flag] --flag 'some help info'");
        assert_eq!(a.name, "flag");
        assert_eq!(a.long.unwrap(), "flag");
        assert!(a.short.is_none());
        assert_eq!(a.help.unwrap(), "some help info");
        assert!(!a.is_set(ArgSettings::MultipleOccurrences));
        assert!(a.val_names.is_none());
        assert!(a.num_vals.is_none());

        let a = Arg::from("--flag 'some help info'");
        assert_eq!(a.name, "flag");
        assert_eq!(a.long.unwrap(), "flag");
        assert!(a.short.is_none());
        assert_eq!(a.help.unwrap(), "some help info");
        assert!(!a.is_set(ArgSettings::MultipleOccurrences));
        assert!(a.val_names.is_none());
        assert!(a.num_vals.is_none());

        let a = Arg::from("[flag] -f --flag 'some help info'");
        assert_eq!(a.name, "flag");
        assert_eq!(a.short.unwrap(), 'f');
        assert_eq!(a.long.unwrap(), "flag");
        assert_eq!(a.help.unwrap(), "some help info");
        assert!(!a.is_set(ArgSettings::MultipleOccurrences));
        assert!(a.val_names.is_none());
        assert!(a.num_vals.is_none());

        let a = Arg::from("[flag] -f... 'some help info'");
        assert_eq!(a.name, "flag");
        assert_eq!(a.short.unwrap(), 'f');
        assert!(a.long.is_none());
        assert_eq!(a.help.unwrap(), "some help info");
        assert!(a.is_set(ArgSettings::MultipleOccurrences));
        assert!(a.val_names.is_none());
        assert!(a.num_vals.is_none());

        let a = Arg::from("[flag] -f --flag... 'some help info'");
        assert_eq!(a.name, "flag");
        assert_eq!(a.long.unwrap(), "flag");
        assert_eq!(a.short.unwrap(), 'f');
        assert_eq!(a.help.unwrap(), "some help info");
        assert!(a.is_set(ArgSettings::MultipleOccurrences));
        assert!(a.val_names.is_none());
        assert!(a.num_vals.is_none());

        let a = Arg::from("-f --flag... 'some help info'");
        assert_eq!(a.name, "flag");
        assert_eq!(a.long.unwrap(), "flag");
        assert_eq!(a.short.unwrap(), 'f');
        assert_eq!(a.help.unwrap(), "some help info");
        assert!(a.is_set(ArgSettings::MultipleOccurrences));
        assert!(a.val_names.is_none());
        assert!(a.num_vals.is_none());

        let a = Arg::from("--flags");
        assert_eq!(a.name, "flags");
        assert_eq!(a.long.unwrap(), "flags");
        assert!(a.val_names.is_none());
        assert!(a.num_vals.is_none());

        let a = Arg::from("--flags...");
        assert_eq!(a.name, "flags");
        assert_eq!(a.long.unwrap(), "flags");
        assert!(a.is_set(ArgSettings::MultipleOccurrences));
        assert!(a.val_names.is_none());
        assert!(a.num_vals.is_none());

        let a = Arg::from("[flags] -f");
        assert_eq!(a.name, "flags");
        assert_eq!(a.short.unwrap(), 'f');
        assert!(a.val_names.is_none());
        assert!(a.num_vals.is_none());

        let a = Arg::from("[flags] -f...");
        assert_eq!(a.name, "flags");
        assert_eq!(a.short.unwrap(), 'f');
        assert!(a.is_set(ArgSettings::MultipleOccurrences));
        assert!(a.val_names.is_none());
        assert!(a.num_vals.is_none());

        let a = Arg::from("-f 'some help info'");
        assert_eq!(a.name, "f");
        assert_eq!(a.short.unwrap(), 'f');
        assert!(a.long.is_none());
        assert_eq!(a.help.unwrap(), "some help info");
        assert!(!a.is_set(ArgSettings::MultipleOccurrences));
        assert!(a.val_names.is_none());
        assert!(a.num_vals.is_none());

        let a = Arg::from("-f");
        assert_eq!(a.name, "f");
        assert_eq!(a.short.unwrap(), 'f');
        assert!(a.val_names.is_none());
        assert!(a.num_vals.is_none());

        let a = Arg::from("-f...");
        assert_eq!(a.name, "f");
        assert_eq!(a.short.unwrap(), 'f');
        assert!(a.is_set(ArgSettings::MultipleOccurrences));
        assert!(a.val_names.is_none());
        assert!(a.num_vals.is_none());
    }

    #[test]
    fn create_option_usage0() {
        // Short only
        let a = Arg::from("[option] -o [opt] 'some help info'");
        assert_eq!(a.name, "option");
        assert_eq!(a.short.unwrap(), 'o');
        assert!(a.long.is_none());
        assert_eq!(a.help.unwrap(), "some help info");
        assert!(!a.is_set(ArgSettings::MultipleOccurrences));
        assert!(!a.is_set(ArgSettings::MultipleValues));
        assert!(a.is_set(ArgSettings::TakesValue));
        assert!(!a.is_set(ArgSettings::Required));
        assert_eq!(a.val_names.unwrap().values().collect::<Vec<_>>(), [&"opt"]);
        assert!(a.num_vals.is_none());
    }

    #[test]
    fn create_option_usage1() {
        let a = Arg::from("-o [opt] 'some help info'");
        assert_eq!(a.name, "o");
        assert_eq!(a.short.unwrap(), 'o');
        assert!(a.long.is_none());
        assert_eq!(a.help.unwrap(), "some help info");
        assert!(!a.is_set(ArgSettings::MultipleOccurrences));
        assert!(!a.is_set(ArgSettings::MultipleValues));
        assert!(a.is_set(ArgSettings::TakesValue));
        assert!(!a.is_set(ArgSettings::Required));
        assert_eq!(a.val_names.unwrap().values().collect::<Vec<_>>(), [&"opt"]);
        assert!(a.num_vals.is_none());
    }

    #[test]
    fn create_option_usage2() {
        let a = Arg::from("<option> -o <opt> 'some help info'");
        assert_eq!(a.name, "option");
        assert_eq!(a.short.unwrap(), 'o');
        assert!(a.long.is_none());
        assert_eq!(a.help.unwrap(), "some help info");
        assert!(!a.is_set(ArgSettings::MultipleOccurrences));
        assert!(!a.is_set(ArgSettings::MultipleValues));
        assert!(a.is_set(ArgSettings::TakesValue));
        assert!(a.is_set(ArgSettings::Required));
        assert_eq!(a.val_names.unwrap().values().collect::<Vec<_>>(), [&"opt"]);
        assert!(a.num_vals.is_none());
    }

    #[test]
    fn create_option_usage3() {
        let a = Arg::from("-o <opt> 'some help info'");
        assert_eq!(a.name, "o");
        assert_eq!(a.short.unwrap(), 'o');
        assert!(a.long.is_none());
        assert_eq!(a.help.unwrap(), "some help info");
        assert!(!a.is_set(ArgSettings::MultipleOccurrences));
        assert!(!a.is_set(ArgSettings::MultipleValues));
        assert!(a.is_set(ArgSettings::TakesValue));
        assert!(a.is_set(ArgSettings::Required));
        assert_eq!(a.val_names.unwrap().values().collect::<Vec<_>>(), [&"opt"]);
        assert!(a.num_vals.is_none());
    }

    #[test]
    fn create_option_usage4() {
        let a = Arg::from("[option] -o [opt]... 'some help info'");
        assert_eq!(a.name, "option");
        assert_eq!(a.short.unwrap(), 'o');
        assert!(a.long.is_none());
        assert_eq!(a.help.unwrap(), "some help info");
        assert!(a.is_set(ArgSettings::MultipleOccurrences));
        assert!(a.is_set(ArgSettings::MultipleValues));
        assert!(a.is_set(ArgSettings::TakesValue));
        assert!(!a.is_set(ArgSettings::Required));
        assert_eq!(a.val_names.unwrap().values().collect::<Vec<_>>(), [&"opt"]);
        assert!(a.num_vals.is_none());
    }

    #[test]
    fn create_option_usage5() {
        let a = Arg::from("[option]... -o [opt] 'some help info'");
        assert_eq!(a.name, "option");
        assert_eq!(a.short.unwrap(), 'o');
        assert!(a.long.is_none());
        assert_eq!(a.help.unwrap(), "some help info");
        assert!(a.is_set(ArgSettings::MultipleOccurrences));
        assert!(a.is_set(ArgSettings::TakesValue));
        assert!(!a.is_set(ArgSettings::Required));
        assert_eq!(a.val_names.unwrap().values().collect::<Vec<_>>(), [&"opt"]);
        assert!(a.num_vals.is_none());
    }

    #[test]
    fn create_option_usage6() {
        let a = Arg::from("-o [opt]... 'some help info'");
        assert_eq!(a.name, "o");
        assert_eq!(a.short.unwrap(), 'o');
        assert!(a.long.is_none());
        assert_eq!(a.help.unwrap(), "some help info");
        assert!(a.is_set(ArgSettings::MultipleOccurrences));
        assert!(a.is_set(ArgSettings::MultipleValues));
        assert!(a.is_set(ArgSettings::TakesValue));
        assert!(!a.is_set(ArgSettings::Required));
        assert_eq!(a.val_names.unwrap().values().collect::<Vec<_>>(), [&"opt"]);
        assert!(a.num_vals.is_none());
    }

    #[test]
    fn create_option_usage7() {
        let a = Arg::from("<option> -o <opt>... 'some help info'");
        assert_eq!(a.name, "option");
        assert_eq!(a.short.unwrap(), 'o');
        assert!(a.long.is_none());
        assert_eq!(a.help.unwrap(), "some help info");
        assert!(a.is_set(ArgSettings::MultipleOccurrences));
        assert!(a.is_set(ArgSettings::MultipleValues));
        assert!(a.is_set(ArgSettings::TakesValue));
        assert!(a.is_set(ArgSettings::Required));
        assert_eq!(a.val_names.unwrap().values().collect::<Vec<_>>(), [&"opt"]);
        assert!(a.num_vals.is_none());
    }

    #[test]
    fn create_option_usage8() {
        let a = Arg::from("<option>... -o <opt> 'some help info'");
        assert_eq!(a.name, "option");
        assert_eq!(a.short.unwrap(), 'o');
        assert!(a.long.is_none());
        assert_eq!(a.help.unwrap(), "some help info");
        assert!(a.is_set(ArgSettings::MultipleOccurrences));
        assert!(a.is_set(ArgSettings::TakesValue));
        assert!(a.is_set(ArgSettings::Required));
        assert_eq!(a.val_names.unwrap().values().collect::<Vec<_>>(), [&"opt"]);
        assert!(a.num_vals.is_none());
    }

    #[test]
    fn create_option_usage9() {
        let a = Arg::from("-o <opt>... 'some help info'");
        assert_eq!(a.name, "o");
        assert_eq!(a.short.unwrap(), 'o');
        assert!(a.long.is_none());
        assert_eq!(a.help.unwrap(), "some help info");
        assert!(a.is_set(ArgSettings::MultipleOccurrences));
        assert!(a.is_set(ArgSettings::MultipleValues));
        assert!(a.is_set(ArgSettings::TakesValue));
        assert!(a.is_set(ArgSettings::Required));
        assert_eq!(a.val_names.unwrap().values().collect::<Vec<_>>(), [&"opt"]);
        assert!(a.num_vals.is_none());
    }

    #[test]
    fn create_option_usage_long1() {
        let a = Arg::from("[option] --opt [opt] 'some help info'");
        assert_eq!(a.name, "option");
        assert_eq!(a.long.unwrap(), "opt");
        assert!(a.short.is_none());
        assert_eq!(a.help.unwrap(), "some help info");
        assert!(!a.is_set(ArgSettings::MultipleOccurrences));
        assert!(!a.is_set(ArgSettings::MultipleValues));
        assert!(a.is_set(ArgSettings::TakesValue));
        assert!(!a.is_set(ArgSettings::Required));
        assert_eq!(a.val_names.unwrap().values().collect::<Vec<_>>(), [&"opt"]);
        assert!(a.num_vals.is_none());
    }

    #[test]
    fn create_option_usage_long2() {
        let a = Arg::from("--opt [option] 'some help info'");
        assert_eq!(a.name, "opt");
        assert_eq!(a.long.unwrap(), "opt");
        assert!(a.short.is_none());
        assert_eq!(a.help.unwrap(), "some help info");
        assert!(!a.is_set(ArgSettings::MultipleOccurrences));
        assert!(!a.is_set(ArgSettings::MultipleValues));
        assert!(a.is_set(ArgSettings::TakesValue));
        assert!(!a.is_set(ArgSettings::Required));
        assert_eq!(
            a.val_names.unwrap().values().collect::<Vec<_>>(),
            [&"option"]
        );
        assert!(a.num_vals.is_none());
    }

    #[test]
    fn create_option_usage_long3() {
        let a = Arg::from("<option> --opt <opt> 'some help info'");
        assert_eq!(a.name, "option");
        assert_eq!(a.long.unwrap(), "opt");
        assert!(a.short.is_none());
        assert_eq!(a.help.unwrap(), "some help info");
        assert!(!a.is_set(ArgSettings::MultipleOccurrences));
        assert!(!a.is_set(ArgSettings::MultipleValues));
        assert!(a.is_set(ArgSettings::TakesValue));
        assert!(a.is_set(ArgSettings::Required));
        assert_eq!(a.val_names.unwrap().values().collect::<Vec<_>>(), [&"opt"]);
        assert!(a.num_vals.is_none());
    }

    #[test]
    fn create_option_usage_long4() {
        let a = Arg::from("--opt <option> 'some help info'");
        assert_eq!(a.name, "opt");
        assert_eq!(a.long.unwrap(), "opt");
        assert!(a.short.is_none());
        assert_eq!(a.help.unwrap(), "some help info");
        assert!(!a.is_set(ArgSettings::MultipleOccurrences));
        assert!(!a.is_set(ArgSettings::MultipleValues));
        assert!(a.is_set(ArgSettings::TakesValue));
        assert!(a.is_set(ArgSettings::Required));
        assert_eq!(
            a.val_names.unwrap().values().collect::<Vec<_>>(),
            [&"option"]
        );
        assert!(a.num_vals.is_none());
    }

    #[test]
    fn create_option_usage_long5() {
        let a = Arg::from("[option] --opt [opt]... 'some help info'");
        assert_eq!(a.name, "option");
        assert_eq!(a.long.unwrap(), "opt");
        assert!(a.short.is_none());
        assert_eq!(a.help.unwrap(), "some help info");
        assert!(a.is_set(ArgSettings::MultipleOccurrences));
        assert!(a.is_set(ArgSettings::MultipleValues));
        assert!(a.is_set(ArgSettings::TakesValue));
        assert!(!a.is_set(ArgSettings::Required));
        assert_eq!(a.val_names.unwrap().values().collect::<Vec<_>>(), [&"opt"]);
        assert!(a.num_vals.is_none());
    }

    #[test]
    fn create_option_usage_long6() {
        let a = Arg::from("[option]... --opt [opt] 'some help info'");
        assert_eq!(a.name, "option");
        assert_eq!(a.long.unwrap(), "opt");
        assert!(a.short.is_none());
        assert_eq!(a.help.unwrap(), "some help info");
        assert!(a.is_set(ArgSettings::MultipleOccurrences));
        assert!(a.is_set(ArgSettings::TakesValue));
        assert!(!a.is_set(ArgSettings::Required));
        assert_eq!(a.val_names.unwrap().values().collect::<Vec<_>>(), [&"opt"]);
        assert!(a.num_vals.is_none());
    }

    #[test]
    fn create_option_usage_long7() {
        let a = Arg::from("--opt [option]... 'some help info'");
        assert_eq!(a.name, "opt");
        assert_eq!(a.long.unwrap(), "opt");
        assert!(a.short.is_none());
        assert_eq!(a.help.unwrap(), "some help info");
        assert!(a.is_set(ArgSettings::MultipleOccurrences));
        assert!(a.is_set(ArgSettings::MultipleValues));
        assert!(a.is_set(ArgSettings::TakesValue));
        assert!(!a.is_set(ArgSettings::Required));
        assert_eq!(
            a.val_names.unwrap().values().collect::<Vec<_>>(),
            [&"option"]
        );
        assert!(a.num_vals.is_none());
    }

    #[test]
    fn create_option_usage_long8() {
        let a = Arg::from("<option> --opt <opt>... 'some help info'");
        assert_eq!(a.name, "option");
        assert_eq!(a.long.unwrap(), "opt");
        assert!(a.short.is_none());
        assert_eq!(a.help.unwrap(), "some help info");
        assert!(a.is_set(ArgSettings::MultipleOccurrences));
        assert!(a.is_set(ArgSettings::MultipleValues));
        assert!(a.is_set(ArgSettings::TakesValue));
        assert!(a.is_set(ArgSettings::Required));
        assert_eq!(a.val_names.unwrap().values().collect::<Vec<_>>(), [&"opt"]);
        assert!(a.num_vals.is_none());
    }

    #[test]
    fn create_option_usage_long9() {
        let a = Arg::from("<option>... --opt <opt> 'some help info'");
        assert_eq!(a.name, "option");
        assert_eq!(a.long.unwrap(), "opt");
        assert!(a.short.is_none());
        assert_eq!(a.help.unwrap(), "some help info");
        assert!(a.is_set(ArgSettings::MultipleOccurrences));
        assert!(a.is_set(ArgSettings::TakesValue));
        assert!(a.is_set(ArgSettings::Required));
        assert_eq!(a.val_names.unwrap().values().collect::<Vec<_>>(), [&"opt"]);
        assert!(a.num_vals.is_none());
    }

    #[test]
    fn create_option_usage_long10() {
        let a = Arg::from("--opt <option>... 'some help info'");
        assert_eq!(a.name, "opt");
        assert_eq!(a.long.unwrap(), "opt");
        assert!(a.short.is_none());
        assert_eq!(a.help.unwrap(), "some help info");
        assert!(
            a.is_set(ArgSettings::MultipleValues) && a.is_set(ArgSettings::MultipleOccurrences)
        );
        assert!(a.is_set(ArgSettings::TakesValue));
        assert!(a.is_set(ArgSettings::Required));
        assert_eq!(
            a.val_names.unwrap().values().collect::<Vec<_>>(),
            [&"option"]
        );
        assert!(a.num_vals.is_none());
    }

    #[test]
    fn create_option_usage_long_equals1() {
        let a = Arg::from("[option] --opt=[opt] 'some help info'");
        assert_eq!(a.name, "option");
        assert_eq!(a.long.unwrap(), "opt");
        assert!(a.short.is_none());
        assert_eq!(a.help.unwrap(), "some help info");
        assert!(
            !(a.is_set(ArgSettings::MultipleValues) || a.is_set(ArgSettings::MultipleOccurrences))
        );
        assert!(a.is_set(ArgSettings::TakesValue));
        assert!(!a.is_set(ArgSettings::Required));
        assert_eq!(a.val_names.unwrap().values().collect::<Vec<_>>(), [&"opt"]);
        assert!(a.num_vals.is_none());
    }

    #[test]
    fn create_option_usage_long_equals2() {
        let a = Arg::from("--opt=[option] 'some help info'");
        assert_eq!(a.name, "opt");
        assert_eq!(a.long.unwrap(), "opt");
        assert!(a.short.is_none());
        assert_eq!(a.help.unwrap(), "some help info");
        assert!(
            !(a.is_set(ArgSettings::MultipleValues) || a.is_set(ArgSettings::MultipleOccurrences))
        );
        assert!(a.is_set(ArgSettings::TakesValue));
        assert!(!a.is_set(ArgSettings::Required));
        assert_eq!(
            a.val_names.unwrap().values().collect::<Vec<_>>(),
            [&"option"]
        );
        assert!(a.num_vals.is_none());
    }

    #[test]
    fn create_option_usage_long_equals3() {
        let a = Arg::from("<option> --opt=<opt> 'some help info'");
        assert_eq!(a.name, "option");
        assert_eq!(a.long.unwrap(), "opt");
        assert!(a.short.is_none());
        assert_eq!(a.help.unwrap(), "some help info");
        assert!(
            !(a.is_set(ArgSettings::MultipleValues) || a.is_set(ArgSettings::MultipleOccurrences))
        );
        assert!(a.is_set(ArgSettings::TakesValue));
        assert!(a.is_set(ArgSettings::Required));
        assert_eq!(a.val_names.unwrap().values().collect::<Vec<_>>(), [&"opt"]);
        assert!(a.num_vals.is_none());
    }

    #[test]
    fn create_option_usage_long_equals4() {
        let a = Arg::from("--opt=<option> 'some help info'");
        assert_eq!(a.name, "opt");
        assert_eq!(a.long.unwrap(), "opt");
        assert!(a.short.is_none());
        assert_eq!(a.help.unwrap(), "some help info");
        assert!(
            !(a.is_set(ArgSettings::MultipleValues) || a.is_set(ArgSettings::MultipleOccurrences))
        );
        assert!(a.is_set(ArgSettings::TakesValue));
        assert!(a.is_set(ArgSettings::Required));
        assert_eq!(
            a.val_names.unwrap().values().collect::<Vec<_>>(),
            [&"option"]
        );
        assert!(a.num_vals.is_none());
    }

    #[test]
    fn create_option_usage_long_equals5() {
        let a = Arg::from("[option] --opt=[opt]... 'some help info'");
        assert_eq!(a.name, "option");
        assert_eq!(a.long.unwrap(), "opt");
        assert!(a.short.is_none());
        assert_eq!(a.help.unwrap(), "some help info");
        assert!(
            a.is_set(ArgSettings::MultipleValues) && a.is_set(ArgSettings::MultipleOccurrences)
        );
        assert!(a.is_set(ArgSettings::TakesValue));
        assert!(!a.is_set(ArgSettings::Required));
        assert_eq!(a.val_names.unwrap().values().collect::<Vec<_>>(), [&"opt"]);
        assert!(a.num_vals.is_none());
    }

    #[test]
    fn create_option_usage_long_equals6() {
        let a = Arg::from("[option]... --opt=[opt] 'some help info'");
        assert_eq!(a.name, "option");
        assert_eq!(a.long.unwrap(), "opt");
        assert!(a.short.is_none());
        assert_eq!(a.help.unwrap(), "some help info");
        assert!(a.is_set(ArgSettings::MultipleOccurrences));
        assert!(a.is_set(ArgSettings::TakesValue));
        assert!(!a.is_set(ArgSettings::Required));
        assert_eq!(a.val_names.unwrap().values().collect::<Vec<_>>(), [&"opt"]);
        assert!(a.num_vals.is_none());
    }

    #[test]
    fn create_option_usage_long_equals7() {
        let a = Arg::from("--opt=[option]... 'some help info'");
        assert_eq!(a.name, "opt");
        assert_eq!(a.long.unwrap(), "opt");
        assert!(a.short.is_none());
        assert_eq!(a.help.unwrap(), "some help info");
        assert!(
            a.is_set(ArgSettings::MultipleValues) && a.is_set(ArgSettings::MultipleOccurrences)
        );
        assert!(a.is_set(ArgSettings::TakesValue));
        assert!(!a.is_set(ArgSettings::Required));
        assert_eq!(
            a.val_names.unwrap().values().collect::<Vec<_>>(),
            [&"option"]
        );
        assert!(a.num_vals.is_none());
    }

    #[test]
    fn create_option_usage_long_equals8() {
        let a = Arg::from("<option> --opt=<opt>... 'some help info'");
        assert_eq!(a.name, "option");
        assert_eq!(a.long.unwrap(), "opt");
        assert!(a.short.is_none());
        assert_eq!(a.help.unwrap(), "some help info");
        assert!(
            a.is_set(ArgSettings::MultipleValues) && a.is_set(ArgSettings::MultipleOccurrences)
        );
        assert!(a.is_set(ArgSettings::TakesValue));
        assert!(a.is_set(ArgSettings::Required));
        assert_eq!(a.val_names.unwrap().values().collect::<Vec<_>>(), [&"opt"]);
        assert!(a.num_vals.is_none());
    }

    #[test]
    fn create_option_usage_long_equals9() {
        let a = Arg::from("<option>... --opt=<opt> 'some help info'");
        assert_eq!(a.name, "option");
        assert_eq!(a.long.unwrap(), "opt");
        assert!(a.short.is_none());
        assert_eq!(a.help.unwrap(), "some help info");
        assert!(a.is_set(ArgSettings::MultipleOccurrences));
        assert!(!a.is_set(ArgSettings::MultipleValues));
        assert!(a.is_set(ArgSettings::TakesValue));
        assert!(a.is_set(ArgSettings::Required));
        assert_eq!(a.val_names.unwrap().values().collect::<Vec<_>>(), [&"opt"]);
        assert!(a.num_vals.is_none());
    }

    #[test]
    fn create_option_usage_long_equals10() {
        let a = Arg::from("--opt=<option>... 'some help info'");
        assert_eq!(a.name, "opt");
        assert_eq!(a.long.unwrap(), "opt");
        assert!(a.short.is_none());
        assert_eq!(a.help.unwrap(), "some help info");
        assert!(
            a.is_set(ArgSettings::MultipleValues) && a.is_set(ArgSettings::MultipleOccurrences)
        );
        assert!(a.is_set(ArgSettings::TakesValue));
        assert!(a.is_set(ArgSettings::Required));
        assert_eq!(
            a.val_names.unwrap().values().collect::<Vec<_>>(),
            [&"option"]
        );
        assert!(a.num_vals.is_none());
    }

    #[test]
    fn create_option_usage_both1() {
        let a = Arg::from("[option] -o --opt [option] 'some help info'");
        assert_eq!(a.name, "option");
        assert_eq!(a.long.unwrap(), "opt");
        assert_eq!(a.short.unwrap(), 'o');
        assert_eq!(a.help.unwrap(), "some help info");
        assert!(
            !(a.is_set(ArgSettings::MultipleValues) || a.is_set(ArgSettings::MultipleOccurrences))
        );
        assert!(a.is_set(ArgSettings::TakesValue));
        assert!(!a.is_set(ArgSettings::Required));
        assert_eq!(
            a.val_names.unwrap().values().collect::<Vec<_>>(),
            [&"option"]
        );
        assert!(a.num_vals.is_none());
    }

    #[test]
    fn create_option_usage_both2() {
        let a = Arg::from("-o --opt [option] 'some help info'");
        assert_eq!(a.name, "opt");
        assert_eq!(a.long.unwrap(), "opt");
        assert_eq!(a.short.unwrap(), 'o');
        assert_eq!(a.help.unwrap(), "some help info");
        assert!(
            !(a.is_set(ArgSettings::MultipleValues) || a.is_set(ArgSettings::MultipleOccurrences))
        );
        assert!(a.is_set(ArgSettings::TakesValue));
        assert!(!a.is_set(ArgSettings::Required));
        assert_eq!(
            a.val_names.unwrap().values().collect::<Vec<_>>(),
            [&"option"]
        );
        assert!(a.num_vals.is_none());
    }

    #[test]
    fn create_option_usage_both3() {
        let a = Arg::from("<option> -o --opt <opt> 'some help info'");
        assert_eq!(a.name, "option");
        assert_eq!(a.long.unwrap(), "opt");
        assert_eq!(a.short.unwrap(), 'o');
        assert_eq!(a.help.unwrap(), "some help info");
        assert!(
            !(a.is_set(ArgSettings::MultipleValues) || a.is_set(ArgSettings::MultipleOccurrences))
        );
        assert!(a.is_set(ArgSettings::TakesValue));
        assert!(a.is_set(ArgSettings::Required));
        assert_eq!(a.val_names.unwrap().values().collect::<Vec<_>>(), [&"opt"]);
        assert!(a.num_vals.is_none());
    }

    #[test]
    fn create_option_usage_both4() {
        let a = Arg::from("-o --opt <option> 'some help info'");
        assert_eq!(a.name, "opt");
        assert_eq!(a.long.unwrap(), "opt");
        assert_eq!(a.short.unwrap(), 'o');
        assert_eq!(a.help.unwrap(), "some help info");
        assert!(
            !(a.is_set(ArgSettings::MultipleValues) || a.is_set(ArgSettings::MultipleOccurrences))
        );
        assert!(a.is_set(ArgSettings::TakesValue));
        assert!(a.is_set(ArgSettings::Required));
        assert_eq!(
            a.val_names.unwrap().values().collect::<Vec<_>>(),
            [&"option"]
        );
        assert!(a.num_vals.is_none());
    }

    #[test]
    fn create_option_usage_both5() {
        let a = Arg::from("[option]... -o --opt [option] 'some help info'");
        assert_eq!(a.name, "option");
        assert_eq!(a.long.unwrap(), "opt");
        assert_eq!(a.short.unwrap(), 'o');
        assert_eq!(a.help.unwrap(), "some help info");
        assert!(a.is_set(ArgSettings::MultipleOccurrences));
        assert!(a.is_set(ArgSettings::TakesValue));
        assert!(!a.is_set(ArgSettings::Required));
        assert_eq!(
            a.val_names.unwrap().values().collect::<Vec<_>>(),
            [&"option"]
        );
        assert!(a.num_vals.is_none());
    }

    #[test]
    fn create_option_usage_both6() {
        let a = Arg::from("-o --opt [option]... 'some help info'");
        assert_eq!(a.name, "opt");
        assert_eq!(a.long.unwrap(), "opt");
        assert_eq!(a.short.unwrap(), 'o');
        assert_eq!(a.help.unwrap(), "some help info");
        assert!(
            a.is_set(ArgSettings::MultipleValues) && a.is_set(ArgSettings::MultipleOccurrences)
        );
        assert!(a.is_set(ArgSettings::TakesValue));
        assert!(!a.is_set(ArgSettings::Required));
        assert_eq!(
            a.val_names.unwrap().values().collect::<Vec<_>>(),
            [&"option"]
        );
        assert!(a.num_vals.is_none());
    }

    #[test]
    fn create_option_usage_both7() {
        let a = Arg::from("<option>... -o --opt <opt> 'some help info'");
        assert_eq!(a.name, "option");
        assert_eq!(a.long.unwrap(), "opt");
        assert_eq!(a.short.unwrap(), 'o');
        assert_eq!(a.help.unwrap(), "some help info");
        assert!(a.is_set(ArgSettings::MultipleOccurrences));
        assert!(a.is_set(ArgSettings::TakesValue));
        assert!(a.is_set(ArgSettings::Required));
        assert_eq!(a.val_names.unwrap().values().collect::<Vec<_>>(), [&"opt"]);
        assert!(a.num_vals.is_none());
    }

    #[test]
    fn create_option_usage_both8() {
        let a = Arg::from("-o --opt <option>... 'some help info'");
        assert_eq!(a.name, "opt");
        assert_eq!(a.long.unwrap(), "opt");
        assert_eq!(a.short.unwrap(), 'o');
        assert_eq!(a.help.unwrap(), "some help info");
        assert!(
            a.is_set(ArgSettings::MultipleValues) && a.is_set(ArgSettings::MultipleOccurrences)
        );
        assert!(a.is_set(ArgSettings::TakesValue));
        assert!(a.is_set(ArgSettings::Required));
        assert_eq!(
            a.val_names.unwrap().values().collect::<Vec<_>>(),
            [&"option"]
        );
        assert!(a.num_vals.is_none());
    }

    #[test]
    fn create_option_usage_both_equals1() {
        let a = Arg::from("[option] -o --opt=[option] 'some help info'");
        assert_eq!(a.name, "option");
        assert_eq!(a.long.unwrap(), "opt");
        assert_eq!(a.short.unwrap(), 'o');
        assert_eq!(a.help.unwrap(), "some help info");
        assert!(
            !(a.is_set(ArgSettings::MultipleValues) || a.is_set(ArgSettings::MultipleOccurrences))
        );
        assert!(a.is_set(ArgSettings::TakesValue));
        assert!(!a.is_set(ArgSettings::Required));
        assert_eq!(
            a.val_names.unwrap().values().collect::<Vec<_>>(),
            [&"option"]
        );
        assert!(a.num_vals.is_none());
    }

    #[test]
    fn create_option_usage_both_equals2() {
        let a = Arg::from("-o --opt=[option] 'some help info'");
        assert_eq!(a.name, "opt");
        assert_eq!(a.long.unwrap(), "opt");
        assert_eq!(a.short.unwrap(), 'o');
        assert_eq!(a.help.unwrap(), "some help info");
        assert!(
            !(a.is_set(ArgSettings::MultipleValues) || a.is_set(ArgSettings::MultipleOccurrences))
        );
        assert!(a.is_set(ArgSettings::TakesValue));
        assert!(!a.is_set(ArgSettings::Required));
        assert_eq!(
            a.val_names.unwrap().values().collect::<Vec<_>>(),
            [&"option"]
        );
        assert!(a.num_vals.is_none());
    }

    #[test]
    fn create_option_usage_both_equals3() {
        let a = Arg::from("<option> -o --opt=<opt> 'some help info'");
        assert_eq!(a.name, "option");
        assert_eq!(a.long.unwrap(), "opt");
        assert_eq!(a.short.unwrap(), 'o');
        assert_eq!(a.help.unwrap(), "some help info");
        assert!(
            !(a.is_set(ArgSettings::MultipleValues) || a.is_set(ArgSettings::MultipleOccurrences))
        );
        assert!(a.is_set(ArgSettings::TakesValue));
        assert!(a.is_set(ArgSettings::Required));
        assert_eq!(a.val_names.unwrap().values().collect::<Vec<_>>(), [&"opt"]);
        assert!(a.num_vals.is_none());
    }

    #[test]
    fn create_option_usage_both_equals4() {
        let a = Arg::from("-o --opt=<option> 'some help info'");
        assert_eq!(a.name, "opt");
        assert_eq!(a.long.unwrap(), "opt");
        assert_eq!(a.short.unwrap(), 'o');
        assert_eq!(a.help.unwrap(), "some help info");
        assert!(
            !(a.is_set(ArgSettings::MultipleValues) || a.is_set(ArgSettings::MultipleOccurrences))
        );
        assert!(a.is_set(ArgSettings::TakesValue));
        assert!(a.is_set(ArgSettings::Required));
        assert_eq!(
            a.val_names.unwrap().values().collect::<Vec<_>>(),
            [&"option"]
        );
        assert!(a.num_vals.is_none());
    }

    #[test]
    fn create_option_usage_both_equals5() {
        let a = Arg::from("[option]... -o --opt=[option] 'some help info'");
        assert_eq!(a.name, "option");
        assert_eq!(a.long.unwrap(), "opt");
        assert_eq!(a.short.unwrap(), 'o');
        assert_eq!(a.help.unwrap(), "some help info");
        assert!(a.is_set(ArgSettings::MultipleOccurrences));
        assert!(a.is_set(ArgSettings::TakesValue));
        assert!(!a.is_set(ArgSettings::Required));
        assert_eq!(
            a.val_names.unwrap().values().collect::<Vec<_>>(),
            [&"option"]
        );
        assert!(a.num_vals.is_none());
    }

    #[test]
    fn create_option_usage_both_equals6() {
        let a = Arg::from("-o --opt=[option]... 'some help info'");
        assert_eq!(a.name, "opt");
        assert_eq!(a.long.unwrap(), "opt");
        assert_eq!(a.short.unwrap(), 'o');
        assert_eq!(a.help.unwrap(), "some help info");
        assert!(
            a.is_set(ArgSettings::MultipleValues) && a.is_set(ArgSettings::MultipleOccurrences)
        );
        assert!(a.is_set(ArgSettings::TakesValue));
        assert!(!a.is_set(ArgSettings::Required));
        assert_eq!(
            a.val_names.unwrap().values().collect::<Vec<_>>(),
            [&"option"]
        );
        assert!(a.num_vals.is_none());
    }

    #[test]
    fn create_option_usage_both_equals7() {
        let a = Arg::from("<option>... -o --opt=<opt> 'some help info'");
        assert_eq!(a.name, "option");
        assert_eq!(a.long.unwrap(), "opt");
        assert_eq!(a.short.unwrap(), 'o');
        assert_eq!(a.help.unwrap(), "some help info");
        assert!(a.is_set(ArgSettings::MultipleOccurrences));
        assert!(a.is_set(ArgSettings::TakesValue));
        assert!(a.is_set(ArgSettings::Required));
        assert_eq!(a.val_names.unwrap().values().collect::<Vec<_>>(), [&"opt"]);
        assert!(a.num_vals.is_none());
    }

    #[test]
    fn create_option_usage_both_equals8() {
        let a = Arg::from("-o --opt=<option>... 'some help info'");
        assert_eq!(a.name, "opt");
        assert_eq!(a.long.unwrap(), "opt");
        assert_eq!(a.short.unwrap(), 'o');
        assert_eq!(a.help.unwrap(), "some help info");
        assert!(
            a.is_set(ArgSettings::MultipleValues) && a.is_set(ArgSettings::MultipleOccurrences)
        );
        assert!(a.is_set(ArgSettings::TakesValue));
        assert!(a.is_set(ArgSettings::Required));
        assert_eq!(
            a.val_names.unwrap().values().collect::<Vec<_>>(),
            [&"option"]
        );
        assert!(a.num_vals.is_none());
    }

    #[test]
    fn create_option_with_vals1() {
        let a = Arg::from("-o <file> <mode> 'some help info'");
        assert_eq!(a.name, "o");
        assert!(a.long.is_none());
        assert_eq!(a.short.unwrap(), 'o');
        assert_eq!(a.help.unwrap(), "some help info");
        assert!(
            !(a.is_set(ArgSettings::MultipleValues) || a.is_set(ArgSettings::MultipleOccurrences))
        );
        assert!(a.is_set(ArgSettings::TakesValue));
        assert!(a.is_set(ArgSettings::Required));
        assert_eq!(
            a.val_names.unwrap().values().collect::<Vec<_>>(),
            [&"file", &"mode"]
        );
        assert_eq!(a.num_vals.unwrap(), 2);
    }

    #[test]
    fn create_option_with_vals2() {
        let a = Arg::from("-o <file> <mode>... 'some help info'");
        assert_eq!(a.name, "o");
        assert!(a.long.is_none());
        assert_eq!(a.short.unwrap(), 'o');
        assert_eq!(a.help.unwrap(), "some help info");
        assert!(
            a.is_set(ArgSettings::MultipleValues) && a.is_set(ArgSettings::MultipleOccurrences)
        );
        assert!(a.is_set(ArgSettings::TakesValue));
        assert!(a.is_set(ArgSettings::Required));
        assert_eq!(
            a.val_names.unwrap().values().collect::<Vec<_>>(),
            [&"file", &"mode"]
        );
        assert_eq!(a.num_vals.unwrap(), 2);
    }

    #[test]
    fn create_option_with_vals3() {
        let a = Arg::from("--opt <file> <mode>... 'some help info'");
        assert_eq!(a.name, "opt");
        assert!(a.short.is_none());
        assert_eq!(a.long.unwrap(), "opt");
        assert_eq!(a.help.unwrap(), "some help info");
        assert!(
            a.is_set(ArgSettings::MultipleValues) && a.is_set(ArgSettings::MultipleOccurrences)
        );
        assert!(a.is_set(ArgSettings::TakesValue));
        assert!(a.is_set(ArgSettings::Required));
        assert_eq!(
            a.val_names.unwrap().values().collect::<Vec<_>>(),
            [&"file", &"mode"]
        );
        assert_eq!(a.num_vals.unwrap(), 2);
    }

    #[test]
    fn create_option_with_vals4() {
        let a = Arg::from("[myopt] --opt <file> <mode> 'some help info'");
        assert_eq!(a.name, "myopt");
        assert!(a.short.is_none());
        assert_eq!(a.long.unwrap(), "opt");
        assert_eq!(a.help.unwrap(), "some help info");
        assert!(
            !(a.is_set(ArgSettings::MultipleValues) || a.is_set(ArgSettings::MultipleOccurrences))
        );
        assert!(a.is_set(ArgSettings::TakesValue));
        assert!(!a.is_set(ArgSettings::Required));
        assert_eq!(
            a.val_names.unwrap().values().collect::<Vec<_>>(),
            [&"file", &"mode"]
        );
        assert_eq!(a.num_vals.unwrap(), 2);
    }

    #[test]
    fn create_option_with_vals5() {
        let a = Arg::from("--opt <file> <mode> 'some help info'");
        assert_eq!(a.name, "opt");
        assert!(a.short.is_none());
        assert_eq!(a.long.unwrap(), "opt");
        assert_eq!(a.help.unwrap(), "some help info");
        assert!(
            !(a.is_set(ArgSettings::MultipleValues) || a.is_set(ArgSettings::MultipleOccurrences))
        );
        assert!(a.is_set(ArgSettings::TakesValue));
        assert!(a.is_set(ArgSettings::Required));
        assert_eq!(a.num_vals.unwrap(), 2);
    }

    #[test]
    fn create_positional_usage() {
        let a = Arg::from("[pos] 'some help info'");
        assert_eq!(a.name, "pos");
        assert_eq!(a.help.unwrap(), "some help info");
        assert!(
            !(a.is_set(ArgSettings::MultipleValues) || a.is_set(ArgSettings::MultipleOccurrences))
        );
        assert!(!a.is_set(ArgSettings::Required));
        assert!(a.val_names.is_none());
        assert!(a.num_vals.is_none());
    }

    #[test]
    fn create_positional_usage0() {
        let a = Arg::from("<pos> 'some help info'");
        assert_eq!(a.name, "pos");
        assert_eq!(a.help.unwrap(), "some help info");
        assert!(
            !(a.is_set(ArgSettings::MultipleValues) || a.is_set(ArgSettings::MultipleOccurrences))
        );
        assert!(a.is_set(ArgSettings::Required));
        assert!(a.val_names.is_none());
        assert!(a.num_vals.is_none());
    }

    #[test]
    fn pos_mult_help() {
        let a = Arg::from("[pos]... 'some help info'");
        assert_eq!(a.name, "pos");
        assert_eq!(a.help.unwrap(), "some help info");
        assert!(
            a.is_set(ArgSettings::MultipleValues) && a.is_set(ArgSettings::MultipleOccurrences)
        );
        assert!(!a.is_set(ArgSettings::Required));
        assert!(a.val_names.is_none());
        assert!(a.num_vals.is_none());
    }

    #[test]
    fn pos_help_lit_single_quote() {
        let a = Arg::from("[pos]... 'some help\' info'");
        assert_eq!(a.name, "pos");
        assert_eq!(a.help.unwrap(), "some help' info");
        assert!(
            a.is_set(ArgSettings::MultipleValues) && a.is_set(ArgSettings::MultipleOccurrences)
        );
        assert!(!a.is_set(ArgSettings::Required));
        assert!(a.val_names.is_none());
        assert!(a.num_vals.is_none());
    }

    #[test]
    fn pos_help_double_lit_single_quote() {
        let a = Arg::from("[pos]... 'some \'help\' info'");
        assert_eq!(a.name, "pos");
        assert_eq!(a.help.unwrap(), "some 'help' info");
        assert!(
            a.is_set(ArgSettings::MultipleValues) && a.is_set(ArgSettings::MultipleOccurrences)
        );
        assert!(!a.is_set(ArgSettings::Required));
        assert!(a.val_names.is_none());
        assert!(a.num_vals.is_none());
    }

    #[test]
    fn pos_help_newline() {
        let a = Arg::from(
            "[pos]... 'some help{n}\
             info'",
        );
        assert_eq!(a.name, "pos");
        assert_eq!(a.help.unwrap(), "some help{n}info");
        assert!(
            a.is_set(ArgSettings::MultipleValues) && a.is_set(ArgSettings::MultipleOccurrences)
        );
        assert!(!a.is_set(ArgSettings::Required));
        assert!(a.val_names.is_none());
        assert!(a.num_vals.is_none());
    }

    #[test]
    fn pos_help_newline_lit_sq() {
        let a = Arg::from(
            "[pos]... 'some help\' stuff{n}\
             info'",
        );
        assert_eq!(a.name, "pos");
        assert_eq!(a.help.unwrap(), "some help' stuff{n}info");
        assert!(
            a.is_set(ArgSettings::MultipleValues) && a.is_set(ArgSettings::MultipleOccurrences)
        );
        assert!(!a.is_set(ArgSettings::Required));
        assert!(a.val_names.is_none());
        assert!(a.num_vals.is_none());
    }

    #[test]
    fn pos_req_mult_help() {
        let a = Arg::from("<pos>... 'some help info'");
        assert_eq!(a.name, "pos");
        assert_eq!(a.help.unwrap(), "some help info");
        assert!(
            a.is_set(ArgSettings::MultipleValues) && a.is_set(ArgSettings::MultipleOccurrences)
        );
        assert!(a.is_set(ArgSettings::Required));
        assert!(a.val_names.is_none());
        assert!(a.num_vals.is_none());
    }

    #[test]
    fn pos_req() {
        let a = Arg::from("<pos>");
        assert_eq!(a.name, "pos");
        assert!(
            !(a.is_set(ArgSettings::MultipleValues) || a.is_set(ArgSettings::MultipleOccurrences))
        );
        assert!(a.is_set(ArgSettings::Required));
        assert!(a.val_names.is_none());
        assert!(a.num_vals.is_none());
    }

    #[test]
    fn pos_mult() {
        let a = Arg::from("[pos]...");
        assert_eq!(a.name, "pos");
        assert!(
            a.is_set(ArgSettings::MultipleValues) && a.is_set(ArgSettings::MultipleOccurrences)
        );
        assert!(!a.is_set(ArgSettings::Required));
        assert!(a.val_names.is_none());
        assert!(a.num_vals.is_none());
    }

    #[test]
    fn nonascii() {
        let a = Arg::from("<ASCII> 'üñíčöĐ€'");
        assert_eq!(a.name, "ASCII");
        assert_eq!(a.help, Some("üñíčöĐ€"));
        let a = Arg::from("<üñíčöĐ€> 'ASCII'");
        assert_eq!(a.name, "üñíčöĐ€");
        assert_eq!(a.help, Some("ASCII"));
        let a = Arg::from("<üñíčöĐ€> 'üñíčöĐ€'");
        assert_eq!(a.name, "üñíčöĐ€");
        assert_eq!(a.help, Some("üñíčöĐ€"));
        let a = Arg::from("-ø 'ø'");
        assert_eq!(a.name, "ø");
        assert_eq!(a.short, Some('ø'));
        assert_eq!(a.help, Some("ø"));
        let a = Arg::from("--üñíčöĐ€ 'Nōṫ ASCII'");
        assert_eq!(a.name, "üñíčöĐ€");
        assert_eq!(a.long, Some("üñíčöĐ€"));
        assert_eq!(a.help, Some("Nōṫ ASCII"));
        let a = Arg::from("[ñämê] --ôpt=[üñíčöĐ€] 'hælp'");
        assert_eq!(a.name, "ñämê");
        assert_eq!(a.long, Some("ôpt"));
        assert_eq!(
            a.val_names.unwrap().values().collect::<Vec<_>>(),
            [&"üñíčöĐ€"]
        );
        assert_eq!(a.help, Some("hælp"));
    }

    #[test]
    fn default_create_flag_usage() {
        let a = Arg::from("[flag] -f @a 'some help info'");
        assert_eq!(a.name, "flag");
        assert_eq!(a.short.unwrap(), 'f');
        assert!(a.long.is_none());
        assert_eq!(a.help.unwrap(), "some help info");
        assert!(!a.is_set(ArgSettings::MultipleOccurrences));
        assert!(a.val_names.is_none());
        assert!(a.num_vals.is_none());
        assert_eq!(a.default_val.unwrap(), "a");
        assert_eq!(a.default_val.unwrap(), "a");

        let b = Arg::from("[flag] @a --flag 'some help info'");
        assert_eq!(b.name, "flag");
        assert_eq!(b.long.unwrap(), "flag");
        assert!(b.short.is_none());
        assert_eq!(b.help.unwrap(), "some help info");
        assert!(!b.is_set(ArgSettings::MultipleOccurrences));
        assert!(a.val_names.is_none());
        assert!(a.num_vals.is_none());
        assert_eq!(a.default_val.unwrap(), "a");
        assert_eq!(a.default_val.unwrap(), "a");

        let b = Arg::from("@a --flag 'some help info'");
        assert_eq!(b.name, "flag");
        assert_eq!(b.long.unwrap(), "flag");
        assert!(b.short.is_none());
        assert_eq!(b.help.unwrap(), "some help info");
        assert!(!b.is_set(ArgSettings::MultipleOccurrences));
        assert!(b.val_names.is_none());
        assert!(b.num_vals.is_none());
        assert_eq!(b.default_val.unwrap(), "a");

        let c = Arg::from("[flag] -f --flag @a 'some help info'");
        assert_eq!(c.name, "flag");
        assert_eq!(c.short.unwrap(), 'f');
        assert_eq!(c.long.unwrap(), "flag");
        assert_eq!(c.help.unwrap(), "some help info");
        assert!(!c.is_set(ArgSettings::MultipleOccurrences));
        assert!(c.val_names.is_none());
        assert!(c.num_vals.is_none());
        assert_eq!(c.default_val.unwrap(), "a");

        let d = Arg::from("[flag] -f... @a 'some help info'");
        assert_eq!(d.name, "flag");
        assert_eq!(d.short.unwrap(), 'f');
        assert!(d.long.is_none());
        assert_eq!(d.help.unwrap(), "some help info");
        assert!(d.is_set(ArgSettings::MultipleOccurrences));
        assert!(d.val_names.is_none());
        assert!(d.num_vals.is_none());
        assert_eq!(d.default_val.unwrap(), "a");

        let e = Arg::from("[flag] -f @a --flag... 'some help info'");
        assert_eq!(e.name, "flag");
        assert_eq!(e.long.unwrap(), "flag");
        assert_eq!(e.short.unwrap(), 'f');
        assert_eq!(e.help.unwrap(), "some help info");
        assert!(e.is_set(ArgSettings::MultipleOccurrences));
        assert!(e.val_names.is_none());
        assert!(e.num_vals.is_none());
        assert_eq!(e.default_val.unwrap(), "a");

        let e = Arg::from("-f @a --flag... 'some help info'");
        assert_eq!(e.name, "flag");
        assert_eq!(e.long.unwrap(), "flag");
        assert_eq!(e.short.unwrap(), 'f');
        assert_eq!(e.help.unwrap(), "some help info");
        assert!(e.is_set(ArgSettings::MultipleOccurrences));
        assert!(e.val_names.is_none());
        assert!(e.num_vals.is_none());
        assert_eq!(e.default_val.unwrap(), "a");

        let e = Arg::from("--flags @a");
        assert_eq!(e.name, "flags");
        assert_eq!(e.long.unwrap(), "flags");
        assert!(e.val_names.is_none());
        assert!(e.num_vals.is_none());
        assert_eq!(e.default_val.unwrap(), "a");

        let e = Arg::from("@a --flags...");
        assert_eq!(e.name, "flags");
        assert_eq!(e.long.unwrap(), "flags");
        assert!(e.is_set(ArgSettings::MultipleOccurrences));
        assert!(e.val_names.is_none());
        assert!(e.num_vals.is_none());
        assert_eq!(e.default_val.unwrap(), "a");

        let e = Arg::from("[flags] -f @a");
        assert_eq!(e.name, "flags");
        assert_eq!(e.short.unwrap(), 'f');
        assert!(e.val_names.is_none());
        assert!(e.num_vals.is_none());
        assert_eq!(e.default_val.unwrap(), "a");

        let e = Arg::from("[flags] @a -f... @b");
        assert_eq!(e.name, "flags");
        assert_eq!(e.short.unwrap(), 'f');
        assert!(e.is_set(ArgSettings::MultipleOccurrences));
        assert!(e.val_names.is_none());
        assert!(e.num_vals.is_none());
        assert_eq!(e.default_val.unwrap(), "b");

        let a = Arg::from("-f @a 'some help info'");
        assert_eq!(a.name, "f");
        assert_eq!(a.short.unwrap(), 'f');
        assert!(a.long.is_none());
        assert_eq!(a.help.unwrap(), "some help info");
        assert!(!a.is_set(ArgSettings::MultipleOccurrences));
        assert!(a.val_names.is_none());
        assert!(a.num_vals.is_none());
        assert_eq!(a.default_val.unwrap(), "a");

        let e = Arg::from("@a -f");
        assert_eq!(e.name, "f");
        assert_eq!(e.short.unwrap(), 'f');
        assert!(e.val_names.is_none());
        assert!(e.num_vals.is_none());
        assert_eq!(e.default_val.unwrap(), "a");

        let e = Arg::from("@a -f...");
        assert_eq!(e.name, "f");
        assert_eq!(e.short.unwrap(), 'f');
        assert!(e.is_set(ArgSettings::MultipleOccurrences));
        assert!(e.val_names.is_none());
        assert!(e.num_vals.is_none());
        assert_eq!(e.default_val.unwrap(), "a");
    }

    #[test]
    fn default_create_option_usage0() {
        // Short only
        let a = Arg::from("[option] @a -o [opt] 'some help info'");
        assert_eq!(a.name, "option");
        assert_eq!(a.short.unwrap(), 'o');
        assert!(a.long.is_none());
        assert_eq!(a.help.unwrap(), "some help info");
        assert!(!a.is_set(ArgSettings::MultipleOccurrences));
        assert!(a.is_set(ArgSettings::TakesValue));
        assert!(!a.is_set(ArgSettings::Required));
        assert_eq!(
            a.val_names.unwrap().values().collect::<Vec<_>>(),
            [&"opt"]
        );
        assert!(a.num_vals.is_none());
        assert_eq!(a.default_val.unwrap(), "a");
    }

    #[test]
    fn default_create_option_usage1() {
        let b = Arg::from("-o [opt] @a 'some help info'");
        assert_eq!(b.name, "o");
        assert_eq!(b.short.unwrap(), 'o');
        assert!(b.long.is_none());
        assert_eq!(b.help.unwrap(), "some help info");
        assert!(!b.is_set(ArgSettings::MultipleOccurrences));
        assert!(b.is_set(ArgSettings::TakesValue));
        assert!(!b.is_set(ArgSettings::Required));
        assert_eq!(
            b.val_names.unwrap().values().collect::<Vec<_>>(),
            [&"opt"]
        );
        assert!(b.num_vals.is_none());
        assert_eq!(b.default_val.unwrap(), "a");
    }

    #[test]
    fn default_create_option_usage2() {
        let c = Arg::from("<option> -o <opt> @a 'some help info'");
        assert_eq!(c.name, "option");
        assert_eq!(c.short.unwrap(), 'o');
        assert!(c.long.is_none());
        assert_eq!(c.help.unwrap(), "some help info");
        assert!(!c.is_set(ArgSettings::MultipleOccurrences));
        assert!(c.is_set(ArgSettings::TakesValue));
        assert!(c.is_set(ArgSettings::Required));
        assert_eq!(
            c.val_names.unwrap().values().collect::<Vec<_>>(),
            [&"opt"]
        );
        assert!(c.num_vals.is_none());
        assert_eq!(c.default_val.unwrap(), "a");
    }

    #[test]
    fn default_create_option_usage3() {
        let d = Arg::from("-o <opt> @a 'some help info'");
        assert_eq!(d.name, "o");
        assert_eq!(d.short.unwrap(), 'o');
        assert!(d.long.is_none());
        assert_eq!(d.help.unwrap(), "some help info");
        assert!(!d.is_set(ArgSettings::MultipleOccurrences));
        assert!(d.is_set(ArgSettings::TakesValue));
        assert!(d.is_set(ArgSettings::Required));
        assert_eq!(
            d.val_names.unwrap().values().collect::<Vec<_>>(),
            [&"opt"]
        );
        assert!(d.num_vals.is_none());
        assert_eq!(d.default_val.unwrap(), "a");
    }

    #[test]
    fn default_create_option_usage4() {
        let a = Arg::from("[option] -o [opt]... @a 'some help info'");
        assert_eq!(a.name, "option");
        assert_eq!(a.short.unwrap(), 'o');
        assert!(a.long.is_none());
        assert_eq!(a.help.unwrap(), "some help info");
        assert!(a.is_set(ArgSettings::MultipleOccurrences));
        assert!(a.is_set(ArgSettings::TakesValue));
        assert!(!a.is_set(ArgSettings::Required));
        assert_eq!(
            a.val_names.unwrap().values().collect::<Vec<_>>(),
            [&"opt"]
        );
        assert!(a.num_vals.is_none());
        assert_eq!(a.default_val.unwrap(), "a");
    }

    #[test]
    fn default_create_option_usage5() {
        let a = Arg::from("[option]... @a -o [opt] 'some help info'");
        assert_eq!(a.name, "option");
        assert_eq!(a.short.unwrap(), 'o');
        assert!(a.long.is_none());
        assert_eq!(a.help.unwrap(), "some help info");
        assert!(a.is_set(ArgSettings::MultipleOccurrences));
        assert!(a.is_set(ArgSettings::TakesValue));
        assert!(!a.is_set(ArgSettings::Required));
        assert_eq!(
            a.val_names.unwrap().values().collect::<Vec<_>>(),
            [&"opt"]
        );
        assert!(a.num_vals.is_none());
        assert_eq!(a.default_val.unwrap(), "a");
    }

    #[test]
    fn default_create_option_usage6() {
        let b = Arg::from("-o @a [opt]... 'some help info'");
        assert_eq!(b.name, "o");
        assert_eq!(b.short.unwrap(), 'o');
        assert!(b.long.is_none());
        assert_eq!(b.help.unwrap(), "some help info");
        assert!(b.is_set(ArgSettings::MultipleOccurrences));
        assert!(b.is_set(ArgSettings::TakesValue));
        assert!(!b.is_set(ArgSettings::Required));
        assert_eq!(
            b.val_names.unwrap().values().collect::<Vec<_>>(),
            [&"opt"]
        );
        assert!(b.num_vals.is_none());
        assert_eq!(b.default_val.unwrap(), "a");
    }

    #[test]
    fn default_create_option_usage7() {
        let c = Arg::from("<option> -o <opt>... @a 'some help info'");
        assert_eq!(c.name, "option");
        assert_eq!(c.short.unwrap(), 'o');
        assert!(c.long.is_none());
        assert_eq!(c.help.unwrap(), "some help info");
        assert!(c.is_set(ArgSettings::MultipleOccurrences));
        assert!(c.is_set(ArgSettings::TakesValue));
        assert!(c.is_set(ArgSettings::Required));
        assert_eq!(
            c.val_names.unwrap().values().collect::<Vec<_>>(),
            [&"opt"]
        );
        assert!(c.num_vals.is_none());
        assert_eq!(c.default_val.unwrap(), "a");
    }

    #[test]
    fn default_create_option_usage8() {
        let c = Arg::from("<option>... -o @a <opt> 'some help info'");
        assert_eq!(c.name, "option");
        assert_eq!(c.short.unwrap(), 'o');
        assert!(c.long.is_none());
        assert_eq!(c.help.unwrap(), "some help info");
        assert!(c.is_set(ArgSettings::MultipleOccurrences));
        assert!(c.is_set(ArgSettings::TakesValue));
        assert!(c.is_set(ArgSettings::Required));
        assert_eq!(
            c.val_names.unwrap().values().collect::<Vec<_>>(),
            [&"opt"]
        );
        assert!(c.num_vals.is_none());
        assert_eq!(c.default_val.unwrap(), "a");
    }

    #[test]
    fn default_create_option_usage9() {
        let d = Arg::from("-o @a <opt>... 'some help info'");
        assert_eq!(d.name, "o");
        assert_eq!(d.short.unwrap(), 'o');
        assert!(d.long.is_none());
        assert_eq!(d.help.unwrap(), "some help info");
        assert!(d.is_set(ArgSettings::MultipleOccurrences));
        assert!(d.is_set(ArgSettings::TakesValue));
        assert!(d.is_set(ArgSettings::Required));
        assert_eq!(
            d.val_names.unwrap().values().collect::<Vec<_>>(),
            [&"opt"]
        );
        assert!(d.num_vals.is_none());
        assert_eq!(d.default_val.unwrap(), "a");
    }

    #[test]
    fn default_create_option_usage_long1() {
        let a = Arg::from("[option] @a --opt [opt] 'some help info'");
        assert_eq!(a.name, "option");
        assert_eq!(a.long.unwrap(), "opt");
        assert!(a.short.is_none());
        assert_eq!(a.help.unwrap(), "some help info");
        assert!(!a.is_set(ArgSettings::MultipleOccurrences));
        assert!(a.is_set(ArgSettings::TakesValue));
        assert!(!a.is_set(ArgSettings::Required));
        assert_eq!(
            a.val_names.unwrap().values().collect::<Vec<_>>(),
            [&"opt"]
        );
        assert!(a.num_vals.is_none());
        assert_eq!(a.default_val.unwrap(), "a");
    }

    #[test]
    fn default_create_option_usage_long2() {
        let b = Arg::from("--opt [option] @a 'some help info'");
        assert_eq!(b.name, "opt");
        assert_eq!(b.long.unwrap(), "opt");
        assert!(b.short.is_none());
        assert_eq!(b.help.unwrap(), "some help info");
        assert!(!b.is_set(ArgSettings::MultipleOccurrences));
        assert!(b.is_set(ArgSettings::TakesValue));
        assert!(!b.is_set(ArgSettings::Required));
        assert_eq!(
            b.val_names.unwrap().values().collect::<Vec<_>>(),
            [&"option"]
        );
        assert!(b.num_vals.is_none());
        assert_eq!(b.default_val.unwrap(), "a");
    }

    #[test]
    fn default_create_option_usage_long3() {
        let c = Arg::from("<option> --opt <opt> @a 'some help info'");
        assert_eq!(c.name, "option");
        assert_eq!(c.long.unwrap(), "opt");
        assert!(c.short.is_none());
        assert_eq!(c.help.unwrap(), "some help info");
        assert!(!c.is_set(ArgSettings::MultipleOccurrences));
        assert!(c.is_set(ArgSettings::TakesValue));
        assert!(c.is_set(ArgSettings::Required));
        assert_eq!(
            c.val_names.unwrap().values().collect::<Vec<_>>(),
            [&"opt"]
        );
        assert!(c.num_vals.is_none());
        assert_eq!(c.default_val.unwrap(), "a");
    }

    #[test]
    fn default_create_option_usage_long4() {
        let d = Arg::from("--opt <option> @a 'some help info'");
        assert_eq!(d.name, "opt");
        assert_eq!(d.long.unwrap(), "opt");
        assert!(d.short.is_none());
        assert_eq!(d.help.unwrap(), "some help info");
        assert!(!d.is_set(ArgSettings::MultipleOccurrences));
        assert!(d.is_set(ArgSettings::TakesValue));
        assert!(d.is_set(ArgSettings::Required));
        assert_eq!(
            d.val_names.unwrap().values().collect::<Vec<_>>(),
            [&"option"]
        );
        assert!(d.num_vals.is_none());
        assert_eq!(d.default_val.unwrap(), "a");
    }

    #[test]
    fn default_create_option_usage_long5() {
        let a = Arg::from("[option] --opt @a [opt]... 'some help info'");
        assert_eq!(a.name, "option");
        assert_eq!(a.long.unwrap(), "opt");
        assert!(a.short.is_none());
        assert_eq!(a.help.unwrap(), "some help info");
        assert!(a.is_set(ArgSettings::MultipleOccurrences));
        assert!(a.is_set(ArgSettings::TakesValue));
        assert!(!a.is_set(ArgSettings::Required));
        assert_eq!(
            a.val_names.unwrap().values().collect::<Vec<_>>(),
            [&"opt"]
        );
        assert!(a.num_vals.is_none());
        assert_eq!(a.default_val.unwrap(), "a");
    }

    #[test]
    fn default_create_option_usage_long6() {
        let a = Arg::from("[option]... --opt [opt] @a 'some help info'");
        assert_eq!(a.name, "option");
        assert_eq!(a.long.unwrap(), "opt");
        assert!(a.short.is_none());
        assert_eq!(a.help.unwrap(), "some help info");
        assert!(a.is_set(ArgSettings::MultipleOccurrences));
        assert!(a.is_set(ArgSettings::TakesValue));
        assert!(!a.is_set(ArgSettings::Required));
        assert_eq!(
            a.val_names.unwrap().values().collect::<Vec<_>>(),
            [&"opt"]
        );
        assert!(a.num_vals.is_none());
        assert_eq!(a.default_val.unwrap(), "a");
    }

    #[test]
    fn default_create_option_usage_long7() {
        let b = Arg::from("@a --opt [option]... 'some help info'");
        assert_eq!(b.name, "opt");
        assert_eq!(b.long.unwrap(), "opt");
        assert!(b.short.is_none());
        assert_eq!(b.help.unwrap(), "some help info");
        assert!(b.is_set(ArgSettings::MultipleOccurrences));
        assert!(b.is_set(ArgSettings::TakesValue));
        assert!(!b.is_set(ArgSettings::Required));
        assert_eq!(
            b.val_names.unwrap().values().collect::<Vec<_>>(),
            [&"option"]
        );
        assert!(b.num_vals.is_none());
        assert_eq!(b.default_val.unwrap(), "a");
    }

    #[test]
    fn default_create_option_usage_long8() {
        let c = Arg::from("<option> --opt @a <opt>... 'some help info'");
        assert_eq!(c.name, "option");
        assert_eq!(c.long.unwrap(), "opt");
        assert!(c.short.is_none());
        assert_eq!(c.help.unwrap(), "some help info");
        assert!(c.is_set(ArgSettings::MultipleOccurrences));
        assert!(c.is_set(ArgSettings::TakesValue));
        assert!(c.is_set(ArgSettings::Required));
        assert_eq!(
            c.val_names.unwrap().values().collect::<Vec<_>>(),
            [&"opt"]
        );
        assert!(c.num_vals.is_none());
        assert_eq!(c.default_val.unwrap(), "a");
    }

    #[test]
    fn default_create_option_usage_long9() {
        let c = Arg::from("<option>... @a --opt <opt> 'some help info'");
        assert_eq!(c.name, "option");
        assert_eq!(c.long.unwrap(), "opt");
        assert!(c.short.is_none());
        assert_eq!(c.help.unwrap(), "some help info");
        assert!(c.is_set(ArgSettings::MultipleOccurrences));
        assert!(c.is_set(ArgSettings::TakesValue));
        assert!(c.is_set(ArgSettings::Required));
        assert_eq!(
            c.val_names.unwrap().values().collect::<Vec<_>>(),
            [&"opt"]
        );
        assert!(c.num_vals.is_none());
        assert_eq!(c.default_val.unwrap(), "a");
    }

    #[test]
    fn default_create_option_usage_long10() {
        let d = Arg::from("@a --opt <option>... 'some help info'");
        assert_eq!(d.name, "opt");
        assert_eq!(d.long.unwrap(), "opt");
        assert!(d.short.is_none());
        assert_eq!(d.help.unwrap(), "some help info");
        assert!(d.is_set(ArgSettings::MultipleOccurrences));
        assert!(d.is_set(ArgSettings::TakesValue));
        assert!(d.is_set(ArgSettings::Required));
        assert_eq!(
            d.val_names.unwrap().values().collect::<Vec<_>>(),
            [&"option"]
        );
        assert!(d.num_vals.is_none());
        assert_eq!(d.default_val.unwrap(), "a");
    }

    #[test]
    fn default_create_option_usage_long_equals1() {
        let a = Arg::from("[option] --opt=[opt] @a 'some help info'");
        assert_eq!(a.name, "option");
        assert_eq!(a.long.unwrap(), "opt");
        assert!(a.short.is_none());
        assert_eq!(a.help.unwrap(), "some help info");
        assert!(!a.is_set(ArgSettings::MultipleOccurrences));
        assert!(a.is_set(ArgSettings::TakesValue));
        assert!(!a.is_set(ArgSettings::Required));
        assert_eq!(
            a.val_names.unwrap().values().collect::<Vec<_>>(),
            [&"opt"]
        );
        assert!(a.num_vals.is_none());
        assert_eq!(a.default_val.unwrap(), "a");
    }

    #[test]
    fn default_create_option_usage_long_equals2() {
        let b = Arg::from("@a --opt=[option] 'some help info'");
        assert_eq!(b.name, "opt");
        assert_eq!(b.long.unwrap(), "opt");
        assert!(b.short.is_none());
        assert_eq!(b.help.unwrap(), "some help info");
        assert!(!b.is_set(ArgSettings::MultipleOccurrences));
        assert!(b.is_set(ArgSettings::TakesValue));
        assert!(!b.is_set(ArgSettings::Required));
        assert_eq!(
            b.val_names.unwrap().values().collect::<Vec<_>>(),
            [&"option"]
        );
        assert!(b.num_vals.is_none());
        assert_eq!(b.default_val.unwrap(), "a");
    }

    #[test]
    fn default_create_option_usage_long_equals3() {
        let c = Arg::from("<option> --opt=<opt> @a 'some help info'");
        assert_eq!(c.name, "option");
        assert_eq!(c.long.unwrap(), "opt");
        assert!(c.short.is_none());
        assert_eq!(c.help.unwrap(), "some help info");
        assert!(!c.is_set(ArgSettings::MultipleOccurrences));
        assert!(c.is_set(ArgSettings::TakesValue));
        assert!(c.is_set(ArgSettings::Required));
        assert_eq!(
            c.val_names.unwrap().values().collect::<Vec<_>>(),
            [&"opt"]
        );
        assert!(c.num_vals.is_none());
        assert_eq!(c.default_val.unwrap(), "a");
    }

    #[test]
    fn default_create_option_usage_long_equals4() {
        let d = Arg::from("@a --opt=<option> 'some help info'");
        assert_eq!(d.name, "opt");
        assert_eq!(d.long.unwrap(), "opt");
        assert!(d.short.is_none());
        assert_eq!(d.help.unwrap(), "some help info");
        assert!(!d.is_set(ArgSettings::MultipleOccurrences));
        assert!(d.is_set(ArgSettings::TakesValue));
        assert!(d.is_set(ArgSettings::Required));
        assert_eq!(
            d.val_names.unwrap().values().collect::<Vec<_>>(),
            [&"option"]
        );
        assert!(d.num_vals.is_none());
        assert_eq!(d.default_val.unwrap(), "a");
    }

    #[test]
    fn default_create_option_usage_long_equals5() {
        let a = Arg::from("[option] --opt=[opt]... @a 'some help info'");
        assert_eq!(a.name, "option");
        assert_eq!(a.long.unwrap(), "opt");
        assert!(a.short.is_none());
        assert_eq!(a.help.unwrap(), "some help info");
        assert!(a.is_set(ArgSettings::MultipleOccurrences));
        assert!(a.is_set(ArgSettings::TakesValue));
        assert!(!a.is_set(ArgSettings::Required));
        assert_eq!(
            a.val_names.unwrap().values().collect::<Vec<_>>(),
            [&"opt"]
        );
        assert!(a.num_vals.is_none());
        assert_eq!(a.default_val.unwrap(), "a");
    }

    #[test]
    fn default_create_option_usage_long_equals6() {
        let a = Arg::from("[option]... --opt=[opt] @a 'some help info'");
        assert_eq!(a.name, "option");
        assert_eq!(a.long.unwrap(), "opt");
        assert!(a.short.is_none());
        assert_eq!(a.help.unwrap(), "some help info");
        assert!(a.is_set(ArgSettings::MultipleOccurrences));
        assert!(a.is_set(ArgSettings::TakesValue));
        assert!(!a.is_set(ArgSettings::Required));
        assert_eq!(
            a.val_names.unwrap().values().collect::<Vec<_>>(),
            [&"opt"]
        );
        assert!(a.num_vals.is_none());
        assert_eq!(a.default_val.unwrap(), "a");
    }

    #[test]
    fn default_create_option_usage_long_equals7() {
        let b = Arg::from("@a --opt=[option]... 'some help info'");
        assert_eq!(b.name, "opt");
        assert_eq!(b.long.unwrap(), "opt");
        assert!(b.short.is_none());
        assert_eq!(b.help.unwrap(), "some help info");
        assert!(b.is_set(ArgSettings::MultipleOccurrences));
        assert!(b.is_set(ArgSettings::TakesValue));
        assert!(!b.is_set(ArgSettings::Required));
        assert_eq!(
            b.val_names.unwrap().values().collect::<Vec<_>>(),
            [&"option"]
        );
        assert!(b.num_vals.is_none());
        assert_eq!(b.default_val.unwrap(), "a");
    }

    #[test]
    fn default_create_option_usage_long_equals8() {
        let c = Arg::from("<option> @a --opt=<opt>... 'some help info'");
        assert_eq!(c.name, "option");
        assert_eq!(c.long.unwrap(), "opt");
        assert!(c.short.is_none());
        assert_eq!(c.help.unwrap(), "some help info");
        assert!(c.is_set(ArgSettings::MultipleOccurrences));
        assert!(c.is_set(ArgSettings::TakesValue));
        assert!(c.is_set(ArgSettings::Required));
        assert_eq!(
            c.val_names.unwrap().values().collect::<Vec<_>>(),
            [&"opt"]
        );
        assert!(c.num_vals.is_none());
        assert_eq!(c.default_val.unwrap(), "a");
    }

    #[test]
    fn default_create_option_usage_long_equals9() {
        let c = Arg::from("<option>... --opt=<opt> @a 'some help info'");
        assert_eq!(c.name, "option");
        assert_eq!(c.long.unwrap(), "opt");
        assert!(c.short.is_none());
        assert_eq!(c.help.unwrap(), "some help info");
        assert!(c.is_set(ArgSettings::MultipleOccurrences));
        assert!(c.is_set(ArgSettings::TakesValue));
        assert!(c.is_set(ArgSettings::Required));
        assert_eq!(
            c.val_names.unwrap().values().collect::<Vec<_>>(),
            [&"opt"]
        );
        assert!(c.num_vals.is_none());
        assert_eq!(c.default_val.unwrap(), "a");
    }

    #[test]
    fn default_create_option_usage_long_equals10() {
        let d = Arg::from("@a --opt=<option>... 'some help info'");
        assert_eq!(d.name, "opt");
        assert_eq!(d.long.unwrap(), "opt");
        assert!(d.short.is_none());
        assert_eq!(d.help.unwrap(), "some help info");
        assert!(d.is_set(ArgSettings::MultipleOccurrences));
        assert!(d.is_set(ArgSettings::TakesValue));
        assert!(d.is_set(ArgSettings::Required));
        assert_eq!(
            d.val_names.unwrap().values().collect::<Vec<_>>(),
            [&"option"]
        );
        assert!(d.num_vals.is_none());
        assert_eq!(d.default_val.unwrap(), "a");
    }

    #[test]
    fn default_create_option_usage_both1() {
        let a = Arg::from("[option] -o @a --opt [option] 'some help info'");
        assert_eq!(a.name, "option");
        assert_eq!(a.long.unwrap(), "opt");
        assert_eq!(a.short.unwrap(), 'o');
        assert_eq!(a.help.unwrap(), "some help info");
        assert!(!a.is_set(ArgSettings::MultipleOccurrences));
        assert!(a.is_set(ArgSettings::TakesValue));
        assert!(!a.is_set(ArgSettings::Required));
        assert_eq!(
            a.val_names.unwrap().values().collect::<Vec<_>>(),
            [&"option"]
        );
        assert!(a.num_vals.is_none());
        assert_eq!(a.default_val.unwrap(), "a");
    }

    #[test]
    fn default_create_option_usage_both2() {
        let b = Arg::from("@a -o --opt [option] 'some help info'");
        assert_eq!(b.name, "opt");
        assert_eq!(b.long.unwrap(), "opt");
        assert_eq!(b.short.unwrap(), 'o');
        assert_eq!(b.help.unwrap(), "some help info");
        assert!(!b.is_set(ArgSettings::MultipleOccurrences));
        assert!(b.is_set(ArgSettings::TakesValue));
        assert!(!b.is_set(ArgSettings::Required));
        assert_eq!(
            b.val_names.unwrap().values().collect::<Vec<_>>(),
            [&"option"]
        );
        assert!(b.num_vals.is_none());
        assert_eq!(b.default_val.unwrap(), "a");
    }

    #[test]
    fn default_create_option_usage_both3() {
        let c = Arg::from("<option> -o @a --opt <opt> 'some help info'");
        assert_eq!(c.name, "option");
        assert_eq!(c.long.unwrap(), "opt");
        assert_eq!(c.short.unwrap(), 'o');
        assert_eq!(c.help.unwrap(), "some help info");
        assert!(!c.is_set(ArgSettings::MultipleOccurrences));
        assert!(c.is_set(ArgSettings::TakesValue));
        assert!(c.is_set(ArgSettings::Required));
        assert_eq!(
            c.val_names.unwrap().values().collect::<Vec<_>>(),
            [&"opt"]
        );
        assert!(c.num_vals.is_none());
        assert_eq!(c.default_val.unwrap(), "a");
    }

    #[test]
    fn default_create_option_usage_both4() {
        let d = Arg::from("@a -o --opt <option> 'some help info'");
        assert_eq!(d.name, "opt");
        assert_eq!(d.long.unwrap(), "opt");
        assert_eq!(d.short.unwrap(), 'o');
        assert_eq!(d.help.unwrap(), "some help info");
        assert!(!d.is_set(ArgSettings::MultipleOccurrences));
        assert!(d.is_set(ArgSettings::TakesValue));
        assert!(d.is_set(ArgSettings::Required));
        assert_eq!(
            d.val_names.unwrap().values().collect::<Vec<_>>(),
            [&"option"]
        );
        assert!(d.num_vals.is_none());
        assert_eq!(d.default_val.unwrap(), "a");
    }

    #[test]
    fn default_create_option_usage_both5() {
        let a = Arg::from("[option]... @a -o --opt [option] 'some help info'");
        assert_eq!(a.name, "option");
        assert_eq!(a.long.unwrap(), "opt");
        assert_eq!(a.short.unwrap(), 'o');
        assert_eq!(a.help.unwrap(), "some help info");
        assert!(a.is_set(ArgSettings::MultipleOccurrences));
        assert!(a.is_set(ArgSettings::TakesValue));
        assert!(!a.is_set(ArgSettings::Required));
        assert_eq!(
            a.val_names.unwrap().values().collect::<Vec<_>>(),
            [&"option"]
        );
        assert!(a.num_vals.is_none());
        assert_eq!(a.default_val.unwrap(), "a");
    }

    #[test]
    fn default_create_option_usage_both6() {
        let b = Arg::from("@a -o --opt [option]... 'some help info'");
        assert_eq!(b.name, "opt");
        assert_eq!(b.long.unwrap(), "opt");
        assert_eq!(b.short.unwrap(), 'o');
        assert_eq!(b.help.unwrap(), "some help info");
        assert!(b.is_set(ArgSettings::MultipleOccurrences));
        assert!(b.is_set(ArgSettings::TakesValue));
        assert!(!b.is_set(ArgSettings::Required));
        assert_eq!(
            b.val_names.unwrap().values().collect::<Vec<_>>(),
            [&"option"]
        );
        assert!(b.num_vals.is_none());
        assert_eq!(b.default_val.unwrap(), "a");
    }

    #[test]
    fn default_create_option_usage_both7() {
        let c = Arg::from("<option>... -o --opt <opt> @a 'some help info'");
        assert_eq!(c.name, "option");
        assert_eq!(c.long.unwrap(), "opt");
        assert_eq!(c.short.unwrap(), 'o');
        assert_eq!(c.help.unwrap(), "some help info");
        assert!(c.is_set(ArgSettings::MultipleOccurrences));
        assert!(c.is_set(ArgSettings::TakesValue));
        assert!(c.is_set(ArgSettings::Required));
        assert_eq!(
            c.val_names.unwrap().values().collect::<Vec<_>>(),
            [&"opt"]
        );
        assert!(c.num_vals.is_none());
        assert_eq!(c.default_val.unwrap(), "a");
    }

    #[test]
    fn default_create_option_usage_both8() {
        let d = Arg::from("@a -o --opt <option>... 'some help info'");
        assert_eq!(d.name, "opt");
        assert_eq!(d.long.unwrap(), "opt");
        assert_eq!(d.short.unwrap(), 'o');
        assert_eq!(d.help.unwrap(), "some help info");
        assert!(d.is_set(ArgSettings::MultipleOccurrences));
        assert!(d.is_set(ArgSettings::TakesValue));
        assert!(d.is_set(ArgSettings::Required));
        assert_eq!(
            d.val_names.unwrap().values().collect::<Vec<_>>(),
            [&"option"]
        );
        assert!(d.num_vals.is_none());
        assert_eq!(d.default_val.unwrap(), "a");
    }

    #[test]
    fn default_create_option_usage_both_equals1() {
        let a = Arg::from("[option] -o --opt=[option] @a 'some help info'");
        assert_eq!(a.name, "option");
        assert_eq!(a.long.unwrap(), "opt");
        assert_eq!(a.short.unwrap(), 'o');
        assert_eq!(a.help.unwrap(), "some help info");
        assert!(!a.is_set(ArgSettings::MultipleOccurrences));
        assert!(a.is_set(ArgSettings::TakesValue));
        assert!(!a.is_set(ArgSettings::Required));
        assert_eq!(
            a.val_names.unwrap().values().collect::<Vec<_>>(),
            [&"option"]
        );
        assert!(a.num_vals.is_none());
        assert_eq!(a.default_val.unwrap(), "a");
    }

    #[test]
    fn default_create_option_usage_both_equals2() {
        let b = Arg::from("@a -o --opt=[option] 'some help info'");
        assert_eq!(b.name, "opt");
        assert_eq!(b.long.unwrap(), "opt");
        assert_eq!(b.short.unwrap(), 'o');
        assert_eq!(b.help.unwrap(), "some help info");
        assert!(!b.is_set(ArgSettings::MultipleOccurrences));
        assert!(b.is_set(ArgSettings::TakesValue));
        assert!(!b.is_set(ArgSettings::Required));
        assert_eq!(
            b.val_names.unwrap().values().collect::<Vec<_>>(),
            [&"option"]
        );
        assert!(b.num_vals.is_none());
        assert_eq!(b.default_val.unwrap(), "a");
    }

    #[test]
    fn default_create_option_usage_both_equals3() {
        let c = Arg::from("<option> -o --opt=<opt> @a 'some help info'");
        assert_eq!(c.name, "option");
        assert_eq!(c.long.unwrap(), "opt");
        assert_eq!(c.short.unwrap(), 'o');
        assert_eq!(c.help.unwrap(), "some help info");
        assert!(!c.is_set(ArgSettings::MultipleOccurrences));
        assert!(c.is_set(ArgSettings::TakesValue));
        assert!(c.is_set(ArgSettings::Required));
        assert_eq!(
            c.val_names.unwrap().values().collect::<Vec<_>>(),
            [&"opt"]
        );
        assert!(c.num_vals.is_none());
        assert_eq!(c.default_val.unwrap(), "a");
    }

    #[test]
    fn default_create_option_usage_both_equals4() {
        let d = Arg::from("@a -o --opt=<option> 'some help info'");
        assert_eq!(d.name, "opt");
        assert_eq!(d.long.unwrap(), "opt");
        assert_eq!(d.short.unwrap(), 'o');
        assert_eq!(d.help.unwrap(), "some help info");
        assert!(!d.is_set(ArgSettings::MultipleOccurrences));
        assert!(d.is_set(ArgSettings::TakesValue));
        assert!(d.is_set(ArgSettings::Required));
        assert_eq!(
            d.val_names.unwrap().values().collect::<Vec<_>>(),
            [&"option"]
        );
        assert!(d.num_vals.is_none());
        assert_eq!(d.default_val.unwrap(), "a");
    }

    #[test]
    fn default_create_option_usage_both_equals5() {
        let a = Arg::from("[option]... -o --opt=[option] @a 'some help info'");
        assert_eq!(a.name, "option");
        assert_eq!(a.long.unwrap(), "opt");
        assert_eq!(a.short.unwrap(), 'o');
        assert_eq!(a.help.unwrap(), "some help info");
        assert!(a.is_set(ArgSettings::MultipleOccurrences));
        assert!(a.is_set(ArgSettings::TakesValue));
        assert!(!a.is_set(ArgSettings::Required));
        assert_eq!(
            a.val_names.unwrap().values().collect::<Vec<_>>(),
            [&"option"]
        );
        assert!(a.num_vals.is_none());
        assert_eq!(a.default_val.unwrap(), "a");
    }

    #[test]
    fn default_create_option_usage_both_equals6() {
        let b = Arg::from("@a -o --opt=[option]... 'some help info'");
        assert_eq!(b.name, "opt");
        assert_eq!(b.long.unwrap(), "opt");
        assert_eq!(b.short.unwrap(), 'o');
        assert_eq!(b.help.unwrap(), "some help info");
        assert!(b.is_set(ArgSettings::MultipleOccurrences));
        assert!(b.is_set(ArgSettings::TakesValue));
        assert!(!b.is_set(ArgSettings::Required));
        assert_eq!(
            b.val_names.unwrap().values().collect::<Vec<_>>(),
            [&"option"]
        );
        assert!(b.num_vals.is_none());
        assert_eq!(b.default_val.unwrap(), "a");
    }

    #[test]
    fn default_create_option_usage_both_equals7() {
        let c = Arg::from("<option>... -o @a --opt=<opt> 'some help info'");
        assert_eq!(c.name, "option");
        assert_eq!(c.long.unwrap(), "opt");
        assert_eq!(c.short.unwrap(), 'o');
        assert_eq!(c.help.unwrap(), "some help info");
        assert!(c.is_set(ArgSettings::MultipleOccurrences));
        assert!(c.is_set(ArgSettings::TakesValue));
        assert!(c.is_set(ArgSettings::Required));
        assert_eq!(
            c.val_names.unwrap().values().collect::<Vec<_>>(),
            [&"opt"]
        );
        assert!(c.num_vals.is_none());
        assert_eq!(c.default_val.unwrap(), "a");
    }

    #[test]
    fn default_create_option_usage_both_equals8() {
        let d = Arg::from("@a -o --opt=<option>... 'some help info'");
        assert_eq!(d.name, "opt");
        assert_eq!(d.long.unwrap(), "opt");
        assert_eq!(d.short.unwrap(), 'o');
        assert_eq!(d.help.unwrap(), "some help info");
        assert!(d.is_set(ArgSettings::MultipleOccurrences));
        assert!(d.is_set(ArgSettings::TakesValue));
        assert!(d.is_set(ArgSettings::Required));
        assert_eq!(
            d.val_names.unwrap().values().collect::<Vec<_>>(),
            [&"option"]
        );
        assert!(d.num_vals.is_none());
        assert_eq!(d.default_val.unwrap(), "a");
    }

    #[test]
    fn default_create_option_with_vals1() {
        let d = Arg::from("@a -o <file> <mode> 'some help info'");
        assert_eq!(d.name, "o");
        assert!(d.long.is_none());
        assert_eq!(d.short.unwrap(), 'o');
        assert_eq!(d.help.unwrap(), "some help info");
        assert!(!d.is_set(ArgSettings::MultipleOccurrences));
        assert!(d.is_set(ArgSettings::TakesValue));
        assert!(d.is_set(ArgSettings::Required));
        assert_eq!(
            d.val_names.unwrap().values().collect::<Vec<_>>(),
            [&"file", &"mode"]
        );
        assert_eq!(d.num_vals.unwrap(), 2);
        assert_eq!(d.default_val.unwrap(), "a");
    }

    #[test]
    fn default_create_option_with_vals2() {
        let d = Arg::from("@a -o <file> <mode>... 'some help info'");
        assert_eq!(d.name, "o");
        assert!(d.long.is_none());
        assert_eq!(d.short.unwrap(), 'o');
        assert_eq!(d.help.unwrap(), "some help info");
        assert!(d.is_set(ArgSettings::MultipleOccurrences));
        assert!(d.is_set(ArgSettings::TakesValue));
        assert!(d.is_set(ArgSettings::Required));
        assert_eq!(
            d.val_names.unwrap().values().collect::<Vec<_>>(),
            [&"file", &"mode"]
        );
        assert_eq!(d.num_vals.unwrap(), 2);
        assert_eq!(d.default_val.unwrap(), "a");
    }

    #[test]
    fn default_create_option_with_vals3() {
        let d = Arg::from("@a --opt <file> <mode>... 'some help info'");
        assert_eq!(d.name, "opt");
        assert!(d.short.is_none());
        assert_eq!(d.long.unwrap(), "opt");
        assert_eq!(d.help.unwrap(), "some help info");
        assert!(d.is_set(ArgSettings::MultipleOccurrences));
        assert!(d.is_set(ArgSettings::TakesValue));
        assert!(d.is_set(ArgSettings::Required));
        assert_eq!(
            d.val_names.unwrap().values().collect::<Vec<_>>(),
            [&"file", &"mode"]
        );
        assert_eq!(d.num_vals.unwrap(), 2);
        assert_eq!(d.default_val.unwrap(), "a");
    }

    #[test]
    fn default_create_option_with_vals4() {
        let d = Arg::from("[myopt] --opt @a <file> <mode> 'some help info'");
        assert_eq!(d.name, "myopt");
        assert!(d.short.is_none());
        assert_eq!(d.long.unwrap(), "opt");
        assert_eq!(d.help.unwrap(), "some help info");
        assert!(!d.is_set(ArgSettings::MultipleOccurrences));
        assert!(d.is_set(ArgSettings::TakesValue));
        assert!(!d.is_set(ArgSettings::Required));
        assert_eq!(
            d.val_names.unwrap().values().collect::<Vec<_>>(),
            [&"file", &"mode"]
        );
        assert_eq!(d.num_vals.unwrap(), 2);
        assert_eq!(d.default_val.unwrap(), "a");
    }

    #[test]
    fn default_create_option_with_vals5() {
        let d = Arg::from("@a --opt @a <file> <mode> 'some help info'");
        assert_eq!(d.name, "opt");
        assert!(d.short.is_none());
        assert_eq!(d.long.unwrap(), "opt");
        assert_eq!(d.help.unwrap(), "some help info");
        assert!(!d.is_set(ArgSettings::MultipleOccurrences));
        assert!(d.is_set(ArgSettings::TakesValue));
        assert!(d.is_set(ArgSettings::Required));
        assert_eq!(d.num_vals.unwrap(), 2);
        assert_eq!(d.default_val.unwrap(), "a");
    }

    #[test]
    fn default_create_positional_usage() {
        let a = Arg::from("[pos] @a 'some help info'");
        assert_eq!(a.name, "pos");
        assert_eq!(a.help.unwrap(), "some help info");
        assert!(!a.is_set(ArgSettings::MultipleOccurrences));
        assert!(!a.is_set(ArgSettings::Required));
        assert!(a.val_names.is_none());
        assert!(a.num_vals.is_none());
        assert_eq!(a.default_val.unwrap(), "a");
    }

    #[test]
    fn default_create_positional_usage0() {
        let b = Arg::from("<pos> @a 'some help info'");
        assert_eq!(b.name, "pos");
        assert_eq!(b.help.unwrap(), "some help info");
        assert!(!b.is_set(ArgSettings::MultipleOccurrences));
        assert!(b.is_set(ArgSettings::Required));
        assert!(b.val_names.is_none());
        assert!(b.num_vals.is_none());
        assert_eq!(b.default_val.unwrap(), "a");
    }

    #[test]
    fn default_pos_mult_help() {
        let c = Arg::from("[pos]... @a 'some help info'");
        assert_eq!(c.name, "pos");
        assert_eq!(c.help.unwrap(), "some help info");
        assert!(c.is_set(ArgSettings::MultipleOccurrences));
        assert!(!c.is_set(ArgSettings::Required));
        assert!(c.val_names.is_none());
        assert!(c.num_vals.is_none());
        assert_eq!(c.default_val.unwrap(), "a");
    }

    #[test]
    fn default_pos_help_lit_single_quote() {
        let c = Arg::from("[pos]... @a 'some help\' info'");
        assert_eq!(c.name, "pos");
        assert_eq!(c.help.unwrap(), "some help' info");
        assert!(c.is_set(ArgSettings::MultipleOccurrences));
        assert!(!c.is_set(ArgSettings::Required));
        assert!(c.val_names.is_none());
        assert!(c.num_vals.is_none());
        assert_eq!(c.default_val.unwrap(), "a");
    }

    #[test]
    fn default_pos_help_double_lit_single_quote() {
        let c = Arg::from("[pos]... @a 'some \'help\' info'");
        assert_eq!(c.name, "pos");
        assert_eq!(c.help.unwrap(), "some 'help' info");
        assert!(c.is_set(ArgSettings::MultipleOccurrences));
        assert!(!c.is_set(ArgSettings::Required));
        assert!(c.val_names.is_none());
        assert!(c.num_vals.is_none());
        assert_eq!(c.default_val.unwrap(), "a");
    }

    #[test]
    fn default_pos_help_newline() {
        let c = Arg::from(
            "[pos]... @a 'some help{n}\
             info'",
        );
        assert_eq!(c.name, "pos");
        assert_eq!(c.help.unwrap(), "some help{n}info");
        assert!(c.is_set(ArgSettings::MultipleOccurrences));
        assert!(!c.is_set(ArgSettings::Required));
        assert!(c.val_names.is_none());
        assert!(c.num_vals.is_none());
        assert_eq!(c.default_val.unwrap(), "a");
    }

    #[test]
    fn default_pos_help_newline_lit_sq() {
        let c = Arg::from(
            "[pos]... @a 'some help\' stuff{n}\
             info'",
        );
        assert_eq!(c.name, "pos");
        assert_eq!(c.help.unwrap(), "some help' stuff{n}info");
        assert!(c.is_set(ArgSettings::MultipleOccurrences));
        assert!(!c.is_set(ArgSettings::Required));
        assert!(c.val_names.is_none());
        assert!(c.num_vals.is_none());
        assert_eq!(c.default_val.unwrap(), "a");
    }

    #[test]
    fn default_pos_req_mult_help() {
        let d = Arg::from("<pos>... @a 'some help info'");
        assert_eq!(d.name, "pos");
        assert_eq!(d.help.unwrap(), "some help info");
        assert!(d.is_set(ArgSettings::MultipleOccurrences));
        assert!(d.is_set(ArgSettings::Required));
        assert!(d.val_names.is_none());
        assert!(d.num_vals.is_none());
        assert_eq!(d.default_val.unwrap(), "a");
    }

    #[test]
    fn default_pos_req() {
        let b = Arg::from("<pos> @a");
        assert_eq!(b.name, "pos");
        assert!(!b.is_set(ArgSettings::MultipleOccurrences));
        assert!(b.is_set(ArgSettings::Required));
        assert!(b.val_names.is_none());
        assert!(b.num_vals.is_none());
        assert_eq!(b.default_val.unwrap(), "a");
    }

    #[test]
    fn default_pos_mult() {
        let c = Arg::from("[pos]... @a");
        assert_eq!(c.name, "pos");
        assert!(c.is_set(ArgSettings::MultipleOccurrences));
        assert!(!c.is_set(ArgSettings::Required));
        assert!(c.val_names.is_none());
        assert!(c.num_vals.is_none());
        assert_eq!(c.default_val.unwrap(), "a");
    }

    #[test]
    fn v3_default_create_flag_usage() {
        let a = Arg::from("[flag] -f 'some help info [default: a]'");
        assert_eq!(a.name, "flag");
        assert_eq!(a.short.unwrap(), 'f');
        assert!(a.long.is_none());
        assert_eq!(a.help.unwrap(), "some help info");
        assert!(!a.is_set(ArgSettings::MultipleOccurrences));
        assert!(a.val_names.is_none());
        assert!(a.num_vals.is_none());
        assert_eq!(a.default_val.unwrap(), "a");
        assert_eq!(a.default_val.unwrap(), "a");

        let b = Arg::from("[flag] --flag 'some help info [default: a]'");
        assert_eq!(b.name, "flag");
        assert_eq!(b.long.unwrap(), "flag");
        assert!(b.short.is_none());
        assert_eq!(b.help.unwrap(), "some help info");
        assert!(!b.is_set(ArgSettings::MultipleOccurrences));
        assert!(a.val_names.is_none());
        assert!(a.num_vals.is_none());
        assert_eq!(a.default_val.unwrap(), "a");
        assert_eq!(a.default_val.unwrap(), "a");

        let b = Arg::from("--flag 'some help info [default: a]'");
        assert_eq!(b.name, "flag");
        assert_eq!(b.long.unwrap(), "flag");
        assert!(b.short.is_none());
        assert_eq!(b.help.unwrap(), "some help info");
        assert!(!b.is_set(ArgSettings::MultipleOccurrences));
        assert!(b.val_names.is_none());
        assert!(b.num_vals.is_none());
        assert_eq!(b.default_val.unwrap(), "a");

        let c = Arg::from("[flag] -f --flag 'some help info [default: a]'");
        assert_eq!(c.name, "flag");
        assert_eq!(c.short.unwrap(), 'f');
        assert_eq!(c.long.unwrap(), "flag");
        assert_eq!(c.help.unwrap(), "some help info");
        assert!(!c.is_set(ArgSettings::MultipleOccurrences));
        assert!(c.val_names.is_none());
        assert!(c.num_vals.is_none());
        assert_eq!(c.default_val.unwrap(), "a");

        let d = Arg::from("[flag] -f... 'some help info [default: a]'");
        assert_eq!(d.name, "flag");
        assert_eq!(d.short.unwrap(), 'f');
        assert!(d.long.is_none());
        assert_eq!(d.help.unwrap(), "some help info");
        assert!(d.is_set(ArgSettings::MultipleOccurrences));
        assert!(d.val_names.is_none());
        assert!(d.num_vals.is_none());
        assert_eq!(d.default_val.unwrap(), "a");

        let e = Arg::from("[flag] -f --flag... 'some help info [default: a]'");
        assert_eq!(e.name, "flag");
        assert_eq!(e.long.unwrap(), "flag");
        assert_eq!(e.short.unwrap(), 'f');
        assert_eq!(e.help.unwrap(), "some help info");
        assert!(e.is_set(ArgSettings::MultipleOccurrences));
        assert!(e.val_names.is_none());
        assert!(e.num_vals.is_none());
        assert_eq!(e.default_val.unwrap(), "a");

        let e = Arg::from("-f --flag... 'some help info [default: a]'");
        assert_eq!(e.name, "flag");
        assert_eq!(e.long.unwrap(), "flag");
        assert_eq!(e.short.unwrap(), 'f');
        assert_eq!(e.help.unwrap(), "some help info");
        assert!(e.is_set(ArgSettings::MultipleOccurrences));
        assert!(e.val_names.is_none());
        assert!(e.num_vals.is_none());
        assert_eq!(e.default_val.unwrap(), "a");

        let a = Arg::from("-f 'some help info [default: a]'");
        assert_eq!(a.name, "f");
        assert_eq!(a.short.unwrap(), 'f');
        assert!(a.long.is_none());
        assert_eq!(a.help.unwrap(), "some help info");
        assert!(!a.is_set(ArgSettings::MultipleOccurrences));
        assert!(a.val_names.is_none());
        assert!(a.num_vals.is_none());
        assert_eq!(a.default_val.unwrap(), "a");
    }

    #[test]
    fn v3_default_create_option_usage0() {
        // Short only
        let a = Arg::from("[option] -o [opt] 'some help info [default: a]'");
        assert_eq!(a.name, "option");
        assert_eq!(a.short.unwrap(), 'o');
        assert!(a.long.is_none());
        assert_eq!(a.help.unwrap(), "some help info");
        assert!(!a.is_set(ArgSettings::MultipleOccurrences));
        assert!(a.is_set(ArgSettings::TakesValue));
        assert!(!a.is_set(ArgSettings::Required));
        assert_eq!(
            a.val_names.unwrap().values().collect::<Vec<_>>(),
            [&"opt"]
        );
        assert!(a.num_vals.is_none());
        assert_eq!(a.default_val.unwrap(), "a");
    }

    #[test]
    fn v3_default_create_option_usage1() {
        let b = Arg::from("-o [opt] 'some help info [default: a]'");
        assert_eq!(b.name, "o");
        assert_eq!(b.short.unwrap(), 'o');
        assert!(b.long.is_none());
        assert_eq!(b.help.unwrap(), "some help info");
        assert!(!b.is_set(ArgSettings::MultipleOccurrences));
        assert!(b.is_set(ArgSettings::TakesValue));
        assert!(!b.is_set(ArgSettings::Required));
        assert_eq!(
            b.val_names.unwrap().values().collect::<Vec<_>>(),
            [&"opt"]
        );
        assert!(b.num_vals.is_none());
        assert_eq!(b.default_val.unwrap(), "a");
    }

    #[test]
    fn v3_default_create_option_usage2() {
        let c = Arg::from("<option> -o <opt> 'some help info [default: a]'");
        assert_eq!(c.name, "option");
        assert_eq!(c.short.unwrap(), 'o');
        assert!(c.long.is_none());
        assert_eq!(c.help.unwrap(), "some help info");
        assert!(!c.is_set(ArgSettings::MultipleOccurrences));
        assert!(c.is_set(ArgSettings::TakesValue));
        assert!(c.is_set(ArgSettings::Required));
        assert_eq!(
            c.val_names.unwrap().values().collect::<Vec<_>>(),
            [&"opt"]
        );
        assert!(c.num_vals.is_none());
        assert_eq!(c.default_val.unwrap(), "a");
    }

    #[test]
    fn v3_default_create_option_usage3() {
        let d = Arg::from("-o <opt> 'some help info [default: a]'");
        assert_eq!(d.name, "o");
        assert_eq!(d.short.unwrap(), 'o');
        assert!(d.long.is_none());
        assert_eq!(d.help.unwrap(), "some help info");
        assert!(!d.is_set(ArgSettings::MultipleOccurrences));
        assert!(d.is_set(ArgSettings::TakesValue));
        assert!(d.is_set(ArgSettings::Required));
        assert_eq!(
            d.val_names.unwrap().values().collect::<Vec<_>>(),
            [&"opt"]
        );
        assert!(d.num_vals.is_none());
        assert_eq!(d.default_val.unwrap(), "a");
    }

    #[test]
    fn v3_default_create_option_usage4() {
        let a = Arg::from("[option] -o [opt]... 'some help info [default: a]'");
        assert_eq!(a.name, "option");
        assert_eq!(a.short.unwrap(), 'o');
        assert!(a.long.is_none());
        assert_eq!(a.help.unwrap(), "some help info");
        assert!(a.is_set(ArgSettings::MultipleOccurrences));
        assert!(a.is_set(ArgSettings::TakesValue));
        assert!(!a.is_set(ArgSettings::Required));
        assert_eq!(
            a.val_names.unwrap().values().collect::<Vec<_>>(),
            [&"opt"]
        );
        assert!(a.num_vals.is_none());
        assert_eq!(a.default_val.unwrap(), "a");
    }

    #[test]
    fn v3_default_create_option_usage5() {
        let a = Arg::from("[option]... -o [opt] 'some help info [default: a]'");
        assert_eq!(a.name, "option");
        assert_eq!(a.short.unwrap(), 'o');
        assert!(a.long.is_none());
        assert_eq!(a.help.unwrap(), "some help info");
        assert!(a.is_set(ArgSettings::MultipleOccurrences));
        assert!(a.is_set(ArgSettings::TakesValue));
        assert!(!a.is_set(ArgSettings::Required));
        assert_eq!(
            a.val_names.unwrap().values().collect::<Vec<_>>(),
            [&"opt"]
        );
        assert!(a.num_vals.is_none());
        assert_eq!(a.default_val.unwrap(), "a");
    }

    #[test]
    fn v3_default_create_option_usage6() {
        let b = Arg::from("-o [opt]... 'some help info [default: a]'");
        assert_eq!(b.name, "o");
        assert_eq!(b.short.unwrap(), 'o');
        assert!(b.long.is_none());
        assert_eq!(b.help.unwrap(), "some help info");
        assert!(b.is_set(ArgSettings::MultipleOccurrences));
        assert!(b.is_set(ArgSettings::TakesValue));
        assert!(!b.is_set(ArgSettings::Required));
        assert_eq!(
            b.val_names.unwrap().values().collect::<Vec<_>>(),
            [&"opt"]
        );
        assert!(b.num_vals.is_none());
        assert_eq!(b.default_val.unwrap(), "a");
    }

    #[test]
    fn v3_default_create_option_usage7() {
        let c = Arg::from("<option> -o <opt>... 'some help info [default: a]'");
        assert_eq!(c.name, "option");
        assert_eq!(c.short.unwrap(), 'o');
        assert!(c.long.is_none());
        assert_eq!(c.help.unwrap(), "some help info");
        assert!(c.is_set(ArgSettings::MultipleOccurrences));
        assert!(c.is_set(ArgSettings::TakesValue));
        assert!(c.is_set(ArgSettings::Required));
        assert_eq!(
            c.val_names.unwrap().values().collect::<Vec<_>>(),
            [&"opt"]
        );
        assert!(c.num_vals.is_none());
        assert_eq!(c.default_val.unwrap(), "a");
    }

    #[test]
    fn v3_default_create_option_usage8() {
        let c = Arg::from("<option>... -o <opt> 'some help info [default: a]'");
        assert_eq!(c.name, "option");
        assert_eq!(c.short.unwrap(), 'o');
        assert!(c.long.is_none());
        assert_eq!(c.help.unwrap(), "some help info");
        assert!(c.is_set(ArgSettings::MultipleOccurrences));
        assert!(c.is_set(ArgSettings::TakesValue));
        assert!(c.is_set(ArgSettings::Required));
        assert_eq!(
            c.val_names.unwrap().values().collect::<Vec<_>>(),
            [&"opt"]
        );
        assert!(c.num_vals.is_none());
        assert_eq!(c.default_val.unwrap(), "a");
    }

    #[test]
    fn v3_default_create_option_usage9() {
        let d = Arg::from("-o <opt>... 'some help info [default: a]'");
        assert_eq!(d.name, "o");
        assert_eq!(d.short.unwrap(), 'o');
        assert!(d.long.is_none());
        assert_eq!(d.help.unwrap(), "some help info");
        assert!(d.is_set(ArgSettings::MultipleOccurrences));
        assert!(d.is_set(ArgSettings::TakesValue));
        assert!(d.is_set(ArgSettings::Required));
        assert_eq!(
            d.val_names.unwrap().values().collect::<Vec<_>>(),
            [&"opt"]
        );
        assert!(d.num_vals.is_none());
        assert_eq!(d.default_val.unwrap(), "a");
    }

    #[test]
    fn v3_default_create_option_usage_long1() {
        let a = Arg::from("[option] --opt [opt] 'some help info [default: a]'");
        assert_eq!(a.name, "option");
        assert_eq!(a.long.unwrap(), "opt");
        assert!(a.short.is_none());
        assert_eq!(a.help.unwrap(), "some help info");
        assert!(!a.is_set(ArgSettings::MultipleOccurrences));
        assert!(a.is_set(ArgSettings::TakesValue));
        assert!(!a.is_set(ArgSettings::Required));
        assert_eq!(
            a.val_names.unwrap().values().collect::<Vec<_>>(),
            [&"opt"]
        );
        assert!(a.num_vals.is_none());
        assert_eq!(a.default_val.unwrap(), "a");
    }

    #[test]
    fn v3_default_create_option_usage_long2() {
        let b = Arg::from("--opt [option] 'some help info [default: a]'");
        assert_eq!(b.name, "opt");
        assert_eq!(b.long.unwrap(), "opt");
        assert!(b.short.is_none());
        assert_eq!(b.help.unwrap(), "some help info");
        assert!(!b.is_set(ArgSettings::MultipleOccurrences));
        assert!(b.is_set(ArgSettings::TakesValue));
        assert!(!b.is_set(ArgSettings::Required));
        assert_eq!(
            b.val_names.unwrap().values().collect::<Vec<_>>(),
            [&"option"]
        );
        assert!(b.num_vals.is_none());
        assert_eq!(b.default_val.unwrap(), "a");
    }

    #[test]
    fn v3_default_create_option_usage_long3() {
        let c = Arg::from("<option> --opt <opt> 'some help info [default: a]'");
        assert_eq!(c.name, "option");
        assert_eq!(c.long.unwrap(), "opt");
        assert!(c.short.is_none());
        assert_eq!(c.help.unwrap(), "some help info");
        assert!(!c.is_set(ArgSettings::MultipleOccurrences));
        assert!(c.is_set(ArgSettings::TakesValue));
        assert!(c.is_set(ArgSettings::Required));
        assert_eq!(
            c.val_names.unwrap().values().collect::<Vec<_>>(),
            [&"opt"]
        );
        assert!(c.num_vals.is_none());
        assert_eq!(c.default_val.unwrap(), "a");
    }

    #[test]
    fn v3_default_create_option_usage_long4() {
        let d = Arg::from("--opt <option> 'some help info [default: a]'");
        assert_eq!(d.name, "opt");
        assert_eq!(d.long.unwrap(), "opt");
        assert!(d.short.is_none());
        assert_eq!(d.help.unwrap(), "some help info");
        assert!(!d.is_set(ArgSettings::MultipleOccurrences));
        assert!(d.is_set(ArgSettings::TakesValue));
        assert!(d.is_set(ArgSettings::Required));
        assert_eq!(
            d.val_names.unwrap().values().collect::<Vec<_>>(),
            [&"option"]
        );
        assert!(d.num_vals.is_none());
        assert_eq!(d.default_val.unwrap(), "a");
    }

    #[test]
    fn v3_default_create_option_usage_long5() {
        let a = Arg::from("[option] --opt [opt]... 'some help info [default: a]'");
        assert_eq!(a.name, "option");
        assert_eq!(a.long.unwrap(), "opt");
        assert!(a.short.is_none());
        assert_eq!(a.help.unwrap(), "some help info");
        assert!(a.is_set(ArgSettings::MultipleOccurrences));
        assert!(a.is_set(ArgSettings::TakesValue));
        assert!(!a.is_set(ArgSettings::Required));
        assert_eq!(
            a.val_names.unwrap().values().collect::<Vec<_>>(),
            [&"opt"]
        );
        assert!(a.num_vals.is_none());
        assert_eq!(a.default_val.unwrap(), "a");
    }

    #[test]
    fn v3_default_create_option_usage_long6() {
        let a = Arg::from("[option]... --opt [opt] 'some help info [default: a]'");
        assert_eq!(a.name, "option");
        assert_eq!(a.long.unwrap(), "opt");
        assert!(a.short.is_none());
        assert_eq!(a.help.unwrap(), "some help info");
        assert!(a.is_set(ArgSettings::MultipleOccurrences));
        assert!(a.is_set(ArgSettings::TakesValue));
        assert!(!a.is_set(ArgSettings::Required));
        assert_eq!(
            a.val_names.unwrap().values().collect::<Vec<_>>(),
            [&"opt"]
        );
        assert!(a.num_vals.is_none());
        assert_eq!(a.default_val.unwrap(), "a");
    }

    #[test]
    fn v3_default_create_option_usage_long7() {
        let b = Arg::from("--opt [option]... 'some help info [default: a]'");
        assert_eq!(b.name, "opt");
        assert_eq!(b.long.unwrap(), "opt");
        assert!(b.short.is_none());
        assert_eq!(b.help.unwrap(), "some help info");
        assert!(b.is_set(ArgSettings::MultipleOccurrences));
        assert!(b.is_set(ArgSettings::TakesValue));
        assert!(!b.is_set(ArgSettings::Required));
        assert_eq!(
            b.val_names.unwrap().values().collect::<Vec<_>>(),
            [&"option"]
        );
        assert!(b.num_vals.is_none());
        assert_eq!(b.default_val.unwrap(), "a");
    }

    #[test]
    fn v3_default_create_option_usage_long8() {
        let c = Arg::from("<option> --opt <opt>... 'some help info [default: a]'");
        assert_eq!(c.name, "option");
        assert_eq!(c.long.unwrap(), "opt");
        assert!(c.short.is_none());
        assert_eq!(c.help.unwrap(), "some help info");
        assert!(c.is_set(ArgSettings::MultipleOccurrences));
        assert!(c.is_set(ArgSettings::TakesValue));
        assert!(c.is_set(ArgSettings::Required));
        assert_eq!(
            c.val_names.unwrap().values().collect::<Vec<_>>(),
            [&"opt"]
        );
        assert!(c.num_vals.is_none());
        assert_eq!(c.default_val.unwrap(), "a");
    }

    #[test]
    fn v3_default_create_option_usage_long9() {
        let c = Arg::from("<option>... --opt <opt> 'some help info [default: a]'");
        assert_eq!(c.name, "option");
        assert_eq!(c.long.unwrap(), "opt");
        assert!(c.short.is_none());
        assert_eq!(c.help.unwrap(), "some help info");
        assert!(c.is_set(ArgSettings::MultipleOccurrences));
        assert!(c.is_set(ArgSettings::TakesValue));
        assert!(c.is_set(ArgSettings::Required));
        assert_eq!(
            c.val_names.unwrap().values().collect::<Vec<_>>(),
            [&"opt"]
        );
        assert!(c.num_vals.is_none());
        assert_eq!(c.default_val.unwrap(), "a");
    }

    #[test]
    fn v3_default_create_option_usage_long10() {
        let d = Arg::from("--opt <option>... 'some help info [default: a]'");
        assert_eq!(d.name, "opt");
        assert_eq!(d.long.unwrap(), "opt");
        assert!(d.short.is_none());
        assert_eq!(d.help.unwrap(), "some help info");
        assert!(d.is_set(ArgSettings::MultipleOccurrences));
        assert!(d.is_set(ArgSettings::TakesValue));
        assert!(d.is_set(ArgSettings::Required));
        assert_eq!(
            d.val_names.unwrap().values().collect::<Vec<_>>(),
            [&"option"]
        );
        assert!(d.num_vals.is_none());
        assert_eq!(d.default_val.unwrap(), "a");
    }

    #[test]
    fn v3_default_create_option_usage_long_equals1() {
        let a = Arg::from("[option] --opt=[opt] 'some help info [default: a]'");
        assert_eq!(a.name, "option");
        assert_eq!(a.long.unwrap(), "opt");
        assert!(a.short.is_none());
        assert_eq!(a.help.unwrap(), "some help info");
        assert!(!a.is_set(ArgSettings::MultipleOccurrences));
        assert!(a.is_set(ArgSettings::TakesValue));
        assert!(!a.is_set(ArgSettings::Required));
        assert_eq!(
            a.val_names.unwrap().values().collect::<Vec<_>>(),
            [&"opt"]
        );
        assert!(a.num_vals.is_none());
        assert_eq!(a.default_val.unwrap(), "a");
    }

    #[test]
    fn v3_default_create_option_usage_long_equals2() {
        let b = Arg::from("--opt=[option] 'some help info [default: a]'");
        assert_eq!(b.name, "opt");
        assert_eq!(b.long.unwrap(), "opt");
        assert!(b.short.is_none());
        assert_eq!(b.help.unwrap(), "some help info");
        assert!(!b.is_set(ArgSettings::MultipleOccurrences));
        assert!(b.is_set(ArgSettings::TakesValue));
        assert!(!b.is_set(ArgSettings::Required));
        assert_eq!(
            b.val_names.unwrap().values().collect::<Vec<_>>(),
            [&"option"]
        );
        assert!(b.num_vals.is_none());
        assert_eq!(b.default_val.unwrap(), "a");
    }

    #[test]
    fn v3_default_create_option_usage_long_equals3() {
        let c = Arg::from("<option> --opt=<opt> 'some help info [default: a]'");
        assert_eq!(c.name, "option");
        assert_eq!(c.long.unwrap(), "opt");
        assert!(c.short.is_none());
        assert_eq!(c.help.unwrap(), "some help info");
        assert!(!c.is_set(ArgSettings::MultipleOccurrences));
        assert!(c.is_set(ArgSettings::TakesValue));
        assert!(c.is_set(ArgSettings::Required));
        assert_eq!(
            c.val_names.unwrap().values().collect::<Vec<_>>(),
            [&"opt"]
        );
        assert!(c.num_vals.is_none());
        assert_eq!(c.default_val.unwrap(), "a");
    }

    #[test]
    fn v3_default_create_option_usage_long_equals4() {
        let d = Arg::from("--opt=<option> 'some help info [default: a]'");
        assert_eq!(d.name, "opt");
        assert_eq!(d.long.unwrap(), "opt");
        assert!(d.short.is_none());
        assert_eq!(d.help.unwrap(), "some help info");
        assert!(!d.is_set(ArgSettings::MultipleOccurrences));
        assert!(d.is_set(ArgSettings::TakesValue));
        assert!(d.is_set(ArgSettings::Required));
        assert_eq!(
            d.val_names.unwrap().values().collect::<Vec<_>>(),
            [&"option"]
        );
        assert!(d.num_vals.is_none());
        assert_eq!(d.default_val.unwrap(), "a");
    }

    #[test]
    fn v3_default_create_option_usage_long_equals5() {
        let a = Arg::from("[option] --opt=[opt]... 'some help info [default: a]'");
        assert_eq!(a.name, "option");
        assert_eq!(a.long.unwrap(), "opt");
        assert!(a.short.is_none());
        assert_eq!(a.help.unwrap(), "some help info");
        assert!(a.is_set(ArgSettings::MultipleOccurrences));
        assert!(a.is_set(ArgSettings::TakesValue));
        assert!(!a.is_set(ArgSettings::Required));
        assert_eq!(
            a.val_names.unwrap().values().collect::<Vec<_>>(),
            [&"opt"]
        );
        assert!(a.num_vals.is_none());
        assert_eq!(a.default_val.unwrap(), "a");
    }

    #[test]
    fn v3_default_create_option_usage_long_equals6() {
        let a = Arg::from("[option]... --opt=[opt] 'some help info [default: a]'");
        assert_eq!(a.name, "option");
        assert_eq!(a.long.unwrap(), "opt");
        assert!(a.short.is_none());
        assert_eq!(a.help.unwrap(), "some help info");
        assert!(a.is_set(ArgSettings::MultipleOccurrences));
        assert!(a.is_set(ArgSettings::TakesValue));
        assert!(!a.is_set(ArgSettings::Required));
        assert_eq!(
            a.val_names.unwrap().values().collect::<Vec<_>>(),
            [&"opt"]
        );
        assert!(a.num_vals.is_none());
        assert_eq!(a.default_val.unwrap(), "a");
    }

    #[test]
    fn v3_default_create_option_usage_long_equals7() {
        let b = Arg::from("--opt=[option]... 'some help info [default: a]'");
        assert_eq!(b.name, "opt");
        assert_eq!(b.long.unwrap(), "opt");
        assert!(b.short.is_none());
        assert_eq!(b.help.unwrap(), "some help info");
        assert!(b.is_set(ArgSettings::MultipleOccurrences));
        assert!(b.is_set(ArgSettings::TakesValue));
        assert!(!b.is_set(ArgSettings::Required));
        assert_eq!(
            b.val_names.unwrap().values().collect::<Vec<_>>(),
            [&"option"]
        );
        assert!(b.num_vals.is_none());
        assert_eq!(b.default_val.unwrap(), "a");
    }

    #[test]
    fn v3_default_create_option_usage_long_equals8() {
        let c = Arg::from("<option> --opt=<opt>... 'some help info [default: a]'");
        assert_eq!(c.name, "option");
        assert_eq!(c.long.unwrap(), "opt");
        assert!(c.short.is_none());
        assert_eq!(c.help.unwrap(), "some help info");
        assert!(c.is_set(ArgSettings::MultipleOccurrences));
        assert!(c.is_set(ArgSettings::TakesValue));
        assert!(c.is_set(ArgSettings::Required));
        assert_eq!(
            c.val_names.unwrap().values().collect::<Vec<_>>(),
            [&"opt"]
        );
        assert!(c.num_vals.is_none());
        assert_eq!(c.default_val.unwrap(), "a");
    }

    #[test]
    fn v3_default_create_option_usage_long_equals9() {
        let c = Arg::from("<option>... --opt=<opt> 'some help info [default: a]'");
        assert_eq!(c.name, "option");
        assert_eq!(c.long.unwrap(), "opt");
        assert!(c.short.is_none());
        assert_eq!(c.help.unwrap(), "some help info");
        assert!(c.is_set(ArgSettings::MultipleOccurrences));
        assert!(c.is_set(ArgSettings::TakesValue));
        assert!(c.is_set(ArgSettings::Required));
        assert_eq!(
            c.val_names.unwrap().values().collect::<Vec<_>>(),
            [&"opt"]
        );
        assert!(c.num_vals.is_none());
        assert_eq!(c.default_val.unwrap(), "a");
    }

    #[test]
    fn v3_default_create_option_usage_long_equals10() {
        let d = Arg::from("--opt=<option>... 'some help info [default: a]'");
        assert_eq!(d.name, "opt");
        assert_eq!(d.long.unwrap(), "opt");
        assert!(d.short.is_none());
        assert_eq!(d.help.unwrap(), "some help info");
        assert!(d.is_set(ArgSettings::MultipleOccurrences));
        assert!(d.is_set(ArgSettings::TakesValue));
        assert!(d.is_set(ArgSettings::Required));
        assert_eq!(
            d.val_names.unwrap().values().collect::<Vec<_>>(),
            [&"option"]
        );
        assert!(d.num_vals.is_none());
        assert_eq!(d.default_val.unwrap(), "a");
    }

    #[test]
    fn v3_default_create_option_usage_both1() {
        let a = Arg::from("[option] -o --opt [option] 'some help info [default: a]'");
        assert_eq!(a.name, "option");
        assert_eq!(a.long.unwrap(), "opt");
        assert_eq!(a.short.unwrap(), 'o');
        assert_eq!(a.help.unwrap(), "some help info");
        assert!(!a.is_set(ArgSettings::MultipleOccurrences));
        assert!(a.is_set(ArgSettings::TakesValue));
        assert!(!a.is_set(ArgSettings::Required));
        assert_eq!(
            a.val_names.unwrap().values().collect::<Vec<_>>(),
            [&"option"]
        );
        assert!(a.num_vals.is_none());
        assert_eq!(a.default_val.unwrap(), "a");
    }

    #[test]
    fn v3_default_create_option_usage_both2() {
        let b = Arg::from("-o --opt [option] 'some help info [default: a]'");
        assert_eq!(b.name, "opt");
        assert_eq!(b.long.unwrap(), "opt");
        assert_eq!(b.short.unwrap(), 'o');
        assert_eq!(b.help.unwrap(), "some help info");
        assert!(!b.is_set(ArgSettings::MultipleOccurrences));
        assert!(b.is_set(ArgSettings::TakesValue));
        assert!(!b.is_set(ArgSettings::Required));
        assert_eq!(
            b.val_names.unwrap().values().collect::<Vec<_>>(),
            [&"option"]
        );
        assert!(b.num_vals.is_none());
        assert_eq!(b.default_val.unwrap(), "a");
    }

    #[test]
    fn v3_default_create_option_usage_both3() {
        let c = Arg::from("<option> -o --opt <opt> 'some help info [default: a]'");
        assert_eq!(c.name, "option");
        assert_eq!(c.long.unwrap(), "opt");
        assert_eq!(c.short.unwrap(), 'o');
        assert_eq!(c.help.unwrap(), "some help info");
        assert!(!c.is_set(ArgSettings::MultipleOccurrences));
        assert!(c.is_set(ArgSettings::TakesValue));
        assert!(c.is_set(ArgSettings::Required));
        assert_eq!(
            c.val_names.unwrap().values().collect::<Vec<_>>(),
            [&"opt"]
        );
        assert!(c.num_vals.is_none());
        assert_eq!(c.default_val.unwrap(), "a");
    }

    #[test]
    fn v3_default_create_option_usage_both4() {
        let d = Arg::from("-o --opt <option> 'some help info [default: a]'");
        assert_eq!(d.name, "opt");
        assert_eq!(d.long.unwrap(), "opt");
        assert_eq!(d.short.unwrap(), 'o');
        assert_eq!(d.help.unwrap(), "some help info");
        assert!(!d.is_set(ArgSettings::MultipleOccurrences));
        assert!(d.is_set(ArgSettings::TakesValue));
        assert!(d.is_set(ArgSettings::Required));
        assert_eq!(
            d.val_names.unwrap().values().collect::<Vec<_>>(),
            [&"option"]
        );
        assert!(d.num_vals.is_none());
        assert_eq!(d.default_val.unwrap(), "a");
    }

    #[test]
    fn v3_default_create_option_usage_both5() {
        let a = Arg::from("[option]... -o --opt [option] 'some help info [default: a]'");
        assert_eq!(a.name, "option");
        assert_eq!(a.long.unwrap(), "opt");
        assert_eq!(a.short.unwrap(), 'o');
        assert_eq!(a.help.unwrap(), "some help info");
        assert!(a.is_set(ArgSettings::MultipleOccurrences));
        assert!(a.is_set(ArgSettings::TakesValue));
        assert!(!a.is_set(ArgSettings::Required));
        assert_eq!(
            a.val_names.unwrap().values().collect::<Vec<_>>(),
            [&"option"]
        );
        assert!(a.num_vals.is_none());
        assert_eq!(a.default_val.unwrap(), "a");
    }

    #[test]
    fn v3_default_create_option_usage_both6() {
        let b = Arg::from("-o --opt [option]... 'some help info [default: a]'");
        assert_eq!(b.name, "opt");
        assert_eq!(b.long.unwrap(), "opt");
        assert_eq!(b.short.unwrap(), 'o');
        assert_eq!(b.help.unwrap(), "some help info");
        assert!(b.is_set(ArgSettings::MultipleOccurrences));
        assert!(b.is_set(ArgSettings::TakesValue));
        assert!(!b.is_set(ArgSettings::Required));
        assert_eq!(
            b.val_names.unwrap().values().collect::<Vec<_>>(),
            [&"option"]
        );
        assert!(b.num_vals.is_none());
        assert_eq!(b.default_val.unwrap(), "a");
    }

    #[test]
    fn v3_default_create_option_usage_both7() {
        let c = Arg::from("<option>... -o --opt <opt> 'some help info [default: a]'");
        assert_eq!(c.name, "option");
        assert_eq!(c.long.unwrap(), "opt");
        assert_eq!(c.short.unwrap(), 'o');
        assert_eq!(c.help.unwrap(), "some help info");
        assert!(c.is_set(ArgSettings::MultipleOccurrences));
        assert!(c.is_set(ArgSettings::TakesValue));
        assert!(c.is_set(ArgSettings::Required));
        assert_eq!(
            c.val_names.unwrap().values().collect::<Vec<_>>(),
            [&"opt"]
        );
        assert!(c.num_vals.is_none());
        assert_eq!(c.default_val.unwrap(), "a");
    }

    #[test]
    fn v3_default_create_option_usage_both8() {
        let d = Arg::from("-o --opt <option>... 'some help info [default: a]'");
        assert_eq!(d.name, "opt");
        assert_eq!(d.long.unwrap(), "opt");
        assert_eq!(d.short.unwrap(), 'o');
        assert_eq!(d.help.unwrap(), "some help info");
        assert!(d.is_set(ArgSettings::MultipleOccurrences));
        assert!(d.is_set(ArgSettings::TakesValue));
        assert!(d.is_set(ArgSettings::Required));
        assert_eq!(
            d.val_names.unwrap().values().collect::<Vec<_>>(),
            [&"option"]
        );
        assert!(d.num_vals.is_none());
        assert_eq!(d.default_val.unwrap(), "a");
    }

    #[test]
    fn v3_default_create_option_usage_both_equals1() {
        let a = Arg::from("[option] -o --opt=[option] 'some help info [default: a]'");
        assert_eq!(a.name, "option");
        assert_eq!(a.long.unwrap(), "opt");
        assert_eq!(a.short.unwrap(), 'o');
        assert_eq!(a.help.unwrap(), "some help info");
        assert!(!a.is_set(ArgSettings::MultipleOccurrences));
        assert!(a.is_set(ArgSettings::TakesValue));
        assert!(!a.is_set(ArgSettings::Required));
        assert_eq!(
            a.val_names.unwrap().values().collect::<Vec<_>>(),
            [&"option"]
        );
        assert!(a.num_vals.is_none());
        assert_eq!(a.default_val.unwrap(), "a");
    }

    #[test]
    fn v3_default_create_option_usage_both_equals2() {
        let b = Arg::from("-o --opt=[option] 'some help info [default: a]'");
        assert_eq!(b.name, "opt");
        assert_eq!(b.long.unwrap(), "opt");
        assert_eq!(b.short.unwrap(), 'o');
        assert_eq!(b.help.unwrap(), "some help info");
        assert!(!b.is_set(ArgSettings::MultipleOccurrences));
        assert!(b.is_set(ArgSettings::TakesValue));
        assert!(!b.is_set(ArgSettings::Required));
        assert_eq!(
            b.val_names.unwrap().values().collect::<Vec<_>>(),
            [&"option"]
        );
        assert!(b.num_vals.is_none());
        assert_eq!(b.default_val.unwrap(), "a");
    }

    #[test]
    fn v3_default_create_option_usage_both_equals3() {
        let c = Arg::from("<option> -o --opt=<opt> 'some help info [default: a]'");
        assert_eq!(c.name, "option");
        assert_eq!(c.long.unwrap(), "opt");
        assert_eq!(c.short.unwrap(), 'o');
        assert_eq!(c.help.unwrap(), "some help info");
        assert!(!c.is_set(ArgSettings::MultipleOccurrences));
        assert!(c.is_set(ArgSettings::TakesValue));
        assert!(c.is_set(ArgSettings::Required));
        assert_eq!(
            c.val_names.unwrap().values().collect::<Vec<_>>(),
            [&"opt"]
        );
        assert!(c.num_vals.is_none());
        assert_eq!(c.default_val.unwrap(), "a");
    }

    #[test]
    fn v3_default_create_option_usage_both_equals4() {
        let d = Arg::from("-o --opt=<option> 'some help info [default: a]'");
        assert_eq!(d.name, "opt");
        assert_eq!(d.long.unwrap(), "opt");
        assert_eq!(d.short.unwrap(), 'o');
        assert_eq!(d.help.unwrap(), "some help info");
        assert!(!d.is_set(ArgSettings::MultipleOccurrences));
        assert!(d.is_set(ArgSettings::TakesValue));
        assert!(d.is_set(ArgSettings::Required));
        assert_eq!(
            d.val_names.unwrap().values().collect::<Vec<_>>(),
            [&"option"]
        );
        assert!(d.num_vals.is_none());
        assert_eq!(d.default_val.unwrap(), "a");
    }

    #[test]
    fn v3_default_create_option_usage_both_equals5() {
        let a = Arg::from("[option]... -o --opt=[option] 'some help info [default: a]'");
        assert_eq!(a.name, "option");
        assert_eq!(a.long.unwrap(), "opt");
        assert_eq!(a.short.unwrap(), 'o');
        assert_eq!(a.help.unwrap(), "some help info");
        assert!(a.is_set(ArgSettings::MultipleOccurrences));
        assert!(a.is_set(ArgSettings::TakesValue));
        assert!(!a.is_set(ArgSettings::Required));
        assert_eq!(
            a.val_names.unwrap().values().collect::<Vec<_>>(),
            [&"option"]
        );
        assert!(a.num_vals.is_none());
        assert_eq!(a.default_val.unwrap(), "a");
    }

    #[test]
    fn v3_default_create_option_usage_both_equals6() {
        let b = Arg::from("-o --opt=[option]... 'some help info [default: a]'");
        assert_eq!(b.name, "opt");
        assert_eq!(b.long.unwrap(), "opt");
        assert_eq!(b.short.unwrap(), 'o');
        assert_eq!(b.help.unwrap(), "some help info");
        assert!(b.is_set(ArgSettings::MultipleOccurrences));
        assert!(b.is_set(ArgSettings::TakesValue));
        assert!(!b.is_set(ArgSettings::Required));
        assert_eq!(
            b.val_names.unwrap().values().collect::<Vec<_>>(),
            [&"option"]
        );
        assert!(b.num_vals.is_none());
        assert_eq!(b.default_val.unwrap(), "a");
    }

    #[test]
    fn v3_default_create_option_usage_both_equals7() {
        let c = Arg::from("<option>... -o --opt=<opt> 'some help info [default: a]'");
        assert_eq!(c.name, "option");
        assert_eq!(c.long.unwrap(), "opt");
        assert_eq!(c.short.unwrap(), 'o');
        assert_eq!(c.help.unwrap(), "some help info");
        assert!(c.is_set(ArgSettings::MultipleOccurrences));
        assert!(c.is_set(ArgSettings::TakesValue));
        assert!(c.is_set(ArgSettings::Required));
        assert_eq!(
            c.val_names.unwrap().values().collect::<Vec<_>>(),
            [&"opt"]
        );
        assert!(c.num_vals.is_none());
        assert_eq!(c.default_val.unwrap(), "a");
    }

    #[test]
    fn v3_default_create_option_usage_both_equals8() {
        let d = Arg::from("-o --opt=<option>... 'some help info [default: a]'");
        assert_eq!(d.name, "opt");
        assert_eq!(d.long.unwrap(), "opt");
        assert_eq!(d.short.unwrap(), 'o');
        assert_eq!(d.help.unwrap(), "some help info");
        assert!(d.is_set(ArgSettings::MultipleOccurrences));
        assert!(d.is_set(ArgSettings::TakesValue));
        assert!(d.is_set(ArgSettings::Required));
        assert_eq!(
            d.val_names.unwrap().values().collect::<Vec<_>>(),
            [&"option"]
        );
        assert!(d.num_vals.is_none());
        assert_eq!(d.default_val.unwrap(), "a");
    }

    #[test]
    fn v3_default_create_option_with_vals1() {
        let d = Arg::from("-o <file> <mode> 'some help info [default: a]'");
        assert_eq!(d.name, "o");
        assert!(d.long.is_none());
        assert_eq!(d.short.unwrap(), 'o');
        assert_eq!(d.help.unwrap(), "some help info");
        assert!(!d.is_set(ArgSettings::MultipleOccurrences));
        assert!(d.is_set(ArgSettings::TakesValue));
        assert!(d.is_set(ArgSettings::Required));
        assert_eq!(
            d.val_names.unwrap().values().collect::<Vec<_>>(),
            [&"file", &"mode"]
        );
        assert_eq!(d.num_vals.unwrap(), 2);
        assert_eq!(d.default_val.unwrap(), "a");
    }

    #[test]
    fn v3_default_create_option_with_vals2() {
        let d = Arg::from("-o <file> <mode>... 'some help info [default: a]'");
        assert_eq!(d.name, "o");
        assert!(d.long.is_none());
        assert_eq!(d.short.unwrap(), 'o');
        assert_eq!(d.help.unwrap(), "some help info");
        assert!(d.is_set(ArgSettings::MultipleOccurrences));
        assert!(d.is_set(ArgSettings::TakesValue));
        assert!(d.is_set(ArgSettings::Required));
        assert_eq!(
            d.val_names.unwrap().values().collect::<Vec<_>>(),
            [&"file", &"mode"]
        );
        assert_eq!(d.num_vals.unwrap(), 2);
        assert_eq!(d.default_val.unwrap(), "a");
    }

    #[test]
    fn v3_default_create_option_with_vals3() {
        let d = Arg::from("--opt <file> <mode>... 'some help info [default: a]'");
        assert_eq!(d.name, "opt");
        assert!(d.short.is_none());
        assert_eq!(d.long.unwrap(), "opt");
        assert_eq!(d.help.unwrap(), "some help info");
        assert!(d.is_set(ArgSettings::MultipleOccurrences));
        assert!(d.is_set(ArgSettings::TakesValue));
        assert!(d.is_set(ArgSettings::Required));
        assert_eq!(
            d.val_names.unwrap().values().collect::<Vec<_>>(),
            [&"file", &"mode"]
        );
        assert_eq!(d.num_vals.unwrap(), 2);
        assert_eq!(d.default_val.unwrap(), "a");
    }

    #[test]
    fn v3_default_create_option_with_vals4() {
        let d = Arg::from("[myopt] --opt <file> <mode> 'some help info [default: a]'");
        assert_eq!(d.name, "myopt");
        assert!(d.short.is_none());
        assert_eq!(d.long.unwrap(), "opt");
        assert_eq!(d.help.unwrap(), "some help info");
        assert!(!d.is_set(ArgSettings::MultipleOccurrences));
        assert!(d.is_set(ArgSettings::TakesValue));
        assert!(!d.is_set(ArgSettings::Required));
        assert_eq!(
            d.val_names.unwrap().values().collect::<Vec<_>>(),
            [&"file", &"mode"]
        );
        assert_eq!(d.num_vals.unwrap(), 2);
        assert_eq!(d.default_val.unwrap(), "a");
    }

    #[test]
    fn v3_default_create_option_with_vals5() {
        let d = Arg::from("--opt <file> <mode> 'some help info [default: a]'");
        assert_eq!(d.name, "opt");
        assert!(d.short.is_none());
        assert_eq!(d.long.unwrap(), "opt");
        assert_eq!(d.help.unwrap(), "some help info");
        assert!(!d.is_set(ArgSettings::MultipleOccurrences));
        assert!(d.is_set(ArgSettings::TakesValue));
        assert!(d.is_set(ArgSettings::Required));
        assert_eq!(d.num_vals.unwrap(), 2);
        assert_eq!(d.default_val.unwrap(), "a");
    }

    #[test]
    fn v3_default_create_positional_usage() {
        let a = Arg::from("[pos] 'some help info [default: a]'");
        assert_eq!(a.name, "pos");
        assert_eq!(a.help.unwrap(), "some help info");
        assert!(!a.is_set(ArgSettings::MultipleOccurrences));
        assert!(!a.is_set(ArgSettings::Required));
        assert!(a.val_names.is_none());
        assert!(a.num_vals.is_none());
        assert_eq!(a.default_val.unwrap(), "a");
    }

    #[test]
    fn v3_default_create_positional_usage0() {
        let b = Arg::from("<pos> 'some help info [default: a]'");
        assert_eq!(b.name, "pos");
        assert_eq!(b.help.unwrap(), "some help info");
        assert!(!b.is_set(ArgSettings::MultipleOccurrences));
        assert!(b.is_set(ArgSettings::Required));
        assert!(b.val_names.is_none());
        assert!(b.num_vals.is_none());
        assert_eq!(b.default_val.unwrap(), "a");
    }

    #[test]
    fn v3_default_pos_mult_help() {
        let c = Arg::from("[pos]... 'some help info [default: a]'");
        assert_eq!(c.name, "pos");
        assert_eq!(c.help.unwrap(), "some help info");
        assert!(c.is_set(ArgSettings::MultipleOccurrences));
        assert!(!c.is_set(ArgSettings::Required));
        assert!(c.val_names.is_none());
        assert!(c.num_vals.is_none());
        assert_eq!(c.default_val.unwrap(), "a");
    }

    #[test]
    fn v3_default_pos_help_lit_single_quote() {
        let c = Arg::from("[pos]... 'some help\' info [default: a]'");
        assert_eq!(c.name, "pos");
        assert_eq!(c.help.unwrap(), "some help' info");
        assert!(c.is_set(ArgSettings::MultipleOccurrences));
        assert!(!c.is_set(ArgSettings::Required));
        assert!(c.val_names.is_none());
        assert!(c.num_vals.is_none());
        assert_eq!(c.default_val.unwrap(), "a");
    }

    #[test]
    fn v3_default_pos_help_double_lit_single_quote() {
        let c = Arg::from("[pos]... 'some \'help\' info [default: a]'");
        assert_eq!(c.name, "pos");
        assert_eq!(c.help.unwrap(), "some 'help' info");
        assert!(c.is_set(ArgSettings::MultipleOccurrences));
        assert!(!c.is_set(ArgSettings::Required));
        assert!(c.val_names.is_none());
        assert!(c.num_vals.is_none());
        assert_eq!(c.default_val.unwrap(), "a");
    }

    #[test]
    fn v3_default_pos_help_newline() {
        let c = Arg::from(
            "[pos]... 'some help{n}\
             info [default: a]'",
        );
        assert_eq!(c.name, "pos");
        assert_eq!(c.help.unwrap(), "some help{n}info");
        assert!(c.is_set(ArgSettings::MultipleOccurrences));
        assert!(!c.is_set(ArgSettings::Required));
        assert!(c.val_names.is_none());
        assert!(c.num_vals.is_none());
        assert_eq!(c.default_val.unwrap(), "a");
    }

    #[test]
    fn v3_default_pos_help_newline_lit_sq() {
        let c = Arg::from(
            "[pos]... 'some help\' stuff{n}\
             info [default: a]'",
        );
        assert_eq!(c.name, "pos");
        assert_eq!(c.help.unwrap(), "some help' stuff{n}info");
        assert!(c.is_set(ArgSettings::MultipleOccurrences));
        assert!(!c.is_set(ArgSettings::Required));
        assert!(c.val_names.is_none());
        assert!(c.num_vals.is_none());
        assert_eq!(c.default_val.unwrap(), "a");
    }

    #[test]
    fn v3_default_pos_req_mult_help() {
        let d = Arg::from("<pos>... 'some help info [default: a]'");
        assert_eq!(d.name, "pos");
        assert_eq!(d.help.unwrap(), "some help info");
        assert!(d.is_set(ArgSettings::MultipleOccurrences));
        assert!(d.is_set(ArgSettings::Required));
        assert!(d.val_names.is_none());
        assert!(d.num_vals.is_none());
        assert_eq!(d.default_val.unwrap(), "a");
    }

    #[test]
    fn v3_default_with_whitespace_pos_req_mult_help() {
        let d = Arg::from("<pos>... 'some help info [default: a b  c]'");
        assert_eq!(d.name, "pos");
        assert_eq!(d.help.unwrap(), "some help info");
        assert!(d.is_set(ArgSettings::MultipleOccurrences));
        assert!(d.is_set(ArgSettings::Required));
        assert!(d.val_names.is_none());
        assert!(d.num_vals.is_none());
        assert_eq!(d.default_val.unwrap(), "a b  c");
    }
}
