# SigmaOS-zen-browser

Zen Browser workspace, theme, and profile integration in Rust for SigmaOS.

## Features

- Multi-workspace organization with pinned URLs and container colors
- Compact mode and sidebar layout configuration (`Left` / `Right`)
- Workspace validation (duplicate IDs, URL scheme validation)
- JSON workspace export and launch argument builder

## Usage

```bash
cargo run -- workspaces
cargo test
```

## License

GPL-3.0-or-later

## See Also

Related SigmaOS **browser** repositories:

- [SigmaOS-brave](../SigmaOS-brave/README.md) — Brave browser integration and launcher profile manager in Rust
- [SigmaOS-chromium](../SigmaOS-chromium/README.md) — Chromium browser profile and policy management for SigmaOS
- [SigmaOS-firefox](../SigmaOS-firefox/README.md) — Firefox profile and extension manager in Rust
- [SigmaOS-librewolf](../SigmaOS-librewolf/README.md) — LibreWolf hardened Firefox profile manager and policy auditor in Rust
- [SigmaOS-waterfox](../SigmaOS-waterfox/README.md) — Waterfox hardened profile manager in Rust
