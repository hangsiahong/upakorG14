# Upakor-G14 Design Document

**Date:** 2026-03-14
**Status:** Approved
**Author:** Design Review via Brainstorming

---

## Overview

Upakor-G14 is a comprehensive Linux-native alternative to G-Helper for ASUS Zephyrus G14 laptops. Built with Tauri v2 (Rust backend, React/Tailwind frontend), optimized for GNOME Wayland.

**Target Hardware:** ASUS Zephyrus G14 (2024/2025 models)
**System Services:** asusd (xyz.ljones.Asusd), supergfxctl (org.asuslinux.supergfxctl)

---

## Architecture

### Layered Trait-Based Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                      React Frontend                         │
│  (Dashboard + Detail Windows for Power/Fans/Lighting/AniMe)  │
└────────────────────────────┬────────────────────────────────┘
                             │ Tauri IPC (invoke/emit)
┌────────────────────────────┴────────────────────────────────┐
│                    Tauri Commands Layer                     │
│           (lib.rs - exposes safe public API)                │
└────────────────────────────┬────────────────────────────────┘
                             │
┌────────────────────────────┴────────────────────────────────┐
│                    Trait Abstraction Layer                  │
│  ┌─────────────────┐  ┌─────────────────┐                  │
│  │   AsusdTrait    │  │ SupergfxctlTrait│                  │
│  └────────┬────────┘  └────────┬────────┘                  │
└───────────┼────────────────────┼────────────────────────────┘
            │                    │
    ┌───────┴────────┐   ┌──────┴─────────┐
    │  RealDBus impl │   │  Mock impl     │
    │  (zbus proxies)│   │  (fake data)   │
    └────────────────┘   └────────────────┘
            │                    │
    ┌───────┴────────────────────┴─────────┐
    │     D-Bus System Bus                 │
    │  xyz.ljones.Asusd + org.asuslinux... │
    └──────────────────────────────────────┘
```

### Key Decisions

- **Feature flag `mock`**: Controls implementation selection at compile time
- **Trait objects**: All hardware access goes through trait methods
- **Tauri commands**: Thin wrappers that call trait methods and return JSON
- **State management**: React Context API for global state, TanStack Query for server state

---

## Component Structure

### Backend (Rust)

```
src-tauri/src/
├── main.rs                          # Tauri entry point
├── lib.rs                           # Tauri commands (public API)
├── traits/
│   ├── mod.rs
│   ├── asusd.rs                     # AsusdTrait definition
│   └── supergfxctl.rs               # SupergfxctlTrait definition
├── implementations/
│   ├── mod.rs
│   ├── real_dbus/
│   │   ├── mod.rs                   # RealDBusFactory
│   │   ├── asusd.rs                 # zbus proxy implementation
│   │   └── supergfxctl.rs           # zbus proxy implementation
│   └── mock/
│       ├── mod.rs                   # MockFactory
│       ├── asusd.rs                 # Mock implementation
│       └── supergfxctl.rs           # Mock implementation
├── models/
│   ├── mod.rs
│   ├── power.rs                     # PowerProfile, ChargeLimit
│   ├── gpu.rs                       # GpuMode, GpuStatus
│   ├── fan.rs                       # FanCurve, FanSpeed
│   ├── lighting.rs                  # AuraMode, Brightness
│   └── hardware.rs                  # Temperature, PowerDraw
└── utils/
    ├── errors.rs                    # Error types and conversion
    └── state.rs                     # Global app state
```

### Frontend (React)

```
src/
├── main.tsx                         # Entry point
├── App.tsx                          # Root component
├── lib/
│   └── tauri.ts                     # Tauri API wrappers
├── components/
│   ├── Dashboard/
│   │   ├── Dashboard.tsx            # Main dashboard overview
│   │   ├── MetricCard.tsx           # Temp, fan, power widgets
│   │   └── QuickActions.tsx         # Profile/GPU quick switches
│   ├── Power/
│   │   ├── PowerPanel.tsx           # Detail window for power
│   │   ├── ProfileSelector.tsx
│   │   └── ChargeLimit.tsx
│   ├── Fans/
│   │   ├── FanPanel.tsx             # Detail window for fans
│   │   └── FanCurveEditor.tsx       # Interactive curve editor
│   ├── Lighting/
│   │   ├── LightingPanel.tsx        # Detail window for RGB
│   │   └── AuraControl.tsx
│   └── AniMe/
│       └── AniMePanel.tsx           # Detail window for AniMe
├── hooks/
│   ├── useHardwareState.ts          # Real-time hardware data
│   ├── usePowerProfiles.ts          # Power profile operations
│   └── useGpuSwitching.ts           # GPU mode operations
├── context/
│   └── HardwareContext.tsx          # Global hardware state
└── types/
    └── tauri.ts                     # TypeScript types from Rust
