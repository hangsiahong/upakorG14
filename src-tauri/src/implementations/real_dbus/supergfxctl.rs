use async_trait::async_trait;
use zbus::Connection;

use crate::traits::SupergfxctlTrait;
use crate::models::{GpuMode, GpuStatus};
use crate::utils::{Result, UpakorError};

pub struct RealSupergfxctl {
    connection: Connection,
}

impl RealSupergfxctl {
    pub async fn new() -> Result<Self> {
        let connection = Connection::system()
            .await
            .map_err(|e| UpakorError::DbusConnection(e.to_string()))?;

        // TODO: Test actual supergfxd connection
        Ok(Self { connection })
    }
}

#[async_trait]
impl SupergfxctlTrait for RealSupergfxctl {
    async fn get_status(&self) -> Result<GpuStatus> {
        // TODO: Implement with actual D-Bus calls to org.asuslinux.Supergfxd
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
