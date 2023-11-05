use askama::Result;

/// Wrap text to a given number of characters
pub fn textwrap<V: std::fmt::Display>(text: V, max_chars: usize) -> Result<String> {
    let text = text.to_string();
    Ok(textwrap::fill(&text, max_chars))
}

/// Replace all forms of extra whitespace (newlines, tabs, consecutive spaces) with a single space
/// to ensure string consumes only one line.
pub fn oneline<V: std::fmt::Display>(text: V) -> Result<String> {
    let text = text.to_string();
    Ok(text
        .replace(['\n', '\t'], " ")
        .replace("\r\n", " ")
        .replace("  ", " "))
}
