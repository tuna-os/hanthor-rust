#!/usr/bin/env python3
import time
from framework import BaseGUITestCase

class TablesGUITest(BaseGUITestCase):
    app_name = "tables"

    def test_spreadsheet_operations(self):
        # 1. Create a new spreadsheet
        new_doc_btn = self.app.child(name="New Document", roleName="button")
        new_doc_btn.do_action(0)
        time.sleep(1.0)
        
        # 2. Focus formula bar and enter a number
        formula_bar = self.app.child(roleName="text")
        self.assertIsNotNone(formula_bar, "Formula bar text widget not found")
        formula_bar.typeText("123")
        formula_bar.keyCombo("Return")
        time.sleep(0.5)
        
        # 3. Move down to cell A2 and enter a formula
        self.app.keyCombo("Down")
        time.sleep(0.2)
        formula_bar.typeText("=A1*2")
        formula_bar.keyCombo("Return")
        time.sleep(0.5)
        
        # 4. Add a new sheet
        add_sheet_btn = self.app.child(name="Add sheet", roleName="button")
        add_sheet_btn.do_action(0)
        time.sleep(0.5)
        
        # 5. Verify Sheet2 exists (it should show up in Sheet list/combobox or tab)
        sheet2_combo = self.app.child(name="Sheet2", roleName="toggle button")
        self.assertIsNotNone(sheet2_combo, "Sheet2 tab/toggle button not found after sheet addition")
        
        # 6. Capture cropped window screenshot
        self.take_screenshot("spreadsheet_calc")

if __name__ == "__main__":
    import unittest
    unittest.main()
