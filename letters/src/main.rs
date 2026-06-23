use gtk4::prelude::*;
mod window;
mod engine;
mod export;

fn main() {
    let suite = suite_common::SuiteApp::new("org.tunaos.letters-rust");
    suite.app.connect_activate(|app| {
        let w = window::LettersWindow::new(app);
        w.present();
    });
    suite.run();
}
