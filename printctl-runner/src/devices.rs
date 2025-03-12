use serial::SystemPort;
use std::path::PathBuf;

#[derive(Debug)]
pub struct Device {
    path: PathBuf,
    vendor: String,
    product: String,
    usb: String,
}

impl Device {
    pub fn path(&self) -> &PathBuf {
        &self.path
    }
    pub fn vendor(&self) -> &String {
        &self.vendor
    }
    pub fn product(&self) -> &String {
        &self.product
    }
    pub fn usb(&self) -> &String {
        &self.usb
    }

    pub fn port(&self) -> Result<SystemPort, serial::Error> {
        serial::open(self.path.as_path())
    }
}

impl From<Device> for PathBuf {
    fn from(value: Device) -> Self {
        value.path
    }
}

use serial_enumerator::{get_serial_list, SerialInfo};

impl From<SerialInfo> for Device {
    fn from(serial_info: SerialInfo) -> Self {
        let field_or_else = || Some(String::from("--"));
        return Self {
            path: serial_info.name.into(),
            vendor: serial_info.vendor.or_else(field_or_else).unwrap(),
            product: serial_info.product.or_else(field_or_else).unwrap(),
            usb: serial_info
                .usb_info
                .and_then(|usbinfo| Some(format!("{}:{}", usbinfo.vid, usbinfo.pid)))
                .or_else(field_or_else)
                .unwrap(),
        };
    }
}

pub fn list_devices() -> Vec<Device> {
    get_serial_list()
        .into_iter()
        .map(|info| Device::from(info))
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_system_port() {
        let devices = list_devices();
        println!("{:#?}", &devices);
    }
}
