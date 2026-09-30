# Phase 1 Test Plan — Display, Compositor & Shell Skeleton

**Covers:** `phase-1-shell-compositor/`

- **Unit:** compositor output/surface management logic, window-manager
  layout algorithms, Dock/menu-bar state machines — all without a real
  GPU (mocked backend).
- **Integration:** headless compositor session bring-up + a Flatpak
  reference app launching, resizing, closing under ContinuumWM in CI
  using a software renderer (llvmpipe).
- **HIL:** GPU-accelerated multi-monitor bring-up, virtual-desktop
  switching, and the sandboxed app-runtime on Tier-1 reference hardware
  — this is where real GPU driver behavior gets exercised.
- **Exit gate:** boots to a usable desktop with window management and a
  handful of ported Linux apps, per the Phase 1 milestone
  (`ROADMAP.md`), demonstrated by the HIL suite passing on all Tier-1
  devices.

## Skeleton index (as of Session 20)

| File | Level | Covers |
|---|---|---|
| `tests/unit/window_manager_layout_test.md` | unit | Snap/tile layout branches |
| `tests/unit/output_manager_test.md` | unit | Physical hot-plug, virtual-output lifecycle |
| `tests/unit/input_privilege_boundary_test.md` | unit | Injected-input constraints (binding on Phase 5) |
| `tests/unit/capability_broker_test.md` | unit | Grant/deny/re-prompt behavior, per-app scoping |
| `tests/unit/accessibility_tree_test.md` | unit | Automatic `Accessible` implementation, live-tree updates |
| `tests/integration/flatpak_app_launch_test.md` | integration | Sandbox launch/teardown, resource cleanup |
| `tests/integration/app_runtime_end_to_end_test.md` | integration | Sandbox + capability broker + accessibility tree together |
| `tests/integration/menubar_status_sources_test.md` | integration | Continuum Center mock + audio routing-priority, wired at UI layer |
| `tests/integration/notification_dnd_test.md` | integration | Notification center + DND + dismiss-observer hook |
| `tests/hil/multi_monitor_gpu_accel_test.md` | HIL | Real GPU-composited multi-monitor |
| `tests/hil/virtual_desktop_switch_perf_test.md` | HIL | Switch latency, RTL layout parity |
| `tests/hil/sandbox_capability_prompt_hil_test.md` | HIL | Real-hardware capability-prompt rendering/interaction |

All entries are skeletons (no implementation to test against yet — see
`phase-1-shell-compositor/README.md`'s status). HIL entries additionally
blocked on Tier-1 hardware selection (open item P0-1).

Skeleton root: `phase-1-shell-compositor/tests/`.
