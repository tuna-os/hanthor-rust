use gtk4 as gtk;
use gtk::prelude::*;
pub struct DecksWindow { window: gtk::ApplicationWindow }
impl DecksWindow {
    pub fn new(app: &gtk::Application) -> Self {
        let win = gtk::ApplicationWindow::builder().application(app).build();
        win.set_title(Some("Decks"));
        win.set_default_size(960, 600);
        let header = suite_common::make_header_bar();
        let toolbar = suite_common::make_toolbar();
        let canvas = gtk::Label::new(Some("🎬 Slide canvas — Rust native"));
        canvas.set_vexpand(true);
        let m = gtk::Box::new(gtk::Orientation::Vertical, 2);
        m.append(&toolbar); m.append(&canvas);
        let c = gtk::Box::new(gtk::Orientation::Vertical, 0);
        c.append(&header); c.append(&m);
        win.set_child(Some(&c));
        Self { window: win }
    }
    pub fn present(&self) { self.window.present(); }
}
