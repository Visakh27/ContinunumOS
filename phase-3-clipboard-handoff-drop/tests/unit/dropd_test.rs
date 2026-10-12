// Skeleton tests for continuum-dropd (not yet implemented).
#[cfg(test)]
mod dropd {
    #[test] fn filename_with_path_separators_is_sanitized() { todo!(); }          // ../../etc/x
    #[test] fn filename_collision_gets_suffix_not_overwrite() { todo!(); }
    #[test] fn file_invisible_outside_quarantine_until_verified_and_scanned() { todo!(); }
    #[test] fn digest_mismatch_deletes_file_and_reports() { todo!(); }
    #[test] fn scan_unavailable_holds_in_quarantine_and_asks_user() { todo!(); }
    #[test] fn resume_uses_highest_contiguous_offset() { todo!(); }
    #[test] fn resume_rejected_if_trust_anchor_epoch_changed() { todo!(); }
    #[test] fn quarantine_quota_and_ttl_enforced() { todo!(); }
    #[test] fn concurrent_transfer_cap_enforced() { todo!(); }
}
