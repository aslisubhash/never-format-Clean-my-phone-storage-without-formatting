# Never Format

## Product Requirements Document (PRD)

**Version:** 1.1
**Status:** Product Definition / MVP Planning
**Platform:** Windows Desktop + Android Companion APK
**Model:** Open Source, Offline-First
**Product Type:** Android Storage Management, Backup, Transfer & Safe Cleanup

---

# 1. Product Overview

**Never Format** is an open-source desktop application that helps users safely manage Android phone storage from a Windows PC.

It combines:

* Storage analysis
* File browsing
* Backup
* Verified file transfer
* Safe cleanup
* Cache management where Android permits it
* WhatsApp backup management
* Duplicate and large-file detection
* Recovery-oriented operation handling
* Optional Android companion APK

The product is designed around one core principle:

> **Analyze → Protect → Verify → Clean**

The application should make it difficult for a user to accidentally lose important data.

### Product Promise

> **Your phone. Your files. Your PC. No cloud. No format.**

### Safety Promise

> **If Never Format cannot verify that an operation is safe, it does nothing.**

---

# 2. Problem Statement

Android phones accumulate large amounts of:

* Photos
* Videos
* Downloads
* Documents
* WhatsApp media
* WhatsApp backups
* APK files
* Archives
* Screenshots
* Temporary files
* Duplicate files
* Application cache

Existing tools often solve only one part of the problem.

Some applications focus on:

* Phone-to-PC transfer
* Android cleaning
* WhatsApp management
* File management
* Backup

But users still need to manually combine multiple tools.

More importantly, aggressive cleaning tools can create uncertainty around:

* What will be deleted
* Whether a file has actually been backed up
* Whether app data will be removed
* Whether the user will be logged out
* Whether a transfer actually completed
* Whether a cleanup operation can be reversed

Never Format addresses this by making **verification and safety the center of the product**.

---

# 3. Goals

## Primary Goals

1. Safely manage Android storage from Windows.
2. Allow users to back up important files to their PC.
3. Verify backups before deleting originals.
4. Free storage without deleting application data.
5. Preserve login sessions and app state.
6. Manage WhatsApp backups safely.
7. Work completely offline.
8. Be open source.
9. Provide transparent operation logs and reports.
10. Provide an Android companion APK for capabilities that require Android-side access.

---

# 4. Non-Goals

Never Format must NOT:

* Factory reset devices
* Format internal storage
* Format SD cards
* Wipe partitions
* Unlock bootloaders
* Lock/relock bootloaders as part of normal workflows
* Root devices
* Bypass Android security
* Extract passwords
* Extract authentication tokens
* Crack WhatsApp encryption
* Automatically delete application data
* Automatically delete credentials
* Upload user files
* Require cloud services
* Require an online account
* Require internet access for core functionality
* Perform hidden background deletion

---

# 5. Product Architecture

Never Format consists of two primary applications.

## 5.1 Never Format Desktop

The Windows desktop application is the **main control center**.

Responsibilities:

* Device discovery
* Device information
* Storage analysis
* File scanning
* Backup
* Verification
* File transfer
* Cleanup planning
* Cleanup execution
* WhatsApp backup management
* Reports
* Operation history
* Safety validation

Recommended technology:

* Tauri
* React
* TypeScript
* Rust
* SQLite
* Tailwind CSS
* shadcn/ui

---

# 6. Android Companion APK

## 6.1 Purpose

The Android APK is a lightweight companion application that enables Android-native functionality that cannot reliably be performed by the Windows application alone.

It should **not** become a second full-featured cleaner.

The PC remains the primary user interface.

### APK responsibilities

* Android-side permissions
* Android-side storage information
* Supported cleanup operations
* Device-side file scanning where useful
* Android-native APIs
* User approval for sensitive operations
* Communication with the desktop application
* Operation confirmation
* Security enforcement

---

# 7. APK Design Principles

The APK must be:

### Lightweight

It should have a small installation footprint and minimal background activity.

### Offline

The APK should work without internet access.

