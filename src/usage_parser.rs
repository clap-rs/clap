use vec_map::VecMap;

use args::Arg;
use args::settings::ArgSettings;
use INTERNAL_ERROR_MSG;

type ParseResult = Result<(), ()>;

#[derive(PartialEq, Debug)]
enum UsageToken {
    Name,
    ValName,
    Short,
    Long,
    Help,
    Multiple,
    Unknown
}

#[doc(hidden)]
#[derive(Debug)]
pub struct UsageParser<'a> {
    usage: &'a str,
    pos: usize,
    start: usize,
    prev: UsageToken,
    explicit_name_set: bool
}

impl<'a> UsageParser<'a> {
    fn new(usage: &'a str) -> Self {
        debugln!("exec=new; usage={:?}", usage);
        UsageParser {
            usage: usage,
            pos: 0,
            start: 0,
            prev: UsageToken::Unknown,
            explicit_name_set: false,
        }
    }

    pub fn from_usage(usage: &'a str) -> Self {
        debugln!("fn=from_usage;");
        UsageParser::new(usage)
    }

    pub fn parse(mut self) -> Arg<'a, 'a> {
        debugln!("fn=parse;");
        let mut arg = Arg::default();
        loop {
            debugln!("iter; pos={};", self.pos);
            self.stop_at(token);
            if self.pos < self.usage.len() {
                if let Some(c) = self.usage.chars().nth(self.pos) {
                    match c {
                        '-'  => self.short_or_long(&mut arg),
                        '.'  => self.multiple(&mut arg),
                        '\'' => self.help(&mut arg),
                        _    => self.name(&mut arg),
                    }
                }
            } else { break; }
        }
        assert!(!arg.name.is_empty(), format!("No name found for Arg when parsing usage string: {}", self.usage));
        let n_vals = if let Some(ref v) = arg.val_names { v.len() } else { 0 };
        if n_vals > 1 {
            arg.num_vals = Some(n_vals as u8);
        }
        debugln!("vals: {:?}", arg.val_names);
        arg
    }

    fn name(&mut self, arg: &mut Arg<'a, 'a>) {
        debugln!("fn=name;");
        if self.usage.chars().nth(self.pos).expect(INTERNAL_ERROR_MSG) == '<' && !self.explicit_name_set { arg.setb(ArgSettings::Required); }
        self.pos += 1;
        self.stop_at(name_end);
        let name = &self.usage[self.start..self.pos];
        if self.prev != UsageToken::Unknown {
            debugln!("setting val name: {}", name);
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
        } else {
            debugln!("setting name: {}", name);
            arg.name = name;
            if arg.long.is_none() && arg.short.is_none() {
                debugln!("explicit name set...");
                self.explicit_name_set = true;
                self.prev = UsageToken::Name;
            }
        }
    }

    fn stop_at<F>(&mut self, f: F) where F: Fn(u32) -> bool {
        debugln!("fn=stop_at;");
        self.start = self.pos;
        for c in self.usage[self.start..].chars() {
            if f(c as u32) { self.pos += 1; continue; }
            break;
        }
    }

    fn short_or_long(&mut self, arg: &mut Arg<'a, 'a>) {
        debugln!("fn=short_or_long;");
        self.pos += 1;
        if self.usage.chars().nth(self.pos).expect(INTERNAL_ERROR_MSG) == '-' {
            self.pos += 1;
            self.long(arg);
            return;
        }
        self.short(arg)
    }

    fn long(&mut self, arg: &mut Arg<'a, 'a>) {
        debugln!("fn=long;");
        self.stop_at(long_end);
        let name = &self.usage[self.start..self.pos];
        if arg.name.is_empty() || (self.prev == UsageToken::Short && arg.name.len() == 1) {
            debugln!("setting name: {}", name);
            arg.name = name;
        }
        debugln!("setting long: {}", name);
        arg.long = Some(name);
        self.prev = UsageToken::Long;
    }

    fn short(&mut self, arg: &mut Arg<'a, 'a>) {
        debugln!("fn=short;");
        let name = &self.usage[self.pos..self.pos + 1];
        debugln!("setting short: {}", name);
        arg.short = Some(name.chars().nth(0).expect(INTERNAL_ERROR_MSG));
        if arg.name.is_empty() {
            debugln!("setting name: {}", name);
            arg.name = name;
        }
        self.prev = UsageToken::Short;
    }

    fn multiple(&mut self, arg: &mut Arg) {
        debugln!("fn=multiple;");
        let mut dot_counter = 1;
        let start = self.pos;
        for c in self.usage[start..].chars() {
            match c {
                '.' => {
                    dot_counter += 1;
                    self.pos += 1;
                    if dot_counter == 3 {
                        debugln!("setting multiple");
                        arg.setb(ArgSettings::Multiple);
                        self.prev = UsageToken::Multiple;
                    self.pos += 1;
                        break;
                    }
                },
                _ => break,
            }
        }
    }

    fn help(&mut self, arg: &mut Arg<'a, 'a>) {
        debugln!("fn=help;");
        self.stop_at(help_start);
        self.start = self.pos+1;
        self.pos = self.usage.len()-1;
        debugln!("setting help: {}", &self.usage[self.start..self.pos]);
        arg.help = Some(&self.usage[self.start..self.pos]);
        self.pos += 1;   // Move to next byte to keep from thinking ending ' is a start
        self.prev = UsageToken::Help;
    }
}

#[inline]
fn name_end(b: u32) -> bool {
    // 93(]), 62(>)
    b > b']' as u32 || b < b'>' as u32 || (b > b'>' as u32 && b < b']' as u32)
}

#[inline]
fn token(b: u32) -> bool {
    // 39('), 45(-), 46(.), 60(<), 91([)
    b < 39 || b > 91 || (b > 46 && b < 91 && b != b'<' as u32) || (b > 39 && b < 45)
}

#[inline]
fn long_end(b: u32) -> bool {
    // 39('), 46(.), 60(<), 61(=), 91([)
    (b < 39 && (b > 13 && b != b' ' as u32)) || b > 91 || (b > 61 && b < 91) || (b > 39 && b < 60 && b != 46)
}

#[inline]
fn help_start(b: u32) -> bool {
    // 39(')
    b < 39 || b > 39
}

#[cfg(test)]
mod test {
    use args::Arg;
    use args::ArgSettings;

