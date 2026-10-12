# Handoff (Android side)

Implements `phase-3-clipboard-handoff-drop/continuum-handoffd/handoff.proto`.

```kotlin
interface HandoffProvider {                 // implemented by participating Android apps
    fun currentActivity(): HandoffActivity?
    fun resume(payload: ByteArray)
}
class HandoffService { fun advertise(a: HandoffActivity) { TODO() }; fun withdraw(id: String) { TODO() } }
```

Android has no OS-level view of "the active document" for other apps, so
phone-to-desktop Handoff only works for apps that integrate a small
Continuum SDK (or via browser-tab sharing from the Continuum Browser).
Desktop-to-phone resume is simpler: open the web fallback URL or deep
link. Coverage will be narrower than the desktop side at alpha; the
developer portal docs should say so.

Status: interface sketch only.
