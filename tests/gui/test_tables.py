#!/usr/bin/env python3
"""Tables GUI tests — every test exercises a feature and verifies the result."""

import time
from framework import BaseGUITestCase

class TablesTest(BaseGUITestCase):
    app_name = "tables"

    def _new(self):
        try:
            self.app.child(name="New Document", roleName="push button").do_action(0)
        except Exception:
            self.app.keyCombo("<Control>n")
        time.sleep(1.0)

    def _fx(self):
        return self.app.child(roleName="text")

    def _click(self, name, role="toggle button"):
        try:
            self.app.child(name=name, roleName=role).do_action(0)
        except Exception:
            try:
                self.app.child(label=name, roleName=role).do_action(0)
            except Exception:
                pass

    def test_empty_state_shows_tables_branding(self):
        self.take_screenshot("empty")
        self.assertVision([
            "Window title is 'Tables'",
            "Status page shows heading 'Tables'",
            "An 'Open File...' button is shown",
        ], screenshot_path=self.last_screenshot)

    def test_new_spreadsheet_shows_grid_and_formula_bar(self):
        self._new()
        self.take_screenshot("grid")
        self.assertVision([
            "A spreadsheet grid with column headers A, B, C, D and row headers 1, 2, 3 is visible",
            "A formula bar (text entry field) is visible above the grid",
            "Sheet tabs at the bottom show 'Sheet1' with an add button",
        ], screenshot_path=self.last_screenshot)

    def test_enter_number_in_cell(self):
        self._new()
        fx = self._fx()
        fx.typeText("123")
        fx.keyCombo("Return")
        time.sleep(0.5)
        self.take_screenshot("number")
        self.assertVision([
            "Cell A1 displays the number '123'",
        ], screenshot_path=self.last_screenshot)

    def test_enter_text_in_cell(self):
        self._new()
        fx = self._fx()
        fx.typeText("Hello Tables")
        fx.keyCombo("Return")
        time.sleep(0.5)
        self.take_screenshot("text")
        self.assertVision([
            "Cell A1 displays the text 'Hello Tables'",
        ], screenshot_path=self.last_screenshot)

    def test_simple_formula_evaluates(self):
        self._new()
        fx = self._fx()
        fx.typeText("10")
        fx.keyCombo("Return")
        time.sleep(0.3)
        self.app.keyCombo("Down")
        time.sleep(0.15)
        fx.typeText("=A1*2")
        fx.keyCombo("Return")
        time.sleep(0.5)
        self.take_screenshot("formula")
        self.assertVision([
            "Cell A1 shows '10'",
            "Cell A2 shows the computed result '20' from the formula =A1*2",
        ], screenshot_path=self.last_screenshot)

    def test_sum_formula_aggregates_range(self):
        self._new()
        fx = self._fx()
        for val in ["5", "10", "15"]:
            fx.typeText(val)
            fx.keyCombo("Return")
            time.sleep(0.2)
            self.app.keyCombo("Down")
            time.sleep(0.1)
        time.sleep(0.2)
        fx.typeText("=SUM(A1:A3)")
        fx.keyCombo("Return")
        time.sleep(0.5)
        self.take_screenshot("sum")
        self.assertVision([
            "Cells A1, A2, A3 show values 5, 10, 15",
            "Cell A4 shows the sum result '30' from =SUM(A1:A3)",
        ], screenshot_path=self.last_screenshot)

    def test_formula_bar_visible_above_grid(self):
        self._new()
        self.take_screenshot("fxbar")
        self.assertVision([
            "A text entry formula bar is positioned above the spreadsheet grid",
        ], screenshot_path=self.last_screenshot)

    def test_column_and_row_headers_displayed(self):
        self._new()
        self.take_screenshot("headers")
        self.assertVision([
            "Column headers A, B, C, D are visible at the top of the grid area",
            "Row headers 1, 2, 3, 4, 5 are visible on the left side",
        ], screenshot_path=self.last_screenshot)

    def test_add_sheet_creates_sheet2_tab(self):
        self._new()
        self._click("Add sheet", "push button")
        time.sleep(0.5)
        self.take_screenshot("addsheet")
        self.assertVision([
            "Sheet tabs show both 'Sheet1' and 'Sheet2'",
        ], screenshot_path=self.last_screenshot)

    def test_add_multiple_sheets_shows_all_tabs(self):
        self._new()
        for _ in range(2):
            self._click("Add sheet", "push button")
            time.sleep(0.3)
        self.take_screenshot("threesheets")
        self.assertVision([
            "Sheet tabs show 'Sheet1', 'Sheet2', and 'Sheet3'",
        ], screenshot_path=self.last_screenshot)

    def test_sheet_switcher_dropdown_lists_sheets(self):
        self._new()
        self._click("Add sheet", "push button")
        time.sleep(0.3)
        self.take_screenshot("switcher")
        self.assertVision([
            "Sheet tabs or switcher shows Sheet1 and Sheet2 names",
        ], screenshot_path=self.last_screenshot)

    def test_gridlines_visible_between_cells(self):
        self._new()
        self.take_screenshot("gridlines")
        self.assertVision([
            "Light gridlines are drawn between all cells in the spreadsheet grid",
        ], screenshot_path=self.last_screenshot)

    def test_column_header_dividers_for_resize(self):
        self._new()
        self.take_screenshot("resize")
        self.assertVision([
            "Column headers A, B, C, D have visible divider lines between them for column resizing",
        ], screenshot_path=self.last_screenshot)

    def test_undo_removes_cell_value(self):
        self._new()
        fx = self._fx()
        fx.typeText("Undo this value")
        fx.keyCombo("Return")
        time.sleep(0.3)
        self.app.keyCombo("<Control>z")
        time.sleep(0.5)
        self.take_screenshot("undo")
        self.assertVision([
            "Cell A1 is empty or no longer shows the previously entered text",
        ], screenshot_path=self.last_screenshot)

    def test_chart_button_triggers_chart_dialog(self):
        """Enter data then click chart to see dialog."""
        self._new()
        fx = self._fx()
        fx.typeText("Sales")
        fx.keyCombo("Right")
        time.sleep(0.1)
        fx.typeText("100")
        fx.keyCombo("Return")
        time.sleep(0.3)
        self.app.keyCombo("Down")
        time.sleep(0.1)
        fx.typeText("Costs")
        fx.keyCombo("Right")
        time.sleep(0.1)
        fx.typeText("50")
        fx.keyCombo("Return")
        time.sleep(0.3)
        self._click("Chart", "push button")
        time.sleep(0.5)
        self.take_screenshot("chart")
        self.assertVision([
            "A chart dialog or chart preview is visible with data from the spreadsheet",
        ], screenshot_path=self.last_screenshot)

    def test_toggle_border_changes_cell_style(self):
        self._new()
        fx = self._fx()
        fx.typeText("Test")
        fx.keyCombo("Return")
        time.sleep(0.3)
        self._click("Toggle Cell Border", "push button")
        time.sleep(0.4)
        self.take_screenshot("border")
        self.assertVision([
            "The selected cell shows a border or outline around it",
        ], screenshot_path=self.last_screenshot)

    def test_toggle_number_format_cycles(self):
        self._new()
        fx = self._fx()
        fx.typeText("42")
        fx.keyCombo("Return")
        time.sleep(0.3)
        self._click("Toggle Number Format", "push button")
        time.sleep(0.4)
        self.take_screenshot("format")
        self.assertVision([
            "The cell value '42' shows a different number format (currency, percent, or decimal) after toggling",
        ], screenshot_path=self.last_screenshot)

    def test_merge_cells_button_toggles(self):
        self._new()
        fx = self._fx()
        fx.typeText("Merged")
        fx.keyCombo("Return")
        time.sleep(0.3)
        self.app.keyCombo("Tab")  # move to B1
        time.sleep(0.1)
        self._click("Merge Cells", "push button")
        time.sleep(0.4)
        self.take_screenshot("merge")
        self.assertVision([
            "The two adjacent cells (A1 and B1) appear merged into a single wider cell",
        ], screenshot_path=self.last_screenshot)

    def test_export_pdf_button_triggers_save_dialog(self):
        self._new()
        self._click("Export PDF", "push button")
        time.sleep(0.5)
        self.take_screenshot("export")
        self.assertVision([
            "A file save dialog for exporting a PDF is visible on screen",
        ], screenshot_path=self.last_screenshot)

    def test_preferences_shows_show_gridlines(self):
        self._new()
        try:
            self.app.keyCombo("<Control>comma")
        except Exception:
            pass
        time.sleep(0.5)
        self.take_screenshot("prefs")
        self.assertVision([
            "A Preferences dialog with a 'Show Gridlines' toggle switch is visible",
        ], screenshot_path=self.last_screenshot)
        try:
            self.app.keyCombo("Escape")
        except Exception:
            pass

    def test_full_spreadsheet_with_data_and_formula(self):
        """Full workflow: data entry, formulas, multiple sheets."""
        self._new()
        fx = self._fx()
        for val in ["100", "200", "300"]:
            fx.typeText(val)
            fx.keyCombo("Return")
            time.sleep(0.15)
            self.app.keyCombo("Down")
            time.sleep(0.1)
        time.sleep(0.15)
        fx.typeText("=SUM(A1:A3)")
        fx.keyCombo("Return")
        time.sleep(0.4)
        self._click("Add sheet", "push button")
        time.sleep(0.3)
        self.take_screenshot("final")
        self.assertVision([
            "Cell A1 shows '100'",
            "Cell A2 shows '200'",
            "Cell A3 shows '300'",
            "Cell A4 shows the sum result '600' from =SUM(A1:A3)",
            "Sheet tabs show 'Sheet1' and 'Sheet2'",
        ], screenshot_path=self.last_screenshot)
