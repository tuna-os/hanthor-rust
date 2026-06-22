use libadwaita as adw;
use gtk4::prelude::*;
pub fn make_app(id: &str) -> gtk4::Application {
    gtk4::Application::new(Some(id), gtk4::gio::ApplicationFlags::empty())
}
pub fn make_header_bar() -> adw::HeaderBar {
    let h = adw::HeaderBar::new();
    h.pack_start(&gtk4::Button::with_label("Open"));
    h.pack_end(&gtk4::Button::with_label("Save"));
    h
}
pub fn make_toolbar() -> gtk4::Box {
    let t = gtk4::Box::new(gtk4::Orientation::Horizontal, 4);
    t.set_halign(gtk4::Align::Center);
    t.add_css_class("toolbar");
    for l in &["B","I","U"] { t.append(&gtk4::ToggleButton::with_label(l)); }
    t
}
