use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;
use tokio::time::interval;
use tracing::{info, warn, debug};
use tauri::{AppHandle, Emitter};

use crate::models::{HardwareMetrics, Temperature, FanSpeeds, PowerDraw};
use crate::traits::AsusdTrait;
use crate::implementations::sysfs_fallback::SysMonitorTrait;

/// Metrics poller for real-time hardware monitoring
pub struct MetricsPoller {
    asusd: Arc<Mutex<Box<dyn AsusdTrait>>>,
    sys_monitor: Option<Arc<Mutex<Box<dyn SysMonitorTrait>>>>,
    app_handle: Option<AppHandle>,
    poll_handle: Arc<Mutex<Option<tokio::task::JoinHandle<()>>>>,
    is_polling: Arc<Mutex<bool>>,
}

impl MetricsPoller {
    /// Create a new metrics poller
    pub fn new(
        asusd: Arc<Mutex<Box<dyn AsusdTrait>>>,
        sys_monitor: Option<Arc<Mutex<Box<dyn SysMonitorTrait>>>>,
    ) -> Self {
        Self {
            asusd,
            sys_monitor,
            app_handle: None,
            poll_handle: Arc::new(Mutex::new(None)),
            is_polling: Arc::new(Mutex::new(false)),
        }
    }

    /// Set the Tauri app handle for event emission
    pub fn with_app_handle(mut self, app_handle: AppHandle) -> Self {
        self.app_handle = Some(app_handle);
        self
    }

    /// Start polling for metrics
    pub async fn start_polling(&self, interval_ms: u64) -> Result<(), Box<dyn std::error::Error>> {
        let mut is_polling = self.is_polling.lock().await;
        if *is_polling {
            debug!("Metrics polling already active");
            return Ok(());
        }
        *is_polling = true;
        drop(is_polling);

        info!("Starting metrics polling with interval: {}ms", interval_ms);

        let asusd = self.asusd.clone();
        let sys_monitor = self.sys_monitor.clone();
        let is_polling = self.is_polling.clone();
        let app_handle = self.app_handle.clone();

        let handle = tokio::spawn(async move {
            let mut timer = interval(Duration::from_millis(interval_ms));
            timer.tick().await; // Skip first immediate tick

            loop {
                timer.tick().await;

                // Check if we should stop
                {
                    let polling = is_polling.lock().await;
                    if !*polling {
                        debug!("Stopping metrics polling");
                        break;
                    }
                }

                // Poll metrics
                match Self::poll_metrics(&asusd, &sys_monitor).await {
                    Ok(metrics) => {
                        debug!(
                            cpu_temp = metrics.temperatures.cpu,
                            cpu_fan = metrics.fan_speeds.cpu_rpm,
                            "Polled hardware metrics"
                        );

                        // Emit Tauri event to frontend if app_handle is available
                        if let Some(ref handle) = app_handle {
                            let _ = handle.emit("hardware_metrics_update", metrics);
                        }
                    }
                    Err(e) => {
                        warn!("Failed to poll metrics: {}", e);

                        // Emit error event if app_handle is available
                        if let Some(ref handle) = app_handle {
                            let _ = handle.emit("metrics_polling_error", format!("{}", e));
                        }
                    }
                }
            }
        });

        let mut poll_handle = self.poll_handle.lock().await;
        *poll_handle = Some(handle);

        Ok(())
    }

    /// Stop polling for metrics
    pub async fn stop_polling(&self) {
        info!("Stopping metrics polling");

        {
            let mut is_polling = self.is_polling.lock().await;
            *is_polling = false;
        }

        let mut poll_handle = self.poll_handle.lock().await;
        if let Some(handle) = poll_handle.take() {
            handle.abort();
        }
    }

    /// Check if currently polling
    pub async fn is_polling(&self) -> bool {
        *self.is_polling.lock().await
    }

    /// Poll metrics once (without starting background polling)
    pub async fn poll_once(&self) -> Result<HardwareMetrics, Box<dyn std::error::Error>> {
        Self::poll_metrics(&self.asusd, &self.sys_monitor).await
    }

    /// Internal method to poll all metrics
    async fn poll_metrics(
        asusd: &Arc<Mutex<Box<dyn AsusdTrait>>>,
        _sys_monitor: &Option<Arc<Mutex<Box<dyn SysMonitorTrait>>>>,
    ) -> Result<HardwareMetrics, Box<dyn std::error::Error>> {
        let asusd_lock = asusd.lock().await;

        // Get all metrics from asusd (or defaults on failure)
        let temps = asusd_lock.get_temperatures().await
            .unwrap_or(Temperature { cpu: 0.0, gpu: None });

        let fans = asusd_lock.get_fan_speeds().await
            .unwrap_or(FanSpeeds {
                cpu_rpm: 0,
                gpu_rpm: 0,
                cpu_percentage: 0,
                gpu_percentage: 0,
            });

        let power = asusd_lock.get_power_draw().await
            .unwrap_or(PowerDraw {
                cpu: 0.0,
                gpu: None,
                total: 0.0,
            });

        drop(asusd_lock);

        Ok(HardwareMetrics {
            temperatures: temps,
            fan_speeds: fans,
            power_draw: power,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::implementations::mock::MockAsusd;
    use crate::implementations::sysfs_fallback::MockSysMonitor;

    #[tokio::test]
    async fn test_poll_once() {
        let asusd = Arc::new(Mutex::new(Box::new(MockAsusd::new()) as Box<dyn AsusdTrait>));
        let sys_monitor = Arc::new(Mutex::new(Box::new(MockSysMonitor::new()) as Box<dyn SysMonitorTrait>));

        let poller = MetricsPoller::new(asusd, Some(sys_monitor));

        let metrics = poller.poll_once().await.unwrap();
        assert!(metrics.temperatures.cpu >= 0.0);
    }

    #[tokio::test]
    async fn test_start_stop_polling() {
        let asusd = Arc::new(Mutex::new(Box::new(MockAsusd::new()) as Box<dyn AsusdTrait>));
        let sys_monitor = None;

        let poller = MetricsPoller::new(asusd, sys_monitor);

        // Start polling
        poller.start_polling(100).await.unwrap();
        assert!(poller.is_polling().await);

        // Give it time to poll once
        tokio::time::sleep(Duration::from_millis(150)).await;

        // Stop polling
        poller.stop_polling().await;
        tokio::time::sleep(Duration::from_millis(50)).await;
        assert!(!poller.is_polling().await);
    }

    #[tokio::test]
    async fn test_double_start_polling() {
        let asusd = Arc::new(Mutex::new(Box::new(MockAsusd::new()) as Box<dyn AsusdTrait>));
        let sys_monitor = None;

        let poller = MetricsPoller::new(asusd, sys_monitor);

        // Start twice should be idempotent
        poller.start_polling(100).await.unwrap();
        poller.start_polling(100).await.unwrap();
        assert!(poller.is_polling().await);

        poller.stop_polling().await;
    }
}
