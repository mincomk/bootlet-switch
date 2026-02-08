use crate::comms::{send_message, wait_for_message};
use types::{MessageBoardToPc, MessagePcToBoard};

mod comms;
mod serial;

pub mod error;
pub use error::*;

pub use serial::find_device;

pub fn get_switch_state() -> crate::Result<bool> {
    let mut port = find_device()?;

    ask_switch_state(&mut *port)
}

fn ask_switch_state(serial: &mut dyn serialport::SerialPort) -> crate::Result<bool> {
    send_message(serial, &MessagePcToBoard::Ask)?;

    let response = wait_for_message(serial)?;

    match response {
        MessageBoardToPc::State(state) => Ok(state),
        _ => Err(crate::Error::InvalidResponse),
    }
}
