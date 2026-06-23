use libadwaita as adw;
use gtk4::prelude::*;

/// Creates an adw::Application, which initialises libadwaita automatically.
pub fn make_app(id: &str) -> adw::Application {
    adw::Application::new(Some(id), gtk4::gio::ApplicationFlags::empty())
}

/// Standard header bar with a hamburger menu.
pub fn make_header_bar() -> adw::HeaderBar {
    let menu = gtk4::gio::Menu::new();
    menu.append(Some("Preferences"), Some("app.preferences"));
    menu.append(Some("Keyboard Shortcuts"), Some("app.shortcuts"));
    menu.append(Some("About"), Some("app.about"));
    let btn = gtk4::MenuButton::new();
    btn.set_icon_name("open-menu-symbolic");
    btn.set_menu_model(Some(&menu));
    let h = adw::HeaderBar::new();
    h.pack_end(&btn);
    h
}

/// Formatting toolbar with a linked button group.
pub fn make_toolbar() -> gtk4::Box {
    let t = gtk4::Box::new(gtk4::Orientation::Horizontal, 0);
    t.add_css_class("linked");
    for label in &["B", "I", "U"] {
        t.append(&gtk4::ToggleButton::with_label(label));
    }
    t
}
