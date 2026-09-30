# Print Stack

CUPS-based printing (IPP Everywhere / driverless printing as the primary
path — matches the "certified hardware, best-effort beyond that"
philosophy already used for Tier-1/Tier-2 device support,
`../../docs/architecture/03-driver-hardware-strategy.md`).

## Print dialog as a ContinuumKit system sheet
Not a per-app reimplementation — one print dialog component in
ContinuumKit, invoked via a standard API call, same integration pattern
as the accessibility-tree and IME work (`../accessibility/`,
`../localization/`): a system-level service every app gets by using
ContinuumKit rather than building its own.

```rust
pub trait PrintService {
    fn available_printers(&self) -> Vec<PrinterDescriptor>;
    fn print(&self, document: PrintDocument, printer: PrinterId, options: PrintOptions) -> PrintJobHandle;
    fn job_status(&self, handle: PrintJobHandle) -> PrintJobStatus;
}
// ContinuumKit's system print sheet calls this; apps call ContinuumKit,
// never PrintService directly — keeps printing gateable by the
// capability broker (../../phase-1-shell-compositor/app-runtime/src/capability-broker/)
// like camera/microphone/filesystem access.
```

## Driver model
IPP Everywhere covers most modern printers without a per-model driver.
Legacy/non-IPP printers are explicitly out of Tier-1 scope for v1,
mirroring the hardware-enablement tiering philosophy — CUPS' own driver
ecosystem remains available as a Tier-2-equivalent best-effort fallback.

## Status
Interface designed; capability-broker integration point identified. No
CUPS integration work done — CUPS isn't yet in any build-system image
manifest (`../../phase-0-foundation/build-system/manifests/`), added as
a Phase 1 image-layer item alongside audio (Session 16).