    #[test]
    fn create_flag_usage() {
        let a = Arg::from_usage("[flag] -f 'some help info'");
        assert_eq!(a.name, "flag");
        assert_eq!(a.short.unwrap(), 'f');
        assert!(a.long.is_none());
        assert_eq!(a.help.unwrap(), "some help info");
        assert!(!a.is_set(ArgSettings::Multiple));
        assert!(a.val_names.is_none());
        assert!(a.num_vals.is_none());

        let b = Arg::from_usage("[flag] --flag 'some help info'");
        assert_eq!(b.name, "flag");
        assert_eq!(b.long.unwrap(), "flag");
        assert!(b.short.is_none());
        assert_eq!(b.help.unwrap(), "some help info");
        assert!(!b.is_set(ArgSettings::Multiple));
        assert!(a.val_names.is_none());
        assert!(a.num_vals.is_none());

        let b = Arg::from_usage("--flag 'some help info'");
        assert_eq!(b.name, "flag");
        assert_eq!(b.long.unwrap(), "flag");
        assert!(b.short.is_none());
        assert_eq!(b.help.unwrap(), "some help info");
        assert!(!b.is_set(ArgSettings::Multiple));
        assert!(b.val_names.is_none());
        assert!(b.num_vals.is_none());

        let c = Arg::from_usage("[flag] -f --flag 'some help info'");
        assert_eq!(c.name, "flag");
        assert_eq!(c.short.unwrap(), 'f');
        assert_eq!(c.long.unwrap(), "flag");
        assert_eq!(c.help.unwrap(), "some help info");
        assert!(!c.is_set(ArgSettings::Multiple));
        assert!(c.val_names.is_none());
        assert!(c.num_vals.is_none());

        let d = Arg::from_usage("[flag] -f... 'some help info'");
        assert_eq!(d.name, "flag");
        assert_eq!(d.short.unwrap(), 'f');
        assert!(d.long.is_none());
        assert_eq!(d.help.unwrap(), "some help info");
        assert!(d.is_set(ArgSettings::Multiple));
        assert!(d.val_names.is_none());
        assert!(d.num_vals.is_none());

        let e = Arg::from_usage("[flag] -f --flag... 'some help info'");
        assert_eq!(e.name, "flag");
        assert_eq!(e.long.unwrap(), "flag");
        assert_eq!(e.short.unwrap(), 'f');
        assert_eq!(e.help.unwrap(), "some help info");
        assert!(e.is_set(ArgSettings::Multiple));
        assert!(e.val_names.is_none());
        assert!(e.num_vals.is_none());

        let e = Arg::from_usage("-f --flag... 'some help info'");
        assert_eq!(e.name, "flag");
        assert_eq!(e.long.unwrap(), "flag");
        assert_eq!(e.short.unwrap(), 'f');
        assert_eq!(e.help.unwrap(), "some help info");
        assert!(e.is_set(ArgSettings::Multiple));
        assert!(e.val_names.is_none());
        assert!(e.num_vals.is_none());

        let e = Arg::from_usage("--flags");
        assert_eq!(e.name, "flags");
        assert_eq!(e.long.unwrap(), "flags");
        assert!(e.val_names.is_none());
        assert!(e.num_vals.is_none());

        let e = Arg::from_usage("--flags...");
        assert_eq!(e.name, "flags");
        assert_eq!(e.long.unwrap(), "flags");
        assert!(e.is_set(ArgSettings::Multiple));
        assert!(e.val_names.is_none());
        assert!(e.num_vals.is_none());

        let e = Arg::from_usage("[flags] -f");
        assert_eq!(e.name, "flags");
        assert_eq!(e.short.unwrap(), 'f');
        assert!(e.val_names.is_none());
        assert!(e.num_vals.is_none());

        let e = Arg::from_usage("[flags] -f...");
        assert_eq!(e.name, "flags");
        assert_eq!(e.short.unwrap(), 'f');
        assert!(e.is_set(ArgSettings::Multiple));
        assert!(e.val_names.is_none());
        assert!(e.num_vals.is_none());

        let a = Arg::from_usage("-f 'some help info'");
        assert_eq!(a.name, "f");
        assert_eq!(a.short.unwrap(), 'f');
        assert!(a.long.is_none());
        assert_eq!(a.help.unwrap(), "some help info");
        assert!(!a.is_set(ArgSettings::Multiple));
        assert!(a.val_names.is_none());
        assert!(a.num_vals.is_none());

        let e = Arg::from_usage("-f");
        assert_eq!(e.name, "f");
        assert_eq!(e.short.unwrap(), 'f');
        assert!(e.val_names.is_none());
        assert!(e.num_vals.is_none());

        let e = Arg::from_usage("-f...");
        assert_eq!(e.name, "f");
        assert_eq!(e.short.unwrap(), 'f');
        assert!(e.is_set(ArgSettings::Multiple));
        assert!(e.val_names.is_none());
        assert!(e.num_vals.is_none());
    }

    #[test]
    fn create_option_usage0() {
        // Short only
        let a = Arg::from_usage("[option] -o [opt] 'some help info'");
        assert_eq!(a.name, "option");
        assert_eq!(a.short.unwrap(), 'o');
        assert!(a.long.is_none());
        assert_eq!(a.help.unwrap(), "some help info");
        assert!(!a.is_set(ArgSettings::Multiple));
        assert!(a.is_set(ArgSettings::TakesValue));
        assert!(!a.is_set(ArgSettings::Required));
        assert_eq!(a.val_names.unwrap().iter().map(|(_, &v)| v).collect::<Vec<_>>(), ["opt"]);
        assert!(a.num_vals.is_none());
    }

    #[test]
    fn create_option_usage1() {
        let b = Arg::from_usage("-o [opt] 'some help info'");
        assert_eq!(b.name, "o");
        assert_eq!(b.short.unwrap(), 'o');
        assert!(b.long.is_none());
        assert_eq!(b.help.unwrap(), "some help info");
        assert!(!b.is_set(ArgSettings::Multiple));
        assert!(b.is_set(ArgSettings::TakesValue));
        assert!(!b.is_set(ArgSettings::Required));
        assert_eq!(b.val_names.unwrap().iter().map(|(_, &v)| v).collect::<Vec<_>>(), ["opt"]);
        assert!(b.num_vals.is_none());
    }

