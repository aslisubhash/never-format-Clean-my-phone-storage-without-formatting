# Never Format

**Clean phone storage without formatting — offline, verified, and under your control.**

Never Format is an open-source Windows desktop app (with a lightweight phone companion) that helps you understand what’s using space on your Android phone, back it up to your PC, verify the copy, and only then remove what you choose. No cloud accounts. No factory reset. No careless deletion.

> Your phone. Your files. Your PC.  
> **If Never Format cannot verify that an operation is safe, it does nothing.**

---

## Why it exists

Phones fill up with photos, videos, downloads, WhatsApp media, caches, and duplicates. Many tools either transfer files *or* clean aggressively — often without proving a backup succeeded. Never Format centers on one loop:

**Analyze → Protect → Verify → Clean**

You always see what will happen. Destructive steps need a verified PC copy and your confirmation.

---

## Features

### Storage insight
- See used / free space at a glance  
- Category breakdown (photos, videos, downloads, WhatsApp, cache, and more)  
- Large-file listing with clear safety labels  

### Safety-first classification
Every item is labeled so you know the risk before acting:

| Label | Meaning |
|---|---|
| Safe | Temporary / supported cache — cleanup allowed |
| Backup required | User files — delete only after verified PC backup |
| Review | Needs a human decision (large / old / ambiguous) |
| Protected | System, app data, current WhatsApp DB — never auto-deleted |
| Unknown | Uncertain — Never Format refuses to delete |

### Verified backup
- Copy photos, videos, documents, downloads, and more to your PC  
- Streamed SHA-256 verification (not “copy finished = success”)  
- Organized folders under `Never Format Backups / <device> / <date> / …`  

### Verified move
Free space on the phone only after:

1. Copy to PC  
2. Hash verify  
3. Your confirmation  
4. Delete on device  
5. Result check  

### Safe cleanup
- Dry-run plans before anything is removed  
- Supported temp / cache cleanup where Android allows it  
- Protected and unknown paths are blocked in the core engine (not just the UI)  

### WhatsApp (Android)
- Discover local WhatsApp backups  
- Keep the **latest** database protected  
- Older backups can be managed only after a verified PC copy  

### Companion app (Android + iOS)
A thin Flutter companion for permissions and approved operations. The **PC remains the control center** — the phone app is not a second full cleaner.

- **Android:** storage / cleanup assist, allowlisted ops  
- **iOS:** Photos / Files backup subset (not full storage cleaning)  
- No internet permission, no analytics, no cloud  

### Privacy & offline
- Works over USB with no network required for core features  
- No accounts, ads, or telemetry by default  
- Local logs and reports only — never file contents or secrets  

---

## How it works

```text
Phone (USB)  ←→  Desktop app (Windows)
                      ↓
                 Rust safety core
                 (scan · backup · verify · delete policy)
```

The UI never runs raw shell deletes. All risky operations go through a validated pipeline:

```text
Plan → Validate → Backup (if needed) → Verify → Confirm → Delete → Verify → Report
```

---

## Platform support

| Capability | Android | iOS |
|---|---|---|
| Full storage scan & cleanup | Yes | No |
| Photos / videos backup | Yes | Yes (subset) |
| WhatsApp backup management | Yes | Not supported |
| Companion app | Yes | Yes (limited) |

**Desktop:** Windows is the primary target. macOS / Linux are planned later.

---

## Download

Prebuilt builds are published via GitHub Actions:

- **Windows** — NSIS installer (`.exe`)  
- **Android companion** — release APK  

**Latest release:** [github.com/aslisubhash/never-format-Clean-my-phone-storage-without-formatting/releases](https://github.com/aslisubhash/never-format-Clean-my-phone-storage-without-formatting/releases)

Or grab CI artifacts from **Actions → Release builds → Artifacts** after a `main` build.

---

## How to use

### 1. Install the desktop app (Windows)
1. Download the installer from [Releases](https://github.com/aslisubhash/never-format-Clean-my-phone-storage-without-formatting/releases).  
2. Run it and open **Never Format**.  

### 2. Connect your Android phone
1. Enable **Developer options → USB debugging**.  
2. Plug in USB and allow debugging on the phone.  
3. In the app, open **Device** and refresh — your phone should appear.  

*(Optional)* Install the companion APK from Releases for permission and cleanup assist.

### 3. Analyze
Open **Dashboard** → **Analyze storage**. You’ll see:
- How much space you have  
- What’s using it  
- What can be cleaned safely  

### 4. Backup, then free space
1. **Backup** — choose files → **Backup & verify**.  
2. **Move** (optional) — copy + verify, then confirm delete on the phone.  
3. **Cleanup** — start with **Dry-run**, then execute only safe items.  

### 5. Review reports
**Reports** keeps operation history and verification outcomes locally on your PC.

---

## Build from source

### Prerequisites
- Rust (stable)  
- Node.js 20+  
- Flutter (companion)  
- Android `adb` (platform-tools) for real devices  

### Desktop
```bash
cd desktop
npm install
npm run tauri dev          # development (mock device if no phone)
npm run tauri build        # Windows installer → src-tauri/target/release/bundle/nsis/
```

### Companion APK
```bash
cd mobile
flutter pub get
flutter build apk --release
# → build/app/outputs/flutter-apk/app-release.apk
```

### Tests
```bash
cargo test -p never-format-core
cd mobile && flutter test
```

More detail: [docs/development](docs/development/README.md).

---

## Security

Never Format will not factory-reset, format storage, wipe partitions, unlock bootloaders, or clear app login data as part of normal workflows.

See [SECURITY.md](SECURITY.md) and [security/deletion-policy.md](security/deletion-policy.md).

---

## License

[Apache-2.0](LICENSE)

---

## Contributing

PRs welcome. Please read [CONTRIBUTING.md](CONTRIBUTING.md) — especially the rule: **prefer refusing a delete over risking user data**.
