# Upakor-G14 Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Build Upakor-G14, a comprehensive Linux-native alternative to G-Helper for ASUS Zephyrus G14 laptops using Tauri v2 with Rust backend and React frontend.

**Architecture:** Layered trait-based architecture with dual-mode development (real D-Bus via zbus or mock implementations for off-hardware development). All hardware access goes through trait abstractions; Tauri commands provide the public API; React frontend uses TanStack Query for state management.

**Tech Stack:** Tauri v2, Rust (zbus, tokio, serde), React 19, TypeScript, TailwindCSS, TanStack Query

---

## Prerequisites

Before starting, ensure you have:
- Rust toolchain (rustc 1.80+)
- Node.js 20+ and Bun or npm
- Tauri CLI: `cargo install tauri-cli --version "^2.0.0"`
- (Optional) asusd and supergfxctl on target machine for real hardware testing

---

## Task 1: Project Dependencies Setup

**Files:**
- Modify: `src-tauri/Cargo.toml`
- Modify: `package.json`
- Create: `tailwind.config.js`
- Create: `postcss.config.js`

**Step 1: Add Rust dependencies**

Edit `src-tauri/Cargo.toml`, add to `[dependencies]`:

```toml
[dependencies]
tauri = { version = "2", features = ["tray-icon"] }
tauri-plugin-opener = "2"
serde = { version = "1", features = ["derive"] }
serde_json = "1"
tokio = { version = "1", features = ["full"] }
zbus = "4"
thiserror = "1"
tracing = "0.1"
tracing-subscriber = "0.3"

[features]
default = []
mock = []
```

**Step 2: Add frontend dependencies**

Edit `package.json`, add to `dependencies`:

```json
{
  "dependencies": {
    "@tanstack/react-query": "^5.0.0",
    "clsx": "^2.0.0",
    "lucide-react": "^0.400.0"
  },
  "devDependencies": {
    "tailwindcss": "^3.4.0",
    "postcss": "^8.4.0",
    "autoprefixer": "^10.4.0",
    "@types/node": "^20.0.0"
  }
}
```

**Step 3: Create Tailwind config**

Create `tailwind.config.js`:

```javascript
/** @type {import('tailwindcss').Config} */
export default {
  content: [
    "./index.html",
    "./src/**/*.{js,ts,jsx,tsx}",
  ],
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
          muted: '#f0f0f0',
        }
      }
    },
  },
  plugins: [],
}
```

**Step 4: Create PostCSS config**

Create `postcss.config.js`:

```javascript
export default {
  plugins: {
    tailwindcss: {},
    autoprefixer: {},
  },
}
```

**Step 5: Update Vite config**

Edit `vite.config.ts`, add CSS support:

```typescript
import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";

export default defineConfig(async () => ({
  plugins: [react()],
  // Vite options tailored for Tauri development and only applied in `tauri dev` or `tauri build`
  //
  // 1. prevent vite from obscuring rust errors
  clearScreen: false,
  // 2. tauri expects a fixed port, fail if that port is not available
  server: {
    port: 1420,
    strictPort: true,
  },
  // 3. to make use of `TAURI_DEBUG` and other env variables
  // https://tauri.app/v1/api/config#buildconfig.beforedevcommand
  envPrefix: ["VITE_", "TAURI_"],
}));
```

**Step 6: Install dependencies**

Run: `bun install`
Expected: All dependencies installed successfully

**Step 7: Commit**

```bash
git add src-tauri/Cargo.toml package.json tailwind.config.js postcss.config.js vite.config.ts
git commit -m "feat: add project dependencies and tooling config"
```

---

## Task 2: Create Project Directory Structure

**Files:**
- Create: `src-tauri/src/traits/mod.rs`
- Create: `src-tauri/src/traits/asusd.rs`
- Create: `src-tauri/src/traits/supergfxctl.rs`
- Create: `src-tauri/src/implementations/mod.rs`
- Create: `src-tauri/src/implementations/real_dbus/mod.rs`
- Create: `src-tauri/src/implementations/real_dbus/asusd.rs`
- Create: `src-tauri/src/implementations/real_dbus/supergfxctl.rs`
- Create: `src-tauri/src/implementations/mock/mod.rs`
- Create: `src-tauri/src/implementations/mock/asusd.rs`
- Create: `src-tauri/src/implementations/mock/supergfxctl.rs`
- Create: `src-tauri/src/models/mod.rs`
- Create: `src-tauri/src/models/power.rs`
- Create: `src-tauri/src/models/gpu.rs`
- Create: `src-tauri/src/models/fan.rs`
- Create: `src-tauri/src/models/lighting.rs`
- Create: `src-tauri/src/models/hardware.rs`
- Create: `src-tauri/src/utils/mod.rs`
- Create: `src-tauri/src/utils/errors.rs`
- Create: `src-tauri/src/utils/state.rs`

**Step 1: Create directory structure**

Run: `mkdir -p src-tauri/src/{traits,implementations/{real_dbus,mock},models,utils}`

**Step 2: Create empty module files**

Run: `touch src-tauri/src/traits/mod.rs src-tauri/src/traits/asusd.rs src-tauri/src/traits/supergfxctl.rs src-tauri/src/implementations/mod.rs src-tauri/src/implementations/real_dbus/mod.rs src-tauri/src/implementations/real_dbus/asusd.rs src-tauri/src/implementations/real_dbus/supergfxctl.rs src-tauri/src/implementations/mock/mod.rs src-tauri/src/implementations/mock/asusd.rs src-tauri/src/implementations/mock/supergfxctl.rs src-tauri/src/models/mod.rs src-tauri/src/models/power.rs src-tauri/src/models/gpu.rs src-tauri/src/models/fan.rs src-tauri/src/models/lighting.rs src-tauri/src/models/hardware.rs src-tauri/src/utils/mod.rs src-tauri/src/utils/errors.rs src-tauri/src/utils/state.rs`

**Step 3: Commit**

```bash
git add src-tauri/src/traits src-tauri/src/implementations src-tauri/src/models src-tauri/src/utils
git commit -m "feat: create project directory structure"
```

---

## Task 3: Define Error Types