    #[test]
    fn create_option_usage2() {
        let c = Arg::from_usage("<option> -o <opt> 'some help info'");
        assert_eq!(c.name, "option");
        assert_eq!(c.short.unwrap(), 'o');
        assert!(c.long.is_none());
        assert_eq!(c.help.unwrap(), "some help info");
        assert!(!c.is_set(ArgSettings::Multiple));
        assert!(c.is_set(ArgSettings::TakesValue));
        assert!(c.is_set(ArgSettings::Required));
        assert_eq!(c.val_names.unwrap().iter().map(|(_, &v)| v).collect::<Vec<_>>(), ["opt"]);
        assert!(c.num_vals.is_none());
    }

    #[test]
    fn create_option_usage3() {
        let d = Arg::from_usage("-o <opt> 'some help info'");
        assert_eq!(d.name, "o");
        assert_eq!(d.short.unwrap(), 'o');
        assert!(d.long.is_none());
        assert_eq!(d.help.unwrap(), "some help info");
        assert!(!d.is_set(ArgSettings::Multiple));
        assert!(d.is_set(ArgSettings::TakesValue));
        assert!(d.is_set(ArgSettings::Required));
        assert_eq!(d.val_names.unwrap().iter().map(|(_, &v)| v).collect::<Vec<_>>(), ["opt"]);
        assert!(d.num_vals.is_none());
    }

    #[test]
    fn create_option_usage4() {
        let a = Arg::from_usage("[option] -o [opt]... 'some help info'");
        assert_eq!(a.name, "option");
        assert_eq!(a.short.unwrap(), 'o');
        assert!(a.long.is_none());
        assert_eq!(a.help.unwrap(), "some help info");
        assert!(a.is_set(ArgSettings::Multiple));
        assert!(a.is_set(ArgSettings::TakesValue));
        assert!(!a.is_set(ArgSettings::Required));
        assert_eq!(a.val_names.unwrap().iter().map(|(_, &v)| v).collect::<Vec<_>>(), ["opt"]);
        assert!(a.num_vals.is_none());
    }

    #[test]
    fn create_option_usage5() {
        let a = Arg::from_usage("[option]... -o [opt] 'some help info'");
        assert_eq!(a.name, "option");
        assert_eq!(a.short.unwrap(), 'o');
        assert!(a.long.is_none());
        assert_eq!(a.help.unwrap(), "some help info");
        assert!(a.is_set(ArgSettings::Multiple));
        assert!(a.is_set(ArgSettings::TakesValue));
        assert!(!a.is_set(ArgSettings::Required));
        assert_eq!(a.val_names.unwrap().iter().map(|(_, &v)| v).collect::<Vec<_>>(), ["opt"]);
        assert!(a.num_vals.is_none());
    }

    #[test]
    fn create_option_usage6() {
        let b = Arg::from_usage("-o [opt]... 'some help info'");
        assert_eq!(b.name, "o");
        assert_eq!(b.short.unwrap(), 'o');
        assert!(b.long.is_none());
        assert_eq!(b.help.unwrap(), "some help info");
        assert!(b.is_set(ArgSettings::Multiple));
        assert!(b.is_set(ArgSettings::TakesValue));
        assert!(!b.is_set(ArgSettings::Required));
        assert_eq!(b.val_names.unwrap().iter().map(|(_, &v)| v).collect::<Vec<_>>(), ["opt"]);
        assert!(b.num_vals.is_none());
    }

    #[test]
    fn create_option_usage7() {
        let c = Arg::from_usage("<option> -o <opt>... 'some help info'");
        assert_eq!(c.name, "option");
        assert_eq!(c.short.unwrap(), 'o');
        assert!(c.long.is_none());
        assert_eq!(c.help.unwrap(), "some help info");
        assert!(c.is_set(ArgSettings::Multiple));
        assert!(c.is_set(ArgSettings::TakesValue));
        assert!(c.is_set(ArgSettings::Required));
        assert_eq!(c.val_names.unwrap().iter().map(|(_, &v)| v).collect::<Vec<_>>(), ["opt"]);
        assert!(c.num_vals.is_none());
    }

    #[test]
    fn create_option_usage8() {
        let c = Arg::from_usage("<option>... -o <opt> 'some help info'");
        assert_eq!(c.name, "option");
        assert_eq!(c.short.unwrap(), 'o');
        assert!(c.long.is_none());
        assert_eq!(c.help.unwrap(), "some help info");
        assert!(c.is_set(ArgSettings::Multiple));
        assert!(c.is_set(ArgSettings::TakesValue));
        assert!(c.is_set(ArgSettings::Required));
        assert_eq!(c.val_names.unwrap().iter().map(|(_, &v)| v).collect::<Vec<_>>(), ["opt"]);
        assert!(c.num_vals.is_none());
    }

    #[test]
    fn create_option_usage9() {
        let d = Arg::from_usage("-o <opt>... 'some help info'");
        assert_eq!(d.name, "o");
        assert_eq!(d.short.unwrap(), 'o');
        assert!(d.long.is_none());
        assert_eq!(d.help.unwrap(), "some help info");
        assert!(d.is_set(ArgSettings::Multiple));
        assert!(d.is_set(ArgSettings::TakesValue));
        assert!(d.is_set(ArgSettings::Required));
        assert_eq!(d.val_names.unwrap().iter().map(|(_, &v)| v).collect::<Vec<_>>(), ["opt"]);
        assert!(d.num_vals.is_none());
    }

    #[test]
    fn create_option_usage_long1() {
        let a = Arg::from_usage("[option] --opt [opt] 'some help info'");
        assert_eq!(a.name, "option");
        assert_eq!(a.long.unwrap(), "opt");
        assert!(a.short.is_none());
        assert_eq!(a.help.unwrap(), "some help info");
        assert!(!a.is_set(ArgSettings::Multiple));
        assert!(a.is_set(ArgSettings::TakesValue));
        assert!(!a.is_set(ArgSettings::Required));
        assert_eq!(a.val_names.unwrap().iter().map(|(_, &v)| v).collect::<Vec<_>>(), ["opt"]);
        assert!(a.num_vals.is_none());
    }

    #[test]
    fn create_option_usage_long2() {
        let b = Arg::from_usage("--opt [option] 'some help info'");
        assert_eq!(b.name, "opt");
        assert_eq!(b.long.unwrap(), "opt");
        assert!(b.short.is_none());
        assert_eq!(b.help.unwrap(), "some help info");
        assert!(!b.is_set(ArgSettings::Multiple));
        assert!(b.is_set(ArgSettings::TakesValue));
        assert!(!b.is_set(ArgSettings::Required));
        assert_eq!(b.val_names.unwrap().iter().map(|(_, &v)| v).collect::<Vec<_>>(), ["option"]);
        assert!(b.num_vals.is_none());
    }

