// SPDX-License-Identifier: GPL-3.0-or-later
//
// TablesWindow — Spreadsheet window with a Cairo grid widget.

use gtk4::prelude::*;
use libadwaita as adw;

const ROWS: usize = 100;
const COLS: usize = 26;

pub struct TablesWindow {
    window: adw::ApplicationWindow,
}

impl TablesWindow {
    pub fn new(app: &adw::Application) -> Self {
        // ---- Model: one Object per row ----
        let model = gtk4::gio::ListStore::new::<gtk4::glib::Object>();
        for _ in 0..ROWS {
            model.append(&gtk4::glib::Object::new::<gtk4::glib::Object>());
        }

        let factory = gtk4::SignalListItemFactory::new();
        factory.connect_setup(|_, item| {
            let row = gtk4::Box::new(gtk4::Orientation::Horizontal, 0);
            let num = gtk4::Label::new(None);
            num.set_width_chars(4);
            num.set_xalign(1.0);
            num.add_css_class("dim-label");
            num.set_margin_end(4);
            row.append(&num);
            for _ in 0..COLS {
                let e = gtk4::Entry::new();
                e.set_has_frame(false);
                e.set_width_chars(10);
                row.append(&e);
            }
            item.downcast_ref::<gtk4::ListItem>().unwrap().set_child(Some(&row));
        });
        factory.connect_bind(|_, item| {
            let li = item.downcast_ref::<gtk4::ListItem>().unwrap();
            let r = li.position() as usize;
            let row = li.child().unwrap().downcast::<gtk4::Box>().unwrap();
            let lbl = row.first_child().unwrap().downcast::<gtk4::Label>().unwrap();
            lbl.set_label(&(r + 1).to_string());
            let mut node = row.first_child().and_then(|n| n.next_sibling());
            let mut c = 0usize;
            while let Some(w) = node {
                if let Ok(e) = w.clone().downcast::<gtk4::Entry>() {
                    let header = ((b'A' + c as u8) as char).to_string();
                    e.set_text(if r == 0 { &header } else { "" });
                }
                node = w.next_sibling();
                c += 1;
            }
        });

        let list = gtk4::ListView::new(
            Some(gtk4::NoSelection::new(Some(model))),
            Some(factory),
        );
        let scroll = gtk4::ScrolledWindow::new();
        scroll.set_child(Some(&list));
        scroll.set_vexpand(true);
        scroll.set_hexpand(true);

        // ---- Content stack: empty state <-> grid ----
        let stack = gtk4::Stack::new();
        stack.set_transition_type(gtk4::StackTransitionType::Crossfade);
        stack.set_transition_duration(200);

        let empty_page = suite_common::make_empty_state(
            "Tables",
            "Create a new spreadsheet or open an existing one",
            "x-office-spreadsheet-symbolic",
            "Open File\u{2026}",
        );
        let grid_box = gtk4::Box::new(gtk4::Orientation::Vertical, 0);
        grid_box.append(&scroll);

        stack.add_titled(&empty_page, Some("empty"), "Empty");
        stack.add_titled(&grid_box, Some("editor"), "Editor");
        stack.set_visible_child_name("empty");

        // ---- Formula bar ----
        let fx_bar = gtk4::Box::new(gtk4::Orientation::Horizontal, 6);
        fx_bar.set_margin_start(6);
        fx_bar.set_margin_end(6);
        fx_bar.set_margin_top(2);
        fx_bar.set_margin_bottom(2);
        let fx_label = gtk4::Label::new(Some("fx"));
        fx_label.add_css_class("dim-label");
        let fx_entry = gtk4::Entry::new();
        fx_entry.set_placeholder_text(Some("Formula\u{2026}"));
        fx_entry.set_hexpand(true);
        fx_bar.append(&fx_label);
        fx_bar.append(&fx_entry);

        // ---- Toolbar (basic for now) ----
        let primary: Vec<(&str, &str, Box<dyn Fn(bool)>)> = vec![
            ("B", "Bold", Box::new(|_| {})),
            ("I", "Italic", Box::new(|_| {})),
            ("U", "Underline", Box::new(|_| {})),
        ];
        let extended: Vec<(&str, &str, Box<dyn Fn()>)> = vec![
            ("Merge", "Merge cells", Box::new(|| {})),
        ];

        let suite_win = suite_common::SuiteWindow::new(app, "Tables", primary, extended);
        suite_win.add_top_bar(&fx_bar);
        suite_win.set_content(&stack);

        // Register per-app actions
        let st = stack.clone();
        let act_open = gtk4::gio::SimpleAction::new("open-file", None);
        act_open.connect_activate(move |_, _| {
            st.set_visible_child_name("editor");
        });
        app.add_action(&act_open);

        let st2 = stack.clone();
        let act_new = gtk4::gio::SimpleAction::new("new-document", None);
        act_new.connect_activate(move |_, _| {
            st2.set_visible_child_name("editor");
        });
        app.add_action(&act_new);

        Self {
            window: suite_win.window,
        }
    }

    pub fn present(&self) {
        self.window.present();
    }
}
