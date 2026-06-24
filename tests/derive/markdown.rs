#![cfg(feature = "unstable-markdown")]

use clap::CommandFactory;
use clap_derive::Parser;
use snapbox::file;

macro_rules! assert_help {
    ($Command:ty, $filename:literal) => {{
        let help = <$Command>::command().render_long_help().ansi().to_string();
        snapbox::assert_data_eq!(help, file![$filename]);
    }};
}

#[test]
fn headers() {
    /// # This is a header
    /// ## second level
    /// ### `additional` *styling **on ~top~ of** it*
    /// regular paragraph
    #[derive(Parser)]
    struct Command;

    assert_help!(Command, "snapshots/headers.term.svg");
}

#[test]
fn inline_styles() {
    /// *emphasis* **bold** ~strike through~ `code`
    ///
    /// *all **of ~them `combined` in~ one** line*
    #[derive(Parser)]
    struct Command;

    assert_help!(Command, "snapshots/inline_styles.term.svg");
}

#[test]
fn links() {
    /// <https://example.com/literal>
    ///
    /// [with name](https://example.com/with%20name)
    ///
    /// ![image](https://example.com/image)
    ///
    /// [referencing][reference]
    ///
    /// [reference]: https://example.com/reference
    #[derive(Parser)]
    struct Command;

    assert_help!(Command, "snapshots/links.term.svg");
}

#[test]
fn html() {
    /// <html>
    ///     <is>
    ///         <used>
    ///     </verbatim>
    /// </html>
    ///
    /// <inline>html</as-well>
    #[derive(Parser)]
    struct Command;

    assert_help!(Command, "snapshots/html.term.svg");
}

#[test]
fn blocks() {
    /// ```rust
    /// This is a *fenced* code block.
    ///
    /// There is not much going on in terms of **styling**.
    /// ```
    ///
    /// ---
    ///
    ///     Code blocks can also be initiated through
    ///     Indentation.
    ///
    /// > This is a block quote.
    /// > **Regular ~styling *should* work~ here.**
    /// >
    /// > # even headings
    /// > and regular paragraphs.
    /// >
    /// > - lists
    /// >   - are
    /// >
    /// > 1. also
    /// >    1. supported
    /// >
    /// > > nesting them
    /// > > > also works (not)
    ///
    #[derive(Parser)]
    struct Command;

    assert_help!(Command, "snapshots/blocks.term.svg");
}

#[test]
fn lists() {
    /// Lists:
    ///
    /// - unordered
    ///   - bullet
    ///     - lists
    /// - with multiple
    ///   - levels
    ///
    /// 0. numeric lists
    /// 1. only care
    ///    1. about the initial number
    /// 2. 5. and count from there
    ///    7. anything goes
    /// 3. though they need an empty line
    #[derive(Parser)]
    struct Command;

    assert_help!(Command, "snapshots/lists.term.svg");
}

#[test]
fn paragraphs() {
    /// Paragraphs are separated by empty lines.
    /// All lines will be joined onto one.
    ///
    /// The first paragraph is used as short help by clap.\
    /// backslashes can be used to insert hard line breaks.
    ///
    /// | these | can   |\
    /// | ----- | ----- |\
    /// | be    | used  |\
    /// | for   | tables|
    ///
    /// Because tables are not yet supported.
    ///
    #[doc = "You can also use trailing spaces for hard breaks,  \nbut this is not really recommended."]
    #[derive(Parser)]
    struct Command;

    assert_help!(Command, "snapshots/paragraphs.term.svg");
}
