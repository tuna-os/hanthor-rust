# justfile — Build, run, and inspect GTK4 office suite apps via Broadway
# Broadway enables web-based GUI inspection (DOM scraping) without a display server
# All build/run commands execute inside the finupdate toolbox for GTK4 dev libraries

# ── Configuration ───────────────────────────────────────────────────────
toolbox := env_var_or_default('GTK_BUILD_TOOLBOX', 'finupdate')
workspace := env_var_or_default('GTK_BUILD_WORKSPACE', '/var/home/james/dev/tuna-os/gtk-office-suite')

default:
    @just --list

# ── Broadway daemon ────────────────────────────────────────────────────

broadway-start:
    @echo "Starting Broadway daemon on port 8085..."
    pkill broadwayd 2>/dev/null || true; sleep 0.5
    gtk4-broadwayd --port 8085 :5 &>/tmp/broadwayd.log &
    sleep 2
    @echo "Broadway running at http://localhost:8085"

broadway-stop:
    pkill broadwayd 2>/dev/null || true
    @echo "Broadway stopped"

# ── Build (inside toolbox) ─────────────────────────────────────────────

build:
    @echo "Building workspace in {{toolbox}} toolbox..."
    toolbox run --container {{toolbox}} cargo build --manifest-path {{workspace}}/Cargo.toml --workspace

check:
    @echo "Checking workspace in {{toolbox}} toolbox..."
    toolbox run --container {{toolbox}} cargo check --manifest-path {{workspace}}/Cargo.toml --workspace

# ── Run apps via Broadway (inside toolbox) ─────────────────────────────

letters-broadway: broadway-start
    pkill letters 2>/dev/null || true; sleep 0.5
    @echo "Starting Letters via Broadway in {{toolbox}} toolbox..."
    toolbox run --container {{toolbox}} \
        env DBUS_SESSION_BUS_ADDRESS="" \
        GSETTINGS_SCHEMA_DIR={{workspace}}/flatpak \
        GDK_BACKEND=broadway \
        BROADWAY_DISPLAY=:5 \
        {{workspace}}/target/debug/letters &
    sleep 4
    @echo "Letters running — http://localhost:8085"

tables-broadway: broadway-start
    pkill tables 2>/dev/null || true; sleep 0.5
    @echo "Starting Tables via Broadway in {{toolbox}} toolbox..."
    toolbox run --container {{toolbox}} \
        env DBUS_SESSION_BUS_ADDRESS="" \
        GSETTINGS_SCHEMA_DIR={{workspace}}/flatpak \
        GDK_BACKEND=broadway \
        BROADWAY_DISPLAY=:5 \
        {{workspace}}/target/debug/tables &
    sleep 4
    @echo "Tables running — http://localhost:8085"

decks-broadway: broadway-start
    pkill decks 2>/dev/null || true; sleep 0.5
    @echo "Starting Decks via Broadway in {{toolbox}} toolbox..."
    toolbox run --container {{toolbox}} \
        env DBUS_SESSION_BUS_ADDRESS="" \
        GSETTINGS_SCHEMA_DIR={{workspace}}/flatpak \
        GDK_BACKEND=broadway \
        BROADWAY_DISPLAY=:5 \
        {{workspace}}/target/debug/decks &
    sleep 4
    @echo "Decks running — http://localhost:8085"

# ── GUI inspection / verification ──────────────────────────────────────

letters-inspect: letters-broadway
    @echo "Waiting for app to render..."
    sleep 2
    @echo "Running Playwright DOM inspector..."
    python3 skills/broadway-inspect/broadway_inspect.py letters

tables-inspect: tables-broadway
    sleep 2
    python3 skills/broadway-inspect/broadway_inspect.py tables

decks-inspect: decks-broadway
    sleep 2
    python3 skills/broadway-inspect/broadway_inspect.py decks

letters-dom:
    @curl -s http://localhost:8085/ | head -50 || echo "Broadway not running. Run: just letters-broadway"

letters-verify: letters-broadway
    sleep 3
    @echo "=== DOM text check ==="
    @curl -s http://localhost:8085/ | wc -c | xargs echo "DOM size (chars):"
    @echo "=== Done ==="

# ── Cleanup ────────────────────────────────────────────────────────────

kill-all:
    pkill broadwayd 2>/dev/null || true
    pkill letters 2>/dev/null || true
    pkill tables 2>/dev/null || true
    pkill decks 2>/dev/null || true
    @echo "All killed"

# ── Full workflow ──────────────────────────────────────────────────────

# Build workspace + run Letters + inspect with Playwright
verify-letters: build letters-inspect kill-all
    @echo "=== Verification complete ==="
