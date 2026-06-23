# GNOME GUI Reference for Hanthor Rust Office Suite

> **Purpose:** Agent-facing reference doc documenting GNOME Rust reference apps + gnome-gui-spec patterns.
> Cloned local repos: `/tmp/loupe`, `/tmp/gnome-tour`, `/tmp/papers`

---

## GNOME Rust Apps — Reference Implementations

These GNOME apps are written in Rust with gtk4-rs + libadwaita and serve as
canonical reference implementations for patterns, idioms, and architecture.

### 1. Loupe (Image Viewer) — `/tmp/loupe`

| Aspect | Details |
|--------|---------|
| **ID** | `org.gnome.Loupe` |
| **Stack** | Rust + gtk4-rs + libadwaita + Cairo + glycin (image decoder) |
| **Pattern** | `AdwApplicationWindow` → `AdwViewStack` (ImageWindow/EditWindow) |
| **Key files** | `src/widgets/window.rs`, `src/widgets/image_window.rs`, `src/widgets/image_window/actions.rs`, `src/widgets/image_view.rs` |
| **UI format** | GtkBuilder XML `.ui` + `CompositeTemplate` derive macro |
| **Unique features** | Subclass pattern (`#[derive(gtk::CompositeTemplate, glib::Properties)]`), `adw::subclass::prelude::*` |

**Key architecture:**
```
src/
├── main.rs                  # fn main() → calls loupe::main()
├── application.rs           # LpApplication (AdwApplication subclass)
├── widgets/
│   ├── window.rs            # LpWindow — top-level window with AdwViewStack
│   ├── image_window.rs      # LpImageWindow — image display + controls
│   ├── image_window/        # actions.rs, controls.rs
│   ├── image_view.rs        # LpImageView — scrollable/zoomable view
│   ├── image.rs             # Image widget with rendering, zoom, pan, rotation
│   ├── edit_window.rs       # Edit window for cropping/editing
│   ├── edit/                # crop.rs, crop_selection.rs
│   ├── shy_bin.rs           # Auto-hides in fullscreen
│   ├── sliding_view.rs      # Sliding/swipe gesture
│   ├── properties_view.rs   # Metadata properties
│   └── print.rs, print_preview.rs
├── deps.rs                  # Re-exports gtk/glib/adw (thin wrapper)
└── util/                    # gettext.rs, root.rs, formatting.rs
```

**Pattern to copy:** Subclass-based widget hierarchy with `CompositeTemplate`, property
bindings via `#[glib::Properties]`, and modular action handlers.

### 2. GNOME Tour — `/tmp/gnome-tour`

| Aspect | Details |
|--------|---------|
| **ID** | `org.gnome.Tour` |
| **Stack** | Rust + gtk4-rs + libadwaita |
| **Pattern** | `AdwApplicationWindow` → `AdwToolbarView` → `AdwCarousel` + overlay buttons |
| **Key files** | `src/widgets/paginator.rs`, `src/widgets/image_page.rs` |
| **UI format** | GtkBuilder XML `.ui` |

**Key patterns:**
- `AdwCarouselIndicatorDots` as `title-widget` in HeaderBar
- Overlay Previous/Next buttons with `style class="circular"` and `margin-start: 12`
- Suggested-action `.circular` button for final "Start" action
- ImagePageWidget with `AdwBin` subclass

### 3. Papers (Document Viewer) — `/tmp/papers`

| Aspect | Details |
|--------|---------|
| **ID** | `org.gnome.Papers` (successor to Evince) |
| **Stack** | Rust + C hybrid workspace; gtk4-rs + libadwaita shell; C document engine |
| **Workspace** | 6 crates: `papers-view`, `papers-view-sys`, `papers-document`, `papers-document-sys`, `shell` (Rust), `thumbnailer` |
| **Key Rust shell files** | `shell/src/window.rs`, `shell/src/application.rs`, `shell/src/document_view.rs`, `shell/src/sidebar.rs`, `shell/src/find_sidebar.rs`, `shell/src/search_box.rs` |
| **Pattern** | `AdwApplicationWindow` → tabbed sidebar + document view + find sidebar |

