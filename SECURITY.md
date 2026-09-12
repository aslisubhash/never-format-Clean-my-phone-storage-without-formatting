# Security Policy

## Reporting a vulnerability

Email security concerns privately to the maintainers (do not open a public issue for exploitable deletion/path bugs).

## Hard rules

- Never Format must not format devices, wipe partitions, factory reset, or unlock bootloaders.
- The companion app must never expose a remote shell or arbitrary delete API.
- Unknown / Protected classifications must never be auto-deleted.
- Delete eligibility requires verified size + SHA-256 match on the PC copy.

## Supported versions

| Version | Supported |
|---|---|
| 0.1.x | Yes |