```

---

## Data Flow

### Read Operations (Dashboard updates)

```
React Component → useHardwareState hook → Tauri command →
Trait method → RealDBus/Mock → D-Bus → Return JSON → React state
```

### Write Operations (User actions)

```
User clicks → React handler → Tauri command → Trait method →
RealDBus → D-Bus method call → Result returned → UI update
```

### Real-time Monitoring

```
zbus Signal Stream → Rust async task → Tauri event →
React event listener → State update → Re-render
```

---

## Error Handling

### Error Hierarchy

```rust
#[derive(Debug, thiserror::Error)]
pub enum UpakorError {
    #[error("D-Bus connection failed: {0}")]
    DbusConnection(String),

    #[error("D-Bus method call failed: {0}")]
    DbusMethod(String),

    #[error("Property not supported by this device")]
    NotSupported,

    #[error("Invalid value: {0}")]
    InvalidValue(String),

    #[error("Permission denied: requires root/privileged access")]
    PermissionDenied,

    #[error("Service not available: {0}")]
    ServiceUnavailable(String),
}
```

### User-Facing Error Messages

- **D-Bus connection lost**: "Unable to connect to system services. Check if asusd is running."
- **Not supported**: "This feature is not available on your device."
- **Permission denied**: "This action requires elevated permissions."

---

## Testing Strategy

### Feature Flag

```toml
[features]
default = []
mock = []
```

### Usage

```bash
# Development without hardware
cargo build --features mock

# Production with real D-Bus
cargo build
```

### Testing Levels

1. **Unit tests**: Test trait implementations with mocks
2. **Integration tests**: Test UI behavior with fake data
3. **Manual testing**: Run on G14 with real D-Bus

---

## UI Design

### Design Principles

- **Style**: Adwaita Light Mode (Clean, high-contrast, professional)
- **Layout**: Dashboard-first with detail windows
- **Colors**: White background, dark text, blue accents
- **Typography**: System fonts

### Tailwind Configuration

```javascript
theme: {
  extend: {
    colors: {
      adwaita: {
        bg: '#ffffff',
        fg: '#1e1e1e',
        accent: '#3584e4',
        success: '#26a269',
        warning: '#e5a50a',
        error: '#c01c28',
      }
    }
  }
}
```

---

## Feature Specifications

### Power & Battery

- **Charge Limit**: Get/set `ChargeControlEndThreshold` via D-Bus
- **Profiles**: Quiet/Balanced/Performance toggle
- **GPU Switching**: Integrated/Hybrid/Dedicated modes via supergfxctl

### Advanced Performance

- **Fan Curves**: Custom curve arrays per profile
- **Overclocking**: GPU Base/Memory clock offsets (Manual mode)

### Lighting

- **Keyboard Aura**: Brightness, modes (Static, Breathing, Rainbow, etc.)
- **Slash Lighting**: 2024/2025 Zephyrus specific
- **AniMe Matrix**: GIF upload, system clocks, battery display

### Hardware Monitoring

- Real-time CPU/GPU temperatures
- Fan speeds (RPM)
- Active power draw
- All non-blocking for UI

---

## Implementation Priority

1. **Phase 1**: Core architecture (traits, mock/real implementations)
2. **Phase 2**: Power profiles & charge limit
3. **Phase 3**: GPU switching
4. **Phase 4**: Hardware monitoring dashboard
5. **Phase 5**: Fan curves & manual mode
6. **Phase 6**: Keyboard lighting & AniMe Matrix

---

## Dependencies

### Backend

- `zbus`: D-Bus communication
- `tokio`: Async runtime
- `serde`/`serde_json`: Serialization
- `thiserror`: Error handling
- `tauri`: Framework

### Frontend

- `@tanstack/react-query`: Server state
- `react`: UI framework
- `tailwindcss`: Styling

---

## Safety Considerations

- All hardware communication via D-Bus (no direct hardware access)
- asusd has built-in safety limits
- Invalid values rejected by daemon
- User confirmation required for destructive operations

---

## Next Steps

1. ✅ Design approved
2. ⏭️ Create detailed implementation plan
3. ⏭️ Set up project structure
4. ⏭️ Implement trait abstractions
5. ⏭️ Build D-Bus interfaces
6. ⏭️ Develop UI components
