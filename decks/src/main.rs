use gtk4::prelude::*;
mod window;
mod engine;
fn main() {
    let app = suite_common::make_app("org.tunaos.decks-rust");
    app.connect_activate(|app| { let w = window::DecksWindow::new(app); w.present(); });
    app.run();
}
