//! The preprocessing we apply to doc comments.
//!
//! #[derive(Parser)] works in terms of "paragraphs". Paragraph is a sequence of
//! non-empty adjacent lines, delimited by sequences of blank (whitespace only) lines.

use crate::item::Method;

use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use std::iter;

pub fn process_md_doc_comment(lines: Vec<String>, name: &str) -> (Option<Method>, Option<Method>) {
    use pulldown_cmark::{Event, Parser, Tag};

    let text = lines
        .iter()
        .skip_while(|s| is_blank(s))
        .flat_map(|s| s.split('\n'))
        .map(|l| if l.starts_with(' ') { &l[1..] } else { l })
        .collect::<Vec<&str>>()
        .join("\n");

    let mut short = TokenStream::new();
    let mut long = TokenStream::new();
    let mut man = TokenStream::new();

    let parser = Parser::new(&text);

    // ordered list of parent blocks where we're currently parsing
    let mut blocking = Vec::new();
    // ordered list of inline features currently active where we're parsing
    let mut inliners = Vec::new();

    #[derive(PartialEq)]
    enum State {
        Short,
        Long,
        Man,
    }

    let mut state = State::Short;

    for def in parser {
        let chunk = match def {
            Event::Start(tag) => {
                match &tag {
                    Tag::Paragraph => blocking.push(tag),
                    Tag::Heading(_level, _fragment, _classes) => todo!("heading"), //blocking.push(tag),
                    Tag::BlockQuote => todo!("blockquote"), //blocking.push(tag),
                    Tag::CodeBlock(_kind) => todo!("codeblock"), //blocking.push(tag),
                    Tag::List(_start) => todo!("list"),     //blocking.push(tag),
                    Tag::Item => todo!("item"),             //blocking.push(tag),
                    Tag::FootnoteDefinition(_label) => todo!("footnote"), //blocking.push(tag),
                    Tag::Table(_alignment) => todo!("table"), //blocking.push(tag),
                    Tag::TableHead => todo!("tablehead"),   //blocking.push(tag),
                    Tag::TableRow => todo!("tablerow"),     //blocking.push(tag),
                    Tag::TableCell => todo!("tablecell"),   //blocking.push(tag),
                    Tag::Emphasis => inliners.push(tag),
                    Tag::Strong => inliners.push(tag),
                    Tag::Strikethrough => todo!("strike"), //inliners.push(tag),
                    Tag::Link(_type, _url, _title) => {}   //todo!("link"), //inliners.push(tag),
                    Tag::Image(_type, _url, _title) => todo!("image"), //inliners.push(tag),
                };
                None
            }
            Event::Text(t) => {
                let t = t.as_ref();
                // StyledStr can only define a single style, just take last inline container
                match inliners.last() {
                    None => Some(quote!(text.none(#t);)),
                    Some(Tag::Strong) => Some(quote!(text.literal(#t);)),
                    Some(Tag::Emphasis) => Some(quote!(text.italic(#t);)),
                    _ => todo!(),
                }
            }
            Event::End(tag) => match &tag {
                // only got twenty dollars in my pocket...
                Tag::Paragraph => {
                    assert_eq!(blocking.pop(), Some(tag));
                    if state == State::Short {
                        state = State::Long;
                    }

                    Some(quote!(text.none("\n\n");))
                }
                Tag::Heading(_level, _fragment, _classes) => {
                    assert_eq!(blocking.pop(), Some(tag));
                    state = State::Man;

                    Some(quote!(text.none("\n\n");))
                }
                Tag::BlockQuote => {
                    assert_eq!(blocking.pop(), Some(tag));
                    None
                }
                Tag::CodeBlock(_kind) => {
                    assert_eq!(blocking.pop(), Some(tag));
                    None
                }
                Tag::List(_start) => {
                    assert_eq!(blocking.pop(), Some(tag));
                    None
                }
                Tag::Item => {
                    assert_eq!(blocking.pop(), Some(tag));
                    None
                }
                Tag::FootnoteDefinition(_label) => {
                    assert_eq!(blocking.pop(), Some(tag));
                    None
                }
                Tag::Table(_alignment) => {
                    assert_eq!(blocking.pop(), Some(tag));
                    None
                }
                Tag::TableHead => {
                    assert_eq!(blocking.pop(), Some(tag));
                    None
                }
                Tag::TableRow => {
                    assert_eq!(blocking.pop(), Some(tag));
                    None
                }
                Tag::TableCell => {
                    assert_eq!(blocking.pop(), Some(tag));
                    None
                }
                Tag::Emphasis => {
                    assert_eq!(inliners.pop(), Some(tag));
                    None
                }
                Tag::Strong => {
                    assert_eq!(inliners.pop(), Some(tag));
                    None
                }
                Tag::Strikethrough => {
                    assert_eq!(inliners.pop(), Some(tag));
                    None
                }
                Tag::Link(_type, _url, _title) => {
                    //assert_eq!(inliners.pop(), Some(tag));
                    None
                }
                Tag::Image(_type, _url, _title) => {
                    assert_eq!(inliners.pop(), Some(tag));
                    None
                }
            },
            Event::Code(t) => {
                let t = t.as_ref();
                Some(quote!(text.code(#t);))
            }
            Event::Html(_) => {
                todo!("drop or panic?")
            }
            Event::FootnoteReference(_) => {
                todo!("can this be handled? just leave the markdown as-is?")
            }
            // single line breaks within paragraphs
            Event::SoftBreak => Some(quote!(text.none(" ");)),
            // double line breaks between paragraphs
            // TODO: peek into the parser to check if there's more content coming to avoid adding
            // blank lines at the end.
            Event::HardBreak => Some(quote!(text.none("\n\n");)),
            Event::Rule => {
                todo!("would need terminal width for this? is there any sort of responsive way to do this?")
            }
            Event::TaskListMarker(checked) => {
                let _marker = if checked { '☑' } else { '☐' };
                None
            }
        };

        if let Some(chunk) = chunk {
            match state {
                State::Short => {
                    short.extend(chunk.clone());
                    long.extend(chunk.clone());
                    man.extend(chunk);
                }
                State::Long => {
                    long.extend(chunk.clone());
                    man.extend(chunk);
                }
                State::Man => {
                    man.extend(chunk);
                }
            }
        }
    }

    let short_name = format_ident!("{}", name);
    let long_name = format_ident!("long_{}", name);

    let short_about = if short.is_empty() {
        None
    } else {
        let text_block = quote! {
            {
                let mut text = clap::builder::StyledStr::new();
                #short
                text
            }
        };
        Some(Method::new(short_name, text_block))
    };

    let long_about = if long.is_empty() {
        None
    } else {
        let text_block = quote! {
            {
                let mut text = clap::builder::StyledStr::new();
                #long
                text
            }
        };
        Some(Method::new(long_name, text_block))
    };

    (short_about, long_about)
}

#[test]
fn md() {
    let inp = r##"
This is the __short__ desciption.

This is the *long* description, it contains an [inline link](rust-lang.org).
"##;
    let _ = r##"

It also contains a [ref link].

# Examples

```
frobulate compular
```

[ref link]: https://github.com/clap-rs/clap

Okay.

    "##;

    // mangle input to match how we'd normally get it
    let lines: Vec<String> = inp.lines().map(|l| format!(" {}", l)).collect();

    let tokens = dbg!(process_md_doc_comment(lines.clone(), "frobulate"));
}

pub fn process_doc_comment(
    lines: Vec<String>,
    name: &str,
    preprocess: bool,
) -> (Option<Method>, Option<Method>) {
    // multiline comments (`/** ... */`) may have LFs (`\n`) in them,
    // we need to split so we could handle the lines correctly
    //
    // we also need to remove leading and trailing blank lines
    let mut lines: Vec<&str> = lines
        .iter()
        .skip_while(|s| is_blank(s))
        .flat_map(|s| s.split('\n'))
        .collect();

    while let Some(true) = lines.last().map(|s| is_blank(s)) {
        lines.pop();
    }

    // remove one leading space no matter what
    for line in lines.iter_mut() {
        if line.starts_with(' ') {
            *line = &line[1..];
        }
    }

    if lines.is_empty() {
        return (None, None);
    }

    let short_name = format_ident!("{}", name);
    let long_name = format_ident!("long_{}", name);

    if let Some(first_blank) = lines.iter().position(|s| is_blank(s)) {
        let (short, long) = if preprocess {
            let paragraphs = split_paragraphs(&lines);
            let short = paragraphs[0].clone();
            let long = paragraphs.join("\n\n");
            (remove_period(short), long)
        } else {
            let short = lines[..first_blank].join("\n");
            let long = lines.join("\n");
            (short, long)
        };

        (
            Some(Method::new(short_name, quote!(#short))),
            Some(Method::new(long_name, quote!(#long))),
        )
    } else {
        let short = if preprocess {
            let s = merge_lines(&lines);
            remove_period(s)
        } else {
            lines.join("\n")
        };

        (
            Some(Method::new(short_name, quote!(#short))),
            Some(Method::new(long_name, quote!(None))),
        )
    }
}

fn split_paragraphs(lines: &[&str]) -> Vec<String> {
    let mut last_line = 0;
    iter::from_fn(|| {
        let slice = &lines[last_line..];
        let start = slice.iter().position(|s| !is_blank(s)).unwrap_or(0);

        let slice = &slice[start..];
        let len = slice
            .iter()
            .position(|s| is_blank(s))
            .unwrap_or(slice.len());

        last_line += start + len;

        if len != 0 {
            Some(merge_lines(&slice[..len]))
        } else {
            None
        }
    })
    .collect()
}

fn remove_period(mut s: String) -> String {
    if s.ends_with('.') && !s.ends_with("..") {
        s.pop();
    }
    s
}

fn is_blank(s: &str) -> bool {
    s.trim().is_empty()
}

fn merge_lines(lines: &[&str]) -> String {
    lines.iter().map(|s| s.trim()).collect::<Vec<_>>().join(" ")
}