    #[test]
    fn create_option_usage_long3() {
        let c = Arg::from_usage("<option> --opt <opt> 'some help info'");
        assert_eq!(c.name, "option");
        assert_eq!(c.long.unwrap(), "opt");
        assert!(c.short.is_none());
        assert_eq!(c.help.unwrap(), "some help info");
        assert!(!c.is_set(ArgSettings::Multiple));
        assert!(c.is_set(ArgSettings::TakesValue));
        assert!(c.is_set(ArgSettings::Required));
        assert_eq!(c.val_names.unwrap().iter().map(|(_, &v)| v).collect::<Vec<_>>(), ["opt"]);
        assert!(c.num_vals.is_none());
    }

    #[test]
    fn create_option_usage_long4() {
        let d = Arg::from_usage("--opt <option> 'some help info'");
        assert_eq!(d.name, "opt");
        assert_eq!(d.long.unwrap(), "opt");
        assert!(d.short.is_none());
        assert_eq!(d.help.unwrap(), "some help info");
        assert!(!d.is_set(ArgSettings::Multiple));
        assert!(d.is_set(ArgSettings::TakesValue));
        assert!(d.is_set(ArgSettings::Required));
        assert_eq!(d.val_names.unwrap().iter().map(|(_, &v)| v).collect::<Vec<_>>(), ["option"]);
        assert!(d.num_vals.is_none());
    }

    #[test]
    fn create_option_usage_long5() {
        let a = Arg::from_usage("[option] --opt [opt]... 'some help info'");
        assert_eq!(a.name, "option");
        assert_eq!(a.long.unwrap(), "opt");
        assert!(a.short.is_none());
        assert_eq!(a.help.unwrap(), "some help info");
        assert!(a.is_set(ArgSettings::Multiple));
        assert!(a.is_set(ArgSettings::TakesValue));
        assert!(!a.is_set(ArgSettings::Required));
        assert_eq!(a.val_names.unwrap().iter().map(|(_, &v)| v).collect::<Vec<_>>(), ["opt"]);
        assert!(a.num_vals.is_none());
    }

    #[test]
    fn create_option_usage_long6() {
        let a = Arg::from_usage("[option]... --opt [opt] 'some help info'");
        assert_eq!(a.name, "option");
        assert_eq!(a.long.unwrap(), "opt");
        assert!(a.short.is_none());
        assert_eq!(a.help.unwrap(), "some help info");
        assert!(a.is_set(ArgSettings::Multiple));
        assert!(a.is_set(ArgSettings::TakesValue));
        assert!(!a.is_set(ArgSettings::Required));
        assert_eq!(a.val_names.unwrap().iter().map(|(_, &v)| v).collect::<Vec<_>>(), ["opt"]);
        assert!(a.num_vals.is_none());
    }

    #[test]
    fn create_option_usage_long7() {
        let b = Arg::from_usage("--opt [option]... 'some help info'");
        assert_eq!(b.name, "opt");
        assert_eq!(b.long.unwrap(), "opt");
        assert!(b.short.is_none());
        assert_eq!(b.help.unwrap(), "some help info");
        assert!(b.is_set(ArgSettings::Multiple));
        assert!(b.is_set(ArgSettings::TakesValue));
        assert!(!b.is_set(ArgSettings::Required));
        assert_eq!(b.val_names.unwrap().iter().map(|(_, &v)| v).collect::<Vec<_>>(), ["option"]);
        assert!(b.num_vals.is_none());
    }

    #[test]
    fn create_option_usage_long8() {
        let c = Arg::from_usage("<option> --opt <opt>... 'some help info'");
        assert_eq!(c.name, "option");
        assert_eq!(c.long.unwrap(), "opt");
        assert!(c.short.is_none());
        assert_eq!(c.help.unwrap(), "some help info");
        assert!(c.is_set(ArgSettings::Multiple));
        assert!(c.is_set(ArgSettings::TakesValue));
        assert!(c.is_set(ArgSettings::Required));
        assert_eq!(c.val_names.unwrap().iter().map(|(_, &v)| v).collect::<Vec<_>>(), ["opt"]);
        assert!(c.num_vals.is_none());
    }

    #[test]
    fn create_option_usage_long9() {
        let c = Arg::from_usage("<option>... --opt <opt> 'some help info'");
        assert_eq!(c.name, "option");
        assert_eq!(c.long.unwrap(), "opt");
        assert!(c.short.is_none());
        assert_eq!(c.help.unwrap(), "some help info");
        assert!(c.is_set(ArgSettings::Multiple));
        assert!(c.is_set(ArgSettings::TakesValue));
        assert!(c.is_set(ArgSettings::Required));
        assert_eq!(c.val_names.unwrap().iter().map(|(_, &v)| v).collect::<Vec<_>>(), ["opt"]);
        assert!(c.num_vals.is_none());
    }

    #[test]
    fn create_option_usage_long10() {
        let d = Arg::from_usage("--opt <option>... 'some help info'");
        assert_eq!(d.name, "opt");
        assert_eq!(d.long.unwrap(), "opt");
        assert!(d.short.is_none());
        assert_eq!(d.help.unwrap(), "some help info");
        assert!(d.is_set(ArgSettings::Multiple));
        assert!(d.is_set(ArgSettings::TakesValue));
        assert!(d.is_set(ArgSettings::Required));
        assert_eq!(d.val_names.unwrap().iter().map(|(_, &v)| v).collect::<Vec<_>>(), ["option"]);
        assert!(d.num_vals.is_none());
    }

    #[test]
    fn create_option_usage_long_equals1() {
        let a = Arg::from_usage("[option] --opt=[opt] 'some help info'");
        assert_eq!(a.name, "option");
        assert_eq!(a.long.unwrap(), "opt");
        assert!(a.short.is_none());
        assert_eq!(a.help.unwrap(), "some help info");
        assert!(!a.is_set(ArgSettings::Multiple));
        assert!(a.is_set(ArgSettings::TakesValue));
        assert!(!a.is_set(ArgSettings::Required));
        assert_eq!(a.val_names.unwrap().iter().map(|(_, &v)| v).collect::<Vec<_>>(), ["opt"]);
        assert!(a.num_vals.is_none());
    }

