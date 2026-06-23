#!/usr/bin/env python3
"""
Broadway Canvas Inspector — uses Playwright to inspect GTK4 apps running via Broadway.
Since GTK4 Broadway renders into a WebGL canvas (not HTML DOM nodes),
this inspector uses screenshots, canvas state queries, and simulated interaction.

Usage: python3 skills/broadway-inspect/broadway_inspect.py [letters|tables|decks]

Assumes Broadway daemon is running on http://localhost:8085.
"""

import sys
import os
import time

BROADWAY_URL = "http://localhost:8085"

def run():
    try:
        from playwright.sync_api import sync_playwright
    except ImportError:
        print("Playwright not installed. Run: pip install playwright && playwright install chromium")
        sys.exit(1)

    app = sys.argv[1] if len(sys.argv) > 1 else "letters"
    print(f"=== Broadway Canvas Inspector: {app} ===")

    with sync_playwright() as p:
        browser = p.chromium.launch(headless=True, args=[
            '--enable-webgl',
            '--use-gl=swiftshader',  # software WebGL renderer
            '--no-sandbox',
            '--disable-setuid-sandbox',
        ])
        page = browser.new_page(viewport={"width": 1024, "height": 768})
        try:
            page.goto(BROADWAY_URL, timeout=10000)
            page.wait_for_timeout(3000)  # Let the app render
        except Exception as e:
            print(f"Warning: page load issue: {e}")

        title = page.title()
        print(f"Page title: {title}")

        # The GTK app renders into a canvas — check canvas presence
        canvas = page.locator("canvas")
        canvas_count = canvas.count()
        print(f"Canvas elements: {canvas_count}")

        # Check if the canvas has content (non-empty dimensions)
        if canvas_count > 0:
            box = canvas.first.bounding_box()
            if box:
                print(f"Canvas size: {int(box['width'])}x{int(box['height'])}")

        # Take a screenshot for visual verification
        screenshot_dir = "/tmp/broadway-screenshots"
        os.makedirs(screenshot_dir, exist_ok=True)
        screenshot_path = f"{screenshot_dir}/{app}-{int(time.time())}.png"
        page.screenshot(path=screenshot_path)
        print(f"Screenshot saved: {screenshot_path}")

        # Simulate keyboard shortcut to create a new document
        page.keyboard.press("Control+n")
        page.wait_for_timeout(1500)

        # Take another screenshot after new document
        screenshot2 = f"{screenshot_dir}/{app}-newdoc-{int(time.time())}.png"
        page.screenshot(path=screenshot2)
        print(f"After Ctrl+N screenshot: {screenshot2}")

        # Try typing some text
        page.keyboard.type("Hello from Letters!")
        page.wait_for_timeout(1000)

        # Bold the text
        page.keyboard.press("Control+b")
        page.wait_for_timeout(500)

        screenshot3 = f"{screenshot_dir}/{app}-typed-{int(time.time())}.png"
        page.screenshot(path=screenshot3)
        print(f"After typing screenshot: {screenshot3}")

        # Verification summary
        print("\n=== Verification Summary ===")
        print(f"  App renders via Broadway: {'YES' if canvas_count > 0 else 'NO'}")
        print(f"  Canvas present with content: {'YES' if canvas_count > 0 and box and box['width'] > 100 else 'NO'}")
        print(f"  Screenshots saved to: {screenshot_dir}/")
        print(f"  For interactive inspection: open http://localhost:8085 in Chrome")

        browser.close()

if __name__ == "__main__":
    run()