**Files:**
- Modify: `src-tauri/src/utils/errors.rs`
- Modify: `src-tauri/src/utils/mod.rs`

**Step 1: Write error types**

Edit `src-tauri/src/utils/errors.rs`:

```rust
use thiserror::Error;

#[derive(Debug, Error)]
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

    #[error("Unknown error: {0}")]
    Unknown(String),
}

impl From<zbus::Error> for UpakorError {
    fn from(err: zbus::Error) -> Self {
        UpakorError::DbusConnection(err.to_string())
    }
}

impl From<UpakorError> for String {
    fn from(err: UpakorError) -> Self {
        err.to_string()
    }
}

pub type Result<T> = std::result::Result<T, UpakorError>;
```

**Step 2: Export from utils module**

Edit `src-tauri/src/utils/mod.rs`:

```rust
pub mod errors;

pub use errors::{UpakorError, Result};
```

**Step 3: Commit**

```bash
git add src-tauri/src/utils/errors.rs src-tauri/src/utils/mod.rs
git commit -m "feat: define error types and Result alias"
```

---

## Task 4: Define Data Models

**Files:**
- Modify: `src-tauri/src/models/power.rs`
- Modify: `src-tauri/src/models/gpu.rs`
- Modify: `src-tauri/src/models/fan.rs`
- Modify: `src-tauri/src/models/lighting.rs`
- Modify: `src-tauri/src/models/hardware.rs`
- Modify: `src-tauri/src/models/mod.rs`

**Step 1: Write power models**

Edit `src-tauri/src/models/power.rs`:

```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PowerProfile {
    Quiet,
    Balanced,
    Performance,
}

impl Default for PowerProfile {
    fn default() -> Self {
        Self::Balanced
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct ChargeLimit {
    pub limit: u8, // 0-100, 0 means no limit
}

impl Default for ChargeLimit {
    fn default() -> Self {
        Self { limit: 100 }
    }
}
```

**Step 2: Write GPU models**

Edit `src-tauri/src/models/gpu.rs`:

```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum GpuMode {
    Integrated,
    Hybrid,
    Dedicated,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GpuStatus {
    pub current_mode: GpuMode,
    pub dedicated_available: bool,
}
```

**Step 3: Write fan models**

Edit `src-tauri/src/models/fan.rs`:

```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FanCurvePoint {
    pub temperature: u8, // Celsius
    pub speed: u8,       // Percentage 0-100
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FanCurve {
    pub cpu_curve: Vec<FanCurvePoint>,
    pub gpu_curve: Vec<FanCurvePoint>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FanSpeeds {
    pub cpu_rpm: u32,
    pub gpu_rpm: u32,
    pub cpu_percentage: u8,
    pub gpu_percentage: u8,
}
```

**Step 4: Write lighting models**

Edit `src-tauri/src/models/lighting.rs`:

```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AuraMode {
    Static,
    Breathing,
    Strobe,
    Rainbow,
    Star,
    Rain,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuraSettings {
    pub mode: AuraMode,
    pub brightness: u8, // 0-100
}

impl Default for AuraSettings {
    fn default() -> Self {
        Self {
            mode: AuraMode::Static,
            brightness: 100,
        }
    }
}
```

**Step 5: Write hardware monitoring models**

Edit `src-tauri/src/models/hardware.rs`:

```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Temperature {
    pub cpu: f32,
    pub gpu: Option<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PowerDraw {
    pub cpu: f32, // Watts
    pub gpu: Option<f32>,
    pub total: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HardwareMetrics {
    pub temperatures: Temperature,
    pub fan_speeds: super::fan::FanSpeeds,
    pub power_draw: PowerDraw,
}
```

**Step 6: Export all models**

Edit `src-tauri/src/models/mod.rs`:

```rust
pub mod power;
pub mod gpu;
pub mod fan;
pub mod lighting;
pub mod hardware;

pub use power::{PowerProfile, ChargeLimit};
pub use gpu::{GpuMode, GpuStatus};
pub use fan::{FanCurve, FanCurvePoint, FanSpeeds};
pub use lighting::{AuraMode, AuraSettings};
pub use hardware::{Temperature, PowerDraw, HardwareMetrics};
```

**Step 7: Commit**

```bash
git add src-tauri/src/models
git commit -m "feat: define data models for all hardware features"
```

---

## Task 5: Define Asusd Trait

**Files:**
- Modify: `src-tauri/src/traits/asusd.rs`
- Modify: `src-tauri/src/traits/mod.rs`

**Step 1: Write Asusd trait**

Edit `src-tauri/src/traits/asusd.rs`:

```rust
use async_trait::async_trait;

use crate::models::*;
use crate::utils::Result;

#[async_trait]
pub trait AsusdTrait: Send + Sync {
    // Power profiles
    async fn get_profile(&self) -> Result<PowerProfile>;
    async fn set_profile(&self, profile: PowerProfile) -> Result<()>;

    // Charge limit
    async fn get_charge_limit(&self) -> Result<ChargeLimit>;
    async fn set_charge_limit(&self, limit: u8) -> Result<()>;

    // Fan speeds
    async fn get_fan_speeds(&self) -> Result<FanSpeeds>;

    // Fan curves
    async fn get_fan_curve(&self, profile: PowerProfile) -> Result<FanCurve>;
    async fn set_fan_curve(&self, profile: PowerProfile, curve: FanCurve) -> Result<()>;

    // Lighting
    async fn get_aura_settings(&self) -> Result<AuraSettings>;
    async fn set_aura_settings(&self, settings: AuraSettings) -> Result<()>;

    // Hardware monitoring
    async fn get_temperatures(&self) -> Result<Temperature>;
    async fn get_power_draw(&self) -> Result<PowerDraw>;

    // Check feature support
    async fn supports_fan_curves(&self) -> bool;
    async fn supports_ani_me(&self) -> bool;
}
```

**Step 2: Export from traits**

Edit `src-tauri/src/traits/mod.rs`:

```rust
pub mod asusd;
pub mod supergfxctl;

pub use asusd::AsusdTrait;
pub use supergfxctl::SupergfxctlTrait;
```

**Step 3: Add async-trait dependency**

