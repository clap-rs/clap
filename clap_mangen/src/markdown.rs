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

/// Render a markdown string to roff, emitting both block-level structure
/// and inline formatting.
///
/// Used in contexts where the function owns the roff output: `description`,
/// `after_help`, `subcommand` about text.
///
/// # Block-level mappings
///
/// - Paragraphs → `.PP`
/// - Headings (all levels) → `.SS` (caller owns the `.SH` context)
/// - Unordered lists → `.IP \(bu 2`
/// - Ordered lists → `.IP N. 4`
/// - Code blocks → `.nf` / `.fi`
/// - Blockquotes → `.RS` / `.RE`
/// - Horizontal rules → `.PP`
/// - Nested lists (1 level) → `.RS` / `.RE`
///
/// # Inline mappings
///
/// Same as [`to_roff_inline`]: bold, italic, code (bold), links.
///
/// # Unsupported elements
///
/// Tables, images, HTML, footnotes, and math are rendered as plain text
/// with structural markers stripped — no information is lost.
pub(crate) fn to_roff(text: &str, roff: &mut Roff) {
    let parser = Parser::new(text);
    let mut inline_buf: Vec<Inline> = Vec::new();
    let mut style_stack: Vec<InlineStyle> = Vec::new();
    let mut list_stack: Vec<ListContext> = Vec::new();
    let mut in_code_block = false;
    let mut heading_buf = String::new();
    let mut in_heading = false;
    let mut link_url: Option<String> = None;
    let mut link_text = String::new();
    let mut need_pp = false;

    for event in parser {
        match event {
            // --- Block-level events ---
            Event::Start(Tag::Paragraph) if need_pp => {
                roff.control("PP", []);
                need_pp = false;
            }
            Event::End(TagEnd::Paragraph) => {
                if !inline_buf.is_empty() {
                    roff.text(std::mem::take(&mut inline_buf));
                }
                need_pp = true;
            }
            Event::Start(Tag::Heading { .. }) => {
                in_heading = true;
                heading_buf.clear();
            }
            Event::End(TagEnd::Heading(_)) => {
                in_heading = false;
                // All heading levels → .SS (caller owns the .SH).
                roff.control("SS", [heading_buf.as_str()]);
                heading_buf.clear();
                need_pp = false;
            }
            Event::Start(Tag::List(start_num)) => {
                // Flush any pending inline content from the parent item
                // before starting a nested list. In tight lists, pulldown-cmark
                // emits Text events without Paragraph wrappers, so the parent
                // item's text may still be in inline_buf.
                if !inline_buf.is_empty() {
                    roff.text(std::mem::take(&mut inline_buf));
                }
                let depth = list_stack.len();
                if depth > 0 {
                    // Nested list — indent with .RS
                    roff.control("RS", ["4"]);
                }
                list_stack.push(ListContext {
                    ordered: start_num.is_some(),
                    index: start_num.unwrap_or(0),
                });
                need_pp = false;
            }
            Event::End(TagEnd::List(_)) => {
                list_stack.pop();
                if !list_stack.is_empty() {
                    // End of nested list — outdent with .RE
                    roff.control("RE", []);
                }
            }
            Event::Start(Tag::Item) => {
                if let Some(ctx) = list_stack.last_mut() {
                    if ctx.ordered {
                        let marker = format!("{}.", ctx.index);
                        roff.control("IP", [&marker, "4"]);
                        ctx.index += 1;
                    } else {
                        roff.control("IP", ["\\(bu", "2"]);
                    }
                }
            }
            Event::End(TagEnd::Item) if !inline_buf.is_empty() => {
                roff.text(std::mem::take(&mut inline_buf));
            }
            Event::Start(Tag::CodeBlock(_)) => {
                in_code_block = true;
                roff.control("nf", []);
                need_pp = false;
            }
            Event::End(TagEnd::CodeBlock) => {
                in_code_block = false;
                roff.control("fi", []);
            }
            Event::Start(Tag::BlockQuote(_)) => {
                roff.control("RS", ["4"]);
                need_pp = false;
            }
            Event::End(TagEnd::BlockQuote(_)) => {
                if !inline_buf.is_empty() {
                    roff.text(std::mem::take(&mut inline_buf));
                }
                roff.control("RE", []);
            }
            Event::Rule => {
                roff.control("PP", []);
                need_pp = false;
            }

            // --- Inline events ---
            Event::Text(t) => {
                if in_heading {
                    heading_buf.push_str(&t);
                } else if in_code_block {
                    roff.text([roman(t.to_string())]);
                } else if link_url.is_some() {
                    link_text.push_str(&t);
                } else {
                    push_styled(&mut inline_buf, &t, &style_stack);
                }
            }
            Event::Code(code) => {
                if in_heading {
                    heading_buf.push_str(&code);
                } else if link_url.is_some() {
                    link_text.push_str(&code);
                } else {
                    inline_buf.push(bold(code.to_string()));
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
                        inline_buf.push(roman(url));
                    } else {
                        inline_buf.push(roman(format!("{link_text} <{url}>")));
                    }
                    link_text.clear();
                }
            }
            Event::SoftBreak => {
                if link_url.is_some() {
                    link_text.push(' ');
                } else {
                    inline_buf.push(roman(" "));
                }
            }
            Event::HardBreak => {
                inline_buf.push(Inline::LineBreak);
            }
            // HTML blocks and inline HTML: strip tags, preserve text content.
            // pulldown-cmark emits the raw HTML as a single string — we render
            // it as roman text so the content isn't silently lost.
            Event::Html(html) | Event::InlineHtml(html) => {
                let text = strip_html_tags(&html);
                if !text.is_empty() {
                    if in_heading {
                        heading_buf.push_str(&text);
                    } else {
                        inline_buf.push(roman(text));
                    }
                }
            }
            // All other events (tables, images, etc.): text content
            // is captured by the Text arm; structure is ignored.
            _ => {}
        }
    }

    // Flush any remaining inline content.
    if !inline_buf.is_empty() {
        roff.text(std::mem::take(&mut inline_buf));
    }
}

