# fleetkeeper-core

Rust library for CAP-85 (Stellar Protocol 28) contract fleets. CAP-85 lets many contracts point to one owner contract's tag, so changing that one entry upgrades every member.

Modules to build, in this order:
1. rpc: Soroban RPC client (read ledger entries, simulate, send, poll).
2. fleet: decode external-ref executables and tags, keeping exact tag bytes; resolve members from a member list.
3. plan: build an upgrade plan showing current vs target Wasm hash.
4. simulate: simulate new code against each member and aggregate results.
5. tx: build, sign, and submit the owner tag-write transaction.
6. history: local JSON log of versions for rollback.

Rules: keep code simple, run cargo fmt and cargo clippy and cargo test before each commit, test only on testnet.
