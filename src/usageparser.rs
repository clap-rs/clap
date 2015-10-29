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

    fn name(&mut self, c: char) -> Option<UsageToken<'u>> {
        if self.e != 0 {
            self.e += 1;
        }
        self.s = self.e + 1;
        let closing = match c {
            '[' => ']',
            '<' => '>',
            _ => unreachable!(),
        };
        while let Some(c) = self.chars.next() {
            self.e += 1;
            if c == closing {
                break;
            }
        }
        if self.e > self.usage.len() {
            return None;
        }

        let name = &self.usage[self.s..self.e];

        Some(UsageToken::Name(name,
                              if c == '<' {
                                  Some(true)
                              } else {
                                  None
                              }))
    }

    fn help(&mut self) -> Option<UsageToken<'u>> {
        self.s = self.e + 2;
        self.e = self.usage.len() - 1;

        while let Some(_) = self.chars.next() {
            continue;
        }

        Some(UsageToken::Help(&self.usage[self.s..self.e]))
    }

    fn long_arg(&mut self) -> Option<UsageToken<'u>> {
        if self.e != 1 {
            self.e += 1;
        }

        self.s = self.e + 1;

        while let Some(c) = self.chars.next() {
            self.e += 1;
            if c == ' ' || c == '=' || c == '.' {
                break;
            }
        }

        if self.e > self.usage.len() {
            return None;
        } else if self.e == self.usage.len() - 1 {
            return Some(UsageToken::Long(&self.usage[self.s..]));
        }

        Some(UsageToken::Long(&self.usage[self.s..self.e]))
    }

    fn short_arg(&mut self, c: char) -> Option<UsageToken<'u>> {
        // When short is first don't increment e
        if self.e != 1 {
            self.e += 1;
        }
        if !c.is_alphanumeric() {
            return None;
        }
        Some(UsageToken::Short(c))
    }

    fn multiple(&mut self) -> bool {
        self.e += 1;
        let mut mult = false;
        for _ in 0..2 {
            self.e += 1;
            match self.chars.next() {
                // longs consume one '.' so they match '.. ' whereas shorts can
                // match '...'
                Some('.') | Some(' ') => {
                    mult = true;
                }
                _ => {
                    // if there is no help or following space all we can match is '..'
                    if self.e == self.usage.len() - 1 {
                        mult = true;
                    }
                    break;
                }
            }
        }

        mult
    }
}

impl<'u> Iterator for UsageParser<'u> {
    type Item = UsageToken<'u>;

    fn next(&mut self) -> Option<UsageToken<'u>> {
        loop {
            match self.chars.next() {
                Some(c) if c == '[' || c == '<' => {
                    return self.name(c);
                }
                Some('\'') => {
                    return self.help();
                }
                Some('-') => {
                    self.e += 1;
                    match self.chars.next() {
                        Some('-') => {
                            return self.long_arg();
                        }
                        Some(c) => {
                            return self.short_arg(c);
                        }
                        _ => {
                            return None;
                        }
                    }
                }
                Some('.') => {
                    if self.multiple() {
                        return Some(UsageToken::Multiple);
                    }
                }
                Some(' ') | Some('=') | Some(']') | Some('>') | Some('\t') | Some(',') => {
                    self.e += 1;
                    continue;
                }
                None => {
                    return None;
                }
                Some(c) => panic!("Usage parser error, unexpected \
                                  \"{}\" at \"{}\", check from_usage call",
                                  c,
                                  self.usage),
            }
        }
    }
}
