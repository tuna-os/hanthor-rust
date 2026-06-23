// SPDX-License-Identifier: GPL-3.0-or-later
//
// LettersWindow — Tabbed word processor following gnome-gui-spec AdwTabView pattern.

use gtk4::{self as gtk, gio, glib, prelude::*};
use libadwaita as adw;
use adw::prelude::{AlertDialogExt, AlertDialogExtManual, AdwDialogExt};
use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;

// ── Per-tab state via widget Qdata ─────────────────────────────────────

#[derive(Clone)]
struct TabData(Rc<RefCell<TabDataInner>>);
struct TabDataInner { file: Option<PathBuf>, closing_after_save: bool }
impl TabData {
    fn new() -> Self { TabData(Rc::new(RefCell::new(TabDataInner { file: None, closing_after_save: false }))) }
}
fn tab_data_set(w: &impl IsA<gtk::Widget>, d: TabData) { unsafe { w.upcast_ref::<gtk::Widget>().set_data("tab-data", d); } }
fn tab_data_get(w: &gtk::Widget) -> Option<TabData> { unsafe { w.data::<TabData>("tab-data").map(|p| p.as_ref().clone()) } }

// ── Make a tab's document widget ──────────────────────────────────────

fn make_doc_widget() -> (gtk::ScrolledWindow, gtk::TextBuffer) {
    let buffer = gtk::TextBuffer::new(None);
    register_formatting_tags(&buffer);
    let editor = gtk::TextView::with_buffer(&buffer);
    editor.set_wrap_mode(gtk::WrapMode::Word);
    editor.set_left_margin(24); editor.set_right_margin(24);
    editor.set_top_margin(16); editor.set_bottom_margin(16);
    editor.set_vexpand(true); editor.set_hexpand(true);
    let scroll = gtk::ScrolledWindow::new();
    scroll.set_child(Some(&editor));
    scroll.set_vexpand(true); scroll.set_hexpand(true);
    (scroll, buffer)
}

// ── LettersWindow ───────────────────────────────────────────────────────

pub struct LettersWindow {
    pub window: adw::ApplicationWindow,
    tab_view: adw::TabView,
    stack: gtk4::Stack,
    word_count_label: gtk4::Label,
}

