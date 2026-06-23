use libadwaita as adw;
use gtk4::prelude::*;
use adw::prelude::*;

pub struct DecksWindow {
    window: adw::ApplicationWindow,
}

impl DecksWindow {
    pub fn new(app: &adw::Application) -> Self {
        let win = adw::ApplicationWindow::builder()
            .application(app)
            .default_width(900)
            .default_height(600)
            .title("Decks")
            .build();

        let content = gtk4::Label::new(Some("Decks — slide canvas here"));
        content.set_vexpand(true);
        content.set_hexpand(true);

        let toolbar_box = gtk4::Box::new(gtk4::Orientation::Horizontal, 6);
        toolbar_box.set_margin_start(6);
        toolbar_box.set_margin_end(6);
        toolbar_box.append(&suite_common::make_toolbar());

        let toolbar_view = adw::ToolbarView::new();
        toolbar_view.add_top_bar(&suite_common::make_header_bar());
        toolbar_view.add_top_bar(&toolbar_box);
        toolbar_view.set_content(Some(&content));

        win.set_content(Some(&toolbar_view));
        Self { window: win }
    }

    pub fn present(&self) {
        self.window.present();
    }
}
