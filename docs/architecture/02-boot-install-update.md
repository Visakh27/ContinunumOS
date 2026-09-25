# Boot, Install & Update

## 2.1 Boot chain

1. UEFI Secure Boot verifies the Continuum bootloader signature.
2. Bootloader (systemd-boot or GRUB, Continuum-signed) verifies and loads
   the kernel + initramfs via dm-verity.
3. Kernel mounts the read-only root partition and hands off to init,
   which starts core services and the continuity daemon suite.
4. ContinuumShell compositor starts and presents the login screen.

Implementation detail: `../../phase-0-foundation/boot/secure-boot/` and
`../../phase-0-foundation/boot/dm-verity/`.

## 2.2 Installer

- Graphical installer (live-boot USB image) handling disk partitioning,
  dual-boot detection, account sign-in, and first-run device pairing
  with an Android phone as part of setup (mirrors macOS's first-run
  Apple ID/Continuity setup).
- OEM pre-install path: a reference image and driver-certification kit
  for hardware partners, analogous to Windows OEM imaging or ChromeOS
  device certification. See `../../phase-6-oem-certification-ga/oem-cert-kit/`.

## 2.3 Update strategy

- Background A/B atomic updates, delta-compressed, verified against a
  signed manifest before activation.
- Staged rollout by device cohort with automatic halt on elevated
  crash/boot-failure telemetry (see `../../cross-cutting/telemetry-privacy/`).
- Kernel and driver updates decoupled from desktop-shell/continuity-daemon
  updates where possible, so continuity features can ship on a faster
  cadence than the kernel.

Implementation detail: `../../phase-0-foundation/boot/ab-update/`.

## 2.4 Recovery mode **[added]**

Not specified in the source plan, which only covers *automatic* rollback
on a failed update. A complete boot story also needs a user-initiated
path for cases A/B rollback doesn't cover: a healthy-looking update that
still leaves the user unable to log in, corrupted user data, or a lost
credential.

- A minimal recovery environment (separate small partition, not part of
  the A/B pair) reachable via a boot-time key combo or from the
  installer USB.
- Recovery actions: reinstall OS (preserving or wiping user data,
  user's choice), roll back to the other A/B slot manually, wipe and
  re-pair continuity trust anchors, export diagnostic logs.
- Recovery environment is verified-boot-covered like the main OS —
  it's a common attack target otherwise.

Detail: `../../cross-cutting/recovery-mode/README.md`.
