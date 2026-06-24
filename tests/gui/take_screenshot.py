#!/usr/bin/env python3
import sys
from playwright.sync_api import sync_playwright

def main():
    if len(sys.argv) < 3:
        print("Usage: take_screenshot.py <url> <output_path>")
        sys.exit(1)
        
    url = sys.argv[1]
    output_path = sys.argv[2]
    
    with sync_playwright() as p:
        browser = p.chromium.launch(
            headless=True,
            args=['--enable-webgl', '--use-gl=swiftshader', '--no-sandbox']
        )
        page = browser.new_page(viewport={"width": 1024, "height": 768})
        page.goto(url)
        page.wait_for_timeout(1000)
        page.screenshot(path=output_path)
        browser.close()

if __name__ == '__main__':
    main()
