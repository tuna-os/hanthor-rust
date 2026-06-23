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

        let notebook = gtk::Notebook::new();
        let tab1 = gtk::Label::new(Some("📄 Document 1"));
        let editor = gtk::TextView::new();
        editor.set_monospace(true); editor.set_wrap_mode(gtk::WrapMode::Word);
        let s = gtk::ScrolledWindow::new(); s.set_child(Some(&editor)); s.set_vexpand(true);
        notebook.append_page(&s, Some(&tab1));

        let status = gtk::Label::new(Some("0 words"));
        status.set_halign(gtk::Align::End);

        let m = gtk::Box::new(gtk::Orientation::Vertical, 2);
        m.append(&toolbar); m.append(&notebook); m.append(&status);
        let c = gtk::Box::new(gtk::Orientation::Vertical, 0);
        c.append(&header); c.append(&m);
        win.set_child(Some(&c));

        // Keyboard shortcuts
        let key = gtk::EventControllerKey::new();
        let editor_ref = editor.clone();
        key.connect_key_pressed(move |_, keyval, _keycode, state| {
            let ctrl = state.contains(gtk::gdk::ModifierType::CONTROL_MASK);
            let shift = state.contains(gtk::gdk::ModifierType::SHIFT_MASK);
            let alt = state.contains(gtk::gdk::ModifierType::ALT_MASK);
            if !ctrl && !alt { return gtk::glib::Propagation::Proceed; }
            let buf = editor_ref.buffer();

            let handled = match (ctrl, shift, alt, keyval) {
                (true, false, false, Key::B) | (true, false, false, Key::b) => { toggle_tag(&buf, "bold", 700, gtk::pango::Style::Normal); true }
                (true, false, false, Key::I) | (true, false, false, Key::i) => { toggle_tag(&buf, "italic", 400, gtk::pango::Style::Italic); true }
                (true, false, false, Key::U) | (true, false, false, Key::u) => { toggle_tag(&buf, "underline", 400, gtk::pango::Style::Normal); true }
                (true, true, false, Key::L) | (true, true, false, Key::l) => { apply_align(&buf, gtk::Justification::Left); true }
                (true, true, false, Key::E) | (true, true, false, Key::e) => { apply_align(&buf, gtk::Justification::Center); true }
                (true, true, false, Key::R) | (true, true, false, Key::r) => { apply_align(&buf, gtk::Justification::Right); true }
                (true, true, false, Key::J) | (true, true, false, Key::j) => { apply_align(&buf, gtk::Justification::Fill); true }
                (true, true, false, Key::_7) => { insert_list(&buf, "• "); true }
                (true, true, false, Key::_8) => { insert_list(&buf, "1. "); true }
                (true, false, false, Key::bracketright) => { change_indent(&buf, true); true }
                (true, false, false, Key::bracketleft) => { change_indent(&buf, false); true }
                (true, false, false, Key::K) | (true, false, false, Key::k) => { insert_link(&buf); true }
                _ => false,
            };
            if handled { gtk::glib::Propagation::Stop } else { gtk::glib::Propagation::Proceed }
        });
        editor.add_controller(key);

        Self { window: win, editor }
    }
    pub fn present(&self) { self.window.present(); }
}

fn toggle_tag(buf: &gtk::TextBuffer, tag_name: &str, weight: i32, style: gtk::pango::Style) {
    let tags = buf.tag_table();
    if let Some(tag) = tags.lookup(tag_name) {
        tags.remove(&tag);
        return;
    }
    let tag = gtk::TextTag::new(Some(tag_name));
    tag.set_weight(weight);
    tag.set_style(style);
    if tag_name == "underline" { tag.set_underline(gtk::pango::Underline::Single); }
    tags.add(&tag);
    if let Some((start, end)) = buf.selection_bounds() {
        buf.apply_tag(&tag, &start, &end);
    }
}

fn apply_align(buf: &gtk::TextBuffer, just: gtk::Justification) {
    let tag_name = "align";
    let tags = buf.tag_table();
    // Remove old alignment
    if let Some(old) = tags.lookup(tag_name) { tags.remove(&old); }
    let tag = gtk::TextTag::new(Some(tag_name));
    tag.set_justification(just);
    tags.add(&tag);
    if let Some((start, end)) = buf.selection_bounds() {
        buf.apply_tag(&tag, &start, &end);
    }
}

fn insert_list(buf: &gtk::TextBuffer, prefix: &str) {
    if let Some((start, end)) = buf.selection_bounds() {
        let text = start.slice(&end).to_string();
        let new_text: String = text.lines().map(|l| format!("{}{}\n", prefix, l)).collect();
        buf.delete(&start, &end);
        buf.insert(&start, &new_text.trim_end());
    }
}

fn change_indent(buf: &gtk::TextBuffer, increase: bool) {
    if let Some((start, end)) = buf.selection_bounds() {
        let text = start.slice(&end).to_string();
        let new_text: String = if increase {
            text.lines().map(|l| format!("\t{}\n", l)).collect()
        } else {
            text.lines().map(|l| format!("{}\n", l.strip_prefix('\t').unwrap_or(l))).collect()
        };
        buf.delete(&start, &end);
        buf.insert(&start, &new_text.trim_end());
    }
}

fn insert_link(buf: &gtk::TextBuffer) {
    if let Some((start, end)) = buf.selection_bounds() {
        let text = start.slice(&end).to_string();
        let tag_name = "link";
        let tags = buf.tag_table();
        let tag = tags.lookup(tag_name).unwrap_or_else(|| { let t = gtk::TextTag::new(Some(tag_name)); t.set_underline(gtk::pango::Underline::Single); tags.add(&t); t });
        buf.apply_tag(&tag, &start, &end);
    }
}
