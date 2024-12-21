use crate::text_provider::TextProvider;

/// Simple template engine suitable for interpolating texts only, without any bells or whistles.
pub(crate) fn interpolate(template: &str, texts: &impl TextProvider) -> String {
    let mut with_texts = String::new();
    let mut parse_state = ParseState::NotStarted;

    for (i, c) in template.char_indices() {
        parse_state = parse_key_template(i, c, parse_state).expect("Text interpolation failed");

        if let ParseState::Complete(s, e) = parse_state {
            let key = &template[s..e];
            with_texts.push_str(texts.get(key));
        }

        if let ParseState::NotStarted = parse_state {
            with_texts.push(c);
        }
    }

    with_texts
}

/// Given a [char] and a [`ParseState`], determine the next state.
/// 
/// ## Inputs
/// 
/// - `idx` - the index of the current [char], which is used for tracking the span of the current key
/// - `current` - the current [char], which we use for determining the next [`ParseState`]
/// - `current_state` - what stage in parsing an interpolation are we at
pub(crate) fn parse_key_template(
    idx: usize,
    current: char,
    current_state: ParseState,
) -> Result<ParseState, &'static str> {
    match current_state {
        // We haven't started parsing an interpolation or we just finished one, but we found an open brace -> transition to Started
        ParseState::NotStarted | ParseState::Complete(..) if current == '{' => Ok(ParseState::Started),
        // We have not started parsing an interpolation and have not encountered an open brace, so maintain current state
        ParseState::NotStarted => Ok(ParseState::NotStarted),
        // We have encountered the the open brace but have not found the start of a key yet, so maintain current state
        ParseState::Started if current.is_ascii_whitespace() => Ok(ParseState::Started),
        // We have encountered an open brace and have no encountered the start of the key, so transition to KeyStarted
        ParseState::Started if current.is_ascii_alphanumeric() => Ok(ParseState::KeyStarted(idx)),
        // If we find any other characters besides whitespace or a valid start char, this is an error and we fail
        ParseState::Started => {
            Err("Chars in braces prior to the start of the key must only be whitespace")
        }
        // If we have started parsing a key and we encounter whitespace, then we're done and transition to KeyFinished
        ParseState::KeyStarted(s) if current.is_ascii_whitespace() => {
            Ok(ParseState::KeyFinished(s))
        }
        // If we are in KeyFinished or KeyStarted state and encounter a closing brance, then we are finished and transition to Complete
        ParseState::KeyFinished(s) | ParseState::KeyStarted(s) if current == '}' => {
            Ok(ParseState::Complete(s, idx))
        }
        // If key hasstarted and we encounter any valid key chars, then we maintain current state and continue
        ParseState::KeyStarted(s)
            if current.is_ascii_alphanumeric() || current == '-' || current == '.' =>
        {
            Ok(ParseState::KeyStarted(s))
        }
        // If we're in key started and haven't hit an earlier pattern, that means we hit an invalid char
        ParseState::KeyStarted(..) => Err("Invalid char encountered while parsing text key"), // TODO: should report more info to help diagnose issues
        // If the key is finished and we found anything other than whitespace or a closing brace, that's an error
        ParseState::KeyFinished(..) => Err("Failed to find closing } character for text interpolation"),
        // We are in a completed state and need to transition back to NotStarted
        ParseState::Complete(..) => Ok(ParseState::NotStarted),
    }
}

/// A set of states to track what stage of parsing an interplation we're at
#[derive(Debug)]
pub(crate) enum ParseState {
    /// We haven't located the opening brace yet
    NotStarted,

    /// We have encountered the opnening brace, but have not yet seen the first valid key character. In other words, we've
    /// only seen whitespace up to now.
    Started,

    /// We have encountered the first valid key character, so we have started parsing a key
    KeyStarted(usize),

    /// We encountered whitespace after having started parsing a key, so the key is finished
    KeyFinished(usize),

    /// We have encountered the closing brace, so the parse is complete
    Complete(usize, usize),
}