    #[test]
    fn create_option_usage_long_equals2() {
        let b = Arg::from_usage("--opt=[option] 'some help info'");
        assert_eq!(b.name, "opt");
        assert_eq!(b.long.unwrap(), "opt");
        assert!(b.short.is_none());
        assert_eq!(b.help.unwrap(), "some help info");
        assert!(!b.is_set(ArgSettings::Multiple));
        assert!(b.is_set(ArgSettings::TakesValue));
        assert!(!b.is_set(ArgSettings::Required));
        assert_eq!(b.val_names.unwrap().iter().map(|(_, &v)| v).collect::<Vec<_>>(), ["option"]);
        assert!(b.num_vals.is_none());
    }

    #[test]
    fn create_option_usage_long_equals3() {
        let c = Arg::from_usage("<option> --opt=<opt> 'some help info'");
        assert_eq!(c.name, "option");
        assert_eq!(c.long.unwrap(), "opt");
        assert!(c.short.is_none());
        assert_eq!(c.help.unwrap(), "some help info");
        assert!(!c.is_set(ArgSettings::Multiple));
        assert!(c.is_set(ArgSettings::TakesValue));
        assert!(c.is_set(ArgSettings::Required));
        assert_eq!(c.val_names.unwrap().iter().map(|(_, &v)| v).collect::<Vec<_>>(), ["opt"]);
        assert!(c.num_vals.is_none());
    }

    #[test]
    fn create_option_usage_long_equals4() {
        let d = Arg::from_usage("--opt=<option> 'some help info'");
        assert_eq!(d.name, "opt");
        assert_eq!(d.long.unwrap(), "opt");
        assert!(d.short.is_none());
        assert_eq!(d.help.unwrap(), "some help info");
        assert!(!d.is_set(ArgSettings::Multiple));
        assert!(d.is_set(ArgSettings::TakesValue));
        assert!(d.is_set(ArgSettings::Required));
        assert_eq!(d.val_names.unwrap().iter().map(|(_, &v)| v).collect::<Vec<_>>(), ["option"]);
        assert!(d.num_vals.is_none());
    }

    #[test]
    fn create_option_usage_long_equals5() {
        let a = Arg::from_usage("[option] --opt=[opt]... 'some help info'");
        assert_eq!(a.name, "option");
        assert_eq!(a.long.unwrap(), "opt");
        assert!(a.short.is_none());
        assert_eq!(a.help.unwrap(), "some help info");
        assert!(a.is_set(ArgSettings::Multiple));
        assert!(a.is_set(ArgSettings::TakesValue));
        assert!(!a.is_set(ArgSettings::Required));
        assert_eq!(a.val_names.unwrap().iter().map(|(_, &v)| v).collect::<Vec<_>>(), ["opt"]);
        assert!(a.num_vals.is_none());
    }

    #[test]
    fn create_option_usage_long_equals6() {
        let a = Arg::from_usage("[option]... --opt=[opt] 'some help info'");
        assert_eq!(a.name, "option");
        assert_eq!(a.long.unwrap(), "opt");
        assert!(a.short.is_none());
        assert_eq!(a.help.unwrap(), "some help info");
        assert!(a.is_set(ArgSettings::Multiple));
        assert!(a.is_set(ArgSettings::TakesValue));
        assert!(!a.is_set(ArgSettings::Required));
        assert_eq!(a.val_names.unwrap().iter().map(|(_, &v)| v).collect::<Vec<_>>(), ["opt"]);
        assert!(a.num_vals.is_none());
    }

    #[test]
    fn create_option_usage_long_equals7() {
        let b = Arg::from_usage("--opt=[option]... 'some help info'");
        assert_eq!(b.name, "opt");
        assert_eq!(b.long.unwrap(), "opt");
        assert!(b.short.is_none());
        assert_eq!(b.help.unwrap(), "some help info");
        assert!(b.is_set(ArgSettings::Multiple));
        assert!(b.is_set(ArgSettings::TakesValue));
        assert!(!b.is_set(ArgSettings::Required));
        assert_eq!(b.val_names.unwrap().iter().map(|(_, &v)| v).collect::<Vec<_>>(), ["option"]);
        assert!(b.num_vals.is_none());
    }

    #[test]
    fn create_option_usage_long_equals8() {
        let c = Arg::from_usage("<option> --opt=<opt>... 'some help info'");
        assert_eq!(c.name, "option");
        assert_eq!(c.long.unwrap(), "opt");
        assert!(c.short.is_none());
        assert_eq!(c.help.unwrap(), "some help info");
        assert!(c.is_set(ArgSettings::Multiple));
        assert!(c.is_set(ArgSettings::TakesValue));
        assert!(c.is_set(ArgSettings::Required));
        assert_eq!(c.val_names.unwrap().iter().map(|(_, &v)| v).collect::<Vec<_>>(), ["opt"]);
        assert!(c.num_vals.is_none());
    }

    #[test]
    fn create_option_usage_long_equals9() {
        let c = Arg::from_usage("<option>... --opt=<opt> 'some help info'");
        assert_eq!(c.name, "option");
        assert_eq!(c.long.unwrap(), "opt");
        assert!(c.short.is_none());
        assert_eq!(c.help.unwrap(), "some help info");
        assert!(c.is_set(ArgSettings::Multiple));
        assert!(c.is_set(ArgSettings::TakesValue));
        assert!(c.is_set(ArgSettings::Required));
        assert_eq!(c.val_names.unwrap().iter().map(|(_, &v)| v).collect::<Vec<_>>(), ["opt"]);
        assert!(c.num_vals.is_none());
    }

    #[test]
    fn create_option_usage_long_equals10() {
        let d = Arg::from_usage("--opt=<option>... 'some help info'");
        assert_eq!(d.name, "opt");
        assert_eq!(d.long.unwrap(), "opt");
        assert!(d.short.is_none());
        assert_eq!(d.help.unwrap(), "some help info");
        assert!(d.is_set(ArgSettings::Multiple));
        assert!(d.is_set(ArgSettings::TakesValue));
        assert!(d.is_set(ArgSettings::Required));
        assert_eq!(d.val_names.unwrap().iter().map(|(_, &v)| v).collect::<Vec<_>>(), ["option"]);
        assert!(d.num_vals.is_none());
    }

