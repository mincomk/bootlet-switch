#[derive(Debug, defmt::Format)]
pub enum Error {
    SerialError,
    DeserializationError,
}

impl From<embassy_usb::driver::EndpointError> for Error {
    fn from(_: embassy_usb::driver::EndpointError) -> Self {
        defmt::warn!("USB EndpointError occurred");
        Error::SerialError
    }
}

pub type Result<T> = core::result::Result<T, Error>;
