// Skeleton tests for continuum-phoned notifications (not yet implemented).
#[cfg(test)]
mod phoned_notifications {
    #[test] fn secret_visibility_notification_never_mirrored() { todo!(); }
    #[test] fn private_notification_redacted_unless_app_opted_in() { todo!(); }
    #[test] fn same_key_update_replaces_existing_entry() { todo!(); }
    #[test] fn repost_with_older_posted_at_ignored() { todo!(); }
    #[test] fn new_app_off_by_default_until_user_enables() { todo!(); }
    #[test] fn removal_from_phone_does_not_generate_dismiss_request_back() { todo!(); }   // no echo
    #[test] fn desktop_dismiss_sends_dismiss_request_once() { todo!(); }
    #[test] fn action_on_removed_notification_returns_notification_gone() { todo!(); }
    #[test] fn flood_triggers_rate_limit_and_suppressed_marker() { todo!(); }
    #[test] fn oversized_or_non_png_icon_rejected_without_panic() { todo!(); }
    #[test] fn content_never_written_to_logs_or_telemetry() { todo!(); }
}
