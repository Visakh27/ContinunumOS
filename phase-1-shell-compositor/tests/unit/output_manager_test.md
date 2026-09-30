# Unit test skeleton: OutputManager (physical + virtual)

Implementation under test (not yet written):
`../../continuumwm/src/output/`

- `test_physical_output_registered_on_hotplug`
- `test_physical_output_removed_on_hot_unplug_without_crash`
- `test_virtual_output_created_for_session_has_no_drm_connector`
  (regression guard: a VirtualOutput must never attempt real DRM mode-setting)
- `test_virtual_output_torn_down_on_session_end`
  (feeds Phase 5's session-loss handling —
  `../../../phase-5-control-hotspot-winapp/continuum-controld/README.md`)
- `test_output_trait_object_uniform_across_physical_and_virtual`
  (compile-time-ish check: code written against `dyn Output` must not
  need to downcast to know which variant it has, per the design intent
  in `../../continuumwm/src/output/README.md`)

Not runnable — pending `src/output/` implementation.