impl LettersWindow {
    pub fn new(app: &adw::Application) -> Self {
        let tab_view = adw::TabView::new();
        tab_view.set_menu_model(Some(&make_tab_menu()));
        let tab_bar = adw::TabBar::new();
        tab_bar.set_view(Some(&tab_view));

        let stack = gtk4::Stack::new();
        stack.set_transition_type(gtk4::StackTransitionType::Crossfade);
        stack.set_transition_duration(200);
        let empty_page = suite_common::make_empty_state(
            "Letters",
            "Create a new document (Ctrl+N), or open an existing one (Ctrl+O)",
            "accessories-text-editor-symbolic",
            "Open File\u{2026}",
        );
        stack.add_titled(&empty_page, Some("empty"), "Empty");
        stack.add_titled(&tab_view, Some("editor"), "Editor");
        stack.set_visible_child_name("empty");

        let toast_overlay = adw::ToastOverlay::new();
        toast_overlay.set_child(Some(&stack));
        let (status_bar, word_count_label) = suite_common::make_status_bar();

        let suite_win = suite_common::SuiteWindow::new(app, "Letters", vec![], vec![]);
        suite_win.add_top_bar(&tab_bar);
        suite_win.set_content(&toast_overlay);
        suite_win.add_bottom_bar(&status_bar);

        let win = suite_win.window.clone();

        // ── Tab: selected-page ──────────────────────────────────────
        let st = stack.clone();
        tab_view.connect_selected_page_notify(move |_| { st.set_visible_child_name("editor"); });

        // ── Tab: close-page with unsaved confirmation ───────────────
        {
            let tv = tab_view.clone();
            let st = stack.clone();
            tab_view.connect_close_page(move |tv, page| {
                if !page.needs_attention() {
                    tv.close_page_finish(page, true);
                    if tv.n_pages() == 0 { st.set_visible_child_name("empty"); }
                    return glib::Propagation::Stop;
                }
                let title = page.title().to_string();
                let tv2 = tv.clone();
                let st2 = st.clone();
                let child = page.child().clone();
                let body = format!("{}\nThis document has not been saved.", title);
                let dialog = adw::AlertDialog::new(Some("Save document?"), Some(body.as_str()));
                dialog.add_responses(&[("cancel", "_Cancel"), ("discard", "_Discard"), ("save", "_Save")]);
                dialog.set_close_response("cancel");
                dialog.set_default_response(Some("save"));
                dialog.set_response_appearance("save", adw::ResponseAppearance::Suggested);
                dialog.set_response_appearance("cancel", adw::ResponseAppearance::Destructive);
                dialog.choose(None::<&gtk::Window>, None::<&gio::Cancellable>,
                    move |response: glib::GString| {
                        match response.as_str() {
                            "save" => {
                                if let Some(td) = tab_data_get(&child) { td.0.borrow_mut().closing_after_save = true; }
                                do_save(&tv2, &st2);
                            }
                            "discard" => {
                                tv2.close_page_finish(&tv2.page(&child), true);
                                if tv2.n_pages() == 0 { st2.set_visible_child_name("empty"); }
                            }
                            _ => { tv2.close_page_finish(&tv2.page(&child), false); }
                        }
                    },
                );
                glib::Propagation::Stop
            });
        }

        // ── Tab: create-window (drag to new window) ────────────────
        {
            let app_weak = app.downgrade();
            tab_view.connect_create_window(move |_| {
                app_weak.upgrade().map(|app| {
                    let nw = LettersWindow::new(&app);
                    nw.present();
                    nw.tab_view
                })
            });
        }

        // ── Window: close-request with dirty check ──────────────────
        {
            let tv = tab_view.clone();
            let w = win.clone();
            win.connect_close_request(move |win| {
                let n = tv.n_pages();
                let mut dirty: Vec<String> = Vec::new();
                for i in 0..n {
                    let p = tv.nth_page(i);
                    if p.needs_attention() { dirty.push(p.title().to_string()); }
                }
                if dirty.is_empty() { return glib::Propagation::Proceed; }
                let body = format!(
                    "The following documents have unsaved changes:\n• {}\nAll unsaved changes will be discarded if you close Letters now.",
                    dirty.join("\n• ")
                );
                let dialog = adw::AlertDialog::new(Some("Unsaved changes"), Some(body.as_str()));
                dialog.add_responses(&[("cancel", "_Cancel"), ("discard", "_Discard All")]);
                dialog.set_close_response("cancel");
                dialog.set_default_response(Some("cancel"));
                dialog.set_response_appearance("cancel", adw::ResponseAppearance::Destructive);
                let win_weak = win.downgrade();
                dialog.choose(Some(win), None::<&gio::Cancellable>,
                    move |response: glib::GString| {
                        if response == "discard" {
                            if let Some(w) = win_weak.upgrade() { w.close(); }
                        }
                    },
                );
                glib::Propagation::Stop
            });
        }

        // ── Actions ────────────────────────────────────────────────
        Self::register_actions(&tab_view, &stack, &word_count_label, &win, app);

        // ── Formatting actions ────────────────────────────────────
        Self::register_formatting_actions(&tab_view, app);

        // Undo/Redo (GtkTextBuffer built-in)
        {
            let tv = tab_view.clone();
            let a = gtk::gio::SimpleAction::new("undo", None);
            a.connect_activate(move |_, _| {
                if let Some(buf) = active_buffer(&tv) { buf.undo(); }
            });
            app.add_action(&a);
            app.set_accels_for_action("app.undo", &["<Primary>z"]);
        }
        {
            let tv = tab_view.clone();
            let a = gtk::gio::SimpleAction::new("redo", None);
            a.connect_activate(move |_, _| {
                if let Some(buf) = active_buffer(&tv) { buf.redo(); }
            });
            app.add_action(&a);
            app.set_accels_for_action("app.redo", &["<Primary>y", "<Primary><Shift>z"]);
        }

        LettersWindow { window: suite_win.window, tab_view, stack, word_count_label }
    }

    pub fn present(&self) { self.window.present(); }