    #[test]
    fn create_option_usage_both1() {
        let a = Arg::from_usage("[option] -o --opt [option] 'some help info'");
        assert_eq!(a.name, "option");
        assert_eq!(a.long.unwrap(), "opt");
        assert_eq!(a.short.unwrap(), 'o');
        assert_eq!(a.help.unwrap(), "some help info");
        assert!(!a.is_set(ArgSettings::Multiple));
        assert!(a.is_set(ArgSettings::TakesValue));
        assert!(!a.is_set(ArgSettings::Required));
        assert_eq!(a.val_names.unwrap().iter().map(|(_, &v)| v).collect::<Vec<_>>(), ["option"]);
        assert!(a.num_vals.is_none());
    }

    #[test]
    fn create_option_usage_both2() {
        let b = Arg::from_usage("-o --opt [option] 'some help info'");
        assert_eq!(b.name, "opt");
        assert_eq!(b.long.unwrap(), "opt");
        assert_eq!(b.short.unwrap(), 'o');
        assert_eq!(b.help.unwrap(), "some help info");
        assert!(!b.is_set(ArgSettings::Multiple));
        assert!(b.is_set(ArgSettings::TakesValue));
        assert!(!b.is_set(ArgSettings::Required));
        assert_eq!(b.val_names.unwrap().iter().map(|(_, &v)| v).collect::<Vec<_>>(), ["option"]);
        assert!(b.num_vals.is_none());
    }

    #[test]
    fn create_option_usage_both3() {
        let c = Arg::from_usage("<option> -o --opt <opt> 'some help info'");
        assert_eq!(c.name, "option");
        assert_eq!(c.long.unwrap(), "opt");
        assert_eq!(c.short.unwrap(), 'o');
        assert_eq!(c.help.unwrap(), "some help info");
        assert!(!c.is_set(ArgSettings::Multiple));
        assert!(c.is_set(ArgSettings::TakesValue));
        assert!(c.is_set(ArgSettings::Required));
        assert_eq!(c.val_names.unwrap().iter().map(|(_, &v)| v).collect::<Vec<_>>(), ["opt"]);
        assert!(c.num_vals.is_none());
    }

    #[test]
    fn create_option_usage_both4() {
        let d = Arg::from_usage("-o --opt <option> 'some help info'");
        assert_eq!(d.name, "opt");
        assert_eq!(d.long.unwrap(), "opt");
        assert_eq!(d.short.unwrap(), 'o');
        assert_eq!(d.help.unwrap(), "some help info");
        assert!(!d.is_set(ArgSettings::Multiple));
        assert!(d.is_set(ArgSettings::TakesValue));
        assert!(d.is_set(ArgSettings::Required));
        assert_eq!(d.val_names.unwrap().iter().map(|(_, &v)| v).collect::<Vec<_>>(), ["option"]);
        assert!(d.num_vals.is_none());
    }

    #[test]
    fn create_option_usage_both5() {
        let a = Arg::from_usage("[option]... -o --opt [option] 'some help info'");
        assert_eq!(a.name, "option");
        assert_eq!(a.long.unwrap(), "opt");
        assert_eq!(a.short.unwrap(), 'o');
        assert_eq!(a.help.unwrap(), "some help info");
        assert!(a.is_set(ArgSettings::Multiple));
        assert!(a.is_set(ArgSettings::TakesValue));
        assert!(!a.is_set(ArgSettings::Required));
        assert_eq!(a.val_names.unwrap().iter().map(|(_, &v)| v).collect::<Vec<_>>(), ["option"]);
        assert!(a.num_vals.is_none());
    }

    #[test]
    fn create_option_usage_both6() {
        let b = Arg::from_usage("-o --opt [option]... 'some help info'");
        assert_eq!(b.name, "opt");
        assert_eq!(b.long.unwrap(), "opt");
        assert_eq!(b.short.unwrap(), 'o');
        assert_eq!(b.help.unwrap(), "some help info");
        assert!(b.is_set(ArgSettings::Multiple));
        assert!(b.is_set(ArgSettings::TakesValue));
        assert!(!b.is_set(ArgSettings::Required));
        assert_eq!(b.val_names.unwrap().iter().map(|(_, &v)| v).collect::<Vec<_>>(), ["option"]);
        assert!(b.num_vals.is_none());
    }

    #[test]
    fn create_option_usage_both7() {
        let c = Arg::from_usage("<option>... -o --opt <opt> 'some help info'");
        assert_eq!(c.name, "option");
        assert_eq!(c.long.unwrap(), "opt");
        assert_eq!(c.short.unwrap(), 'o');
        assert_eq!(c.help.unwrap(), "some help info");
        assert!(c.is_set(ArgSettings::Multiple));
        assert!(c.is_set(ArgSettings::TakesValue));
        assert!(c.is_set(ArgSettings::Required));
        assert_eq!(c.val_names.unwrap().iter().map(|(_, &v)| v).collect::<Vec<_>>(), ["opt"]);
        assert!(c.num_vals.is_none());
    }

    #[test]
    fn create_option_usage_both8() {
        let d = Arg::from_usage("-o --opt <option>... 'some help info'");
        assert_eq!(d.name, "opt");
        assert_eq!(d.long.unwrap(), "opt");
        assert_eq!(d.short.unwrap(), 'o');
        assert_eq!(d.help.unwrap(), "some help info");
        assert!(d.is_set(ArgSettings::Multiple));
        assert!(d.is_set(ArgSettings::TakesValue));
        assert!(d.is_set(ArgSettings::Required));
        assert_eq!(d.val_names.unwrap().iter().map(|(_, &v)| v).collect::<Vec<_>>(), ["option"]);
        assert!(d.num_vals.is_none());
    }

    #[test]
    fn create_option_usage_both_equals1() {
        let a = Arg::from_usage("[option] -o --opt=[option] 'some help info'");
        assert_eq!(a.name, "option");
        assert_eq!(a.long.unwrap(), "opt");
        assert_eq!(a.short.unwrap(), 'o');
        assert_eq!(a.help.unwrap(), "some help info");
        assert!(!a.is_set(ArgSettings::Multiple));
        assert!(a.is_set(ArgSettings::TakesValue));
        assert!(!a.is_set(ArgSettings::Required));
        assert_eq!(a.val_names.unwrap().iter().map(|(_, &v)| v).collect::<Vec<_>>(), ["option"]);
        assert!(a.num_vals.is_none());
    }