Ideally:

* No `INTERNET` permission
* No analytics SDK
* No cloud backend
* No account system
* No advertising

### Permission-minimal

Only request permissions when needed.

The application should explain:

> Why this permission is required
> What it allows
> What Never Format will do with it

### Safety-first

The APK must not expose a generic remote shell or arbitrary destructive commands.

It should expose a restricted set of approved operations.

For example:

```text
SCAN_STORAGE
LIST_FILES
READ_FILE_METADATA
COPY_FILE
VERIFY_FILE
REQUEST_CACHE_CLEANUP
EXECUTE_APPROVED_CLEANUP
GET_WHATSAPP_BACKUPS
```

It should NOT expose:

```text
FORMAT_DEVICE
WIPE_STORAGE
FACTORY_RESET
DELETE_ARBITRARY_PATH
UNLOCK_BOOTLOADER
CLEAR_APP_DATA
```

---

# 8. Desktop ↔ Android Communication

The primary connection should be:

**Android Phone ↔ USB ↔ Windows PC**

Possible communication mechanisms:

* ADB
* USB transport
* Android companion service
* MTP for normal file transfer

The communication layer should be abstracted behind a service interface.

Example:

```text
Desktop UI
    ↓
Tauri IPC
    ↓
Rust Core
    ↓
Device Manager
    ↓
ADB / MTP / Companion APK
    ↓
Android Device
```

The UI should never directly execute ADB commands.

---

# 9. Offline Requirement

Offline operation is a hard product requirement.

The following environment must be supported:

```text
PC Internet: OFF
PC Wi-Fi: OFF
Phone Wi-Fi: OFF
Phone Mobile Data: OFF
USB: Connected
```

Core features must continue working.

The application must not:

* Check online activation
* Require login
* Upload diagnostics
* Contact cloud APIs
* Require internet-based device authentication
* Download components during normal operation

---

# 10. Core User Flow

The primary experience should be:

```text
Connect Phone
      ↓
Detect Device
      ↓
Analyze Storage
      ↓
Identify Safe Opportunities
      ↓
Choose Action
      ↓
Backup if Required
      ↓
Verify Backup
      ↓
Ask for Confirmation
      ↓
Perform Cleanup
      ↓
Verify Result
      ↓
Generate Report
```

---

# 11. Safety Workflow

Every destructive operation must follow:

```text
PLAN
 ↓
VALIDATE
 ↓
BACKUP IF REQUIRED
 ↓
VERIFY
 ↓
USER CONFIRMATION
 ↓
DELETE
 ↓
VERIFY RESULT
 ↓
REPORT
```

Never Format must never jump directly from:

```text
Scan → Delete
```

for user files.

---

# 12. Storage Analyzer

The application should scan the phone and display:

* Total storage
* Used storage
* Free storage
* Category breakdown
* Largest files
* Potential duplicates
* Temporary files
* Cache
* WhatsApp storage
* Downloads
* Media
* Documents
* APKs
* Archives

### Categories

```text
Photos
Videos
Audio
Documents
Downloads
WhatsApp
APKs
Archives
Screenshots
Large Files
Duplicate Files
Temporary Files
Cache
Other
```

---

# 13. File Classification System

Every discovered item should receive a safety classification.

### SAFE_TO_CLEAN

Known temporary/cache content where deletion is supported.

### SAFE_AFTER_BACKUP

User files that can be deleted only after successful backup verification.

### REVIEW_REQUIRED

Examples:

* Large videos
* Duplicate files
* Old APKs
* Old downloads
* Large archives

### PROTECTED

Examples:

* Application data
* Authentication state
* Databases
* System files
* Current WhatsApp database
* Important Android directories

### UNKNOWN

Anything that cannot be confidently classified.

Unknown files must never be automatically deleted.

---

# 14. Backup System

Backup is one of the most important features.

Supported initial categories:

* Photos
* Videos
* Documents
* Downloads
* WhatsApp
* Music

Example destination:

```text
D:\Never Format Backups\
```

