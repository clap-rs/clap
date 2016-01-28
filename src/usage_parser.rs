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
        self.pos += 1;
        self.stop_at(help_end);
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
    fn help_end(b: u32) -> bool {
        // 39(')
        b > 39 || b < 39
    }