Edit `src-tauri/Cargo.toml`, add to `[dependencies]`:

```toml
async-trait = "0.1"
```

**Step 4: Commit**

```bash
git add src-tauri/src/traits src-tauri/Cargo.toml
git commit -m "feat: define AsusdTrait with all required methods"
```

---

## Task 6: Define Supergfxctl Trait

**Files:**
- Modify: `src-tauri/src/traits/supergfxctl.rs`

**Step 1: Write Supergfxctl trait**

Edit `src-tauri/src/traits/supergfxctl.rs`:

```rust
use async_trait::async_trait;

use crate::models::{GpuMode, GpuStatus};
use crate::utils::Result;

#[async_trait]
pub trait SupergfxctlTrait: Send + Sync {
    // Get current GPU mode and status
    async fn get_status(&self) -> Result<GpuStatus>;

    // Switch GPU mode (may require user confirmation/sudo)
    async fn set_mode(&self, mode: GpuMode) -> Result<()>;

    // Check if a mode is available
    async fn is_mode_available(&self, mode: GpuMode) -> Result<bool>;
}
```

**Step 2: Commit**

```bash
git add src-tauri/src/traits/supergfxctl.rs
git commit -m "feat: define SupergfxctlTrait for GPU switching"
```

---

## Task 7: Implement Mock Asusd

**Files:**
- Modify: `src-tauri/src/implementations/mock/asusd.rs`
- Modify: `src-tauri/src/implementations/mock/mod.rs`

**Step 1: Write mock implementation**

Edit `src-tauri/src/implementations/mock/asusd.rs`:

```rust
use async_trait::async_trait;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::traits::AsusdTrait;
use crate::models::*;
use crate::utils::Result;

#[derive(Clone)]
pub struct MockAsusd {
    state: Arc<RwLock<MockState>>,
}

struct MockState {
    profile: PowerProfile,
    charge_limit: u8,
    aura_settings: AuraSettings,
}

impl Default for MockState {
    fn default() -> Self {
        Self {
            profile: PowerProfile::Balanced,
            charge_limit: 100,
            aura_settings: AuraSettings::default(),
        }
    }
}

impl MockAsusd {
    pub fn new() -> Self {
        Self {
            state: Arc::new(RwLock::new(MockState::default())),
        }
    }
}

#[async_trait]
impl AsusdTrait for MockAsusd {
    async fn get_profile(&self) -> Result<PowerProfile> {
        let state = self.state.read().await;
        Ok(state.profile)
    }

    async fn set_profile(&self, profile: PowerProfile) -> Result<()> {
        let mut state = self.state.write().await;
        state.profile = profile;
        Ok(())
    }

    async fn get_charge_limit(&self) -> Result<ChargeLimit> {
        let state = self.state.read().await;
        Ok(ChargeLimit { limit: state.charge_limit })
    }

    async fn set_charge_limit(&self, limit: u8) -> Result<()> {
        if limit > 100 {
            return Err(crate::utils::UpakorError::InvalidValue(
                "Charge limit must be 0-100".to_string(),
            ));
        }
        let mut state = self.state.write().await;
        state.charge_limit = limit;
        Ok(())
    }

    async fn get_fan_speeds(&self) -> Result<FanSpeeds> {
        Ok(FanSpeeds {
            cpu_rpm: 2500,
            gpu_rpm: 0,
            cpu_percentage: 45,
            gpu_percentage: 0,
        })
    }

    async fn get_fan_curve(&self, _profile: PowerProfile) -> Result<FanCurve> {
        Ok(FanCurve {
            cpu_curve: vec![
                FanCurvePoint { temperature: 50, speed: 30 },
                FanCurvePoint { temperature: 70, speed: 50 },
                FanCurvePoint { temperature: 85, speed: 100 },
            ],
            gpu_curve: vec![
                FanCurvePoint { temperature: 55, speed: 30 },
                FanCurvePoint { temperature: 75, speed: 60 },
                FanCurvePoint { temperature: 90, speed: 100 },
            ],
        })
    }

    async fn set_fan_curve(&self, _profile: PowerProfile, _curve: FanCurve) -> Result<()> {
        Ok(())
    }

    async fn get_aura_settings(&self) -> Result<AuraSettings> {
        let state = self.state.read().await;
        Ok(state.aura_settings.clone())
    }

    async fn set_aura_settings(&self, settings: AuraSettings) -> Result<()> {
        let mut state = self.state.write().await;
        state.aura_settings = settings;
        Ok(())
    }

    async fn get_temperatures(&self) -> Result<Temperature> {
        Ok(Temperature {
            cpu: 55.0,
            gpu: Some(45.0),
        })
    }

    async fn get_power_draw(&self) -> Result<PowerDraw> {
        Ok(PowerDraw {
            cpu: 25.0,
            gpu: Some(15.0),
            total: 40.0,
        })
    }

    async fn supports_fan_curves(&self) -> bool {
        true
    }

    async fn supports_ani_me(&self) -> bool {
        true
    }
}
```

**Step 2: Export from mock module**

Edit `src-tauri/src/implementations/mock/mod.rs`:

```rust
pub mod asusd;
pub mod supergfxctl;

pub use asusd::MockAsusd;
pub use supergfxctl::MockSupergfxctl;
```

**Step 3: Commit**

```bash
git add src-tauri/src/implementations/mock
git commit -m "feat: implement MockAsusd with simulated hardware state"
```

---

## Task 8: Implement Mock Supergfxctl

**Files:**
- Modify: `src-tauri/src/implementations/mock/supergfxctl.rs`

**Step 1: Write mock implementation**

Edit `src-tauri/src/implementations/mock/supergfxctl.rs`:

```rust
use async_trait::async_trait;

use crate::traits::SupergfxctlTrait;
use crate::models::{GpuMode, GpuStatus};
use crate::utils::Result;

#[derive(Clone)]
pub struct MockSupergfxctl {
    current_mode: GpuMode,
}

impl MockSupergfxctl {
    pub fn new() -> Self {
        Self {
            current_mode: GpuMode::Hybrid,
        }
    }
}

impl Default for MockSupergfxctl {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl SupergfxctlTrait for MockSupergfxctl {
    async fn get_status(&self) -> Result<GpuStatus> {
        Ok(GpuStatus {
            current_mode: self.current_mode,
            dedicated_available: true,
        })
    }

    async fn set_mode(&self, mode: GpuMode) -> Result<()> {
        // In real implementation, this would trigger a mode switch
        // For mock, we'll just pretend it succeeds
        Ok(())
    }

    async fn is_mode_available(&self, _mode: GpuMode) -> Result<bool> {
        Ok(true)
    }
}
```

