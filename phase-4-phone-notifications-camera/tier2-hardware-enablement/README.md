# Tier-2 Hardware Enablement

The Phase 4 exit criteria require best-effort support for hardware beyond
the certified Tier-1 list, demonstrated by a HIL pass on a Tier-2 sample
(`../../docs/architecture/03-driver-hardware-strategy.md`).

## What Tier-2 promises
Best effort, no certification: boots with verified boot, graphics,
Wi-Fi, Bluetooth and audio via upstream drivers. Continuity features may
be partial. Every Tier-2 device gets an honest row in a published support
matrix instead of a blanket claim.

## Sample selection
The sample is a rotating set chosen to cover the things that actually
vary and break continuity features:
- GPU vendor (Intel, AMD, NVIDIA open and proprietary)
- Wi-Fi chipset (decides Wi-Fi Direct / Wi-Fi Aware availability)
- Bluetooth chipset (classic HFP/SCO behavior differs a lot; ADR 0007)
- Firmware Secure Boot quirks
- Form factor and storage size (A/B doubles the OS footprint)
Number of devices and who provides them: open (P4-9).

## Test pass (subset of Tier-1 HIL)
1. Verified boot chain with enrolled keys; tamper refused.
2. A/B update and rollback.
3. Graphics at native resolution, suspend/resume.
4. Audio in/out; classic Bluetooth call audio over HFP.
5. BLE pairing with the OEM phone matrix.
6. Probe and record Wi-Fi Direct / Wi-Fi Aware availability.
7. Clipboard sync and one file drop end to end.

## Support matrix (template)
| Device | Boot/verified | Graphics | Wi-Fi | BT classic (calls) | BLE pairing | Wi-Fi Direct/Aware | Camera stream | Status |
|---|---|---|---|---|---|---|---|---|
| *(none selected yet)* | | | | | | | | |

Cell values: `works`, `partial (note)`, `untested`, `broken (issue link)`.

## Promotion and demotion
- Tier-2 to Tier-1 candidate: passes the full certification checklist
  and has an owner for ongoing HIL.
- Tracked, not gated: boot success per `device_class` from the opt-in
  boot-health beacon, to spot devices with problems. No numeric gate is
  set for Tier-2 (the 99.5% crash-free target applies to the certified fleet).

Status: plan and template only; no devices selected.
