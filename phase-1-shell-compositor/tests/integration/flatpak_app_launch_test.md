# Integration test skeleton: Flatpak app launch under sandbox

**Level:** integration, headless compositor (software renderer),
no physical hardware.

1. Launch a reference Flatpak app (e.g. a simple GTK "hello world") under
   the app runtime.
2. Assert: app window appears as a compositor surface within N ms.
3. Assert: app cannot access the filesystem outside its sandbox scope
   without an explicit capability grant (negative test).
4. Close app; assert clean surface teardown, no compositor resource leak.

Not runnable yet — pending `../../app-runtime/` and `../../continuumwm/`
implementations.
