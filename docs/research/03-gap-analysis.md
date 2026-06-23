# Letters Feature Gap Analysis

> Core word processor target: WYSIWYG editing, page layout (margins/rulers/columns), styles, headers/footers, print/preview, DOCX roundtrip, spell-check.
> Date: 2026-06-23

---

## Current State Summary

Letters v1.0 is built on **GtkTextView** with **TextTags** for formatting and **pulldown-cmark** for Markdown import. It handles tabbed document editing, basic formatting (B/I/U/S/H, headings, code, blockquote), alignment, lists, find/replace, undo/redo, Markdown-based file I/O, and Markdown macros. It is a capable simple rich text editor but is **not** a word processor — it has no page awareness, no print layout, and no visual page boundaries.

---

## Feature Gap Table

### 1. Page Margins

| Aspect | Current State | Target State | Gap |
|--------|--------------|--------------|-----|
| Visual margins | `editor.set_left_margin(24)` etc. — widget padding, not page margins | Visible page edges with margin area shown as gray/white boundary | **Large** |
| Margin adjustment | Not adjustable by user | Draggable margin markers on ruler, or page setup dialog | **Large** |
| Margin in print | Pango layout margins applied via `GtkPrintOperation` | Consistent on-screen and print margins | **Medium** |

**Effort:** Medium (3-5d)  
**Approach:** Two levels:
1. **Simple:** Add a page setup dialog (GtkPageSetupUnixDialog) that sets margins. Apply margins as widget padding on screen, use GtkPrintOperation for print.
2. **Full:** Render a page background behind the GtkTextView (Cairo overlay or fixed container) showing the page boundaries with gray margin area outside.

**Research source:** GtkPageSetupUnixDialog API in gtk4-rs. rdocx-layout for consistent page model.

---

### 2. Rulers

| Aspect | Current State | Target State | Gap |
|--------|--------------|--------------|-----|
| Horizontal ruler | None | Horizontal ruler at top showing indents, tabs, margin markers | **Missing entirely** |
| Vertical ruler | None | Optional vertical ruler on left | **Missing entirely** |
| Drag interaction | None | Drag margin/tab/indent markers on ruler | **Missing entirely** |

**Effort:** Large (5-7d)  
**Approach:** Custom Cairo widget positioned above the editor. Must:
- Sync with GtkTextView's Pango layout for accurate measurements
- Handle mouse drag to adjust left/right indents, first-line indent, tab stops
- Update in real-time as cursor moves (paragraph-level indents change)
- Map ruler units to page units (inches, cm, points)

**Research source:** AbiWord's ruler widget (C, GPL-2.0 — architecture reference only). Pango layout measurement APIs. The ruler needs to query PangoLayout from the GtkTextView to get accurate text positions.

**Key challenge:** GtkTextView doesn't expose its PangoLayout directly. We need to either:
- Use `GtkTextView::create_pango_layout()` to create a parallel layout for measurement
- Use Pango directly from the Cairo drawing context via `pangocairo`

---

### 3. Page Breaks

| Aspect | Current State | Target State | Gap |
|--------|--------------|--------------|-----|
| Page break visualization | None | Visual page boundary in editor (dotted line or page gap) | **Missing entirely** |
| Page break insertion | None | Ctrl+Enter inserts page break | **Missing entirely** |
| Pagination | None | Document paginated into pages based on paper size and margins | **Missing entirely** |

**Effort:** Large (7-10d)  
**Approach:** This is the hardest feature. Two architectural options:

**Option A — Extend GtkTextView (Moderate):** Insert a special character or widget at page break positions. GtkTextView supports `GtkTextChildAnchor` which can embed widgets. We could:
1. Paginate the document using Pango layout measurement
2. Insert invisible page break markers at calculated positions
3. Render a dotted line or gap between pages

**Option B — Custom Cairo canvas (Major):** Replace GtkTextView rendering with our own Cairo-based page renderer. This is a full rewrite of the rendering layer. See architecture recommendation task.

**Research source:** rdocx-layout for pagination algorithm. Rudra-Editor's layout pipeline for split/continuation patterns. text-engine's Pango-based text measurement.

