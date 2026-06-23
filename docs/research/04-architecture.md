# Architecture Recommendation

> GtkTextView extension vs. custom Cairo canvas — trade-off analysis for Letters print layout.
> Date: 2026-06-23

---

## Decision Question

How should Letters evolve from a rich-text editor (GtkTextView + TextTags) into a word processor with page-aware print layout (margins, rulers, page breaks, headers/footers, columns, print preview)?

---

## Option A: Extend GtkTextView (Recommended)

### Architecture

```
┌─ GtkScrolledWindow ──────────────────────────┐
│  ┌─ Page Container (Cairo custom widget) ──┐  │
│  │  ┌─ Header area (GtkTextView, small) ─┐  │  │
│  │  └────────────────────────────────────┘  │  │
│  │  ┌─ Body (GtkTextView, editable) ──────┐  │  │
│  │  │  Text with TextTag formatting       │  │  │
│  │  │  Visual page break markers          │  │  │
│  │  └─────────────────────────────────────┘  │  │
│  │  ┌─ Footer area (GtkTextView, small) ─┐  │  │
│  │  └────────────────────────────────────┘  │  │
│  └──────────────────────────────────────────┘  │
│  ┌─ Ruler (Cairo custom widget, horizontal) ─┐ │
│  └───────────────────────────────────────────┘ │
└─────────────────────────────────────────────────┘
```

**Keep GtkTextView** for all text editing. Add **surrounding Cairo widgets** for print layout visualization. Paginate document using a **layout engine** (Pango or rdocx-layout) that runs asynchronously and inserts visual page break markers.

### Pros

1. **Preserves all editing features** — undo/redo, spell-check, IME input, accessibility, selection, clipboard — all work natively because GtkTextView handles them.

2. **Incremental** — can ship small improvements (page rectangle, rulers, print) without breaking existing editing.

3. **Lower risk** — no rewrite of the editing core. The most complex part (text editing) stays as-is.

4. **Native GTK integration** — GtkTextView integrates with GTK's accessibility tree, theming, input methods, and right-click menus.

5. **Existing code reuse** — all formatting actions, Markdown macros, find/replace, list handling continue to work.

### Cons

1. **"Fake" pages** — page boundaries are drawn around the widget, not real breaks in the text flow. The text is still one continuous scroll.

2. **Headers/footers are separate widgets** — they don't participate in the main text flow. Copy/paste between header and body is awkward.

3. **Pagination is approximate** — we calculate where breaks should be and insert markers, but if the user edits text near a break, the markers may drift until the layout engine re-runs.

4. **Columns are hard** — multi-column layout requires text to flow column 1 → column 2, which GtkTextView doesn't support natively.

5. **Print preview fidelity** — what you see on screen (one continuous GtkTextView) differs from what prints (multiple pages). The visual gap between editing and printing is bridged by the layout engine, but there's always a discrepancy.

### Best for: Phase 1-2 (immediate to medium-term)

---

## Option B: Hybrid — Per-Page GtkTextViews

### Architecture

```
┌─ GtkScrolledWindow ───────────────────────────────┐
│  ┌─ Page 1 ────────────────────────────────────┐  │
│  │  ┌─ Header (GtkTextView) ────────────────┐  │  │
│  │  ├─ Body (GtkTextView, editable) ────────┤  │  │
│  │  ├─ Footer (GtkTextView) ────────────────┤  │  │
│  │  └───────────────────────────────────────┘  │  │
│  └──────────────────────────────────────────────┘  │
│  [page gap — visual separator]                     │
│  ┌─ Page 2 ────────────────────────────────────┐  │
│  │  ┌─ Header (GtkTextView) ────────────────┐  │  │
│  │  ├─ Body (GtkTextView, editable) ────────┤  │  │
│  │  ├─ Footer (GtkTextView) ────────────────┤  │  │
│  │  └───────────────────────────────────────┘  │  │
│  └──────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────┘
```

**Split the document into per-page GtkTextBuffers.** Each page has its own GtkTextView body and optional header/footer TextViews. A layout engine paginates the document and distributes text across page buffers.

### Pros

1. **Real pages** — each page is a real widget, headers/footers are separate editable areas, and what you see is what prints.

2. **Print preview is editing** — no separate preview mode. The editing view IS the print view.

3. **Headers/footers are first-class** — separate editable areas within the page container.

### Cons

1. **Cross-page text flow** — when the user types at the end of page 1, should text flow to page 2? This requires a text overflow/flow system — essentially a mini layout engine that moves text between buffers.

2. **Multi-buffer complexity** — find/replace must search across all page buffers. Undo across page boundaries is tricky. Copy/paste across pages works but cursor position tracking is complex.

3. **Performance** — 50 pages = 50 GtkTextViews + 50 header/footer TextViews. GTK can handle this but it's heavier than one buffer.

4. **Rewrite of editing core** — most of Letters' current editing logic assumes one buffer per tab. Multi-buffer requires significant refactoring.

5. **Selection across pages** — selecting text that spans pages 1-3 requires a custom selection model.

