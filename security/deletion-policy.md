# Deletion Policy

## Absolute bans

Never Format must never:

- Factory reset
- Format internal storage or SD cards
- Wipe partitions
- Unlock / relock bootloaders
- Clear application data / credentials
- Delete `PROTECTED` or `UNKNOWN` items automatically

## Eligibility

A phone file may be deleted only when **all** are true:

1. Path passes `protected_paths::assert_deletable`
2. Safety class is `SAFE_TO_CLEAN`, or `SAFE_AFTER_BACKUP` with verified PC copy
3. For backup-required items: dest exists, readable, size recorded, SHA-256 matches
4. User explicitly confirmed (except dry-run)
5. Operation is not marked dry-run

## Uncertainty rule

If unsure → **DO NOTHING**.
