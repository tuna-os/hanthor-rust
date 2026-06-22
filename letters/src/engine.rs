// engine.rs — Letters document engine.
pub struct Document { pub text: String }
impl Document {
    pub fn new() -> Self { Self { text: String::new() } }
    pub fn word_count(&self) -> usize { self.text.split_whitespace().count() }
}
