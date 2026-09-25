# Technology Stack Summary

| Component | Technology |
|---|---|
| Kernel | Linux LTS + Continuum hardware-enablement patch set |
| Init & service management | systemd (or purpose-built init, ADR 0002), D-Bus IPC |
| Compositor | Custom Wayland compositor (wlroots-based) — "ContinuumWM" |
| Desktop shell | Rust, custom GPU-composited UI toolkit |
| Continuity daemons | Rust/C++, protobuf wire format, QUIC/WebRTC + BLE/Wi-Fi Direct transport |
| Android companion | Kotlin, Jetpack Compose, CameraX, Telecom/Telephony APIs |
| App framework | ContinuumKit SDK, Flatpak-derived sandbox, Wine/Proton-derived Windows compatibility |
| Security | UEFI Secure Boot, dm-verity, TPM-backed keystore, Ed25519 + Noise/TLS 1.3 |
| Update system | A/B atomic partitions, delta updates, signed manifests |

## Rationale notes

- **Rust for daemons and shell:** memory safety matters disproportionately
  here because continuity daemons hold kernel-adjacent privileges
  (Section 8) — a memory-safety bug in `continuum-controld` (input
  injection) or `continuum-camerad` (virtual device) is a
  privilege-escalation path, not just a crash. C++ is scoped only where
  a daemon must bind to an existing C/C++ library with no mature Rust
  equivalent.
- **protobuf over JSON/custom binary:** schema evolution (adding fields
  without breaking older Companion versions) is a hard requirement given
  the OS and the Android Companion update independently.
- **wlroots over a compositor written from scratch:** avoids
  re-solving GPU buffer management, multi-GPU, and protocol-compliance
  problems the Wayland ecosystem has already solved; Continuum's value
  is the shell built on top, not the compositor internals.
