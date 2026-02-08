#![no_std]
#![no_main]

use crate::{core::comms, hal::Serial};
use types::*;

use embassy_executor::Spawner;

use {defmt_rtt as _, panic_probe as _};

mod error;
pub use error::*;

mod stm32;

mod core;
mod hal;

mod macros;

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let r = stm32::init(spawner).await;

    defmt::info!("Starting bootlet-switch");

    let mut usb_serial = r.usb;

    loop {
        usb_serial.wait_connection().await;

        defmt::info!("USB connection established, waiting for messages");

        loop {
            match comms::wait_for_message(&mut usb_serial).await {
                Ok(message) => match message {
                    MessagePcToBoard::Ask => {
                        let switch_state = r.switch.is_low();
                        defmt::info!("Switch state requested, state: {}", switch_state);

                        let response = MessageBoardToPc::State(switch_state);
                        if let Err(e) = comms::send_message(&mut usb_serial, &response).await {
                            defmt::warn!(
                                "Error sending message, restarting connection wait: {:?}",
                                e
                            );
                            break;
                        }
                    }
                    _ => {
                        defmt::warn!("Unexpected message received, restarting connection wait");
                        break;
                    }
                },
                Err(e) => {
                    defmt::warn!(
                        "Error receiving message, restarting connection wait: {:?}",
                        e
                    );

                    break;
                }
            }
        }
    }
}
