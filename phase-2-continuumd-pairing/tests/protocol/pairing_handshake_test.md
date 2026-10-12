# Protocol test skeleton: pairing handshake

Schema under test: `../../pairing/pairing.proto`.

- `test_valid_qr_confirmation_completes_pairing`: InitiatePairing -> Response{accepted} -> out-of-band QR match -> ConfirmPairing -> Response{confirmed, trust_anchor}
- `test_wrong_numeric_code_rejected_and_no_trust_anchor_written`
- `test_pairing_timeout_returns_to_pairable_state`
- `test_protocol_version_mismatch_declines_with_reason`: accepted=false plus decline_reason, never reaches confirmation
- `test_rotation_increments_epoch_and_reconnect_succeeds`
- `test_rotate_trust_anchor_rejects_stale_epoch`
- `test_revoke_pairing_requires_valid_signature`
- `test_malformed_frame_rejected_without_crash` (fuzz-style, every message type)

Not runnable: needs the emulated pairing harness and the `continuumd` implementation.
