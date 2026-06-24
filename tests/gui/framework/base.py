import os
import sys
import time
import subprocess
import unittest
import mss
from PIL import Image
from dogtail import tree

class BaseGUITestCase(unittest.TestCase):
    app_name = None  # to be overridden by subclasses
    
    @classmethod
    def setUpClass(cls):
        if not cls.app_name:
            raise unittest.SkipTest("app_name not set")
            
    def setUp(self):
        # Resolve directories
        self.framework_dir = os.path.dirname(os.path.abspath(__file__))
        self.gui_dir = os.path.dirname(self.framework_dir)
        self.workspace_dir = os.path.dirname(os.path.dirname(self.gui_dir))
        
        # Path to compiled binary
        self.bin_path = os.path.join(self.workspace_dir, "target", "debug", self.app_name)
        if not os.path.exists(self.bin_path):
            raise RuntimeError(f"Binary not found at {self.bin_path}. Run 'cargo build' first.")
            
        # Clear any leftover processes
        subprocess.run(["pkill", "-x", self.app_name], stderr=subprocess.DEVNULL)
        time.sleep(0.2)
        
        # Launch app under GDK_BACKEND=x11
        env = os.environ.copy()
        env["GDK_BACKEND"] = "x11"
        self.process = subprocess.Popen(
            [self.bin_path],
            env=env,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            text=True
        )
        
        # Wait for application node in AT-SPI tree
        self.app = self.wait_for_app(self.app_name)
        
    def tearDown(self):
        if hasattr(self, "process") and self.process:
            self.process.terminate()
            try:
                self.process.wait(timeout=2)
            except subprocess.TimeoutExpired:
                self.process.kill()
                
    def wait_for_app(self, name: str, timeout: float = 15.0) -> tree.Accessible:
        deadline = time.monotonic() + timeout
        while time.monotonic() < deadline:
            try:
                app = tree.root.application(name)
                if app:
                    return app
            except Exception:
                pass
            time.sleep(0.5)
        raise RuntimeError(f"Application '{name}' did not appear in AT-SPI registry within {timeout}s")
        
    def get_window_geometry(self) -> tuple[int, int, int, int]:
        """Locates the main frame window of the app and returns (x, y, w, h)."""
        try:
            # GTK4 main window is typically registered with role frame
            frame = self.app.child(roleName='frame')
            if frame:
                return frame.position[0], frame.position[1], frame.size[0], frame.size[1]
        except Exception as e:
            print(f"Warning: Failed to get frame geometry via Dogtail: {e}")
        # Return fallback full-screen or standard window coordinates if lookup fails
        return 0, 0, 1920, 1080

    def take_screenshot(self, suffix: str, crop: bool = True):
        """Takes a screenshot. If crop is True, crops to the main frame window."""
        output_filename = f"{self.app_name}_screenshot"
        if suffix:
            output_filename += f"_{suffix}"
        output_filename += ".png"
        
        output_path = os.path.join(self.gui_dir, output_filename)
        
        display = os.environ.get("DISPLAY")
        if not display:
            print("Warning: DISPLAY not set, skipping screenshot.")
            return
            
        try:
            with mss.mss() as sct:
                sct.shot(output=output_path)
            
            if crop:
                x, y, w, h = self.get_window_geometry()
                # Make sure coordinates are valid before cropping
                if w > 0 and h > 0:
                    img = Image.open(output_path)
                    # Coordinates: (left, upper, right, lower)
                    cropped_img = img.crop((x, y, x + w, y + h))
                    cropped_img.save(output_path)
                    print(f"Saved window-cropped screenshot to {output_path}")
                else:
                    print(f"Saved full-screen screenshot to {output_path} (invalid geometry: {x},{y},{w},{h})")
            else:
                print(f"Saved full-screen screenshot to {output_path}")
        except Exception as e:
            print(f"Failed to capture screenshot: {e}")
