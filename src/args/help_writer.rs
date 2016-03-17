use std::io;
use std::fmt::Display;

use args::AnyArg;
use args::settings::ArgSettings;
use term;

const TAB: &'static str = "    ";

pub struct HelpWriter<'a, A> where A: 'a {
    a: &'a A,
    l: usize,
    nlh: bool,
    pub skip_pv: bool,
    term_w: Option<usize>,
}

impl<'a, 'n, 'e, A> HelpWriter<'a, A> where A: AnyArg<'n, 'e> + Display {
    pub fn new(a: &'a A, l: usize, nlh: bool) -> Self {
        HelpWriter {
            a: a,
            l: l,
            nlh: nlh,
            skip_pv: false,
            term_w: term::dimensions().map(|(w, _)| w),
        }
    }
    pub fn write_to<W: io::Write>(&self, w: &mut W) -> io::Result<()> {
        debugln!("fn=write_to;");
        try!(self.short(w));
        try!(self.long(w));
        try!(self.val(w));
        try!(self.help(w));
        write!(w, "\n")
    }

    fn short<W>(&self, w: &mut W) -> io::Result<()>
        where W: io::Write
    {
        debugln!("fn=short;");
        try!(write!(w, "{}", TAB));
        if let Some(s) = self.a.short() {
            write!(w, "-{}", s)
        } else if self.a.has_switch() {
            write!(w, "{}", TAB)
        } else {
            Ok(())
        }
    }

    fn long<W>(&self, w: &mut W) -> io::Result<()>
        where W: io::Write
    {
        debugln!("fn=long;");
        if !self.a.has_switch() {
            return Ok(());
        }
        if self.a.takes_value() {
            if let Some(l) = self.a.long() {
                try!(write!(w, "{}--{}", if self.a.short().is_some() { ", " } else { "" }, l));
            }
            try!(write!(w, " "));
        } else {
            if let Some(l) = self.a.long() {
                try!(write!(w, "{}--{}", if self.a.short().is_some() { ", " } else { "" }, l));
                if !self.nlh || !self.a.is_set(ArgSettings::NextLineHelp) {
                    write_spaces!((self.l + 4) - (l.len() + 2), w);
                }
            } else {
                if !self.nlh || !self.a.is_set(ArgSettings::NextLineHelp) {
                    // 6 is tab (4) + -- (2)
                    write_spaces!((self.l + 6), w);
                }
            }
        }
        Ok(())
    }

    fn val<W>(&self, w: &mut W) -> io::Result<()>
        where W: io::Write
    {
        debugln!("fn=val;");
        if !self.a.takes_value() {
            return Ok(());
        }
        if let Some(ref vec) = self.a.val_names() {
            let mut it = vec.iter().peekable();
            while let Some((_, val)) = it.next() {
                try!(write!(w, "<{}>", val));
                if it.peek().is_some() { try!(write!(w, " ")); }
            }
            let num = vec.len();
            if self.a.is_set(ArgSettings::Multiple) && num == 1 {
                try!(write!(w, "..."));
            }
        } else if let Some(num) = self.a.num_vals() {
            let mut it = (0..num).peekable();
            while let Some(_) = it.next() {
                try!(write!(w, "<{}>", self.a.name()));
                if it.peek().is_some() { try!(write!(w, " ")); }
            }
        } else {
            try!(write!(w, "{}", self.a));
        }
        if self.a.has_switch() {
            if !(self.nlh || self.a.is_set(ArgSettings::NextLineHelp)) {
                let self_len = self.a.to_string().len();
                // subtract ourself
                let mut spcs = self.l - self_len;
                // Since we're writing spaces from the tab point we first need to know if we
                // had a long and short, or just short
                if self.a.long().is_some() {
                    // Only account 4 after the val
                    spcs += 4;
                } else {
                    // Only account for ', --' + 4 after the val
                    spcs += 8;
                }
                write_spaces!(spcs, w);
            }
        } else {
            if !(self.nlh || self.a.is_set(ArgSettings::NextLineHelp)) {
                write_spaces!(self.l + 4 - (self.a.to_string().len()), w);
            }
        }
        Ok(())
    }

