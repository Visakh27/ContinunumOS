#!/usr/bin/env bats
# Unit tests for kernel config fragment merging.
# Fragments under test: phase-0-foundation/kernel/config/*.kconfig
# Merge logic under test: (build-system, not yet implemented)

@test "base + security + continuity + graphics fragments merge without conflicting options" {
  skip "build-system merge tooling not yet implemented"
  # run merge-kconfig-fragments.sh over the four fragments in
  # phase-0-foundation/kernel/config/ and assert no option is set to
  # conflicting values across fragments (e.g. same CONFIG_ key, different value)
}

@test "merged config enables all options required by continuity.kconfig" {
  skip "build-system merge tooling not yet implemented"
  # Assert CONFIG_BT, CONFIG_BT_LE, CONFIG_VIDEO_V4L2LOOPBACK etc. present
  # in the merged output — regression guard against a future fragment
  # accidentally disabling one via a conflicting =n
}

@test "security fragment enables CONFIG_MODULE_SIG_FORCE" {
  skip "build-system merge tooling not yet implemented"
  # Regression guard: this option must never silently drop, since
  # unsigned-module loading defeats part of the verified-boot chain.
}
