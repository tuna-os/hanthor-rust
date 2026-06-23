use libadwaita as adw;
use gtk4::{self as gtk, gio, glib, prelude::*};
use adw::prelude::*;

// Number of columns and rows for the initial blank sheet.
const COLS: usize = 10;
const ROWS: usize = 50;

// A GObject wrapper for a single spreadsheet row (Vec of cell strings).
mod row_object {
    use gtk4::glib;
    use std::cell::RefCell;

    #[derive(Default)]
    pub struct RowData {
        pub cells: RefCell<Vec<String>>,
    }

    #[glib::object_subclass]
    impl glib::subclass::ObjectSubclass for RowData {
        const NAME: &'static str = "SpreadsheetRow";
        type Type = super::SpreadsheetRow;
        type ParentType = glib::Object;
    }

    impl glib::subclass::object::ObjectImpl for RowData {}
}

glib::wrapper! {
    pub struct SpreadsheetRow(ObjectSubclass<row_object::RowData>);
}

impl SpreadsheetRow {
    pub fn new(cells: Vec<String>) -> Self {
        use glib::subclass::prelude::ObjectSubclassIsExt;
        let obj: Self = glib::Object::new();
        obj.imp().cells.replace(cells);
        obj
    }
    pub fn get(&self, col: usize) -> String {
        use glib::subclass::prelude::ObjectSubclassIsExt;
        self.imp().cells.borrow().get(col).cloned().unwrap_or_default()
    }
}

pub struct TablesWindow {
    window: adw::ApplicationWindow,
}

impl TablesWindow {
    pub fn new(app: &adw::Application) -> Self {
        let win = adw::ApplicationWindow::builder()
            .application(app)
            .default_width(900)
            .default_height(600)
            .title("Tables")
            .build();

        // --- model ---
        let store = gio::ListStore::new::<SpreadsheetRow>();
        for r in 0..ROWS {
            let cells: Vec<String> = (0..COLS)
                .map(|c| format!("{}{}", (b'A' + c as u8) as char, r + 1))
                .collect();
            store.append(&SpreadsheetRow::new(cells));
        }
        let selection = gtk::NoSelection::new(Some(store));

        // --- column view ---
        let col_view = gtk::ColumnView::new(Some(selection));
        col_view.add_css_class("data-table");

        for c in 0..COLS {
            let col_header = format!("{}", (b'A' + c as u8) as char);
            let factory = gtk::SignalListItemFactory::new();
            factory.connect_setup(|_, item| {
                let item = item.downcast_ref::<gtk::ListItem>().unwrap();
                let label = gtk::Label::new(None);
                label.set_xalign(0.0);
                label.set_margin_start(4);
                label.set_margin_end(4);
                item.set_child(Some(&label));
            });
            factory.connect_bind(move |_, item| {
                let item = item.downcast_ref::<gtk::ListItem>().unwrap();
                let row = item.item().and_downcast::<SpreadsheetRow>().unwrap();
                let label = item.child().and_downcast::<gtk::Label>().unwrap();
                label.set_text(&row.get(c));
            });
            let column = gtk::ColumnViewColumn::new(Some(&col_header), Some(factory));
            column.set_fixed_width(80);
            col_view.append_column(&column);
        }

        let scroll = gtk::ScrolledWindow::new();
        scroll.set_child(Some(&col_view));
        scroll.set_vexpand(true);
        scroll.set_hexpand(true);

        // --- formula bar ---
        let formula = gtk::Entry::new();
        formula.set_placeholder_text(Some("Formula…"));
        formula.set_hexpand(true);

        let formula_row = gtk::Box::new(gtk::Orientation::Horizontal, 4);
        formula_row.set_margin_start(6);
        formula_row.set_margin_end(6);
        formula_row.set_margin_top(4);
        formula_row.set_margin_bottom(4);
        formula_row.append(&gtk::Label::new(Some("fx")));
        formula_row.append(&formula);

        let toolbar_box = gtk::Box::new(gtk::Orientation::Horizontal, 6);
        toolbar_box.set_margin_start(6);
        toolbar_box.set_margin_end(6);
        toolbar_box.append(&suite_common::make_toolbar());

        let toolbar_view = adw::ToolbarView::new();
        toolbar_view.add_top_bar(&suite_common::make_header_bar());
        toolbar_view.add_top_bar(&toolbar_box);
        toolbar_view.add_top_bar(&formula_row);
        toolbar_view.set_content(Some(&scroll));

        win.set_content(Some(&toolbar_view));
        Self { window: win }
    }

    pub fn present(&self) {
        self.window.present();
    }
}
