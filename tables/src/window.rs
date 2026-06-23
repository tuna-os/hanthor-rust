// SPDX-License-Identifier: GPL-3.0-or-later
use libadwaita as adw;
use adw::prelude::*;
use gtk4::{self as gtk, gio, prelude::*};

const ROWS: usize = 100;
const COLS: usize = 26;

pub struct TablesWindow {
    window: adw::ApplicationWindow,
}

impl TablesWindow {
    pub fn new(app: &adw::Application) -> Self {
        let win = adw::ApplicationWindow::builder()
            .application(app)
            .default_width(900)
            .default_height(600)
            .title("Tables")
            .build();

        // Model: one Object per row (position used to derive row number)
        let model = gio::ListStore::new::<gtk::glib::Object>();
        for _ in 0..ROWS {
            model.append(&gtk::glib::Object::new::<gtk::glib::Object>());
        }

        let factory = gtk::SignalListItemFactory::new();
        factory.connect_setup(|_, item| {
            let row = gtk::Box::new(gtk::Orientation::Horizontal, 0);
            // row-number label
            let num = gtk::Label::new(None);
            num.set_width_chars(4);
            num.set_xalign(1.0);
            num.add_css_class("dim-label");
            num.set_margin_end(4);
            row.append(&num);
            // cell entries
            for _ in 0..COLS {
                let e = gtk::Entry::new();
                e.set_has_frame(false);
                e.set_width_chars(10);
                row.append(&e);
            }
            item.downcast_ref::<gtk::ListItem>().unwrap().set_child(Some(&row));
        });
        factory.connect_bind(|_, item| {
            let li = item.downcast_ref::<gtk::ListItem>().unwrap();
            let r = li.position() as usize;
            let row = li.child().unwrap().downcast::<gtk::Box>().unwrap();
            // update row number
            row.first_child().unwrap().downcast::<gtk::Label>().unwrap()
                .set_label(&(r + 1).to_string());
            // clear/header cells
            let mut node = row.first_child().and_then(|n| n.next_sibling());
            let mut c = 0usize;
            while let Some(w) = node {
                if let Ok(e) = w.clone().downcast::<gtk::Entry>() {
                    e.set_text(if r == 0 { &((b'A' + c as u8) as char).to_string() } else { "" });
                }
                node = w.next_sibling();
                c += 1;
            }
        });

        let list = gtk::ListView::new(
            Some(gtk::NoSelection::new(Some(model))),
            Some(factory),
        );
        let scroll = gtk::ScrolledWindow::new();
        scroll.set_child(Some(&list));
        scroll.set_vexpand(true);
        scroll.set_hexpand(true);

        // formula bar
        let fx_bar = gtk::Box::new(gtk::Orientation::Horizontal, 6);
        fx_bar.set_margin_start(6);
        fx_bar.set_margin_end(6);
        fx_bar.set_margin_top(2);
        fx_bar.set_margin_bottom(2);
        let fx_label = gtk::Label::new(Some("fx"));
        fx_label.add_css_class("dim-label");
        let fx_entry = gtk::Entry::new();
        fx_entry.set_placeholder_text(Some("Formula…"));
        fx_entry.set_hexpand(true);
        fx_bar.append(&fx_label);
        fx_bar.append(&fx_entry);

        let toolbar_box = gtk::Box::new(gtk::Orientation::Horizontal, 6);
        toolbar_box.set_margin_start(6);
        toolbar_box.set_margin_end(6);
        toolbar_box.append(&suite_common::make_toolbar());

        let toolbar_view = adw::ToolbarView::new();
        toolbar_view.add_top_bar(&suite_common::make_header_bar());
        toolbar_view.add_top_bar(&toolbar_box);
        toolbar_view.add_top_bar(&fx_bar);
        toolbar_view.set_content(Some(&scroll));

        win.set_content(Some(&toolbar_view));
        Self { window: win }
    }

    pub fn present(&self) {
        self.window.present();
    }
}