### Best for: Medium-term (if Option A's page boundary issues become blockers)

---

## Option C: Custom Cairo Canvas

### Architecture

```
┌─ GtkDrawingArea (custom Cairo rendering) ────────────┐
│  ┌─ Page 1 ──────────────────────────────────────┐  │
│  │  Header text (rendered via Cairo + Pango)      │  │
│  │  Body text (rendered via Cairo + Pango)        │  │
│  │  Footer text (rendered via Cairo + Pango)      │  │
│  └───────────────────────────────────────────────┘  │
│  ┌─ Page 2 ──────────────────────────────────────┐  │
│  │  ...                                           │  │
│  └───────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────┘
```

**Replace GtkTextView entirely.** Render everything via Cairo + Pango on a GtkDrawingArea. Implement custom text editing (cursor, selection, IME, keyboard input) on top of the canvas.

### Pros

1. **Full control** — every pixel is under our control. Page breaks, columns, headers/footers, floating images, text wrapping around images — all achievable.

2. **Consistent print and screen** — same rendering code for both. True WYSIWYG.

3. **No GtkTextView limitations** — no fighting the widget's assumptions about continuous text flow.

### Cons

1. **Massive scope** — this is building a text editor from scratch. Cursor rendering, selection painting, IME composition window, text input, keyboard navigation (arrows, home/end, page up/down, Ctrl+arrows), mouse click positioning, double-click word selection, triple-click paragraph selection, clipboard, drag-and-drop, accessibility, right-click context menus, auto-scroll during selection — GtkTextView gives us ALL of this for free.

2. **Lose native accessibility** — GtkTextView integrates with Orca screen reader. Custom canvas would need manual accessibility implementation.

3. **Lose spell-check** — GtkTextView's built-in spell-check is lost. Would need hunspell/gspell integration from scratch.

4. **Lose undo/redo** — would need to implement our own undo stack.

5. **Lose IME** — input method support (CJK, emoji, compose key) is non-trivial.

6. **Effort: months, not weeks** — estimates for a production-quality custom text editor range from 6-12 months for a team.

### Best for: Never (for this project). This is what Google Docs did, and it took a large team years.

---

## Recommendation: **Option A → Option B (Phased)**

### Phase 1 — Extend GtkTextView (Now)

```
GtkTextView (editing)
├── Page Container widget (Cairo background — white page on gray canvas)
├── Ruler widget (Cairo — horizontal, drag to adjust margins/indents)
├── Page setup dialog (GtkPageSetupUnixDialog)
└── Print via GtkPrintOperation (direct from GtkTextView)
```

**Delivers:** Visual page look, rulers, print, page setup.  
**Keeps:** All editing features.  
**Effort:** 10-15d

### Phase 2 — Integrate Layout Engine

```
GtkTextView (editing)
├── rdocx or Pango-based pagination engine (async, runs on idle/after edits)
├── Visual page break markers (dotted lines or gaps in the editor)
├── Page numbering display (at page break markers)
├── Headers/footers as GtkTextViews above/below body (per-page or shared)
└── Print preview (render paginated document to Cairo)
```

**Delivers:** Page breaks, headers/footers, print preview.  
**Risks:** Page break markers drift during editing. Mitigate with debounced re-pagination (300ms after last edit).  
**Effort:** 15-25d

### Phase 3 — Evaluate Hybrid (If Needed)

If Phase 2 reveals that "fake page breaks" in a continuous GtkTextView are too limiting (e.g., columns are impossible, header/footer editing is awkward, print fidelity is poor), transition to per-page GtkTextViews.

**Trigger conditions:**
- Users report page breaks don't match print output
- Columns feature requires text flow that GtkTextView can't do
- Header/footer UX is confusing with separate widgets

**Effort:** 20-30d (significant refactoring)

---

## Layout Engine Choice

The layout engine that powers pagination can be one of:

| Engine | Approach | Pros | Cons |
|--------|----------|------|------|
| **rdocx-layout** | Use rdocx crate's layout engine | Production-quality pagination, text shaping, PDF export. Already solves the hard problems. | Own document model — needs bridge from GtkTextBuffer. v0.x API risk. |
| **Pango (native)** | Use PangoLayout from gtk4-rs for text measurement | No new dependency. Already available. Direct measurement of GtkTextView's fonts. | Must implement pagination algorithm ourselves. No PDF export. |
| **Custom** | Build our own | Maximum control | Rebuilding what rdocx already does. Not recommended. |

**Recommendation:** Use **rdocx-layout** when ready for Phase 2. For Phase 1, Pango direct measurement is sufficient (we only need ruler positioning, not full pagination).

---

## Summary

| Aspect | Recommendation |
|--------|---------------|
| **Architecture** | Option A (Extend GtkTextView) → Option B (Hybrid) if needed |
| **Never do** | Option C (Custom canvas) — too expensive, lose too much |
| **Layout engine** | rdocx-layout for Phase 2+, Pango for Phase 1 |
| **Key principle** | Preserve GtkTextView editing features at all costs |
| **Migration strategy** | Incremental — never throw away working editing code |
