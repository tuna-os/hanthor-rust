# Survey: Rust/GTK4 WYSIWYG Editors & Word Processors

> Research phase for Letters — viable WYSIWYG word processor target.
> Survey date: 2026-06-23

---

## Tier 1 — High Relevance, Rust, Portability Likely

### 1. rdocx (`crates.io/crates/rdocx`)

| Field | Detail |
|-------|--------|
| **Repo** | https://docs.rs/rdocx |
| **Language** | Pure Rust |
| **License** | MIT / Apache-2.0 |
| **Status** | Published crate (v0.x, active 2026) |

**What it does:** The only Rust crate that combines DOCX read/write with a built-in layout engine. Separate sub-crates for layout (`rdocx-layout`) and the high-level DOCX API (`rdocx`). The layout engine performs style resolution, font loading, text shaping, line breaking, and pagination. Can render to PDF with font subsetting and bookmarks. Also exports to HTML, Markdown, and PNG.

**Portability assessment:** Extremely high. This is the single most valuable find for Letters. It solves:
- **DOCX roundtrip** — read and write .docx natively, no pandoc/CLI dependency
- **Layout engine** — pagination, text shaping, font handling — could be used for print layout
- **PDF export** — integrated PDF export with font subsetting

The layout engine (`rdocx-layout`) in particular could serve as the pagination backend for Letters' print layout. Since it's pure Rust with MIT/Apache-2.0 license, we can fork, embed, or use as a library with zero friction.

**Key sub-crates:**
- `rdocx` — high-level DOCX API (Python-docx-like interface)
- `rdocx-layout` — layout engine (style resolution → positioned page frames) — on docs.rs

**Cargo.toml:** `rdocx = "0.x"` (check latest on crates.io)

---

### 2. docx-rs (`crates.io/crates/docx-rs` / `docx-rust`)

| Field | Detail |
|-------|--------|
| **Repo** | https://github.com/cstkingkey/docx-rs |
| **Language** | Pure Rust |
| **License** | MIT |
| **Status** | Stable crate |

**What it does:** Read and write DOCX files programmatically. Provides a builder API for constructing documents with paragraphs, tables, images, styles, headers, footers, and page breaks. Does NOT include a layout engine — it's purely a file format library.

**Portability assessment:** High for DOCX I/O. If we use `rdocx` for the layout engine, `docx-rs` is an alternative for the simpler file format layer. However, `rdocx` already includes both, making `docx-rs` a secondary choice unless we find API tradeoffs that favor it.

---

## Tier 2 — High Relevance, Rust, Licensing/Platform Constraints

### 3. Rudra-Editor (`github.com/Rudra-Office/Rudra-Editor`)

| Field | Detail |
|-------|--------|
| **Repo** | https://github.com/Rudra-Office/Rudra-Editor |
| **Language** | Pure Rust (1,390+ tests, zero C/C++ deps) |
| **License** | AGPL-3.0 |
| **Status** | Active (2026), web editor + Docker |

**What it does:** A modular document and spreadsheet engine SDK. Reads/writes DOCX, ODT, PDF, TXT, Markdown, XLSX, ODS, CSV. Includes CRDT collaboration, a page layout engine (s1-layout), text shaping (rustybuzz), font subsetting, PDF export, charts, and a web-based editor.

**Architecture (crates):**
| Crate | Purpose |
|-------|---------|
| `s1-model` | Shared document model (blocks, paragraphs, runs, styles) |
| `s1-layout` | Page layout engine (fontdb, unicode-script) |
| `s1-format-docx` | DOCX reader/writer (quick-xml + zip) |
| `s1-format-pdf` | PDF export |
| `s1-format-odt` | ODT support |
| `s1-text` | Text handling |
| `s1-ops` | Operational transforms (undo/redo) |
| `s1engine` | Top-level engine API |

**Portability assessment:** **Architecture reference only.** AGPL-3.0 is a hard blocker — we cannot port any code. However, the architecture is extremely well-designed and serves as an excellent reference for:
- How to structure a document model (s1-model)
- How to separate layout engine from format I/O
- How to compose format readers/writers as separate crates
- Document model design (block/paragraph/run hierarchy)

The crate decomposition is worth studying as a template for Letters' architecture.

---

### 4. Velotype (`github.com/manyougz/velotype`)

| Field | Detail |
|-------|--------|
| **Repo** | https://github.com/manyougz/velotype |
| **Language** | Rust |
| **Framework** | gpui (Zed editor framework) — NOT GTK4 |
| **License** | Unknown (check repo) |

**What it does:** A WYSIWYG Markdown editor with block-based editing. Uses a custom AST-based approach: parse Markdown → block tree → render. Supports headings, code blocks, tables, images, inline formatting. Has an undo/redo stack, auto-save, PDF/HTML export, and inline markdown parsing (regex-based).

