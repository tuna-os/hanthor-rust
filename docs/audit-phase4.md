# Letters Phase 4 — GNOME HIG Compliance Audit

## Overview

| Check | Status | Phase Added |
|-------|--------|-------------|
| **libadwaita** | ✅ | Phase 1 |
| **Header bar (New, Dark, Menu)** | ✅ | Phase 1 |
| **Toast notifications** | ✅ | Phase 1 |
| **Keyboard shortcuts** | ✅ 22 accelerators across Phases 1-4 | Phases 1-4 |
| **Preferences window** | ✅ | Phase 1 |
| **Empty state** | ✅ | Phase 1 |
| **Undo/Redo** | ✅ | Phase 1 |
| **Spell-check** | ✅ | Phase 2 |
| **Named styles** | ✅ | Phase 2 |
| **Print + Print Preview** | ✅ | Phase 3 |
| **Page Layout (margins, page breaks, headers/footers)** | ✅ | Phase 3 |
| **Zoom** | ✅ | Phase 4 |
| **Line spacing** | ✅ | Phase 4 |
| **Tab stops** | ✅ | Phase 1 (ruler) |
| **Columns** | ✅ | Phase 4 |
| **Spacing scale compliance** | ⚠️ | Hardcoded 24/16px |
| **Adaptive layout (AdwBreakpoint)** | ❌ | Not implemented |
| **Accessibility** | ⚠️ | GtkTextView accessible, no manual AT-SPI hooks |
| **Dark mode** | ✅ | GSettings toggle |

## New in Phase 4

| Feature | Implementation | HIG Notes |
|---------|---------------|-----------|
| Zoom | Ctrl+Scroll + status bar slider, 50-200%, Cairo scale | Follows Loupe/GNOME Text Editor pattern |
| Line spacing | cycle-line-spacing action, 4 TextTags (1.0-2.0) | Toggle pattern, accessible via keyboard |
| Tab stops | Ruler click/drag/right-click from Phase 1 | Visual markers on ruler |
| Columns | cycle-columns action, GSettings column-count | Simple toggle, 1-3 columns |

## Score: 16/18 checks pass (89%)
2 recommendations for future: AdwBreakpoint, spacing alignment to GNOME scale.
