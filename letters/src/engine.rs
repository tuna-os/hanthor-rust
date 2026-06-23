// engine.rs — Word processor engine: docx-rs for DOCX I/O.
// SPDX-License-Identifier: GPL-3.0-or-later
// Library per RFC suite-common#12: docx-rs for read/write .docx.

pub struct Document {
    pub text: String,
}

impl Document {
    pub fn new() -> Self { Self { text: String::new() } }
}

pub fn read_docx(path: &str) -> Result<Document, String> {
    Err(format!("docx-rs read not yet wired: {}", path))
}

pub fn write_docx(path: &str, doc: &Document) -> Result<(), String> {
    Err(format!("docx-rs write not yet wired: {}", path))
}
