// engine.rs — Presentation engine: ppt-rs for PPTX I/O.
// SPDX-License-Identifier: GPL-3.0-or-later
// Library per RFC suite-common#12: ppt-rs for read/write .pptx.

pub struct Deck {
    pub slides: Vec<Slide>,
}

pub struct Slide {
    pub title: String,
    pub objects: Vec<SlideObject>,
}

pub enum SlideObject {
    TextBox { text: String, x: f64, y: f64, w: f64, h: f64 },
    Rect { x: f64, y: f64, w: f64, h: f64 },
}

impl Deck {
    pub fn new() -> Self { Self { slides: vec![Slide { title: "Slide 1".into(), objects: vec![] }] } }
}

pub fn read_pptx(path: &str) -> Result<Deck, String> {
    Err(format!("ppt-rs read not yet wired: {}", path))
}

pub fn write_pptx(path: &str, _deck: &Deck) -> Result<(), String> {
    Err(format!("ppt-rs write not yet wired: {}", path))
}