Recommended structure:

```text
Never Format Backups/
└── Samsung Galaxy S24/
    └── 2026-09-12/
        ├── Photos/
        ├── Videos/
        ├── Documents/
        ├── Downloads/
        ├── Music/
        └── WhatsApp/
            ├── Databases/
            └── Media/
```

---

# 15. Backup Verification

Never Format must not consider a file backed up merely because copying finished.

The system should verify:

1. File exists at destination.
2. File size matches.
3. SHA-256 hash matches where practical.
4. Destination is readable.
5. Backup operation is recorded.

Only then should the source become eligible for deletion.

### Example

```text
Phone:
IMG_1234.jpg
4.82 MB

        ↓ COPY

PC:
IMG_1234.jpg
4.82 MB

        ↓ VERIFY

SHA-256:
MATCH

        ↓

Eligible for deletion
```

---

# 16. Copy vs Move

The default operation must be:

> **Copy**

Never Format should make deletion an explicit second-stage operation.

For a Move operation:

```text
COPY
 ↓
VERIFY
 ↓
USER CONFIRMATION
 ↓
DELETE SOURCE
```

If verification fails:

> **Do not delete the original.**

If USB disconnects:

> **Do not delete the original.**

If the PC runs out of space:

> **Do not delete the original.**

If the application crashes:

> **Unverified originals remain untouched.**

---

# 17. Cleanup System

Cleanup should have safety levels.

## Very Safe

Examples:

* Supported temporary files
* Supported application cache
* Known temporary artifacts

## Safe After Verification

Examples:

* Backed-up photos
* Backed-up videos
* Backed-up documents
* Backed-up downloads

## Review Required

Examples:

* Large files
* Duplicate files
* Old APKs
* Old downloads
* Old media

## Protected

Examples:

* Application data
* Credentials
* Login state
* System files
* Unknown data

---

# 18. Application Cache

The product must distinguish between:

### Cache

Temporary/rebuildable data.

### Application Data

Potentially includes:

* Login information
* Databases
* Preferences
* Offline content
* User-created data
* Authentication state

Never Format must **never substitute application-data deletion for cache deletion**.

If Android/OEM restrictions prevent safe cache clearing:

> “Android does not allow Never Format to safely clear this app's cache on this device. Your app data and login state were left untouched.”

The user should be offered alternatives or allowed to skip.

---

# 19. WhatsApp Management

WhatsApp should have a dedicated section.

The application should detect supported WhatsApp backup locations.

Example:

```text
WhatsApp/
├── Databases/
└── Media/
```

Features:

* Detect WhatsApp backup
* Display backup dates
* Identify latest backup
* Backup WhatsApp to PC
* Verify backup
* Keep latest backup
* Identify older backups
* Allow manual deletion after confirmation

Never Format must never automatically delete the latest WhatsApp database.

---

# 20. WhatsApp Safety

The application must clearly distinguish:

```text
Current Backup
Older Backup
Media
Database
Unknown WhatsApp Files
```

Example warning:

> “This is the newest WhatsApp database found on the device. Never Format will protect it from automatic cleanup.”

---

# 21. Duplicate Finder

P1 feature.

The system can identify duplicates using:

* File size
* File name
* Metadata
* SHA-256 hash

A duplicate should only be considered a true duplicate after content verification.

Example:

```text
IMG_1234.jpg
IMG_1234 (1).jpg

Size: Same
SHA-256: Same

→ Duplicate
```

---

# 22. Large File Finder

Users can filter by:

```text
>100 MB
>500 MB
>1 GB
>5 GB
```

The user can then review or back up selected files.

Large files should never automatically be considered junk.

---

# 23. Dry Run

Before destructive cleanup, users can select:

> **Preview Cleanup**

The application should show:

```text
Files to remove: 247
Space to recover: 8.4 GB

Protected: 32
Requires backup: 97
Safe to clean: 118
Unknown: 12
```

Unknown files should not be included in automatic deletion.

---

# 24. Confirmation Screen

Before deletion:

