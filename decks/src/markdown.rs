// markdown.rs — Markdown macro rendering for Decks text boxes.
// SPDX-License-Identifier: GPL-3.0-or-later
//
// Interprets **bold**, *italic*, `code`, # heading conventions
// using pulldown-cmark parser + Pango attributes on Cairo context.
// Full implementation deferred — see #3 for tracking.

use gtk4::cairo;

/// Render markdown text. Currently a stub — restores plain text fallback.
pub fn render_markdown(
    _cr: &cairo::Context, _text: &str,
    _x: f64, _y: f64, _max_width: f64, _font_size: f64,
) -> f64 { 0.0 }
