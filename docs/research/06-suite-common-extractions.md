# Suite-Common Extractions — Cross-App Cohesion

> Components and patterns that should move from Letters into suite-common for Tables and Decks reuse.
> Date: 2026-06-23

---

## Current suite-common (baseline)

The current `suite-common` crate provides:

| Component | Used By | Description |
|-----------|---------|-------------|
| `SuiteApp` | Letters, Tables, Decks | Application scaffolding — Gio actions, accelerators, dark mode, GSettings |
| `SuiteWindow` | Letters, Tables, Decks | Window chrome — HeaderBar, ToolbarView, SuiteToolbar |
| `SuiteToolbar` | Letters | Responsive formatting toolbar (primary/extended) |
| `SuiteTabView` | Not used | Tabbed document wrapper (created but unused) |
| `make_header_bar()` | SuiteWindow (internal) | Standard header bar with New, Dark toggle, Menu |
| `make_empty_state()` | Letters, Tables, Decks | AdwStatusPage for empty document state |
| `make_status_bar()` | Letters | Bottom status bar with word count label |
| `make_preferences_window()` | Available | Preferences dialog builder |
| `make_preferences_page()` | Available | Preferences page builder |
| `make_preferences_group()` | Available | Preferences group builder |
| `show_shortcuts_dialog()` | Available | Keyboard shortcuts dialog |
| `show_about_dialog()` | SuiteApp (internal) | About dialog |

---

## Extraction Candidates

### 1. File Dialog Helpers — Open/Save File Pickers

**Currently in:** `letters/src/window.rs` (do_open, do_save, do_save_as functions — ~200 lines)  
**Used by Letters only.** Tables and Decks have their own file dialog code.

**What to extract:**
```rust
// suite-common
pub struct FileDialogHelper {
    parent: adw::ApplicationWindow,
}
impl FileDialogHelper {
    /// Show open dialog with file filters, return selected path
    pub fn open(&self, filters: &[FileFilter]) -> Option<PathBuf>;
    /// Show save dialog, return selected path
    pub fn save(&self, suggested_name: &str, filters: &[FileFilter]) -> Option<PathBuf>;
    /// Show save dialog for export (different path, doesn't update current file)
    pub fn export(&self, suggested_name: &str, filters: &[FileFilter]) -> Option<PathBuf>;
}

/// Standard file filters for office formats
pub fn docx_filter() -> gtk::FileFilter;
pub fn md_filter() -> gtk::FileFilter;
pub fn pdf_filter() -> gtk::FileFilter;
pub fn odt_filter() -> gtk::FileFilter;
```

**Rationale:** All three apps need Open/Save/Export dialogs with office format filters. Letters' implementation is the most complete (includes unsaved change handling, recent files). Extracting this gives Tables and Decks the same UX.

---

### 2. Toast Notification System

**Currently in:** `letters/src/window.rs` (AdwToastOverlay + toast helpers — ~30 lines)  
**Used by Letters only.** Tables has a comment about it ("toast via overlay — suite_common doesn't expose it directly").

**What to extract:**
```rust
// suite-common
pub struct ToastManager {
    overlay: adw::ToastOverlay,
}
impl ToastManager {
    pub fn new(widget: &impl IsA<gtk::Widget>) -> Self;
    pub fn show_toast(&self, message: &str);
    pub fn show_saved_toast(&self, file_name: &str);
    pub fn show_error_toast(&self, message: &str);
    pub fn widget(&self) -> &adw::ToastOverlay;
}
```

**Rationale:** All three apps need toast notifications (save confirmation, errors, auto-save). Tables explicitly wants this. A shared ToastManager reduces duplication.

---

### 3. Auto-Save Timer Infrastructure

**Currently in:** `letters/src/window.rs` (glib timeout + GSettings integration — ~40 lines)  
**Used by Letters only.**

**What to extract:**
```rust
// suite-common
pub struct AutoSaveTimer {
    source_id: Cell<Option<glib::SourceId>>,
}
impl AutoSaveTimer {
    pub fn new(settings: &gio::Settings, save_fn: impl Fn() + 'static) -> Self;
    pub fn reset(&self);       // Reset timer (called after manual save)
    pub fn cancel(&self);      // Stop auto-save
    pub fn restart(&self);     // Restart auto-save
}
```

**Rationale:** Tables will want auto-save. Decks will want auto-save. The timer pattern is identical in all cases. Extract it with a callback for the app-specific save logic.

---

### 4. Find/Replace Banner Widget

**Currently in:** `letters/src/window.rs` (make_find_replace_widget, FindState, navigate_match — ~250 lines)  
**Used by Letters only.**

**What to extract:**
```rust
// suite-common
pub struct FindReplaceBanner {
    revealer: gtk::Revealer,
    search_entry: gtk::SearchEntry,
    replace_entry: gtk::Entry,
    match_label: gtk::Label,
}
impl FindReplaceBanner {
    pub fn new(search_fn: impl Fn(&str, bool) -> usize + 'static,
               replace_fn: impl Fn(&str, &str) -> usize + 'static,
               navigate_fn: impl Fn(i32) + 'static) -> Self;
    pub fn show(&self);
    pub fn hide(&self);
    pub fn widget(&self) -> &gtk::Revealer;
}
```

