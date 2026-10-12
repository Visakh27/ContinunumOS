# Protocol test skeleton: malformed phone input

Fuzz `clipboard.proto`, `handoff.proto` and `drop.proto` parsers (bounded
runtime in CI, long runs pre-release). Required properties for every message type:
- truncated, oversized, and wrong-type fields rejected without panic
- length fields larger than the remaining buffer rejected
- unknown fields ignored (forward compatibility), unknown enum values handled
- no unbounded allocation from attacker-controlled sizes
Not runnable until implemented.
