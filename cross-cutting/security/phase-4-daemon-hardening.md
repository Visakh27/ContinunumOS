# Phase 4 Daemon Threat Model and Hardening Profile

Design-time only, like `phase-3-daemon-hardening.md`. It does not replace
an independent review of the implementation (open item P4-7). Same shared
baseline as Phase 3 (unprivileged user, `NoNewPrivileges`, read-only OS
view, seccomp allowlist, D-Bus policy restricting each daemon to its own
name and the `continuumd` methods it needs).

## continuum-phoned
Extra privilege: none beyond D-Bus (notification center, audio service).
No network socket, no input injection.

| Threat | Why it matters | Mitigation |
|---|---|---|
| Notification content leaks (one-time codes, private messages) | Mirrored text is among the most sensitive data on the phone | Only the shell and Phone app can read it (no third-party capability); never logged; never in telemetry; `PRIVATE` redacted by default; `SECRET` never sent |
| Spoofed notifications from a compromised paired phone | Fake "system" or security alerts for phishing | Always labeled with phone name and app name; visually distinct from native notifications; mirrored notifications can only send actions back to the phone, never run anything on the desktop |
| Flooding | Notification spam as denial of service | Per-app and global rate limits; drop with a visible "N suppressed" marker |
| Malformed icon image | Image decoder attack surface | Size cap, PNG only, decode with a hardened decoder in the daemon's sandbox, fuzzed |
| Unwanted calls placed | A compromised caller could dial premium numbers | `DIAL` only on a user action in the Phone app, never programmatically; confirmation for unusual numbers is open (P4-8) |
| Reply text injection | Desktop reply sent as the user | Replies sent only from an explicit send action in the Phone app |

## continuum-camerad
Extra privilege: create/remove a PipeWire camera node; read the data path
handle from `continuumd`. No network socket.

| Threat | Why it matters | Mitigation |
|---|---|---|
| Covert camera activation | Worst-case privacy failure | Desktop user gesture required; phone can't start streams; notification and Android indicator on the phone; Companion rejects requests not from a connected, enabled desktop |
| Hidden or spoofed indicator | A compromised daemon could hide that the camera is live | The "Phone camera active" indicator is drawn by the shell from the PipeWire node and `continuumd` state, never by `continuum-camerad` |
| Untrusted video decode | H.264 from the phone is attacker-controlled input to a complex decoder | Decode in a separate, tighter sandbox process (own seccomp profile, no filesystem, no network); fuzz the decode path; prefer a hardened decoder |
| Frame exfiltration | Any app with the camera grant sees frames | Per-app `Capability::Camera`; frames not written to disk by the daemon |
| Data-path abuse | Daemon talks to arbitrary hosts | Receives an already-connected handle only |
| Silent resume after reconnect | Stream returning without the user | No auto resume past the grace window |

## Cross-daemon isolation
Receiving a notification or call event must never be able to start the
camera, and neither daemon may call the other. Verified by D-Bus policy
tests (extend `phase-5-control-hotspot-winapp/tests/security/`).

## Not yet decided
- Which hardened H.264 decoder to use, and whether hardware decode is allowed given the attack surface.
- Rate-limit numbers.
- Confirmation UX for dialing from the desktop.

## Status
Design-time profile. Independent implementation review still required.
