#!/usr/bin/env bash
set -euo pipefail

echo "[ci-tests] Running curated integration tests..."
if [[ "${SKIP_CARGO_TESTS:-0}" != "1" ]]; then
  pushd code-rs >/dev/null

  cargo test -p code-login --tests -q
  cargo test -p code-chatgpt --tests -q
  cargo test -p code-apply-patch --tests -q
  cargo test -p code-execpolicy --tests -q
  cargo test -p mcp-types --tests -q

  popd >/dev/null
fi


echo "[ci-tests] CLI smokes with host binary..."
BIN="${CI_CLI_BIN:-}"
if [[ -z "${BIN}" ]]; then
  if [[ -x ./code-rs/target/dev-fast/magic ]]; then
    BIN=./code-rs/target/dev-fast/magic
  elif [[ -x ./code-rs/target/debug/magic ]]; then
    BIN=./code-rs/target/debug/magic
  fi
fi

if [[ -z "${BIN}" || ! -x "${BIN}" ]]; then
  echo "[ci-tests] CLI binary not found; building debug binary..."
  pushd code-rs >/dev/null
  cargo build --bin magic -q
  popd >/dev/null
  BIN=./code-rs/target/debug/magic
fi

"${BIN}" --version >/dev/null
"${BIN}" completion bash >/dev/null
"${BIN}" doctor >/dev/null || true

echo "[ci-tests] Done."
