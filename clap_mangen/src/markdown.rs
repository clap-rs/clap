use pulldown_cmark::{Event, Parser, Tag, TagEnd};
use roff::{Inline, Roff, bold, italic, roman};

/// Render a markdown string as inline roff elements.
///
/// Processes inline markdown formatting and returns a `Vec<Inline>`.
/// Used in contexts where the caller is composing a single text line
/// (e.g. option help, about strings).
///
/// # Supported formatting
///
/// - `**bold**` and `__bold__` → `Bold`
/// - `*italic*` and `_italic_` → `Italic`
/// - `` `code` `` → `Bold` (man page convention: literals are bold)
/// - `[text](url)` → `text <url>` in roman
/// - `<url>` (autolinks) → `url` (no duplication)
///
/// Block-level elements (headings, lists, code blocks) are flattened:
/// their text content is preserved but structural formatting is dropped.
pub(crate) fn to_roff_inline(text: &str) -> Vec<Inline> {
    let parser = Parser::new(text);
    let mut inlines = Vec::new();
    let mut style_stack: Vec<InlineStyle> = Vec::new();
    let mut link_url: Option<String> = None;
    let mut link_text = String::new();

    for event in parser {
        match event {
            Event::Text(t) => {
                if link_url.is_some() {
                    link_text.push_str(&t);
                } else {
                    push_styled(&mut inlines, &t, &style_stack);
                }
            }
            Event::Code(code) => {
                // Man page convention: code/literals are bold.
                if link_url.is_some() {
                    link_text.push_str(&code);
                } else {
                    inlines.push(bold(code.to_string()));
                }
            }
            Event::Start(Tag::Strong) => style_stack.push(InlineStyle::Bold),
            Event::End(TagEnd::Strong) => {
                style_stack.pop();
            }
            Event::Start(Tag::Emphasis) => style_stack.push(InlineStyle::Italic),
            Event::End(TagEnd::Emphasis) => {
                style_stack.pop();
            }
            Event::Start(Tag::Link { dest_url, .. }) => {
                link_url = Some(dest_url.to_string());
                link_text.clear();
            }
            Event::End(TagEnd::Link) => {
                if let Some(url) = link_url.take() {
                    if link_text == url || link_text.is_empty() {
                        // Autolink or empty label — just show URL.
                        inlines.push(roman(url));
                    } else {
                        // Labeled link: text <url>
                        inlines.push(roman(format!("{link_text} <{url}>")));
                    }
                    link_text.clear();
                }
            }
            // Headings flattened to bold in inline context.
            Event::Start(Tag::Heading { .. }) => {
                style_stack.push(InlineStyle::Bold);
            }
            Event::End(TagEnd::Heading(_)) => {
                style_stack.pop();
            }
            // Paragraph boundaries become spaces in inline context.
            Event::End(TagEnd::Paragraph) => {
                inlines.push(roman(" "));
            }
            Event::SoftBreak => {
                if link_url.is_some() {
                    link_text.push(' ');
                } else {
                    inlines.push(roman(" "));
                }
            }
            Event::HardBreak => {
                inlines.push(Inline::LineBreak);
            }
            Event::Html(html) | Event::InlineHtml(html) => {
                let text = strip_html_tags(&html);
                if !text.is_empty() {
                    push_styled(&mut inlines, &text, &style_stack);
                }
            }
            // All other events: ignore structure, text content is captured
            // by the Text/Code arms above.
            _ => {}
        }
    }

    // Trim trailing space from paragraph end.
    if let Some(Inline::Roman(s)) = inlines.last() {
        if s == " " {
            inlines.pop();
        }
    }

    if inlines.is_empty() {
        inlines.push(roman(text));
    }

    inlines
}

/// Inline style context for the style stack.
enum InlineStyle {
    Bold,
    Italic,
}

