# Deep-Dive: Top 3 Candidates

> Research phase for Letters — deep code reading and portability assessment.
> Date: 2026-06-23

---

## 1. rdocx — DOCX Library with Built-in Layout Engine

**Status:** Published crate, actively maintained (2026)

### Architecture

rdocx is split into at least two crates:

| Crate | docs.rs | Purpose |
|-------|---------|---------|
| `rdocx` | https://docs.rs/rdocx | High-level DOCX API — create, read, modify .docx files. Python-docx-like interface. |
| `rdocx-layout` | https://docs.rs/rdocx-layout | Layout engine — converts DOCX flow model to positioned page frames. Style resolution, font loading, text shaping, line breaking, pagination. |

### What's Portable (and How)

**Directly embeddable as a Rust dependency.** MIT/Apache-2.0 license — no copyleft restrictions. We can:

1. **DOCX roundtrip:** Use `rdocx` to read .docx into its document model, then write back out. This replaces the current pandoc CLI approach (fragile, slow, requires pandoc installed). 

2. **Layout engine:** `rdocx-layout` can paginate a document and produce positioned page frames. This is the foundation for:
   - Print preview (show pages as they'll print)
   - Page break visualization
   - Headers/footers positioning
   - Page numbering

3. **PDF export:** rdocx renders paginated documents to PDF with font subsetting — replaces typst CLI dependency.

### Integration Strategy

The key question: **rdocx has its own document model. Letters uses GtkTextBuffer.** We need a bridge:

```
Letters (GtkTextBuffer + TextTags)
    ↓ serialize to
Markdown or rdocx document model
    ↓ feed to
rdocx-layout (paginates)
    ↓ produces
Positioned page frames
    ↓ render via
Cairo DrawingArea (print preview / export)
```

**Option A:** Serialize GtkTextBuffer → Markdown → rdocx document model → layout → render
**Option B:** Maintain a parallel document model in memory (like Rudra-Editor's approach) and sync it with GtkTextBuffer

Option A is simpler to start with. Option B is the long-term architecture.

### rdocx-layout Capabilities (from docs.rs)

| Feature | Status | Relevance |
|---------|--------|-----------|
| Style resolution | ✓ | Computes effective attributes (font, size, color, spacing) |
| Text shaping | ✓ | Characters → positioned glyphs (via rustybuzz?) |
| Line breaking | ✓ | Knuth-Plass optimal or greedy fallback |
| Block stacking | ✓ | Paragraphs with spacing-before/after |
| Pagination | ✓ | Break into pages, page-break-before support |
| Page frames | ✓ | Positioned pages with content areas |
| PDF output | ✓ | Font subsetting, bookmarks |
| HTML output | ✓ | Export to HTML |
| Markdown output | ✓ | Export to Markdown |

### Risks / Unknowns

- **API maturity:** Version 0.x — API may change. Pin version.
- **Document model fidelity:** Does rdocx preserve all DOCX features (styles, headers/footers, page setup) through roundtrip?
- **Text shaping dependency:** What crate does it use? rustybuzz? allsorts? HarfBuzz binding?
- **Integration complexity:** The bridge between GtkTextBuffer and rdocx's document model is the hard part.

**Action:** `cargo doc --open rdocx` and `cargo doc --open rdocx-layout` on a local machine to read the full API.

---

## 2. Rudra-Editor — Architecture Reference (AGPL-3.0 — Do Not Port)

**Repo:** https://github.com/Rudra-Office/Rudra-Editor  
**License:** AGPL-3.0 — **CANNOT PORT CODE**  
**Status:** Active, 1,390+ tests, production-ready web editor

### Architecture (reference only)

Rudra-Editor is the best-architected Rust document engine we found. Its crate decomposition is a template for Letters' future architecture:

```
s1engine (top-level API)
├── s1-model       ← Document model (zero deps, pure data structures)
├── s1-layout      ← Page layout engine (fontdb, unicode-script)
├── s1-text        ← Text handling
├── s1-ops         ← Operational transforms (undo/redo/collab)
├── s1-crdt        ← CRDT collaboration
├── s1-format-docx ← DOCX reader/writer (quick-xml + zip)
├── s1-format-odt  ← ODT reader/writer
├── s1-format-pdf  ← PDF export
├── s1-format-md   ← Markdown support
├── s1-format-txt  ← Plain text
├── s1-format-xlsx ← Spreadsheet support
├── s1-convert     ← Format conversion
```

### Document Model (s1-model) — Key Design Decisions

**Tree structure:** Document → Body → Paragraph → Run → Text. Every node has a globally unique NodeId (replica_id, counter) — CRDT-ready design. For single-user, replica_id = 0.

**Key types:**
- `Node` — generic tree node with `NodeType` enum (Paragraph, Run, Text, Table, Image, etc.)
- `DocumentModel` — flat container with O(1) node lookup (HashMap<NodeId, Node>)
- `AttributeMap` — key-value attributes per node (bold, italic, font_size, color, etc.)
- `SectionProperties` — page size, margins, orientation, header/footer refs
- `Style` — named style with base style inheritance
- `NumberingDefinitions` — list numbering (bullet, decimal, roman, etc.)
- `MediaStore` — images and other media (HashMap<MediaId, MediaItem>)

**What's impressive:** Zero external dependencies in s1-model. Pure Rust data structures. This is the cleanest document model for a word processor I've seen in Rust.

### Layout Engine (s1-layout) — Pipeline

```
DocumentModel
    ↓ Style resolution (resolve_paragraph_style, resolve_run_style)
Resolved styles
    ↓ Text shaping (via s1-text)
Positioned glyphs
    ↓ Line breaking (Knuth-Plass / greedy)
Lines
    ↓ Block stacking (paragraphs → blocks)
Blocks
    ↓ Pagination (break into pages)
LayoutDocument (pages → blocks → lines → glyph runs)
```

**LayoutDocument output types:**
- `LayoutPage` — width, height, content_area (Rect), blocks, header, footer, footnotes, floating_images
- `LayoutBlock` — source NodeId, bounds (Rect), kind (Paragraph/Table/Image)
- `LayoutBlockKind::Paragraph` — lines, text_align, background_color, border, list_marker, indent fields, line_height, is_continuation, split_at_line
- `LayoutLine` — baseline_y, height, glyph runs
- `GlyphRun` — font, font_size, color, glyphs with positions

**Notable:** The layout engine handles paragraph splitting across pages (`is_continuation`, `split_at_line`). This is essential for print layout.

### DOCX Format (s1-format-docx) — 27 source files

Comprehensive DOCX support with separate parser/writer for:
- Content (document.xml body)
- Styles (styles.xml)
- Numbering (numbering.xml)
- Headers/Footers
- Footnotes/Endnotes
- Comments
- Sections
- Metadata
- Properties
- Streaming mode for large docs

Uses `quick-xml` + `zip` — same approach Letters would use.

### What We Can Learn (Without Porting Code)

1. **Document model design:** The Node/NodeId/AttributeMap pattern is clean and could guide Letters' internal model — even without porting code, the design is replicable.

2. **Crate decomposition:** Separating model, layout, and format I/O into independent crates is the right architecture for Letters long-term.

3. **Layout pipeline:** The 6-step layout process (style resolution → shaping → line breaking → stacking → pagination → output) is the canonical approach.

4. **Paragraph splitting:** `is_continuation` / `split_at_line` pattern for handling paragraph breaks across pages.

5. **Section properties:** Page size, margins, orientation, header/footer references — the SectionProperties design maps directly to Word/LibreOffice's section model.

---

## 3. text-engine — C/GTK4 Rich Text Framework (Architecture Reference)

**Repo:** https://github.com/mjakeman/text-engine  
**Language:** C  
**License:** MPL-2.0 / LGPL-2.1 (dual)  
**Status:** Experimental, early-stage  
**Lines:** ~7,300 (C + headers)

### Architecture

text-engine provides the cleanest C/GTK4 model-view separation we've found:

```
Model (data)          Layout (compute positions)       UI (render)
───────               ─────────────────────────         ──────────
block.c               layout.c                          display.c
document.c            layoutblock.c (PangoLayout)       inspector.c
paragraph.c           layoutbox.c (box model)
run.c                 layoutinline.c (inline items)
frame.c
image.c
item.c / mark.c / opaque.c
```

### Document Model

Hierarchical document model:
- `TextDocument` — top-level document
- `TextFrame` — a page or section frame with dimensions
- `TextBlock` — a block-level element (paragraph)
- `TextRun` — a run of text with uniform formatting
- `TextItem` — base type for all content items
- `TextMark` — formatting mark (bold, italic, etc.)
- `TextImage` — inline image
- `TextOpaque` — opaque embedded object

### Layout Engine

- `TextLayout` — builds a layout tree from a document frame
- `TextLayoutBlock` — wraps a PangoLayout for text measurement and rendering
- `TextLayoutBox` — abstract box model for positioning (x, y, width, height)
- `TextLayoutInline` — inline-level layout items

The layout engine uses **Pango** directly for text measurement (`PangoLayout`, `PangoContext`). This is relevant because GTK4 applications already have access to Pango via gtk4-rs — no additional dependency needed.

### Key Patterns Worth Porting (Concept → Rust)

1. **Model/View/Layout separation:** The three-layer architecture is directly applicable to Letters.

2. **Pango-based text measurement:** text-engine shows how to use PangoLayout for text measurement outside of a GtkTextView. This is critical for print layout where we need to measure text BEFORE rendering it in a custom Cairo canvas.

3. **Box model for layout:** The `TextLayoutBox` with x/y/width/height and parent/child relationships is the correct abstraction for page layout.

4. **Frame concept:** `TextFrame` representing a page with dimensions maps directly to print layout pages.

### Limitations

- **Early stage:** TODO list includes "Add basic input/editing capabilities" — not production-ready
- **C, not Rust:** Porting would require rewriting in Rust from scratch while preserving architecture
- **No print output:** Has layout, but no PDF or print export

---

## Portability Summary

| Candidate | Code Portable? | Architecture Portable? | Key Takeaway |
|-----------|---------------|------------------------|--------------|
| **rdocx** | ✓ Direct dependency | ✓ | DOCX roundtrip + pagination layout engine in pure Rust |
| **Rudra-Editor** | ✗ (AGPL-3.0) | ✓ Excellent reference | Document model design, crate decomposition, layout pipeline |
| **text-engine** | ✗ (C→Rust friction) | ✓ Clean reference | Model/view/layout separation, Pango-based text measurement |

## Recommended Action

1. **Immediate:** Add `rdocx` as a dependency and prototype DOCX roundtrip. Even without layout integration, replacing pandoc CLI with native Rust DOCX I/O is a win.

2. **Study:** Read Rudra-Editor's `s1-model` crate for document model design patterns.

3. **Reference:** Study text-engine's Pango-based text measurement approach for future print layout.

4. **Long-term:** Letters should evolve toward a Rudra-Editor-like architecture: separate model crate, layout engine, and format I/O crates.
