// engine.rs — Spreadsheet engine using Formulizer.
// SPDX-License-Identifier: GPL-3.0-or-later

use std::path::Path;

pub struct Spreadsheet {
    data: Vec<Vec<String>>,
    rows: usize,
    cols: usize,
}

impl Spreadsheet {
    pub fn new(rows: usize, cols: usize) -> Result<Self, String> {
        Ok(Self { data: vec![vec![String::new(); cols]; rows], rows, cols })
    }
    pub fn set_cell(&mut self, r: usize, c: usize, v: &str) {
        if r < self.rows && c < self.cols { self.data[r][c] = v.to_string(); }
    }
    pub fn get_cell(&self, r: usize, c: usize) -> &str {
        if r < self.rows && c < self.cols { &self.data[r][c] } else { "" }
    }
    pub fn to_grid(&self) -> Vec<Vec<String>> { self.data.clone() }
}

pub fn read_spreadsheet(_p: &Path) -> Result<Spreadsheet, String> {
    Err("File I/O: use calamine or Formulizer reader".into())
}
pub fn write_spreadsheet(_p: &Path, _s: &Spreadsheet) -> Result<(), String> {
    Err("File I/O: use rust_xlsxwriter or Formulizer writer".into())
}
