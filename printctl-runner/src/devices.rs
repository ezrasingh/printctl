use std::path::PathBuf;

use serial_enumerator::{get_serial_list, SerialInfo};

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

    pub fn port(&self) {
        todo!()
    }
}

impl From<Device> for PathBuf {
    fn from(value: Device) -> Self {
        value.path
    }
}

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
