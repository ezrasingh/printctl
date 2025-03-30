use crate::prelude::*;

use serialport::{SerialPort, SerialPortInfo};

pub type DeviceInfo = SerialPortInfo;
pub type DevicePort = Box<dyn SerialPort>;

pub trait DeviceManager {
    fn available_ports(&self) -> Vec<DeviceInfo> {
        serialport::available_ports().unwrap_or_default()
    }

    fn open_port(&self, device_path: &str, baud_rate: u32) -> Result<DevicePort> {
        let port = serialport::new(device_path, baud_rate).open()?;
        Ok(port)
    }
}