    #[test]
    fn create_option_usage_both_equals2() {
        let b = Arg::from_usage("-o --opt=[option] 'some help info'");
        assert_eq!(b.name, "opt");
        assert_eq!(b.long.unwrap(), "opt");
        assert_eq!(b.short.unwrap(), 'o');
        assert_eq!(b.help.unwrap(), "some help info");
        assert!(!b.is_set(ArgSettings::Multiple));
        assert!(b.is_set(ArgSettings::TakesValue));
        assert!(!b.is_set(ArgSettings::Required));
        assert_eq!(b.val_names.unwrap().iter().map(|(_, &v)| v).collect::<Vec<_>>(), ["option"]);
        assert!(b.num_vals.is_none());
    }

    #[test]
    fn create_option_usage_both_equals3() {
        let c = Arg::from_usage("<option> -o --opt=<opt> 'some help info'");
        assert_eq!(c.name, "option");
        assert_eq!(c.long.unwrap(), "opt");
        assert_eq!(c.short.unwrap(), 'o');
        assert_eq!(c.help.unwrap(), "some help info");
        assert!(!c.is_set(ArgSettings::Multiple));
        assert!(c.is_set(ArgSettings::TakesValue));
        assert!(c.is_set(ArgSettings::Required));
        assert_eq!(c.val_names.unwrap().iter().map(|(_, &v)| v).collect::<Vec<_>>(), ["opt"]);
        assert!(c.num_vals.is_none());
    }

    #[test]
    fn create_option_usage_both_equals4() {
        let d = Arg::from_usage("-o --opt=<option> 'some help info'");
        assert_eq!(d.name, "opt");
        assert_eq!(d.long.unwrap(), "opt");
        assert_eq!(d.short.unwrap(), 'o');
        assert_eq!(d.help.unwrap(), "some help info");
        assert!(!d.is_set(ArgSettings::Multiple));
        assert!(d.is_set(ArgSettings::TakesValue));
        assert!(d.is_set(ArgSettings::Required));
        assert_eq!(d.val_names.unwrap().iter().map(|(_, &v)| v).collect::<Vec<_>>(), ["option"]);
        assert!(d.num_vals.is_none());
    }

    #[test]
    fn create_option_usage_both_equals5() {
        let a = Arg::from_usage("[option]... -o --opt=[option] 'some help info'");
        assert_eq!(a.name, "option");
        assert_eq!(a.long.unwrap(), "opt");
        assert_eq!(a.short.unwrap(), 'o');
        assert_eq!(a.help.unwrap(), "some help info");
        assert!(a.is_set(ArgSettings::Multiple));
        assert!(a.is_set(ArgSettings::TakesValue));
        assert!(!a.is_set(ArgSettings::Required));
        assert_eq!(a.val_names.unwrap().iter().map(|(_, &v)| v).collect::<Vec<_>>(), ["option"]);
        assert!(a.num_vals.is_none());
    }

    #[test]
    fn create_option_usage_both_equals6() {
        let b = Arg::from_usage("-o --opt=[option]... 'some help info'");
        assert_eq!(b.name, "opt");
        assert_eq!(b.long.unwrap(), "opt");
        assert_eq!(b.short.unwrap(), 'o');
        assert_eq!(b.help.unwrap(), "some help info");
        assert!(b.is_set(ArgSettings::Multiple));
        assert!(b.is_set(ArgSettings::TakesValue));
        assert!(!b.is_set(ArgSettings::Required));
        assert_eq!(b.val_names.unwrap().iter().map(|(_, &v)| v).collect::<Vec<_>>(), ["option"]);
        assert!(b.num_vals.is_none());
    }

    #[test]
    fn create_option_usage_both_equals7() {
        let c = Arg::from_usage("<option>... -o --opt=<opt> 'some help info'");
        assert_eq!(c.name, "option");
        assert_eq!(c.long.unwrap(), "opt");
        assert_eq!(c.short.unwrap(), 'o');
        assert_eq!(c.help.unwrap(), "some help info");
        assert!(c.is_set(ArgSettings::Multiple));
        assert!(c.is_set(ArgSettings::TakesValue));
        assert!(c.is_set(ArgSettings::Required));
        assert_eq!(c.val_names.unwrap().iter().map(|(_, &v)| v).collect::<Vec<_>>(), ["opt"]);
        assert!(c.num_vals.is_none());
    }

    #[test]
    fn create_option_usage_both_equals8() {
        let d = Arg::from_usage("-o --opt=<option>... 'some help info'");
        assert_eq!(d.name, "opt");
        assert_eq!(d.long.unwrap(), "opt");
        assert_eq!(d.short.unwrap(), 'o');
        assert_eq!(d.help.unwrap(), "some help info");
        assert!(d.is_set(ArgSettings::Multiple));
        assert!(d.is_set(ArgSettings::TakesValue));
        assert!(d.is_set(ArgSettings::Required));
        assert_eq!(d.val_names.unwrap().iter().map(|(_, &v)| v).collect::<Vec<_>>(), ["option"]);
        assert!(d.num_vals.is_none());
    }

    #[test]
    fn create_option_with_vals1() {
        let d = Arg::from_usage("-o <file> <mode> 'some help info'");
        assert_eq!(d.name, "o");
        assert!(d.long.is_none());
        assert_eq!(d.short.unwrap(), 'o');
        assert_eq!(d.help.unwrap(), "some help info");
        assert!(!d.is_set(ArgSettings::Multiple));
        assert!(d.is_set(ArgSettings::TakesValue));
        assert!(d.is_set(ArgSettings::Required));
        assert_eq!(d.val_names.unwrap().iter().map(|(_, &v)| v).collect::<Vec<_>>(), ["file", "mode"]);
        assert_eq!(d.num_vals.unwrap(), 2);
    }

    #[test]
    fn create_option_with_vals2() {
        let d = Arg::from_usage("-o <file> <mode>... 'some help info'");
        assert_eq!(d.name, "o");
        assert!(d.long.is_none());
        assert_eq!(d.short.unwrap(), 'o');
        assert_eq!(d.help.unwrap(), "some help info");
        assert!(d.is_set(ArgSettings::Multiple));
        assert!(d.is_set(ArgSettings::TakesValue));
        assert!(d.is_set(ArgSettings::Required));
        assert_eq!(d.val_names.unwrap().iter().map(|(_, &v)| v).collect::<Vec<_>>(), ["file", "mode"]);
        assert_eq!(d.num_vals.unwrap(), 2);
    }

