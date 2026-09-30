# Protocol test skeleton: pairing handshake

- `test_valid_qr_confirmation_completes_pairing`
- `test_wrong_numeric_code_rejected_and_no_trust_anchor_written`
- `test_pairing_timeout_after_N_seconds_with_no_confirmation`
- `test_re_pair_after_trust_anchor_rotation_succeeds`
- `test_malformed_pairing_frame_rejected_without_crash` (fuzz-style)

Run against the emulated pairing harness
(`../../../docs/testing/test-environment-setup.md`). Not runnable —
pending `../../pairing/` and `../../continuumd/` implementations.
