# ClientScripts — Horizon 26.3 native mod (experimental)

## Upload to GitHub
Upload `horizon-mod.toml`, `Cargo.toml` and the `src` directory into the **repository root**. Do not upload a ZIP as the only file. This repository URL can then be used in Horizon Launcher > Mods > Add a native mod from GitHub.

## Version and testing
This project uses two **real commit hashes** published in existing 26.3 native mod documentation, not fake hashes. Those hashes are **old** and may not be compatible with the current Horizon launcher. It was not compiled here (no Horizon SDK or Rust compiler is installed). If a native build fails, inspect its build log and use `cargo horizon-mod update` for the installed Horizon revision.

## What actually works so far
- Contains the currently required flat project manifest, Cargo manifest, and Rust crate layout.
- Implements a minimal mod API hook that reports parser readiness when a player joins an integrated server, if Horizon loads the mod successfully.
- Implements an independent Rust `.csk` parser with independent files, line-number errors, script list, and reload logic in `src/scripts.rs`.
- Does NOT yet register `/cs` or load scripts during the game tick.
- Does NOT yet operate the cursor, attack, block placement, hotbar, or shield breaking. Those need the correct client host and game-input APIs.
- `.csk` scripts are examples only until a client event dispatcher is connected.

## Updating source revisions
Install Rust and the official Horizon `cargo-horizon-mod` tool, then prefer making an SDK-synced mod using:

```
cargo horizon-mod new client_scripts --template native
```

Compare the generated `horizon-mod.toml`, `Cargo.toml`, and `src/lib.rs` to this repository. The official generator fills the revision hashes for your build and fetches the API SDK, so its code takes precedence where incompatible.

Official docs: https://arismoko.github.io/horizon/start/first-mod.html
