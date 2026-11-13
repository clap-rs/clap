/// Compute the display width of `text`
///
/// # Examples
///
/// **Note:** When the `unicode` Cargo feature is disabled, all characters are presumed to take up
/// 1 width.  With the feature enabled, function will correctly deal with [combining characters] in
/// their decomposed form (see [Unicode equivalence]).
///
/// An example of a decomposed character is “é”, which can be decomposed into: “e” followed by a
/// combining acute accent: “◌́”.  Without the `unicode` Cargo feature, every `char` has a width of
/// 1. This includes the combining accent:
///
/// ## Emojis and CJK Characters
///
/// Characters such as emojis and [CJK characters] used in the
/// Chinese, Japanese, and Korean languages are seen as double-width,
/// even if the `unicode-width` feature is disabled:
///
/// # Limitations
///
/// The displayed width of a string cannot always be computed from the
/// string alone. This is because the width depends on the rendering
/// engine used. This is particularly visible with [emoji modifier
/// sequences] where a base emoji is modified with, e.g., skin tone or
/// hair color modifiers. It is up to the rendering engine to detect
/// this and to produce a suitable emoji.
///
/// A simple example is “❤️”, which consists of “❤” (U+2764: Black
/// Heart Symbol) followed by U+FE0F (Variation Selector-16). By
/// itself, “❤” is a black heart, but if you follow it with the
/// variant selector, you may get a wider red heart.
///
/// A more complex example would be “👨‍🦰” which should depict a man
/// with red hair. Here the computed width is too large — and the
/// width differs depending on the use of the `unicode-width` feature:
///
/// This happens because the grapheme consists of three code points:
/// “👨” (U+1F468: Man), Zero Width Joiner (U+200D), and “🦰”
/// (U+1F9B0: Red Hair). You can see them above in the test. With
/// `unicode-width` enabled, the ZWJ is correctly seen as having zero
/// width, without it is counted as a double-width character.
///
/// ## Terminal Support
///
/// Modern browsers typically do a great job at combining characters
/// as shown above, but terminals often struggle more. As an example,
/// Gnome Terminal version 3.38.1, shows “❤️” as a big red heart, but
/// shows "👨‍🦰" as “👨🦰”.
///
/// [combining characters]: https://en.wikipedia.org/wiki/Combining_character
/// [Unicode equivalence]: https://en.wikipedia.org/wiki/Unicode_equivalence
/// [CJK characters]: https://en.wikipedia.org/wiki/CJK_characters
/// [emoji modifier sequences]: https://unicode.org/emoji/charts/full-emoji-modifiers.html
#[inline(never)]
pub(crate) fn display_width(text: &str) -> usize {
    let mut width = 0;

    let mut control_sequence = false;
    let control_terminate: char = 'm';

    for ch in text.chars() {
        if ch.is_ascii_control() {
            control_sequence = true;
        } else if control_sequence && ch == control_terminate {
            control_sequence = false;
            continue;
        }

        if !control_sequence {
            width += ch_width(ch);
        }
    }
    width
}

#[cfg(feature = "unicode")]
fn ch_width(ch: char) -> usize {
    unicode_width::UnicodeWidthChar::width(ch).unwrap_or(0)
}

#[cfg(not(feature = "unicode"))]
fn ch_width(_: char) -> usize {
    1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(feature = "unicode")]
    use unicode_width::UnicodeWidthChar;

    #[test]
    fn emojis_have_correct_width() {
        use unicode_properties::UnicodeEmoji as _;

        // Emojis in the Basic Latin (ASCII) and Latin-1 Supplement
        // blocks all have a width of 1 column. This includes
        // characters such as '#' and '©'.
        for ch in '\u{1}'..'\u{FF}' {
            if ch.is_emoji_char() {
                let desc = format!("{:?} U+{:04X}", ch, ch as u32);

                #[cfg(feature = "unicode")]
                assert_eq!(ch.width().unwrap(), 1, "char: {desc}");

                #[cfg(not(feature = "unicode"))]
                assert_eq!(ch_width(ch), 1, "char: {desc}");
            }
        }

        // Emojis in the remaining blocks of the Basic Multilingual
        // Plane (BMP), in the Supplementary Multilingual Plane (SMP),
        // and in the Supplementary Ideographic Plane (SIP), are all 1
        // or 2 columns wide when unicode-width is used, and always 2
        // columns wide otherwise. This includes all of our favorite
        // emojis such as 😊.
        for ch in '\u{FF}'..'\u{2FFFF}' {
            if ch.is_emoji_char() {
                let desc = format!("{:?} U+{:04X}", ch, ch as u32);

                #[cfg(feature = "unicode")]
                assert!(ch.width().unwrap() <= 2, "char: {desc}");

                #[cfg(not(feature = "unicode"))]
                assert_eq!(ch_width(ch), 1, "char: {desc}");
            }
        }

        // The remaining planes contain almost no assigned code points
        // and thus also no emojis.
    }

    #[test]
    #[cfg(feature = "unicode")]
    fn display_width_works() {
        assert_eq!("Café Plain".len(), 11); // “é” is two bytes
        assert_eq!(display_width("Café Plain"), 10);
    }

    #[test]
    #[cfg(feature = "unicode")]
    fn display_width_narrow_emojis() {
        assert_eq!(display_width("⁉"), 1);
    }

    #[test]
    #[cfg(feature = "unicode")]
    fn display_width_narrow_emojis_variant_selector() {
        assert_eq!(display_width("⁉\u{fe0f}"), 1);
    }

    #[test]
    #[cfg(feature = "unicode")]
    fn display_width_emojis() {
        assert_eq!(display_width("😂😭🥺🤣✨😍🙏🥰😊🔥"), 20);
    }
}
