#!/usr/bin/env python3
import time
from framework import BaseGUITestCase

class LettersGUITest(BaseGUITestCase):
    app_name = "letters"

    def test_create_and_format_document(self):
        # 1. Verify we can create a new document
        new_doc_btn = self.app.child(name="New Document", roleName="button")
        new_doc_btn.do_action(0)
        time.sleep(1.0)
        
        # 2. Find the editor text field and type some content
        editor = self.app.child(roleName="text")
        self.assertIsNotNone(editor, "Editor text widget not found after document creation")
        editor.typeText("Antigravity GTK4 Letters Test - Hello World!")
        time.sleep(0.5)
        
        # Select all text to apply formatting
        editor.keyCombo("<Control>a")
        time.sleep(0.2)
        
        # 3. Toggle Bold formatting
        bold_btn = self.app.child(name="bold", roleName="toggle button")
        bold_btn.do_action(0)
        time.sleep(0.5)
        
        # 4. Take a cropped window screenshot to verify rendering visually
        self.take_screenshot("bold_formatted")
        
    def test_toggle_dark_mode(self):
        # Toggle the dark mode switch
        dark_mode_btn = self.app.child(name="Toggle Dark Mode", roleName="toggle button")
        dark_mode_btn.do_action(0)
        time.sleep(0.5)
        
        # Capture screenshot to verify dark mode styling visually
        self.take_screenshot("dark_mode")

if __name__ == "__main__":
    import unittest
    unittest.main()
