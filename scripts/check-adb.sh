#!/usr/bin/env bash
set -euo pipefail
echo "Checking adb…"
if command -v adb >/dev/null 2>&1; then
  adb version
  adb devices -l
else
  echo "adb not on PATH — install Android platform-tools"
  exit 1
fi