**Key challenge:** Live pagination during editing. Every keystroke potentially changes page breaks. Need efficient incremental recalculation.

---

### 4. Columns

| Aspect | Current State | Target State | Gap |
|--------|--------------|--------------|-----|
| Multi-column layout | None | 2-3 column layouts with balanced text flow | **Missing entirely** |

**Effort:** Large (5-7d, dependent on pagination)  
**Approach:** Columns require pagination to be working first. Text must flow from bottom of column 1 on page N to top of column 2 on page N, then to column 1 on page N+1.

**Research source:** rdocx-layout may support columns via DOCX section properties. Rudra-Editor's LayoutBlock with continuation tracking is the right pattern.

---

### 5. Styles System

| Aspect | Current State | Target State | Gap |
|--------|--------------|--------------|-----|
| Named styles | TextTags with names (h1-h6, code, blockquote) — no style inheritance | Style panel with named styles, inheritance, preview | **Medium** |
| Style application | Toolbar buttons apply TextTags | Style dropdown + keyboard shortcuts | **Small** |
| Style modification | None | Modify style → all instances update | **Large** |
| Default styles | Hardcoded TextTag properties | Configurable default styles (Normal, Heading 1-6, etc.) | **Medium** |

**Effort:** Medium (3-5d for basic; 7-10d for full)  
**Approach:** Build a style system on top of GtkTextTags:
1. Define styles as data (name, base style, font, size, weight, color, spacing)
2. Create/update TextTags from style definitions
3. "Modify style" → update TextTag properties → all text with that tag updates
4. Style panel in sidebar or dialog

**Research source:** Rudra-Editor's `s1-model/src/styles.rs` — Style struct with base style inheritance. LibreOffice Writer's style system concept.

---

### 6. Headers & Footers

| Aspect | Current State | Target State | Gap |
|--------|--------------|--------------|-----|
| Header/footer editing | None | Double-click header/footer area to edit | **Missing entirely** |
| Page numbering | None | Auto page numbers in header/footer | **Missing entirely** |
| Different first page | None | Option for different first page header/footer | **Missing entirely** |

**Effort:** Large (5-7d, dependent on pagination)  
**Approach:** Requires page awareness. Once pages are rendered:
1. Reserve space at top/bottom of each page for header/footer
2. Allow editing header/footer content separately from body
3. Store header/footer content as part of document model
4. Page number field as a special token that evaluates at render time

**Research source:** rdocx-layout for header/footer positioning. Rudra-Editor's `SectionProperties` with `HeaderFooterRef`.

---

### 7. Print & Print Preview

| Aspect | Current State | Target State | Gap |
|--------|--------------|--------------|-----|
| Print | No print support | File → Print with page setup dialog | **Missing entirely** |
| Print preview | No preview | Show document as pages with same layout as printed | **Missing entirely** |
| Page setup | No dialog | Paper size, orientation, margins dialog | **Missing entirely** |

**Effort:** Medium (3-5d for basic; 7-10d for full preview)  
**Approach:**
1. **Basic print:** Use `GtkPrintOperation` with a `draw_page` handler. Render the document content to the print context using Cairo. No preview — just print.
2. **Print preview:** If we implement the Cairo page canvas (Option B for page breaks), the same rendering code works for print preview. If we stay with GtkTextView, we'd need a separate preview widget.

**Research source:** GtkPrintOperation API in gtk4-rs. `draw_page` signal with Cairo context. GtkPageSetupUnixDialog.

---

### 8. DOCX Roundtrip

| Aspect | Current State | Target State | Gap |
|--------|--------------|--------------|-----|
| DOCX read | pandoc CLI → Markdown (fragile, external dep) | Native Rust DOCX reader | **Large** |
| DOCX write | pandoc CLI from Markdown (fragile, external dep) | Native Rust DOCX writer | **Large** |
| Format fidelity | Loses formatting in conversion | Preserves styles, headers/footers, images, page setup | **Large** |

**Effort:** Medium-Large (5-10d with rdocx)  
**Approach:** Use `rdocx` crate as native Rust dependency:
1. **Read:** `rdocx` reads .docx → use rdocx's document model, then convert to Letters' internal representation (GtkTextBuffer + TextTags)
2. **Write:** Letters document → rdocx document model → write .docx
3. Remove pandoc/typst CLI dependencies entirely.

