use std::str::Chars;

pub enum UsageToken<'u> {
    Name(&'u str, Option<bool>),
    Short(char),
    Long(&'u str),
    Help(&'u str),
    Multiple,
}

pub struct UsageParser<'u> {
    usage: &'u str,
    chars: Chars<'u>,
    s: usize,
    e: usize,
}

impl<'u> UsageParser<'u> {
    pub fn with_usage(u: &'u str) -> UsageParser<'u> {
        UsageParser {
            usage: u,
            chars: u.chars(),
            s: 0,
            e: 0,
        }
    }


}

impl<'u> Iterator for UsageParser<'u> {
    type Item = UsageToken<'u>;

    fn next(&mut self) -> Option<UsageToken<'u>> {
        loop {
            match self.chars.next() {
                Some(c) if c == '[' || c == '<'  => {
                    // self.s = self.e + 1;
                    if self.e != 0 {
                        self.e += 1;
                    }
                    self.s = self.e + 1;
                    let closing = match c {
                        '[' => ']',
                        '<' => '>',
                        _   => unreachable!()
                    };
                    while let Some(c) =  self.chars.next() {
                        self.e += 1;
                        if c == closing { break }
                    }
                    if self.e > self.usage.len() { return None }

                    let name = &self.usage[self.s..self.e];

                    return Some(UsageToken::Name(name, if c == '<' { Some(true) } else { None }));
                },
                Some('\'') => {
                    self.s = self.e + 2;
                    self.e = self.usage.len() - 1;

                    while let Some(_) = self.chars.next() { continue }

                    return Some(UsageToken::Help(&self.usage[self.s..self.e]));
                },
                Some('-')  => {
                    self.e += 1;
                    match self.chars.next() {
                        Some('-') => {
                            if self.e != 1 {
                                self.e += 1;
                            }

                            self.s = self.e + 1;

                            while let Some(c) = self.chars.next() {
                                self.e += 1;
                                if c == ' ' || c == '=' || c == '.' { break }
                            }
                            if self.e > self.usage.len() { return None }

                            if self.e == self.usage.len() - 1 {
                                return Some(UsageToken::Long(&self.usage[self.s..]))
                            }
                            return Some(UsageToken::Long(&self.usage[self.s..self.e]))
                        },
                        Some(c)  => {
                            // When short is first don't increment e
                            if self.e != 1 {
                                self.e += 1;
                            }
                            // Short
                            if !c.is_alphanumeric() {
                                return None
                            }
                            return Some(UsageToken::Short(c))
                        },
                        _      => {
                            return None
                        }
                    }
                },
                Some('.') => {
                    self.e += 1;
                    let mut mult = false;
                    for _ in 0..2 {
                        self.e += 1;
                        match self.chars.next() {
                            // longs consume one '.' so they match '.. ' whereas shorts can
                            // match '...'
                            Some('.') | Some(' ')  => { mult = true; },
                            _          => {
                                // if there is no help or following space all we can match is '..'
                                if self.e == self.usage.len() - 1 {
                                    mult = true;
                                }
                                break;
                            }
                        }
                    }
                    if mult { return Some(UsageToken::Multiple) }
                },
                Some(' ') | Some('=') | Some(']') | Some('>') | Some('\t') | Some(',') => {
                    self.e += 1;
                    continue
                },
                _  => {
                    return None
                }
            }
        }
    }
}
