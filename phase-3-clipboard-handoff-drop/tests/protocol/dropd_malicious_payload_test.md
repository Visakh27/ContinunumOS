# Protocol test skeleton: continuum-dropd malicious payload rejection

- `test_eicar_test_file_rejected_before_reaching_files_app`
- `test_oversized_transfer_rejected_with_clear_error`
- `test_transfer_resumes_after_simulated_wifi_direct_drop`
- `test_transfer_does_not_resume_past_session_trust_anchor_rotation`
  (security: a resumed transfer must not implicitly re-trust a revoked pairing)

Not runnable — pending `../../continuum-dropd/` implementation.
