// Skeleton tests for continuum-handoffd (not yet implemented).
#[cfg(test)]
mod handoffd {
    #[test] fn new_advertisement_replaces_previous_for_same_activity() { todo!(); }
    #[test] fn advertisement_expires_after_ttl_without_refresh() { todo!(); }
    #[test] fn advertisement_cleared_when_session_reaches_lost() { todo!(); }
    #[test] fn payload_over_cap_rejected() { todo!(); }
    #[test] fn payload_is_never_parsed_or_modified() { todo!(); }
    #[test] fn app_without_capability_cannot_advertise() { todo!(); }
    #[test] fn missing_app_uses_web_fallback_else_rejects_app_not_installed() { todo!(); }
    #[test] fn web_fallback_url_must_be_http_or_https() { todo!(); }
}
