# Upakor-G14

A comprehensive Linux-native alternative to G-Helper for ASUS Zephyrus G14 laptops.

## Features

- **Power Management**: Switch between Quiet, Balanced, and Performance profiles
- **Battery Optimization**: Set charge limits to prolong battery health
- **Hardware Monitoring**: Real-time CPU/GPU temperatures, fan speeds, and power draw
- **GPU Switching**: Toggle between Integrated, Hybrid, and Dedicated GPU modes

## Requirements

- Linux with Wayland (GNOME recommended)
- asusd daemon
- supergfxctl service

## Installation

```bash
# Clone repository
git clone <repository-url>
cd upakorG14

# Install dependencies
bun install

# Build application
bun run tauri build

# Or run in development mode
bun run tauri dev
```

## Development

### Mock Mode (No Hardware)

```bash
cargo build --features mock
bun run tauri dev
```

### Real Hardware Mode

```bash
# Ensure asusd and supergfxctl are running
bun run tauri dev
```

## Architecture

- **Backend**: Rust with Tauri v2, zbus for D-Bus communication
- **Frontend**: React 19, TypeScript, TailwindCSS, TanStack Query

## License

MIT
