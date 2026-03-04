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
