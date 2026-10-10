# Architecture

```text
apps/desktop (planned Tauri shell)
          |
crates/torrent-client (Tokio orchestration and CLI)
          |
crates/torrent-core (protocol types, parsing, validation)
```

The dependency direction points toward `torrent-core`. Core protocol types should not depend on Tokio, the command line, or Tauri. This makes the rules easier to test and keeps the desktop interface replaceable.

The CLI is the first application surface because it makes each milestone runnable before introducing a GUI. When the Tauri app is added, put its frontend and Tauri configuration under `apps/desktop`; share the engine through crates rather than copying protocol behavior into UI commands.
