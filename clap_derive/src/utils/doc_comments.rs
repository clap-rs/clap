//! The preprocessing we apply to doc comments.
//!
//! #[derive(Parser)] works in terms of "paragraphs". Paragraph is a sequence of
//! non-empty adjacent lines, delimited by sequences of blank (whitespace only) lines.

use crate::attrs::Method;

use quote::{format_ident, quote};
use std::iter;

pub fn process_doc_comment(lines: Vec<String>, name: &str, preprocess: bool) -> Vec<Method> {
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
        return vec![];
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

        vec![
            Method::new(short_name, quote!(#short)),
            Method::new(long_name, quote!(#long)),
        ]
    } else {
        let short = if preprocess {
            let s = process_paragraph(&lines);
            remove_period(s)
        } else {
            lines.join("\n")
        };

        vec![
            Method::new(short_name, quote!(#short)),
            Method::new(long_name, quote!(None)),
        ]
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
            .unwrap_or_else(|| slice.len());

        last_line += start + len;

        if len != 0 {
            Some(process_paragraph(&slice[..len]))
        } else {
            None
        }
    })
    .collect()
}

fn process_paragraph(lines: &[&str]) -> String {
    lines
        .iter()
        .map(|ln| ln.trim())
         // Ignore empty lines even if they were escaped with slash
        .filter(|ln| ln != &"\\" && !ln.is_empty())
        // Run actual slash escaping
        .map(|ln| {
            let expected_len = ln.len() + 1;
            let mut chars = ln.chars();
            let mut ln = String::with_capacity(expected_len);
            while let Some(c) = chars.next() {
                if c == '\\' {
                     match chars.next() {
                        // Escape slash with slash
                        Some('\\') => {
                            ln.push('\\')
                        },
                        // Don't escape other letters
                        Some(x) => {
                            ln.push('\\');
                            ln.push(x);
                        },
                        // Escape newline
                        None => {
                            // Remove whitespace so it wouldn't
                            // mess with terminal wrapping
                            ln = trim_end(ln);
                            ln.push('\n');
                            return ln
                        },
                    }
                } else {
                    ln.push(c);
                }
            }
            // Since every line is trimmed we need
            // this space to avoid gluing words together
            ln.push(' ');
            ln
        })
        .collect()
}

fn trim_end(mut s: String) -> String {
    let while_text = s.trim_end_matches(|c: char| c.is_ascii_whitespace()).len();
    s.truncate(while_text);
    s
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
