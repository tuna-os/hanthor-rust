use gtk4 as gtk;
use gtk::prelude::*;
pub struct LettersWindow { window: gtk::ApplicationWindow }
impl LettersWindow {
    pub fn new(app: &gtk::Application) -> Self {
        let win = gtk::ApplicationWindow::builder().application(app).build();
        win.set_title(Some("Letters"));
        win.set_default_size(800, 600);
        let header = suite_common::make_header_bar();
        let toolbar = suite_common::make_toolbar();
        let editor = gtk::TextView::new();
        editor.set_monospace(true); editor.set_wrap_mode(gtk::WrapMode::Word);
        let s = gtk::ScrolledWindow::new(); s.set_child(Some(&editor)); s.set_vexpand(true);
        let m = gtk::Box::new(gtk::Orientation::Vertical, 2);
        m.append(&toolbar); m.append(&s);
        let c = gtk::Box::new(gtk::Orientation::Vertical, 0);
        c.append(&header); c.append(&m);
        win.set_child(Some(&c));
        Self { window: win }
    }
    pub fn present(&self) { self.window.present(); }
}
