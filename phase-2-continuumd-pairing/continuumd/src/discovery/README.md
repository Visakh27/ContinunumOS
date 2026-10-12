# Discovery

BLE advertisement/scanning + mDNS discovery — the two mechanisms
`continuumd` uses to find a paired (or pairable) phone, per
`../../../docs/architecture/06-continuity-subsystem.md#81-continuumd--the-core-continuity-daemon`.

## State machine

```
IDLE --start_discovery()--> SCANNING --candidate seen (BLE or mDNS)--> CANDIDATE_FOUND
CANDIDATE_FOUND --protocol version handshake ok--> PAIRABLE --> hand off to ../pairing/
```

Already-paired devices skip un-paired discovery: reconnect goes through
`session/` using the stored trust anchor. Discovery is only used to
*locate* the phone, since its BLE/mDNS address can change between
connections.

## Interface sketch
```rust
pub enum DiscoveryState { Idle, Scanning, CandidateFound(CandidateDevice), Pairable(CandidateDevice) }

pub trait DiscoveryBackend {
    fn start_ble_scan(&mut self);
    fn start_mdns_query(&mut self);
    fn stop(&mut self);
}
// BleDiscoveryBackend and MdnsDiscoveryBackend both feed the same
// DiscoveryState machine; a candidate can be found via either transport.
```

## Status
Interface + state machine design only.
