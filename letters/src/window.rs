// SPDX-License-Identifier: GPL-3.0-or-later
//
// LettersWindow — Word processor window with GtkTextView editor.

use gtk4::prelude::*;
use libadwaita as adw;

pub struct LettersWindow {
    window: adw::ApplicationWindow,
    buffer: gtk4::TextBuffer,
    pub editor: gtk4::TextView,
}

impl LettersWindow {
    pub fn new(app: &adw::Application) -> Self {
        // ---- GtkTextBuffer with basic formatting tags ----
        let buffer = gtk4::TextBuffer::new(None);
        register_formatting_tags(&buffer);

        // ---- Editor widget ----
        let editor = gtk4::TextView::with_buffer(&buffer);
        editor.set_wrap_mode(gtk4::WrapMode::Word);
        editor.set_left_margin(24);
        editor.set_right_margin(24);
        editor.set_top_margin(16);
        editor.set_bottom_margin(16);
        editor.set_vexpand(true);
        editor.set_hexpand(true);

        let scroll = gtk4::ScrolledWindow::new();
        scroll.set_child(Some(&editor));
        scroll.set_vexpand(true);
        scroll.set_hexpand(true);

        // ---- Content stack: empty state <-> editor ----
        let stack = gtk4::Stack::new();
        stack.set_transition_type(gtk4::StackTransitionType::Crossfade);
        stack.set_transition_duration(200);

        let empty_page = suite_common::make_empty_state(
            "Letters",
            "Create a new document or open an existing one",
            "accessories-text-editor-symbolic",
            "Open File\u{2026}",
        );
        stack.add_titled(&empty_page, Some("empty"), "Empty");
        stack.add_titled(&scroll, Some("editor"), "Editor");
        stack.set_visible_child_name("empty");

        // ---- Toolbar actions ----
        let buf = buffer.clone();
        let bold_cb = move |active: bool| toggle_tag(&buf, "bold", active);
        let buf = buffer.clone();
        let italic_cb = move |active: bool| toggle_tag(&buf, "italic", active);
        let buf = buffer.clone();
        let underline_cb = move |active: bool| toggle_tag(&buf, "underline", active);

        let primary: Vec<(&str, &str, Box<dyn Fn(bool)>)> = vec![
            ("B", "Bold (Ctrl+B)", Box::new(bold_cb)),
            ("I", "Italic (Ctrl+I)", Box::new(italic_cb)),
            ("U", "Underline (Ctrl+U)", Box::new(underline_cb)),
        ];
        let extended: Vec<(&str, &str, Box<dyn Fn()>)> = vec![
            ("S", "Strikethrough", Box::new(|| {})),
            ("H", "Highlight", Box::new(|| {})),
        ];

        // ---- SuiteWindow (chrome) ----
        let suite_win = suite_common::SuiteWindow::new(app, "Letters", primary, extended);
        suite_win.set_content(&stack);

        // ---- Register per-app actions (SuiteApp's "new" forwards to "new-document") ----
        let st = stack.clone();
        let buf = buffer.clone();
        let act_new = gtk4::gio::SimpleAction::new("new-document", None);
        act_new.connect_activate(move |_, _| {
            buf.set_text("");
            st.set_visible_child_name("editor");
        });
        app.add_action(&act_new);

        let st2 = stack.clone();
        let act_open = gtk4::gio::SimpleAction::new("open-file", None);
        act_open.connect_activate(move |_, _| {
            st2.set_visible_child_name("editor");
        });
        app.add_action(&act_open);

        Self {
            window: suite_win.window,
            buffer,
            editor,
        }
    }

    pub fn present(&self) {
        self.window.present();
    }
}

/// Register GtkTextTags for formatting.
fn register_formatting_tags(buffer: &gtk4::TextBuffer) {
    let tb = buffer.tag_table();
    let bold = gtk4::TextTag::builder()
        .name("bold")
        .weight(700)
        .build();
    tb.add(&bold);

    let italic = gtk4::TextTag::builder()
        .name("italic")
        .style(gtk4::pango::Style::Italic)
        .build();
    tb.add(&italic);

    let underline = gtk4::TextTag::builder()
        .name("underline")
        .underline(gtk4::pango::Underline::Single)
        .build();
    tb.add(&underline);

    let strikethrough = gtk4::TextTag::builder()
        .name("strikethrough")
        .strikethrough(true)
        .build();
    tb.add(&strikethrough);
}

/// Toggle a named tag on the current selection.
fn toggle_tag(buffer: &gtk4::TextBuffer, tag_name: &str, active: bool) {
    if let Some(tag) = buffer.tag_table().lookup(tag_name) {
        let sel = buffer.selection_bounds();
        if let Some((start, end)) = sel {
            if active {
                buffer.apply_tag(&tag, &start, &end);
            } else {
                buffer.remove_tag(&tag, &start, &end);
            }
        }
    }
}
