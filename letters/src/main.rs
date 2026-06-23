use gtk4::prelude::*;
mod window;
mod engine;

fn main() {
    let app = suite_common::make_app("org.tunaos.letters-rust");
    app.connect_activate(|app| {
        let w = window::LettersWindow::new(app);
        w.present();
    });
    app.run();
}
