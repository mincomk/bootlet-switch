use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Serial port error: {0}")]
    SerialPort(#[from] serialport::Error),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Device not found")]
    DeviceNotFound,

    #[error("Invalid configuration: {0}")]
    InvalidConfiguration(String),

    #[error("Deserialization error")]
    DeserializationError,

    #[error("Invalid response from device")]
    InvalidResponse,
}

pub type Result<T> = std::result::Result<T, Error>;
