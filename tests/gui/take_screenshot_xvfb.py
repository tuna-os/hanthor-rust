#!/usr/bin/env python3
import os
import sys
import mss
from PIL import Image

def main():
    output_path = sys.argv[1] if len(sys.argv) > 1 else "screenshot.png"
    display = os.environ.get("DISPLAY")
    if not display:
        print("Error: DISPLAY environment variable is not set. Is Xvfb running?")
        sys.exit(1)
        
    crop_box = None
    if len(sys.argv) >= 6:
        try:
            x = int(sys.argv[2])
            y = int(sys.argv[3])
            w = int(sys.argv[4])
            h = int(sys.argv[5])
            crop_box = (x, y, x + w, y + h)
        except ValueError:
            print("Warning: Could not parse crop coordinates as integers, skipping crop.")

    try:
        with mss.mss() as sct:
            if not sct.monitors:
                print(f"Error: No monitors found on DISPLAY={display}")
                sys.exit(1)
            sct.shot(output=output_path)
        
        if crop_box:
            img = Image.open(output_path)
            cropped_img = img.crop(crop_box)
            cropped_img.save(output_path)
            print(f"Cropped screenshot to {crop_box} and saved to {output_path}")
        else:
            print(f"Screenshot saved to {output_path}")
    except Exception as e:
        print(f"Failed to take screenshot on DISPLAY={display}: {e}")
        sys.exit(1)

if __name__ == '__main__':
    main()
