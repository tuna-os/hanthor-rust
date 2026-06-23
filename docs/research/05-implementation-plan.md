# Implementation Plan — Letters Word Processor

> Phased plan with effort estimates, dependencies, and portable code sources.
> Goal: Make Letters a viable WYSIWYG word processor (MS Word / LibreOffice Writer / Google Docs alternative).
> Date: 2026-06-23

---

## Overview

| Phase | Deliverable | Effort | Depends On |
|-------|-------------|--------|------------|
| P1 — Quick Wins | Visual page look, rulers, print, page setup | 10-15d | Nothing |
| P2 — DOCX & Styles | Native DOCX R/W, style system, DOCX roundtrip | 10-17d | Phase 1 |
| P3 — Print Layout | Page breaks, headers/footers, print preview | 15-25d | Phase 2 |
| P4 — Polish | Columns, zoom, line spacing, tab stops | 10-15d | Phase 3 |
| **Total** | | **45-72d** | |

---

## Phase 1 — Quick Wins (10-15d)

> Goal: Letters looks and behaves like a word processor. No pagination yet — visual only.

### 1.1 Fix Spell-Check (1-2d)

**What:** Verify GtkTextView spell-check works and wire the right-click suggestion menu.

**How:**
- Check if `gspell` or `enchant2` is installed in the Flatpak runtime
- Ensure `set_enable_spell_checking(true)` is wired to a GSettings toggle (already done)
- Wire `set_extra_menu()` for right-click suggestions (add to dictionary, ignore)

**Portable code:** None — this is GTK API usage.

**Files to modify:** `letters/src/window.rs` (spell-check section)

**Verification:** Misspelled words show red underlines. Right-click shows suggestions.

---

### 1.2 Page Container — Visual Page Rectangle (2-3d)

**What:** Wrap the GtkTextView in a custom Cairo widget that draws a white page rectangle on a gray "desktop" background.