    fn register_actions(tv: &adw::TabView, st: &gtk4::Stack, wc: &gtk4::Label, win: &adw::ApplicationWindow, app: &adw::Application) {
        // New document
        {
            let tv = tv.clone(); let st = st.clone();
            let a = gtk::gio::SimpleAction::new("new-document", None);
            a.connect_activate(move |_, _| {
                let (scroll, buf) = make_doc_widget();
                let page = tv.append(&scroll);
                page.set_title("Untitled Document");
                page.set_needs_attention(false);
                st.set_visible_child_name("editor");
                tab_data_set(&scroll, TabData::new());
                let p = tv.page(&scroll);
                buf.connect_modified_changed(move |b| { p.set_needs_attention(b.is_modified()); });
            });
            app.add_action(&a);
        }

        // Open file
        {
            let tv = tv.clone(); let st = st.clone(); let w = win.clone();
            let a = gtk::gio::SimpleAction::new("open-file", None);
            a.connect_activate(move |_, _| {
                let tv = tv.clone(); let st = st.clone(); let w = w.clone();
                let dlg = gtk::FileDialog::new();
                let f = gtk::FileFilter::new();
                f.add_pattern("*.md"); f.add_pattern("*.txt"); f.add_pattern("*.html"); f.add_pattern("*.docx");
                f.set_name(Some("Documents"));
                let fl = gio::ListStore::new::<gtk::FileFilter>();
                fl.append(&f);
                dlg.set_filters(Some(&fl));
                dlg.open(Some(&w), None::<&gio::Cancellable>,
                    move |result: Result<gio::File, glib::Error>| {
                        if let Ok(file) = result {
                            let path = file.path().unwrap_or_default();
                            let content = std::fs::read_to_string(&path).unwrap_or_default();
                            let name = file.basename().map(|p| p.display().to_string()).unwrap_or_default();
                            let (scroll, buf) = make_doc_widget();
                            buf.set_text(&content);
                            let td = TabData::new();
                            td.0.borrow_mut().file = Some(path);
                            tab_data_set(&scroll, td);
                            let page = tv.append(&scroll);
                            page.set_title(&name);
                            page.set_tooltip(&name);
                            page.set_needs_attention(false);
                            st.set_visible_child_name("editor");
                            let p = tv.page(&scroll);
                            buf.connect_modified_changed(move |b| { p.set_needs_attention(b.is_modified()); });
                        }
                    },
                );
            });
            app.add_action(&a);
        }

        // Save
        {
            let tv = tv.clone(); let st = st.clone();
            let a = gtk::gio::SimpleAction::new("save-file", None);
            a.connect_activate(move |_, _| { do_save(&tv, &st); });
            app.add_action(&a);
        }

        // Save As
        {
            let tv = tv.clone(); let w = win.clone();
            let a = gtk::gio::SimpleAction::new("save-file-as", None);
            a.connect_activate(move |_, _| {
                let tv = tv.clone(); let w = w.clone();
                let dlg = gtk::FileDialog::new();
                let f = gtk::FileFilter::new();
                f.add_pattern("*.md"); f.add_pattern("*.txt"); f.add_pattern("*.docx");
                f.set_name(Some("Documents"));
                let fl = gio::ListStore::new::<gtk::FileFilter>();
                fl.append(&f);
                dlg.set_filters(Some(&fl));
                dlg.save(Some(&w), None::<&gio::Cancellable>,
                    move |result: Result<gio::File, glib::Error>| {
                        if let Ok(file) = result {
                            if let Some(path) = file.path() {
                                if let Some(page) = tv.selected_page() {
                                    let child = page.child();
                                    let buf = child.first_child()
                                        .and_then(|c| c.downcast::<gtk::TextView>().ok())
                                        .and_then(|tv| Some(tv.buffer()));
                                    if let Some(buf) = buf {
                                        let text = buf.text(&buf.start_iter(), &buf.end_iter(), false);
                                        let _ = std::fs::write(&path, text.as_str());
                                    }
                                    page.set_needs_attention(false);
                                    if let Some(name) = file.basename() { page.set_title(&name.display().to_string()); }
                                    if let Some(path) = file.path() {
                                        if let Some(s) = path.to_str() { page.set_tooltip(s); }
                                    }
                                    if let Some(td) = tab_data_get(&child) {
                                        td.0.borrow_mut().file = file.path();
                                    }
                                }
                            }
                        }
                    },
                );
            });
            app.add_action(&a);
        }
    }
}

// ── Active buffer helper ─────────────────────────────────────────────

fn active_buffer(tv: &adw::TabView) -> Option<gtk::TextBuffer> {
    tv.selected_page().and_then(|p| {
        p.child().first_child()
            .and_then(|c| c.downcast::<gtk::TextView>().ok())
            .map(|tv| tv.buffer())
    })
}

/// Apply a named GtkTextTag to the current selection or cursor position.
fn apply_tag_to_active(tv: &adw::TabView, tag_name: &str) {
    if let Some(buf) = active_buffer(tv) {
        if let Some(tag) = buf.tag_table().lookup(tag_name) {
            let sel = buf.selection_bounds();
            if let Some((start, end)) = sel {
                buf.apply_tag(&tag, &start, &end);
            }
        }
    }
}