**Step 2: Commit**

```bash
git add src-tauri/src/implementations/mock/supergfxctl.rs
git commit -m "feat: implement MockSupergfxctl for GPU switching"
```

---

## Task 9: Implement Real D-Bus Asusd

**Files:**
- Modify: `src-tauri/src/implementations/real_dbus/asusd.rs`
- Modify: `src-tauri/src/implementations/real_dbus/mod.rs`

**Step 1: Write zbus proxy definitions**

Edit `src-tauri/src/implementations/real_dbus/asusd.rs`:

```rust
use async_trait::async_trait;
use zbus::{Connection, Proxy};

use crate::traits::AsusdTrait;
use crate::models::*;
use crate::utils::{Result, UpakorError};

// zbus proxy for asusd platform interface
#[derive(Proxy)]
#[zbus(path = "/xyz/ljones/Platform")]
#[zbus(interface = "xyz.ljones.Asusd")]
struct AsusdProxy<'a> {
    #[zbus(property)]
    profile: String,
    #[zbus(property)]
    charge_control_end_threshold: u8,
}

pub struct RealAsusd {
    connection: Connection,
}

impl RealAsusd {
    pub async fn new() -> Result<Self> {
        let connection = Connection::system()
            .await
            .map_err(|e| UpakorError::DbusConnection(e.to_string()))?;

        // Test connection
        let _proxy = AsusdProxy::new(&connection)
            .await
            .map_err(|e| UpakorError::ServiceUnavailable(e.to_string()))?;

        Ok(Self { connection })
    }
}

#[async_trait]
impl AsusdTrait for RealAsusd {
    async fn get_profile(&self) -> Result<PowerProfile> {
        let proxy = AsusdProxy::new(&self.connection).await?;
        let profile_str = proxy.profile().await?;

        match profile_str.as_str() {
            "quiet" => Ok(PowerProfile::Quiet),
            "balanced" => Ok(PowerProfile::Balanced),
            "performance" => Ok(PowerProfile::Performance),
            _ => Err(UpakorError::Unknown(format!("Unknown profile: {}", profile_str))),
        }
    }

    async fn set_profile(&self, profile: PowerProfile) -> Result<()> {
        let proxy = AsusdProxy::new(&self.connection).await?;
        let profile_str = match profile {
            PowerProfile::Quiet => "quiet",
            PowerProfile::Balanced => "balanced",
            PowerProfile::Performance => "performance",
        };
        proxy.set_profile(profile_str).await?;
        Ok(())
    }

    async fn get_charge_limit(&self) -> Result<ChargeLimit> {
        let proxy = AsusdProxy::new(&self.connection).await?;
        let limit = proxy.charge_control_end_threshold().await?;
        Ok(ChargeLimit { limit })
    }

    async fn set_charge_limit(&self, limit: u8) -> Result<()> {
        let proxy = AsusdProxy::new(&self.connection).await?;
        proxy.set_charge_control_end_threshold(limit).await?;
        Ok(())
    }

    async fn get_fan_speeds(&self) -> Result<FanSpeeds> {
        // TODO: Implement with actual D-Bus calls
        Ok(FanSpeeds {
            cpu_rpm: 0,
            gpu_rpm: 0,
            cpu_percentage: 0,
            gpu_percentage: 0,
        })
    }

    async fn get_fan_curve(&self, _profile: PowerProfile) -> Result<FanCurve> {
        // TODO: Implement with actual D-Bus calls
        Ok(FanCurve {
            cpu_curve: vec![],
            gpu_curve: vec![],
        })
    }

    async fn set_fan_curve(&self, _profile: PowerProfile, _curve: FanCurve) -> Result<()> {
        // TODO: Implement with actual D-Bus calls
        Ok(())
    }

    async fn get_aura_settings(&self) -> Result<AuraSettings> {
        // TODO: Implement with actual D-Bus calls
        Ok(AuraSettings::default())
    }

    async fn set_aura_settings(&self, _settings: AuraSettings) -> Result<()> {
        // TODO: Implement with actual D-Bus calls
        Ok(())
    }

    async fn get_temperatures(&self) -> Result<Temperature> {
        // TODO: Implement with actual D-Bus calls
        Ok(Temperature {
            cpu: 0.0,
            gpu: None,
        })
    }

    async fn get_power_draw(&self) -> Result<PowerDraw> {
        // TODO: Implement with actual D-Bus calls
        Ok(PowerDraw {
            cpu: 0.0,
            gpu: None,
            total: 0.0,
        })
    }

    async fn supports_fan_curves(&self) -> bool {
        // TODO: Query device capabilities
        true
    }

    async fn supports_ani_me(&self) -> bool {
        // TODO: Query device capabilities
        false
    }
}
```

**Step 2: Export from real_dbus module**

Edit `src-tauri/src/implementations/real_dbus/mod.rs`:

```rust
pub mod asusd;
pub mod supergfxctl;

pub use asusd::RealAsusd;
pub use supergfxctl::RealSupergfxctl;
```

**Step 3: Commit**

```bash
git add src-tauri/src/implementations/real_dbus
git commit -m "feat: implement RealAsusd with zbus D-Bus proxy"
```

---

## Task 10: Implement Real D-Bus Supergfxctl

**Files:**
- Modify: `src-tauri/src/implementations/real_dbus/supergfxctl.rs`

**Step 1: Write zbus proxy implementation**

Edit `src-tauri/src/implementations/real_dbus/supergfxctl.rs`:

