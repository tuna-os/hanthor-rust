use libadwaita as adw;
use gtk4::prelude::*;
use adw::prelude::*;

pub struct LettersWindow {
    window: adw::ApplicationWindow,
}

impl LettersWindow {
    pub fn new(app: &adw::Application) -> Self {
        let win = adw::ApplicationWindow::builder()
            .application(app)
            .default_width(900)
            .default_height(600)
            .title("Letters")
            .build();

        let editor = gtk4::TextView::new();
        editor.set_vexpand(true);
        editor.set_hexpand(true);
        editor.set_wrap_mode(gtk4::WrapMode::Word);
        editor.set_left_margin(24);
        editor.set_right_margin(24);
        editor.set_top_margin(16);
        editor.set_bottom_margin(16);

        let scroll = gtk4::ScrolledWindow::new();
        scroll.set_child(Some(&editor));
        scroll.set_vexpand(true);

        let toolbar_box = gtk4::Box::new(gtk4::Orientation::Horizontal, 6);
        toolbar_box.set_margin_start(6);
        toolbar_box.set_margin_end(6);
        toolbar_box.append(&suite_common::make_toolbar());

        let toolbar_view = adw::ToolbarView::new();
        toolbar_view.add_top_bar(&suite_common::make_header_bar());
        toolbar_view.add_top_bar(&toolbar_box);
        toolbar_view.set_content(Some(&scroll));

        win.set_content(Some(&toolbar_view));
        Self { window: win }
    }

    pub fn present(&self) {
        self.window.present();
    }
}
