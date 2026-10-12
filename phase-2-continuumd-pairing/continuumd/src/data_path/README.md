# Data path (added Session 44)

Bulk and media traffic (file transfers, camera video) needs a
high-bandwidth path (Wi-Fi Direct, Wi-Fi Aware, or LAN), separate from the
control session.

**Why this lives in `continuumd`:** the daemon design principle is that
only `continuumd` opens network sockets, which is what bounds the network
attack surface to one daemon. Phase 3 first described `continuum-dropd`
as opening its own data path, which contradicts that. With two consumers
now (drop, camera) the contradiction would double, so the path is
negotiated and opened here.

## Interface sketch
```rust
pub struct DataPathRequest { pub consumer: Consumer, pub direction: Direction, pub min_kbps: u32 }
pub enum Consumer { Drop, Camera }
// Returns an already-connected, authenticated stream handle (file
// descriptor passed over a Unix socket). The feature daemon never sees
// addresses or credentials and cannot open other connections.
pub trait DataPathService { fn open(&mut self, req: DataPathRequest) -> Result<DataPathHandle, DataPathError>; }
```
- One path per consumer request, closed when the handle is dropped.
- Per-consumer limits (concurrent paths, bandwidth) enforced here.
- Falls back across Wi-Fi Aware, Wi-Fi Direct and LAN; fallback order is open (P3-8).

Design only.