```rust
use async_trait::async_trait;
use zbus::{Connection, Proxy};

use crate::traits::SupergfxctlTrait;
use crate::models::{GpuMode, GpuStatus};
use crate::utils::{Result, UpakorError};

#[derive(Proxy)]
#[zbus(path = "/org/asuslinux/Supergfxd")]
#[zbus(interface = "org.asuslinux.Daemon")]
struct SupergfxdProxy<'a> {
    // Methods and properties will be added as needed
}

pub struct RealSupergfxctl {
    connection: Connection,
}

impl RealSupergfxctl {
    pub async fn new() -> Result<Self> {
        let connection = Connection::system()
            .await
            .map_err(|e| UpakorError::DbusConnection(e.to_string()))?;

        // Test connection
        let _proxy = SupergfxdProxy::new(&connection)
            .await
            .map_err(|e| UpakorError::ServiceUnavailable(e.to_string()))?;

        Ok(Self { connection })
    }
}

#[async_trait]
impl SupergfxctlTrait for RealSupergfxctl {
    async fn get_status(&self) -> Result<GpuStatus> {
        // TODO: Implement with actual D-Bus calls
        Ok(GpuStatus {
            current_mode: GpuMode::Hybrid,
            dedicated_available: true,
        })
    }

    async fn set_mode(&self, mode: GpuMode) -> Result<()> {
        // TODO: Implement with actual D-Bus calls
        Ok(())
    }

    async fn is_mode_available(&self, mode: GpuMode) -> Result<bool> {
        // TODO: Implement with actual D-Bus calls
        Ok(true)
    }
}
```

**Step 2: Commit**

```bash
git add src-tauri/src/implementations/real_dbus/supergfxctl.rs
git commit -m "feat: implement RealSupergfxctl with zbus D-Bus proxy"
```

---

## Task 11: Implement Factory Pattern

**Files:**
- Modify: `src-tauri/src/implementations/mod.rs`
- Modify: `src-tauri/src/lib.rs`

**Step 1: Create implementation factory**

Edit `src-tauri/src/implementations/mod.rs`:

```rust
use crate::traits::{AsusdTrait, SupergfxctlTrait};

#[cfg(feature = "mock")]
pub mod mock;

#[cfg(not(feature = "mock"))]
pub mod real_dbus;

pub async fn create_asusd() -> Result<Box<dyn AsusdTrait>> {
    #[cfg(feature = "mock")]
    {
        Ok(Box::new(mock::MockAsusd::new()))
    }

    #[cfg(not(feature = "mock"))]
    {
        let instance = real_dbus::RealAsusd::new().await?;
        Ok(Box::new(instance))
    }
}

pub async fn create_supergfxctl() -> Result<Box<dyn SupergfxctlTrait>> {
    #[cfg(feature = "mock")]
    {
        Ok(Box::new(mock::MockSupergfxctl::new()))
    }

    #[cfg(not(feature = "mock"))]
    {
        let instance = real_dbus::RealSupergfxctl::new().await?;
        Ok(Box::new(instance))
    }
}
```

**Step 2: Update lib.rs with module declarations**

Edit `src-tauri/src/lib.rs`:

```rust
mod traits;
mod implementations;
mod models;
mod utils;

use implementations::{create_asusd, create_supergfxctl};
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct AppState {
    pub asusd: Arc<Mutex<Box<dyn traits::AsusdTrait>>>,
    pub supergfxctl: Arc<Mutex<Box<dyn traits::SupergfxctlTrait>>>,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Tauri setup will be added in next tasks
}
```

**Step 3: Commit**

```bash
git add src-tauri/src/implementations/mod.rs src-tauri/src/lib.rs
git commit -m "feat: add factory pattern for implementation selection"
```

---

## Task 12: Implement Tauri Commands - Power Profiles

**Files:**
- Modify: `src-tauri/src/lib.rs`

**Step 1: Add Tauri commands for power profiles**

Edit `src-tauri/src/lib.rs`, add:

```rust
use tauri::State;

#[tauri::command]
async fn get_power_profile(state: State<'_, AppState>) -> Result<PowerProfile, String> {
    let asusd = state.asusd.lock().await;
    asusd.get_profile().await.map_err(|e| e.to_string())
}

#[tauri::command]
async fn set_power_profile(state: State<'_, AppState>, profile: String) -> Result<(), String> {
    let asusd = state.asusd.lock().await;
    let profile = match profile.as_str() {
        "quiet" => PowerProfile::Quiet,
        "balanced" => PowerProfile::Balanced,
        "performance" => PowerProfile::Performance,
        _ => return Err("Invalid profile name".to_string()),
    };
    asusd.set_profile(profile).await.map_err(|e| e.to_string())
}

#[tauri::command]
async fn get_charge_limit(state: State<'_, AppState>) -> Result<ChargeLimit, String> {
    let asusd = state.asusd.lock().await;
    asusd.get_charge_limit().await.map_err(|e| e.to_string())
}

#[tauri::command]
async fn set_charge_limit(state: State<'_, AppState>, limit: u8) -> Result<(), String> {
    let asusd = state.asusd.lock().await;
    asusd.set_charge_limit(limit).await.map_err(|e| e.to_string())
}
```

**Step 2: Register commands in main.rs**

Edit `src-tauri/src/main.rs`:

