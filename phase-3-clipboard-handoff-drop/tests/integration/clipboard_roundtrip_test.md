# Integration test skeleton: clipboard round-trip

1. Copy text on emulated Companion.
2. Assert desktop clipboard receives it within the 500ms KPI budget
   (measured, not just asserted present).
3. Copy an image on desktop; assert Companion clipboard receives it.
4. Repeat with a small file; assert content-type negotiation picks the
   file path, not text/image.

Not runnable — pending implementation.
