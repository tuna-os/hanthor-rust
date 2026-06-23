use gtk4::prelude::*;
mod engine;
mod export;
mod window;

fn main() {
    let app = suite_common::make_app("org.tunaos.tables-rust");
    app.connect_activate(|app| {
        let win = window::TablesWindow::new(app);
        win.present();
    });
    app.run();
}