/// Toggle a named GtkTextTag on the current selection.
fn toggle_tag(tv: &adw::TabView, tag_name: &str) {
    if let Some(buf) = active_buffer(tv) {
        if let Some(tag) = buf.tag_table().lookup(tag_name) {
            let sel = buf.selection_bounds();
            if let Some((start, end)) = sel {
                // Get all tags at the start of the selection and check if ours is present
                let tags_at_cursor = start.tags();
                let has = tags_at_cursor.iter().any(|t| t.name().as_deref() == Some(tag_name));
                if has {
                    buf.remove_tag(&tag, &start, &end);
                } else {
                    buf.apply_tag(&tag, &start, &end);
                }
            }
        }
    }
}

// ── Formatting action handlers ────────────────────────────────────────

fn toggle_inline_bold(tv: &adw::TabView)  { toggle_tag(tv, "bold"); }
fn toggle_inline_italic(tv: &adw::TabView) { toggle_tag(tv, "italic"); }
fn toggle_inline_underline(tv: &adw::TabView) { toggle_tag(tv, "underline"); }
fn toggle_inline_strikethrough(tv: &adw::TabView) { toggle_tag(tv, "strikethrough"); }
fn toggle_highlight(tv: &adw::TabView) { toggle_tag(tv, "highlight"); }

impl LettersWindow {
    fn register_formatting_actions(tv: &adw::TabView, app: &adw::Application) {
        // Inline formatting
        let pairs: &[(&str, fn(&adw::TabView))] = &[
            ("bold", toggle_inline_bold),
            ("italic", toggle_inline_italic),
            ("underline", toggle_inline_underline),
            ("strikethrough", toggle_inline_strikethrough),
            ("highlight", toggle_highlight),
        ];
        for (name, handler) in pairs {
            let tv = tv.clone();
            let a = gtk::gio::SimpleAction::new(name, None);
            a.connect_activate(move |_, _| handler(&tv));
            app.add_action(&a);
        }

        // Shortcuts for B/I/U
        app.set_accels_for_action("app.bold", &["<Primary>b"]);
        app.set_accels_for_action("app.italic", &["<Primary>i"]);
        app.set_accels_for_action("app.underline", &["<Primary>u"]);

        // Alignment
        let align_names: &[&str] = &["align-left", "align-center", "align-right", "align-justify"];
        for name in align_names {
            let tv = tv.clone();
            let a = gtk::gio::SimpleAction::new(name, None);
            let name = *name;
            a.connect_activate(move |_, _| {
                if let Some(buf) = active_buffer(&tv) {
                    // Get cursor position from selection bounds
                    let bounds = buf.selection_bounds();
                    let (anchor, _) = bounds.unwrap_or_else(|| {
                        (buf.start_iter(), buf.start_iter())
                    });
                    let mut line_start = anchor.clone();
                    line_start.backward_line();
                    let mut line_end = anchor.clone();
                    line_end.forward_line();
                    // Remove all alignment tags from this line first
                    for an in &["align-left", "align-center", "align-right", "align-justify"] {
                        if let Some(at) = buf.tag_table().lookup(an) {
                            buf.remove_tag(&at, &line_start, &line_end);
                        }
                    }
                    // Apply the requested alignment
                    if let Some(tag) = buf.tag_table().lookup(name) {
                        buf.apply_tag(&tag, &line_start, &line_end);
                    }
                }
            });
            app.add_action(&a);
        }
        app.set_accels_for_action("app.align-left", &["<Primary>l"]);
        app.set_accels_for_action("app.align-center", &["<Primary>e"]);
        app.set_accels_for_action("app.align-right", &["<Primary>r"]);
        app.set_accels_for_action("app.align-justify", &["<Primary>j"]);

        // Font size
        {
            let tv = tv.clone();
            let a = gtk::gio::SimpleAction::new("increase-font", None);
            a.connect_activate(move |_, _| {
                if let Some(buf) = active_buffer(&tv) {
                    // Apply a larger scale tag
                    if let Some(tag) = buf.tag_table().lookup("font-larger") {
                        let sel = buf.selection_bounds();
                        if let Some((start, end)) = sel {
                            buf.apply_tag(&tag, &start, &end);
                        }
                    }
                }
            });
            app.add_action(&a);
        }
        {
            let tv = tv.clone();
            let a = gtk::gio::SimpleAction::new("decrease-font", None);
            a.connect_activate(move |_, _| {
                if let Some(buf) = active_buffer(&tv) {
                    if let Some(tag) = buf.tag_table().lookup("font-smaller") {
                        let sel = buf.selection_bounds();
                        if let Some((start, end)) = sel {
                            buf.apply_tag(&tag, &start, &end);
                        }
                    }
                }
            });
            app.add_action(&a);
        }
        app.set_accels_for_action("app.increase-font", &["<Primary><Shift>greater"]);
        app.set_accels_for_action("app.decrease-font", &["<Primary><Shift>less"]);

        // Styles
        let styles: &[(&str, &str)] = &[
            ("style_p", ""),
            ("style_h1", "h1"), ("style_h2", "h2"), ("style_h3", "h3"),
            ("style_h4", "h4"), ("style_h5", "h5"), ("style_h6", "h6"),
            ("style_code", "code"), ("style_quote", "blockquote"),
        ];
        for (action_name, tag_name) in styles {
            let tv = tv.clone();
            let a = gtk::gio::SimpleAction::new(action_name, None);
            let tag_name = *tag_name;
            a.connect_activate(move |_, _| {
                if !tag_name.is_empty() {
                    apply_tag_to_active(&tv, tag_name);
                }
            });
            app.add_action(&a);
        }
    }
}

