// SPDX-License-Identifier: GPL-3.0-or-later
use libadwaita as adw;
use gtk4::prelude::*;

pub fn make_app(id: &str) -> adw::Application {
    adw::Application::builder().application_id(id).build()
}

pub fn make_header_bar() -> adw::HeaderBar {
    let menu = gtk4::gio::Menu::new();
    menu.append(Some("About"), Some("app.about"));
    let btn = gtk4::MenuButton::builder()
        .icon_name("open-menu-symbolic")
        .menu_model(&menu)
        .build();
    let h = adw::HeaderBar::new();
    h.pack_end(&btn);
    h
}

pub fn make_toolbar() -> gtk4::Box {
    let t = gtk4::Box::new(gtk4::Orientation::Horizontal, 0);
    t.add_css_class("linked");
    for label in &["B", "I", "U"] {
        t.append(&gtk4::ToggleButton::with_label(label));
    }
    t
}
