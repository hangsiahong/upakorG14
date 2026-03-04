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