struct ListContext {
    ordered: bool,
    index: u64,
}

/// Strip HTML tags from a string, preserving only text content.
///
/// This is a simple state-machine approach — not a full HTML parser.
/// Sufficient for the common cases in help text (e.g. `<b>text</b>`,
/// `<br>`, `<div>content</div>`).
fn strip_html_tags(html: &str) -> String {
    let mut result = String::new();
    let mut in_tag = false;
    for ch in html.chars() {
        match ch {
            '<' => in_tag = true,
            '>' => in_tag = false,
            _ if !in_tag => result.push(ch),
            _ => {}
        }
    }
    result.trim().to_owned()
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

    #[test]
    fn block_paragraphs() {
        let mut roff = Roff::default();
        to_roff("first paragraph\n\nsecond paragraph", &mut roff);
        let output = roff.to_roff();
        assert_eq!(output, "first paragraph\n.PP\nsecond paragraph\n");
    }

    #[test]
    fn block_heading_becomes_ss() {
        let mut roff = Roff::default();
        to_roff("## Options\n\nSome text", &mut roff);
        let output = roff.to_roff();
        assert!(
            output.starts_with(".SS"),
            "expected .SS heading in: {output}"
        );
        assert!(
            output.contains("Options"),
            "expected heading text in: {output}"
        );
    }

    #[test]
    fn block_heading_all_levels_are_ss() {
        for level in ["#", "##", "###", "####"] {
            let mut roff = Roff::default();
            to_roff(&format!("{level} Title"), &mut roff);
            let output = roff.to_roff();
            assert!(
                output.contains(".SS"),
                "heading '{level}' should produce .SS, got: {output}"
            );
        }
    }

    #[test]
    fn block_heading_with_code() {
        let mut roff = Roff::default();
        to_roff("## The `--flag` option\n\nDetails here.", &mut roff);
        let output = roff.to_roff();
        assert!(output.contains(".SS"), "expected .SS in: {output}");
        assert!(
            output.contains("--flag"),
            "code text should be in heading: {output}"
        );
    }

    #[test]
    fn block_inline_formatting_in_paragraph() {
        let mut roff = Roff::default();
        to_roff("Use **bold** and *italic* and `code`", &mut roff);
        let output = roff.to_roff();
        assert!(output.contains("\\fB"), "expected bold escape in: {output}");
        assert!(
            output.contains("\\fI"),
            "expected italic escape in: {output}"
        );
        assert!(
            !output.contains("**"),
            "markdown ** should not appear in: {output}"
        );
    }

    #[test]
    fn block_unordered_list() {
        let mut roff = Roff::default();
        to_roff("- first\n- second\n- third", &mut roff);
        let output = roff.to_roff();
        assert!(
            output.contains(".IP \\(bu 2"),
            "expected bullet point in: {output}"
        );
        assert!(output.contains("first"), "expected 'first' in: {output}");
        assert!(output.contains("second"), "expected 'second' in: {output}");
    }

    #[test]
    fn block_ordered_list() {
        let mut roff = Roff::default();
        to_roff("1. first\n2. second\n3. third", &mut roff);
        let output = roff.to_roff();
        assert!(output.contains("1."), "expected '1.' in: {output}");
        assert!(output.contains("2."), "expected '2.' in: {output}");
    }

    #[test]
    fn block_code_block() {
        let mut roff = Roff::default();
        to_roff("```\nlet x = 1;\nlet y = 2;\n```", &mut roff);
        let output = roff.to_roff();
        assert!(output.contains(".nf"), "expected .nf in: {output}");
        assert!(output.contains(".fi"), "expected .fi in: {output}");
        assert!(output.contains("let x = 1;"), "expected code in: {output}");
    }

    #[test]
    fn block_code_block_preserves_blank_lines() {
        let mut roff = Roff::default();
        to_roff("```\nline1\n\nline3\n```", &mut roff);
        let output = roff.to_roff();
        assert!(
            output.contains("line1\n\nline3"),
            "blank line should be preserved in code block: {output}"
        );
    }

    #[test]
    fn block_blockquote() {
        let mut roff = Roff::default();
        to_roff("> quoted text", &mut roff);
        let output = roff.to_roff();
        assert!(output.contains(".RS"), "expected .RS in: {output}");
        assert!(output.contains(".RE"), "expected .RE in: {output}");
        assert!(output.contains("quoted text"), "expected text in: {output}");
    }

    #[test]
    fn block_nested_list() {
        let mut roff = Roff::default();
        to_roff("- outer\n  - inner\n- outer2", &mut roff);
        let output = roff.to_roff();
        assert!(
            output.contains(".RS"),
            "expected .RS for nesting in: {output}"
        );
        assert!(output.contains("inner"), "expected 'inner' in: {output}");
    }

    #[test]
    fn block_tight_nested_list() {
        let mut roff = Roff::default();
        to_roff("- parent\n  - child1\n  - child2", &mut roff);
        let output = roff.to_roff();
        assert!(
            !output.contains("parentchild"),
            "parent and child text should not merge: {output}"
        );
        assert!(output.contains("parent"), "expected 'parent' in: {output}");
        assert!(output.contains("child1"), "expected 'child1' in: {output}");
    }

    #[test]
    fn block_deeply_nested_list() {
        let mut roff = Roff::default();
        to_roff("- a\n  - b\n    - c\n      - d", &mut roff);
        let output = roff.to_roff();
        assert!(
            !output.contains("abcd"),
            "item text should not be concatenated: {output}"
        );
        let a_pos = output.find('a').expect("should contain 'a'");
        let rs_pos = output.find(".RS").expect("should contain .RS");
        assert!(
            a_pos < rs_pos,
            "'a' should appear before first .RS in: {output}"
        );
    }

    #[test]
    fn block_list_item_multi_paragraph() {
        let mut roff = Roff::default();
        to_roff(
            "- first para\n\n  second para in same item\n\n- next item",
            &mut roff,
        );
        let output = roff.to_roff();
        assert!(
            output.contains("first para"),
            "expected first para in: {output}"
        );
        assert!(
            output.contains("second para"),
            "expected second para in: {output}"
        );
    }

    #[test]
    fn block_list_after_paragraph() {
        let mut roff = Roff::default();
        to_roff("Available modes:\n\n- fast\n- slow\n- auto", &mut roff);
        let output = roff.to_roff();
        assert!(
            output.contains("Available modes:"),
            "paragraph in: {output}"
        );
        assert!(output.contains(".IP"), "list should have .IP in: {output}");
    }

    #[test]
    fn block_rule_becomes_pp() {
        let mut roff = Roff::default();
        to_roff("before\n\n---\n\nafter", &mut roff);
        let output = roff.to_roff();
        assert!(output.contains("before"), "expected 'before' in: {output}");
        assert!(output.contains("after"), "expected 'after' in: {output}");
    }

    #[test]
    fn block_link_in_paragraph() {
        let mut roff = Roff::default();
        to_roff("See [docs](https://example.com) for info", &mut roff);
        let output = roff.to_roff();
        assert!(output.contains("docs"), "expected link text in: {output}");
        assert!(
            output.contains("https://example.com"),
            "expected URL in: {output}"
        );
    }

    #[test]
    fn block_html_preserves_text() {
        let mut roff = Roff::default();
        to_roff("<div>some html</div>", &mut roff);
        let output = roff.to_roff();
        assert!(
            output.contains("some html"),
            "HTML block text content should be preserved: '{output}'"
        );
    }

    #[test]
    fn block_html_br() {
        let mut roff = Roff::default();
        to_roff("before\n\n<br>\n\nafter", &mut roff);
        let output = roff.to_roff();
        assert!(output.contains("before"), "expected 'before' in: {output}");
        assert!(output.contains("after"), "expected 'after' in: {output}");
    }

    #[test]
    fn block_empty() {
        let mut roff = Roff::default();
        to_roff("", &mut roff);
        let output = roff.to_roff();
        assert_eq!(output, "");
    }

    #[test]
    fn block_emoji() {
        let mut roff = Roff::default();
        to_roff("## 🎯 Goals\n\n- 🚀 Fast\n- 🔒 Secure", &mut roff);
        let output = roff.to_roff();
        assert!(output.contains("🎯 Goals"), "emoji heading in: {output}");
        assert!(output.contains("🚀 Fast"), "emoji list item in: {output}");
    }

    #[test]
    fn block_dot_prefix_escaped() {
        let mut roff = Roff::default();
        to_roff(".SH MALICIOUS", &mut roff);
        let output = roff.to_roff();
        assert!(
            output.contains("\\&.SH"),
            "leading dot should be escaped in: {output}"
        );
    }

    #[test]
    fn block_code_block_dot_prefix_escaped() {
        let mut roff = Roff::default();
        to_roff("```\n.SH INJECTED\n.PP\n```", &mut roff);
        let output = roff.to_roff();
        assert!(
            output.contains("\\&.SH"),
            "dots in code blocks should be escaped in: {output}"
        );
    }

    #[test]
    fn block_roff_escape_sequences_not_interpreted() {
        let mut roff = Roff::default();
        to_roff(r"contains \fB fake bold \fR markers", &mut roff);
        let output = roff.to_roff();
        assert!(
            output.contains("\\\\fB"),
            "backslash should be escaped in: {output}"
        );
    }
}
