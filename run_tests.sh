#!/usr/bin/env bash
set -euo pipefail

# === CONFIGURATION ===
# Path to the compiled binary; adjust if your binary is somewhere else:
BINARY="./target/debug/cool-rs"

if [ ! -x "$BINARY" ]; then
  echo "Binary not found or not executable: $BINARY"
  echo "Run: cargo build"
  exit 1
fi

# === HELPER FUNCTIONS ===
run_valid_test() {
  local file="$1"
  if "$BINARY" --file "$file" >/dev/null 2>&1; then
    echo "[PASS] valid:   $file"
  else
    echo "[FAIL] valid:   $file"
    exit 1
  fi
}

run_invalid_test() {
  local file="$1"
  if "$BINARY" --file "$file" >/dev/null 2>&1; then
    echo "[FAIL] invalid: $file (EXPECTED failure, but succeeded)"
    exit 1
  else
    echo "[PASS] invalid: $file"
  fi
}

# === MAIN ===
echo "=== Building project ==="
cargo build

echo "=== Running valid test cases ==="
for file in tests/valid/*.cl; do
  run_valid_test "$file"
done

echo "=== Running invalid test cases ==="
for file in tests/invalid/*.cl; do
  run_invalid_test "$file"
done

echo "=== ALL TESTS PASSED ==="
exit 0
