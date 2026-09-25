# Driver & Hardware Support Strategy

- **Tier 1 (launch):** a certified hardware list — a curated set of
  laptops/desktops (reference OEM devices) with full driver validation,
  similar to ChromeOS's device certification model, so v1 doesn't have
  to solve "all PC hardware ever made" on day one.
- **Tier 2:** best-effort generic support for common GPU (Intel/AMD/NVIDIA
  open or proprietary drivers), Wi-Fi/Bluetooth, and peripheral classes
  already supported upstream in Linux.
- A dedicated Hardware Enablement team maintains the Continuum kernel
  patch set, upstreaming fixes back to mainline Linux where possible to
  reduce long-term maintenance burden.
- Firmware/driver signing pipeline integrated with the update system so
  driver updates go through the same A/B atomic rollout as the OS.

## External display / dock / Thunderbolt-USB4 **[added]**

The compositor spec (`04-desktop-shell-ux.md`) assumes multi-monitor
support exists, but hot-plug docking (DisplayPort Alt Mode over USB-C,
Thunderbolt/USB4 tunneling, hub power delivery negotiation) is its own
hardware-enablement stream, not a compositor feature — it lives at the
kernel/udev layer and needs its own certification checklist alongside
the Tier-1 device list. Tracked under `../../cross-cutting/external-display/`.

## Certification checklist (feeds Phase 6)

A Tier-1 device is not "supported" until it passes, at minimum:
- Full boot chain (Secure Boot → dm-verity → login) with production keys.
- GPU acceleration for the compositor at native resolution/refresh rate.
- Wi-Fi, Bluetooth (BLE required for continuity pairing), audio in/out,
  camera, all USB-C/Thunderbolt ports, and the trackpad/keyboard.
- Suspend/resume and battery reporting accuracy.
- A/B update install and rollback exercised at least once on the
  physical unit.
- Continuity pairing and one feature daemon (clipboard) exercised
  end-to-end with a reference Android device.

See `../../phase-6-oem-certification-ga/oem-cert-kit/` for the full,
versioned compliance suite this checklist grows into.
