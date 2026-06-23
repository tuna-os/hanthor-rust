use gtk4 as gtk;
use gtk::prelude::*;
use gtk::gdk::Key;

pub struct LettersWindow { window: gtk::ApplicationWindow, editor: gtk::TextView }

impl LettersWindow {
    pub fn new(app: &gtk::Application) -> Self {
        let win = gtk::ApplicationWindow::builder().application(app).build();
        win.set_title(Some("Letters"));
        win.set_default_size(800, 600);
        let header = suite_common::make_header_bar();
        let toolbar = suite_common::make_toolbar();
        let styles = gtk::DropDown::from_strings(&["Paragraph","Heading 1","Heading 2","Heading 3","Code","Quote"]);
        toolbar.append(&styles);
        toolbar.append(&gtk::Button::with_label("Table"));

        let editor = gtk::TextView::new();
        editor.set_monospace(true); editor.set_wrap_mode(gtk::WrapMode::Word);
        let s = gtk::ScrolledWindow::new(); s.set_child(Some(&editor)); s.set_vexpand(true);

        // Tabs
        let notebook = gtk::Notebook::new();
        let tab1 = gtk::Label::new(Some("Document 1"));
        notebook.append_page(&s, Some(&tab1));
        let status = gtk::Label::new(Some("0 words")); status.set_halign(gtk::Align::End);

        let m = gtk::Box::new(gtk::Orientation::Vertical, 2);
        m.append(&toolbar); m.append(&notebook); m.append(&status);
        let c = gtk::Box::new(gtk::Orientation::Vertical, 0);
        c.append(&header); c.append(&m);
        win.set_child(Some(&c));

        // Keyboard shortcuts — Google Docs style
        let key = gtk::EventControllerKey::new();
        let editor_ref = editor.clone();
        key.connect_key_pressed(move |_ctrl, keyval, _keycode, state| {
            let ctrl = state.contains(gtk::gdk::ModifierType::CONTROL_MASK);
            let shift = state.contains(gtk::gdk::ModifierType::SHIFT_MASK);
            let alt = state.contains(gtk::gdk::ModifierType::ALT_MASK);
            if !ctrl && !alt { return gtk::glib::Propagation::Proceed; }
            let buf = editor_ref.buffer();
            let handled = match (ctrl, shift, alt, keyval) {
                // Bold / Italic / Underline
                (true, false, false, Key::b) => { toggle_tag(&buf, "bold"); true }
                (true, false, false, Key::i) => { toggle_tag(&buf, "italic"); true }
                (true, false, false, Key::u) => { toggle_tag(&buf, "underline"); true }
                // Align Left/Center/Right/Justify
                (true, true, false, Key::l) => { add_tag(&buf, "align-left", |t| t.set_justification(gtk::Justification::Left)); true }
                (true, true, false, Key::e) => { add_tag(&buf, "align-center", |t| t.set_justification(gtk::Justification::Center)); true }
                (true, true, false, Key::r) => { add_tag(&buf, "align-right", |t| t.set_justification(gtk::Justification::Right)); true }
                (true, true, false, Key::j) => { add_tag(&buf, "align-justify", |t| t.set_justification(gtk::Justification::Fill)); true }
                // Bulleted / Numbered list
                (true, true, false, Key::_7) => { insert_prefix(&buf, "• "); true }
                (true, true, false, Key::_8) => { insert_prefix(&buf, "1. "); true }
                // Indent / Outdent
                (true, false, false, Key::bracketright) => { indent_text(&buf, true); true }
                (true, false, false, Key::bracketleft) => { indent_text(&buf, false); true }
                // Insert link
                (true, false, false, Key::k) => { add_tag(&buf, "link", |t| { t.set_underline(gtk::pango::Underline::Single); }); true }
                // Find / Replace
                (true, false, false, Key::f) => { println!("Find: Ctrl+F"); true }
                (true, false, false, Key::h) => { println!("Replace: Ctrl+H"); true }
                // Heading styles Ctrl+Alt+0-6
                (true, false, true, Key::_1) => { add_tag(&buf, "h1", |t| { t.set_weight(700); t.set_scale(2.0); }); true }
                (true, false, true, Key::_2) => { add_tag(&buf, "h2", |t| { t.set_weight(700); t.set_scale(1.7); }); true }
                (true, false, true, Key::_3) => { add_tag(&buf, "h3", |t| { t.set_weight(700); t.set_scale(1.4); }); true }
                (true, false, true, Key::_4) => { add_tag(&buf, "h4", |t| { t.set_weight(600); t.set_scale(1.2); }); true }
                (true, false, true, Key::_5) => { add_tag(&buf, "h5", |t| { t.set_weight(600); t.set_scale(1.1); }); true }
                (true, false, true, Key::_6) => { add_tag(&buf, "h6", |t| { t.set_weight(600); t.set_scale(1.0); }); true }
                (true, false, true, Key::_0) => { add_tag(&buf, "normal", |t| { t.set_weight(400); t.set_scale(1.0); }); true }
                (true, true, false, Key::period) => { add_tag(&buf, "fs-up", |t| t.set_scale(1.2)); true }
                (true, true, false, Key::comma) => { add_tag(&buf, "fs-down", |t| t.set_scale(0.9)); true }
                _ => false,
            };
            if handled { gtk::glib::Propagation::Stop } else { gtk::glib::Propagation::Proceed }
        });
        editor.add_controller(key);

        Self { window: win, editor }
    }
    pub fn present(&self) { self.window.present(); }
}

