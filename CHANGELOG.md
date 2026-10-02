# Changelog

## 0.1.1 — Android ADB core loop

- Recursive ADB scan via `find` so real phones populate photos/videos/cache
- Mock Galaxy only in debug builds (`DeviceManager::default`)
- Move confirm resumes verified delete (no re-copy)
- Cleanup dry-run no longer uses a fake mock transport
- Backup free-space gate when scan sizes are known
- Device/Dashboard UX for empty ADB and mock badge

## 0.1.0 — Foundation

- Monorepo: Tauri desktop, Rust core, Flutter companion
- Device discovery (ADB + mock + iOS probe)
- Storage scan, classifier, protected paths
- Verified backup, move, dry-run cleanup
- WhatsApp backup discovery with latest protection
- Companion allowlist protocol (Android + iOS capability sets)
- Offline-first SQLite operation history and reports
- CI: Windows NSIS installer + Android release APK artifacts (and tagged GitHub Releases)
