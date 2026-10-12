# Phase 3 Daemon Threat Model and Hardening Profile

The phase exit criteria require a sandboxing security review of the three
daemons before public alpha. This is the design-time half: threats and
required hardening. **It does not replace an independent review of the
implementation**, which cannot happen until there is code. Tracked as an
open item.

## Shared baseline (all three daemons)
- Dedicated unprivileged user per daemon; `NoNewPrivileges`.
- Read-only OS view (`ProtectSystem=strict`), private `/tmp`, no home
  directory access except the paths listed below.
- Seccomp syscall allowlist per daemon (derived once implemented).
- Control traffic only via `continuumd`'s D-Bus surface; D-Bus policy
  restricts each daemon to its own name and the `continuumd` methods it
  needs. No daemon may call another feature daemon directly.
- Input from the phone is untrusted: parse with bounded sizes, reject
  unknown-length-prefix abuse, fuzz in CI (`tests/protocol/`).

## Per-daemon

| Daemon | Extra privilege | Network | Filesystem | Main threats | Required mitigation |
|---|---|---|---|---|---|
| `continuum-clipboardd` | Clipboard read/write via compositor | None | Own runtime dir only | A paired device (or one that compromised the session) overwriting the clipboard to hijack a paste, e.g. swapping a crypto address; leaking password-manager content | Sensitive-content rule; per-feature toggle; size caps; visible indicator on remote write; no logging of content |
| `continuum-handoffd` | None beyond D-Bus | None | Own runtime dir only | Malicious activity payload or URL; spoofed activity title; payload used to attack the app that resumes | Opaque capped payload; never parsed by the OS; resume goes to the app's own handler, not a shell; web fallback URL restricted to http/https and shown to the user |
| `continuum-dropd` | Receives a connected data-path handle from `continuumd` (no socket creation) | None of its own | Quarantine dir (quota, TTL) and user folder (write after verification only) | Malware delivery; path traversal via filename; disk-fill by abandoned transfers; resuming under a revoked pairing; scanner bypass | Quarantine-first; filename sanitization; SHA-256 verify; scan; quota and TTL; trust-anchor epoch check on resume |

## Cross-daemon isolation requirement
A compromised `continuum-clipboardd` must not be able to reach
`continuum-controld` (Phase 5). Verified by D-Bus policy tests in
`phase-5-control-hotspot-winapp/tests/security/`; this document sets the
requirement now so Phase 3 daemons are written to it.

## Not yet decided
- Which scanner backs `av_scan` (ClamAV-class vs. platform service) and how
  signatures update on a read-only root.
- Whether clipboard sync of images needs content sanitization (decoder
  attack surface).

## Status
Design-time threat model and profile. Independent implementation review
is still required before alpha.

**Update (Session 44):** `continuum-dropd` no longer opens its own data path; `continuumd`'s `data_path` service does, which keeps network access in one daemon.
