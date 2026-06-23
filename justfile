# justfile — Build, run, test, and inspect the GTK4 office suite
# All build/run commands execute inside a toolbox for GTK4 dev libraries

# ── Configuration ───────────────────────────────────────────────────────
toolbox := env_var_or_default('GTK_BUILD_TOOLBOX', 'finupdate')
workspace := env_var_or_default('GTK_BUILD_WORKSPACE', '/var/home/james/dev/tuna-os/gtk-office-suite')
toolbox_image := "registry.fedoraproject.org/fedora-toolbox:43"
# Debian has the GTK4 broadway backend enabled (Fedora doesn't)
broadway_toolbox := env_var_or_default('GTK_BROADWAY_TOOLBOX', 'debian-broadway')

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

# ── Broadway (Debian toolbox — Debian enables the GTK4 broadway backend) ─

broadway-setup:
    @if ! toolbox list --containers | awk '{print $$2}' | grep -qx '{{broadway_toolbox}}'; then \
        echo "Creating {{broadway_toolbox}} toolbox (Debian Trixie)..."; \
        toolbox create -y --image docker.io/library/debian:trixie {{broadway_toolbox}}; \
        toolbox run --container {{broadway_toolbox}} sudo apt-get update -y; \
        toolbox run --container {{broadway_toolbox}} sudo apt-get install -y \
            libgtk-4-dev libadwaita-1-dev libpango1.0-dev libcairo2-dev \
            cargo rustc gcc pkg-config curl; \
    fi
    @echo "Debian Broadway toolbox ready"

broadway-start:
    toolbox run --container {{broadway_toolbox}} pkill broadwayd 2>/dev/null || true; sleep 0.5
    toolbox run --container {{broadway_toolbox}} gtk4-broadwayd --port 8085 :5 &
    sleep 2
    @echo "Broadway at http://localhost:8085"

broadway-build:
    toolbox run --container {{broadway_toolbox}} cargo build --manifest-path {{workspace}}/Cargo.toml -p letters

letters-broadway: broadway-setup broadway-start broadway-build
    toolbox run --container {{broadway_toolbox}} pkill letters 2>/dev/null || true; sleep 0.5
    toolbox run --container {{broadway_toolbox}} \
        env GSETTINGS_SCHEMA_DIR={{workspace}}/flatpak \
        GDK_BACKEND=broadway BROADWAY_DISPLAY=:5 \
        {{workspace}}/target/debug/letters &
    sleep 4
    @echo "Letters via Debian Broadway at http://localhost:8085"

# ── Cleanup ────────────────────────────────────────────────────────────

kill-all:
    toolbox run --container {{toolbox}} pkill broadwayd 2>/dev/null || true
    toolbox run --container {{toolbox}} pkill letters 2>/dev/null || true
    toolbox run --container {{toolbox}} pkill tables 2>/dev/null || true
    toolbox run --container {{toolbox}} pkill decks 2>/dev/null || true
    @echo "All killed"
