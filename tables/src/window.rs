// window.rs — Tables with text grid (gtk4-rs ListView too complex for v0.9).
use gtk4 as gtk;
use gtk::prelude::*;

pub struct TablesWindow {
    window: gtk::ApplicationWindow,
}

impl TablesWindow {
    pub fn new(app: &gtk::Application) -> Self {
        let win = gtk::ApplicationWindow::builder().application(app).build();
        win.set_title(Some("Tables"));
        win.set_default_size(900, 600);

        let header = suite_common::make_header_bar();
        let toolbar = suite_common::make_toolbar();
        let formula = gtk::Entry::new();
        formula.set_placeholder_text(Some("Formula…"));

        // Simple text grid
        let grid = gtk::TextView::new();
        grid.set_monospace(true);
        grid.set_editable(true);
        let buf = grid.buffer();
        let mut text = String::new();
        for r in 0..30 {
            for c in 0..6 {
                if r == 0 { text.push_str(&(b'A' + c as u8).to_string()); }
                else { text.push_str(&format!("{},{}", r, c)); }
                if c < 5 { text.push('\t'); }
            }
            text.push('\n');
        }
        buf.set_text(&text);
        let scroll = gtk::ScrolledWindow::new();
        scroll.set_child(Some(&grid));
        scroll.set_vexpand(true);

        let main = gtk::Box::new(gtk::Orientation::Vertical, 2);
        main.append(&toolbar); main.append(&formula); main.append(&scroll);

        let container = gtk::Box::new(gtk::Orientation::Vertical, 0);
        container.append(&header); container.append(&main);
        win.set_child(Some(&container));
        Self { window: win }
    }
    pub fn present(&self) { self.window.present(); }
}
