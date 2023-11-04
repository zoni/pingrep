use askama::Result;

pub fn textwrap<V: std::fmt::Display>(text: V, max_chars: usize) -> Result<String> {
    let text = text.to_string();
    Ok(textwrap::fill(&text, max_chars))
}