```rust
// Prevents additional console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    tauri::Builder::default()
        .manage(upakorg14_lib::AppState {
            asusd: std::sync::Arc::new(tokio::sync::Mutex::new(
                Box::new(upakorg14_lib::implementations::mock::MockAsusd::new()) as Box<dyn upakorg14_lib::traits::AsusdTrait>
            )),
            supergfxctl: std::sync::Arc::new(tokio::sync::Mutex::new(
                Box::new(upakorg14_lib::implementations::mock::MockSupergfxctl::new()) as Box<dyn upakorg14_lib::traits::SupergfxctlTrait>
            )),
        })
        .invoke_handler(tauri::generate_handler![
            upakorg14_lib::get_power_profile,
            upakorg14_lib::set_power_profile,
            upakorg14_lib::get_charge_limit,
            upakorg14_lib::set_charge_limit,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

**Step 3: Commit**

```bash
git add src-tauri/src/lib.rs src-tauri/src/main.rs
git commit -m "feat: add Tauri commands for power profiles"
```

---

## Task 13: Implement Tauri Commands - Hardware Monitoring

**Files:**
- Modify: `src-tauri/src/lib.rs`

**Step 1: Add hardware monitoring commands**

Edit `src-tauri/src/lib.rs`, add:

```rust
#[tauri::command]
async fn get_hardware_metrics(state: State<'_, AppState>) -> Result<HardwareMetrics, String> {
    let asusd = state.asusd.lock().await;

    let temps = asusd.get_temperatures().await.map_err(|e| e.to_string())?;
    let fans = asusd.get_fan_speeds().await.map_err(|e| e.to_string())?;
    let power = asusd.get_power_draw().await.map_err(|e| e.to_string())?;

    Ok(HardwareMetrics {
        temperatures: temps,
        fan_speeds: fans,
        power_draw: power,
    })
}
```

**Step 2: Register in main.rs**

Edit `src-tauri/src/main.rs`, add to invoke_handler:

```rust
        .invoke_handler(tauri::generate_handler![
            upakorg14_lib::get_power_profile,
            upakorg14_lib::set_power_profile,
            upakorg14_lib::get_charge_limit,
            upakorg14_lib::set_charge_limit,
            upakorg14_lib::get_hardware_metrics,
        ])
```

**Step 3: Commit**

```bash
git add src-tauri/src/lib.rs src-tauri/src/main.rs
git commit -m "feat: add Tauri command for hardware monitoring"
```

---

## Task 14: Setup Frontend Base

**Files:**
- Modify: `src/main.tsx`
- Modify: `src/App.tsx`
- Create: `src/index.css`
- Create: `src/lib/tauri.ts`
- Create: `src/types/tauri.ts`

**Step 1: Add global CSS**

Create `src/index.css`:

```css
@tailwind base;
@tailwind components;
@tailwind utilities;

@layer base {
  body {
    @apply bg-adwaita-bg text-adwaita-fg;
  }
}

@layer components {
  .card {
    @apply bg-white rounded-lg shadow-md border border-gray-200;
  }

  .btn-primary {
    @apply bg-adwaita-accent text-white px-4 py-2 rounded-md hover:opacity-90 transition-opacity;
  }

  .btn-secondary {
    @apply bg-adwaita-muted text-adwaita-fg px-4 py-2 rounded-md hover:bg-gray-200 transition-colors;
  }
}
```

**Step 2: Create Tauri API wrappers**

Create `src/lib/tauri.ts`:

```typescript
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';

export const tauri = {
  // Power profiles
  getPowerProfile: () => invoke<string>('get_power_profile'),
  setPowerProfile: (profile: string) => invoke<void>('set_power_profile', { profile }),

  // Charge limit
  getChargeLimit: () => invoke<number>('get_charge_limit'),
  setChargeLimit: (limit: number) => invoke<void>('set_charge_limit', { limit }),

  // Hardware monitoring
  getHardwareMetrics: () => invoke<any>('get_hardware_metrics'),
};
```

**Step 3: Update main.tsx**

Edit `src/main.tsx`:

```typescript
import React from 'react';
import ReactDOM from 'react-dom/client';
import App from './App';
import './index.css';

ReactDOM.createRoot(document.getElementById('root') as HTMLElement).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>,
);
```

**Step 4: Update App.tsx**

Edit `src/App.tsx`:

```typescript
import { QueryClient, QueryClientProvider } from '@tanstack/react-query';

const queryClient = new QueryClient({
  defaultOptions: {
    queries: {
      refetchInterval: 1000, // Refresh every second
    },
  },
});

function App() {
  return (
    <QueryClientProvider client={queryClient}>
      <div className="min-h-screen bg-adwaita-bg">
        <div className="container mx-auto p-8">
          <h1 className="text-3xl font-bold mb-8">Upakor-G14</h1>
          <p className="text-adwaita-fg">ASUS Zephyrus G14 Control</p>
        </div>
      </div>
    </QueryClientProvider>
  );
}

export default App;
```

**Step 5: Commit**

```bash
git add src/main.tsx src/App.tsx src/index.css src/lib src/types
git commit -m "feat: setup frontend base with Tailwind and React Query"
```

---

## Task 15: Create Dashboard Component

**Files:**
- Create: `src/components/Dashboard/Dashboard.tsx`
- Create: `src/components/Dashboard/MetricCard.tsx`
- Create: `src/components/Dashboard/QuickActions.tsx`
- Create: `src/hooks/useHardwareState.ts`

**Step 1: Create hardware state hook**

Create `src/hooks/useHardwareState.ts`:

```typescript
import { useQuery } from '@tanstack/react-query';
import { tauri } from '../lib/tauri';

export function useHardwareState() {
  const metrics = useQuery({
    queryKey: ['hardware-metrics'],
    queryFn: tauri.getHardwareMetrics,
  });

  const profile = useQuery({
    queryKey: ['power-profile'],
    queryFn: tauri.getPowerProfile,
  });

  const chargeLimit = useQuery({
    queryKey: ['charge-limit'],
    queryFn: tauri.getChargeLimit,
  });

  return {
    metrics: metrics.data,
    profile: profile.data,
    chargeLimit: chargeLimit.data,
    isLoading: metrics.isLoading || profile.isLoading || chargeLimit.isLoading,
    error: metrics.error || profile.error || chargeLimit.error,
  };
}
```

**Step 2: Create metric card component**

Create `src/components/Dashboard/MetricCard.tsx`:

```typescript
interface MetricCardProps {
  title: string;
  value: string | number;
  unit?: string;
  icon?: React.ReactNode;
}

export function MetricCard({ title, value, unit, icon }: MetricCardProps) {
  return (
    <div className="card p-4">
      <div className="flex items-center justify-between">
        <div>
          <p className="text-sm text-gray-500">{title}</p>
          <p className="text-2xl font-bold">
            {value}{unit && <span className="text-lg text-gray-500 ml-1">{unit}</span>}
          </p>
        </div>
        {icon && <div className="text-adwaita-accent">{icon}</div>}
      </div>
    </div>
  );
}
```

**Step 3: Create quick actions component**

Create `src/components/Dashboard/QuickActions.tsx`:

```typescript
import { Cpu, Zap } from 'lucide-react';
import { useHardwareState } from '../../hooks/useHardwareState';

