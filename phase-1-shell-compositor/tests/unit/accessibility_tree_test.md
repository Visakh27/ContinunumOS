# Unit test skeleton: accessibility tree

Implementation under test (not yet written):
`../../app-runtime/src/accessibility-tree/`

- `test_standard_button_widget_implements_accessible_automatically`
- `test_custom_widget_without_accessible_impl_flagged_at_dev_time`
  (a lint/warning, not a runtime failure — feeds
  `../../../cross-cutting/developer-portal/` docs guidance)
- `test_accessibility_tree_reflects_live_widget_tree_changes`
  (a dynamically added child widget appears in the next tree query)

Not runnable — pending implementation. First real consumer (a screen
reader) not yet designed; this file specifies the producer side only.