**Key architecture:**
```
shell/src/
├── main.rs                   # Entry point
├── application.rs            # Application setup + actions
├── window.rs                 # Main window — navigation + tabs
├── document_view.rs          # Document view widget
│   ├── actions.rs            # Toolbar/document actions
│   ├── io.rs                 # Open/save/export
│   ├── print.rs              # Print support
│   └── signals.rs            # Signal handlers
├── sidebar.rs                # Sidebar panel + thumbnails/links/annotations/layers
├── sidebar_thumbnails.rs
├── sidebar_links.rs
├── sidebar_layers.rs
├── sidebar_annotations.rs    # Annotation sidebar
├── sidebar_attachments.rs    # Embedded attachments panel
├── find_sidebar.rs           # Find-in-document panel
├── search_box.rs             # Search widget
├── page_selector.rs          # Page number navigation
├── properties_window.rs      # Document properties
├── password_view.rs          # Password prompt for encrypted PDFs
├── loader_view.rs            # Loading overlay
├── file_monitor.rs           # Auto-reload on file change
└── deps.rs                   # Crate dependency re-exports
```

---

## gnome-gui-spec — Key Patterns for Office Suite

Saved at `/tmp/gnome-gui-spec/`. Key takeaways for hanthor-rust:

### Window Architecture
```
AdwApplicationWindow
├── AdwBreakpoint (adaptive)
├── AdwToolbarView
│   ├── AdwHeaderBar [start: primary actions] [center: title] [end: menu]
│   ├── [content] AdwTabView / AdwOverlaySplitView / GtkStack
│   └── [bottom] AdwViewSwitcherBar / statusbar
└── AdwToastOverlay
```

### Design Tokens
- **Spacing:** 6 (default row), 12 (default container), 18 (wide), 24 (section)
- **Margins:** 6 (default row), 12 (default container), 18 (card), 24 (section)
- **Typography:** System font (Adwaita Sans), `.dim-label` for secondary text

### Key Patterns (from INTENT-MAP.md)

| Intent | Widget | Best Reference |
|--------|--------|----------------|
| Tabbed documents | `AdwTabView` + `AdwTabBar` | GNOME Text Editor |
| Window + sidebar | `AdwOverlaySplitView` | GNOME Disk Utility |
| Preferences | `AdwPreferencesDialog` + `GSettings` | All GNOME apps |
| Empty state | `AdwStatusPage` | Decibels, Loupe |
| Toast feedback | `AdwToast` + `AdwToastOverlay` | All GNOME apps |
| Keyboard shortcuts | `AdwShortcutsDialog` | All GNOME apps |
| Alert dialogs | `AdwAlertDialog` | All GNOME apps |
| About dialog | `AdwAboutDialog` | All GNOME apps |
| Adaptive breakpoints | `AdwBreakpoint` | Loupe, Disk Utility |
| Undo with toast | `AdwToast` with undo button | Text Editor |

### Audit Score Gaps (from AUDIT-GNOME-GUI-SPEC.md files)
- **Preferences:** Missing GSettings backend (6/7 — need schema + binding)
- **Empty state:** No AdwStatusPage for fresh/empty documents
- **Accessibility:** WebKit/canvas internals not bridged to AT-SPI

---

## Rust-Specific GNOME Patterns

### Widget Subclass Pattern (from Loupe)
```rust
use adw::subclass::prelude::*;
use gtk::CompositeTemplate;

#[derive(Default, CompositeTemplate, glib::Properties)]
#[template(file = "widget.ui")]
#[properties(wrapper_type = super::MyWidget)]
pub struct MyWidget {
    #[template_child]
    pub child_widget: TemplateChild<gtk::Button>,
}

#[glib::object_subclass]
impl ObjectSubclass for MyWidget { ... }

#[glib::derived_properties]
impl ObjectImpl for MyWidget { ... }

impl WidgetImpl for MyWidget { ... }
impl adw::ApplicationWindowImpl for MyWidget { ... }
```

### UI Template (GtkBuilder XML pattern)
Use `.ui` XML files with `CompositeTemplate` (as Loupe/Papers do) rather than
code-only builders. This matches the gnome-gui-spec component library which
provides Blueprint and XML examples.

---

## Cloned Local Repos (read-only references)

| App | Path | How to Read |
|-----|------|-------------|
| Loupe | `/tmp/loupe` | `read path="/tmp/loupe/src/widgets/window.rs"` |
| GNOME Tour | `/tmp/gnome-tour` | `read path="/tmp/gnome-tour/src/widgets/paginator.rs"` |
| Papers | `/tmp/papers` | `read path="/tmp/papers/shell/src/window.rs"` |
| gnome-gui-spec | `/tmp/gnome-gui-spec` | Start with `SKILL.md` then `INTENT-MAP.md` |