**Key modules:**
- `editor/document.rs` — Markdown → editor tree deserialization
- `editor/render.rs` — Block rendering with Pango layout (gpui-specific)
- `editor/events.rs` — Block-level event handling (split, merge, delete, indent)
- `editor/history.rs` — Undo/redo stack
- `editor/export.rs` — PDF/HTML export
- `components/markdown/inline.rs` — Inline markdown parser (**bold**, *italic*, `code`, [links])
- `components/block/element.rs` — Block element types

**Portability assessment:** **Architecture reference only.** The gpui framework is not portable to GTK4. However, several patterns are worth studying:
- The block-based document model (P, H1-H6, code, table, image blocks)
- The inline Markdown regex-based parser for live macros
- The undo/redo stack design (if GtkTextBuffer built-in is insufficient)
- The export pipeline architecture

---

### 5. Marko Editor (`github.com/mmMike/marko-editor`)

| Field | Detail |
|-------|--------|
| **Repo** | https://github.com/mmMike/marko-editor |
| **Language** | Rust + gtk4-rs + pulldown-cmark |
| **License** | Unknown (check repo) |

**What it does:** A WYSIWYG Markdown note-taking editor built on GTK4. Uses GtkTextBuffer with TextTags for formatting. Converts between Markdown and tag-based formatting via pulldown-cmark's event stream.

**Key modules:**
- `textbuffermd.rs` — `assign_markdown()` / `to_markdown()` — bidirectional Markdown↔TextTag conversion
- `textbufferext.rs` — Cursor helpers, link/image tag creation, paragraph formatting
- `texttag.rs` — Tag definitions (BOLD, ITALIC, H1-H6, CODE, MONO, STRIKE, LINK, IMAGE)
- `texttagtable.rs` — Tag factory with Markdown↔tag mapping
- `textview.rs` — Main editor widget with key handlers, undo/redo, autocomplete, search

**Portability assessment:** **High architecture/pattern reference** for GtkTextBuffer + Markdown integration. Our Letters already uses this pattern (pulldown-cmark events → TextTags). However, Marko Editor does NOT implement live Markdown macros (**bold** → bold as you type) — formatting is done via keyboard shortcuts/toolbar. Letters has already surpassed Marko Editor in this regard (we have live macros). Marko is still useful as a reference for:
- Tag naming conventions
- The Markdown↔TextTag mapping table pattern
- TextTag application via pulldown-cmark event stream

---

## Tier 3 — High Relevance, Non-Rust

### 6. text-engine (`github.com/mjakeman/text-engine`)

| Field | Detail |
|-------|--------|
| **Repo** | https://github.com/mjakeman/text-engine |
| **Language** | C |
| **Framework** | GTK4 (native) |
| **License** | MPL-2.0 / LGPL-2.1 (dual) |
| **Status** | Heavy development / experimental |

**What it does:** A rich-text editing framework for GTK 4. Designed as a "build your own word processor" kit. Provides a model/view architecture with layout engine.

**Architecture (cloned & inspected):**