```text
You are about to remove:

118 temporary/cache files
8.4 GB

97 personal files have been backed up
and verified.

Protected files will not be touched.

[Cancel]     [Confirm Cleanup]
```

For higher-risk actions, require additional confirmation.

---

# 25. Operation Recovery

The application should survive:

* USB disconnection
* PC shutdown
* Android reboot
* Transfer failure
* Destination drive disconnect
* Application crash

Operations should have states:

```text
PLANNED
RUNNING
PAUSED
COMPLETED
FAILED
PARTIALLY_COMPLETED
CANCELLED
```

The system should allow safe resumption where possible.

---

# 26. Reports

After major operations, generate a local report.

Example:

```text
Never Format Report

Device:
Samsung Galaxy S24

Backup:
4,218 files
42.7 GB

Verified:
4,218 files

Deleted:
4,218 files

Space Recovered:
42.7 GB

Errors:
0
```

Reports remain on the user's computer.

---

# 27. Local Database

Use SQLite.

Store:

* Device metadata
* Backup sessions
* File metadata
* Hashes
* Cleanup sessions
* Operation states
* Preferences
* Local reports

Do not store:

* File contents
* Passwords
* Authentication tokens
* Private message contents

---

# 28. Security Requirements

The system must protect against:

* Path traversal
* Arbitrary deletion
* Command injection
* Unsafe shell execution
* Malicious filenames
* Incorrect device targeting
* Accidental deletion
* Destination confusion
* Interrupted transfers

Avoid unsafe construction such as:

```text
shell("delete " + userProvidedPath)
```

Use structured APIs and validated arguments instead.

---

# 29. Protected Path System

The core should maintain a protected-path policy.

Examples:

```text
/system
/data
/vendor
/product
/metadata
```

and sensitive application directories where applicable.

The exact protection mechanism should vary by Android version and device capabilities.

Protected paths should be validated in the Rust core rather than trusted solely by the UI.

---

# 30. Architecture

Recommended architecture:

```text
┌───────────────────────────────┐
│       React / TypeScript      │
│             UI                │
└───────────────┬───────────────┘
                │
          Tauri IPC
                │
┌───────────────▼───────────────┐
│          Rust Core             │
│                               │
│ DeviceManager                 │
│ ADBManager                    │
│ MTPManager                    │
│ StorageScanner                │
│ FileClassifier                │
│ BackupManager                 │
│ VerificationManager           │
│ CleanupManager                 │
│ WhatsAppManager               │
│ ProtectedPathManager          │
│ SecurityManager               │
│ DatabaseManager               │
│ OperationLogger               │
└───────────────┬───────────────┘
                │
       ┌────────┴────────┐
       │                 │
      ADB               MTP
       │                 │
       └────────┬────────┘
                │
        Android Device
                │
       Never Format APK
```

---

# 31. Android APK Architecture

Suggested modules:

```text
NeverFormat Android
│
├── DeviceService
├── StorageService
├── PermissionService
├── CleanupService
├── FileService
├── WhatsAppService
├── PCConnectionService
└── SecurityService
```

The APK should expose only narrowly defined operations to the desktop application.

---

# 32. Desktop Technology Stack

### Frontend

* React
* TypeScript
* Tailwind CSS
* shadcn/ui

### Desktop Runtime

* Tauri

### Backend

* Rust

### Database

* SQLite

### Android

* Kotlin
* Android SDK

### Device Communication

* ADB
* MTP
* USB
* Android companion communication layer

---

# 33. Open Source Repository

Recommended structure:

```text
never-format/
│
├── desktop/
├── android/
├── core/
├── tests/
│   ├── unit/
│   ├── integration/
│   ├── security/
│   └── device/
│
├── docs/
│   ├── architecture/
│   ├── adb/
│   ├── whatsapp/
│   ├── android/
│   ├── offline/
│   └── development/
│
├── security/
│   ├── threat-model.md
│   └── deletion-policy.md
│
├── scripts/
│
├── README.md
├── LICENSE
├── CONTRIBUTING.md
├── SECURITY.md
├── CODE_OF_CONDUCT.md
└── CHANGELOG.md
```

