use gtk4::prelude::*;
mod engine;
mod export;
mod charts;
mod window;
mod undo;

fn main() {
    let suite = suite_common::SuiteApp::new("org.tunaos.tables-rust");
    let shortcuts: &[(&str, &[(&str, &str)])] = &[
        ("Editing", &[
            ("<Control>z", "Undo"),
            ("<Control><Shift>z", "Redo"),
            ("Delete", "Clear cell"),
            ("Enter", "Commit cell edit"),
        ]),
        ("File", &[
            ("<Control>s", "Save"),
            ("<Control>o", "Open"),
            ("<Control>n", "New spreadsheet"),
        ]),
    ];
    let act_shortcuts = gtk4::gio::SimpleAction::new("show-shortcuts", None);
    let s = shortcuts;
    act_shortcuts.connect_activate(move |_, _| {
        suite_common::show_shortcuts_dialog(s);
    });
    suite.app.add_action(&act_shortcuts);
    suite.app.set_accels_for_action("app.show-shortcuts", &["<Primary>question"]);
    suite.app.connect_activate(|app| {
        let win = window::TablesWindow::new(app);
        win.present();
    });
    suite.run();
}
