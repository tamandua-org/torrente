# Torrente

A learning-oriented BitTorrent client written in Rust. The project is organized as a Cargo workspace so the protocol engine can stay independent of user interfaces.

## Workspace

- `crates/torrent-core` — protocol and domain logic. This crate should remain usable without Tokio or Tauri so its concepts can be learned and tested in isolation.
- `crates/torrent-client` — Tokio-powered command-line application and asynchronous orchestration around the core.
- `apps/desktop` — planned Tauri desktop interface; it will call the client/core through a deliberate application boundary rather than own protocol logic.
- `docs/learning` — learning notes, protocol walkthroughs, and design decisions.
- `tests/fixtures` — small, redistributable protocol fixtures for integration tests.

## Development

Requires Rust 1.85 or newer (edition 2024).

```sh
cargo check --workspace
cargo run -p torrent-client -- --help
```

The project is being built in milestones. The initial target is understanding and parsing metainfo before adding trackers, peer-wire networking, piece verification, and a desktop UI.

## License

Licensed under either [MIT](LICENSE-MIT) or [Apache-2.0](LICENSE-APACHE), at your option.