The final open-source license should be selected before the public release.

---

# 34. MVP Scope

## P0 — Must Have

### Desktop

* Windows application
* USB device detection
* ADB detection
* MTP detection
* Device information
* Storage summary
* File scanning
* Storage categories
* Large file detection
* Photos/videos/documents/downloads backup
* WhatsApp backup
* SHA-256 verification
* Copy mode
* Verified Move mode
* Dry-run cleanup
* Supported cache cleanup
* Safe temporary cleanup
* Operation logs
* Reports
* Failure recovery
* Offline operation

### Android APK

* Lightweight companion
* Device identification
* Permission management
* Android storage information
* Supported Android-side cleanup operations
* Approved operation requests from PC
* User confirmation for sensitive operations
* Local USB communication
* No cloud
* No analytics
* No internet dependency

---

# 35. P1 Features

* Duplicate finder
* Advanced large-file analysis
* Backup resume
* Backup compression
* Encrypted backups
* Multiple backup destinations
* External drive support
* Multiple device profiles
* Local scheduled cleanup
* Detailed reports
* Improved Android-native scanning
* Backup snapshots

---

# 36. P2 Features

* Wireless ADB
* NAS backup
* macOS
* Linux
* Advanced WhatsApp media management
* Device migration
* Backup versioning
* Additional Android-native integrations

Wireless functionality should remain optional and must not introduce cloud dependence.

---

# 37. User Experience

The main dashboard should answer three questions immediately:

### 1. How much storage do I have?

```text
128 GB Used
38 GB Free
```

### 2. What's consuming it?

```text
Videos       42 GB
WhatsApp     18 GB
Photos       14 GB
Downloads     8 GB
Other        16 GB
```

### 3. What can I safely remove?

```text
Safe to clean          3.2 GB
Backup then remove     8.7 GB
Needs review           12.4 GB
Protected              27.8 GB
```

---

# 38. Main Navigation

```text
Dashboard
Storage
Files
Backup
Move
Cleanup
WhatsApp
Reports
Device
Settings
```

The interface should emphasize safety status rather than technical Android terminology.

---

# 39. Safety UI

Every operation should have a visible safety classification.

Example:

```text
🟢 Safe
🟡 Review
🔵 Backup Required
🔴 Protected
⚪ Unknown
```

Never hide important safety information behind advanced menus.

---

# 40. Permissions UX

The application should explain permissions in plain language.

Example:

> **USB Debugging Required**
>
> Never Format uses USB debugging to inspect your Android device and perform supported maintenance operations.
>
> No files are uploaded to the internet.
>
> [How to enable] [Continue]

For APK permissions:

> **Storage Access**
>
> This permission allows Never Format to identify files that may be consuming storage. It does not give Never Format permission to automatically delete protected data.

---

# 41. Performance Requirements

The application should:

* Handle large storage volumes
* Avoid loading entire files into memory
* Stream large-file hashing
* Process files asynchronously
* Provide progress indicators
* Allow cancellation where safe
* Avoid freezing the UI
* Resume interrupted operations where possible

---

# 42. Privacy

Privacy requirements:

* No cloud upload
* No telemetry by default
* No advertising
* No account
* No tracking
* No remote file analysis
* No external analytics

Local logs must not contain:

* Passwords
* Tokens
* Message contents
* Private document contents

If telemetry is ever introduced, it must be:

* Opt-in
* Documented
* Disabled by default
* Privacy-preserving

---

# 43. Testing

Testing must include:

### Unit Tests

* File classification
* Hash verification
* Path validation
* Protected paths
* Cleanup planning
* Storage calculations

### Integration Tests

* ADB
* MTP
* Android APK communication
* Backup
* Verification
* Cleanup

### Security Tests

* Path traversal
* Malicious filenames
* Arbitrary deletion attempts
* Command injection
* Incorrect-device targeting
* Permission failures

### Device Tests

Test across:

