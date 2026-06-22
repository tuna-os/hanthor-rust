// engine.rs — Decks presentation engine.
use std::path::Path;

pub struct Deck { pub slides: Vec<Slide> }
pub struct Slide { pub title: String, pub objects: Vec<SlideObject> }
pub enum SlideObject { TextBox { text: String, x: f64, y: f64, w: f64, h: f64 }, Rect { x: f64, y: f64, w: f64, h: f64 } }

impl Deck {
    pub fn new() -> Self { Self { slides: vec![Slide { title: "Slide 1".into(), objects: vec![] }] } }
    pub fn add_slide(&mut self) { self.slides.push(Slide { title: format!("Slide {}", self.slides.len() + 1), objects: vec![] }); }
    pub fn delete_slide(&mut self, idx: usize) { if idx < self.slides.len() { self.slides.remove(idx); } }
}
