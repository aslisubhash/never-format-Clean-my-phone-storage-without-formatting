# Architecture

## Layers

1. **React UI** — navigation, safety badges, confirmations
2. **Tauri IPC** — typed commands only
3. **Rust core** — DeviceManager, transports, scanner, classifier, backup, verify, cleanup, WhatsApp, companion, SQLite
4. **Transports** — ADB, MTP (probe), iOS (libimobiledevice probe), Mock, Companion protocol

## Capability gates

- `AndroidFull` — scan, backup, move, cleanup, WhatsApp
- `IosMediaSubset` — list/export/delete photos after verify

## Data

SQLite stores device metadata, operations, reports, preferences. Never stores file contents, passwords, or message bodies.
