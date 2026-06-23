use gtk4::prelude::*;
mod engine;
mod export;
mod charts;
mod window;

fn main() {
    let suite = suite_common::SuiteApp::new("org.tunaos.tables-rust");
    suite.app.connect_activate(|app| {
        let win = window::TablesWindow::new(app);
        win.present();
    });
    suite.run();
}
