#!/usr/bin/env python3
import time
from framework import BaseGUITestCase

class DecksGUITest(BaseGUITestCase):
    app_name = "decks"

    def test_slide_lifecycle(self):
        # 1. Create a new presentation
        new_doc_btn = self.app.child(name="New Document", roleName="button")
        new_doc_btn.do_action(0)
        time.sleep(1.0)
        
        # 2. Verify Slide 1 is created by default
        slide1_label = self.app.child(name="Slide 1", roleName="label")
        self.assertIsNotNone(slide1_label, "Default Slide 1 label not found")
        
        # 3. Add a new slide
        add_slide_btn = self.app.child(name="Add Slide", roleName="button")
        add_slide_btn.do_action(0)
        time.sleep(0.5)
        
        # 4. Verify Slide 2 is added
        slide2_label = self.app.child(name="Slide 2", roleName="label")
        self.assertIsNotNone(slide2_label, "Slide 2 label not found after Add Slide")
        
        # 5. Expand speaker notes
        speaker_notes_btn = self.app.child(name="Speaker Notes", roleName="button")
        speaker_notes_btn.do_action(0)
        time.sleep(0.5)
        
        # 6. Take a cropped window screenshot
        self.take_screenshot("new_slide_notes")

if __name__ == "__main__":
    import unittest
    unittest.main()
