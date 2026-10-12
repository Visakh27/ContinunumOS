// Skeleton tests for continuum-camerad (not yet implemented).
#[cfg(test)]
mod camerad {
    #[test] fn stream_start_without_desktop_user_gesture_rejected() { todo!(); }
    #[test] fn phone_side_cannot_initiate_stream() { todo!(); }
    #[test] fn camera_busy_emits_placeholder_frames_not_a_stalled_node() { todo!(); }
    #[test] fn camera_returns_within_30s_resumes_else_ends_with_timeout() { todo!(); }
    #[test] fn lost_beyond_grace_ends_stream_and_does_not_auto_resume() { todo!(); }
    #[test] fn indicator_state_comes_from_shell_not_from_camerad() { todo!(); }
    #[test] fn frames_never_written_to_disk() { todo!(); }
    #[test] fn node_removed_on_stream_end_or_daemon_crash() { todo!(); }
}