/// Toggle a simple text tag on/off for the current selection.
fn toggle_tag(buf: &gtk::TextBuffer, tag_name: &str) {
    let tags = buf.tag_table();
    if tags.lookup(tag_name).is_some() {
        // Tag exists — remove it
        if let Some(old) = tags.lookup(tag_name) { tags.remove(&old); }
        return;
    }
    let tag = gtk::TextTag::new(Some(tag_name));
    match tag_name {
        "bold" => tag.set_weight(700),
        "italic" => tag.set_style(gtk::pango::Style::Italic),
        "underline" => tag.set_underline(gtk::pango::Underline::Single),
        _ => {}
    }
    tags.add(&tag);
    apply_tag_to_selection(buf, &tag);
}

/// Add a configured tag to the current selection.
fn add_tag<F: FnOnce(&gtk::TextTag)>(buf: &gtk::TextBuffer, tag_name: &str, configure: F) {
    let tags = buf.tag_table();
    let tag = tags.lookup(tag_name).unwrap_or_else(|| {
        let t = gtk::TextTag::new(Some(tag_name));
        configure(&t);
        tags.add(&t);
        t
    });
    apply_tag_to_selection(buf, &tag);
}

fn apply_tag_to_selection(buf: &gtk::TextBuffer, tag: &gtk::TextTag) {
    let bounds = buf.selection_bounds();
    let (start, end) = match bounds {
        Some((s, e)) => (s, e),
        None => return,
    };
    buf.apply_tag(tag, &start, &end);
}

fn insert_prefix(buf: &gtk::TextBuffer, prefix: &str) {
    let bounds = buf.selection_bounds();
    let (start, end) = match bounds {
        Some((s, e)) => (s, e),
        None => return,
    };
    let text = buf.text(&start, &end, false).to_string();
    let new_text: String = text.lines().map(|l| format!("{}{}\n", prefix, l)).collect();
    buf.delete(&start, &end);
    buf.insert(&start, new_text.trim_end());
}

fn indent_text(buf: &gtk::TextBuffer, increase: bool) {
    let bounds = buf.selection_bounds();
    let (start, end) = match bounds {
        Some((s, e)) => (s, e),
        None => return,
    };
    let text = buf.text(&start, &end, false).to_string();
    let new_text: String = if increase {
        text.lines().map(|l| format!("\t{}\n", l)).collect()
    } else {
        text.lines().map(|l| format!("{}\n", l.strip_prefix('\t').unwrap_or(l))).collect()
    };
    buf.delete(&start, &end);
    buf.insert(&start, new_text.trim_end());
}

// Add heading shortcuts to the key handler
// Ctrl+Alt+1 = H1, Ctrl+Alt+2 = H2, etc.
(
