use embassy_executor::Spawner;
use embassy_stm32::gpio::{Input, Pull};

use types::constant::*;

use crate::stm32::usb::Stm32Usb;

mod config;
mod pins;
mod usb;

pub struct Stm32InitResult {
    pub usb: Stm32Usb,
    pub switch: Input<'static>,
}

pub async fn init(spawner: Spawner) -> Stm32InitResult {
    let p = embassy_stm32::init(config::config());
    let pins = pins::init_pins(p);

    let usb = usb::init_usb(
        spawner,
        pins.usb,
        pins.usb_dp,
        pins.usb_dm,
        USB_MANUFACTURER,
        USB_PRODUCT,
        USB_VID,
        USB_PID,
    );

    let switch = Input::new(pins.switch, Pull::Up);

    Stm32InitResult { usb, switch }
}
