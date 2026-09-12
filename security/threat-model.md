# Threat Model (v0.1)

## Assets

- User media and documents on phone and PC
- WhatsApp backup files
- Local SQLite metadata (paths, hashes — not file contents)

## Trust boundaries

1. **UI → Rust core** — UI may request ops; core validates paths, safety class, and eligibility.
2. **Core → Device transport** — structured argv / APIs only; no shell string concatenation.
3. **Desktop ↔ Companion** — allowlisted ops only; forbidden wipe/format/shell.

## Adversaries

- Malicious filenames (`../`, metacharacters)
- Accidental user confirmation fatigue
- Wrong-device targeting (mitigate with serial binding on every op)
- Compromised companion build (limit blast radius via allowlist)

## Mitigations

| Threat | Mitigation |
|---|---|
| Path traversal | `protected_paths` + normalize |
| Arbitrary delete | Safety class + eligibility gate |
| Command injection | No shell; argv arrays |
| Unverified delete | SHA-256 match required |
| iOS overclaim | Capability negotiation |

## Out of scope

- Physical attacker with unlocked phone
- Malicious ADB host already trusted by the user
