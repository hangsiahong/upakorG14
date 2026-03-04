use thiserror::Error;

/// Comprehensive error types for Upakor-G14
#[derive(Debug, Error)]
pub enum UpakorError {
    /// D-Bus connection failed
    #[error("D-Bus connection failed: {message}")]
    DbusConnection {
        message: String,
        #[source]
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    },

    /// D-Bus method call failed
    #[error("D-Bus method call failed: {method} on {interface}: {message}")]
    DbusMethod {
        method: String,
        interface: String,
        message: String,
        #[source]
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    },

    /// Property not supported by this device
    #[error("Feature '{feature}' not supported on this device")]
    NotSupported {
        feature: String,
        suggestion: Option<String>,
    },

    /// Invalid user input
    #[error("Invalid value for '{field}': {value}. {message}")]
    InvalidValue {
        field: String,
        value: String,
        message: String,
    },

    /// Permission denied - requires elevated privileges
    #[error("Permission denied: {operation} requires root/privileged access")]
    PermissionDenied {
        operation: String,
        suggestion: Option<String>,
    },

    /// Service unavailable
    #[error("Service '{service}' not available: {message}")]
    ServiceUnavailable {
        service: String,
        message: String,
        suggestion: Option<String>,
    },

    /// Configuration file errors
    #[error("Configuration error in {file}: {message}")]
    Config {
        file: String,
        message: String,
        #[source]
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    },

    /// Hardware-specific errors
    #[error("Hardware error: {operation} on {device} failed: {message}")]
    Hardware {
        operation: String,
        device: String,
        message: String,
        #[source]
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    },

    /// I/O errors
    #[error("I/O error: {path} - {message}")]
    Io {
        path: String,
        message: String,
        #[source]
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    },

    /// Unknown/generic errors
    #[error("{message}")]
    Unknown {
        message: String,
        #[source]
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    },
}

impl UpakorError {
    /// Create a user-friendly message for display in UI
    pub fn user_message(&self) -> String {
        match self {
            UpakorError::DbusConnection { message, .. } => {
                format!("Failed to connect to system services: {}", message)
            }
            UpakorError::DbusMethod { method, message, .. } => {
                format!("Failed to execute {}: {}", method, message)
            }
            UpakorError::NotSupported { feature, suggestion } => {
                let mut msg = format!("Your device doesn't support {}", feature);
                if let Some(sugg) = suggestion {
                    msg.push_str(&format!("\n\nSuggestion: {}", sugg));
                }
                msg
            }
            UpakorError::InvalidValue { field, message, .. } => {
                format!("Invalid {}: {}", field, message)
            }
            UpakorError::PermissionDenied { operation, suggestion } => {
                let mut msg = format!("Permission denied: {} requires administrator access", operation);
                if let Some(sugg) = suggestion {
                    msg.push_str(&format!("\n\nSuggestion: {}", sugg));
                }
                msg
            }
            UpakorError::ServiceUnavailable { service, suggestion, .. } => {
                let mut msg = format!("{} service is not available", service);
                if let Some(sugg) = suggestion {
                    msg.push_str(&format!("\n\nSuggestion: {}", sugg));
                }
                msg
            }
            UpakorError::Config { file, message, .. } => {
                format!("Configuration error in {}: {}", file, message)
            }
            UpakorError::Hardware { device, message, .. } => {
                format!("Hardware error on {}: {}", device, message)
            }
            UpakorError::Io { path, message, .. } => {
                format!("File error: {} - {}", path, message)
            }
            UpakorError::Unknown { message, .. } => {
                message.clone()
            }
        }
    }

    /// Check if this error is retryable (transient)
    pub fn is_retryable(&self) -> bool {
        match self {
            UpakorError::DbusConnection { .. } => true,
            UpakorError::ServiceUnavailable { .. } => true,
            UpakorError::Hardware { message, .. } => {
                message.contains("timed out") || message.contains("temporary")
            }
            _ => false,
        }
    }
}

impl From<zbus::Error> for UpakorError {
    fn from(err: zbus::Error) -> Self {
        UpakorError::DbusConnection {
            message: err.to_string(),
            source: Some(Box::new(err)),
        }
    }
}

impl From<std::io::Error> for UpakorError {
    fn from(err: std::io::Error) -> Self {
        UpakorError::Io {
            path: "unknown".to_string(),
            message: err.to_string(),
            source: Some(Box::new(err)),
        }
    }
}

impl From<UpakorError> for String {
    fn from(err: UpakorError) -> Self {
        err.to_string()
    }
}

pub type Result<T> = std::result::Result<T, UpakorError>;