    fn help<W>(&self, w: &mut W) -> io::Result<()>
        where W: io::Write
    {
        debugln!("fn=help;");
        let spec_vals = self.spec_vals();
        let mut help = String::new();
        let h = self.a.help().unwrap_or("");
        let spcs = if self.nlh || self.a.is_set(ArgSettings::NextLineHelp) {
            8 // "tab" + "tab"
        } else {
            self.l + 12
        };
        // determine if our help fits or needs to wrap
        let width = self.term_w.unwrap_or(0);
        debugln!("Term width...{}", width);
        let too_long = self.term_w.is_some() && (spcs + h.len() + spec_vals.len() >= width);
        debugln!("Too long...{:?}", too_long);

        // Is help on next line, if so newline + 2x tab
        if self.nlh || self.a.is_set(ArgSettings::NextLineHelp) {
            try!(write!(w, "\n{}{}", TAB, TAB));
        }

        debug!("Too long...");
        if too_long {
            sdebugln!("Yes");
            help.push_str(h);
            help.push_str(&*spec_vals);
            debugln!("help: {}", help);
            debugln!("help len: {}", help.len());
            // Determine how many newlines we need to insert
            let avail_chars = width - spcs;
            debugln!("Usable space: {}", avail_chars);
            let longest_w = {
                let mut lw = 0;
                for l in help.split(' ').map(|s| s.len()) {
                    if l > lw {
                        lw = l;
                    }
                }
                lw
            };
            debugln!("Longest word...{}", longest_w);
            debug!("Enough space...");
            if longest_w < avail_chars {
                sdebugln!("Yes");
                let mut indices = vec![];
                let mut idx = 0;
                loop {
                    idx += avail_chars - 1;
                    if idx >= help.len() { break; }
                    // 'a' arbitrary non space char
                    if help.chars().nth(idx).unwrap_or('a') != ' ' {
                        idx = find_idx_of_space(&*help, idx);
                    }
                    debugln!("Adding idx: {}", idx);
                    debugln!("At {}: {:?}", idx, help.chars().nth(idx));
                    indices.push(idx);
                    if &help[idx..].len() <= &avail_chars {
                        break;
                    }
                }
                for (i, idx) in indices.iter().enumerate() {
                    debugln!("iter;i={},idx={}", i, idx);
                    let j = idx+(2*i);
                    debugln!("removing: {}", j);
                    debugln!("at {}: {:?}", j, help.chars().nth(j));
                    help.remove(j);
                    help.insert(j, '{');
                    help.insert(j + 1 , 'n');
                    help.insert(j + 2, '}');
                }
            } else {
                sdebugln!("No");
            }
        } else { sdebugln!("No"); }
        let help = if !help.is_empty() {
            &*help
        } else if !spec_vals.is_empty() {
            help.push_str(h);
            help.push_str(&*spec_vals);
            &*help
        } else {
            h
        };
        if help.contains("{n}") {
            if let Some(part) = help.split("{n}").next() {
                try!(write!(w, "{}", part));
            }
            for part in help.split("{n}").skip(1) {
                try!(write!(w, "\n"));
                if self.nlh || self.a.is_set(ArgSettings::NextLineHelp) {
                    try!(write!(w, "{}{}", TAB, TAB));
                } else {
                    if self.a.has_switch() {
                        write_spaces!(self.l + 12, w);
                    } else {
                        write_spaces!(self.l + 8, w);
                    }
                }
                try!(write!(w, "{}", part));
            }
        } else {
            try!(write!(w, "{}", help));
        }
        Ok(())
    }

    fn spec_vals(&self) -> String {
        debugln!("fn=spec_vals;");
        if let Some(ref pv) = self.a.default_val() {
            debugln!("Writing defaults");
            return format!(" [default: {}] {}", pv,
                if !self.skip_pv {
                    if let Some(ref pv) = self.a.possible_vals() {
                        format!(" [values: {}]", pv.join(", "))
                    } else { "".into() }
                } else { "".into() }
            );
        } else if !self.skip_pv {
            debugln!("Writing values");
            if let Some(ref pv) = self.a.possible_vals() {
                debugln!("Possible vals...{:?}", pv);
                return format!(" [values: {}]", pv.join(", "));
            }
        }
        String::new()
    }
}

fn find_idx_of_space(full: &str, start: usize) -> usize {
    debugln!("fn=find_idx_of_space;");
    let haystack = &full[..start];
    debugln!("haystack: {}", haystack);
    for (i, c) in haystack.chars().rev().enumerate() {
        debugln!("iter;c={},i={}", c, i);
        if c == ' ' {
            debugln!("Found space returning start-i...{}", start - (i+1));
            return start - (i+1);
        }
    }
    0
}
