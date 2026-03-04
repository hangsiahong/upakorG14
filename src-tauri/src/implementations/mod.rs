use crate::traits::{AsusdTrait, SupergfxctlTrait};
use crate::utils::Result;

// Make both modules available - the feature flag controls which one gets used
pub mod mock;
pub mod real_dbus;
pub mod sysfs_fallback;

#[cfg(feature = "mock")]
pub use mock::{MockAsusd, MockSupergfxctl};

#[cfg(not(feature = "mock"))]
pub use real_dbus::{RealAsusd, RealSupergfxctl};

pub use sysfs_fallback::{SysMonitorTrait, RealSysMonitor, MockSysMonitor};

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
