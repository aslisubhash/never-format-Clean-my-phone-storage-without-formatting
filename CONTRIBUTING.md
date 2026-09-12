# Contributing

Thanks for helping build Never Format.

## Principles

1. Prefer false negatives over false positives for deletion.
2. All destructive paths go through Rust core validation.
3. No network required for core features; do not add analytics by default.
4. Keep the Flutter companion thin — PC is the control center.

## Dev workflow

1. `cargo test -p never-format-core`
2. `cd desktop && npm run build`
3. `cd mobile && flutter test`
4. Open a PR with a clear safety impact note if touching delete/backup/verify.

## Code of conduct

Be respectful. See [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md).
