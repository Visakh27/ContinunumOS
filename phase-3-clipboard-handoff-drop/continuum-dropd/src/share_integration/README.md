# Share and Files integration

- **Share sheet:** a system component. Any app's Share action lists
  paired devices that have Drop enabled; the app hands over the file
  through the sheet and needs no capability of its own.
- **Files app:** a "Nearby Device" sidebar entry for each paired device
  (per `../../../../docs/architecture/04-desktop-shell-ux.md`).
  Drag-and-drop onto the entry sends; incoming files land in the user's
  chosen folder only after quarantine passes.
- **Incoming prompts:** unknown-sender or large-file offers prompt via the
  native notification center
  (`../../../../phase-1-shell-compositor/shell-skeleton/notification-center/README.md`).
  Offers from a paired device within the size limit may auto-accept if
  the user enabled it.

Status: design only.
