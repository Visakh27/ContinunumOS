# Calls (Android side)

Reports `CallState` and, as a fallback, executes `CallControl`.

- Call state via telephony callbacks (needs the phone-state permission).
  The incoming number on recent Android versions needs extra access that
  Google Play restricts, so the Companion does not rely on it: the caller
  number reaches the desktop over Bluetooth hands-free (ADR 0007), which
  carries caller ID.
- Answer/reject/hang-up from the session only where Android allows it
  without being the default dialer; otherwise rely on HFP control. Which
  commands are available without default-dialer status must be verified
  per Android version (open item P4-1 dependency).
- Contact-name lookup is optional (contacts permission) and off by default.
Interface sketch only.
