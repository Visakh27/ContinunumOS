# Tier-1 Reference Hardware List (Provisional)

Referenced by `kernel/VERSION_PIN`, `build-system/conf/local.conf.template`
(`MACHINE` variable), every HIL test in `tests/hil/`, and the
certification checklist in `../docs/architecture/03-driver-hardware-strategy.md`.
Filed as open item P0-1 in `../docs/issues/phase-0-open-items.md`.

**This list is a placeholder structure, not a real hardware decision.**
Selecting actual reference SKUs is a business/engineering decision
(pricing, OEM relationship, GPU vendor commitment) outside this
repository's scope — this file defines the *shape* that decision needs
to take so every downstream reference to "Tier-1 reference hardware"
has somewhere concrete to point once it's made.

## Required fields per device

| Field | Purpose |
|---|---|
| `device_id` | Short machine-readable slug (used as Yocto `MACHINE`, telemetry `device_class`, HIL lab inventory key) |
| CPU / GPU | Drives Kconfig fragment selection (`kernel/config/graphics.kconfig` vendor options) |
| Wi-Fi/BT chipset | Drives `kernel/config/continuity.kconfig` driver requirements |
| Display | Native resolution/refresh rate — feeds the Phase 1 compositor's baseline target and the boot-time budget (open item P0-9) |
| Storage | Must be sized for A/B's doubled-partition footprint (ADR 0003 consequence) |
| Ports | External-display/dock capability (`../cross-cutting/external-display/`) depends on this |
| Ownership | Who holds the physical unit(s) for the HIL lab |

## Provisional slots (fill in real hardware here)

| device_id | Role | Status |
|---|---|---|
| `tier1-reference-primary` | Primary HIL target — every CI kernel-patch-gate test (`../ci/pipelines/kernel-patch-gate.yaml`) and the first Secure Boot/A/B validation runs against this device | **Unselected** |
| `tier1-reference-secondary` | Second Tier-1 device, different GPU vendor (recommended: cover at least Intel + one discrete GPU vendor, since `graphics.kconfig` branches by vendor) | **Unselected** |
| *(additional Tier-1 devices)* | Grown incrementally toward GA; the Phase 6 certification sweep (`../phase-6-oem-certification-ga/tests/certification/tier1_full_compliance_test.md`) requires the full, final list | **Not started** |

## How to close this out
1. Pick `tier1-reference-primary` — this alone unblocks `kernel/VERSION_PIN`,
   the build-system `MACHINE` default, and every currently-unrunnable HIL
   test file.
2. Update this table with real make/model/specs.
3. Update `docs/issues/phase-0-open-items.md` (P0-1) to Closed.
4. Set the boot-time budget in `tests/hil/test_secure_boot_tamper_detection.md`
   (P0-9) once a first real boot on this device is timed.
