// engine.rs — Letters document engine.
pub struct Document { pub text: String }
impl Document {
    pub fn new() -> Self { Self { text: String::new() } }
    pub fn word_count(&self) -> usize { self.text.split_whitespace().count() }
}

pub fn markdown_to_html(md: &str) -> String {
    use pulldown_cmark::{Parser, html};
    let parser = Parser::new(md);
    let mut buf = String::new();
    html::push_html(&mut buf, parser);
    buf
}
