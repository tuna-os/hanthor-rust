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

        // Tabs
        let notebook = gtk::Notebook::new();
        let tab1 = gtk::Label::new(Some("📄 Tab 1"));
        let editor = gtk::TextView::new();
        editor.set_monospace(true); editor.set_wrap_mode(gtk::WrapMode::Word);
        let s = gtk::ScrolledWindow::new(); s.set_child(Some(&editor)); s.set_vexpand(true);
        notebook.append_page(&s, Some(&tab1));

        // Word count status bar
        let status = gtk::Label::new(Some("0 words"));
        status.set_halign(gtk::Align::End);

        let m = gtk::Box::new(gtk::Orientation::Vertical, 2);
        m.append(&toolbar); m.append(&notebook); m.append(&status);
        let c = gtk::Box::new(gtk::Orientation::Vertical, 0);
        c.append(&header); c.append(&m);
        win.set_child(Some(&c));
        Self { window: win }
    }
    pub fn present(&self) { self.window.present(); }
}
