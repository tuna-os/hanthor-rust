# Letters Phase 3 — GNOME HIG Compliance Audit

## Overview

| Check | Status | Notes |
|-------|--------|-------|
| **libadwaita** | ✅ | AdwApplicationWindow, AdwTabView, AdwTabBar, AdwToastOverlay, AdwAlertDialog |
| **Header bar** | ✅ | Custom HeaderBar with New, Dark toggle, Menu button |
| **Toast notifications** | ✅ | AdwToastOverlay present (wired for save notifications via suite-common) |
| **Keyboard shortcuts** | ✅ | 18 accelerators: Ctrl+N/O/S/P/Z/Y/B/I/U/F/L/E/R/J/K/comma/question |
| **Preferences window** | ✅ | AdwPreferencesDialog with General, Editing, Appearance pages |
| **Empty state** | ✅ | AdwStatusPage when no document open |
| **Undo/Redo** | ✅ | GtkTextBuffer built-in undo/redo, Ctrl+Z / Ctrl+Y |
| **Spacing** | ⚠️ | Hardcoded 24px/16px margins on editor. Should use GNOME spacing scale (6, 12, 24) |
| **Adaptive layout** | ❌ | No AdwBreakpoint for responsive toolbar collapse |
| **Spell-check** | ✅ | zspell-backed with GSettings toggle |
| **File dialogs** | ✅ | Native GtkFileDialog with format filters |

## Key Patterns Present

- **Tabbed documents**: AdwTabView with close-page confirmation (AdwAlertDialog)
- **Find/Replace**: GtkRevealer with GtkSearchEntry
- **Drag to new window**: connect_create_window
- **Dark mode**: StyleManager toggle, GSettings persistence

## Recommendations

1. Add AdwBreakpoint for responsive toolbar (Phase 4)
2. Align spacing to GNOME scale: use 6/12/24 instead of 24/16 hardcoded
3. Add keyboard shortcuts dialog (AdwShortcutsWindow)
