use types::constant::*;
use types::*;

use crate::{core::util, hal::Serial};

pub async fn wait_for_message(serial: &mut impl Serial) -> crate::Result<MessagePcToBoard> {
    let mut buf = [0u8; USB_BUFFER_SIZE];
    util::read_until_cobs(serial, &mut buf).await?;

    postcard::from_bytes_cobs(&mut buf).map_err(|_| crate::Error::DeserializationError)
}

pub async fn send_message(
    serial: &mut impl Serial,
    message: &MessageBoardToPc,
) -> crate::Result<()> {
    let mut buf = [0u8; USB_BUFFER_SIZE];
    let len = postcard::to_slice_cobs(message, &mut buf)
        .map_err(|_| crate::Error::DeserializationError)?
        .len();

    serial.write(&buf[..len]).await
}
