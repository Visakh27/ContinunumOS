# Clipboard (Android side)

Kotlin implementation of `phase-3-clipboard-handoff-drop/continuum-clipboardd/clipboard.proto`.

```kotlin
class ClipboardSync(private val session: ContinuumSession) {
    fun onLocalClipChanged(clip: ClipData) { TODO() }          // build ClipboardEvent, send
    suspend fun onRemoteEvent(e: ClipboardEvent): ClipboardAck { TODO() } // apply + loop-guard
}
```

**Real Android constraint (high risk):** since Android 10, apps running in
the background cannot read the clipboard, and Android 12+ shows a toast
when an app reads it. Phone-to-desktop sync therefore cannot be fully
automatic from a plain background service. Options to evaluate, none
chosen yet: a foreground-triggered share/quick-settings action, an
accessibility-service path (Play policy risk, same class as the input
injection risk in the plan's risk register), or an input-method
integration. Desktop-to-phone (writing the clipboard) is not restricted
the same way. Until decided, treat phone-to-desktop clipboard as the
least certain Phase 3 feature.

Status: interface sketch only.
