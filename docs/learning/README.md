# Learning path

Use this directory for notes that explain the protocol and the design decisions in the implementation. Keep notes tied to milestones so the repository itself becomes a record of what you learned.

1. **Bencode and metainfo** — parse a `.torrent` file, preserve the exact encoded `info` bytes, and calculate its SHA-1 info hash.
2. **Piece model** — map file lengths to piece ranges and verify piece hashes.
3. **Tracker discovery** — understand announce requests/responses and compact peer lists.
4. **Peer wire protocol** — implement handshake, message framing, bitfields, and request/piece messages.
5. **Async orchestration** — use Tokio tasks, channels, cancellation, and timeouts to coordinate peers.
6. **Storage and resume** — write verified blocks safely and persist download state.
7. **Tauri desktop UI** — expose application commands and progress events while keeping protocol logic in Rust crates.

Each milestone should have a small explanation, tests for the protocol rules, and a runnable demonstration where practical.