/// Push text with the current style from the stack.
/// The innermost (most recent) style wins.
fn push_styled(inlines: &mut Vec<Inline>, text: &str, stack: &[InlineStyle]) {
    let inline = match stack.last() {
        Some(InlineStyle::Bold) => bold(text),
        Some(InlineStyle::Italic) => italic(text),
        None => roman(text),
    };
    inlines.push(inline);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn inline_plain_text() {
        let result = to_roff_inline("hello world");
        assert_eq!(result, vec![roman("hello world")]);
    }

    #[test]
    fn inline_bold() {
        let result = to_roff_inline("use **this** flag");
        assert_eq!(result, vec![roman("use "), bold("this"), roman(" flag")]);
    }

    #[test]
    fn inline_italic() {
        let result = to_roff_inline("the *value* argument");
        assert_eq!(
            result,
            vec![roman("the "), italic("value"), roman(" argument")]
        );
    }

    #[test]
    fn inline_code_is_bold() {
        let result = to_roff_inline("see `--verbose` for details");
        assert_eq!(
            result,
            vec![roman("see "), bold("--verbose"), roman(" for details")]
        );
    }

    #[test]
    fn inline_link_with_label() {
        let result = to_roff_inline("see [the docs](https://example.com)");
        assert_eq!(
            result,
            vec![roman("see "), roman("the docs <https://example.com>")]
        );
    }

    #[test]
    fn inline_autolink() {
        let result = to_roff_inline("visit <https://example.com>");
        assert_eq!(result, vec![roman("visit "), roman("https://example.com")]);
    }

    #[test]
    fn inline_nested_bold_in_italic() {
        let result = to_roff_inline("*this is **important***");
        assert_eq!(result, vec![italic("this is "), bold("important")]);
    }

    #[test]
    fn inline_heading_becomes_bold() {
        let result = to_roff_inline("# My Heading");
        assert_eq!(result, vec![bold("My Heading")]);
    }

    #[test]
    fn inline_strips_block_structure() {
        let result = to_roff_inline("first\n\nsecond");
        assert_eq!(result, vec![roman("first"), roman(" "), roman("second")]);
    }

    #[test]
    fn inline_consecutive_code_spans() {
        let result = to_roff_inline("`a` `b` `c`");
        assert_eq!(
            result,
            vec![bold("a"), roman(" "), bold("b"), roman(" "), bold("c")]
        );
    }

    #[test]
    fn inline_unclosed_bold() {
        let result = to_roff_inline("use **bold but never close");
        assert!(
            !result.iter().any(|i| matches!(i, Inline::Bold(_))),
            "unclosed bold should not produce Bold inlines: {result:?}"
        );
    }

    #[test]
    fn inline_html_preserves_text() {
        let result = to_roff_inline("use <b>bold</b> text");
        let has_bold_text = result.iter().any(|i| match i {
            Inline::Roman(s) => s.contains("bold"),
            _ => false,
        });
        assert!(
            has_bold_text,
            "HTML text content should be preserved: {result:?}"
        );
    }

    #[test]
    fn inline_emoji() {
        let result = to_roff_inline("Use 🚀 to launch");
        assert_eq!(result, vec![roman("Use 🚀 to launch")]);
    }

    #[test]
    fn inline_emoji_in_bold() {
        let result = to_roff_inline("**🔥 hot** stuff");
        assert_eq!(result, vec![bold("🔥 hot"), roman(" stuff")]);
    }

    #[test]
    fn inline_emoji_in_code() {
        let result = to_roff_inline("run `🎉 party`");
        assert_eq!(result, vec![roman("run "), bold("🎉 party")]);
    }

    #[test]
    fn inline_zwj_emoji() {
        let result = to_roff_inline("family: 👨\u{200d}👩\u{200d}👧\u{200d}👦");
        assert_eq!(
            result,
            vec![roman("family: 👨\u{200d}👩\u{200d}👧\u{200d}👦")]
        );
    }
}
