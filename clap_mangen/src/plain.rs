use roff::{Inline, Roff, roman};

/// Render text to roff without any markdown processing.
///
/// Each non-blank line is emitted as a `roman()` text line.
/// Blank lines are emitted as `.PP` paragraph breaks.
///
/// # Example
///
/// Given the input:
///
/// ```text
/// first paragraph
///
/// second paragraph
/// ```
///
/// The output is equivalent to:
///
/// ```text
/// first paragraph
/// .PP
/// second paragraph
/// ```
pub(crate) fn to_roff(text: &str, roff: &mut Roff) {
    for line in text.lines() {
        if line.trim().is_empty() {
            roff.control("PP", []);
        } else {
            roff.text([roman(line)]);
        }
    }
}

/// Render text as inline roff elements without any markdown processing.
///
/// Returns the text wrapped in a single `roman()` inline element.
/// This is used in contexts where the caller is building a `Vec<Inline>`
/// to compose into a larger text line (e.g. option help, about strings).
///
/// # Example
///
/// `"some help text"` becomes `vec![Roman("some help text")]`
pub(crate) fn to_roff_inline(text: &str) -> Vec<Inline> {
    vec![roman(text)]
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn to_roff_single_line() {
        let mut roff = Roff::default();
        to_roff("hello world", &mut roff);
        let output = roff.to_roff();
        assert_eq!(output, "hello world\n");
    }

    #[test]
    fn to_roff_multiple_lines() {
        let mut roff = Roff::default();
        to_roff("line one\nline two", &mut roff);
        let output = roff.to_roff();
        assert_eq!(output, "line one\nline two\n");
    }

    #[test]
    fn to_roff_blank_line_becomes_pp() {
        let mut roff = Roff::default();
        to_roff("first\n\nsecond", &mut roff);
        let output = roff.to_roff();
        assert_eq!(output, "first\n.PP\nsecond\n");
    }

    #[test]
    fn to_roff_empty_input() {
        let mut roff = Roff::default();
        to_roff("", &mut roff);
        let output = roff.to_roff();
        assert_eq!(output, "");
    }

    #[test]
    fn to_roff_inline_wraps_in_roman() {
        let result = to_roff_inline("some text");
        assert_eq!(result, vec![roman("some text")]);
    }

    #[test]
    fn to_roff_inline_empty() {
        let result = to_roff_inline("");
        assert_eq!(result, vec![roman("")]);
    }
}
