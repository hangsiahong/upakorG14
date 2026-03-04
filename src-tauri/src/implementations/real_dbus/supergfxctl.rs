use async_trait::async_trait;
use zbus::Connection;

use crate::traits::SupergfxctlTrait;
use crate::models::{GpuMode, GpuStatus};
use crate::utils::{Result, UpakorError};

/// D-Bus interface for supergfxd (GPU switching daemon)
///
/// The supergfxd service provides GPU mode switching for ASUS laptops with
/// hybrid graphics (integrated + dedicated GPU).
pub struct RealSupergfxctl {
    connection: Connection,
    destination: Option<&'static str>,
    path: &'static str,
}

impl RealSupergfxctl {
    pub async fn new() -> Result<Self> {
        let connection = Connection::system()
            .await
            .map_err(|e| UpakorError::DbusConnection(e.to_string()))?;

        // Try to detect the actual supergfxd service
        let destinations = [
            "org.asuslinux.Supergfxd",
        ];

        let mut working_destination = None;
        for dest in destinations {
            // Try to ping the service
            let result = connection.call_method(
                Some(dest),
                "/org/asuslinux/Supergfxd",
                Some("org.freedesktop.DBus.Introspectable"),
                "Introspect",
                &()
            )
                .await;

            if result.is_ok() {
                working_destination = Some(dest);
                tracing::info!("Connected to supergfxd D-Bus service: {}", dest);
                break;
            }
        }

        if working_destination.is_none() {
            tracing::warn!("supergfxd daemon not found on D-Bus, service will use fallback implementations");
        }

        Ok(Self {
            connection,
            destination: working_destination,
            path: "/org/asuslinux/Supergfxd",
        })
    }

    /// Check if D-Bus service is available
    fn is_service_available(&self) -> bool {
        self.destination.is_some()
    }
}

#[async_trait]
impl SupergfxctlTrait for RealSupergfxctl {
    async fn get_status(&self) -> Result<GpuStatus> {
        if !self.is_service_available() {
            tracing::debug!("D-Bus service unavailable, returning default GPU status");
            return Ok(GpuStatus {
                current_mode: GpuMode::Hybrid,
                dedicated_available: true,
            });
        }

        // TODO: Implement actual D-Bus call when service API is confirmed
        // For now, return default
        Ok(GpuStatus {
            current_mode: GpuMode::Hybrid,
            dedicated_available: true,
        })
    }

    async fn set_mode(&self, mode: GpuMode) -> Result<()> {
        if !self.is_service_available() {
            return Err(UpakorError::ServiceUnavailable(
                "supergfxd D-Bus service not available".to_string()
            ));
        }

        let mode_str = match mode {
            GpuMode::Integrated => "integrated",
            GpuMode::Hybrid => "hybrid",
            GpuMode::Dedicated => "dedicated",
        };

        tracing::info!("Switching GPU mode to: {}", mode_str);
        // TODO: Implement actual D-Bus call
        // GPU mode switching may require authentication and can take time

        Ok(())
    }

    async fn is_mode_available(&self, mode: GpuMode) -> Result<bool> {
        let status = self.get_status().await?;

        match mode {
            GpuMode::Integrated => Ok(true), // Integrated is always available
            GpuMode::Hybrid => Ok(true),     // Hybrid is usually available
            GpuMode::Dedicated => Ok(status.dedicated_available),
        }
    }
}