**Research source:** rdocx crate (MIT/Apache-2.0). rdocx-layout for page model alignment.

**Risk:** rdocx is v0.x — API may change. Mitigation: pin version in Cargo.toml.

---

### 9. Spell-Check

| Aspect | Current State | Target State | Gap |
|--------|--------------|--------------|-----|
| Spell-check | `set_enable_spell_checking(true)` toggle | Working spell-check with dictionary support | **Small** |
| Underline errors | Provided by GTK if spell backend available | Actually shows red underlines | **Unknown** |
| Add to dictionary | None | Right-click → add to dictionary | **Small** |

**Effort:** Small (1-2d)  
**Approach:** GtkTextView 4.10+ has built-in spell-check support via `set_enable_spell_checking()` and `set_extra_menu()`. The toggle exists in Letters but needs:
1. Verify GTK spell backend is installed (gspell, enchant, or hunspell)
2. Test that misspelled words show red underlines
3. Wire `set_extra_menu()` to provide right-click suggestions and "add to dictionary"

**Research source:** GTK 4.10+ documentation. GtkTextView spell-checking requires a spell backend (usually gspell or enchant2) installed on the system. Not a code change — a runtime dependency verification.

---

### 10. Print Layout — GtkTextView Padding vs. Visual Page

| Aspect | Current State | Target State | Gap |
|--------|--------------|--------------|-----|
| Page visualization | Widget padding (left_margin=24, etc.) | Visible page rectangle with gray background | **Medium** |
| Page gap | None | Gap between pages (visual separator) | **Large** |
| Page background | Transparent (app background) | White page on gray canvas | **Medium** |

**Effort:** Medium (3-5d for basic page visualization)  
**Approach:** This is a visual enhancement that bridges the current padding-based approach to something that looks like a word processor:
1. Wrap the GtkTextView in a custom container widget
2. Use Cairo to draw a white page rectangle behind the text
3. Use a gray background for the "desktop" area around the page
4. This does NOT require pagination — it's purely visual

---

## Effort Summary

| Feature | Current | Effort | Depends On | Recommended Approach |
|---------|---------|--------|------------|---------------------|
| Spell-check | Partial (toggle exists) | 1-2d | GtkTextView spell backend | Verify runtime, wire extra_menu |
| DOCX roundtrip | Pandoc-based | 5-10d | rdocx crate | Replace pandoc with rdocx |
| Page margins (visual) | Widget padding | 3-5d | Cairo page container | Visual page rectangle widget |
| Styles system | TextTags only | 3-5d | None | Style data + TextTag sync |
| Print (basic) | None | 3-5d | GtkPrintOperation | Direct print from GtkTextView |
| Page setup dialog | None | 1-2d | GtkPageSetupUnixDialog | Standard GTK dialog |
| Rulers | None | 5-7d | Pango layout measurement | Custom Cairo ruler widget |
| Print preview | None | 5-7d | Pagination | Same rendering as print |
| Page breaks + pagination | None | 7-10d | rdocx-layout or Pango | Central feature — see architecture task |
| Headers/footers | None | 5-7d | Pagination | Page-aware content areas |
| Columns | None | 5-7d | Pagination | Multi-column text flow |
| **Total** | | **43-65d** | | |

---

## Phase Recommendation

### Phase 1 — Quick Wins (5-8d)
- Fix spell-check (verify backend, wire menu)
- Add page setup dialog (GtkPageSetupUnixDialog)
- Basic print (GtkPrintOperation from GtkTextView)
- Visual page rectangle (Cairo container widget)

### Phase 2 — DOCX & Styles (8-15d)
- Native DOCX roundtrip via rdocx
- Styles system (named styles with inheritance)
- Rulers (horizontal, with drag interaction)

### Phase 3 — Print Layout (15-25d)
- Page breaks and pagination
- Print preview
- Headers/footers
- Columns

### Phase 4 — Polish (5-10d)
- Zoom
- Page numbering
- Tab stops
- Line spacing control
