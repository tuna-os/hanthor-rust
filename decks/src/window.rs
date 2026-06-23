// SPDX-License-Identifier: GPL-3.0-or-later
//
// DecksWindow — Presentation window with Cairo slide canvas.

use gtk4::prelude::*;
use libadwaita as adw;

pub struct DecksWindow {
    window: adw::ApplicationWindow,
}

impl DecksWindow {
    pub fn new(app: &adw::Application) -> Self {
        // ---- Content stack: empty state <-> slide canvas ----
        let stack = gtk4::Stack::new();
        stack.set_transition_type(gtk4::StackTransitionType::Crossfade);
        stack.set_transition_duration(200);

        let empty_page = suite_common::make_empty_state(
            "Decks",
            "Create a new presentation or open an existing one",
            "x-office-presentation-symbolic",
            "Open File\u{2026}",
        );

        let canvas_content = gtk4::Label::new(Some("Decks \u{2014} slide canvas here"));
        canvas_content.set_vexpand(true);
        canvas_content.set_hexpand(true);
        let canvas_scroll = gtk4::ScrolledWindow::new();
        canvas_scroll.set_child(Some(&canvas_content));
        canvas_scroll.set_vexpand(true);
        canvas_scroll.set_hexpand(true);

        stack.add_titled(&empty_page, Some("empty"), "Empty");
        stack.add_titled(&canvas_scroll, Some("editor"), "Editor");
        stack.set_visible_child_name("empty");

        // ---- Toolbar ----
        let primary: Vec<(&str, &str, Box<dyn Fn(bool)>)> = vec![
            ("B", "Bold", Box::new(|_| {})),
            ("I", "Italic", Box::new(|_| {})),
            ("U", "Underline", Box::new(|_| {})),
        ];
        let extended: Vec<(&str, &str, Box<dyn Fn()>)> = vec![
            ("T", "Text Box", Box::new(|| {})),
            ("R", "Rectangle", Box::new(|| {})),
        ];

        let suite_win = suite_common::SuiteWindow::new(app, "Decks", primary, extended);
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