export function QuickActions() {
  const { profile, chargeLimit } = useHardwareState();

  return (
    <div className="card p-6">
      <h2 className="text-xl font-semibold mb-4">Quick Actions</h2>

      <div className="space-y-4">
        <div>
          <label className="flex items-center gap-2 text-sm font-medium mb-2">
            <Cpu className="w-4 h-4" />
            Power Profile
          </label>
          <select
            className="w-full p-2 border rounded-md bg-white"
            value={profile || 'balanced'}
          >
            <option value="quiet">Quiet</option>
            <option value="balanced">Balanced</option>
            <option value="performance">Performance</option>
          </select>
        </div>

        <div>
          <label className="flex items-center gap-2 text-sm font-medium mb-2">
            <Zap className="w-4 h-4" />
            Charge Limit: {chargeLimit}%
          </label>
          <input
            type="range"
            min="50"
            max="100"
            step="10"
            value={chargeLimit || 100}
            className="w-full"
          />
        </div>
      </div>
    </div>
  );
}
```

**Step 4: Create dashboard component**

Create `src/components/Dashboard/Dashboard.tsx`:

```typescript
import { MetricCard } from './MetricCard';
import { QuickActions } from './QuickActions';
import { useHardwareState } from '../../hooks/useHardwareState';
import { Thermometer, HardDrive, Gauge } from 'lucide-react';

export function Dashboard() {
  const { metrics, isLoading, error } = useHardwareState();

  if (isLoading) {
    return <div className="p-8">Loading...</div>;
  }

  if (error) {
    return <div className="p-8 text-adwaita-error">Error: {error.message}</div>;
  }

  return (
    <div className="space-y-6">
      <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
        <MetricCard
          title="CPU Temperature"
          value={metrics?.temperatures.cpu.toFixed(1) || '0'}
          unit="°C"
          icon={<Thermometer className="w-6 h-6" />}
        />
        <MetricCard
          title="Total Power"
          value={metrics?.power_draw.total.toFixed(1) || '0'}
          unit="W"
          icon={<Gauge className="w-6 h-6" />}
        />
        <MetricCard
          title="CPU Fan"
          value={metrics?.fan_speeds.cpu_percentage || '0'}
          unit="%"
          icon={<HardDrive className="w-6 h-6" />}
        />
      </div>

      <QuickActions />
    </div>
  );
}
```

**Step 5: Update App.tsx to use Dashboard**

Edit `src/App.tsx`:

```typescript
import { QueryClient, QueryClientProvider } from '@tanstack/react-query';
import { Dashboard } from './components/Dashboard/Dashboard';

const queryClient = new QueryClient({
  defaultOptions: {
    queries: {
      refetchInterval: 1000,
    },
  },
});

function App() {
  return (
    <QueryClientProvider client={queryClient}>
      <div className="min-h-screen bg-adwaita-bg">
        <div className="container mx-auto p-8">
          <h1 className="text-3xl font-bold mb-8">Upakor-G14</h1>
          <Dashboard />
        </div>
      </div>
    </QueryClientProvider>
  );
}

export default App;
```

**Step 6: Commit**

```bash
git add src/components/Dashboard src/hooks
git commit -m "feat: create dashboard component with metric cards"
```

---

## Task 16: Add Interactivity - Profile & Charge Limit

**Files:**
- Create: `src/hooks/usePowerProfiles.ts`
- Modify: `src/components/Dashboard/QuickActions.tsx`

**Step 1: Create power profiles hook**

Create `src/hooks/usePowerProfiles.ts`:

```typescript
import { useMutation, useQueryClient } from '@tanstack/react-query';
import { tauri } from '../lib/tauri';

export function usePowerProfiles() {
  const queryClient = useQueryClient();

  const setProfile = useMutation({
    mutationFn: tauri.setPowerProfile,
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['power-profile'] });
    },
  });

  const setChargeLimit = useMutation({
    mutationFn: tauri.setChargeLimit,
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['charge-limit'] });
    },
  });

  return { setProfile, setChargeLimit };
}
```

**Step 2: Update QuickActions with interactivity**

Edit `src/components/Dashboard/QuickActions.tsx`:

```typescript
import { Cpu, Zap } from 'lucide-react';
import { useHardwareState } from '../../hooks/useHardwareState';
import { usePowerProfiles } from '../../hooks/usePowerProfiles';

