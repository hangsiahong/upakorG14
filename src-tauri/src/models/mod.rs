pub mod power;
pub mod gpu;
pub mod fan;
pub mod lighting;
pub mod hardware;
pub mod settings;
pub mod capabilities;

pub use power::{PowerProfile, ChargeLimit};
pub use gpu::{GpuMode, GpuStatus};
pub use fan::{FanCurve, FanCurvePoint, FanSpeeds};
pub use lighting::{AuraMode, AuraSettings};
pub use hardware::{Temperature, PowerDraw, HardwareMetrics};
pub use settings::{Settings, ConfigManager, ConfigError};
pub use capabilities::{HardwareCapabilities, DeviceInfo};
