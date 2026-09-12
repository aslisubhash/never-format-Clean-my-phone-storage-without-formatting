# iOS companion scope

iOS cannot offer Android-parity storage cleaning.

Supported (P1):

- Photos library listing / export via companion
- Verified PC backup
- Optional delete of user-approved photos after hash verify

Not supported:

- Broad filesystem scan
- Cache cleanup
- WhatsApp database management

Desktop transport: `IosTransport` probes `idevice_id` (libimobiledevice). Full sync uses companion-assisted export.
