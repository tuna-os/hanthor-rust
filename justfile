# justfile — Build, run, test, and inspect the GTK4 office suite
# All build/run commands execute inside a toolbox for GTK4 dev libraries
# Broadway verification uses a Debian Sid podman container (has GTK4 broadway backend)

# ── Configuration ───────────────────────────────────────────────────────
toolbox := env_var_or_default('GTK_BUILD_TOOLBOX', 'finupdate')
workspace := env_var_or_default('GTK_BUILD_WORKSPACE', '/var/home/james/dev/tuna-os/gtk-office-suite')
toolbox_image := "registry.fedoraproject.org/fedora-toolbox:43"
broadway_container := "broadway-letters"

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

# ── Smoke test ─────────────────────────────────────────────────────────

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

# ── Broadway (Debian Sid podman container — only Debian has GTK4 broadway) ──

broadway-setup:
    @if ! podman container exists {{broadway_container}} 2>/dev/null; then \
        echo "Creating {{broadway_container}} container (Debian Sid)..."; \
        podman run -d --name {{broadway_container}} --network=host \
            -v {{workspace}}:/workspace:Z \
            docker.io/library/debian:sid sleep infinity; \
        podman exec --user root {{broadway_container}} bash -c ' \
            export DEBIAN_FRONTEND=noninteractive; \
            apt-get update -qq; \
            apt-get install -y --no-install-recommends \
                ca-certificates libgtk-4-dev libadwaita-1-dev libgtk-4-bin \
                cargo rustc gcc pkg-config curl \
                libpango1.0-dev libcairo2-dev libglib2.0-dev; \
        '; \
    fi
    @echo "Broadway container ready"

broadway-build: broadway-setup
    @echo "Building Letters in Broadway container..."
    podman exec broadway-letters bash -c 'export HOME=/tmp; cd /workspace; cargo build -p letters 2>&1' | tail -3
    @echo "Build complete"

broadway-start:
    podman exec broadway-letters pkill broadwayd 2>/dev/null || true; sleep 0.5
    podman exec -d broadway-letters gtk4-broadwayd --port 8085 :5
    sleep 2
    @echo "Broadway at http://localhost:8085"

letters-broadway: broadway-build broadway-start
    podman exec broadway-letters pkill letters 2>/dev/null || true; sleep 0.5
    podman exec -d broadway-letters bash -c ' \
        export GDK_BACKEND=broadway BROADWAY_DISPLAY=:5; \
        export GSETTINGS_SCHEMA_DIR=/workspace/flatpak; \
        /workspace/target/debug/letters \
    '
    sleep 4
    @echo "Letters via Broadway at http://localhost:8085"

letters-inspect: letters-broadway
    sleep 2
    python3 skills/broadway-inspect/broadway_inspect.py letters

broadway-stop:
    podman exec broadway-letters pkill broadwayd 2>/dev/null || true
    podman exec broadway-letters pkill letters 2>/dev/null || true
    @echo "Broadway stopped"

# ── Cleanup ────────────────────────────────────────────────────────────

kill-all:
    podman exec {{broadway_container}} pkill broadwayd 2>/dev/null || true
    podman exec {{broadway_container}} pkill letters 2>/dev/null || true
    toolbox run --container {{toolbox}} pkill letters 2>/dev/null || true
    @echo "All killed"
