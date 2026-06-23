# justfile — Build, run, test, and inspect the GTK4 office suite
# All build/run commands execute inside a toolbox for GTK4 dev libraries

# ── Configuration ───────────────────────────────────────────────────────
toolbox := env_var_or_default('GTK_BUILD_TOOLBOX', 'finupdate')
workspace := env_var_or_default('GTK_BUILD_WORKSPACE', '/var/home/james/dev/tuna-os/gtk-office-suite')
toolbox_image := "registry.fedoraproject.org/fedora-toolbox:43"

default:
    @just --list

# ── Toolbox ────────────────────────────────────────────────────────────

setup:
    @if ! toolbox list --containers | awk '{print $$2}' | grep -qx '{{toolbox}}'; then \
        echo "Creating {{toolbox}} toolbox..."; \
        toolbox create -y --image {{toolbox_image}} {{toolbox}}; \
    fi
    toolbox run --container {{toolbox}} sudo dnf install -y \
        cargo rust gcc \
        gtk4-devel libadwaita-devel pango-devel cairo-devel \
        openssl-devel
    @echo "Toolbox {{toolbox}} ready"

reset-toolbox:
    toolbox rm -f {{toolbox}} || true
    just setup

# ── Build & check ──────────────────────────────────────────────────────

check:
    toolbox run --container {{toolbox}} cargo check --manifest-path {{workspace}}/Cargo.toml --workspace

build:
    toolbox run --container {{toolbox}} cargo build --manifest-path {{workspace}}/Cargo.toml --workspace

test:
    toolbox run --container {{toolbox}} cargo test --manifest-path {{workspace}}/Cargo.toml --all-targets

lint:
    toolbox run --container {{toolbox}} cargo clippy --manifest-path {{workspace}}/Cargo.toml --all-targets -- \
        -D clippy::correctness -D clippy::suspicious \
        -W clippy::style -W clippy::complexity -W clippy::perf \
        -A deprecated -A unused

preflight: check lint test

# ── Smoke test (headless launch + verify) ──────────────────────────────

verify-letters: build
    #!/usr/bin/env bash
    set -euo pipefail
    pkill letters 2>/dev/null || true; sleep 0.5
    log=$(mktemp)
    timeout 6 toolbox run --container {{toolbox}} \
        env GSETTINGS_SCHEMA_DIR={{workspace}}/flatpak \
        {{workspace}}/target/debug/letters >"$log" 2>&1 &
    pid=$!; sleep 4
    if kill -0 "$pid" 2>/dev/null; then
        echo "PASS: Letters launched and running"
        kill "$pid" 2>/dev/null || true
    else
        echo "FAIL: Letters exited early"
        cat "$log"; exit 1
    fi

# ── Flatpak (for Broadway backend — GNOME SDK includes broadway GDK) ──

flatpak-letters:
    mkdir -p {{workspace}}/.flatpak-cache
    flatpak run org.flatpak.Builder \
        --force-clean --user --install --install-deps-from=flathub \
        --disable-rofiles-fuse \
        --state-dir={{workspace}}/.flatpak-cache/state \
        --repo={{workspace}}/.flatpak-cache/repo \
        {{workspace}}/.flatpak-cache/build flatpak/org.tunaos.letters-rust.json

flatpak-letters-broadway: flatpak-letters broadway-start
    @echo "Running Letters Flatpak via Broadway..."
    flatpak run --env=GDK_BACKEND=broadway --env=BROADWAY_DISPLAY=:5 org.tunaos.letters-rust &>/tmp/letters-flatpak-broadway.log &
    sleep 4
    @echo "Letters Flatpak at http://localhost:8085"

flatpak-letters-inspect: flatpak-letters-broadway
    sleep 2
    @echo "Running Playwright inspector via podman..."
    podman run --rm --network=host --volume {{workspace}}:/workspace:ro \
        mcr.microsoft.com/playwright/python:latest \
        python3 /workspace/skills/broadway-inspect/broadway_inspect.py letters

broadway-start:
    toolbox run --container {{toolbox}} pkill broadwayd 2>/dev/null || true; sleep 0.5
    toolbox run --container {{toolbox}} gtk4-broadwayd --port 8085 :5 &>/tmp/broadwayd.log &
    sleep 2
    @echo "Broadway at http://localhost:8085"

broadway-stop:
    toolbox run --container {{toolbox}} pkill broadwayd 2>/dev/null || true
    @echo "Broadway stopped"

# ── Broadway + Playwright inspection ───────────────────────────────────

letters-broadway: broadway-start build
    pkill letters 2>/dev/null || true; sleep 0.5
    toolbox run --container {{toolbox}} \
        env DBUS_SESSION_BUS_ADDRESS="" \
        GSETTINGS_SCHEMA_DIR={{workspace}}/flatpak \
        GDK_BACKEND=broadway BROADWAY_DISPLAY=:5 \
        {{workspace}}/target/debug/letters &>/tmp/letters-broadway.log &
    sleep 4
    @echo "Letters at http://localhost:8085"

tables-broadway: broadway-start build
    pkill tables 2>/dev/null || true; sleep 0.5
    toolbox run --container {{toolbox}} \
        env DBUS_SESSION_BUS_ADDRESS="" \
        GSETTINGS_SCHEMA_DIR={{workspace}}/flatpak \
        GDK_BACKEND=broadway BROADWAY_DISPLAY=:5 \
        {{workspace}}/target/debug/tables &>/tmp/tables-broadway.log &
    sleep 4
    @echo "Tables at http://localhost:8085"

decks-broadway: broadway-start build
    pkill decks 2>/dev/null || true; sleep 0.5
    toolbox run --container {{toolbox}} \
        env DBUS_SESSION_BUS_ADDRESS="" \
        GSETTINGS_SCHEMA_DIR={{workspace}}/flatpak \
        GDK_BACKEND=broadway BROADWAY_DISPLAY=:5 \
        {{workspace}}/target/debug/decks &>/tmp/decks-broadway.log &
    sleep 4
    @echo "Decks at http://localhost:8085"

letters-inspect: letters-broadway
    sleep 2
    @echo "Running Playwright inspector via podman..."
    podman run --rm --network=host --volume {{workspace}}:/workspace:ro \
        mcr.microsoft.com/playwright/python:latest \
        python3 /workspace/skills/broadway-inspect/broadway_inspect.py letters

tables-inspect: tables-broadway
    sleep 2
    podman run --rm --network=host --volume {{workspace}}:/workspace:ro \
        mcr.microsoft.com/playwright/python:latest \
        python3 /workspace/skills/broadway-inspect/broadway_inspect.py tables

decks-inspect: decks-broadway
    sleep 2
    podman run --rm --network=host --volume {{workspace}}:/workspace:ro \
        mcr.microsoft.com/playwright/python:latest \
        python3 /workspace/skills/broadway-inspect/broadway_inspect.py decks

# ── Cleanup ────────────────────────────────────────────────────────────

kill-all:
    toolbox run --container {{toolbox}} pkill broadwayd 2>/dev/null || true
    toolbox run --container {{toolbox}} pkill letters 2>/dev/null || true
    toolbox run --container {{toolbox}} pkill tables 2>/dev/null || true
    toolbox run --container {{toolbox}} pkill decks 2>/dev/null || true
    @echo "All killed"
