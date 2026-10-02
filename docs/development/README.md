# Development

## Workspace

```text
Cargo.toml          # workspace: core + desktop/src-tauri
core/               # never-format-core
desktop/            # Vite + React + Tauri
mobile/             # Flutter companion
```

## Commands

```bash
cargo test -p never-format-core
cargo clippy -p never-format-core -- -D warnings
cd desktop && npm ci && npm run build
cd mobile && flutter test
```

## Mock device

Debug builds (`cfg!(debug_assertions)`) include a Mock Galaxy Android device when no real ADB
device is present. Release builds never inject the mock — connect a phone with USB debugging.

Real devices are scanned recursively via `adb shell find <root> -type f` (see `docs/adb/README.md`).
