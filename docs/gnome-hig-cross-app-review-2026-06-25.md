# GNOME HIG Cross-App Review
Date: 2026-06-25
Reviewer: Gemma-4-31B (via Lemonade)
Screenshots: pre-fix. Fixes applied after: toolbar icons, page bg transparent, sidebar controls.

*   Three apps: Letters, Decks, Tables.
    *   Criteria: GNOME HIG (Human Interface Guidelines) compliance.
    *   Format: Score (A-F) and 1-2 sentence summary each.
    *   Constraint: Be concise.

    *   *Letters:*
        *   Appearance: Dark mode, custom toolbar at the top with many buttons (bold, italic, etc.).
        *   UI Elements: Centered splash screen with a custom icon and a blue pill-shaped button.
        *   HIG analysis: The toolbar is very dense and doesn't follow standard Adwaita/GTK4 patterns (too many disparate buttons in a row). The button shape and placement are non-standard.
        *   Score: D/C.

    *   *Decks:*
        *   Appearance: Dark mode, side pane for slides, main editor area.
        *   UI Elements: Top bar has icons and a title. Centered splash screen similar to Letters.
        *   HIG analysis: Better layout (sidebar/main view), but still uses the same non-standard button and splash style. Toolbars are a bit cluttered.
        *   Score: C.

    *   *Tables:*
        *   Appearance: Dark mode, top toolbar, centered splash.
        *   UI elements: Toolbar contains specific table-related formatting options.
        *   HIG analysis: Similar to Letters. Excessive top-bar buttons, non-standard button styling.
        *   Score: D/C.

    *   *Letters:* D. Non-standard toolbar density and button styling deviate significantly from Adwaita guidelines.
    *   *Decks:* C. Layout is more familiar, but the button styling and splash screen remain non-compliant.
    *   *Tables:* D. Overcrowded header bar and inconsistent widget shapes clash with GNOME's minimalist design language.

    *   Letters: D - Overcrowded toolbar and non-standard button shapes violate Adwaita's minimalist aesthetic.
    *   Decks: C - Layout is better, but UI elements still deviate from standard HIG patterns.
    *   Tables: D - Excessive header bar buttons and custom widget styling ignore GNOME design standards.