    #[test]
    fn create_option_with_vals3() {
        let d = Arg::from_usage("--opt <file> <mode>... 'some help info'");
        assert_eq!(d.name, "opt");
        assert!(d.short.is_none());
        assert_eq!(d.long.unwrap(), "opt");
        assert_eq!(d.help.unwrap(), "some help info");
        assert!(d.is_set(ArgSettings::Multiple));
        assert!(d.is_set(ArgSettings::TakesValue));
        assert!(d.is_set(ArgSettings::Required));
        assert_eq!(d.val_names.unwrap().iter().map(|(_, &v)| v).collect::<Vec<_>>(), ["file", "mode"]);
        assert_eq!(d.num_vals.unwrap(), 2);
    }

    #[test]
    fn create_option_with_vals4() {
        let d = Arg::from_usage("[myopt] --opt <file> <mode> 'some help info'");
        assert_eq!(d.name, "myopt");
        assert!(d.short.is_none());
        assert_eq!(d.long.unwrap(), "opt");
        assert_eq!(d.help.unwrap(), "some help info");
        assert!(!d.is_set(ArgSettings::Multiple));
        assert!(d.is_set(ArgSettings::TakesValue));
        assert!(!d.is_set(ArgSettings::Required));
        assert_eq!(d.val_names.unwrap().iter().map(|(_, &v)| v).collect::<Vec<_>>(), ["file", "mode"]);
        assert_eq!(d.num_vals.unwrap(), 2);
    }

    #[test]
    fn create_option_with_vals5() {
        let d = Arg::from_usage("--opt <file> <mode> 'some help info'");
        assert_eq!(d.name, "opt");
        assert!(d.short.is_none());
        assert_eq!(d.long.unwrap(), "opt");
        assert_eq!(d.help.unwrap(), "some help info");
        assert!(!d.is_set(ArgSettings::Multiple));
        assert!(d.is_set(ArgSettings::TakesValue));
        assert!(d.is_set(ArgSettings::Required));
        assert_eq!(d.num_vals.unwrap(), 2);
    }

    #[test]
    fn create_positional_usage() {
        let a = Arg::from_usage("[pos] 'some help info'");
        assert_eq!(a.name, "pos");
        assert_eq!(a.help.unwrap(), "some help info");
        assert!(!a.is_set(ArgSettings::Multiple));
        assert!(!a.is_set(ArgSettings::Required));
        assert!(a.val_names.is_none());
        assert!(a.num_vals.is_none());
    }

    #[test]
    fn create_positional_usage0() {
        let b = Arg::from_usage("<pos> 'some help info'");
        assert_eq!(b.name, "pos");
        assert_eq!(b.help.unwrap(), "some help info");
        assert!(!b.is_set(ArgSettings::Multiple));
        assert!(b.is_set(ArgSettings::Required));
        assert!(b.val_names.is_none());
        assert!(b.num_vals.is_none());
    }

    #[test]
    fn pos_mult_help() {
        let c = Arg::from_usage("[pos]... 'some help info'");
        assert_eq!(c.name, "pos");
        assert_eq!(c.help.unwrap(), "some help info");
        assert!(c.is_set(ArgSettings::Multiple));
        assert!(!c.is_set(ArgSettings::Required));
        assert!(c.val_names.is_none());
        assert!(c.num_vals.is_none());
    }

    #[test]
    fn pos_help_lit_single_quote() {
        let c = Arg::from_usage("[pos]... 'some help\' info'");
        assert_eq!(c.name, "pos");
        assert_eq!(c.help.unwrap(), "some help' info");
        assert!(c.is_set(ArgSettings::Multiple));
        assert!(!c.is_set(ArgSettings::Required));
        assert!(c.val_names.is_none());
        assert!(c.num_vals.is_none());
    }

    #[test]
    fn pos_help_double_lit_single_quote() {
        let c = Arg::from_usage("[pos]... 'some \'help\' info'");
        assert_eq!(c.name, "pos");
        assert_eq!(c.help.unwrap(), "some 'help' info");
        assert!(c.is_set(ArgSettings::Multiple));
        assert!(!c.is_set(ArgSettings::Required));
        assert!(c.val_names.is_none());
        assert!(c.num_vals.is_none());
    }

    #[test]
    fn pos_help_newline() {
        let c = Arg::from_usage("[pos]... 'some help{n}\
                                           info'");
        assert_eq!(c.name, "pos");
        assert_eq!(c.help.unwrap(), "some help{n}info");
        assert!(c.is_set(ArgSettings::Multiple));
        assert!(!c.is_set(ArgSettings::Required));
        assert!(c.val_names.is_none());
        assert!(c.num_vals.is_none());
    }

    #[test]
    fn pos_help_newline_lit_sq() {
        let c = Arg::from_usage("[pos]... 'some help\' stuff{n}\
                                           info'");
        assert_eq!(c.name, "pos");
        assert_eq!(c.help.unwrap(), "some help' stuff{n}info");
        assert!(c.is_set(ArgSettings::Multiple));
        assert!(!c.is_set(ArgSettings::Required));
        assert!(c.val_names.is_none());
        assert!(c.num_vals.is_none());
    }

    #[test]
    fn pos_req_mult_help() {
        let d = Arg::from_usage("<pos>... 'some help info'");
        assert_eq!(d.name, "pos");
        assert_eq!(d.help.unwrap(), "some help info");
        assert!(d.is_set(ArgSettings::Multiple));
        assert!(d.is_set(ArgSettings::Required));
        assert!(d.val_names.is_none());
        assert!(d.num_vals.is_none());
    }

    #[test]
    fn pos_req() {
        let b = Arg::from_usage("<pos>");
        assert_eq!(b.name, "pos");
        assert!(!b.is_set(ArgSettings::Multiple));
        assert!(b.is_set(ArgSettings::Required));
        assert!(b.val_names.is_none());
        assert!(b.num_vals.is_none());
    }

    #[test]
    fn pos_mult() {
        let c = Arg::from_usage("[pos]...");
        assert_eq!(c.name, "pos");
        assert!(c.is_set(ArgSettings::Multiple));
        assert!(!c.is_set(ArgSettings::Required));
        assert!(c.val_names.is_none());
        assert!(c.num_vals.is_none());
    }
}
