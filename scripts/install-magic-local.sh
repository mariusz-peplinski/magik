#!/usr/bin/env bash
set -euo pipefail

usage() {
  cat <<'USAGE'
Install the local `magic` CLI into a user bin directory.

Usage:
  ./scripts/install-magic-local.sh [--build] [--copy|--link] [--install-dir DIR]

Options:
  --build            Force a fresh local build via ./build-fast.sh
  --copy             Copy the binary into install dir (default)
  --link             Symlink to code-rs/bin/magic for live repo updates
  --install-dir DIR  Install destination (default: $HOME/.local/bin)
  -h, --help         Show this help text
USAGE
}

SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" >/dev/null 2>&1 && pwd)"
REPO_ROOT="$(cd -- "${SCRIPT_DIR}/.." >/dev/null 2>&1 && pwd)"

INSTALL_DIR="${HOME}/.local/bin"
MODE="copy"
FORCE_BUILD=0

while [[ $# -gt 0 ]]; do
  case "$1" in
    --build)
      FORCE_BUILD=1
      ;;
    --copy)
      MODE="copy"
      ;;
    --link)
      MODE="link"
      ;;
    --install-dir)
      shift
      INSTALL_DIR="${1:-}"
      if [[ -z "${INSTALL_DIR}" ]]; then
        echo "Error: --install-dir requires a path" >&2
        exit 1
      fi
      ;;
    -h|--help)
      usage
      exit 0
      ;;
    *)
      echo "Unknown arg: $1" >&2
      usage
      exit 1
      ;;
  esac
  shift
done

SOURCE_BIN="${REPO_ROOT}/code-rs/bin/magic"
DEST_BIN="${INSTALL_DIR}/magic"

if [[ ${FORCE_BUILD} -eq 1 || ! -x "${SOURCE_BIN}" ]]; then
  echo "Building magic binary..."
  (
    cd "${REPO_ROOT}"
    ./build-fast.sh
  )
fi

if [[ ! -x "${SOURCE_BIN}" ]]; then
  echo "Error: binary not found at ${SOURCE_BIN}" >&2
  exit 1
fi

mkdir -p "${INSTALL_DIR}"

if [[ -e "${DEST_BIN}" || -L "${DEST_BIN}" ]]; then
  rm -f "${DEST_BIN}"
fi

if [[ "${MODE}" == "link" ]]; then
  ln -s "${SOURCE_BIN}" "${DEST_BIN}"
  echo "Installed symlink: ${DEST_BIN} -> ${SOURCE_BIN}"
else
  cp -f "${SOURCE_BIN}" "${DEST_BIN}"
  chmod +x "${DEST_BIN}"
  echo "Installed binary copy: ${DEST_BIN}"
fi

if command -v magic >/dev/null 2>&1; then
  echo "magic on PATH: $(command -v magic)"
else
  echo "Warning: 'magic' is not currently on PATH."
  echo "Add this to your shell profile if needed:"
  echo "  export PATH=\"${INSTALL_DIR}:\$PATH\""
fi

echo "Done. Try: magic --version"

