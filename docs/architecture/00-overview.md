# Architecture Overview

## What "new OS" means

Continuum OS is a new, independent desktop operating system, not an
application layer on top of an existing OS. It does not reinvent a CPU
instruction set, device drivers, or a kernel from zero — no shipping OS
does, including macOS (Darwin/XNU on a BSD/Mach lineage) or Android
(Linux kernel). The credible, buildable path — the one ChromeOS, SteamOS,
and every independent Linux-based desktop OS took — is:

1. Adopt a proven, actively-maintained open-source kernel (mainline Linux
   LTS) so existing GPU, Wi-Fi, Bluetooth, and peripheral drivers work on
   day one.
2. Build Continuum's actual differentiation on top of it: a new
   display/compositor stack, a new desktop shell and UX, a new
   system-service architecture, a new app framework, and — the point of
   the project — Android continuity as first-class, always-on
   kernel-adjacent daemons rather than a userland app.

## Layer model

| Layer | Responsibility | Key technology |
|---|---|---|
| Kernel | Process/memory management, scheduling, device drivers, networking stack | Linux LTS kernel, mainline + minimal Continuum patch set |
| Hardware Abstraction / Services | Power management, input, display mode-setting, radio (Wi-Fi/BLE) management | systemd, udev, NetworkManager/iwd, BlueZ (extended) |
| Compositor / Display Server | Window compositing, GPU acceleration, multi-monitor, virtual displays (for Universal Control) | Custom Wayland compositor ("ContinuumWM"), built on wlroots |
| System Services (Continuity Daemons) | Clipboard, Handoff, Drop, Phone, Control, Hotspot, Camera | Continuum daemons (Rust/C++), D-Bus/IPC, protobuf-based device protocol |
| Desktop Shell / UX | Menu bar, Dock, Spotlight-equivalent search, Mission-Control-equivalent, Finder-equivalent Files app | ContinuumShell (Rust + custom UI toolkit, GPU-composited) |
| App Framework & Compatibility | Native SDK, sandboxing, Linux app compatibility, Windows app compatibility | ContinuumKit SDK, Flatpak-style sandbox, Wine/Proton-based layer |
| Security & Update | Verified boot, A/B updates, app sandboxing, permission broker | Secure Boot chain, dm-verity, capability-based permission system |

See `08-technology-stack.md` for the component-level technology table.

## The one hard physical constraint

The phone remains a separate physical device running its own OS
(Android). No desktop OS can run code inside another vendor's phone
without something installed on that phone — this is true of macOS/iOS
Continuity too (Apple ships its own daemons on both sides because Apple
controls both OSes). Continuum OS ships its continuity logic as native
OS daemons on the desktop side, paired with an open-sourced **Continuum
Companion** service on the Android side, speaking the same protocol as
if it were built into Android — the same compromise every non-Apple
continuity project (KDE Connect, Phone Link, Samsung Flow) makes.

## Document index

- `01-kernel-and-base-system.md` — kernel choice, init, filesystem, update model, package format
- `02-boot-install-update.md` — boot chain, installer, update strategy
- `03-driver-hardware-strategy.md` — hardware tiers, driver enablement
- `04-desktop-shell-ux.md` — ContinuumShell design
- `05-app-ecosystem-compatibility.md` — ContinuumKit, Linux/Windows compat, browser
- `06-continuity-subsystem.md` — continuumd + feature daemons + Android companion
- `07-security-permission-model.md` — verified boot, sandboxing, pairing security
- `08-technology-stack.md` — component-level tech choices
- `09-additions-and-gaps.md` — items not in the source plan, added during scaffolding
- `../adr/` — point-in-time architecture decision records