// ── Save logic ───────────────────────────────────────────────────────

fn do_save(tv: &adw::TabView, _stack: &gtk4::Stack) {
    if let Some(page) = tv.selected_page() {
        if !page.needs_attention() { return; }
        let child = page.child();
        if let Some(td) = tab_data_get(&child) {
            let path = td.0.borrow().file.clone();
            if let Some(path) = path {
                let buf = child.first_child()
                    .and_then(|c| c.downcast::<gtk::TextView>().ok())
                    .and_then(|tv| Some(tv.buffer()));
                if let Some(buf) = buf {
                    let text = buf.text(&buf.start_iter(), &buf.end_iter(), false);
                    let _ = std::fs::write(&path, text.as_str());
                }
                page.set_needs_attention(false);
                if let Some(name) = path.file_name().and_then(|s| s.to_str()) {
                    page.set_title(name);
                }
            }
        }
    }
}

// ── Tab context menu ─────────────────────────────────────────────────

fn make_tab_menu() -> gio::Menu {
    let m = gio::Menu::new();
    let s1 = gio::Menu::new();
    s1.append(Some("Move _Left"), Some("page.move-left"));
    s1.append(Some("Move _Right"), Some("page.move-right"));
    m.append_section(Some("Reorder"), &s1);
    let s2 = gio::Menu::new();
    s2.append(Some("_Move to New Window"), Some("page.move-to-new-window"));
    m.append_section(Some("Window"), &s2);
    let s3 = gio::Menu::new();
    s3.append(Some("Close _Other Tabs"), Some("win.close-other-pages"));
    s3.append(Some("_Close"), Some("win.close-current-page"));
    m.append_section(Some("Close"), &s3);
    m
}

// ── TextTag registration ────────────────────────────────────────────────

pub fn register_formatting_tags(buffer: &gtk::TextBuffer) {
    let tb = buffer.tag_table();
    macro_rules! add { ($tag:expr) => { tb.add(&$tag); }}
    add!(gtk::TextTag::builder().name("bold").weight(700).build());
    add!(gtk::TextTag::builder().name("italic").style(gtk4::pango::Style::Italic).build());
    add!(gtk::TextTag::builder().name("underline").underline(gtk4::pango::Underline::Single).build());
    add!(gtk::TextTag::builder().name("strikethrough").strikethrough(true).build());
    add!(gtk::TextTag::builder().name("highlight").background("#FFFF00").build());
    add!(gtk::TextTag::builder().name("h1").scale(2.0).weight(700).build());
    add!(gtk::TextTag::builder().name("h2").scale(1.5).weight(700).build());
    add!(gtk::TextTag::builder().name("h3").scale(1.17).weight(700).build());
    add!(gtk::TextTag::builder().name("h4").scale(1.0).weight(700).build());
    add!(gtk::TextTag::builder().name("h5").scale(0.83).weight(700).build());
    add!(gtk::TextTag::builder().name("h6").scale(0.67).weight(700).build());
    add!(gtk::TextTag::builder().name("code").family("Monospace").background("#F0F0F0").foreground("#333333").build());
    add!(gtk::TextTag::builder().name("blockquote").left_margin(40).style(gtk4::pango::Style::Italic).foreground("#666666").build());
    // Alignment tags
    add!(gtk::TextTag::builder().name("align-left").justification(gtk::Justification::Left).build());
    add!(gtk::TextTag::builder().name("align-center").justification(gtk::Justification::Center).build());
    add!(gtk::TextTag::builder().name("align-right").justification(gtk::Justification::Right).build());
    add!(gtk::TextTag::builder().name("align-justify").justification(gtk::Justification::Fill).build());
    // Font size tags
    add!(gtk::TextTag::builder().name("font-larger").scale(1.2).build());
    add!(gtk::TextTag::builder().name("font-smaller").scale(0.833).build());
}