* Samsung
* Google Pixel
* Xiaomi
* OnePlus
* Motorola
* Other major Android OEMs

and multiple Android versions.

---

# 44. Failure Scenarios

## USB Disconnect During Backup

Expected behavior:

```text
Backup interrupted.

12,481 / 18,200 files verified.

Original files were NOT deleted.

[Reconnect & Resume]
```

## Hash Mismatch

```text
Verification failed.

Original file has been preserved.

The file is NOT eligible for deletion.
```

## Insufficient PC Storage

```text
Not enough space on the backup drive.

Required: 42.8 GB
Available: 17.2 GB

No files will be deleted.
```

## Android Permission Denied

```text
Never Format could not access this operation.

Nothing was deleted.

[View Required Permission]
```

---

# 45. Definition of Done

An MVP feature is complete only when:

* Happy path works
* Offline mode works
* Permission denial works
* USB disconnect is handled
* Transfer failure is handled
* User confirmation exists
* Operation is logged
* Security tests pass
* Protected paths remain protected
* Unverified files cannot be deleted
* Recovery/resume behavior is defined
* No cloud dependency exists

---

# 46. Success Metrics

Primary metrics:

### Safety

**0 confirmed data-loss incidents**

### Transfer reliability

**>99.5% verified successful transfers**

### Cleanup reliability

**>99% successful supported cleanup operations**

### Offline compliance

**100% of core MVP functionality works without internet**

### User outcome

Users should be able to perform their first safe cleanup in:

**<10 minutes**

---

# 47. Development Roadmap

## Phase 1 — Foundation

* Repository setup
* Tauri
* React
* TypeScript
* Rust
* SQLite
* CI
* Documentation
* Security policy

## Phase 2 — Device Layer

* ADB
* MTP
* Device detection
* Device information
* USB connection handling

## Phase 3 — Storage Layer

* File scanner
* Categories
* Storage dashboard
* Large files
* Protected paths

## Phase 4 — Backup

* Backup engine
* Destination management
* SHA-256 verification
* Reports
* Backup history

## Phase 5 — Move

* Copy
* Verify
* Confirmation
* Delete
* Recovery

## Phase 6 — Android APK

* Lightweight APK
* Device pairing/connection
* Permission handling
* Android-side services
* Restricted operation protocol
* Security validation

## Phase 7 — WhatsApp

* Backup discovery
* Backup
* Verification
* Latest-backup protection
* Older-backup management

## Phase 8 — Cleanup

* Dry run
* Safe cleanup
* Supported cache operations
* Review workflow
* Cleanup verification

## Phase 9 — Security & Release

* Threat model
* Security audit
* Device compatibility testing
* Documentation
* Public GitHub release
* Contribution workflow

---

# 48. Key Product Rule

The most important rule in the entire project is:

> **Never Format never deletes what it cannot prove is safe to delete.**

The application should favor:

**False negative over false positive.**

If Never Format incorrectly says:

> “I can't safely clean this.”

nothing bad happens.

If Never Format incorrectly says:

> “This is safe to delete.”

the user could lose data.

Therefore uncertainty must always result in:

> **DO NOTHING.**

---

# 49. Final Product Definition

**Never Format** is an open-source, offline-first Android storage management system consisting of:

### Never Format Desktop

The primary Windows application for:

* Storage analysis
* Backup
* Verified transfer
* Cleanup
* WhatsApp management
* Reports
* Device management

### Never Format Android

A lightweight Android companion for:

* Android-native permissions
* Device-side operations
* Supported cleanup
* Storage access
* Secure PC communication
* User approval

Together:

```text
                 NEVER FORMAT
                       │
          ┌────────────┴────────────┐
          │                         │
   Windows Desktop             Android APK
          │                         │
          └────────────┬────────────┘
                       │
                      USB
                       │
                  Android Phone
```

The product should remain:

**Offline. Open source. Backup-first. Verification-first. Privacy-first.**

And most importantly:

> **No cloud. No factory reset. No formatting. No careless deletion.**
