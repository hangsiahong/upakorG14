use tracing::{info, warn, error, debug};
use std::time::Instant;

/// Helper for timing operations and logging duration
pub struct OperationTimer {
    name: String,
    start: Instant,
}

impl OperationTimer {
    /// Start timing a new operation
    pub fn new(name: impl Into<String>) -> Self {
        let name = name.into();
        debug!(operation = %name, "Starting operation");
        Self {
            name,
            start: Instant::now(),
        }
    }

    /// Complete the operation and log the duration
    pub fn complete(self) {
        let duration = self.start.elapsed();
        info!(
            operation = %self.name,
            duration_ms = duration.as_millis(),
            "Operation completed"
        );
    }

    /// Complete the operation with a custom message
    pub fn complete_with(self, message: impl Into<String>) {
        let duration = self.start.elapsed();
        let msg = message.into();
        info!(
            operation = %self.name,
            duration_ms = duration.as_millis(),
            message = %msg,
            "Operation completed"
        );
    }

    /// Complete the operation with a result
    pub fn complete_result<T, E: std::fmt::Display>(self, result: &std::result::Result<T, E>) {
        let duration = self.start.elapsed();
        match result {
            Ok(_) => {
                info!(
                    operation = %self.name,
                    duration_ms = duration.as_millis(),
                    status = "success",
                    "Operation completed"
                );
            }
            Err(e) => {
                error!(
                    operation = %self.name,
                    duration_ms = duration.as_millis(),
                    status = "error",
                    error = %e,
                    "Operation failed"
                );
            }
        }
    }
}

/// Retry logic for transient errors
pub async fn retry_with_backoff<F, T, E>(
    operation_name: &str,
    mut operation: F,
    max_retries: u32,
    initial_delay_ms: u64,
) -> std::result::Result<T, E>
where
    F: FnMut() -> std::pin::Pin<Box<dyn std::future::Future<Output = std::result::Result<T, E>> + Send>>,
    E: std::fmt::Display,
{
    let mut delay = initial_delay_ms;
    let mut last_error = None;

    for attempt in 0..=max_retries {
        if attempt > 0 {
            debug!(
                operation = %operation_name,
                attempt,
                delay_ms = delay,
                "Retrying operation"
            );
            tokio::time::sleep(tokio::time::Duration::from_millis(delay)).await;
            delay = (delay * 2).min(5000); // Exponential backoff, max 5 seconds
        }

        match operation().await {
            Ok(result) => {
                if attempt > 0 {
                    info!(
                        operation = %operation_name,
                        attempts = attempt + 1,
                        "Operation succeeded after retries"
                    );
                }
                return Ok(result);
            }
            Err(e) => {
                warn!(
                    operation = %operation_name,
                    attempt,
                    error = %e,
                    "Operation attempt failed"
                );
                last_error = Some(e);
            }
        }
    }

    Err(last_error.unwrap())
}

/// Log system information at startup
pub fn log_system_info() {
    info!("Upakor-G14 starting...");
    info!("OS: {}", std::env::consts::OS);
    info!("Arch: {}", std::env::consts::ARCH);
    info!("Version: {}", env!("CARGO_PKG_VERSION"));

    // Log D-Bus session type
    if let Ok(session) = std::env::var("XDG_SESSION_TYPE") {
        info!("Session type: {}", session);
    }

    // Check if running in terminal
    if atty::is(atty::Stream::Stdout) {
        info!("Running in terminal mode");
    } else {
        info!("Running in GUI mode");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_operation_timer() {
        let timer = OperationTimer::new("test_operation");
        std::thread::sleep(std::time::Duration::from_millis(10));
        timer.complete();
    }

    #[tokio::test]
    async fn test_retry_with_backoff_success() {
        // Simplified test - just verify it eventually succeeds
        let mut call_count = 0;
        let result = retry_with_backoff(
            "test_operation",
            || {
                call_count += 1;
                Box::pin(async move {
                    if call_count < 3 {
                        Err("temporary error")
                    } else {
                        Ok("success")
                    }
                })
            },
            5,
            10,
        )
        .await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "success");
    }

    #[tokio::test]
    async fn test_retry_with_backoff_failure() {
        let result = retry_with_backoff(
            "test_operation",
            || Box::pin(async { Err::<(), _>("persistent error") }),
            2,
            10,
        )
        .await;

        assert!(result.is_err());
    }
}
