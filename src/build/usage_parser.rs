// Internal
use crate::build::{Arg, ArgSettings};
use crate::util::{Key, VecMap};
use crate::INTERNAL_ERROR_MSG;

#[derive(PartialEq, Debug)]
enum UsageToken {
    Name,
    ValName,
    Short,
    Long,
    Help,
    Multiple,
    Unknown,
}

#[doc(hidden)]
#[derive(Debug)]
pub struct UsageParser<'a> {
    usage: &'a str,
    pos: usize,
    start: usize,
    prev: UsageToken,
    explicit_name_set: bool,
}

impl<'a> UsageParser<'a> {
    fn new(usage: &'a str) -> Self {
        debugln!("UsageParser::new: usage={:?}", usage);
        UsageParser {
            usage,
            pos: 0,
            start: 0,
            prev: UsageToken::Unknown,
            explicit_name_set: false,
        }
    }

    pub fn from_usage(usage: &'a str) -> Self {
        debugln!("UsageParser::from_usage;");
        UsageParser::new(usage)
    }

    pub fn parse(mut self) -> Arg<'a> {
        debugln!("UsageParser::parse;");
        let mut arg = Arg::default();
        arg.disp_ord = 999;
        arg.unified_ord = 999;
        loop {
            debugln!("UsageParser::parse:iter: pos={};", self.pos);
            self.stop_at(token);
            if let Some(&c) = self.usage.as_bytes().get(self.pos) {
                match c {
                    b'-' => self.short_or_long(&mut arg),
                    b'.' => self.multiple(&mut arg),
                    b'\'' => self.help(&mut arg),
                    _ => self.name(&mut arg),
                }
            } else {
                break;
            }
        }
        arg.num_vals = match arg.val_names {
            Some(ref v) if v.len() >= 2 => Some(v.len() as u64),
            _ => None,
        };
        if !arg.has_switch() && arg.is_set(ArgSettings::MultipleOccurrences) {
            // We had a positional and need to set mult vals too
            arg.setb(ArgSettings::MultipleValues);
        }
        debugln!("UsageParser::parse: vals...{:?}", arg.val_names);
        arg
    }

    fn name(&mut self, arg: &mut Arg<'a>) {
        debugln!("UsageParser::name;");
        if *self
            .usage
            .as_bytes()
            .get(self.pos)
            .expect(INTERNAL_ERROR_MSG)
            == b'<'
            && !self.explicit_name_set
        {
            arg.setb(ArgSettings::Required);
        }
        self.pos += 1;
        self.stop_at(name_end);
        let name = &self.usage[self.start..self.pos];
        if self.prev == UsageToken::Unknown {
            debugln!("UsageParser::name: setting name...{}", name);
            arg.id = name.key();
            arg.name = name;
            if arg.long.is_none() && arg.short.is_none() {
                debugln!("UsageParser::name: explicit name set...");
                self.explicit_name_set = true;
                self.prev = UsageToken::Name;
            }
        } else {
            debugln!("UsageParser::name: setting val name...{}", name);
            if let Some(ref mut v) = arg.val_names {
                let len = v.len();
                v.insert(len, name);
            } else {
                let mut v = VecMap::new();
                v.insert(0, name);
                arg.val_names = Some(v);
                arg.setb(ArgSettings::TakesValue);
            }
            self.prev = UsageToken::ValName;
        }
    }

    fn stop_at<F>(&mut self, f: F)
    where
        F: Fn(u8) -> bool,
    {
        debugln!("UsageParser::stop_at;");
        self.start = self.pos;
        self.pos += self.usage[self.start..]
            .bytes()
            .take_while(|&b| f(b))
            .count();
    }

    fn short_or_long(&mut self, arg: &mut Arg<'a>) {
        debugln!("UsageParser::short_or_long;");
        self.pos += 1;
        if *self
            .usage
            .as_bytes()
            .get(self.pos)
            .expect(INTERNAL_ERROR_MSG)
            == b'-'
        {
            self.pos += 1;
            self.long(arg);
            return;
        }
        self.short(arg)
    }

    fn long(&mut self, arg: &mut Arg<'a>) {
        debugln!("UsageParser::long;");
        self.stop_at(long_end);
        let name = &self.usage[self.start..self.pos];
        if !self.explicit_name_set {
            debugln!("UsageParser::long: setting name...{}", name);
            arg.id = name.key();
            arg.name = name;
        }
        debugln!("UsageParser::long: setting long...{}", name);
        arg.long = Some(name);
        self.prev = UsageToken::Long;
    }

    fn short(&mut self, arg: &mut Arg<'a>) {
        debugln!("UsageParser::short;");
        let start = &self.usage[self.pos..];
        let short = start.chars().nth(0).expect(INTERNAL_ERROR_MSG);
        debugln!("UsageParser::short: setting short...{}", short);
        arg.short = Some(short);
        if arg.name.is_empty() {
            // --long takes precedence but doesn't set self.explicit_name_set
            let name = &start[..short.len_utf8()];
            debugln!("UsageParser::short: setting name...{}", name);
            arg.id = name.key();
            arg.name = name;
        }
        self.prev = UsageToken::Short;
    }

    // "something..."
    fn multiple(&mut self, arg: &mut Arg) {
        debugln!("UsageParser::multiple;");
        let mut dot_counter = 1;
        let start = self.pos;
        let mut bytes = self.usage[start..].bytes();
        while bytes.next() == Some(b'.') {
            dot_counter += 1;
            self.pos += 1;
            if dot_counter == 3 {
                debugln!("UsageParser::multiple: setting multiple");
                if arg.is_set(ArgSettings::TakesValue) {
                    arg.setb(ArgSettings::MultipleValues);
                }
                arg.setb(ArgSettings::MultipleOccurrences);
                self.prev = UsageToken::Multiple;
                self.pos += 1;
                break;
            }
        }
    }

    fn help(&mut self, arg: &mut Arg<'a>) {
        debugln!("UsageParser::help;");
        self.stop_at(help_start);
        self.start = self.pos + 1;
        self.pos = self.usage.len() - 1;
        debugln!(
            "UsageParser::help: setting help...{}",
            &self.usage[self.start..self.pos]
        );
        arg.help = Some(&self.usage[self.start..self.pos]);
        self.pos += 1; // Move to next byte to keep from thinking ending ' is a start
        self.prev = UsageToken::Help;
    }
}

#[inline]
fn name_end(b: u8) -> bool { b != b']' && b != b'>' }

#[inline]
fn token(b: u8) -> bool { b != b'\'' && b != b'.' && b != b'<' && b != b'[' && b != b'-' }

#[inline]
fn long_end(b: u8) -> bool {
    b != b'\'' && b != b'.' && b != b'<' && b != b'[' && b != b'=' && b != b' '
}

#[inline]
fn help_start(b: u8) -> bool { b != b'\'' }

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
}
