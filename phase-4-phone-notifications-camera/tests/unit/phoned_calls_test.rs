// Skeleton tests for call state and audio routing (not yet implemented).
#[cfg(test)]
mod phoned_calls {
    #[test] fn ringing_does_not_change_audio_routing() { todo!(); }
    #[test] fn active_with_hfp_connected_switches_to_call_device_and_remembers_previous() { todo!(); }
    #[test] fn manual_device_change_during_call_wins_and_is_not_restored_over() { todo!(); }
    #[test] fn ended_restores_previous_devices() { todo!(); }
    #[test] fn active_without_hfp_shows_phone_only_and_does_not_route() { todo!(); }
    #[test] fn session_loss_mid_call_marks_state_stale_and_hides_session_only_controls() { todo!(); }
    #[test] fn dial_only_accepted_from_user_action_in_phone_app() { todo!(); }
}
