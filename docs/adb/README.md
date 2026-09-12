# ADB notes

- Prefer `adb -s <serial> <cmd…>` with discrete argv — never `shell("…"+user)`.
- Device state `device` required; `unauthorized` should show USB debugging UX.
- Storage totals via `df /data` (OEM variance expected).
- File listing via `ls -la`; pull via `adb pull`.
- Delete via `adb shell rm -- <path>` only after core policy checks.
