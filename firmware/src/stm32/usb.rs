use embassy_executor::Spawner;
use embassy_stm32::{
    Peri, peripherals,
    usb::{DmPin, DpPin, Driver},
};
use embassy_usb::{
    UsbDevice,
    class::cdc_acm::{CdcAcmClass, State},
};
use static_cell::StaticCell;

use crate::{hal::Serial, make_static, stm32::config::Irqs};
use types::constant::*;

pub type Stm32UsbDriver = Driver<'static, peripherals::USB>;
pub type Stm32UsbClass = CdcAcmClass<'static, Stm32UsbDriver>;

pub struct Stm32Usb {
    class: Stm32UsbClass,
}

pub fn init_usb(
    spawner: Spawner,
    usb: Peri<'static, peripherals::USB>,
    dp: Peri<'static, impl DpPin<peripherals::USB>>,
    dm: Peri<'static, impl DmPin<peripherals::USB>>,
    manufacturer: &'static str,
    product: &'static str,
    vid: u16,
    pid: u16,
) -> Stm32Usb {
    let driver: Stm32UsbDriver = embassy_stm32::usb::Driver::new(usb, Irqs, dp, dm);

    let mut usb_config = embassy_usb::Config::new(vid, pid);
    usb_config.manufacturer = Some(manufacturer);
    usb_config.product = Some(product);

    let device_descriptor = make_static!([u8; 256], [0; 256]);
    let config_descriptor = make_static!([u8; 256], [0; 256]);
    let bos_descriptor = make_static!([u8; 256], [0; 256]);
    let control_buf = make_static!([u8; USB_BUFFER_SIZE], [0; USB_BUFFER_SIZE]);

    let state = make_static!(State, State::new());

    let mut builder = embassy_usb::Builder::new(
        driver,
        usb_config,
        device_descriptor,
        config_descriptor,
        bos_descriptor,
        control_buf,
    );

    let class: Stm32UsbClass = CdcAcmClass::new(&mut builder, state, USB_BUFFER_SIZE as u16);
    let usb = builder.build();

    spawner.must_spawn(usb_task(usb));

    Stm32Usb { class }
}

impl Serial for Stm32Usb {
    async fn wait_connection(&mut self) {
        self.class.wait_connection().await;
    }

    async fn write(&mut self, data: &[u8]) -> crate::Result<()> {
        self.class.write_packet(data).await?;
        Ok(())
    }

    async fn read(&mut self, buffer: &mut [u8]) -> crate::Result<usize> {
        let n = self.class.read_packet(buffer).await?;
        Ok(n)
    }
}

#[embassy_executor::task]
async fn usb_task(mut device: UsbDevice<'static, Stm32UsbDriver>) {
    device.run().await;
}
