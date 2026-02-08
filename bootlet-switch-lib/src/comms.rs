use serialport::SerialPort;
use types::constant::USB_BUFFER_SIZE;
use types::*;

fn read_until_cobs(serial: &mut dyn SerialPort, buf: &mut [u8]) -> crate::Result<()> {
    let mut index = 0;
    loop {
        let mut byte = [0u8; 1];
        serial.read_exact(&mut byte)?;
        if byte[0] == 0 {
            break;
        }
        if index >= buf.len() {
            return Err(crate::Error::DeserializationError);
        }
        buf[index] = byte[0];
        index += 1;
    }
    Ok(())
}

pub fn wait_for_message(serial: &mut dyn SerialPort) -> crate::Result<MessageBoardToPc> {
    let mut buf = [0u8; USB_BUFFER_SIZE];

    read_until_cobs(serial, &mut buf)?;

    let message: MessageBoardToPc =
        postcard::from_bytes_cobs(&mut buf).map_err(|_| crate::Error::DeserializationError)?;
    Ok(message)
}

pub fn send_message(serial: &mut dyn SerialPort, message: &MessagePcToBoard) -> crate::Result<()> {
    let mut buf = [0u8; USB_BUFFER_SIZE];
    let len = postcard::to_slice_cobs(message, &mut buf)
        .map_err(|_| crate::Error::DeserializationError)?
        .len();

    serial.write_all(&buf[..len])?;

    Ok(())
}
