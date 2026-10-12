// Skeleton tests for continuumd/src/discovery/ (not yet implemented).
#[cfg(test)]
mod discovery_state_machine {
    #[test] fn idle_to_scanning_on_start_discovery() { todo!(); }
    #[test] fn scanning_to_candidate_found_on_ble_match() { todo!(); }
    #[test] fn scanning_to_candidate_found_on_mdns_match() { todo!(); }
    #[test] fn candidate_found_to_pairable_after_version_handshake() { todo!(); }
    #[test] fn already_paired_reconnect_skips_scanning_state() { todo!(); }
}
