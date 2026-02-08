use embassy_stm32::{Peri, Peripherals, peripherals::*};

macro_rules! define_pins {
    ($($name:ident: $ty:ty),* $(,)?) => {
        pub struct BoardPins {
            $(pub $name: Peri<'static, $ty>,)*
        }
    };
}

define_pins! {
    usb: USB,
    usb_dp: PA12,
    usb_dm: PA11,

    switch: PA0,
}

pub fn init_pins(p: Peripherals) -> BoardPins {
    BoardPins {
        usb: p.USB,
        usb_dp: p.PA12,
        usb_dm: p.PA11,
        switch: p.PA0,
    }
}