**Rationale:** Tables needs find/replace in spreadsheets. Decks needs find/replace in slides. The banner UI (search entry, replace entry, match count, prev/next buttons, case-sensitive toggle) is identical. The search/replace logic is app-specific (TextBuffer search vs. cell search vs. slide search) and passed as closures.

**Note:** This is the largest extraction (~250 lines). Worth it because the UI widget is complex and Tables/Decks will need identical UI.

---

### 5. Unsaved Change Tracking

**Currently in:** `letters/src/window.rs` (needs_attention tracking, close-page confirmation, close-window confirmation — ~150 lines)  
**Used by Letters only.**

**What to extract:**
```rust
// suite-common
pub trait DirtyTracker {
    fn is_dirty(&self) -> bool;
    fn mark_clean(&self);
    fn show_save_discard_dialog(&self, title: &str, on_save: impl Fn(), on_discard: impl Fn());
}
```

**Rationale:** All three apps need unsaved change tracking. The "Save / Discard / Cancel" dialog pattern is identical. Extract as a trait that each app's document model implements.

---

### 6. Page/Print Infrastructure (Future)

**Currently in:** Not yet implemented (Phase 1 of plan).  
**Will be created in Letters.**

**What to extract (after Letters implements it):**
```rust
// suite-common
pub struct PageSetup {
    pub paper_size: (f64, f64),  // width, height in points
    pub orientation: PageOrientation,
    pub margins: Margins,
}
pub struct Margins { pub top: f64, pub bottom: f64, pub left: f64, pub right: f64; }
pub fn show_page_setup_dialog(parent: &gtk::Window, current: &PageSetup) -> Option<PageSetup>;
```

**Rationale:** Tables and Decks will need page setup for printing spreadsheets and slide handouts. The page setup dialog and data model should be shared.

---

### 7. Zoom Widget

**Currently in:** Not yet implemented (Phase 4).  
**Will be created in Letters.**

**What to extract (after Letters implements it):**
```rust
// suite-common
pub struct ZoomControl {
    slider: gtk::Scale,
}
impl ZoomControl {
    pub fn new(min: f64, max: f64, default: f64) -> Self;
    pub fn on_zoom_changed(&self, f: impl Fn(f64) + 'static);
    pub fn widget(&self) -> &gtk::Scale;
}
```

**Rationale:** Tables and Decks need zoom. Letters needs zoom. The zoom slider widget and Ctrl+Scroll binding should be shared.

---

## Extraction Priority

| Priority | Component | Effort | Used By | Rationale |
|----------|-----------|--------|---------|-----------|
| **P1** | File Dialog Helpers | 1-2d | L, T, D | All apps need file dialogs. Immediately reduces duplication. |
| **P1** | Toast Notification System | 0.5d | L, T, D | Trivial extraction. Tables explicitly wants this. |
| **P2** | Auto-Save Timer | 0.5d | L, T, D | Simple extraction. Low effort, high reuse. |
| **P2** | Unsaved Change Tracking | 2-3d | L, T, D | Significant effort to extract as a trait. High value for all apps. |
| **P3** | Find/Replace Banner | 2-3d | L, T, D | Large extraction. Worth it long-term but not urgent. |
| **P4** | Page Setup Infrastructure | N/A | L, T, D | Extract after Letters implements it (Phase 1-2). |
| **P4** | Zoom Widget | N/A | L, T, D | Extract after Letters implements it (Phase 4). |

---

## What to Keep App-Specific

Not everything should be in suite-common. These stay app-specific:

| Component | Why App-Specific |
|-----------|-----------------|
| **Formatting toolbar** (B/I/U/alignment) | Letters' toolbar is text-specific. Tables needs formula bar. Decks needs shape tools. |
| **Document model** | Each app has different data models (TextBuffer vs. SheetModel vs. SlideModel). |
| **File I/O engine** | Different formats per app (Markdown/DOCX vs. XLSX/CSV vs. PPTX). |
| **Layout engine** | Letters' pagination is different from Tables' grid rendering / Decks' slide layout. |
| **Status bar content** | Word count (Letters) vs. cell info (Tables) vs. slide count (Decks). |
| **Keyboard shortcuts** | Ctrl+B is Bold in Letters, could be something else in Tables. |

---

## Extraction Roadmap

```
Now (before Phase 1):
├── P1: File Dialog Helpers       → suite-common
└── P1: Toast Notification System → suite-common

During Phase 1:
├── P2: Auto-Save Timer           → suite-common
└── P2: Unsaved Change Tracking   → suite-common (trait)

During Phase 2:
└── P3: Find/Replace Banner       → suite-common

After Letters Phase 3-4:
├── P4: Page Setup Infrastructure  → suite-common
└── P4: Zoom Widget                → suite-common
```

## Impact on Tables and Decks

Once extractions are in suite-common:
- **Tables** gets: proper file dialogs, toast notifications, auto-save, unsaved change tracking, find/replace in spreadsheets
- **Decks** gets: proper file dialogs, toast notifications, auto-save, unsaved change tracking, find/replace in slides
- All three apps get consistent UX for file operations, save/discard flow, and notifications
