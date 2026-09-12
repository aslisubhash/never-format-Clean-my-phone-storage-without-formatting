# Never Format

Open-source, offline-first Android (and iOS subset) storage management from your Windows PC.

> **Your phone. Your files. Your PC. No cloud. No format.**

> **If Never Format cannot verify that an operation is safe, it does nothing.**

## Architecture

- **Desktop** (`desktop/`) — Tauri 2 + React + TypeScript + Tailwind
- **Core** (`core/`) — Rust library: device transports, scan, classify, backup, verify, cleanup, WhatsApp, companion protocol
- **Mobile** (`mobile/`) — Flutter companion for Android + iOS (thin permissions / allowlisted ops)

```text
React UI → Tauri IPC → Rust Core → ADB / MTP / iOS / Companion → Phone
```

## Safety pipeline

```text
PLAN → VALIDATE → BACKUP IF REQUIRED → VERIFY → USER CONFIRMATION → DELETE → VERIFY RESULT → REPORT
```

## Quick start

### Prerequisites

- Rust (stable)
- Node.js 20+
- Flutter (for companion)
- Android platform-tools (`adb`) for real devices

### Download prebuilt binaries (CI)

GitHub Actions builds:

- **Windows installer** (NSIS `.exe`) — artifact `never-format-windows`
- **Android companion APK** — artifact `never-format-android-apk`

After a push to `main` (or a manual **Release builds** workflow run), open:

**Actions → Release builds → (latest run) → Artifacts**

Tag a version to publish a GitHub Release with the same files attached:

```bash
git tag v0.1.0
git push origin v0.1.0
```

### Desktop

```bash
cd desktop
npm install
npm run tauri dev
```

Without a phone connected, a **mock device** appears in debug builds so you can exercise the full UI.

On Windows, local installer build:

```bash
cd desktop
npm ci
npm run tauri build
# → desktop/src-tauri/target/release/bundle/nsis/
```

### Core tests

```bash
cargo test -p never-format-core
```

### Flutter companion

```bash
cd mobile
flutter pub get
flutter test
flutter build apk --release
# → mobile/build/app/outputs/flutter-apk/app-release.apk
```

## Platform scope

| | Android | iOS |
|---|---|---|
| Full storage scan / cleanup | Yes (P0) | No |
| Photos/Videos backup | Yes | Yes (P1 subset) |
| WhatsApp backup mgmt | Yes | Out of scope |

## License

Apache-2.0 — see [LICENSE](LICENSE).

## Security

See [SECURITY.md](SECURITY.md) and [security/](security/).
