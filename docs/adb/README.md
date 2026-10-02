# ADB notes

- Prefer `adb -s <serial> <cmd…>` with discrete argv — never `shell("…"+user)`.
- Device state `device` required; `unauthorized` should show USB debugging UX.
- Storage totals via `df /data` (OEM variance expected).
- Recursive file listing via `find <root> -type f` with `-exec stat -c '%s\t%n' {} ;` for sizes (fallback: path-only `find`).
- Caps: 20k files per root; 8k under `/sdcard/Android/data`.
- Pull via `adb pull`.
- Delete via `adb shell rm -- <path>` only after core policy checks.