export function QuickActions() {
  const { profile, chargeLimit } = useHardwareState();
  const { setProfile, setChargeLimit } = usePowerProfiles();

  return (
    <div className="card p-6">
      <h2 className="text-xl font-semibold mb-4">Quick Actions</h2>

      <div className="space-y-4">
        <div>
          <label className="flex items-center gap-2 text-sm font-medium mb-2">
            <Cpu className="w-4 h-4" />
            Power Profile
          </label>
          <select
            className="w-full p-2 border rounded-md bg-white"
            value={profile || 'balanced'}
            onChange={(e) => setProfile.mutate(e.target.value)}
          >
            <option value="quiet">Quiet</option>
            <option value="balanced">Balanced</option>
            <option value="performance">Performance</option>
          </select>
        </div>

        <div>
          <label className="flex items-center gap-2 text-sm font-medium mb-2">
            <Zap className="w-4 h-4" />
            Charge Limit: {chargeLimit}%
          </label>
          <input
            type="range"
            min="50"
            max="100"
            step="10"
            value={chargeLimit || 100}
            onChange={(e) => setChargeLimit.mutate(Number(e.target.value))}
            className="w-full"
          />
        </div>
      </div>
    </div>
  );
}
```

**Step 3: Commit**

```bash
git add src/hooks/usePowerProfiles.ts src/components/Dashboard/QuickActions.tsx
git commit -m "feat: add profile and charge limit interactivity"
```

---

## Task 17: Test with Mock Implementation

**Files:**
- No file changes

**Step 1: Build with mock feature**

Run: `cargo build --manifest-path=src-tauri/Cargo.toml --features mock`

Expected: Successful build with mock implementation

**Step 2: Run dev server**

Run: `bun run tauri dev`

Expected: Application launches, displays dashboard with mock data

**Step 3: Verify features**

Check:
- Dashboard shows temperature (~55°C), power (~40W), fan percentage
- Power profile selector changes between Quiet/Balanced/Performance
- Charge limit slider changes value
- Data refreshes every second

**Step 4: Commit fixes if needed**

If any issues found, fix and commit:

```bash
git add .
git commit -m "fix: resolve mock implementation issues"
```

---

## Task 18: Prepare Production Build Configuration

**Files:**
- Modify: `src-tauri/tauri.conf.json`
- Modify: `src-tauri/Cargo.toml`

**Step 1: Update Tauri config**

Edit `src-tauri/tauri.conf.json`, configure window and app info:

```json
{
  "$schema": "https://schema.tauri.app/config/2.0.0",
  "productName": "Upakor-G14",
  "version": "0.1.0",
  "identifier": "com.upakor.g14",
  "build": {
    "beforeDevCommand": "bun run dev",
    "devUrl": "http://localhost:1420",
    "beforeBuildCommand": "bun run build",
    "frontendDist": "../dist"
  },
  "app": {
    "windows": [
      {
        "title": "Upakor-G14",
        "width": 800,
        "height": 600,
        "resizable": true,
        "hiddenTitle": true
      }
    ],
    "security": {
      "csp": null
    }
  }
}
```

**Step 2: Update main.rs for production**

Edit `src-tauri/src/main.rs`, use conditional compilation:

```rust
// Prevents additional console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    // Initialize logging
    tracing_subscriber::fmt::init();

    // Create runtime
    let rt = tokio::runtime::Runtime::new().expect("Failed to create runtime");

    // Create implementations
    #[cfg(feature = "mock")]
    let (asusd, supergfxctl) = {
        use upakorg14_lib::implementations::mock::{MockAsusd, MockSupergfxctl};
        use upakorg14_lib::traits::{AsusdTrait, SupergfxctlTrait};
        (
            Box::new(MockAsusd::new()) as Box<dyn AsusdTrait>,
            Box::new(MockSupergfxctl::new()) as Box<dyn SupergfxctlTrait>,
        )
    };

    #[cfg(not(feature = "mock"))]
    let (asusd, supergfxctl) = rt.block_on(async {
        use upakorg14_lib::implementations::real_dbus::{RealAsusd, RealSupergfxctl};
        use upakorg14_lib::traits::{AsusdTrait, SupergfxctlTrait};
        let asusd = RealAsusd::new().await.expect("Failed to connect to asusd");
        let supergfxctl = RealSupergfxctl::new().await.expect("Failed to connect to supergfxd");
        (
            Box::new(asusd) as Box<dyn AsusdTrait>,
            Box::new(supergfxctl) as Box<dyn SupergfxctlTrait>,
        )
    });

    tauri::Builder::default()
        .manage(upakorg14_lib::AppState {
            asusd: std::sync::Arc::new(tokio::sync::Mutex::new(asusd)),
            supergfxctl: std::sync::Arc::new(tokio::sync::Mutex::new(supergfxctl)),
        })
        .invoke_handler(tauri::generate_handler![
            upakorg14_lib::get_power_profile,
            upakorg14_lib::set_power_profile,
            upakorg14_lib::get_charge_limit,
            upakorg14_lib::set_charge_limit,
            upakorg14_lib::get_hardware_metrics,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

**Step 3: Commit**

```bash
git add src-tauri/tauri.conf.json src-tauri/src/main.rs src-tauri/Cargo.toml
git commit -m "feat: configure production build settings"
```

---

## Task 19: Build Production Binary

**Files:**
- No file changes

**Step 1: Build production app**

Run: `bun run tauri build`

Expected: Creates production binary in `src-tauri/target/release/bundle/`

**Step 2: Test production binary**

Run the built binary and verify:
- Application launches
- Dashboard displays correctly
- Interactive controls work

**Step 3: Commit any build fixes**

If issues found:

```bash
git add .
git commit -m "fix: resolve production build issues"
```

---

## Task 20: Documentation and README

**Files:**
- Modify: `README.md`
- Create: `docs/D-BUS-API.md`

**Step 1: Update README**

Edit `README.md`:

```markdown
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
```

**Step 2: Create D-Bus API documentation**

Create `docs/D-BUS-API.md`:

```markdown
# D-Bus API Reference

## Asusd (xyz.ljones.Asusd)

### Path: `/xyz/ljones/Platform`

#### Properties

- `profile` (string): Current power profile (quiet/balanced/performance)
- `charge_control_end_threshold` (uint8): Battery charge limit (0-100)

### Methods

TODO: Add complete method documentation

## Supergfxd (org.asuslinux.Supergfxd)

### Path: `/org/asuslinux/Supergfxd`

TODO: Add complete API documentation
```

**Step 3: Commit documentation**

```bash
git add README.md docs/D-BUS-API.md
git commit -m "docs: add README and D-Bus API documentation"
```

---

## Final Steps

### Testing Checklist

- [ ] Mock mode builds and runs
- [ ] Dashboard displays mock data correctly
- [ ] Power profile switching works
- [ ] Charge limit adjustment works
- [ ] Production build succeeds
- [ ] Documentation is complete

### Future Enhancements

Beyond this initial implementation:

1. **Complete D-Bus integration**: Fill in TODO items for full asusd/supergfxctl API
2. **Fan curve editor**: UI for custom fan curves
3. **GPU switching**: Full integration with supergfxctl
4. **Lighting control**: Keyboard RGB, Slash lighting, AniMe Matrix
5. **System tray**: Background running with tray icon
6. **Settings persistence**: Remember user preferences
7. **Notifications**: Alert on important events

### Git Tag

```bash
git tag -a v0.1.0 -m "Initial release: Dashboard with power profiles and hardware monitoring"
git push origin v0.1.0
```

---

**Plan complete!** This implements a solid foundation with:
- ✅ Trait-based architecture with dual-mode development
- ✅ Power profile switching
- ✅ Charge limit control
- ✅ Hardware monitoring dashboard
- ✅ Adwaita-inspired UI
- ✅ Mock implementation for development
- ✅ Real D-Bus structure for production

Ready to execute!
