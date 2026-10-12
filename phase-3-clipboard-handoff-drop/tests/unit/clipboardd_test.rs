// Skeleton tests for continuum-clipboardd (not yet implemented).
#[cfg(test)]
mod clipboardd {
    #[test] fn remote_write_does_not_echo_back_to_sender() { todo!(); }          // loop guard by event_id
    #[test] fn concurrent_copies_resolve_by_lamport_clock_then_device_id() { todo!(); }
    #[test] fn text_over_inline_limit_becomes_file_reference() { todo!(); }
    #[test] fn sensitive_clip_not_synced_by_default() { todo!(); }
    #[test] fn sensitive_clip_never_appears_in_logs_or_telemetry() { todo!(); }
    #[test] fn event_dropped_while_session_lost_is_not_replayed() { todo!(); }
    #[test] fn disabled_feature_rejects_inbound_with_reason() { todo!(); }
}