| Module | Files | Purpose |
|--------|-------|---------|
| **model/** | block, document, fragment, frame, image, item, mark, opaque, paragraph, run | Document model — block/paragraph/run hierarchy, frames, images, marks |
| **layout/** | layout, layoutblock, layoutbox, layoutinline | Layout engine — block layout, inline layout, box model |
| **tree/** | node | Tree data structure for document hierarchy |
| **editor/** | editor | Editing operations |
| **ui/** | display, inspector | Rendering (Cairo), inspector/debug |
| **format/** | import-html | Format import |

**Portability assessment:** **Architecture reference only (C→Rust is high friction).** The model/view separation is clean and worth studying:
- The block/paragraph/run document model is well-designed
- The layout module's separation of block/inline/box layout is instructive
- The TODO list confirms this is early-stage: "Add basic input/editing capabilities" is still listed

However, C→Rust porting is impractical for an implementation goal. Use as architectural reference for document model design.

**License note:** MPL-2.0 allows use in proprietary projects; LGPL-2.1 allows linking. Porting concepts (not code) has no license issues.

---

### 7. AbiWord (`gitlab.gnome.org/GNOME/abiword`)

| Field | Detail |
|-------|--------|
| **Repo** | https://gitlab.gnome.org/GNOME/abiword (or github.com/AbiWord/abiword) |
| **Language** | C / C++ |
| **Framework** | GTK3 (originally GTK2) — not GTK4 |
| **License** | GPL-2.0 |
| **Status** | Mature, low activity |

**What it does:** A lightweight, cross-platform word processor. Features include page layout, headers/footers, tables, images, styles, spell-check, print/preview, and DOCX import/export.

**Portability assessment:** **Architecture reference only (C++→Rust is impractical).** AbiWord has battle-tested implementations of:
- Page layout with headers/footers
- Ruler widget
- Print preview
- Style system
- DOCX roundtrip via libwpd/wvWare

These are reference implementations for understanding HOW these features work, not code to port. Studying AbiWord's ruler widget and page layout pipeline can inform our own design.

**Note:** AbiWord's GTK3 codebase is not directly useful for GTK4 bindings. Concepts only.

---

### 8. GtkSourceView5

| Field | Detail |
|-------|--------|
| **Repo** | https://gitlab.gnome.org/GNOME/gtksourceview |
| **Language** | C |
| **Framework** | GTK4 (extends GtkTextView) |
| **License** | LGPL-2.1+ |
| **Status** | Mature, actively maintained |
| **gtk4-rs binding** | `sourceview5` crate |

**What it does:** Extends GtkTextView with advanced text capabilities: syntax highlighting, line numbers, code folding, search/replace with highlighting, completion, and more. Primarily designed for code editors, but its GtkTextView extensions are relevant.

**Portability assessment:** **Directly usable as a dependency.** GtkSourceView5 can be wrapped in our existing GtkTextView-based Letters to get:
- Better search/replace with match highlighting (already partially implemented)
- Line numbers (useful for legal/technical documents)
- Gutter infrastructure that could be repurposed for page margin indicators

The Rust binding `sourceview5` crate should be evaluated for inclusion.

---

## Tier 4 — Reference Only / Lower Priority

### 9. wysiwyg-rich-text-editor-rust-wasm (`github.com/Pterygoidien/wysiwyg-rich-text-editor-rust-wasm`)

| Field | Detail |
|-------|--------|
| **Repo** | https://github.com/Pterygoidien/wysiwyg-rich-text-editor-rust-wasm |
| **Language** | Rust + WASM + Svelte 5 (TypeScript) |
| **License** | Unknown |

**What it does:** A professional-grade, canvas-based paginated rich text editor. Custom layout engine in Rust/WASM, Svelte Shell for the UI. Supports multiple pages, columns, images, tables, and rich text formatting. Similar to Google Docs in approach — canvas-based rendering with a custom layout engine in WASM.

**Portability assessment:** **Architecture reference only.** The WASM/canvas approach is fundamentally different from GTK4. However, the layout engine concepts (pagination, column layout, text flow) are worth studying. The Rust/WASM layout engine is pure Rust and could theoretically be adapted, but the web shell is not relevant.

**Note:** Clone timed out (large repo). Worth revisiting for layout engine research.

---

### 10. Tau (`github.com/Cogitri/Tau`)

| Field | Detail |
|-------|--------|
| **Repo** | https://github.com/Cogitri/Tau (formerly gxi) |
| **Language** | Rust |
| **Framework** | GTK (Xi editor backend) |
| **License** | Unknown |

**What it does:** A GTK frontend for the Xi text editor backend (Rust). Xi uses a client/server architecture with a Rust core. It's a code editor, not a word processor.

**Portability assessment:** **Low.** Code editor, not word processor. The Xi backend (Rope data structure, CRDT operations) is sophisticated but designed for plain text editing, not rich text with print layout. The Rust GTK frontend patterns may have some value, but Marko Editor is a better reference.

---

## Summary Matrix

| # | Candidate | Lang | License | Portability | Key Value |
|---|-----------|------|---------|-------------|-----------|
| 1 | **rdocx** | Rust | MIT/Apache-2.0 | **Directly embeddable** | DOCX R/W + layout engine + PDF |
| 2 | docx-rs | Rust | MIT | **Directly embeddable** | DOCX R/W (alternative) |
| 3 | Rudra-Editor | Rust | AGPL-3.0 | Architecture ref | Doc model, crate decomposition |
| 4 | Velotype | Rust (gpui) | Unknown | Architecture ref | Block model, Markdown parser |
| 5 | Marko Editor | Rust/GTK4 | Unknown | Pattern ref | GtkTextView + MD integration |
| 6 | text-engine | C/GTK4 | MPL-2.0/LGPL-2.1 | Architecture ref | Model/view separation, layout |
| 7 | AbiWord | C++/GTK3 | GPL-2.0 | Architecture ref | Ruler, pagination, print preview |
| 8 | GtkSourceView5 | C/GTK4 | LGPL-2.1+ | **Dependency candidate** | Gutter, search highlighting |
| 9 | wasm-editor | Rust/WASM | Unknown | Architecture ref | Canvas pagination engine |
| 10 | Tau | Rust/GTK | Unknown | Low | Xi backend (code editor) |

---

## Key Insight

The ecosystem has no production-grade Rust/GTK4 word processor with print layout. However, there are two powerful building blocks:

1. **rdocx** solves DOCX roundtrip and provides a pagination layout engine — the hardest problems
2. **Letters** already has the WYSIWYG editing UX (GtkTextView + TextTags + Markdown macros + find/replace + toolbar)

The gap is bridging them: using rdocx's layout engine to drive print layout visualization in Letters, while keeping the GtkTextView for editing.
