use serialport::{SerialPort, SerialPortType};
use types::constant::{USB_PID, USB_VID};

pub fn find_device() -> crate::Result<Box<dyn SerialPort>> {
    let ports = serialport::available_ports()?;

    let port_info = ports
        .into_iter()
        .find(|p| {
            if let SerialPortType::UsbPort(info) = &p.port_type {
                info.vid == USB_VID && info.pid == USB_PID
            } else {
                false
            }
        })
        .ok_or(crate::Error::DeviceNotFound)?;

    tracing::debug!("Found device on port: {}", port_info.port_name);

    let port = serialport::new(port_info.port_name, 115200)
        .timeout(std::time::Duration::from_millis(1000))
        .open()?;

    port.clear(serialport::ClearBuffer::All)?;

    Ok(port)
}
