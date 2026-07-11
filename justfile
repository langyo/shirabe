# shirabe — browser automation via CDP.

set shell := ["bash", "-c"]
set windows-shell := ["bash.exe", "-c"]
set unstable
set lists

# Shared celestia-devtools recipes — NOT in git. Stage with: just fetch.
# `import?` silently skips when absent, so this justfile parses pre-fetch.
import? "./.just/git-bash-interop.just"
import? "./.just/celestia-devtools.just"

# Stage shared celestia-devtools recipes into .just/ (gitignored).
# Source order: explicit URL arg → local pip bundle (offline) → GitHub raw.
# curl honors HTTP_PROXY/HTTPS_PROXY/ALL_PROXY env vars automatically.
[script('bash')]
fetch URL='':
    #!/usr/bin/env bash
    set -euo pipefail
    out=.just/celestia-devtools.just
    mkdir -p .just
    if [ -n "{{URL}}" ]; then
      echo "[fetch] {{URL}} -> $out"
      curl -fsSL "{{URL}}" -o "$out"
    elif command -v celestia-devtools >/dev/null 2>&1; then
      src=$(celestia-devtools include-path)
      echo "[fetch] local bundle ($src) -> $out"
      cp "$src" "$out"
    else
      echo "[fetch] github raw -> $out"
      curl -fsSL "https://raw.githubusercontent.com/celestia-island/celestia-devtools/dev/src/celestia_devtools/common.just" -o "$out"
    fi
    echo "[fetch] wrote $out"

default:
    @just --list

# Format all sources.
fmt:
    cargo fmt --all

# Check formatting without writing.
fmt-check:
    cargo fmt --all -- --check

# Type-check all targets and features.
check:
    SHIRABE_SKIP_BROWSER_FETCH=1 cargo check --all-targets --all-features

# Clippy with -D warnings.
clippy:
    SHIRABE_SKIP_BROWSER_FETCH=1 cargo clippy --all-targets --all-features -- -D warnings

# Run the test suite.
test:
    SHIRABE_SKIP_BROWSER_FETCH=1 cargo test --all-features

# Build all features.
build:
    SHIRABE_SKIP_BROWSER_FETCH=1 cargo build --all-features

# One-shot local gate: fmt-check + clippy + test.
ci:
    just fmt-check
    just clippy
    just test

# ── npx distribution (local dry-run) ─────────────────────────────────────────
#
# These wrap the shared recipes from celestia-devtools.just with shirabe's
# metadata. CI does the actual publish (see .github/workflows/npm-release.yml);
# locally these only stage ./dist and run `npm pack --dry-run`.
#
#   just npm-dist-local                              # reassemble root from existing dist/
#   just npm-dist-local 0.1.0 path/to/shirabe x86_64-unknown-linux-gnu
npm-dist-local version='' binary='' target='':
    SHIRABE_SKIP_BROWSER_FETCH=1 just npm-dist shirabe {{version}} {{binary}} {{target}}
