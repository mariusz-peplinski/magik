#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" >/dev/null 2>&1 && pwd)"
REPO_ROOT="$(cd -- "${SCRIPT_DIR}/.." >/dev/null 2>&1 && pwd)"

if [[ "${1:-}" == "-h" || "${1:-}" == "--help" ]]; then
  cat <<'USAGE'
Bootstrap a local development environment and build the CLI.

What it does:
  - Verifies Xcode Command Line Tools on macOS.
  - Installs rustup (and the default Rust toolchain) if missing.
  - Runs ./build-fast.sh (the canonical local check).

Usage:
  ./scripts/bootstrap-local.sh

Notes:
  - rustup is installed under ~/.cargo and ~/.rustup.
  - If you install rustup for the first time, add ~/.cargo/bin to PATH:
      . "$HOME/.cargo/env"
USAGE
  exit 0
fi

if [[ "$(uname -s)" == "Darwin" ]]; then
  if ! xcode-select -p >/dev/null 2>&1; then
    echo "Error: Xcode Command Line Tools are required." >&2
    echo "Run: xcode-select --install" >&2
    exit 1
  fi
fi

if ! command -v rustup >/dev/null 2>&1; then
  if ! command -v curl >/dev/null 2>&1; then
    echo "Error: curl is required to install rustup." >&2
    exit 1
  fi
  echo "rustup not found; installing..."
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
fi

if [[ -f "${HOME}/.cargo/env" ]]; then
  # shellcheck disable=SC1091
  . "${HOME}/.cargo/env"
fi

if ! command -v rustup >/dev/null 2>&1; then
  echo "Error: rustup is still not on PATH." >&2
  echo "Try: . \"$HOME/.cargo/env\"" >&2
  exit 1
fi

cd "${REPO_ROOT}"
./build-fast.sh

echo ""
echo "Built. Try: ./code-rs/bin/magik --version"
