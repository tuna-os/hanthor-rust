// undo.rs — Tables-specific commands implementing suite_common::undo::Command.
// SPDX-License-Identifier: GPL-3.0-or-later

use suite_common::undo::Command;
use suite_common::format::NumberFormat;
use crate::window::{SheetModel, CellBorder};

// ── App state wrapper for undo ──────────────────────────────────────────
// Commands operate on the active sheet within AppState.

pub struct SheetState {
    pub sheets: Vec<SheetModel>,
    pub active_sheet: usize,
}

impl SheetState {
    pub fn sheet(&self) -> &SheetModel { &self.sheets[self.active_sheet] }
    pub fn sheet_mut(&mut self) -> &mut SheetModel { &mut self.sheets[self.active_sheet] }
}

// ── Cell Edit ───────────────────────────────────────────────────────────

pub struct CellEditCmd {
    pub row: usize,
    pub col: usize,
    pub old_val: String,
    pub new_val: String,
    pub old_formula: bool,
    pub new_formula: bool,
}

impl Command<SheetState> for CellEditCmd {
    fn apply(&self, state: &mut SheetState) {
        let sh = state.sheet_mut();
        sh.data[self.row][self.col] = self.new_val.clone();
        sh.formulas[self.row][self.col] = self.new_formula;
    }
    fn undo(&self, state: &mut SheetState) {
        let sh = state.sheet_mut();
        sh.data[self.row][self.col] = self.old_val.clone();
        sh.formulas[self.row][self.col] = self.old_formula;
    }
    fn description(&self) -> &str { "Edit Cell" }
}

// ── Column Resize ───────────────────────────────────────────────────────

pub struct ColResizeCmd {
    pub col: usize,
    pub old_width: f64,
    pub new_width: f64,
}

impl Command<SheetState> for ColResizeCmd {
    fn apply(&self, state: &mut SheetState) {
        state.sheet_mut().set_col_width(self.col, self.new_width);
    }
    fn undo(&self, state: &mut SheetState) {
        state.sheet_mut().set_col_width(self.col, self.old_width);
    }
    fn description(&self) -> &str { "Resize Column" }
}

// ── Sort ────────────────────────────────────────────────────────────────

pub struct SortCmd {
    pub col: usize,
    pub old_data: Vec<Vec<String>>,
    pub old_formulas: Vec<Vec<bool>>,
    pub old_formats: Vec<Vec<NumberFormat>>,
    pub old_borders: Vec<Vec<CellBorder>>,
    pub new_data: Vec<Vec<String>>,
    pub new_formulas: Vec<Vec<bool>>,
    pub new_formats: Vec<Vec<NumberFormat>>,
    pub new_borders: Vec<Vec<CellBorder>>,
}

impl Command<SheetState> for SortCmd {
    fn apply(&self, state: &mut SheetState) {
        let sh = state.sheet_mut();
        sh.data = self.new_data.clone();
        sh.formulas = self.new_formulas.clone();
        sh.formats = self.new_formats.clone();
        sh.borders = self.new_borders.clone();
    }
    fn undo(&self, state: &mut SheetState) {
        let sh = state.sheet_mut();
        sh.data = self.old_data.clone();
        sh.formulas = self.old_formulas.clone();
        sh.formats = self.old_formats.clone();
        sh.borders = self.old_borders.clone();
    }
    fn description(&self) -> &str { "Sort" }
}

// ── Format Change ───────────────────────────────────────────────────────

pub struct FormatCmd {
    pub row: usize,
    pub col: usize,
    pub old_format: NumberFormat,
    pub new_format: NumberFormat,
}

impl Command<SheetState> for FormatCmd {
    fn apply(&self, state: &mut SheetState) {
        state.sheet_mut().formats[self.row][self.col] = self.new_format.clone();
    }
    fn undo(&self, state: &mut SheetState) {
        state.sheet_mut().formats[self.row][self.col] = self.old_format.clone();
    }
    fn description(&self) -> &str { "Change Format" }
}

// ── Border Change ───────────────────────────────────────────────────────

pub struct BorderCmd {
    pub row: usize,
    pub col: usize,
    pub old_border: CellBorder,
    pub new_border: CellBorder,
}

impl Command<SheetState> for BorderCmd {
    fn apply(&self, state: &mut SheetState) {
        state.sheet_mut().borders[self.row][self.col] = self.new_border.clone();
    }
    fn undo(&self, state: &mut SheetState) {
        state.sheet_mut().borders[self.row][self.col] = self.old_border.clone();
    }
    fn description(&self) -> &str { "Change Border" }
}

// ── Merge Toggle ────────────────────────────────────────────────────────

pub struct MergeCmd {
    pub merges: Vec<(usize, usize, usize, usize)>,
}

impl Command<SheetState> for MergeCmd {
    fn apply(&self, state: &mut SheetState) {
        state.sheet_mut().merges = self.merges.clone();
    }
    fn undo(&self, state: &mut SheetState) {
        // We don't know the previous merge state, so this is a snapshot toggle.
        // In a full implementation, we'd store old_merges separately.
        let sh = state.sheet_mut();
        if !self.merges.is_empty() {
            sh.merges.clear();
        }
    }
    fn description(&self) -> &str { "Merge Cells" }
}

// ── Freeze Toggle ───────────────────────────────────────────────────────

pub struct FreezeCmd {
    pub old_rows: usize,
    pub old_cols: usize,
    pub new_rows: usize,
    pub new_cols: usize,
}

impl Command<SheetState> for FreezeCmd {
    fn apply(&self, state: &mut SheetState) {
        let sh = state.sheet_mut();
        sh.frozen_rows = self.new_rows;
        sh.frozen_cols = self.new_cols;
    }
    fn undo(&self, state: &mut SheetState) {
        let sh = state.sheet_mut();
        sh.frozen_rows = self.old_rows;
        sh.frozen_cols = self.old_cols;
    }
    fn description(&self) -> &str { "Freeze Panes" }
}
