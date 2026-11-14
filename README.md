<div align="center">

# Melon MC

**Melon MC — High-performance Minecraft server core written in Rust.**

[![License: GPL-3.0](https://img.shields.io/badge/License-GPLv3-yellow.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/Rust-1.97%2B-orange.svg)](https://www.rust-lang.org/)
[![Minecraft](https://img.shields.io/badge/Minecraft%20Java-26.3%20(777)-brightgreen.svg)](https://minecraft.wiki/)
[![Bedrock](https://img.shields.io/badge/Minecraft%20Bedrock-1.26.60%20(2225)-blue.svg)](https://minecraft.wiki/)
[![Ko-fi](https://img.shields.io/badge/Support-Ko--fi-ff5e5b.svg)](https://ko-fi.com/mixitosik)

</div>

Melon MC is an ultra-fast, multi-threaded Minecraft server core built entirely in Rust. Designed for exceptional performance, low memory footprint, and high player density, Melon MC delivers vanilla gameplay parity alongside modern proxy and plugin capabilities.

---

## Highlights

- **Multi-Threaded Performance**: Leverages Rayon and Tokio with an entity-island scheduler to tick independent entity groups concurrently across available CPU cores.
- **Vanilla Parity**: Accurate combat calculations (armor toughness, shield blocking angles and knockback, sweeping edge, mace smashes), full redstone diodes and logic, mob AI goals, and vanilla advancements.
- **Massive Explosion Resilience**: Built-in explosion batching and rate limiting prevents server freeze and client timeouts even under detonations of 20,000+ TNT.
- **Modern Proxy Integration**: Native Velocity Modern Forwarding support with cryptographic verification, BungeeGuard, and BungeeCord integration.
- **Cross-Platform Support**: Built-in support for both Java Edition (Protocol 777 / 26.3) and Bedrock Edition (Protocol 2225 / 1.26.60) via RakNet and NetherNet.
- **Fast Chunk Storage**: Supports standard Anvil (`.mca`), Linear (`.linear`), and high-compression Pump (`.pump`) chunk storage formats.
- **WASM & Native Plugins**: Secure WebAssembly (WASI) component model plugin runtime via Wasmtime, alongside native dynamic library (`.dll` / `.so`) plugin loading.

---

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (version 1.85 or newer, 1.97+ recommended)
- Git

### Building from Source

```bash
# Clone the repository
git clone https://github.com/Raytolfas/Melon.git
cd melon

# Build release binary
cargo build --release --bin melon
```

The compiled binary will be placed at `target/release/melon` (or `target/release/melon.exe` on Windows).

### Running

To launch Melon MC:

```bash
./target/release/melon
```

On first run, Melon MC initializes default configuration files (`melon.toml`) and generates world dimensions (`minecraft:overworld`, `minecraft:the_nether`, `minecraft:the_end`) with instantaneous startup times.

---

## Configuration (`melon.toml`)

Melon MC is configured via `melon.toml`. Example key settings:

```toml
# Server listening address and online mode
[networking.java]
enabled = true
address = "0.0.0.0:25565"
online_mode = true
max_players = 1000

# Velocity Proxy configuration
[networking.proxy.velocity]
enabled = false
secret = "your_velocity_secret"

# World chunk format: "anvil", "linear", or "pump"
[world.chunk]
type = "pump"

# PvP and Combat rules
[pvp]
enabled = true
knockback = true
hurt_animation = true
```

---

## Plugins

Melon MC loads plugins placed inside the `./plugins/` directory:

1. **WebAssembly Plugins (`.wasm`)**: Compiled with target `wasm32-wasip1`, sandboxed with fine-grained host permissions.
2. **Native Plugins (`.dll` / `.so`)**: Compiled as Rust `cdylib` against `pumpkin-core`.

---

## Server Deployment

For Pterodactyl and Pelican panel users, an importable egg definition is available in [egg-melon.json](egg-melon.json).

---

## Support & Funding

If you find Melon MC helpful and want to support ongoing development, contributions are warmly welcomed:

[![Ko-fi](https://img.shields.io/badge/Ko--fi-mixitosik-ff5e5b?style=for-the-badge&logo=ko-fi&logoColor=white)](https://ko-fi.com/mixitosik)

---

## License & Attribution

- **Melon MC Core**: Licensed under the [GNU General Public License v3.0 (GPLv3)](LICENSE).
- **Plugin API (`pumpkin-plugin-api`)**: Dual-licensed under [MIT](crates/pumpkin-plugin-api/LICENSE-MIT) OR [Apache-2.0](crates/pumpkin-plugin-api/LICENSE-APACHE).
- **Upstream Heritage**: Melon MC is based upon and forked from the Pumpkin MC project. Copyright &copy; 2024–2026 Pumpkin MC Contributors, 2026 Melon MC Project.
- **Minecraft Assets**: Minecraft is a registered trademark of Mojang Synergies AB. See [assets/NOTICE.md](assets/NOTICE.md) for third-party asset licenses and attribution.