**How:**
- Create `letters/src/page_container.rs` — custom widget extending `gtk::Widget`
- Override `snapshot` to draw Cairo background:
  - Fill widget area with gray (#C0C0C0) — the "desktop"
  - Draw a white rectangle (#FFFFFF) at the page position with drop shadow
- Page size comes from GSettings (A4 default: 595x842 points at 72 DPI)
- Scale page to fit the widget width
- Page margins drawn as gray lines inside the white rectangle

**Portable code:**
- Loupe's `image_view.rs` — Cairo DrawingArea with gesture handling (reference for custom widget pattern)
- text-engine's `display.c` — Cairo rendering of document layout (concept reference)

**Files to create:** `letters/src/page_container.rs`  
**Files to modify:** `letters/src/window.rs` (wrap editor in page container)

**Verification:** Editor shows a white page on gray background. Margins are visible.

---

### 1.3 Rulers — Horizontal Ruler (3-5d)

**What:** Horizontal ruler above the editor showing margin markers, indent markers, and tab stops.

**How:**
- Create `letters/src/ruler.rs` — custom Cairo widget
- Draw:
  - Tick marks at regular intervals (inches or cm, configurable)
  - Left margin marker (draggable triangle)
  - Right margin marker (draggable triangle)
  - First-line indent marker (top triangle, draggable)
  - Left indent marker (bottom triangle, draggable)
  - Tab stop markers (L-shaped, click to add, drag to move, right-click to remove)
- Mouse interaction via `GestureClick` and `EventControllerMotion`
- Sync with GtkTextView's `left_margin`, `right_margin`, and paragraph indentation
- Measure text position using `pangocairo::functions::create_layout()` with the editor's PangoContext

**Portable code:**
- AbiWord's ruler widget — C++ concept reference (margins, indents, tabs on ruler)
- Rudra-Editor's `LayoutBlock::indent_left` / `indent_right` / `indent_first_line` — data model reference

**Files to create:** `letters/src/ruler.rs`  
**Files to modify:** `letters/src/window.rs` (add ruler above editor)

**Verification:** Ruler shows tick marks. Drag margin markers changes editor padding. Drag indent changes paragraph indents.

---

### 1.4 Page Setup Dialog (1-2d)

**What:** File → Page Setup dialog for paper size, orientation, margins.

**How:**
- Use `gtk::PageSetupUnixDialog` — standard GTK page setup dialog
- Wire to GSettings for persistence
- Apply page size to the page container widget

**Portable code:** None — standard GTK API.

**Files to modify:** `letters/src/window.rs` (add action + dialog)

**Verification:** Open Page Setup, change to US Letter Landscape, page container resizes.

---

### 1.5 Basic Print (2-3d)

**What:** File → Print using GtkPrintOperation, rendering directly from GtkTextView.

**How:**
- Create `gtk::PrintOperation`
- Connect `draw_page` signal — render the GtkTextView content to the print Cairo context
- Page ranges, copies, and collation handled by GtkPrintOperation

**Portable code:**
- GTK4 PrintOperation examples — `draw_page` signal with Cairo context
- GtkTextView → Cairo rendering pattern

**Files to modify:** `letters/src/window.rs` (add Print action + handler)  
**New dependency:** None (GtkPrintOperation is in gtk4)

**Verification:** Ctrl+P opens print dialog. Print to PDF file produces correct output.

---

## Phase 2 — DOCX & Styles (10-17d)

> Goal: Native DOCX roundtrip (no pandoc dependency). Named styles with inheritance.

### 2.1 Add rdocx Dependency + Bridge Layer (3-5d)

**What:** Add `rdocx` crate, build a bridge between GtkTextBuffer and rdocx's document model.

**How:**
- Add `rdocx = "0.x"` to `letters/Cargo.toml` (pin version)
- Create `letters/src/docx_bridge.rs`:
  - `textbuffer_to_rdocx(buf: &TextBuffer) -> rdocx::Document` — walks TextTags and converts to rdocx paragraphs/runs/styles
  - `rdocx_to_textbuffer(doc: &rdocx::Document) -> TextBuffer` — creates a new buffer with TextTags from rdocx styles
- Map TextTag names to rdocx style names (bold ↔ Bold, h1 ↔ Heading 1, etc.)
- Handle images: extract from rdocx document, save to temp, insert as Markdown `![alt](path)`

**Portable code:**
- rdocx crate (direct dependency)
- Rudra-Editor's `s1-format-docx` — architecture reference for DOCX reader/writer structure
- Marko Editor's `textbuffermd.rs::assign_markdown()` — similar tag-mapping pattern

**Files to create:** `letters/src/docx_bridge.rs`  
**Files to modify:** `letters/Cargo.toml`, `letters/src/engine.rs` (replace pandoc)

**Verification:** Open .docx file → content appears with formatting preserved. Save as .docx → reopen → content matches.

---

### 2.2 Replace pandoc/typst with Native DOCX I/O (2-3d)

**What:** Remove pandoc CLI dependency from `engine.rs`. Use rdocx for .docx read/write.

**How:**
- Modify `engine.rs::read()` — use `docx_bridge.rs` for .docx files instead of pandoc
- Modify `engine.rs::write()` — use `docx_bridge.rs` for .docx output
- Keep pandoc as optional fallback for .odt/.rtf export if needed
- Replace typst PDF export with rdocx-layout PDF export (or keep typst as fallback)

**Portable code:** rdocx crate (direct dependency)

**Files to modify:** `letters/src/engine.rs`

**Verification:** `cargo test` — existing docx roundtrip test passes without pandoc.

---

### 2.3 Styles System (3-5d)

**What:** Named styles with inheritance. Style panel in sidebar or dialog.

**How:**
- Create `letters/src/styles.rs`:
  - `Style` struct: name, base_style (Option<String>), font, size, weight, color, spacing, alignment
  - `StyleSheet` — collection of named styles with inheritance resolution
  - Default styles: Normal, Heading 1-6, Title, Subtitle, Code, Blockquote
- `apply_style(buf: &TextBuffer, style_name: &str)` — applies TextTags based on resolved Style
- Style modification: when a style is changed, update all TextTags with that name → all text updates
- Style UI: dropdown in toolbar with style preview (rendered text showing font/size/weight)

**Portable code:**
- Rudra-Editor's `s1-model/src/styles.rs` — Style struct with base_style inheritance (AGPL — concept only)
- LibreOffice Writer's style system concept (Heading 1 based on Heading, based on Normal)

**Files to create:** `letters/src/styles.rs`  
**Files to modify:** `letters/src/window.rs` (add style dropdown to toolbar)

**Verification:** Select "Heading 1" from style dropdown → text becomes large bold. Modify Heading 1 style → all H1 text updates.

---

### 2.4 DOCX Style Roundtrip (2-4d)

**What:** Preserve styles when reading/writing DOCX.

**How:**
- Extend `docx_bridge.rs` to map rdocx styles ↔ Letters styles
- On read: rdocx paragraph/character styles → Letters StyleSheet
- On write: Letters StyleSheet → rdocx styles
- Handle style inheritance (basedOn in DOCX)
- Preserve custom styles from imported documents

**Portable code:** rdocx crate — style handling. Rudra-Editor's `s1-format-docx/src/style_parser.rs` / `style_writer.rs` — reference.

**Files to modify:** `letters/src/docx_bridge.rs`, `letters/src/styles.rs`

**Verification:** Open .docx with custom styles → styles appear in Letters. Save → reopen in Word → styles preserved.

---

## Phase 3 — Print Layout (15-25d)

> Goal: Real pagination, page breaks, headers/footers, print preview.

### 3.1 Layout Engine Integration (5-7d)

**What:** Integrate rdocx-layout (or Pango-based layout) for document pagination.

**How:**
- Create `letters/src/layout.rs`:
  - `paginate(buf: &TextBuffer, page_width: f64, page_height: f64, margins: Margins) -> Vec<Page>`
  - Each `Page` has: page_index, content_area (Rect), blocks (paragraph positions)
  - Runs asynchronously via `glib::idle_add()` or debounced timer (300ms after last edit)
- For Pango-based approach:
  - Create a PangoLayout from the text buffer's content
  - Measure lines, accumulate height, insert page breaks at page boundaries
  - Track which TextIter positions correspond to page breaks
- For rdocx-layout approach:
  - Serialize buffer → rdocx DocumentModel → rdocx-layout → positioned pages
  - Map page/block positions back to TextIter positions in the buffer

**Portable code:**
- rdocx-layout crate (direct dependency if using rdocx approach)
- Rudra-Editor's `s1-layout/src/engine.rs` — layout pipeline reference
- text-engine's `layout.c` — Pango-based layout reference

**Files to create:** `letters/src/layout.rs`  
**Files to modify:** `letters/Cargo.toml` (add rdocx-layout if using)

**Verification:** Run paginate on a multi-page document → returns correct page boundaries. Timings: <100ms for 50-page doc.

---

### 3.2 Page Break Visualization (3-4d)

**What:** Show page breaks in the editor as visual separators.

**How:**
- After layout engine runs, insert invisible markers at page break positions in the GtkTextBuffer
- Render page breaks as dotted/dashed lines or page gaps in the page container widget
- Show page numbers at break positions (e.g., "Page 2" centered in the gap)
- Page breaks update on a debounced timer after edits

**Portable code:**
- Rudra-Editor's `LayoutPage` — page boundary concept
- Google Docs / Word page break visualization (UX reference)

**Files to modify:** `letters/src/layout.rs`, `letters/src/page_container.rs`

**Verification:** Document longer than one page shows page breaks. Editing text shifts page breaks.

---

### 3.3 Headers & Footers (4-6d)

**What:** Editable headers and footers per section.

**How:**
- Add header/footer GtkTextViews above and below the body TextView in the page container
- Store header/footer content as separate TextBuffers (one per section)
- Toggle header/footer visibility via View menu or double-click header/footer area
- Page number field: insert special marker `{page}` that renders as current page number
- Different first page header/footer option

**Portable code:**
- Rudra-Editor's `SectionProperties::HeaderFooterRef` — data model reference
- rdocx-layout — header/footer positioning in page layout

**Files to create:** `letters/src/headers.rs`  
**Files to modify:** `letters/src/page_container.rs`, `letters/src/layout.rs`

**Verification:** Double-click top of page → header editing area appears. Type header text → appears on all pages. Close header → returns to body editing.

---

### 3.4 Print Preview (3-5d)

**What:** Show a print preview that uses the same layout engine to render pages as they'll print.

**How:**
- Create `letters/src/print_preview.rs`:
  - Custom window or dialog showing document pages rendered via Cairo
  - Uses the same `paginate()` function from the layout engine
  - Each page rendered with headers/footers/page numbers
  - Thumbnail sidebar or page navigation
- Print from preview → sends to GtkPrintOperation

**Portable code:**
- rdocx-layout PDF export — same rendering could be used for screen preview

**Files to create:** `letters/src/print_preview.rs`  
**Files to modify:** `letters/src/window.rs` (add Print Preview action)

**Verification:** File → Print Preview → shows document as pages. Matches printed output.

---

### 3.5 DOCX Roundtrip with Layout Features (2-3d)

**What:** Ensure headers/footers, page setup, and section properties roundtrip through DOCX.

**How:**
- Extend `docx_bridge.rs` to read/write:
  - Section properties (page size, margins, orientation)
  - Headers/footers (per section, different first page)
  - Page breaks
- Test roundtrip with Word-created documents containing these features

**Portable code:** rdocx crate. Rudra-Editor's `s1-format-docx/src/header_footer_parser.rs` / `header_footer_writer.rs` — reference.

**Files to modify:** `letters/src/docx_bridge.rs`

**Verification:** Open Word document with headers/footers → they appear in Letters. Save → reopen in Word → preserved.

---

## Phase 4 — Polish (10-15d)

> Goal: Columns, zoom, line spacing, tab stops — the features that make it feel complete.

### 4.1 Columns (5-7d)

**What:** Multi-column section layout.

**How:**
- Extend layout engine to support column count per section
- Text flows: column 1 → column 2 → column 3 → next page
- Column gap and column separator line options
- Requires Phase 3 layout engine to be complete

**Portable code:** rdocx-layout (may support columns via DOCX section properties). Rudra-Editor — check if column support exists.

**Files to modify:** `letters/src/layout.rs`, `letters/src/page_container.rs`

---

### 4.2 Zoom (1-2d)

**What:** Zoom in/out of the document (50%-200%).

**How:**
- Scale the page container widget using `gtk::Widget::set_overflow()` or CSS transform
- Or render via Cairo with a scale factor
- Zoom slider in status bar or Ctrl+Scroll

**Portable code:** Loupe's zoom implementation — Cairo scale factor.

**Files to modify:** `letters/src/page_container.rs`, `letters/src/window.rs`

---

### 4.3 Line Spacing & Paragraph Spacing (2-3d)

**What:** Control line spacing (1.0, 1.15, 1.5, 2.0) and paragraph before/after spacing.

**How:**
- GtkTextView supports `pixels_above_lines`, `pixels_below_lines`, `pixels_inside_wrap` via TextTag
- Apply these as TextTag properties
- Toolbar dropdown or paragraph dialog for spacing control

**Portable code:** None — GtkTextTag API.

**Files to modify:** `letters/src/window.rs` (formatting toolbar)

---

### 4.4 Tab Stops (2-3d)

**What:** Configurable tab stops on the ruler.

**How:**
- GtkTextBuffer supports `GtkTextTag::tabs()` — set tab stops per paragraph
- Ruler interaction: click to add tab stop, drag to move, right-click to remove
- Tab types: left, center, right, decimal
- Sync ruler tab stops with TextTag tab stops

**Portable code:** AbiWord's ruler widget (tab stop interaction reference).

**Files to modify:** `letters/src/ruler.rs`, `letters/src/window.rs`

---

## Dependencies Map

```
Phase 1 (Quick Wins)
├── 1.1 Spell-Check           (no deps)
├── 1.2 Page Container         (no deps)
├── 1.3 Rulers                 (no deps)
├── 1.4 Page Setup             (no deps)
└── 1.5 Basic Print            (no deps)

Phase 2 (DOCX & Styles)
├── 2.1 rdocx Bridge           (no deps — new crate dependency)
├── 2.2 Native DOCX I/O        (depends on 2.1)
├── 2.3 Styles System          (no deps)
└── 2.4 DOCX Style Roundtrip   (depends on 2.1, 2.3)

Phase 3 (Print Layout)
├── 3.1 Layout Engine          (depends on 2.1 for rdocx approach)
├── 3.2 Page Break Viz         (depends on 3.1)
├── 3.3 Headers/Footers        (depends on 3.1)
├── 3.4 Print Preview          (depends on 3.1, 3.3)
└── 3.5 DOCX Layout Roundtrip (depends on 2.1, 3.1, 3.3)

Phase 4 (Polish)
├── 4.1 Columns                (depends on 3.1)
├── 4.2 Zoom                   (depends on 1.2)
├── 4.3 Line Spacing           (no deps)
└── 4.4 Tab Stops              (depends on 1.3)
```

---

## Portable Code Sources Summary

| Source | License | What to Port | Phase |
|--------|---------|-------------|-------|
| **rdocx** crate | MIT/Apache-2.0 | Direct dependency — DOCX R/W, layout engine, PDF export | P2, P3 |
| **rdocx-layout** crate | MIT/Apache-2.0 | Direct dependency — pagination, text shaping, font subsetting | P3 |
| Rudra-Editor `s1-model` | AGPL-3.0 (concept only) | Document model design: Node/NodeId/AttributeMap, SectionProperties, Style inheritance | P2 design |
| Rudra-Editor `s1-layout` | AGPL-3.0 (concept only) | Layout pipeline: style resolution → shaping → line breaking → stacking → pagination | P3 design |
| Rudra-Editor `s1-format-docx` | AGPL-3.0 (concept only) | DOCX parser/writer structure: content, styles, numbering, headers/footers | P2 design |
| text-engine `layout.*` | MPL-2.0 (concept only) | Pango-based layout measurement, box model for page layout | P3 design |
| text-engine `model/*` | MPL-2.0 (concept only) | Model/view/layout separation, document hierarchy | Architecture design |
| AbiWord ruler | GPL-2.0 (concept only) | Ruler widget: margin markers, indent markers, tab stops, drag interaction | P1 design |
| Loupe `image_view.rs` | GPL-3.0 (concept only) | Cairo DrawingArea pattern, gesture handling, zoom | P1 design |
| Marko Editor `textbuffermd.rs` | Unknown | Markdown ↔ TextTag bidirectional conversion pattern | Architecture reference |
| GtkSourceView5 | LGPL-2.1+ | Dependency candidate — gutter, search highlighting | P4 optional |
