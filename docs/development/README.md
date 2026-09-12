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

Debug desktop builds include a mock Android device when no ADB device is present.
