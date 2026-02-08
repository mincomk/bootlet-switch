use embassy_stm32::{
    Config, bind_interrupts, peripherals,
    rcc::{AHBPrescaler, APBPrescaler, Hse, HseMode, Pll, PllMul, PllPreDiv, PllSource, Sysclk},
    time::mhz,
    usb,
};

bind_interrupts!(pub struct Irqs {
    USB_LP_CAN1_RX0 => usb::InterruptHandler<peripherals::USB>;
});

pub fn config() -> Config {
    let mut config = Config::default();
    config.rcc.hse = Some(Hse {
        freq: mhz(8),
        mode: HseMode::Oscillator,
    });
    config.rcc.sys = Sysclk::PLL1_P;
    config.rcc.pll = Some(Pll {
        src: PllSource::HSE,
        prediv: PllPreDiv::DIV1,
        mul: PllMul::MUL9,
    });
    config.rcc.ahb_pre = AHBPrescaler::DIV1;
    config.rcc.apb1_pre = APBPrescaler::DIV2;
    config.rcc.apb2_pre = APBPrescaler::DIV1;

    config
}
