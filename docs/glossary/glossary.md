# Glossary

- **Verified boot** — a boot chain where each stage cryptographically
  verifies the next before executing it, preventing unauthorized OS
  modification.
- **A/B (atomic) updates** — an update model with two OS partitions,
  where updates install to the inactive partition and only activate
  after a successful boot, enabling instant rollback.
- **Compositor** — the system component that combines all app windows
  into the final image shown on screen, and the layer at which Universal
  Control's input injection operates.
- **V4L2 (Video4Linux2)** — the Linux kernel API for video capture
  devices; exposing the phone camera as a V4L2 device makes it usable by
  any app, not just a bespoke driver.
- **Continuity daemon** — an always-running OS system service (not a
  user-installed app) implementing one continuity feature, with elevated
  privileges an app sandbox could not hold.
- **dm-verity** — a Linux kernel device-mapper target that verifies a
  block device's integrity against a signed hash tree at mount time.
- **Tier-1 / Tier-2 hardware** — Tier-1: a curated, fully driver-validated
  certified device list at launch. Tier-2: best-effort generic support
  for common hardware already supported upstream in Linux.
- **HIL (Hardware-in-the-loop)** — automated testing executed against
  physical reference devices rather than emulation.
- **SBOM (Software Bill of Materials)** — a published inventory of every
  component and dependency in a released artifact.
