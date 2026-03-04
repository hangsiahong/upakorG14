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
