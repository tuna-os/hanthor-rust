# GUI Testing Research & Improvement Recommendations

> Research conducted 2026-06-24 via SearXNG metasearch across DuckDuckGo, Google, Brave, and Wikipedia.

## Table of Contents

1. [GTK4 Window Shadows & Dogtail Positioning](#1-gtk4-window-shadows--dogtail-positioning)
2. [AT-SPI Startup Race Conditions](#2-at-spi-startup-race-conditions)
3. [Dogtail Configuration & Compatibility](#3-dogtail-configuration--compatibility)
4. [Xvfb Resolution & DPI](#4-xvfb-resolution--dpi)
5. [Animation Disabling for Test Reliability](#5-animation-disabling-for-test-reliability)
6. [Timing & Flakiness Patterns](#6-timing--flakiness-patterns)
7. [Alternative/Complementary Tools](#7-alternativecomplementary-tools)
8. [mss Screenshot Best Practices](#8-mss-screenshot-best-practices)
9. [Concrete Improvement Recommendations](#9-concrete-improvement-recommendations)

---

## 1. GTK4 Window Shadows & Dogtail Positioning

### Finding

**GTK4's compositor-managed window shadows cause Dogtail coordinate calculations to be incorrect.** Dogtail uses AT-SPI to locate widgets in the accessibility tree, but when it translates to screen coordinates for click/drag operations, it queries the X11 window geometry. With GTK4 client-side decorations, the reported window frame includes shadows that inflate the geometry beyond the actual interactive area.

### Sources

- [Tails GitLab: Disable GTK4 shadows for Dogtail compatibility (#21022)](https://gitlab.tails.boum.org/tails/tails/-/issues/21022) — "For GTK4 applications, disabling window shadows is essential to ensure accurate positioning by Dogtail. I can confirm that it fixes one..."
- [vhumpa/dogtail README](https://github.com/vhumpa/dogtail) — Official dogtail fork notes this as a known requirement.
- [dogtail PyPI (1.0.8)](https://pypi.org/project/dogtail/1.0.8/) — "For GTK4 applications, disabling window shadows is essential to ensure accurate positioning by Dogtail."

### Recommendation

Disable GTK4 window shadows in the test environment. Three approaches (ordered by preference):

1. **CSS override** — Create `~/.config/gtk-4.0/gtk.css` with:
   ```css
   window {
       box-shadow: none;
   }
   window decoration {
       box-shadow: none;
   }
   ```

2. **GSettings key** — Set `org.gnome.desktop.interface:gtk-enable-animations` to `false` (this also disables shadows in some compositor paths, though not guaranteed).

3. **Programmatic** — Set the GTK CSS provider programmatically in the test harness before the app starts.

---

## 2. AT-SPI Startup Race Conditions

### Finding

The current `justfile` uses a **fixed `sleep 1.5`** between launching `at-spi-bus-launcher`/`at-spi2-registryd` and running the app. This is fragile because:

- AT-SPI bus registration is asynchronous and D-Bus activation-dependent
- On slow or resource-constrained systems, 1.5s may not be enough
- On fast systems, we waste time waiting needlessly
- There is **no error detection** — if the bus fails to start, the test hangs or fails obscurely

### Sources

- [GNOME/at-spi2-core bus README](https://github.com/GNOME/at-spi2-core/blob/main/bus/README.md) — Documents `at-spi-bus-launcher` lifecycle management.
- [Arch Linux Forum: at-spi2-registryd control](https://bbs.archlinux.org/viewtopic.php?id=189975) — Community experiences with startup ordering.
- Multiple Arch forum threads document "Could not register with accessibility bus" errors from startup ordering issues.

### Recommendation

Replace the fixed `sleep 1.5` with a **poll-and-verify pattern**:

```bash
# Launch AT-SPI infrastructure
/usr/libexec/at-spi-bus-launcher --launch-immediately &
/usr/libexec/at-spi2-registryd &

# Poll until the accessibility bus is ready
for i in $(seq 1 30); do
    if gdbus call --session \
        --dest org.a11y.Bus \
        --object-path /org/a11y/bus \
        --method org.freedesktop.DBus.Peer.Ping \
        >/dev/null 2>&1; then
        echo "AT-SPI bus ready after ${i}s"
        break
    fi
    sleep 1
done
```

Alternative: check for the D-Bus socket or use `dbus-send` / `busctl`.

---

## 3. Dogtail Configuration & Compatibility

### Finding

The **vhumpa/dogtail fork** (recommended over the stale upstream) adds several quality-of-life improvements:

| Feature | Detail |
|---------|--------|
| Config files | `/etc/dogtail/config` and `/etc/dogtail/config.json` |
| Role name compatibility | at-spi2-core >= 2.53 changed `"push button"` → `"button"`. Dogtail can override back to `"push button"` if enabled in config. |
| Logging | Configurable log levels via config |

### Relevance

Our current test stubs (`test_letters.py`, etc.) only verify app presence in the a11y tree. As tests grow more sophisticated (clicking buttons, filling fields), the role name compatibility will matter.

### Recommendation

1. Pin to the **vhumpa/dogtail** fork rather than the stale PyPI release.
2. Create a minimal `/etc/dogtail/config.json` in the toolbox:
   ```json
   {
       "logLevel": "debug",
       "atspiCompatibility": {
           "overridePushButton": true
       }
   }
   ```
3. Add a `setup-gui` step that installs from the vhumpa fork rather than Fedora's `python3-dogtail` package.

---

## 4. Xvfb Resolution & DPI

### Finding

By default, `xvfb-run` creates a display with **very low resolution** (often 800x600 or even smaller). This can cause:

- GTK4 windows to be clipped or layout to change unexpectedly
- Buttons/widgets to be outside the viewport
- Screenshots to be unrepresentative of real usage

### Sources

- [Stack Overflow: Xvfb incorrect resolution](https://stackoverflow.com/questions/43637429/xvfb-xvfb-run-incorrect-resolution)
- [Katalon Community: Xvfb screen resolution](https://forum.katalon.com/t/xvfb-screen-resolution/62037)
- [silverstripe/silverstripe-travis-support #9](https://github.com/silverstripe/silverstripe-travis-support/issues/9)

### Recommendation

Pass explicit screen dimensions to `xvfb-run`:

```bash
xvfb-run -a -s "-screen 0 1920x1080x24" bash -c "..."
```

Flags: `-screen <screen> <width>x<height>x<depth>`. 1920×1080×24 is a reasonable default for modern desktop testing.

---

## 5. Animation Disabling for Test Reliability

### Finding

GTK4 toolkit-wide animations (transitions, scrolling, widget reveals) introduce **non-deterministic timing** that makes test assertions flaky. Dogtail may attempt to click a button before it finishes fading in, or query widget geometry mid-transition.

### Sources

- [GTK4 docs: `gtk-enable-animations`](https://docs.gtk.org/gtk4/property.Settings.gtk-enable-animations.html)
- Tails test suite disables animations alongside shadows.

### Recommendation

Add `GTK4` animation disabling to the test environment:

```bash
gsettings set org.gnome.desktop.interface gtk-enable-animations false
```

Or set it programmatically via environment before launching the app:

```bash
GSETTINGS_SCHEMA_DIR=/path gsettings set org.gnome.desktop.interface gtk-enable-animations false
```

---

## 6. Timing & Flakiness Patterns

### Finding

Fixed `sleep` calls are the primary source of test flakiness in GUI test suites. The current `test-gui-local` target uses:

- `sleep 1.5` after AT-SPI launch
- `sleep 3` after app launch

Both are vulnerable to system load variance.

### Recommendation

Adopt a **retry-with-timeout** pattern in test scripts:

```python
import time

def wait_for_app(name: str, timeout: float = 10.0):
    deadline = time.monotonic() + timeout
    while time.monotonic() < deadline:
        try:
            app = tree.root.application(name)
            if app.child_count > 0:
                return app
        except Exception:
            pass
        time.sleep(0.5)
    raise RuntimeError(f"Application '{name}' did not appear within {timeout}s")
```

This is more robust than a fixed `sleep 3` because it returns as soon as the app is ready, and fails with a clear message if it never appears.

---

## 7. Alternative/Complementary Tools

### Finding

| Tool | Mechanism | X11 | Wayland | Notes |
|------|-----------|-----|---------|-------|
| **Dogtail** | AT-SPI accessibility tree | ✓ | ✓ | Current choice. Best for widget-level interaction. |
| **xdotool** | X11 SendEvent | ✓ | ✗ | Fast, scriptable, but X11-only. Good for keyboard/mouse simulation alongside dogtail. |
| **ydotool** | Linux uinput | ✓ | ✓ | Works regardless of display backend. Lower-level (raw input). No widget awareness. |
| **LDTP** | AT-SPI (like dogtail) | ✓ | ✓ | Older, less maintained. Similar approach. |

### Recommendation

Dogtail is the correct primary tool. Consider **xdotool as a complement** for actions that are awkward through AT-SPI alone (e.g., global keyboard shortcuts, drag operations). Install it in the toolbox:

```bash
sudo dnf install -y xdotool
```

---

## 8. mss Screenshot Best Practices

### Finding

The current `take_screenshot_xvfb.py` uses `mss.mss().shot()` which captures **monitor 0** — this works because Xvfb exposes a single screen. However:

- If multiple monitors are configured in Xvfb, it captures the primary one only
- It does not verify that `DISPLAY` is set
- No error if the display is blank/dead

### Sources

- [ScreenshotOne: mss Python guide](https://screenshotone.com/blog/mss-python-screen-capture/) — "Make sure DISPLAY environment variable is set, or run with a display server."

### Recommendation

Add explicit display verification:

```python
import os
import mss

def take_screenshot(output_path: str):
    display = os.environ.get("DISPLAY")
    if not display:
        raise RuntimeError("DISPLAY not set — is Xvfb running?")
    
    with mss.mss() as sct:
        # Verify at least one monitor is usable
        if not sct.monitors:
            raise RuntimeError(f"No monitors found on DISPLAY={display}")
        sct.shot(output=output_path)
```

---

## 9. Concrete Improvement Recommendations

### Priority 1 — Fix Now

| # | Change | Effort | Impact |
|---|--------|--------|--------|
| 1 | **Disable GTK4 window shadows** via CSS in test env | Small | Prevents coordinate bugs before they appear |
| 2 | **Replace fixed sleeps with poll/retry** in justfile and test scripts | Medium | Eliminates the #1 source of flakiness |
| 3 | **Set explicit Xvfb resolution** (`1920x1080x24`) | Trivial | Prevents layout clipping, realistic screenshots |

### Priority 2 — Soon

| # | Change | Effort | Impact |
|---|--------|--------|--------|
| 4 | **Disable GTK4 animations** via GSettings | Trivial | Removes animation timing from test surface area |
| 5 | **Add retry-with-timeout** to `wait_for_app` in test scripts | Small | Makes tests self-healing on slow systems |
| 6 | **Configure dogtail** with config.json for logLevel and roleName compat | Small | Better debugging when tests fail |

### Priority 3 — Nice to Have

| # | Change | Effort | Impact |
|---|--------|--------|--------|
| 7 | **Switch to vhumpa/dogtail fork** from Fedora package | Medium | Better GTK4 support, config, active maintenance |
| 8 | **Add xdotool** as complementary input tool | Trivial | Keyboard shortcuts, drag-drop in tests |
| 9 | **Enhance mss script** with display verification | Trivial | Better error messages |
| 10 | **CI integration** — run `test-gui-local` in GitHub Actions or similar | Large | Catch regressions automatically |

---

## Sources

| # | Source | URL |
|---|--------|-----|
| 1 | Tails GitLab — GTK4 shadows + Dogtail | https://gitlab.tails.boum.org/tails/tails/-/issues/21022 |
| 2 | vhumpa/dogtail README | https://github.com/vhumpa/dogtail/blob/master/README.md |
| 3 | dogtail PyPI 1.0.8 | https://pypi.org/project/dogtail/1.0.8/ |
| 4 | GNOME at-spi2-core bus README | https://github.com/GNOME/at-spi2-core/blob/main/bus/README.md |
| 5 | Arch Linux — at-spi2-registryd control | https://bbs.archlinux.org/viewtopic.php?id=189975 |
| 6 | GTK4 gtk-enable-animations docs | https://docs.gtk.org/gtk4/property.Settings.gtk-enable-animations.html |
| 7 | Stack Overflow — Xvfb resolution | https://stackoverflow.com/questions/43637429/xvfb-xvfb-run-incorrect-resolution |
| 8 | ScreenshotOne — mss Python guide | https://screenshotone.com/blog/mss-python-screen-capture/ |
| 9 | Fedora Magazine — Automation through Accessibility | https://fedoramagazine.org/automation-through-accessibility/ |
| 10 | Freedesktop AT-SPI2 spec | https://www.freedesktop.org/wiki/Accessibility/AT-SPI2/ |
